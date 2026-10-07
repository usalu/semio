use crate::standards::v1::subsets::any::io::binary::snapshot::*;
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
async fn equation_pack_schema_identity_is_derived_and_retains_persisted_handles(){let snapshot=EquationSnapshot::default();store::os_store::test_support::assert_pack_schema_identity(&snapshot);let decoded=decode(&encode(&snapshot)).expect("decode");assert_eq!(decoded,snapshot);}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::op::EquationMutation;
    use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    // 🔐️ Owner-installing guard — a bare `ArtifactStore::new` refuses every `Apply` with
    // `edit history insertion requires its exact mutation retirement factory`.
    let mut store = crate::host::owned::new_equation_store(create_document_envelope("semio.equation/v1", "math-demo", EquationSnapshot::default(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None })], transaction: None }).await.expect("apply");
    let edit: &Edit<EquationMutation> = store.envelope().vcs.edits.last().expect("edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<EquationSnapshot, EquationMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}

/// 🪆️ The pack carries the parent-owned graph and point cloud with their derived handles; the derived child content itself is
/// never embedded (it is re-derived by genesis).
#[semio_framework_async_macros::async_test]
async fn the_parent_state_and_its_handles_survive_pack(){let mut graph=EquationGraph{algorithm:"dfs".into(),algorithm_seed:Some("c".into()),directed:false,..EquationGraph::default()};graph.nodes.truncate(3);let seeded=crate::equation_snapshot_with_state(&graph,&EquationGeometry{points:EquationGeometry::default().points.into_iter().take(2).collect()});for snapshot in[EquationSnapshot::default(),seeded]{let decoded=decode(&encode(&snapshot)).expect("decode");assert_eq!(decoded,snapshot);}}
