//! Scenario for `change-accidental-assumed-force`.
#[test]
fn applies_change_accidental_assumed_force() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
