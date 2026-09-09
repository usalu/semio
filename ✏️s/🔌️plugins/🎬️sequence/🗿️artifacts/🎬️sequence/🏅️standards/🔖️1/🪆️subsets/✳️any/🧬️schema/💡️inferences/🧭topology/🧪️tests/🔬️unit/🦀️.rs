use super::*;
use crate::{SequenceEdge, SequenceFixture, SequenceStep, StepParams};

fn step(id: &str) -> SequenceStep {
    SequenceStep { id: id.into(), kind: "state.set".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }
}

fn edge(id: &str, from: &str, to: &str) -> SequenceEdge {
    SequenceEdge { id: id.into(), from: from.into(), to: to.into() }
}

fn snapshot_from(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> neural_engine::ColdOwner<SequenceSnapshot> {
    neural_engine::ColdOwner::new(SequenceSnapshot::from_fixture(SequenceFixture { schema: crate::SEQUENCE_DOCUMENT_SCHEMA.into(), steps, edges }))
}

#[semio_framework_async_macros::async_test]
async fn linear_chain_orders_by_dependency_and_depth_by_distance_from_root() {
    let snapshot = snapshot_from(vec![step("a"), step("b"), step("c")], vec![edge("e1", "a", "b"), edge("e2", "b", "c")]);
    let topology = compute_sequence_topology(&snapshot);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&1));
    assert_eq!(topology.depth.get("c"), Some(&2));
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total() {
    let snapshot = snapshot_from(vec![step("a"), step("b")], vec![edge("e1", "a", "b"), edge("e2", "b", "a")]);
    let topology = compute_sequence_topology(&snapshot);
    assert!(!topology.cycle_free);
    assert_eq!(topology.node_count, 2);
    assert_eq!(topology.topo_order.len(), 2, "still covers every step even though it is not a valid topological order");
}

#[semio_framework_async_macros::async_test]
async fn a_dangling_edge_is_ignored() {
    let snapshot = snapshot_from(vec![step("a")], vec![edge("e1", "a", "missing")]);
    let topology = compute_sequence_topology(&snapshot);
    assert!(topology.cycle_free);
    assert_eq!(topology.depth.get("a"), Some(&0));
}

#[semio_framework_async_macros::async_test]
async fn diamond_depth_takes_the_longest_incoming_path() {
    // a -> b -> d, a -> c -> d: d's depth must be 2 (via either b or c), not 1.
    let snapshot = snapshot_from(vec![step("a"), step("b"), step("c"), step("d")], vec![edge("e1", "a", "b"), edge("e2", "a", "c"), edge("e3", "b", "d"), edge("e4", "c", "d")]);
    let topology = compute_sequence_topology(&snapshot);
    assert_eq!(topology.depth.get("d"), Some(&2));
    assert!(topology.cycle_free);
}
