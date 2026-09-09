use super::*;
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
    let envelope = store::create_document_envelope(crate::MATH_DOCUMENT_SCHEMA, "math-demo", initial, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let mutation = UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None };
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(mutation)], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
