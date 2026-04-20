//! Supplemental tests; large suites remain in `lib.rs` until modular split.

#[test]
fn session_kit_is_arc_rwlock_wrapped() {
    let kit = crate::Kit::from_json_str(
        r#"{"guid":"00000000-0000-7000-8000-000000000001","name":"smoke"}"#,
    )
    .expect("minimal kit");
    let session = crate::KitGraphSession::new(kit);
    let h = session.kit_handle().expect("handle");
    assert!(h.read().is_ok());
}
