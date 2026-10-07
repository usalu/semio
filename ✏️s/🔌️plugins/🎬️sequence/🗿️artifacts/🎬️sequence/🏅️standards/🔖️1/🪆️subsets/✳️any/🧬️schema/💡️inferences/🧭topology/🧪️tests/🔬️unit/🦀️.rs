use super::*;
use crate::{SequenceEdge, SequenceStep, StepParams};

fn step(id: &str) -> SequenceStep {
    SequenceStep { id: id.into(), kind: "state.set".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }
}

fn edge(id: &str, from: &str, to: &str) -> SequenceEdge {
    SequenceEdge { id: id.into(), from: from.into(), to: to.into() }
}

fn topology_of(steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>) -> SequenceTopology {
    let steps = neural_engine::ColdOwner::new(steps);
    compute_scene_topology(&steps, &edges)
}

/// 🌱️ LAW (audit F7): a parent whose `content` child is not derivable here — decoded, reloaded, remote — infers the empty
/// topology instead of trapping; the genesis document infers its derivable two-step chain.
#[semio_framework_async_macros::async_test]
async fn a_non_derivable_parent_infers_the_empty_topology_and_genesis_its_chain() {
    let wire_only = SequenceSnapshot { schema: crate::SEQUENCE_DOCUMENT_SCHEMA.into(), content: store::ArtifactChild::new("foreign-content".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "foreign-content".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() } }) };
    let empty = compute_sequence_topology(&wire_only);
    assert_eq!((empty.node_count, empty.cycle_free, empty.topo_order.len()), (0, true, 0));
    let genesis = crate::snapshot::schema::default_persisted_snapshot();
    assert_eq!(compute_sequence_topology(&genesis).topo_order, vec!["step-1".to_string(), "step-2".to_string()]);
    neural_engine::ColdRetire::retire_cold(wire_only);
    neural_engine::ColdRetire::retire_cold(genesis);
}

#[semio_framework_async_macros::async_test]
async fn linear_chain_orders_by_dependency_and_depth_by_distance_from_root() {
    let topology = topology_of(vec![step("a"), step("b"), step("c")], vec![edge("e1", "a", "b"), edge("e2", "b", "c")]);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&1));
    assert_eq!(topology.depth.get("c"), Some(&2));
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total() {
    let topology = topology_of(vec![step("a"), step("b")], vec![edge("e1", "a", "b"), edge("e2", "b", "a")]);
    assert!(!topology.cycle_free);
    assert_eq!(topology.node_count, 2);
    assert_eq!(topology.topo_order.len(), 2, "still covers every step even though it is not a valid topological order");
}

#[semio_framework_async_macros::async_test]
async fn a_dangling_edge_is_ignored() {
    let topology = topology_of(vec![step("a")], vec![edge("e1", "a", "missing")]);
    assert!(topology.cycle_free);
    assert_eq!(topology.depth.get("a"), Some(&0));
}

#[semio_framework_async_macros::async_test]
async fn diamond_depth_takes_the_longest_incoming_path() {
    // a -> b -> d, a -> c -> d: d's depth must be 2 (via either b or c), not 1.
    let topology = topology_of(vec![step("a"), step("b"), step("c"), step("d")], vec![edge("e1", "a", "b"), edge("e2", "a", "c"), edge("e3", "b", "d"), edge("e4", "c", "d")]);
    assert_eq!(topology.depth.get("d"), Some(&2));
    assert!(topology.cycle_free);
}
