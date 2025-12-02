#![no_std]
#![no_main]

use arduino_hal::{Delay, I2c, Usart, i2c::Direction, prelude::_unwrap_infallible_UnwrapInfallible, usart::Baudrate};
use embedded_dht_rs::dht22::Dht22;
use embedded_hal::delay::DelayNs;
use lcd_lcm1602_i2c::{Backlight, Font, sync_lcd::Lcd};
use panic_halt as _;
use ufmt::{uDisplay, uWrite, uwrite, uwriteln};
use ufmt_float::uFmt_f32;

const BAUD_RATE: u32 = 9600;
const I2C_SPEED: u32 = 400_000;
const LOOP_MS: u32 = 500;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    /*let serial = Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        Baudrate::new(BAUD_RATE)
    );*/

    let mut led = pins.d13.into_output();

    let mut i2c = I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        I2C_SPEED,
    );

    //i2c.i2cdetect(&mut serial, Direction::Write).unwrap();

    let mut dht = Dht22::new(pins.a0.into_opendrain_high(), Delay::new());

    let delay = &mut Delay::new();
    let mut lcd = Lcd::new(&mut i2c, delay)
        .with_address(0x27)
        .with_cursor_on(false)
        .with_cursor_blink(false)
        .with_rows(2)
        .init()
        .unwrap();

    lcd.clear().unwrap();
    lcd.backlight(Backlight::On).unwrap();
    lcd.font_mode(Font::Font5x10).unwrap();

    let delay = &mut Delay::new();

    loop {
        led.toggle();

        let data = dht.read().unwrap();

        lcd.set_cursor(0, 0).unwrap();
        uwrite!(lcd, "T {}\u{00df}C {}\u{00df}F", uFmt_f32::Two(data.temperature), uFmt_f32::One(data.temperature*9.0/5.0 + 32.0)).unwrap();

        lcd.set_cursor(1, 0).unwrap();
        uwrite!(lcd, "H {}%", uFmt_f32::Two(data.humidity)).unwrap();

        delay.delay_ms(LOOP_MS);
    }
}