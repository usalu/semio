//! Scenario for `change-construction-activity`.
#[test]
fn applies_change_construction_activity() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
