use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::EquationSnapshot;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    use crate::standards::v1::subsets::graph::schema::mutations::change_graph_directed::ChangeGraphDirected;
    let operation = EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn math_document_text_round_trips_through_store() {
    use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
    let initial = EquationSnapshot::default();
    let envelope = store::os_store::document_materialize::create_document_envelope(crate::MATH_DOCUMENT_SCHEMA, "math-demo", initial, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    // 🔐️ Owner-installing guard (`🚪️io/💾️binary/📸️snapshot`) — a bare `ArtifactStore::new` refuses
    // every `Apply` with `edit history insertion requires its exact mutation retirement factory`.
    let mut store = crate::host::owned::new_equation_store(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    let mutation = UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None };
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(mutation)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
