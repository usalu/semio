//! ⚖️ Layout app — binary command protocol surface + laws (constitutional: protocol).

use layout_op::LayoutOperation;
use protocol::OpBinary;

/// 📦 Encodes a `LayoutOperation` to its binary command form.
pub fn encode_op(operation: &LayoutOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `LayoutOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<LayoutOperation, protocol::ProtocolError> {
    LayoutOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use layout::PagePatch;
    use protocol::CollectionOperation;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = layout_engine::default_document();
        let page_id = document.pages[0].id.clone();
        let operation = LayoutOperation::Pages(CollectionOperation::Patch { id: page_id, patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_applied_operations() {
        use layout::LAYOUT_FIXTURE_SCHEMA;

        let initial = layout_engine::default_document();
        let envelope = store::create_document_envelope(LAYOUT_FIXTURE_SCHEMA, "layout-doc-text-test", initial, None);
        let mut doc_store: store::DocumentStore<layout::LayoutDocument, LayoutOperation> = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { width: Some(640.0), ..Default::default() } })],
                description: Some("resize page".into()),
            })
            .expect("apply patch page width");
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![LayoutOperation::Pages(CollectionOperation::Patch { id: "page-1".into(), patch: PagePatch { name: Some("Renamed".into()), ..Default::default() } })],
                description: Some("rename page".into()),
            })
            .expect("apply patch page");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
        store::test_support::assert_live_equals_replay(&doc_store);
    }
}
//#endregion 🧪Tests
