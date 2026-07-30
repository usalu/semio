//! ⚖️ Block 5D app — binary command protocol surface + laws (constitutional: protocol).

use block_5d_op::Block5dOperation;
use protocol::OpBinary;

/// 📦 Encodes a `Block5dOperation` to its binary command form.
pub fn encode_op(operation: &Block5dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Block5dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block5dOperation, protocol::ProtocolError> {
    Block5dOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block5d_document_vcs_replays_granular_operations() {
        use block_5d::BLOCK_5D_SCHEMA;
        use block_5d_op::Block5dStore;
        use block_shared::BlockKindIdentity;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Block5dStore::new(create_document_envelope(BLOCK_5D_SCHEMA, "block5d", block_5d_engine::empty_block5d_definition(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Block5dOperation::SetPartKind { part_kind: BlockKindIdentity { id: "p1".into(), name: "p1".into(), label: "P1".into(), ..Default::default() } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.part_kind.id, "p1");
    }
}
//#endregion 🧪Tests
