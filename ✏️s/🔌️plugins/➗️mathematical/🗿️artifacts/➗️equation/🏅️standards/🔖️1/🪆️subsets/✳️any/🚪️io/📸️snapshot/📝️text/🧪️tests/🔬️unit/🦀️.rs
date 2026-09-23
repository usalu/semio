use super::*;

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

/// 🕸️ The composed children's scene is the document's content, so `print_dsl`/`parse_dsl` must carry
/// it: `assert_dsl_round_trip` compares the snapshot, whose child handles compare by identity only and
/// are blind to the `EquationWorkingScene` owner behind them (ticket 26/09/19, `📓️knowledge.md` §4, §13).
#[semio_framework_async_macros::async_test]
async fn the_scene_survives_a_text_round_trip() {
    for snapshot in [EquationSnapshot::default(), seeded_scene_snapshot()] {
        let reparsed = parse_dsl(&print_dsl(&snapshot)).expect("reparse");
        let (before, after) = (crate::equation_scene(&snapshot), crate::equation_scene(&reparsed));
        assert_eq!(after.graph, before.graph);
        assert_eq!(after.geometry, before.geometry);
        assert!(crate::equation_scene_owner(&reparsed).is_some(), "a parsed document owns its scene");
    }
}

/// 🎬️ The curated `demo` example is what `setActiveExample("demo")` loads into the play pane, so it
/// must carry the default graph and geometry the Graph and Geometry windows draw.
#[semio_framework_async_macros::async_test]
async fn the_demo_asset_carries_the_default_scene() {
    let parsed = parse_dsl(include_str!("../../../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("parse example");
    let scene = crate::equation_scene(&parsed);
    assert_eq!(scene.graph, EquationGraph::default());
    assert_eq!(scene.geometry, EquationGeometry::default());
}

/// 📖️ `graph=` and `geometry=` are required lines of `📖️.grammar.semio`: a body that names the three
/// children without their content is not a document of this format.
#[semio_framework_async_macros::async_test]
async fn a_body_without_its_scene_lines_is_refused() {
    let printed = print_dsl(&EquationSnapshot::default());
    for line in ["graph=", "geometry="] {
        let stripped = printed.lines().filter(|row| !row.starts_with(line)).collect::<Vec<_>>().join("\n");
        assert!(parse_dsl(&stripped).is_err(), "a body without its {line} line must be refused");
    }
}
