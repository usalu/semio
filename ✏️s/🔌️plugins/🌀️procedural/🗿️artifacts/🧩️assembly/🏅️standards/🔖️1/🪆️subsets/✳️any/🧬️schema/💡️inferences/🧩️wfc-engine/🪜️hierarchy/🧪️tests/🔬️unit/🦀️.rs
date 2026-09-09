use super::*;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::topology::{GraphTopology, GraphTopologyBuilder};

fn checkerboard(n: usize) -> (CompiledModel, GraphTopology) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    for i in 0..n.saturating_sub(1) {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
    }
    (model, tb.build().unwrap())
}

fn single_node_model(pattern_count: usize) -> (CompiledModel, GraphTopology) {
    let mut b = ModelBuilder::new();
    for _ in 0..pattern_count {
        b.add_pattern(1.0);
    }
    b.add_relation("r");
    (b.compile().unwrap(), GraphTopologyBuilder::new(1).build().unwrap())
}

fn always_satisfiable_child(_node: NodeId, _pattern: PatternId) -> (CompiledModel, GraphTopology, SearchConfig) {
    let (model, topo) = single_node_model(2);
    (model, topo, SearchConfig::default())
}

#[test]
fn solves_macro_then_every_child() {
    let (model, topo) = checkerboard(4);
    let config = SearchConfig::default();
    match solve_hierarchy(&model, &topo, &config, 1, always_satisfiable_child) {
        HierarchyOutcome::Solved { macro_solution, children } => {
            assert_eq!(macro_solution.assignment.len(), 4);
            assert_eq!(children.len(), 4);
            for child in &children {
                assert_eq!(child.assignment.len(), 1);
            }
        }
        HierarchyOutcome::MacroUnsatisfiable => panic!("expected Solved, got MacroUnsatisfiable"),
        HierarchyOutcome::ChildFailed { node } => panic!("expected Solved, got ChildFailed at {node}"),
    }
}

#[test]
fn reports_macro_unsatisfiable_without_attempting_children() {
    // K5 with only 4 colors: unsatisfiable regardless of any child model.
    let mut b = ModelBuilder::new();
    let patterns: Vec<_> = (0..4).map(|_| b.add_pattern(1.0)).collect();
    let ne = b.add_relation("ne");
    for &a in &patterns {
        for &c in &patterns {
            if a != c {
                b.allow(ne, a, c);
            }
        }
    }
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(5);
    for i in 0..5 {
        for j in (i + 1)..5 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(j), ne);
            tb.arc(NodeId::from_index(j), NodeId::from_index(i), ne);
        }
    }
    let topo = tb.build().unwrap();
    let config = SearchConfig::default();

    let never_called = |_: NodeId, _: PatternId| -> (CompiledModel, GraphTopology, SearchConfig) { panic!("child should never be attempted") };
    let outcome = solve_hierarchy(&model, &topo, &config, 1, never_called);
    assert!(matches!(outcome, HierarchyOutcome::MacroUnsatisfiable));
}

/// 🧪️ A two-node model whose single relation allows nothing at all — any arc using it is
/// unsatisfiable, regardless of pattern count.
fn unsatisfiable_child(_node: NodeId, _pattern: PatternId) -> (CompiledModel, GraphTopology, SearchConfig) {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let never = b.add_relation("never");
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(2);
    tb.arc(NodeId(0), NodeId(1), never);
    tb.arc(NodeId(1), NodeId(0), never);
    (model, tb.build().unwrap(), SearchConfig::default())
}

#[test]
fn reports_which_node_child_failed_at() {
    let (model, topo) = checkerboard(3);
    let config = SearchConfig::default();
    let child_model_for = |node: NodeId, pattern: PatternId| -> (CompiledModel, GraphTopology, SearchConfig) {
        if node == NodeId(1) {
            unsatisfiable_child(node, pattern)
        } else {
            always_satisfiable_child(node, pattern)
        }
    };

    let outcome = solve_hierarchy(&model, &topo, &config, 1, child_model_for);
    assert!(matches!(outcome, HierarchyOutcome::ChildFailed { node: NodeId(1) }));
}

#[test]
fn child_seeds_differ_by_node_and_reproduce_deterministically() {
    assert_eq!(child_seed(1, NodeId(0)), child_seed(1, NodeId(0)));
    assert_ne!(child_seed(1, NodeId(0)), child_seed(1, NodeId(1)));
    assert_ne!(child_seed(1, NodeId(0)), child_seed(2, NodeId(0)));
}
