//! Scenario for `change-silo-mu`.
#[test]
fn applies_change_silo_mu() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
