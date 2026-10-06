use super::*;
use crate::{EquationGeometry, EquationGraph};

/// ⚖️ LAW: a state diff carries the new graph and geometry with their re-minted derived handles, and applies to exactly them.
#[semio_framework_async_macros::async_test]
async fn a_state_diff_round_trips_through_apply() {
    let base = EquationSnapshot::default();
    let mut graph = base.graph.clone();
    graph.algorithm = "components".into();
    let applied = crate::equation_state_diff(graph.clone(), base.geometry.clone()).apply(&base).expect("valid mutation diff");
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
