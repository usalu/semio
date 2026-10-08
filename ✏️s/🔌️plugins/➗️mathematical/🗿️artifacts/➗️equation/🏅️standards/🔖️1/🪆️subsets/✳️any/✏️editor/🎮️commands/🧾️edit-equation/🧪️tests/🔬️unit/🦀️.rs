use super::*;
use crate::editor::equation::unit_tests::context::{dispatch, math_app};
use crate::editor::equation::EquationCommand;
use crate::standards::v1::subsets::any::io::text::snapshot::math_graph_to_dsl;
use crate::{EquationEdge, EquationNode};

fn node(id: &str, label: &str, x: f64, y: f64) -> EquationNode {
    EquationNode { id: id.into(), label: label.into(), x, y }
}

fn edge(id: &str, source: &str, target: &str) -> EquationEdge {
    EquationEdge { id: id.into(), source: source.into(), target: target.into() }
}

#[semio_framework_async_macros::async_test]
async fn edit_equation_applies_the_edited_graph_and_geometry() {
    let mut app = math_app().await;
    let geometry = EquationGeometry { points: vec![EquationPoint { x: 1.0, y: 2.0 }] };
    let graph = EquationGraph { algorithm: "components".into(), ..Default::default() };
    dispatch(&mut app, EquationCommand::EditEquation(EditEquation { graph: math_graph_to_dsl(&graph), geometry: geometry.clone() })).await;
    let projection = app.snapshot().expect("projection");
    assert_eq!(projection.graph.algorithm, "components");
    assert_eq!(projection.geometry.clone(), geometry);
}

#[test]
fn graph_edits_are_the_concrete_kinds_of_each_changed_field_with_exact_positions() {
    let base = EquationGraph { directed: false, nodes: vec![node("a", "A", 0.0, 0.0), node("b", "B", 1.0, 0.0), node("c", "C", 2.0, 0.0)], edges: vec![edge("e0", "a", "b"), edge("e1", "b", "c")], algorithm: "none".into(), algorithm_seed: None };
    let next = EquationGraph { directed: true, nodes: vec![node("a", "A2", 0.0, 0.0), node("n", "N", 5.0, 5.0), node("c", "C", 3.0, 1.0)], edges: vec![edge("e2", "n", "c")], algorithm: "none".into(), algorithm_seed: None };
    let kinds: Vec<&str> = equation_graph_edit_leaves(&base, &next).iter().map(|leaf| protocol::SemanticMutation::semantics(leaf).kind).collect();
    assert_eq!(kinds, ["change-graph-directed", "disconnect-nodes", "disconnect-nodes", "delete-node", "create-node", "change-node-label", "set-node-positions", "connect-nodes"]);
}

#[test]
fn point_edits_reposition_append_at_the_index_and_drop_the_tail_last_first() {
    let point = |x: f64| EquationPoint { x, y: 0.0 };
    let kinds = |base: &[EquationPoint], next: &[EquationPoint]| -> Vec<&'static str> { equation_point_edit_leaves(base, next).iter().map(|leaf| protocol::SemanticMutation::semantics(leaf).kind).collect() };
    assert_eq!(kinds(&[point(0.0)], &[point(1.0), point(2.0)]), ["set-point-positions", "insert-point"]);
    assert_eq!(kinds(&[point(0.0), point(1.0), point(2.0)], &[point(0.0)]), ["remove-point", "remove-point"]);
    assert!(kinds(&[point(0.0)], &[point(0.0)]).is_empty());
}
