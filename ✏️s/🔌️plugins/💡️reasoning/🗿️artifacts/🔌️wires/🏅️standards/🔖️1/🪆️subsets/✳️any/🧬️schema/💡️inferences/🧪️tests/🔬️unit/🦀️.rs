use super::*;
use crate::empty_wires_snapshot;
use protocol::Inference;
use semio_framework_value::DslValue;

fn chain_content() -> crate::SemioGraphSnapshot {
    let nodes = vec![DslValue::object([("id".into(), DslValue::String("a".into()))]), DslValue::object([("id".into(), DslValue::String("b".into()))])];
    let edges = vec![DslValue::object([("id".into(), DslValue::String("e1".into())), ("source".into(), DslValue::String("a".into())), ("target".into(), DslValue::String("b".into()))])];
    crate::wires_content_snapshot(&nodes, &edges)
}

/// ⚖️ LAW: a parent-only inference is deterministic and reads the empty topology, because the parent carries no board.
#[semio_framework_async_macros::async_test]
async fn a_parent_only_inference_reads_the_empty_topology() {
    let snapshot = empty_wires_snapshot();
    assert_eq!(WiresInference::infer(&snapshot).expect("parent-only inference"), WiresInference::default());
    assert_eq!(WiresInference::default().topology, WiresTopology::default());
}

/// ⚖️ LAW: the composed topology counts the board the content child carries.
#[semio_framework_async_macros::async_test]
async fn topology_counts_the_composed_board_graph() {
    let composed = crate::wires_composed(&empty_wires_snapshot(), &chain_content());
    let topology = super::super::topology::compute_wires_topology(&composed.board);
    assert_eq!((topology.node_count, topology.edge_count, topology.cycle_free), (2, 1, true));
}
