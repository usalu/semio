//! ⚖️ Forms app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! `FormsCommand` — the app-engine `AppCommand::Command` binary command envelope (B1 pure-trait flip).
//! One variant per `create_forms_app`'s real declared action.

use forms_op::FormOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub type FormsEnvelope = playbook::PlaybookEnvelope;
pub type FormsStore = playbook::PlaybookStore;
//#endregion 🔖️Types

/// 📦️ Encodes a `FormOperation` to its binary command form.
pub fn encode_op(operation: &FormOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FormOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<FormOperation, protocol::ProtocolError> {
    FormOperation::decode_op(bytes)
}

//#region 🔖️FormsCommand
/// 🎯️ B1: `FormsPlayApp::Command` — the SOLE dispatch surface for forms' own behavior, covering
/// every declared action. Generic value payloads that used to be a heterogeneous JSON `Value` args
/// field (multi-kind question defaults, vector-field numbers, building-component params, …) are
/// carried as JSON-encoded text (`value_json`/`values_json`), parsed per-field in
/// `forms_ui::FormsPlayApp::handle` — the same idiom `layout_protocol::LayoutCommand::PatchPage`/
/// `PatchFrame` and `shooting_protocol::ShootingCommand::SetFixtureJson` use for "opaque JSON payload"
/// data with no single concrete `dsl`-typed shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FormsCommand {
    // 👁️ Config-only — mutate ephemeral config state, never emit document operations.
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "try-value")]
    SetTryValue { key: String, value_json: Option<String>, option_value: Option<String>, vector_index: Option<u64>, param_key: Option<String> },
    #[dsl(key = "try-values")]
    SetTryValues { values_json: String },
    #[dsl(key = "reset-try")]
    ResetTry,
    #[dsl(key = "previous-step")]
    PreviousStep,
    #[dsl(key = "next-step")]
    NextStep,
    #[dsl(key = "submit")]
    Submit,
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },

    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-step")]
    AddStep,
    #[dsl(key = "patch-step")]
    PatchStep { step_id: String, field: String, value: String },
    #[dsl(key = "remove-step")]
    RemoveStep { step_id: String },
    #[dsl(key = "move-step")]
    MoveStep { step_id: String, index: u64 },
    #[dsl(key = "update-form")]
    UpdateForm { title: String },
    #[dsl(key = "add-question")]
    AddQuestion { kind: String, step_id: Option<String> },
    #[dsl(key = "remove-question")]
    RemoveQuestion { question_id: String },
    #[dsl(key = "patch-questions")]
    PatchQuestions { question_ids: Vec<String>, field: String, value_json: String, param_key: Option<String> },
    #[dsl(key = "patch-question-options")]
    PatchQuestionOptions { question_ids: Vec<String>, option_value: String, field: String, value_json: String },
    #[dsl(key = "add-question-option")]
    AddQuestionOption { question_id: String, label: String },
    #[dsl(key = "remove-question-option")]
    RemoveQuestionOption { question_id: String, option_value: String },
    #[dsl(key = "patch-vector-field")]
    PatchVectorField { question_id: String, field_key: String, field: String, value_json: String },
    #[dsl(key = "add-vector-field")]
    AddVectorField { question_id: String, field_key: String },
    #[dsl(key = "remove-vector-field")]
    RemoveVectorField { question_id: String, field_key: String },
    #[dsl(key = "move-question")]
    MoveQuestion { question_id: String, to_step_id: String, target_id: Option<String>, position: String, index: Option<u64> },
    #[dsl(key = "drop-question-kind")]
    DropQuestionKind { kind: String, target_id: String, drop_position: String },
    #[dsl(key = "spec-json")]
    SetSpecJson { json: String },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },

    // 🐚️ Shell effects — export round-trips through the host, no operations either way.
    #[dsl(key = "export-fixture")]
    ExportFixture,
}
//#endregion 🔖️FormsCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::{FormStep, FORMS_DOCUMENT_SCHEMA};
    use forms_engine::empty_forms_projection;
    use store::create_document_envelope;

    #[test]
    fn forms_document_vcs_materializes() {
        let store = FormsStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let projection = store.projection().expect("projection");
        assert_eq!(projection.schema, FORMS_DOCUMENT_SCHEMA);
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = FormsStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        store.dispatch(store::DocumentCommand::Apply { operations: vec![FormOperation::AddStep { step, index: None }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 2);
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FormOperation::UpdatePlaybook { title: Some("Renamed".into()) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    //#region 🧪️FormsCommand
    #[test]
    fn forms_command_op_text_and_binary_round_trip_every_variant() {
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetSelection { ids: vec!["q1".into()] });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetTryValue { key: "q1".into(), value_json: Some("\"Ada\"".into()), option_value: None, vector_index: None, param_key: None });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetTryValue { key: "q1".into(), value_json: None, option_value: Some("a".into()), vector_index: Some(2), param_key: Some("height".into()) });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetTryValues { values_json: r#"{"name":"Ada"}"#.into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::ResetTry);
        store::test_support::assert_op_line_round_trip(&FormsCommand::PreviousStep);
        store::test_support::assert_op_line_round_trip(&FormsCommand::NextStep);
        store::test_support::assert_op_line_round_trip(&FormsCommand::Submit);
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetContributions { json: "[]".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::AddStep);
        store::test_support::assert_op_line_round_trip(&FormsCommand::PatchStep { step_id: "s1".into(), field: "title".into(), value: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::RemoveStep { step_id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::MoveStep { step_id: "s1".into(), index: 0 });
        store::test_support::assert_op_line_round_trip(&FormsCommand::UpdateForm { title: "My Form".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::AddQuestion { kind: "text".into(), step_id: Some("s1".into()) });
        store::test_support::assert_op_line_round_trip(&FormsCommand::AddQuestion { kind: "text".into(), step_id: None });
        store::test_support::assert_op_line_round_trip(&FormsCommand::RemoveQuestion { question_id: "q1".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::PatchQuestions { question_ids: vec!["q1".into(), "q2".into()], field: "required".into(), value_json: "true".into(), param_key: None });
        store::test_support::assert_op_line_round_trip(&FormsCommand::PatchQuestionOptions { question_ids: vec!["q1".into()], option_value: "a".into(), field: "label".into(), value_json: "\"Option A\"".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::AddQuestionOption { question_id: "q1".into(), label: "New option".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::RemoveQuestionOption { question_id: "q1".into(), option_value: "a".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::PatchVectorField { question_id: "q1".into(), field_key: "x".into(), field: "value".into(), value_json: "1.0".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::AddVectorField { question_id: "q1".into(), field_key: "w".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::RemoveVectorField { question_id: "q1".into(), field_key: "w".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::MoveQuestion { question_id: "q1".into(), to_step_id: "s2".into(), target_id: Some("q2".into()), position: "before".into(), index: Some(0) });
        store::test_support::assert_op_line_round_trip(&FormsCommand::DropQuestionKind { kind: "slider".into(), target_id: "step:s1".into(), drop_position: "inside".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetSpecJson { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::SetActiveExample { example_id: "default".into() });
        store::test_support::assert_op_line_round_trip(&FormsCommand::ExportFixture);

        let command = FormsCommand::UpdateForm { title: "My Form".into() };
        let bytes = command.encode_op().expect("encode command");
        assert_eq!(FormsCommand::decode_op(&bytes).expect("decode command"), command);
    }
    //#endregion 🧪️FormsCommand
}
//#endregion 🧪️Tests
