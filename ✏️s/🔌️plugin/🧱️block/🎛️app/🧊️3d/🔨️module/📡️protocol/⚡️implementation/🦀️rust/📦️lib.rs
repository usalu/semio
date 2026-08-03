//! ⚖️ Block 3D app — binary command protocol surface + laws (constitutional: protocol).

use block_3d_op::Block3dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Block3dOperation` to its binary command form.
pub fn encode_op(operation: &Block3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dOperation, protocol::ProtocolError> {
    Block3dOperation::decode_op(bytes)
}

//#region 🔖️Block3dCommand
/// 🎯️ `Block3dPlayApp::Command` — the sole dispatch surface for block-3d's behavior, one variant per
/// declared manifest action (`block_3d_ui::create_block3d_app`). Mirrors
/// `shooting_protocol::ShootingCommand`'s shape/derive conventions exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Block3dCommand {
    #[dsl(key = "patchObjectKind")]
    PatchObjectKind { field: String, value: String },
    #[dsl(key = "addRepresentation")]
    AddRepresentation,
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "addVortexKind")]
    AddVortexKind,
    #[dsl(key = "removeVortexKind")]
    RemoveVortexKind { id: String },
    #[dsl(key = "addVortex")]
    AddVortex,
    #[dsl(key = "removeVortex")]
    RemoveVortex { id: String },
    #[dsl(key = "setActiveExample")]
    SetActiveExample { id: String },
    #[dsl(key = "edit")]
    Edit { text: String },
    // 👁️ Config-only (was `Block3dPlayApp`'s `RefCell` runtime fields) — emit `config_operations`, never document operations.
    #[dsl(key = "setSelection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "setActiveRepresentation")]
    SetActiveRepresentation { representation_id: Option<String> },
}
//#endregion 🔖️Block3dCommand

//#region 🧪️Tests
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

    #[test]
    fn block3d_command_binary_round_trips() {
        let command = Block3dCommand::RemoveVortex { id: "v0".into() };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Block3dCommand::decode_op(&bytes).expect("decode"), command);
        let selection = Block3dCommand::SetSelection { ids: vec!["representation:r0".into()] };
        let bytes = selection.encode_op().expect("encode");
        assert_eq!(Block3dCommand::decode_op(&bytes).expect("decode"), selection);
    }
}
//#endregion 🧪️Tests
