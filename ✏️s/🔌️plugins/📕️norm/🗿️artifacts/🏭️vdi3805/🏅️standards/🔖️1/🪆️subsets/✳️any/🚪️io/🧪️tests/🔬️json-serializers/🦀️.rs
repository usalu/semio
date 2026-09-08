
use super::*;

#[semio_framework_async_macros::async_test]
async fn catalog_and_document_json_round_trip() {
    let doc = Vdi3805Snapshot::default();
    let json = catalog_to_json(&doc.catalog).expect("to_json");
    let restored = catalog_from_json(&json).expect("from_json");
    assert_eq!(restored.products.len(), doc.catalog.products.len());
    assert!(catalog_from_json("not json").is_err());

    let doc_json = document_to_json(&doc).expect("doc to_json");
    let restored_doc = document_from_json(&doc_json).expect("doc from_json");
    assert_eq!(restored_doc.strict_mode, doc.strict_mode);
    assert!(document_from_json("not json").is_err());
}
