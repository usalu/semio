//! ⚖️ Block 3D app — binary command protocol surface + laws (constitutional: protocol).

use block_3d_op::Block3dOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Block3dOperation` to its binary command form.
pub fn encode_op(operation: &Block3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Block3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dOperation, protocol::ProtocolError> {
    Block3dOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block3d_document_vcs_replays_granular_operations() {
        use block_3d::BLOCK_3D_SCHEMA;
        use block_3d_op::Block3dStore;
        use block_shared::BlockKindIdentity;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Block3dStore::new(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", block_3d_engine::empty_block3d_definition(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Block3dOperation::SetObjectKind { object_kind: BlockKindIdentity { id: "o1".into(), name: "o1".into(), label: "O1".into(), ..Default::default() } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "o1");
    }
}
//#endregion 🧪Tests
