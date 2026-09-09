use super::*;
use protocol::Inference;

#[test]
fn inference_determinism_law() {
    let snapshot = PlaygroundSnapshot::default();
    assert_eq!(PlaygroundInference::infer(&snapshot), PlaygroundInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(PlaygroundInference::infer(&PlaygroundSnapshot::default()), PlaygroundInference::default());
}

#[test]
fn topology_is_the_vacuous_empty_graph() {
    let topology = infer_topology(&PlaygroundSnapshot::default());
    assert!(topology.topo_order.is_empty());
    assert!(topology.depth.is_empty());
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 0);
}
