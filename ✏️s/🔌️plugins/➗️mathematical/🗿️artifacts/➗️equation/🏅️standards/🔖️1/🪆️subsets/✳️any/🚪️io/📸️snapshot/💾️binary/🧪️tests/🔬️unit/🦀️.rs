use super::*;
use crate::{EquationGeometry, EquationGraph};

#[semio_framework_async_macros::async_test]
async fn equation_snapshot_dsl_pack_equivalence_default() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&EquationSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn equation_snapshot_dsl_pack_equivalence_with_seed_and_empty_collections() {
    let mut graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
    graph.nodes.clear();
    graph.edges.clear();
    let snapshot = crate::equation_snapshot_with_state(&graph, &EquationGeometry { points: Vec::new() });
    store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::EquationMutation;
    use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    // 🔐️ Owner-installing guard — a bare `ArtifactStore::new` refuses every `Apply` with
    // `edit history insertion requires its exact mutation retirement factory`.
    let mut store = super::new_equation_store(create_document_envelope("semio.equation/v1", "math-demo", EquationSnapshot::default(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None })], description: None }).await.expect("apply");
    let edit: &Edit<EquationMutation> = store.envelope().vcs.edits.last().expect("edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<EquationSnapshot, EquationMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}

/// 🕸️ The pack twin of the text codec's scene law: `assert_dsl_pack_equivalence` compares snapshots,
/// whose child handles are blind to the `EquationWorkingScene` owner, so the scene is compared directly.
#[semio_framework_async_macros::async_test]
async fn the_scene_survives_a_pack_round_trip() {
    let mut graph = EquationGraph { algorithm: "dfs".into(), algorithm_seed: Some("c".into()), directed: false, ..EquationGraph::default() };
    graph.nodes.truncate(3);
    let seeded = crate::equation_snapshot_with_state(&graph, &EquationGeometry { points: EquationGeometry::default().points.into_iter().take(2).collect() });
    for snapshot in [EquationSnapshot::default(), seeded] {
        let decoded = decode(&encode(&snapshot)).expect("decode");
        let (before, after) = (crate::equation_scene(&snapshot), crate::equation_scene(&decoded));
        assert_eq!(after.graph, before.graph);
        assert_eq!(after.geometry, before.geometry);
        assert!(crate::equation_scene_owner(&decoded).is_some(), "a decoded pack owns its scene");
    }
}
