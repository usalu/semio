//! ⚖️ Block 2D app — binary command protocol surface + laws (constitutional: protocol).

use block_2d_op::Block2dOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Block2dOperation` to its binary command form.
pub fn encode_op(operation: &Block2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Block2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block2dOperation, protocol::ProtocolError> {
    Block2dOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block2d_document_vcs_replays_granular_operations() {
        use block_2d::BLOCK_2D_SCHEMA;
        use block_2d_op::Block2dStore;
        use block_shared::BlockKindIdentity;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Block2dStore::new(create_document_envelope(BLOCK_2D_SCHEMA, "block2d", block_2d_engine::empty_block2d_definition(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Block2dOperation::SetNodeKind { node_kind: BlockKindIdentity { id: "n1".into(), name: "n1".into(), label: "N1".into(), ..Default::default() } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.node_kind.id, "n1");
    }
}
//#endregion 🧪Tests
