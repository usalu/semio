use crate::binary::mutations::*;
use semio_s_artifact_norm_din4108_mut_building::change_airtightness_n50::ChangeAirtightnessN50;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let mutation = Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 1.2 });
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let bytes = encode_op(&mutation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), mutation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_through_store() {
    let envelope = store::os_store::document_materialize::create_document_envelope("norm.din4108/v1", "din4108", Din4108Snapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::os_store::test_support::plain_test_store(envelope).await;
    let mutation = Din4108Mutation::ChangeAirtightnessN50(ChangeAirtightnessN50 { new_airtightness_n50: 1.2 });
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
