use base64::Engine as _;

fn main() {
    let mut state = 0x4d59_5df4_d0f3_3173u64;
    for len in 0..=4096usize {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            bytes.push(state as u8);
        }
        let expected = base64::engine::general_purpose::STANDARD.encode(&bytes);
        let actual = protocol::base64_standard_encode(&bytes);
        assert_eq!(actual, expected, "encode mismatch at length {len}");
        assert_eq!(protocol::base64_standard_decode(actual.as_bytes()), Ok(bytes.clone()), "owned decode mismatch at length {len}");
        assert_eq!(base64::engine::general_purpose::STANDARD.decode(actual.as_bytes()), Ok(bytes), "reference decode mismatch at length {len}");
    }
    for malformed in [b"Zg".as_slice(), b"Z g=".as_slice(), b"=m9v".as_slice(), b"Zm=v".as_slice(), b"Zg==Zm8=".as_slice(), b"Zh==".as_slice(), b"Zm9=".as_slice()] {
        assert!(protocol::base64_standard_decode(malformed).is_err(), "owned decoder accepted malformed input {malformed:?}");
        assert!(base64::engine::general_purpose::STANDARD.decode(malformed).is_err(), "reference decoder accepted malformed input {malformed:?}");
    }
    println!("[DEBUG] p9y base64 differential parity: 4097 payloads and 7 malformed vectors");
}
