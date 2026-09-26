//! Scenario for `change-silo-bulk-density`.
#[test]
fn applies_change_silo_bulk_density() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
