//! Scenario for `change-structure-kind`.
#[test]
fn applies_change_structure_kind() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
