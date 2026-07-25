//! 📋 Forms document domain — a forms-flavored instance of the generic `protocol` list/block
//! model. `forms` owns only what is genuinely forms-specific (its document schema id and default
//! empty projection); the strict-list step/block domain, VCS operations, and runtime helpers live in
//! `protocol` and are re-exported here under forms' historical names.

pub use protocol::{
    apply_protocol_edit_operation as apply_form_edit_operation, can_advance, default_value_for_block as default_value_for_question, eval_protocol_expr as eval_form_expr, find_block_location as find_question_location,
    flatten_protocol_blocks as flatten_form_questions, initial_values as initial_try_values, is_block_visible as is_question_visible, is_extension_block_kind as is_extension_question_kind, step_errors, visible_blocks as visible_questions,
    ProtocolBlock as FormQuestion, ProtocolBlockOption as FormQuestionOption, ProtocolDiff as FormDiff, ProtocolExpr as FormExpr, ProtocolOperation as FormOperation, ProtocolSpec as FormSpec, ProtocolStep as FormStep,
    ProtocolValidationError as FormValidationError, ProtocolVectorField as FormVectorField, PROTOCOL_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";

pub type FormsEnvelope = protocol::ProtocolEnvelope;
pub type FormsStore = protocol::ProtocolStore;

pub fn empty_forms_projection() -> FormSpec {
    FormSpec { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }] }
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::create_document_vcs_envelope;

    #[test]
    fn forms_document_vcs_materializes() {
        let store = FormsStore::new(create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let projection = store.projection().expect("projection");
        assert_eq!(projection.schema, FORMS_DOCUMENT_SCHEMA);
    }

    #[test]
    fn update_form_op_sets_title() {
        let spec = empty_forms_projection();
        let operation = FormOperation::UpdateProtocol { title: Some("Renamed".into()) };
        let next = apply_form_edit_operation(&spec, &operation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = FormsStore::new(create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        store.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![FormOperation::AddStep { step, index: None }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 2);
    }

    #[test]
    fn question_fields_roundtrip() {
        let json = r#"{
            "id":"q1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let question: FormQuestion = serde_json::from_str(json).expect("question json");
        assert_eq!(question.min, Some(1.0));
        assert_eq!(question.unit.as_deref(), Some("people"));
        assert!(question.required.unwrap_or(false));
    }
}
//#endregion 🧪Tests
