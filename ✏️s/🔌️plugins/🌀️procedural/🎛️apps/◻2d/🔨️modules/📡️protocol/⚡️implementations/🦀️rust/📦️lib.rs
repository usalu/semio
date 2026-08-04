//! ⚖️ Procedural 2D app — binary command protocol surface + laws (constitutional: protocol).

use procedural_2d_op::Procedural2dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Procedural2dOperation` to its binary command form.
pub fn encode_op(operation: &Procedural2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural2dOperation, protocol::ProtocolError> {
    Procedural2dOperation::decode_op(bytes)
}

//#region 🔖️Procedural2dCommand
/// 🎯️ Wave-2: `Procedural2dPlayApp::Command` — the SOLE dispatch surface for procedural2d's own
/// behavior. One variant per action declared in `create_procedural2d_app`'s static manifest (see
/// `procedural_2d_ui::Procedural2dPlayApp::command_id`, which maps each variant back to its action
/// id string). Field shapes mirror each action's real args exactly, except two free-form payloads
/// that are genuinely untyped elsewhere in this codebase too (`nodeGraphEdit`'s flow-host sub-op
/// array, `setEvalOutputs`'s eval-tick JSON) — those stay JSON-text fields, parsed inside `handle()`
/// exactly like the pre-Wave-2 code did, rather than inventing a parallel typed sub-op enum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural2dCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
    #[dsl(key = "move-media-node")]
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    #[dsl(key = "add-widget")]
    AddWidget { kind: String, neuron_kind: Option<String>, x: Option<f64>, y: Option<f64> },
    #[dsl(key = "remove-widget")]
    RemoveWidget { widget_id: String },
    #[dsl(key = "connect-media-ports")]
    ConnectMediaPorts { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "add-generation")]
    AddGeneration,
    #[dsl(key = "remove-generation")]
    RemoveGeneration { id: String },
    #[dsl(key = "rename-generation")]
    RenameGeneration { id: String, name: String },
    #[dsl(key = "update-generation-values")]
    UpdateGenerationValues { generation_id: Option<String>, question_id: String, value: dsl::DslValue },

    // 👁️ Config-only — ephemeral selection/camera/show-mode/eval-scratch (was `ShootingPlayRuntime`-style
    // app-struct state), emit `config_operations`, never document operations.
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { viewport_json: String },
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "select-node")]
    SelectNode { ids: Vec<String> },
    #[dsl(key = "node-graph-select")]
    NodeGraphSelect { ids: Vec<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover,
    #[dsl(key = "set-show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "generate")]
    Generate,
    #[dsl(key = "set-eval-outputs")]
    SetEvalOutputs { outputs_json: String },
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown,
    #[dsl(key = "canvas-pointer-move")]
    CanvasPointerMove,
    #[dsl(key = "canvas-pointer-up")]
    CanvasPointerUp,
    #[dsl(key = "canvas-wheel")]
    CanvasWheel,
    #[dsl(key = "select-generation")]
    SelectGeneration { id: Option<String> },

    // 🧵️ Internal, host-driven — self-dispatched via `HostEffect::DispatchAction` (`FlowEvalTick`) or
    // pushed by the shell session (`SetLocale`), never declared in the command palette (undeclared in
    // the manifest today too — mirrors `shooting_protocol::ShootingCommand::SetLocale`).
    #[dsl(key = "flow-eval-tick")]
    FlowEvalTick,
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️Procedural2dCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use procedural_2d::PROCEDURAL_2D_SCHEMA;
    use procedural_2d_engine::empty_procedural2d_projection;
    use procedural_2d_op::Procedural2dStore;
    use store::{create_document_envelope, test_support, DocumentCommand};

    //#region 🔖️DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural2dStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DocumentTextTests

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Procedural2dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = Procedural2dStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        let edit: &Edit<Procedural2dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        test_support::assert_command_envelope_round_trip::<procedural_2d::Procedural2dDocument, Procedural2dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
