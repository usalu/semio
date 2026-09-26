//! Scenario for `remove-roofs`.
#[test]
fn applies_remove_roofs() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
