//! ⚖️ Raster app — binary command protocol surface + laws (constitutional: protocol).

use raster_op::RasterOperation;
use protocol::OpBinary;

/// 📦 Encodes a `RasterOperation` to its binary command form.
pub fn encode_op(operation: &RasterOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `RasterOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RasterOperation, protocol::ProtocolError> {
    RasterOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use raster::{RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = raster_engine::empty_raster_document();
        let operation = RasterOperation::AddLayer {
            parent_id: None,
            index: document.layers.len(),
            layer: Box::new(RasterLayerNode::Pixel {
                id: "op-binary-test".into(),
                name: "Op Binary Test".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(64),
                height: Some(64),
                image_key: None,
            }),
        };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn raster_document_text_round_trips_store_with_applied_operation() {
        use raster::{RasterLayerNode, RasterProjection, RasterTransform};

        let envelope = store::create_document_envelope::<RasterProjection, RasterOperation>(
            RASTER_DOCUMENT_SCHEMA,
            "doc-text-test",
            raster_engine::empty_raster_document(),
            None,
        );
        let mut store = store::DocumentStore::new(envelope);
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![RasterOperation::AddLayer {
                    parent_id: None,
                    index: 1,
                    layer: Box::new(RasterLayerNode::Adjustment {
                        id: "adjust-text".into(),
                        name: "Levels".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "levels".into(),
                        params: serde_json::Map::new(),
                    }),
                }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪Tests
