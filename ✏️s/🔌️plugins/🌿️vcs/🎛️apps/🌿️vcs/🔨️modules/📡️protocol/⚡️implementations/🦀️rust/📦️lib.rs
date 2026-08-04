//! ⚖️ VCS app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use vcs_op::VcsDemoOperation;

/// 📦️ Encodes a `VcsDemoOperation` to its binary command form.
pub fn encode_op(operation: &VcsDemoOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `VcsDemoOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<VcsDemoOperation, protocol::ProtocolError> {
    VcsDemoOperation::decode_op(bytes)
}

//#region 🔖️VcsDemoCommand
/// 🎯️ `VcsPlayApp::Command` — the SOLE dispatch surface for the vcs demo app's own behavior. The six
/// history actions (undo/redo/commitCheckpoint/createAlternative/switchAlternative/checkoutCheckpoint)
/// never reach here — `VcsDocumentApp` intercepts those itself as host mechanics, not app behavior (see
/// `shooting_protocol::ShootingCommand`'s identical doc). Field shapes mirror each action's old JSON
/// `args` object exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum VcsDemoCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "increment-counter")]
    IncrementCounter,
    #[dsl(key = "patch-projection")]
    PatchProjection { field: String, value: String },
    #[dsl(key = "text-edit")]
    TextEdit { text: String },
    #[dsl(key = "edit")]
    Edit { text: String },

    // 👁️ Config-only (was ephemeral `VcsPlayApp`/`ViewState` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "no-operation")]
    NoOperation,
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown,
    #[dsl(key = "canvas-pointer-move")]
    CanvasPointerMove,
    #[dsl(key = "canvas-pointer-up")]
    CanvasPointerUp,
    #[dsl(key = "canvas-wheel")]
    CanvasWheel,
}
//#endregion 🔖️VcsDemoCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = VcsDemoOperation::SetCounter { counter: 7 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::IncrementCounter);
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::PatchProjection { field: "title".into(), value: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::TextEdit { text: "{}".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::Edit { text: "{}".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::SetSelection { ids: vec!["checkpoint-1".into()] });
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::NoOperation);
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::CanvasPointerDown);
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::CanvasPointerMove);
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::CanvasPointerUp);
        store::test_support::assert_op_line_round_trip(&VcsDemoCommand::CanvasWheel);
    }

    #[test]
    fn vcs_demo_command_op_binary_agrees_with_text() {
        store::test_support::assert_op_text_binary_equivalence(&VcsDemoCommand::PatchProjection { field: "counter".into(), value: "3".into() });
    }
}
//#endregion 🧪️Tests
