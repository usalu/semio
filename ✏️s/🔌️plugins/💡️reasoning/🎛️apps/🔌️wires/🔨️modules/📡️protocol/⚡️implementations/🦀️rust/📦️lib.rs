//! ⚖️ Reasoning wires app — binary command protocol surface + laws (constitutional: protocol).
//!
//! 🎯️ Also hosts `WiresCommand` — the app-engine `DocumentApp::Command` binary command envelope
//! (B1 pure-trait conversion, mirroring `shooting_protocol::ShootingCommand`). One variant per
//! `ReasoningWiresPlayApp`'s formerly stringly-typed `handle_action` match arm — see
//! `reasoning_wires_ui::ReasoningWiresPlayApp::handle` for the dispatch.

use protocol::OpBinary;
use reasoning_wires_op::MindmapWiresOperation;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `MindmapWiresOperation` to its binary command form.
pub fn encode_op(operation: &MindmapWiresOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `MindmapWiresOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MindmapWiresOperation, protocol::ProtocolError> {
    MindmapWiresOperation::decode_op(bytes)
}

//#region 🔖️WiresCommand
/// 🎯️ B1: `ReasoningWiresPlayApp::Command` — the SOLE dispatch surface for this app's behavior. Field
/// shapes mirror each action's real (pre-B1) JSON `args` object. `#[derive(dsl::DslOps)]` gives this a
/// binary (`OpBinary`) AND text (`OpText`) codec, matching `shooting_protocol::ShootingCommand`'s
/// derive/attribute conventions exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum WiresCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "add-node")]
    AddNode { kind: String },
    #[dsl(key = "add-relationship")]
    AddRelationship { kind: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "force-layout")]
    ForceLayout,
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "pointer-move")]
    CanvasPointerMove { x: f64, y: f64 },

    // 👁️ Config-only (was ephemeral `WiresPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "document-select")]
    DocumentSelect { ids: Vec<String> },
    #[dsl(key = "pointer-down")]
    CanvasPointerDown { id: Option<String>, x: f64, y: f64 },
    #[dsl(key = "pointer-up")]
    CanvasPointerUp,
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️WiresCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use reasoning_wires::MindmapWiresDocument;
    use serde_json::json;

    /// 🗄️ Local envelope/store alias for the whole-store tests below — mirrors the `pub type
    /// MindmapWiresEnvelope`/`MindmapWiresStore` the pre-split `reasoning_mindmap` crate exported,
    /// scoped here since this is the only crate that still needs it after the constitutional split.
    type MindmapWiresStore = store::DocumentStore<MindmapWiresDocument, MindmapWiresOperation>;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        let operation = MindmapWiresOperation::AddNode { node };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn store_applies_node_add() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(reasoning_wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", reasoning_wires_engine::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MindmapWiresOperation::AddNode { node }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = MindmapWiresStore::new(store::create_document_envelope(reasoning_wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", reasoning_wires_engine::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MindmapWiresOperation::AddNode { node }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `MindmapWiresOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Uses `AddNode` (not
    /// `ReplaceDocument`) deliberately — see `reasoning_wires_op`'s own tests for the known,
    /// still-open `ReplaceDocument` op-text ordering divergence on its raw `DslValue` fields.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = MindmapWiresStore::new(store::create_document_envelope(reasoning_wires::MINDMAP_WIRES_SCHEMA, "mindmap-wires", reasoning_wires_engine::empty_mindmap_wires_document(), None));
        let node = dsl::to_dsl_value(&json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).expect("node serializes");
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MindmapWiresOperation::AddNode { node }], description: None }).expect("apply");
        let edit: &Edit<MindmapWiresOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<MindmapWiresDocument, MindmapWiresOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
