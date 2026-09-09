use super::*;

#[semio_framework_async_macros::async_test]
async fn math_projection_dsl_round_trips_default() {
    store::os_store::test_support::assert_dsl_round_trip(&EquationSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn example_primary_text_round_trips() {
    let text = include_str!("../../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
    let parsed = parse_dsl(text).expect("parse example");
    store::os_store::test_support::assert_dsl_round_trip(&parsed);
}

#[semio_framework_async_macros::async_test]
async fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
    let mut graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
    graph.nodes.clear();
    graph.edges.clear();
    let projection = crate::equation_snapshot_with_state(graph, EquationGeometry { points: Vec::new() });
    store::os_store::test_support::assert_dsl_round_trip(&projection);
}
