//! Scenario for `change-en-sk`.
#[test]
fn applies_change_en_sk() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
