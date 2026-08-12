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
pub use crate::artifacts::forms::schema::snapshot::FormsSnapshot;
pub use crate::artifacts::forms::schema::diff::FormsDiff;
pub use crate::artifacts::forms::schema::mutations::FormMutation;
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

//#region 🔖️Declaration
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below (and note's own
/// `pilot_languages` helper). Sole caller is `declaration()` below (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "forms.forms",
                    extension: Some("forms"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::forms::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::forms::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::forms::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::forms::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("forms.forms"),
                },
                dsl::LanguageSpec {
                    id: "forms.forms.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::forms::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::forms::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("forms.forms.op"),
                },
                dsl::LanguageSpec {
                    id: "forms.forms.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::forms::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::forms::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("forms.forms.diff"),
                },
                dsl::LanguageSpec {
                    id: "forms.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::forms::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::forms::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("forms.pack"),
                },
                dsl::LanguageSpec {
                    id: "forms.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("forms.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::apps::forms::config::schema::register_app_schema()` is the
/// one exception, still called from `📋️forms/🦀️component.rs`'s own `.setup()`: it registers the
/// `FormsPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has
/// no field for (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's
/// artifact-scoped function set (mirrors `🗒️note`'s exemplar exactly, see its own engine `declaration()`).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.forms")
        .schema(crate::artifacts::forms::schema::forms_artifact_schema_descriptor())
        .inferences([crate::artifacts::forms::standards::v1::subsets::any::schema::inferences::forms_artifact_inference_descriptor()])
        .composers(crate::artifacts::forms::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::forms::FormsPlayApp>()
        .build()
}
//#endregion 🔖️Declaration

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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::forms::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("FormsComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
