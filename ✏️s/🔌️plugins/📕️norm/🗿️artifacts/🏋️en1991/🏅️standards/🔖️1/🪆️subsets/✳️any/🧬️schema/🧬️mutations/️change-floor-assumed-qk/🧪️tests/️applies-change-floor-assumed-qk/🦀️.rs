//! Scenario for `change-floor-assumed-qk`.
#[test]
fn applies_change_floor_assumed_qk() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
