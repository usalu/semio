//! 🧪️ Reproduces the audited eager hexadecimal-nibble expression against the standard-library oracle.

//#region Subject
fn nibble(value: u8) -> Option<u8> {
    if value.is_ascii_digit() {
        return Some(value - b'0');
    }
    (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
}
//#endregion Subject

//#region Oracle
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    for value in [b'!', b'A', b'0', b'f'] {
        let observed = std::panic::catch_unwind(|| nibble(value));
        let expected = char::from(value).to_digit(16).filter(|_| value.is_ascii_digit() || (b'a'..=b'f').contains(&value)).map(|value| value as u8);
        println!("[DEBUG] byte={value} actual={observed:?} oracle={expected:?}");
        if matches!(value, b'!' | b'A') {
            assert!(observed.is_err());
            assert_eq!(expected, None);
        } else {
            assert_eq!(observed.unwrap(), expected);
        }
    }
}
//#endregion Oracle
