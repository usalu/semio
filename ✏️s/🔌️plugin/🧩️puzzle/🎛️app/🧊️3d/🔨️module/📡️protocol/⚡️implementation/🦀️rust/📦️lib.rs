//! ⚖️ Puzzle 3d app — binary command protocol surface + laws (constitutional: protocol).

use puzzle_3d_op::Puzzle3dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Puzzle3dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dOperation, protocol::ProtocolError> {
    Puzzle3dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle3d_document_vcs_replays_granular_operations() {
        use puzzle_3d::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
        use puzzle_3d_op::Puzzle3dStore;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", puzzle_3d_engine::empty_puzzle3d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Puzzle3dOperation::SetObject { index: 0, object: Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "o1");
    }
}
//#endregion 🧪️Tests
