//! 🔬️ The binary facet round-trips and refuses a foreign envelope.


#[test]
fn encoding_then_decoding_is_a_fixed_point() {
    let snapshot = crate::examples::blocks::snapshot();
    let bytes = crate::standards::v1::subsets::any::io::binary::snapshot::encode(&snapshot);
    assert!(bytes.len() > 64);
    assert_eq!(crate::standards::v1::subsets::any::io::binary::snapshot::decode(&bytes).expect("pack decodes"), snapshot);
}

#[test]
fn a_foreign_envelope_is_refused_rather_than_misread() {
    assert!(crate::standards::v1::subsets::any::io::binary::snapshot::decode(b"not a semio pack at all").is_err());
}
