use crate::standards::v1::subsets::any::io::text::snapshot::*;

#[semio_framework_async_macros::async_test]
async fn math_projection_dsl_round_trips_default() {
    store::os_store::test_support::assert_dsl_round_trip(&EquationSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn example_primary_text_round_trips() {
    let text = include_str!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let parsed = parse_dsl(text).expect("parse example");
    store::os_store::test_support::assert_dsl_round_trip(&parsed);
}

#[semio_framework_async_macros::async_test]
async fn math_projection_dsl_round_trips_with_seed_and_empty_collections() {
    let mut graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
    graph.nodes.clear();
    graph.edges.clear();
    let projection = crate::equation_snapshot_with_state(&graph, &EquationGeometry { points: Vec::new() });
    store::os_store::test_support::assert_dsl_round_trip(&projection);
}

fn seeded_scene_snapshot() -> EquationSnapshot {
    let mut graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("b".into()), ..EquationGraph::default() };
    graph.nodes.truncate(2);
    graph.edges.retain(|edge| graph.nodes.iter().any(|node| node.id == edge.source) && graph.nodes.iter().any(|node| node.id == edge.target));
    crate::equation_snapshot_with_state(&graph, &EquationGeometry { points: EquationGeometry::default().points.into_iter().take(3).collect() })
}

/// 🪆️ The text carries the parent-owned graph and point cloud and the derived handles: a reparsed document is the authored one.
#[semio_framework_async_macros::async_test]
async fn the_parent_state_and_its_handles_survive_text() {
    for snapshot in [EquationSnapshot::default(), seeded_scene_snapshot()] {
        let reparsed = parse_dsl(&print_dsl(&snapshot)).expect("reparse");
        assert_eq!(reparsed, snapshot);
        assert_eq!((reparsed.notation.clone(), reparsed.results.clone(), reparsed.computed.clone()), crate::equation_children(&reparsed.graph, &reparsed.geometry));
    }
}

/// 🎬️ The curated asset is the default document: its graph, point cloud, derived handles and exact expression.
#[semio_framework_async_macros::async_test]
async fn the_demo_asset_is_the_default_document() {
    let parsed = parse_dsl(include_str!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("parse example");
    assert_eq!(parsed, EquationSnapshot::default());
}
/// 🧭️ Unknown root fields and foreign envelope identities are refused rather than ignored.
#[semio_framework_async_macros::async_test]
async fn unowned_scene_lines_and_foreign_identity_are_refused(){let printed=print_dsl(&EquationSnapshot::default());for line in["scene={}","board={}"]{assert!(parse_dsl(&format!("{printed}\n{line}")).is_err());}assert!(parse_dsl(&printed.replacen("mathematical.equation.dsl","stdio.json.dsl",1)).is_err());}
