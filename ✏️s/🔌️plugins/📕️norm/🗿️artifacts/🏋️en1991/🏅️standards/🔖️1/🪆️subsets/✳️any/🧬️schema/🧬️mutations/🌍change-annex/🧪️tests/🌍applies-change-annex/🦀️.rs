//! Scenario for `change-annex`.
#[test]
fn applies_change_annex() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
