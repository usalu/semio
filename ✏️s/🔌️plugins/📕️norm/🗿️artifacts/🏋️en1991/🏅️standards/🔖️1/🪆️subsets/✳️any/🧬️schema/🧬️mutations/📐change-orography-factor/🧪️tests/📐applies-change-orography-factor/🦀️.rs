//! Scenario for `change-orography-factor`.
#[test]
fn applies_change_orography_factor() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
