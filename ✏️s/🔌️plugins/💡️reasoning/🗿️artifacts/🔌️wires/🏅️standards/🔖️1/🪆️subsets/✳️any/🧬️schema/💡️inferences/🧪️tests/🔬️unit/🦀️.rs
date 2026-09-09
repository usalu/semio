use super::*;
use crate::empty_wires_snapshot;
use dsl::DslValue;
use protocol::Inference;

fn chain_snapshot() -> WiresSnapshot {
    let mut snapshot = empty_wires_snapshot();
    let nodes = vec![DslValue::object([("id".into(), DslValue::String("a".into()))]), DslValue::object([("id".into(), DslValue::String("b".into()))])];
    let edges = vec![DslValue::object([("id".into(), DslValue::String("e1".into())), ("source".into(), DslValue::String("a".into())), ("target".into(), DslValue::String("b".into()))])];
    snapshot.content = crate::wires_content_child_with_owner(nodes, edges);
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = chain_snapshot();
    assert_eq!(WiresInference::infer(&snapshot), WiresInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(WiresInference::infer(&empty_wires_snapshot()), WiresInference::default());
}

#[semio_framework_async_macros::async_test]
async fn topology_counts_the_board_graph() {
    let inferred = WiresInference::infer(&chain_snapshot());
    assert_eq!(inferred.topology.node_count, 2);
    assert_eq!(inferred.topology.edge_count, 1);
    assert!(inferred.topology.cycle_free);
}
