use crate::standards::v1::subsets::any::io::component::binary::mutations::*;
use semio_s_artifact_lowpoly_lowpoly_mut_objects::rename_object;
use semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let projection = default_snapshot();
    let object_id = projection.objects[0].id.clone();
    let operation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed".into() });
    semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_after_applying_an_operation() {
    let projection = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json(r#"{"vertices":[],"faces":[]}"#, "obj-1", "Fixture");
    let object_id = projection.objects[0].id.clone();
    let envelope = store::os_store::document_materialize::create_document_envelope::<semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot, LowpolyMutation>(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    // 🏪️ A bare store carries no owner catalog and refuses its first edit (`edit history insertion
    // requires its exact mutation retirement factory`); install the exact owners production installs
    // through `LowpolyPlayApp::build_document_store_owners`, and close them before drop.
    doc_store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot, LowpolyMutation>());
    let operation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed Layer".into() });
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![operation], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    while !doc_store.close_owned_terminal_is_empty() {
        doc_store.close_owned_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("lowpoly document store closes through its exact bounded owners");
    }
}
