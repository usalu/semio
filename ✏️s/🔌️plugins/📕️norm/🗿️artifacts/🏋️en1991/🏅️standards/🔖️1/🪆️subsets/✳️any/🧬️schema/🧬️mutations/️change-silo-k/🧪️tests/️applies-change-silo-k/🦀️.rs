//! Scenario for `change-silo-k`.
#[test]
fn applies_change_silo_k() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
