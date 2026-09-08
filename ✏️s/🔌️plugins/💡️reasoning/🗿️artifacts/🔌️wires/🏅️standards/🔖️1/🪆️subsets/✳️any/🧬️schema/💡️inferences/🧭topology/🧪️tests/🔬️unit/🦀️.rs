
use super::*;

fn board(nodes: &[&str], edges: &[(&str, &str)]) -> DslValue {
    DslValue::object([
        ("nodes".into(), DslValue::Array(nodes.iter().map(|id| DslValue::object([("id".into(), DslValue::String((*id).into()))])).collect())),
        ("edges".into(), DslValue::Array(edges.iter().map(|(source, target)| DslValue::object([("source".into(), DslValue::String((*source).into())), ("target".into(), DslValue::String((*target).into()))])).collect())),
    ])
}

#[semio_framework_async_macros::async_test]
async fn a_tree_is_cycle_free_with_one_component() {
    let topology = compute_wires_topology(&board(&["a", "b", "c"], &[("a", "b"), ("b", "c")]));
    assert_eq!(topology.node_count, 3);
    assert_eq!(topology.edge_count, 2);
    assert_eq!(topology.component_count, 1);
    assert!(topology.cycle_free);
}

#[semio_framework_async_macros::async_test]
async fn a_triangle_closes_a_cycle() {
    let topology = compute_wires_topology(&board(&["a", "b", "c"], &[("a", "b"), ("b", "c"), ("c", "a")]));
    assert!(!topology.cycle_free);
    assert_eq!(topology.component_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn disconnected_nodes_count_as_separate_components() {
    let topology = compute_wires_topology(&board(&["a", "b"], &[]));
    assert_eq!(topology.component_count, 2);
    assert_eq!(topology.edge_count, 0);
}
