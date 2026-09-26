//! Scenario for `change-height`.
#[test]
fn applies_change_height() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
