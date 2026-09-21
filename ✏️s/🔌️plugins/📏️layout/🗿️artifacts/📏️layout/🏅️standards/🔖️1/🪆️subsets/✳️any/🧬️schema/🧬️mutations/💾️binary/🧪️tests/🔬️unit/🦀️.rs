use super::*;
use crate::mutations::rename_layout;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = LayoutMutation::RenameLayout(rename_layout::RenameLayout { new_name: "Renamed".into() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_binary_round_trips_a_store_with_applied_operations() {
    use crate::LAYOUT_DOCUMENT_SCHEMA;

    let initial = crate::standards::v1::subsets::any::schema::default_document();
    let envelope: super::LayoutEnvelope = store::create_document_envelope(LAYOUT_DOCUMENT_SCHEMA, "layout-doc-binary-test", initial, None);
    let mut doc_store = super::new_layout_store(envelope).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![LayoutMutation::RenameLayout(rename_layout::RenameLayout { new_name: "Renamed".into() })], description: Some("rename document".into()) }).await.expect("apply rename");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store.0).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store.0).await;
    store::os_store::test_support::assert_live_equals_replay(&doc_store.0).await;
}
