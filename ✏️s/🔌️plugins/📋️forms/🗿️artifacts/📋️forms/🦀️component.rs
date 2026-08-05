//! 📋️ Forms artifact — the document entity this plugin's app edits.
//!
//! `FormSpec`'s fields and every block/step/expr type are NOT owned here — they live in the shared
//! `playbook` kernel crate (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook`) because `forms` owns only
//! what is genuinely forms-specific (its document schema id); the strict-list step/block domain, VCS
//! operations, and runtime helpers live in `playbook` and are re-exported here under forms' historical
//! names. This component re-exports the app-facing surface so every sibling taxonomy node (`🔺️diff`,
//! `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol instead of reaching
//! into the kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use playbook::{
    PlaybookBlock as FormQuestion, PlaybookBlockOption as FormQuestionOption, PlaybookExpr as FormExpr, PlaybookSpec as FormSpec, PlaybookStep as FormStep, PlaybookValidationError as FormValidationError, PlaybookVectorField as FormVectorField,
    PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

/// 🔖️ The store envelope schema id (distinct from the `ArtifactKindSpec.schema` media catalogue id below
/// — see `artifact_kind`'s doc for why the two are kept apart).
pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::forms::create_forms_app`'s `🔖️Manifest` region. Deliberately uses `"form.dictionary"`
/// as BOTH `id` and `schema` (unlike most artifacts, which keep the media catalogue id and the store
/// schema distinct) — forms' typed `dictionary:out` port re-exports the document under that same media
/// kind id, so keeping them equal is the accurate shape, not an oversight.
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
