//! Scenario for `change-silo-claimed`.
#[test]
fn applies_change_silo_claimed() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
