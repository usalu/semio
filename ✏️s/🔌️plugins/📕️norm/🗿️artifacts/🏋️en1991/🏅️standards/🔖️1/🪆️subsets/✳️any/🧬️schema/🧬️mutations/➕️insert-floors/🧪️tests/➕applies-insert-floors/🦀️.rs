//! Scenario for `insert-floors`.
#[test]
fn applies_insert_floors() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
