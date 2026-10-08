use super::*;
use crate::EquationPoint;

/// ⚖️ LAW: a state diff carries the sparse slots only, and applying it re-derives the handles — exactly the snapshot built from the new state.
#[semio_framework_async_macros::async_test]
async fn a_state_diff_round_trips_through_apply() {
    let base = EquationSnapshot::default();
    let diff = EquationDiff { algorithm: Some("components".into()), ..Default::default() };
    let applied = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    let mut graph = base.graph.clone();
    graph.algorithm = "components".into();
    assert_eq!((applied.graph.algorithm.as_str(), &applied.geometry), ("components", &base.geometry));
    assert_eq!(applied, crate::equation_snapshot_with_state(&graph, &base.geometry), "the handles are the derivation of the new state");
    assert_eq!(applied.results, base.results, "a derived output whose content did not change keeps its address");
}

/// ⚖️ LAW: absorb keeps the incoming slots and the earlier ones it does not replace.
#[semio_framework_async_macros::async_test]
async fn absorb_prefers_the_incoming_slots_when_present() {
    let mut first = EquationDiff { directed: Some(true), algorithm: Some("bfs".into()), ..Default::default() };
    first.absorb(EquationDiff { algorithm: Some("components".into()), ..Default::default() });
    assert_eq!(first.directed, Some(true));
    assert_eq!(first.algorithm.as_deref(), Some("components"));
}

/// ⚖️ LAW: positional point edits fold adjacent same-index edits and the negative diff restores the cloud.
#[semio_framework_async_macros::async_test]
async fn point_edits_fold_and_invert() {
    let base = EquationSnapshot::default();
    let point = |x: f64| EquationPoint { x, y: x };
    let insert = EquationDiff { points: Some(EquationPointsDelta { edits: vec![EquationPointEdit::Insert { at: 2, point: point(9.0) }] }), ..Default::default() };
    let mut folded = insert.clone();
    folded.absorb(EquationDiff { points: Some(EquationPointsDelta { edits: vec![EquationPointEdit::Remove { at: 2 }] }), ..Default::default() });
    assert!(folded.points.is_none_or(|delta| delta.edits.is_empty()), "insert then remove of one index cancels");
    let inserted = protocol::apply_diff(&insert, &base).expect("insert applies");
    assert_eq!(inserted.geometry.points[2], point(9.0));
    let restored = protocol::apply_diff(&protocol::DiffAlgebra::inverse(&insert, &base), &inserted).expect("negative diff applies");
    assert_eq!(restored, base);
}

/// ⚖️ LAW: a positional node delta inserts, moves and removes at middle rows, sums, and inverts row by row.
#[semio_framework_async_macros::async_test]
async fn positional_node_delta_sums_and_inverts_at_middle_rows() {
    let node = |id: &str| crate::EquationNode { id: id.into(), label: id.into(), x: 0.0, y: 0.0 };
    let mut base = EquationSnapshot::default();
    base.graph.nodes = vec![node("a"), node("b"), node("c")];
    let order = |snapshot: &EquationSnapshot| snapshot.graph.nodes.iter().map(|row| row.id.clone()).collect::<Vec<_>>();
    let diff = |delta: EquationNodesDelta| EquationDiff { nodes: Some(delta), ..Default::default() };
    let insert = diff(EquationNodesDelta::insertion(1, node("x")));
    let inserted = protocol::apply_diff(&insert, &base).expect("valid insertion");
    let remove = diff(EquationNodesDelta::removal(&inserted.graph.nodes, 3));
    let removed = protocol::apply_diff(&remove, &inserted).expect("valid removal");
    let relocate = diff(EquationNodesDelta::relocation(&removed.graph.nodes, 0, 2));
    let moved = protocol::apply_diff(&relocate, &removed).expect("valid relocation");
    assert_eq!(order(&moved), ["x", "b", "a"]);
    let mut sum = insert;
    sum.absorb(remove);
    sum.absorb(relocate);
    assert_eq!(protocol::apply_diff(&sum, &base).expect("valid sum"), moved);
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&sum, &base), &moved).expect("valid inverse"), base);
}
