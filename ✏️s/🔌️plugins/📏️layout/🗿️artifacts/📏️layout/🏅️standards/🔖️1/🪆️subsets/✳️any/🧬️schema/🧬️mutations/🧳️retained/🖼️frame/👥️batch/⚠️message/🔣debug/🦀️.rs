//! 🔣️ One original scalar becomes the exact Rust String Debug body in fixed stack storage.
/// 🔤️ String context keeps apostrophes while std escape_debug supplies the canonical Unicode classification.
pub fn rust_string_debug_scalar(value:char,output:&mut[u8;10])->usize{if value=='\''{output[0]=b'\'';return 1}let mut used=0;for scalar in value.escape_debug(){let mut utf8=[0;4];let encoded=scalar.encode_utf8(&mut utf8).as_bytes();output[used..used+encoded.len()].copy_from_slice(encoded);used+=encoded.len()}used}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
