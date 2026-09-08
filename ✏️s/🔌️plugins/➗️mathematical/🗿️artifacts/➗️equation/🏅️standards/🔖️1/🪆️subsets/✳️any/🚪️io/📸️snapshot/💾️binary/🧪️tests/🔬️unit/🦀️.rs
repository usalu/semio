
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
    let snapshot = crate::equation_snapshot_with_state(graph, EquationGeometry { points: Vec::new() });
    store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::EquationMutation;
    use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, ArtifactStore, create_document_envelope};

    let mut store: ArtifactStore<EquationSnapshot, EquationMutation> = ArtifactStore::new(create_document_envelope("semio.equation/v1", "math-demo", EquationSnapshot::default(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None })], description: None }).await.expect("apply");
    let edit: &Edit<EquationMutation> = store.envelope().vcs.edits.last().expect("edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<EquationSnapshot, EquationMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
