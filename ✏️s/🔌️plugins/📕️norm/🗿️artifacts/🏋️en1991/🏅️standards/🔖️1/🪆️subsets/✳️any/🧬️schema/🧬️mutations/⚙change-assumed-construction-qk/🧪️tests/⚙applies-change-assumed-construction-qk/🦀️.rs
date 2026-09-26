//! Scenario for `change-assumed-construction-qk`.
#[test]
fn applies_change_assumed_construction_qk() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
