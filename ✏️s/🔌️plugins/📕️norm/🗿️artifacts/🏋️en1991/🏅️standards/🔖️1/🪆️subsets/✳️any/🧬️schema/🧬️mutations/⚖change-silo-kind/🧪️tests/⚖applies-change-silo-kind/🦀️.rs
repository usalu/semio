//! Scenario for `change-silo-kind`.
#[test]
fn applies_change_silo_kind() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
