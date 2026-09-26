//! Scenario for `change-depth`.
#[test]
fn applies_change_depth() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
