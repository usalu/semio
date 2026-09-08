
use super::*;
use crate::LOWPOLY_DOCUMENT_SCHEMA;
use crate::mutations::rename_object;
use crate::schema::default_snapshot;

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
    let projection = default_snapshot();
    let object_id = projection.objects[0].id.clone();
    let envelope = store::create_document_envelope::<crate::LowpolySnapshot, LowpolyMutation>(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let operation = LowpolyMutation::RenameObject(rename_object::RenameObject { id: object_id, new_name: "Renamed Layer".into() });
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
    semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}
