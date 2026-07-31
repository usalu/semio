//! ⚖️ GIS 2D app — binary command protocol surface + laws (constitutional: protocol).

use gis2d_op::GisMapOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `GisMapOperation` to its binary command form.
pub fn encode_op(operation: &GisMapOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapOperation, protocol::ProtocolError> {
    GisMapOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use gis2d::{MapFeature, GIS_MAP_SCHEMA};
    use protocol::CollectionOperation;
    use serde_json::json;

    fn sample_patch_feature() -> MapFeature {
        MapFeature { id: "p1".into(), data: json!({ "id": "p1", "lon": 1.0, "lat": 2.0 }) }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis_map_document_text_round_trips_through_store() {
        let initial = gis2d_engine::empty_gis_map_projection();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 })],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
