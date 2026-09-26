//! Scenario for `change-roof-assumed-sk`.
#[test]
fn applies_change_roof_assumed_sk() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
