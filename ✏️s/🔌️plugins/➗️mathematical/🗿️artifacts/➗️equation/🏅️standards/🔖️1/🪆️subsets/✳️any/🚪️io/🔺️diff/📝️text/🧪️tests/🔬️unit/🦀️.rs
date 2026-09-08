
use super::*;

#[semio_framework_async_macros::async_test]
async fn diff_from_state_round_trips_through_apply() {
    // 🔎️ `notation`/`results`/`computed` are three co-derived projections of the SAME
    // `(graph, geometry)` pair — a graph-scoped change regenerates all three handles, unlike the
    // old per-slot ("graph slot only") isolation this test named before the migration.
    let base = EquationSnapshot::default();
    let mut graph = crate::equation_graph(&base);
    graph.algorithm = "components".into();
    let geometry = crate::equation_geometry(&base);
    let diff = diff_from_state(&graph, &geometry);
    let applied = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(crate::equation_graph(&applied).algorithm, "components");
    assert_eq!(crate::equation_geometry(&applied), geometry);
}

#[semio_framework_async_macros::async_test]
async fn absorb_prefers_the_incoming_slots_when_present() {
    let (notation_a, _, _) = equation_children_from_state(&EquationGraph::default(), &EquationGeometry::default());
    let mut first = EquationDiff { notation: Some(notation_a), ..Default::default() };
    let (_, results_b, _) = equation_children_from_state(&EquationGraph::default(), &EquationGeometry { points: Vec::new() });
    let second = EquationDiff { results: Some(results_b.clone()), ..Default::default() };
    first.absorb(second);
    assert!(first.notation.is_some());
    assert_eq!(first.results, Some(results_b));
}
