//! Scenario for `insert-accidental-cases`.
#[test]
fn applies_insert_accidental_cases() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
