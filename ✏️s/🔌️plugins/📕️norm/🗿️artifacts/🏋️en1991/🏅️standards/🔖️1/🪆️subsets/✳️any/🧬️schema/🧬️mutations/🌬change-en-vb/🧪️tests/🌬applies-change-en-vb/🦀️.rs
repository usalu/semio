//! Scenario for `change-en-vb`.
#[test]
fn applies_change_en_vb() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
