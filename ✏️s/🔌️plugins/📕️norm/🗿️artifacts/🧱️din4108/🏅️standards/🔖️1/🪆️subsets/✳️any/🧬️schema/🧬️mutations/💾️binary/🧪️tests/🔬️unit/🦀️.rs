
use super::*;
use crate::Din4108Snapshot;
use crate::document_schema::mutations::change_airtightness_n50::ChangeAirtightnessN50;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let mutation = Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 1.2 });
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let bytes = encode_op(&mutation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), mutation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_through_store() {
    let envelope = store::create_document_envelope("norm.din4108/v1", "din4108", Din4108Snapshot::default(), None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let mutation = Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 1.2 });
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
