//! Scenario for `change-self-weight-assumed-gk`.
#[test]
fn applies_change_self_weight_assumed_gk() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
