//! Scenario for `change-coast-or-island`.
#[test]
fn applies_change_coast_or_island() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
