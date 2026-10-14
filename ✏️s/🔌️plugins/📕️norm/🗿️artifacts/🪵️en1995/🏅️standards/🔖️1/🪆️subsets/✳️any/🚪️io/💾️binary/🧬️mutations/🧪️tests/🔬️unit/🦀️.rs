use crate::binary::mutations::*;
use semio_s_artifact_norm_en1995_mut_members::change_annex;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;

fn sample_mutation() -> En1995Mutation {
    En1995Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: semio_s_plugin_norm_compliance::AnnexChoice::En })
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let mutation = sample_mutation();
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let bytes = encode_op(&mutation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), mutation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_through_store() {
    let envelope = store::os_store::document_materialize::create_document_envelope("norm.en1995/v1", "en1995", En1995Snapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    // 🏪️ A bare `ArtifactStore::new` installs NO document-store owners: the first `Apply` is refused
    // with `edit history insertion requires its exact mutation retirement factory`, and a store that
    // survived would then trip the terminal-empty shallow-shell witness in `Drop`.
    let mut store = store::os_store::test_support::plain_test_store(envelope).await;
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![sample_mutation()], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store::os_store::test_support::close_plain_test_store(&mut store);
}
