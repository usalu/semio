//! ⚖️ GIS 3D app — binary command protocol surface + laws (constitutional: protocol).

use gis3d_op::Gis3dTerrainOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Gis3dTerrainOperation` to its binary command form.
pub fn encode_op(operation: &Gis3dTerrainOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Gis3dTerrainOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Gis3dTerrainOperation, protocol::ProtocolError> {
    Gis3dTerrainOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use gis3d::Gis3dTerrainDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis3d_terrain_document_text_round_trips_through_store() {
        let initial = Gis3dTerrainDocument { exaggeration: 1.0 };
        let envelope = store::create_document_envelope(gis3d::GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪Tests
