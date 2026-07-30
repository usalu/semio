//! ⚖️ Draw app — binary command protocol surface + laws (constitutional: protocol).

use draw_op::DrawOperation;
use protocol::OpBinary;

/// 📦 Encodes a `DrawOperation` to its binary command form.
pub fn encode_op(operation: &DrawOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `DrawOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DrawOperation, protocol::ProtocolError> {
    DrawOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw::{DrawDocument, DRAW_DOCUMENT_SCHEMA};
    use draw_engine::{create_draw_shape_layer_rect, default_draw_document, layer_id};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = default_draw_document("doc-text-test", None);
        let operation = DrawOperation::SetCamera { camera: document.camera };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_an_applied_operation() {
        let initial = default_draw_document("doc-text-test", None);
        let envelope = store::create_document_envelope::<DrawDocument, DrawOperation>(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        doc_store
            .dispatch(store::DocumentCommand::Apply { operations: vec![DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) }], description: Some("add rect".into()) })
            .expect("apply add layer");
        doc_store
            .dispatch(store::DocumentCommand::Apply { operations: vec![DrawOperation::SetLayerOpacity { layer_id: layer_id_value, opacity: 0.5 }], description: Some("set opacity".into()) })
            .expect("apply set opacity");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
        store::test_support::assert_live_equals_replay(&doc_store);
    }
}
//#endregion 🧪Tests
