//! ⚙️ Forms app — headless compute (constitutional: engine). Pure compute over the `FormSpec`
//! document lives in the shared `playbook` kernel crate (the strict-list step/block domain forms
//! instantiates); this crate re-exports it under forms' historical names and adds the forms-specific
//! default/example document builders.

use forms::{FormSpec, FormStep, FORMS_DOCUMENT_SCHEMA};

pub use playbook::{
    can_advance, default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location,
    flatten_playbook_blocks as flatten_form_questions, initial_values as initial_try_values, is_block_visible as is_question_visible, is_extension_block_kind as is_extension_question_kind,
    step_errors, visible_blocks as visible_questions,
};

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
