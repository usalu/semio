//! ⚙️ Forms artifact — headless compute over the `FormSpec` projection (constitutional: engine).
//!
//! Pure compute over the shared `playbook` kernel crate's step/block domain is re-exported here under
//! forms' historical names; this component adds the forms-specific default/example document builders,
//! the `forms.form` media I/O declaration, and every doc-pure helper with MORE THAN ONE consumer across
//! the app's taxonomy tree (a helper with exactly one consumer lives in that consumer's component file).
//! `FormsConfig` (view state, not document state) does NOT live here — see `🎛️apps/📋️forms/🦀️config.rs`.

use crate::artifacts::forms::op::FormOperation;
// 🧷️ Aliased (not the bare `dsl` name): this file also needs the EXTERN `dsl` crate (kernel DSL
// value/derive surface) for `value_to_dsl`/`dsl_to_value` below — importing the artifact's own `dsl`
// submodule under the bare name would shadow that crate and break every `dsl::DslValue`/`dsl::to_dsl_value`
// reference in this file (confirmed by `cargo check`: E0425/E0433 "not found in `dsl`").
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{FormQuestion, FormSpec, FormStep, FORMS_DOCUMENT_SCHEMA};
use serde_json::Value;

//#region 🔖️Types
pub use playbook::{
    can_advance, default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, initial_values as initial_try_values,
    is_block_visible as is_question_visible, is_extension_block_kind as is_extension_question_kind, step_errors, visible_blocks as visible_questions,
};
//#endregion 🔖️Types

//#region 🔖️Register
/// 🗂️ Registers `FormSpec`'s pack↔dsl codec under `FORMS_DOCUMENT_SCHEMA` so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse forms documents without
/// depending on this crate's concrete `Projection`/`Operation` types. Called from the plugin root's
/// `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::forms::FormsPlayApp>(FORMS_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "forms.forms",
        extension: Some("forms"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::forms::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::forms::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::forms::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::forms::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("forms.forms"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "forms.forms.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::forms::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::forms::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("forms.forms.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "forms.forms.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::forms::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::forms::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("forms.forms.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "forms.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::forms::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::forms::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("forms.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "forms.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::forms::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("forms.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ Forms' typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `forms.form` document schema) plus the WORKFLOWS-END-TO-END-TYPED-PORTS
/// `dictionary:out` port: the form's currently-configured default field values (see
/// [`initial_try_values`]), re-exported as a `form.dictionary` JSON object keyed by question id — the
/// layout app's `fields:in` counterpart.
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
/// generated `.forms` DSL text.
pub fn building_component_spec() -> FormSpec {
    forms_dsl::parse_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ The `default` (Contact) example, parsed once from `forms_dsl::DEFAULT_EXAMPLE_TEXT` — the source of truth
/// for every "default" example call site (`setActiveExample`, `App::example`).
pub fn default_example_spec() -> FormSpec {
    forms_dsl::parse_dsl(forms_dsl::DEFAULT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ JSON re-serialization of [`default_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn default_example_json() -> String {
    serde_json::to_string(&default_example_spec()).expect("serialize default example document")
}

/// 📄️ The `onboarding` example, parsed once from `forms_dsl::ONBOARDING_EXAMPLE_TEXT`.
pub fn onboarding_example_spec() -> FormSpec {
    forms_dsl::parse_dsl(forms_dsl::ONBOARDING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_projection())
}

/// 📄️ JSON re-serialization of [`onboarding_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn onboarding_example_json() -> String {
    serde_json::to_string(&onboarding_example_spec()).expect("serialize onboarding example document")
}

/// 🔠️ Every `(step title, question)` pair in document order — the empty-inspector diagnostic and every
/// command test's "did the edit land" assertion share this flattening.
pub fn flatten_questions(spec: &FormSpec) -> Vec<(String, FormQuestion)> {
    spec.steps.iter().flat_map(|step| step.blocks.iter().map(|question| (step.title.clone(), question.clone()))).collect()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️QuestionLocation
pub struct QuestionLocation {
    pub step_id: String,
    pub question: FormQuestion,
}

/// 🔎️ Locates a question by id anywhere in the document — the single lookup every question-editing
/// command (`❓️question`, `🔘️option`, `📐️vector`) and the inspection panel share.
pub fn locate_question(spec: &FormSpec, question_id: &str) -> Option<QuestionLocation> {
    for step in &spec.steps {
        if let Some(question) = step.blocks.iter().find(|question| question.id == question_id) {
            return Some(QuestionLocation { step_id: step.id.clone(), question: question.clone() });
        }
    }
    None
}

/// ✏️ Locates `question_id` in `spec`, applies `mutate` to a clone, and returns the `UpdateBlock`
/// operation that records the edit — the single seam every inspector/command patch flows through.
/// Returns `None` if the question no longer exists.
pub fn update_block_operation(spec: &FormSpec, question_id: &str, mutate: impl FnOnce(&mut FormQuestion)) -> Option<FormOperation> {
    let location = locate_question(spec, question_id)?;
    let mut question = location.question;
    mutate(&mut question);
    Some(FormOperation::UpdateBlock { step_id: location.step_id, block: question })
}
//#endregion 🔖️QuestionLocation

//#region 🔖️Ids

/// 🆔️ A process-unique id for a newly created step/question/option — shared by every command that
/// creates one (`addStep`, `addQuestion`, `dropQuestionKind`, `addQuestionOption`).
pub fn create_form_id(prefix: &str) -> String {
    let serial = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{serial}")
}

/// 🌳️ The document-tree node id for a step — shared by the document panel (tree item ids) and the
/// question drag/drop commands (resolving a drop target back to its owning step).
pub fn forms_play_step_tree_id(step_id: &str) -> String {
    format!("step:{step_id}")
}
//#endregion 🔖️Ids

//#region 🔖️Values
/// 🔄️ Converts a `serde_json::Value` to a `dsl::DslValue` — falls back to `Null` on an unsupported shape
/// (never occurs for the plain JSON literals every question default carries).
pub fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

/// 🔄️ Converts a `dsl::DslValue` back to a `serde_json::Value`.
pub fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

/// 🔤️ A `dsl::DslValue` rendered as a display string — the inspector's text-field representation of a
/// question's typed default.
pub fn dsl_string_value(value: &dsl::DslValue) -> String {
    json_string_value(&dsl_to_value(value))
}

/// 🔢️ A `dsl::DslValue` rendered as `f64` — the inspector's numeric-field representation of a question's
/// typed default.
pub fn dsl_f64_value(value: &dsl::DslValue) -> f64 {
    json_f64_value(&dsl_to_value(value))
}

/// 🔤️ A `serde_json::Value` rendered as a display string — shared by the inspector's editable fields and
/// the try wizard's current-answer rendering.
pub fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// 🔢️ A `serde_json::Value` rendered as `f64` (0.0 on a non-numeric shape).
pub fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}
//#endregion 🔖️Values

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    #[test]
    fn locate_question_finds_a_question_anywhere_in_the_document() {
        let spec = building_component_spec();
        let first_id = spec.steps[0].blocks[0].id.clone();
        let location = locate_question(&spec, &first_id).expect("question located");
        assert_eq!(location.step_id, spec.steps[0].id);
    }

    #[test]
    fn update_block_operation_returns_none_for_a_missing_question() {
        let spec = empty_forms_projection();
        assert!(update_block_operation(&spec, "missing", |question| question.label = "x".into()).is_none());
    }

    #[test]
    fn update_block_operation_patches_the_located_question() {
        let mut spec = building_component_spec();
        let question_id = spec.steps[0].blocks[0].id.clone();
        let operation = update_block_operation(&spec, &question_id, |question| question.label = "Renamed".into()).expect("operation");
        spec = apply_form_edit_operation(&spec, &operation);
        assert_eq!(spec.steps[0].blocks[0].label, "Renamed");
    }

    fn apply_form_edit_operation(spec: &FormSpec, operation: &FormOperation) -> FormSpec {
        crate::artifacts::forms::op::apply_form_edit_operation(spec, operation)
    }

    #[test]
    fn create_form_id_is_unique_per_call() {
        assert_ne!(create_form_id("q"), create_form_id("q"));
    }

    #[test]
    fn forms_play_step_tree_id_prefixes_with_step() {
        assert_eq!(forms_play_step_tree_id("s1"), "step:s1");
    }

    #[test]
    fn dsl_value_conversions_round_trip_through_json() {
        // 🩹️ A whole-number float (e.g. `6.0`) round-trips through `dsl::DslValue` as the integer-typed
        // `serde_json::Number` (`6`), which does not `==` the float-typed literal despite being numerically
        // equal — a `dsl` value-system characteristic, not something this conversion controls. Use a
        // fractional value here so the round-trip assertion is unambiguous.
        let value = json!({ "height": 6.5 });
        assert_eq!(dsl_to_value(&value_to_dsl(&value)), value);
        assert_eq!(dsl_string_value(&value_to_dsl(&json!("hello"))), "hello");
        assert_eq!(dsl_f64_value(&value_to_dsl(&json!(42.5))), 42.5);
    }

    #[test]
    fn flatten_questions_lists_every_block_across_steps() {
        let spec = onboarding_example_spec();
        assert!(!flatten_questions(&spec).is_empty());
    }

    #[test]
    fn json_value_helpers_stringify_primitives() {
        assert_eq!(json_string_value(&json!("a")), "a");
        assert_eq!(json_string_value(&json!(true)), "true");
        assert_eq!(json_string_value(&Value::Null), "");
        assert_eq!(json_f64_value(&json!(5)), 5.0);
        assert_eq!(json_f64_value(&Value::Null), 0.0);
    }

    #[test]
    fn default_and_onboarding_examples_parse_and_serialize() {
        assert!(!default_example_json().is_empty());
        assert!(!onboarding_example_json().is_empty());
    }
}
//#endregion 🧪️Tests
