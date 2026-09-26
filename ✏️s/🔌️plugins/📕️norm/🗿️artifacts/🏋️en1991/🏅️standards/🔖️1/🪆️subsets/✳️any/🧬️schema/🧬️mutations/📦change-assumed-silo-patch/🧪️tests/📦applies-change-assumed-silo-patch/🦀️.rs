//! Scenario for `change-assumed-silo-patch`.
#[test]
fn applies_change_assumed_silo_patch() {
    let base = crate::En1991Snapshot::default();
    let _ = base.assumed_silo_patch;
}
