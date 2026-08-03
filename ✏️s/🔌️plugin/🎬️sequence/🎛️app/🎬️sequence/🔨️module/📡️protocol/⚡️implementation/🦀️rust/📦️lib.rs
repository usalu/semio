//! ⚖️ Sequence app — binary command protocol surface + laws (constitutional: protocol).
//!
//! 🎯️ Also hosts `SequenceCommand` — the app-engine `DocumentApp::Command` binary command envelope
//! (B1 pure-trait pivot, mirrors `shooting_protocol::ShootingCommand`). One variant per real declared
//! action in `sequence_ui::create_sequence_app`; the pre-B1 stringly-typed `{kind,name,args}`
//! `handle_action` dispatch is gone entirely — `sequence_ui::SequencePlayApp::handle` is the sole
//! dispatch surface.

use protocol::OpBinary;
use sequence::SequenceCamera;
use sequence_op::SequenceOperation;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `SequenceOperation` to its binary command form.
pub fn encode_op(operation: &SequenceOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SequenceOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<SequenceOperation, protocol::ProtocolError> {
    SequenceOperation::decode_op(bytes)
}

//#region 🔖️SequenceCommand
/// 🎯️ B1: `SequencePlayApp::Command` — the SOLE dispatch surface for sequence's own behavior.
/// Consolidates a handful of pre-B1 action ids that only ever mutated the same ephemeral state
/// (`setSelection`/`selectNode`/`nodeGraphSelect`/`graphPointerDown` all collapsed onto
/// `SetSelection`) — no legacy fallback, per the repo's no-backwards-compatibility rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SequenceCommand {
    // ✏️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-step")]
    AddStep { kind: String, x: f64, y: f64 },
    #[dsl(key = "add-step-to-slot")]
    AddStepToSlot { kind: String, x: f64, y: f64, owner: String, slot_name: String },
    #[dsl(key = "add-step-dropped")]
    AddStepDropped { kind: String, x: f64, y: f64, picked_step_id: Option<String> },
    #[dsl(key = "remove-step")]
    RemoveStep { id: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "move-step")]
    MoveStep { node_id: String, x: f64, y: f64 },
    #[dsl(key = "connect-steps")]
    ConnectSteps { source_node_id: String, target_node_id: String },
    #[dsl(key = "disconnect-steps")]
    DisconnectSteps { from_id: String, to_id: String },
    #[dsl(key = "set-step-params")]
    SetStepParams { id: String, params_json: String },
    #[dsl(key = "set-step-collapsed")]
    SetStepCollapsed { id: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },

    // 👁️ Config-only (was ephemeral `SequencePlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "set-selection")]
    SetSelection { step_ids: Vec<String> },
    #[dsl(key = "set-orientation")]
    SetOrientation { value: String },
    #[dsl(key = "run")]
    Run,
    #[dsl(key = "stop")]
    Stop,
    #[dsl(key = "set-viewport")]
    SetViewport {
        #[dsl(block)]
        camera: SequenceCamera,
    },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}
//#endregion 🔖️SequenceCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use sequence::default_fixture;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SequenceOperation::StepsPatch { id: "step-1".into(), patch: sequence::SequenceStepPatch { x: Some(42.0), ..Default::default() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 🧪️ Whole-store round trip: applies an operation through a real `SequenceStore`, then proves
    /// the resulting envelope survives both the text and binary document-level protocols.
    #[test]
    fn sequence_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<sequence::SequenceFixture, SequenceOperation>(sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence-text-test", default_fixture(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: sequence::SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: sequence::StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    //#region 🔖️SequenceCommandTests
    #[test]
    fn sequence_command_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SequenceCommand::AddStep { kind: "log.print".into(), x: 1.0, y: 2.0 });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::AddStepToSlot { kind: "log.print".into(), x: 1.0, y: 2.0, owner: "step-1".into(), slot_name: "then".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: None });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::RemoveStep { id: "step-1".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::DeleteSelection);
        store::test_support::assert_op_line_round_trip(&SequenceCommand::MoveStep { node_id: "step-1".into(), x: 5.0, y: 6.0 });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetStepParams { id: "step-1".into(), params_json: "{\"a\":1}".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetStepCollapsed { id: "step-1".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::Reorganize);
        store::test_support::assert_op_line_round_trip(&SequenceCommand::NodeGraphEdit { operations_json: "[]".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetSelection { step_ids: vec!["step-1".into(), "step-2".into()] });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetOrientation { value: "topBottom".into() });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::Run);
        store::test_support::assert_op_line_round_trip(&SequenceCommand::Stop);
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetViewport { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&SequenceCommand::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn sequence_command_binary_round_trips() {
        let command = SequenceCommand::AddStep { kind: "math.add".into(), x: 10.0, y: 20.0 };
        store::test_support::assert_op_text_binary_equivalence(&command);
    }
    //#endregion 🔖️SequenceCommandTests
}
//#endregion 🧪️Tests
