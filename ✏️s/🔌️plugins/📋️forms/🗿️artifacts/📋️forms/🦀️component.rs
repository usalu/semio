//! 📋️ Forms artifact — the document entity this plugin's app edits.
//!
//! Domain step/block/expr types live in the shared `playbook` kernel crate and are re-exported here under
//! forms' historical names. `FormsSnapshot` is defined in `📸️snapshot/🧬️schema` and re-exported here.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use crate::playbook::{
    PlaybookBlock as FormQuestion, PlaybookBlockOption as FormQuestionOption, PlaybookExpr as FormExpr, PlaybookStep as FormStep,
    PlaybookValidationError as FormValidationError, PlaybookVectorField as FormVectorField, PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";
pub use crate::artifacts::forms::snapshot::schema::FormsSnapshot;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::forms::create_forms_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "form.dictionary".into(),
        name: "Form Dictionary".into(),
        source_format: "form.dictionary".into(),
        component_kind: "forms".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: "form.dictionary".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_uses_the_dictionary_media_kind_as_both_id_and_schema() {
        assert_eq!(artifact_kind().id, "form.dictionary");
        assert_eq!(artifact_kind().schema, "form.dictionary");
        assert_eq!(FORMS_DOCUMENT_SCHEMA, "forms.form");
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
//#endregion 🧪️Tests
