//! ⚖️ Block 2D app — binary command protocol surface + laws (constitutional: protocol).

use block_2d_op::Block2dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Block2dOperation` to its binary command form.
pub fn encode_op(operation: &Block2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block2dOperation, protocol::ProtocolError> {
    Block2dOperation::decode_op(bytes)
}

//#region 🔖️Block2dCommand
/// 🎯️ `Block2dPlayApp::Command` — the sole dispatch surface for block-2d's behavior, one variant per
/// declared manifest action (`block_2d_ui::create_block2d_app`). Mirrors
/// `shooting_protocol::ShootingCommand`'s shape/derive conventions exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block2dCommand {
    #[dsl(key = "patchNodeKind")]
    PatchNodeKind { field: String, value: String },
    #[dsl(key = "addHandleKind")]
    AddHandleKind,
    #[dsl(key = "removeHandleKind")]
    RemoveHandleKind { id: String },
    #[dsl(key = "addHandle")]
    AddHandle,
    #[dsl(key = "removeHandle")]
    RemoveHandle { id: String },
    #[dsl(key = "addCompatibilityRule")]
    AddCompatibilityRule { source: String, target: String },
    #[dsl(key = "removeCompatibilityRule")]
    RemoveCompatibilityRule { id: String },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { id: String },
    #[dsl(key = "edit")]
    Edit { text: String },
    // 👁️ Config-only (was `Block2dPlayApp::selected_ids`'s `RefCell`) — emits `config_operations`, never document operations.
    #[dsl(key = "setSelection")]
    SetSelection { ids: Vec<String> },
}
//#endregion 🔖️Block2dCommand

//#region 🧪️Tests
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

    #[test]
    fn block2d_command_binary_round_trips() {
        let command = Block2dCommand::RemoveHandle { id: "h0".into() };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Block2dCommand::decode_op(&bytes).expect("decode"), command);
        let selection = Block2dCommand::SetSelection { ids: vec!["handle-kind:b-l".into()] };
        let bytes = selection.encode_op().expect("encode");
        assert_eq!(Block2dCommand::decode_op(&bytes).expect("decode"), selection);
    }
}
//#endregion 🧪️Tests
