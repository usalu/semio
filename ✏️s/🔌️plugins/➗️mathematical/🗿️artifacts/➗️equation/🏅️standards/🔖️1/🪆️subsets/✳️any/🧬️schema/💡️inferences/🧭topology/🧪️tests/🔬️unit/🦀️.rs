use super::*;
use crate::{EquationEdge, EquationNode};

fn node(id: &str) -> EquationNode {
    EquationNode { id: id.into(), label: id.into(), x: 0.0, y: 0.0 }
}

fn edge(id: &str, source: &str, target: &str) -> EquationEdge {
    EquationEdge { id: id.into(), source: source.into(), target: target.into() }
}

fn graph(nodes: Vec<EquationNode>, edges: Vec<EquationEdge>) -> EquationGraph {
    EquationGraph { directed: true, nodes, edges, algorithm: "topo".into(), algorithm_seed: None }
}

//#region 🧪️TopologyLaws
#[semio_framework_async_macros::async_test]
async fn a_direct_cycle_between_two_nodes_is_reported() {
    let g = graph(vec![node("a"), node("b")], vec![edge("e1", "a", "b"), edge("e2", "b", "a")]);
    let topology = compute_equation_topology(&g);
    assert!(!topology.cycle_free, "a->b->a is a genuine cycle");
}

#[semio_framework_async_macros::async_test]
async fn an_edge_to_a_missing_node_is_dropped_not_a_cycle() {
    let g = graph(vec![node("a")], vec![edge("e1", "a", "missing")]);
    let topology = compute_equation_topology(&g);
    assert!(topology.cycle_free);
    assert_eq!(topology.topo_order, vec!["a"]);
}

#[semio_framework_async_macros::async_test]
async fn an_undirected_display_flag_still_uses_the_edge_data_source_target() {
    let mut g = graph(vec![node("a"), node("b")], vec![edge("e1", "a", "b")]);
    g.directed = false;
    let topology = compute_equation_topology(&g);
    assert!(topology.cycle_free);
    assert_eq!(topology.depth["b"], 1);
}
//#endregion 🧪️TopologyLaws
