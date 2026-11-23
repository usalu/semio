pub fn label(value: u32) -> String {
    let mut buffer = itoa::Buffer::new();
    format!("side-{}", buffer.format(value))
}
