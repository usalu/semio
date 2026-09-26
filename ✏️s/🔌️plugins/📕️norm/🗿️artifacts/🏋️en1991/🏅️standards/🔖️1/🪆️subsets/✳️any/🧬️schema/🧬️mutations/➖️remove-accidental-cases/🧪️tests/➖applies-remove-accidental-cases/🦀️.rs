//! Scenario for `remove-accidental-cases`.
#[test]
fn applies_remove_accidental_cases() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
