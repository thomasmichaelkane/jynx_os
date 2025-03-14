#![no_std] 
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jynx_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jynx_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello Lan{}", "!");

    jynx_os::init();

    

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jynx_os::test_panic_handler(info)
}