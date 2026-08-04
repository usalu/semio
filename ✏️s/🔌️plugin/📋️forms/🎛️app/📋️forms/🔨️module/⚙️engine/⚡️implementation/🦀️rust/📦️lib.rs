//! ⚙️ Forms app — headless compute (constitutional: engine). Pure compute over the `FormSpec`
//! document lives in the shared `playbook` kernel crate (the strict-list step/block domain forms
//! instantiates); this crate re-exports it under forms' historical names and adds the forms-specific
//! default/example document builders.

use forms::{FormSpec, FormStep, FORMS_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

pub use playbook::{
    can_advance, default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, initial_values as initial_try_values,
    is_block_visible as is_question_visible, is_extension_block_kind as is_extension_question_kind, step_errors, visible_blocks as visible_questions,
};

//#region 🔖️Config
/// 🧮️ B1: forms' real `DocumentApp::Config` — absorbs every field that used to live on
/// `forms_ui::FormsPlayApp`'s `RefCell<FormsPlayRuntime>` (blueprint selection, the Try wizard's
/// active step, its in-progress answer values) plus `locale` (was read off `view_state.locale`) and
/// `contributions_json` (was read off `view_state.contributions_json` — the host-declared
/// `Contribution::PlaybookBlockKind` list backing extension question rendering; `DocumentApp::render`
/// dropped `ViewState` entirely in B1, so the host now pushes contributions into config via
/// `forms_op::FormsConfigOperation::SetContributions`, mirroring how it now pushes locale via
/// `SetLocale`). `try_values`/`contributions` are both heterogeneous JSON (per-question-kind value
/// shapes; an arbitrary `Contribution` list) with no single concrete `dsl`-typed shape, so both stay
/// JSON-blob strings — the same idiom `layout_engine::LayoutConfig`'s port-recipe sibling
/// (`LayoutDocument::data_fields_json`) and `shooting_protocol::ShootingCommand::SetFixtureJson` use
/// for "opaque JSON payload, never a document/config field type of its own" data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "formscfg")]
#[dsl(layout = "lines")]
pub struct FormsConfig {
    /// 👁️ Selected blueprint step/question ids — was `FormsPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The Try wizard's active step index — was `FormsPlayRuntime::current_step_index`.
    pub current_step_index: u32,
    /// 👁️ The Try wizard's in-progress answer overrides (JSON object text, question id -> value) —
    /// was `FormsPlayRuntime::try_values`.
    pub try_values_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-declared plugin contributions (JSON array of `{pluginId, contribution}`, only
    /// `Contribution::PlaybookBlockKind` entries matter) — was read off `view_state.contributions_json`.
    pub contributions_json: String,
}

impl Default for FormsConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), current_step_index: 0, try_values_json: "{}".into(), locale: "en-US".into(), contributions_json: "[]".into() }
    }
}

store::impl_whole_record_config!(FormsConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ Forms' typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `forms.form` document schema) plus the WORKFLOWS-END-TO-END-TYPED-PORTS
/// `dictionary:out` port: the form's currently-configured default field values (see
/// `playbook::initial_values`, re-exported here as `initial_try_values`), re-exported as a
/// `form.dictionary` JSON object keyed by question id — the layout app's `fields:in` counterpart.
pub fn forms_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: FORMS_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "dictionary:out".into(),
            label: "Dictionary".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("form.dictionary".into()),
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "form.dictionary".into(), name: "Form".into(), dimension: "data".into(), component_kind: "forms".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
/// 🌱️ The forms app's empty document — a single "Inputs" step with no blocks yet.
pub fn empty_forms_projection() -> FormSpec {
    FormSpec { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }] }
}

/// 🌱️ The forms app's default document — the building-component fixture, seeded from its derive-
/// generated `.forms` DSL text (see `playbook::PlaybookSpec`'s `dsl::DslDocument` derive). Used as
/// `DocumentApp::initial_projection`.
pub fn building_component_spec() -> FormSpec {
    <FormSpec as store::DocumentDsl>::parse_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ The `default` (Contact) example, parsed once from {@link forms_dsl::DEFAULT_EXAMPLE_TEXT} — the
/// source of truth for every "default" example call site (`setActiveExample`, `App::example`).
pub fn default_example_spec() -> FormSpec {
    <FormSpec as store::DocumentDsl>::parse_dsl(forms_dsl::DEFAULT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ JSON re-serialization of {@link default_example_spec}, for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn default_example_json() -> String {
    serde_json::to_string(&default_example_spec()).expect("serialize default example document")
}

/// 📄️ The `onboarding` example, parsed once from {@link forms_dsl::ONBOARDING_EXAMPLE_TEXT}.
pub fn onboarding_example_spec() -> FormSpec {
    <FormSpec as store::DocumentDsl>::parse_dsl(forms_dsl::ONBOARDING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ JSON re-serialization of {@link onboarding_example_spec}, for the framework-generic call sites
/// that contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn onboarding_example_json() -> String {
    serde_json::to_string(&onboarding_example_spec()).expect("serialize onboarding example document")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_config_default_matches_the_existing_runtime_defaults() {
        let config = FormsConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.current_step_index, 0);
        assert_eq!(config.try_values_json, "{}");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.contributions_json, "[]");
    }

    #[test]
    fn forms_config_dsl_and_pack_round_trip() {
        let config = FormsConfig { selected_ids: vec!["q1".into(), "q2".into()], current_step_index: 2, try_values_json: r#"{"name":"Ada"}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn forms_io_declares_dictionary_out_port() {
        let io = forms_io();
        assert_eq!(io.document_schema, FORMS_DOCUMENT_SCHEMA);
        let dictionary_out = io.ports.iter().find(|port| port.id == "dictionary:out").expect("dictionary:out declared");
        assert_eq!(dictionary_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(dictionary_out.kind_id.as_deref(), Some("form.dictionary"));
        assert_eq!(dictionary_out.multiplicity, semio_framework_core::PortMultiplicity::Many);
        let all_ports = io.all_ports();
        assert!(all_ports.iter().any(|port| port.id == "document:in"));
        assert!(all_ports.iter().any(|port| port.id == "document:out"));
    }
}
//#endregion 🧪️Tests
