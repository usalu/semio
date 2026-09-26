//! Scenario for `insert-roofs`.
#[test]
fn applies_insert_roofs() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
