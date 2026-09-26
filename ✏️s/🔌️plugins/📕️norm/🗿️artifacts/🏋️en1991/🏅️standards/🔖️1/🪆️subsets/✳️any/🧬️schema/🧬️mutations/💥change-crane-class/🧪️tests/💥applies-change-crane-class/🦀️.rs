//! Scenario for `change-crane-class`.
#[test]
fn applies_change_crane_class() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
