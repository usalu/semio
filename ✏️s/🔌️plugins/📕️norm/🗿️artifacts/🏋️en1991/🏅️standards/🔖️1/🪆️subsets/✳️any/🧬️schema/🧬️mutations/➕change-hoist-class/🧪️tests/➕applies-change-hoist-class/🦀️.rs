//! Scenario for `change-hoist-class`.
#[test]
fn applies_change_hoist_class() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
