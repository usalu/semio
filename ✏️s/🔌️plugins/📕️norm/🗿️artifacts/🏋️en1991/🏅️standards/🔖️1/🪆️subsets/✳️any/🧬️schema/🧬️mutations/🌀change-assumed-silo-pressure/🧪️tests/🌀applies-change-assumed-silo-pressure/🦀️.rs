//! Scenario for `change-assumed-silo-pressure`.
#[test]
fn applies_change_assumed_silo_pressure() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
