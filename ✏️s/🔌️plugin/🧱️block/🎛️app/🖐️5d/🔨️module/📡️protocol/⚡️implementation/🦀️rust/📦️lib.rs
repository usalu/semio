//! ⚖️ Block 5D app — binary command protocol surface + laws (constitutional: protocol).

use block_5d_op::Block5dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Block5dOperation` to its binary command form.
pub fn encode_op(operation: &Block5dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block5dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block5dOperation, protocol::ProtocolError> {
    Block5dOperation::decode_op(bytes)
}

//#region 🔖️Block5dCommand
/// 🎯️ `Block5dPlayApp::Command` — the sole dispatch surface for block-5d's behavior, one variant per
/// declared manifest action (`block_5d_ui::create_block5d_app`). Mirrors
/// `shooting_protocol::ShootingCommand`'s shape/derive conventions exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block5dCommand {
    #[dsl(key = "patchPartKind")]
    PatchPartKind { field: String, value: String },
    #[dsl(key = "addGripKind")]
    AddGripKind,
    #[dsl(key = "removeGripKind")]
    RemoveGripKind { id: String },
    #[dsl(key = "addGrip")]
    AddGrip,
    #[dsl(key = "removeGrip")]
    RemoveGrip { id: String },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { id: String },
    #[dsl(key = "edit")]
    Edit { text: String },
    // 👁️ Config-only (was `Block5dPlayApp::selected_ids`'s `RefCell`) — emits `config_operations`, never document operations.
    #[dsl(key = "setSelection")]
    SetSelection { ids: Vec<String> },
}
//#endregion 🔖️Block5dCommand

//#region 🧪️Tests
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
        store.dispatch(DocumentCommand::Apply { operations: vec![Block5dOperation::SetPartKind { part_kind: BlockKindIdentity { id: "p1".into(), name: "p1".into(), label: "P1".into(), ..Default::default() } }], description: None }).expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.part_kind.id, "p1");
    }

    #[test]
    fn block5d_command_binary_round_trips() {
        let command = Block5dCommand::RemoveGrip { id: "g0".into() };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Block5dCommand::decode_op(&bytes).expect("decode"), command);
        let selection = Block5dCommand::SetSelection { ids: vec!["grip-kind:b-l".into()] };
        let bytes = selection.encode_op().expect("encode");
        assert_eq!(Block5dCommand::decode_op(&bytes).expect("decode"), selection);
    }
}
//#endregion 🧪️Tests
