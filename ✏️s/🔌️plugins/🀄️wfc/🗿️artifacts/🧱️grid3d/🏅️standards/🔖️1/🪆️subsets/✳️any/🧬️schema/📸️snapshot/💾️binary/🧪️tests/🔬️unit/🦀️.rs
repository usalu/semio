//! 🔬️ The binary facet round-trips, refuses a foreign envelope, and releases a displaced snapshot
//! through the bounded retirement rather than an unbounded `Drop`.


#[test]
fn encoding_then_decoding_is_a_fixed_point() {
    let snapshot = crate::examples::blocks::snapshot();
    let bytes = super::encode(&snapshot);
    assert!(bytes.len() > 64);
    assert_eq!(super::decode(&bytes).expect("pack decodes"), snapshot);
}

#[test]
fn a_foreign_envelope_is_refused_rather_than_misread() {
    assert!(super::decode(b"not a semio pack at all").is_err());
}

#[test]
fn a_displaced_snapshot_is_released_in_bounded_steps() {
    let mut live = crate::examples::blocks::snapshot();
    let bytes = super::encode(&crate::examples::pipes_3d::snapshot());
    let mut retirement: Box<dyn store::ErasedSnapshotRetirement> = super::decode_into(&mut live, &bytes).expect("decode in place");
    assert_eq!(live, crate::examples::pipes_3d::snapshot());
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        retirement.close_step(8, 1 << 16).expect("retirement step");
        steps += 1;
        assert!(steps < 64, "retirement must terminate");
    }
    assert!(steps >= 7, "every collection is charged its own step");
}
