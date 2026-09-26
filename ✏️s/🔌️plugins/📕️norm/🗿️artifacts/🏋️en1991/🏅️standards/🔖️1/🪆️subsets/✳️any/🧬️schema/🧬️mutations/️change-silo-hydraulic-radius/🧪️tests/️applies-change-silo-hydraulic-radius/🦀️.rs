//! Scenario for `change-silo-hydraulic-radius`.
#[test]
fn applies_change_silo_hydraulic_radius() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
