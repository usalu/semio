use super::*;
use crate::{EquationGeometry, EquationGraph, EquationPoint};

/// ⚖️ LAW: a state diff carries the sparse slots and the re-minted derived handles, and applies to exactly the snapshot built from the new state.
#[semio_framework_async_macros::async_test]
async fn a_state_diff_round_trips_through_apply() {
    let base = EquationSnapshot::default();
    let diff = crate::equation_state_diff(EquationDiff { algorithm: Some("components".into()), ..Default::default() }, &base);
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
    let (notation_a, _, _) = crate::equation_children(&EquationGraph::default(), &EquationGeometry::default());
    let mut first = EquationDiff { notation: Some(notation_a), ..Default::default() };
    let (_, results_b, _) = crate::equation_children(&EquationGraph::default(), &EquationGeometry { points: Vec::new() });
    let second = EquationDiff { results: Some(results_b.clone()), ..Default::default() };
    first.absorb(second);
    assert!(first.notation.is_some());
    assert_eq!(first.results, Some(results_b));
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
