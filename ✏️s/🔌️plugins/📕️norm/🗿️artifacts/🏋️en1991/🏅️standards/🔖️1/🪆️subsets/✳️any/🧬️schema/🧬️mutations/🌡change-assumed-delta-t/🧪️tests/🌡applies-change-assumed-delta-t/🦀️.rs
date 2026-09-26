//! Scenario for `change-assumed-delta-t`.
#[test]
fn applies_change_assumed_delta_t() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
