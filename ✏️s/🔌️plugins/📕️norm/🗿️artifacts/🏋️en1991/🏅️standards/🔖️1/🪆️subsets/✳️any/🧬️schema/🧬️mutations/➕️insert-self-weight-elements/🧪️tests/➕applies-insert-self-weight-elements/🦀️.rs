//! Scenario for `insert-self-weight-elements`.
#[test]
fn applies_insert_self_weight_elements() {
    let base = crate::En1991Snapshot::default();
    // smoke: construct payload type exists
    let _ = base.annex;
}
