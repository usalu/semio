#[no_mangle]
pub extern "C" fn echo(value: u32) -> u32 {
    std::hint::black_box(value)
}
