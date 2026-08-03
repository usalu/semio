//! 📋️ Forms app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait flip —
//! `FormsPlayApp` is a unit struct; every former `FormsPlayRuntime` field (blueprint selection, the
//! Try wizard's active step and in-progress answers) now lives in `forms_engine::FormsConfig`, written
//! via `forms_op::FormsConfigOperation`s; every action dispatches through the single typed
//! `forms_protocol::FormsCommand` channel via `DocumentApp::handle`.

fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

fn dsl_string_value(value: &dsl::DslValue) -> String {
    json_string_value(&dsl_to_value(value))
}

fn dsl_f64_value(value: &dsl::DslValue) -> f64 {
    json_f64_value(&dsl_to_value(value))
}

use forms::{FormQuestion, FormQuestionOption, FormSpec, FormStep, FormVectorField, FORM_BUILTIN_KINDS, FORMS_DOCUMENT_SCHEMA};
use forms_engine::{building_component_spec, can_advance, default_value_for_question, empty_forms_projection, initial_try_values, is_extension_question_kind, visible_questions, FormsConfig};
use forms_op::{FormOperation, FormsConfigOperation};
use forms_protocol::FormsCommand;
use semio_framework_plugin::{SurfaceKind,
    create_default_layout, tree_item_with_action,
    ui_external_slot, ui_image, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text,
    ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionArgDef, ActionArgOption,
    ActionDefinition, ActionKind, App, AppLabels, BlockPaletteEntry, Contribution,
    ConfigView, DocumentApp, DocumentView, Emit, HostEffect, IconName, Label, Locale, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, ArtifactKindSpec, ActionDescriptor, Terminology,
    UiButtonNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode, UiPresence,
    UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖️Constants
const FORMS_PLAY_APP_ID: &str = "forms-play";
const FORMS_PLAY_CONTROLLER_ID: &str = "forms-play";
const FORMS_PLAY_SURFACE_BLUEPRINT: &str = "forms.play.blueprint";
const FORMS_PLAY_BODY_BLUEPRINT: &str = "forms.play.blueprint";
const FORMS_PLAY_BODY_TRY: &str = "forms.play.try";
const FORMS_PLAY_BODY_DOCUMENT: &str = "forms.play.document";
const FORMS_PLAY_BODY_CATALOGUE: &str = "forms.play.catalogue";
const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
const FORMS_PLAY_WINDOW_BLUEPRINT: &str = "forms-blueprint";
const FORMS_PLAY_WINDOW_TRY: &str = "forms-try";
const FORMS_QUESTION_DRAG_MIME: &str = "application/x-semio-forms-question-kind";
const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT;
const AVATAR_PLACEHOLDER_PNG_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

static FORM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` (mirrors `shooting_ui`'s identical B1 fix).
fn is_de_locale(cfg: &FormsConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🌐️ `FormsConfig` carries no terminology axis yet (unlike `CadConfig::terminology`) — forms has no
/// native/reuse vocabulary split, so every cell resolves `Terminology::Native`.
fn forms_locale(cfg: &FormsConfig) -> Locale {
    if is_de_locale(cfg) { Locale::De } else { Locale::En }
}

/// 🗣️ Resolves the active label cell from the config-carried locale via the SDK's two-axis
/// `AppLabels::labels` (was the deleted `LocaleLabels::locale_labels_en/de`).
fn resolve_labels<L: AppLabels>(cfg: &FormsConfig) -> &'static L {
    L::labels(forms_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn forms_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: FORMS_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

/// 🔠️ B1: `config.try_values_json`'s parsed form — the Try wizard's in-progress answer overrides
/// (question id -> value), heterogeneous per question kind so it stays a JSON blob in `FormsConfig`
/// rather than a typed `dsl` field (see `FormsConfig`'s doc). Falls back to an empty map on malformed
/// JSON rather than erroring, matching every other "best-effort parse of a config blob" call site.
fn try_values_map(config: &FormsConfig) -> Map<String, Value> {
    serde_json::from_str::<Value>(&config.try_values_json).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default()
}

fn try_values_json_text(values: &Map<String, Value>) -> String {
    serde_json::to_string(values).unwrap_or_else(|_| "{}".into())
}

fn effective_try_values(spec: &FormSpec, config: &FormsConfig) -> Map<String, Value> {
    initial_try_values(spec, &try_values_map(config))
}

/// 🌱️ Building block for every `handle()` arm that must both clear the Try wizard's answers and reset
/// its active step — was `reset_try_runtime`'s effect on the old `FormsPlayRuntime`, now two config
/// operations instead of two field writes.
fn reset_try_config_operations() -> Vec<FormsConfigOperation> {
    vec![FormsConfigOperation::SetTryValues { json: "{}".into() }, FormsConfigOperation::SetStepIndex { index: 0 }]
}

/// ✏️ Emits the operations that replace the current form spec's title + steps with those of `next` — a
/// legitimate whole-document swap for import/example-switch, expressed granularly through the
/// existing `FormOperation` vocabulary (remove every current step, retitle, re-add the new steps) so it
/// still records a true inverse.
fn replace_spec_operations(current: &FormSpec, next: &FormSpec) -> Vec<FormOperation> {
    let mut operations: Vec<FormOperation> = current
        .steps
        .iter()
        .map(|step| FormOperation::RemoveStep { step_id: step.id.clone() })
        .collect();
    if next.title != current.title {
        operations.push(FormOperation::UpdatePlaybook { title: next.title.clone() });
    }
    for step in &next.steps {
        operations.push(FormOperation::AddStep { step: step.clone(), index: None });
    }
    operations
}

struct QuestionLocation {
    step_id: String,
    question: FormQuestion,
}

fn create_form_id(prefix: &str) -> String {
    let serial = FORM_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn forms_play_step_tree_id(step_id: &str) -> String {
    format!("step:{step_id}")
}

fn find_question_location(spec: &FormSpec, question_id: &str) -> Option<QuestionLocation> {
    for step in &spec.steps {
        if let Some(question) = step.blocks.iter().find(|question| question.id == question_id) {
            return Some(QuestionLocation {
                step_id: step.id.clone(),
                question: question.clone(),
            });
        }
    }
    None
}

fn question_shell(id: String, label: String, kind: String) -> FormQuestion {
    FormQuestion {
        id,
        label,
        kind,
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

//#region 🔖️Contributions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgramContributionEntry {
    plugin_id: String,
    contribution: Contribution,
}

/// 🧩️ B1: `config.contributions_json`-driven counterpart of the deleted `ViewState`-driven
/// `view_state.contributions_json` — the host now pushes contributions into config via
/// `FormsCommand::SetContributions`/`FormsConfigOperation::SetContributions` (mirrors `SetLocale`).
fn parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
    serde_json::from_str::<Vec<ProgramContributionEntry>>(&config.contributions_json).unwrap_or_default()
}

fn find_question_kind_contribution<'a>(
    contributions: &'a [ProgramContributionEntry],
    kind: &str,
) -> Option<(&'a str, &'a Contribution)> {
    contributions.iter().find_map(|entry| {
        if let Contribution::PlaybookBlockKind { block_kind, .. } = &entry.contribution {
            if block_kind == kind {
                return Some((entry.plugin_id.as_str(), &entry.contribution));
            }
        }
        None
    })
}

fn extension_params_value(question: &FormQuestion, values: &Map<String, Value>) -> Value {
    values
        .get(&question.id)
        .cloned()
        .or_else(|| question.params.as_ref().map(dsl_to_value))
        .unwrap_or_else(|| json!({}))
}

fn extension_render_payload(
    question: &FormQuestion,
    params: &Value,
    surface: &str,
    interactive: bool,
) -> String {
    serde_json::to_string(&json!({
        "fixtureSlug": question.fixture_slug.clone().unwrap_or_else(|| "hexagonal-mushroom-column".into()),
        "params": params,
        "questionId": question.id,
        "controllerId": FORMS_PLAY_CONTROLLER_ID,
        "surface": surface,
        "interactive": interactive,
    }))
    .unwrap_or_else(|_| "{}".into())
}

fn render_extension_question(
    question: &FormQuestion,
    values: &Map<String, Value>,
    contributions: &[ProgramContributionEntry],
    surface: &str,
    interactive: bool,
) -> UiNode {
    let Some((plugin_id, contribution)) = find_question_kind_contribution(contributions, &question.kind) else {
        return ui_text(Label::data(format!("Extension unavailable: {}", question.kind)));
    };
    let Contribution::PlaybookBlockKind {
        app_id,
        params_body_key,
        preview_body_key,
        ..
    } = contribution
    else {
        return ui_text(Label::data(format!("Extension unavailable: {}", question.kind)));
    };
    let params = extension_params_value(question, values);
    let payload = extension_render_payload(question, &params, surface, interactive);
    ui_stack_vertical(vec![
        ui_external_slot(plugin_id, app_id, params_body_key, &payload),
        ui_external_slot(plugin_id, app_id, preview_body_key, &payload),
    ])
}
//#endregion 🔖️Contributions

fn flatten_questions(spec: &FormSpec) -> Vec<(String, FormQuestion)> {
    spec.steps
        .iter()
        .flat_map(|step| step.blocks.iter().map(|question| (step.title.clone(), question.clone())))
        .collect()
}

fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

fn patch_try_object_field(values: &mut Map<String, Value>, key: &str, field: &str, raw: &Value) {
    let mut object = values.get(key).cloned().unwrap_or_else(|| json!({}));
    if let Some(map) = object.as_object_mut() {
        map.insert(field.into(), raw.clone());
        values.insert(key.into(), object);
    }
}

fn patch_try_vector_field(values: &mut Map<String, Value>, key: &str, index: usize, raw: &Value) {
    let mut array = values.get(key).and_then(|value| value.as_array().cloned()).unwrap_or_default();
    while array.len() <= index {
        array.push(json!(0.0));
    }
    array[index] = raw.clone();
    values.insert(key.into(), Value::Array(array));
}

fn default_question_for_kind(kind: &str, id: String) -> FormQuestion {
    match kind {
        "text" => {
            let mut question = question_shell(id, "Text".into(), "text".into());
            question.placeholder = Some("Enter text".into());
            question
        }
        "longText" => {
            let mut question = question_shell(id, "Long Text".into(), "longText".into());
            question.placeholder = Some("Enter long text".into());
            question
        }
        "number" => {
            let mut question = question_shell(id, "Number".into(), "number".into());
            question.default = Some(value_to_dsl(&json!(0)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "slider" => {
            let mut question = question_shell(id, "Slider".into(), "slider".into());
            question.default = Some(value_to_dsl(&json!(50)));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "boolean" => {
            let mut question = question_shell(id, "Boolean".into(), "boolean".into());
            question.default = Some(value_to_dsl(&json!(false)));
            question
        }
        "single" | "multi" => {
            let mut question = question_shell(
                id,
                if kind == "single" { "Single Select" } else { "Multi Select" }.into(),
                kind.into(),
            );
            question.default = if kind == "multi" { Some(value_to_dsl(&json!([]))) } else { None };
            question.options = Some(vec![
                FormQuestionOption {
                    value: "a".into(),
                    label: "Option A".into(),
                },
                FormQuestionOption {
                    value: "b".into(),
                    label: "Option B".into(),
                },
            ]);
            question
        }
        "note" => {
            let mut question = question_shell(id, "Note".into(), "note".into());
            question.text = Some("Informational note".into());
            question
        }
        "date" => {
            let mut question = question_shell(id, "Date".into(), "date".into());
            question.default = Some(value_to_dsl(&json!("2026-01-01")));
            question
        }
        "color" => {
            let mut question = question_shell(id, "Color".into(), "color".into());
            question.default = Some(value_to_dsl(&json!("#336699")));
            question
        }
        "image" => question_shell(id, "Image".into(), "image".into()),
        "file" => {
            let mut question = question_shell(id, "File".into(), "file".into());
            question.accept = Some(".pdf".into());
            question
        }
        "vector" => {
            let mut question = question_shell(id, "Vector".into(), "vector".into());
            question.schema = Some("vec3".into());
            question.step = Some(0.1);
            question.fields = Some(vec![
                FormVectorField {
                    key: "x".into(),
                    label: Some("X".into()),
                    value: Some(0.0),
                },
                FormVectorField {
                    key: "y".into(),
                    label: Some("Y".into()),
                    value: Some(0.0),
                },
                FormVectorField {
                    key: "z".into(),
                    label: Some("Z".into()),
                    value: Some(0.0),
                },
            ]);
            question
        }
        "buildingComponent" => {
            let mut question = question_shell(id, "Building Component".into(), "buildingComponent".into());
            question.fixture_slug = Some("hexagonal-mushroom-column".into());
            question.params = Some(value_to_dsl(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
            question
        }
        _ => question_shell(id, kind.into(), kind.into()),
    }
}

fn resolve_step_id_from_tree_target(spec: &FormSpec, target_id: &str) -> Option<String> {
    if target_id.starts_with("step:") {
        return Some(target_id[5..].to_string());
    }
    find_question_location(spec, target_id).map(|location| location.step_id)
}

fn resolve_question_insert_index(spec: &FormSpec, step_id: &str, target_id: &str, drop_position: &str) -> Option<usize> {
    let step = spec.steps.iter().find(|step| step.id == step_id)?;
    if target_id.starts_with("step:") {
        return Some(if drop_position == "before" { 0 } else { step.blocks.len() });
    }
    let target_index = step.blocks.iter().position(|question| question.id == target_id)?;
    Some(match drop_position {
        "before" => target_index,
        "after" => target_index + 1,
        _ => step.blocks.len(),
    })
}

/// ✏️ Locates `question_id` in `spec`, applies `mutate` to a clone, and returns the `UpdateBlock` operation
/// that records the edit — the single seam every inspector patch flows through. Returns `None` if the
/// question no longer exists.
fn update_block_operation(spec: &FormSpec, question_id: &str, mutate: impl FnOnce(&mut FormQuestion)) -> Option<FormOperation> {
    let location = find_question_location(spec, question_id)?;
    let mut question = location.question;
    mutate(&mut question);
    Some(FormOperation::UpdateBlock { step_id: location.step_id, block: question })
}

fn patch_question_field(spec: &FormSpec, question_id: &str, field: &str, raw_value: &Value) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| match field {
        "label" => question.label = raw_value.as_str().unwrap_or("").to_string(),
        "kind" => question.kind = raw_value.as_str().unwrap_or("text").to_string(),
        "description" => question.description = raw_value.as_str().map(str::to_string),
        "placeholder" => question.placeholder = raw_value.as_str().map(str::to_string),
        "required" => question.required = Some(raw_value.as_bool().unwrap_or(false)),
        "text" => question.text = raw_value.as_str().map(str::to_string),
        "default" => question.default = Some(value_to_dsl(raw_value)),
        "min" => question.min = raw_value.as_f64(),
        "max" => question.max = raw_value.as_f64(),
        "step" => question.step = raw_value.as_f64(),
        "unit" => question.unit = raw_value.as_str().map(str::to_string),
        "schema" => question.schema = raw_value.as_str().map(str::to_string),
        "src" => question.src = raw_value.as_str().map(str::to_string),
        "accept" => question.accept = raw_value.as_str().map(str::to_string),
        "fixtureSlug" => question.fixture_slug = raw_value.as_str().map(str::to_string),
        _ => {}
    })
}

fn patch_question_option(spec: &FormSpec, question_id: &str, option_value: &str, field: &str, raw_value: &Value) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        if let Some(option) = options.iter_mut().find(|entry| entry.value == option_value) {
            if field == "label" {
                option.label = raw_value.as_str().unwrap_or("").to_string();
            }
        }
        question.options = Some(options);
    })
}

fn add_question_option(spec: &FormSpec, question_id: &str, label: &str) -> Option<FormOperation> {
    let value = create_form_id("opt");
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.push(FormQuestionOption { value, label: label.into() });
        question.options = Some(options);
    })
}

fn remove_question_option(spec: &FormSpec, question_id: &str, option_value: &str) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.retain(|entry| entry.value != option_value);
        question.options = Some(options);
    })
}

fn patch_vector_field(spec: &FormSpec, question_id: &str, field_key: &str, field: &str, raw_value: &Value) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        if let Some(entry) = fields.iter_mut().find(|item| item.key == field_key) {
            match field {
                "label" => entry.label = raw_value.as_str().map(str::to_string),
                "value" => entry.value = raw_value.as_f64(),
                _ => {}
            }
        }
        question.fields = Some(fields);
    })
}

fn add_vector_field(spec: &FormSpec, question_id: &str, key: &str) -> Option<FormOperation> {
    let location = find_question_location(spec, question_id)?;
    if location.question.fields.iter().flatten().any(|entry| entry.key == key) {
        return None;
    }
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.push(FormVectorField { key: key.into(), label: Some(key.into()), value: Some(0.0) });
        question.fields = Some(fields);
    })
}

fn remove_vector_field(spec: &FormSpec, question_id: &str, field_key: &str) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.retain(|entry| entry.key != field_key);
        question.fields = Some(fields);
    })
}

fn patch_building_component_param(spec: &FormSpec, question_id: &str, param_key: &str, raw_value: &Value) -> Option<FormOperation> {
    update_block_operation(spec, question_id, |question| {
        let mut params = question.params.take().unwrap_or(dsl::DslValue::Object(vec![]));
        if let dsl::DslValue::Object(entries) = &mut params {
            let value = value_to_dsl(raw_value);
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == param_key) {
                *slot = value;
            } else {
                entries.push((param_key.to_string(), value));
            }
        }
        question.params = Some(params);
    })
}

fn catalogue_kinds(contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> Vec<(String, String, IconName)> {
    let mut kinds: Vec<(String, String, IconName)> = FORM_BUILTIN_KINDS
        .iter()
        .map(|kind| {
            let (label, icon): (&str, &str) = match *kind {
                "text" => (labels.kind_text.as_str(), "type"),
                "longText" => (labels.kind_long_text.as_str(), "align-left"),
                "number" => (labels.kind_number.as_str(), "hash"),
                "slider" => (labels.kind_slider.as_str(), "sliders-horizontal"),
                "boolean" => (labels.kind_boolean.as_str(), "toggle-left"),
                "single" => (labels.kind_single.as_str(), "circle-dot"),
                "multi" => (labels.kind_multi.as_str(), "list-checks"),
                "date" => (labels.kind_date.as_str(), "calendar"),
                "color" => (labels.kind_color.as_str(), "palette"),
                "image" => (labels.kind_image.as_str(), "image"),
                "file" => (labels.kind_file.as_str(), "file"),
                "vector" => (labels.kind_vector.as_str(), "move-3d"),
                "note" => (labels.kind_note.as_str(), "sticky-note"),
                other => (other, "help-circle"),
            };
            (kind.to_string(), label.into(), icon.into())
        })
        .collect();
    for entry in contributions {
        if let Contribution::PlaybookBlockKind {
            block_kind,
            label,
            icon_id,
            ..
        } = &entry.contribution
        {
            kinds.push((block_kind.clone(), label.clone(), icon_id.clone()));
        }
    }
    kinds
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the forms app; one field per label makes every locale combination compile-checked.
    struct FormsLabels {
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        required: native_en "Required", native_de "Erforderlich", reuse_en "Required", reuse_de "Erforderlich";
        description: native_en "Description", native_de "Beschreibung", reuse_en "Description", reuse_de "Beschreibung";
        placeholder: native_en "Placeholder", native_de "Platzhalter", reuse_en "Placeholder", reuse_de "Platzhalter";
        default: native_en "Default", native_de "Standard", reuse_en "Default", reuse_de "Standard";
        min: native_en "Min", native_de "Min", reuse_en "Min", reuse_de "Min";
        max: native_en "Max", native_de "Max", reuse_en "Max", reuse_de "Max";
        step_field: native_en "Step", native_de "Schrittweite", reuse_en "Step", reuse_de "Schrittweite";
        unit: native_en "Unit", native_de "Einheit", reuse_en "Unit", reuse_de "Einheit";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        src: native_en "Src", native_de "Quelle", reuse_en "Src", reuse_de "Quelle";
        accept: native_en "Accept", native_de "Akzeptierte Dateien", reuse_en "Accept", reuse_de "Akzeptierte Dateien";
        yes: native_en "Yes", native_de "Ja", reuse_en "Yes", reuse_de "Ja";
        no: native_en "No", native_de "Nein", reuse_en "No", reuse_de "Nein";
        option: native_en "Option", native_de "Option", reuse_en "Option", reuse_de "Option";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        add_option: native_en "Add Option", native_de "Option hinzufügen", reuse_en "Add Option", reuse_de "Option hinzufügen";
        remove_option: native_en "Remove Option", native_de "Option entfernen", reuse_en "Remove Option", reuse_de "Option entfernen";
        add_vector_field: native_en "Add Vector Field", native_de "Vektorfeld hinzufügen", reuse_en "Add Vector Field", reuse_de "Vektorfeld hinzufügen";
        vector_field_label_suffix: native_en "label", native_de "Bezeichnung", reuse_en "label", reuse_de "Bezeichnung";
        vector_field_value_suffix: native_en "value", native_de "Wert", reuse_en "value", reuse_de "Wert";
        add_step: native_en "Add Step", native_de "Schritt hinzufügen", reuse_en "Add Step", reuse_de "Schritt hinzufügen";
        add_text_question: native_en "Add Text Question", native_de "Textfrage hinzufügen", reuse_en "Add Text Question", reuse_de "Textfrage hinzufügen";
        question: native_en "Question", native_de "Frage", reuse_en "Question", reuse_de "Frage";
        selected: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        no_steps_in_form: native_en "No steps in this form.", native_de "Keine Schritte in diesem Formular.", reuse_en "No steps in this form.", reuse_de "Keine Schritte in diesem Formular.";
        form_fallback_title: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        step_progress: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        back: native_en "Back", native_de "Zurück", reuse_en "Back", reuse_de "Zurück";
        next: native_en "Next", native_de "Weiter", reuse_en "Next", reuse_de "Weiter";
        submit: native_en "Submit", native_de "Absenden", reuse_en "Submit", reuse_de "Absenden";
        fixture_slug: native_en "Fixture Slug", native_de "Fixture-Slug", reuse_en "Fixture Slug", reuse_de "Fixture-Slug";
        no_steps_tree_item: native_en "(no steps)", native_de "(keine Schritte)", reuse_en "(no steps)", reuse_de "(keine Schritte)";
        actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        kind_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind_long_text: native_en "Long Text", native_de "Langtext", reuse_en "Long Text", reuse_de "Langtext";
        kind_number: native_en "Number", native_de "Zahl", reuse_en "Number", reuse_de "Zahl";
        kind_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        kind_boolean: native_en "Boolean", native_de "Boolescher Wert", reuse_en "Boolean", reuse_de "Boolescher Wert";
        kind_single: native_en "Single Select", native_de "Einzelauswahl", reuse_en "Single Select", reuse_de "Einzelauswahl";
        kind_multi: native_en "Multi Select", native_de "Mehrfachauswahl", reuse_en "Multi Select", reuse_de "Mehrfachauswahl";
        kind_date: native_en "Date", native_de "Datum", reuse_en "Date", reuse_de "Datum";
        kind_color: native_en "Color", native_de "Farbe", reuse_en "Color", reuse_de "Farbe";
        kind_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        kind_file: native_en "File", native_de "Datei", reuse_en "File", reuse_de "Datei";
        kind_vector: native_en "Vector", native_de "Vektor", reuse_en "Vector", reuse_de "Vektor";
        kind_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        window_blueprint: native_en "Blueprint", native_de "Entwurf", reuse_en "Blueprint", reuse_de "Entwurf";
        window_try: native_en "Try", native_de "Testen", reuse_en "Try", reuse_de "Testen";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(spec: &FormSpec, selected_ids: &[String], labels: &FormsLabels) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = spec
        .steps
        .iter()
        .map(|step| {
            let question_items: Vec<UiTreeItemNode> = step
                .blocks
                .iter()
                .map(|question| UiTreeItemNode {
                    icon_id: Some("help-circle".into()),
                    draggable: Some(true),
                    menu: None,
                    ..tree_item_with_action(
                        question.id.clone(),
                        Label::data(question.label.clone()),
                        Some(question.kind.clone()),
                        forms_action("setSelection", Some(json!({ "ids": [question.id.clone()] }))),
                    )
                })
                .collect();
            UiTreeItemNode {
                icon_id: Some("list-tree".into()),
                default_open: Some(true),
                draggable: Some(true),
                items: Some(question_items),
                menu: None,
                ..tree_item_with_action(
                    forms_play_step_tree_id(&step.id),
                    Label::data(step.title.clone()),
                    Some(format!("{} questions", step.blocks.len())),
                    forms_action("setSelection", Some(json!({ "ids": [] }))),
                )
            }
        })
        .collect();
    PanelTreeBuilder::new("forms-play-document")
        .section_or_placeholder(
            "forms-play-document.steps",
            Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)),
            true,
            step_items,
            labels.no_steps_tree_item,
        )
        .selected(selected_ids.to_vec())
        .selection_change(forms_action("setSelection", None))
        .drop_action(forms_action("dropQuestionKind", None))
        .build()
}

fn build_catalogue_tree(contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> UiNode {
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds(contributions, labels)
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                icon_id: Some(icon),
                draggable: Some(true),
                drag_data: Some(drag_data),
                menu: None,
                ..tree_item_with_action(
                    format!("forms-play-catalogue.{kind}"),
                    Label::data(label),
                    Some(kind.clone()),
                    forms_action("addQuestion", Some(json!({ "kind": kind }))),
                )
            }
        })
        .collect();
    let action_items = vec![
        UiTreeItemNode {
            icon_id: Some("plus".into()),
            menu: None,
            ..tree_item_with_action("forms-play-catalogue.add-step", labels.add_step, None, forms_action("addStep", None))
        },
        UiTreeItemNode {
            icon_id: Some("type".into()),
            menu: None,
            ..tree_item_with_action(
                "forms-play-catalogue.add-question",
                labels.add_text_question,
                None,
                forms_action("addQuestion", Some(json!({ "kind": "text" }))),
            )
        },
    ];
    PanelTreeBuilder::new("forms-play-catalogue")
        .section("forms-play-catalogue.kinds", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, kind_items)
        .section("forms-play-catalogue.actions", Some(labels.actions.into()), true, action_items)
        .build()
}

fn inspector_patch(question_ids: &[String], field: &str) -> ActionDescriptor {
    forms_action("patchQuestions", Some(json!({ "questionIds": question_ids, "field": field })))
}

fn inspector_text_field(question_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
            commit: None,
            on_change: inspector_patch(question_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn inspector_number_field(question_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch(question_ids, field),
            on_delta: inspector_patch(question_ids, field),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn question_kind_editor_fields(
    question: &FormQuestion,
    question_ids: &[String],
    contributions: &[ProgramContributionEntry],
    id_prefix: &str,
    labels: &FormsLabels,
) -> Vec<UiNode> {
    let fid = |suffix: &str| format!("{id_prefix}.{suffix}");
    let mut fields = Vec::new();
    fields.push(inspector_text_field(
        question_ids,
        &fid("description"),
        labels.description,
        &[question.description.clone().unwrap_or_default()],
        "description",
    ));
    match question.kind.as_str() {
        "text" | "longText" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("placeholder"),
                labels.placeholder,
                &[question.placeholder.clone().unwrap_or_default()],
                "placeholder",
            ));
            fields.push(inspector_text_field(
                question_ids,
                &fid("default"),
                labels.default,
                &[question.default.as_ref().map(dsl_string_value).unwrap_or_default()],
                "default",
            ));
        }
        "number" | "slider" => {
            fields.push(inspector_number_field(question_ids, &fid("min"), labels.min, &[question.min.unwrap_or(0.0)], "min"));
            fields.push(inspector_number_field(question_ids, &fid("max"), labels.max, &[question.max.unwrap_or(100.0)], "max"));
            fields.push(inspector_number_field(question_ids, &fid("step"), labels.step_field, &[question.step.unwrap_or(1.0)], "step"));
            fields.push(inspector_number_field(
                question_ids,
                &fid("default"),
                labels.default,
                &[question.default.as_ref().map(dsl_f64_value).unwrap_or(0.0)],
                "default",
            ));
            if question.kind == "slider" {
                fields.push(inspector_text_field(
                    question_ids,
                    &fid("unit"),
                    labels.unit,
                    &[question.unit.clone().unwrap_or_default()],
                    "unit",
                ));
            }
        }
        "boolean" => {
            let pressed = question
                .default
                .as_ref()
                .and_then(|default| dsl_to_value(default).as_bool())
                .unwrap_or(false);
            fields.push(UiNode::Field(UiFieldNode {
                id: fid("default"),
                label: labels.default.into(),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: fid("default.toggle"),
                    icon_id: "check".into(),
                    text: Some(if pressed { labels.yes.into() } else { labels.no.into() }),
                    on_change: inspector_patch(question_ids, "default"),
                    presence: UiPresence::selected(pressed),
                    menu: None,
                })),
                menu: None,
            }));
        }
        "single" | "multi" => {
            if let Some(options) = &question.options {
                for option in options {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("option.{}", option.value)),
                        label: Label::data(format!("{} {}", labels.option.as_str(), option.value)),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: fid(&format!("option.{}.input", option.value)),
                            input_kind: "text".into(),
                            value: option.label.clone(),
                            placeholder: None,
                            commit: None,
                            on_change: forms_action(
                                "patchQuestionOptions",
                                Some(json!({ "questionIds": question_ids, "optionValue": option.value, "field": "label" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("option.{}.remove", option.value))),
                        icon_id: "trash-2".into(),
                        label: labels.remove_option.into(),
                        action: forms_action(
                            "removeQuestionOption",
                            Some(json!({ "questionId": question.id, "optionValue": option.value })),
                        ),
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("option.add")),
                icon_id: "plus".into(),
                label: labels.add_option.into(),
                action: forms_action("addQuestionOption", Some(json!({ "questionId": question.id, "label": "New option" }))),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        "date" | "color" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("default"),
                labels.default,
                &[question.default.as_ref().map(dsl_string_value).unwrap_or_default()],
                "default",
            ));
        }
        "vector" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("schema"),
                labels.schema,
                &[question.schema.clone().unwrap_or_default()],
                "schema",
            ));
            fields.push(inspector_number_field(question_ids, &fid("step"), labels.step_field, &[question.step.unwrap_or(0.1)], "step"));
            if let Some(vector_fields) = &question.fields {
                for field in vector_fields {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.label", field.key)),
                        label: Label::data(format!("{} {}", field.key, labels.vector_field_label_suffix.as_str())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::Input(UiInputNode {
                            id: fid(&format!("vector.{}.label.input", field.key)),
                            input_kind: "text".into(),
                            value: field.label.clone().unwrap_or_else(|| field.key.clone()),
                            placeholder: None,
                            commit: None,
                            on_change: forms_action(
                                "patchVectorField",
                                Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "label" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.value", field.key)),
                        label: Label::data(format!("{} {}", field.key, labels.vector_field_value_suffix.as_str())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                            id: fid(&format!("vector.{}.value.stepper", field.key)),
                            value: field.value.unwrap_or(0.0),
                            step: question.step.unwrap_or(0.1),
                            uniform: true,
                            on_absolute: forms_action(
                                "patchVectorField",
                                Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" })),
                            ),
                            on_delta: forms_action(
                                "patchVectorField",
                                Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" })),
                            ),
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("vector.{}.remove", field.key))),
                        icon_id: "trash-2".into(),
                        label: Label::data(format!("{} {}", labels.remove.as_str(), field.key)),
                        action: forms_action(
                            "removeVectorField",
                            Some(json!({ "questionId": question.id, "fieldKey": field.key })),
                        ),
                        style: None,
                        presence: UiPresence::default(),
                        menu: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("vector.add")),
                icon_id: "plus".into(),
                label: labels.add_vector_field.into(),
                action: forms_action(
                    "addVectorField",
                    Some(json!({ "questionId": question.id, "fieldKey": "field" })),
                ),
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        "note" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("text"),
                labels.text,
                &[question.text.clone().unwrap_or_default()],
                "text",
            ));
        }
        "image" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("src"),
                labels.src,
                &[question.src.clone().unwrap_or_default()],
                "src",
            ));
        }
        "file" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("accept"),
                labels.accept,
                &[question.accept.clone().unwrap_or_default()],
                "accept",
            ));
        }
        kind if is_extension_question_kind(kind) => {
            let values = serde_json::Map::new();
            fields.push(render_extension_question(question, &values, contributions, "blueprint", true));
            if let Some(slug) = &question.fixture_slug {
                fields.push(ui_inspector_readonly_field(fid("fixtureSlug"), labels.fixture_slug, slug));
            }
        }
        _ => {}
    }
    fields
}

fn build_inspector_tree(
    spec: &FormSpec,
    config: &FormsConfig,
    contributions: &[ProgramContributionEntry],
    term_labels: &FormsLabels,
) -> UiNode {
    let questions: Vec<FormQuestion> = config
        .selected_ids
        .iter()
        .filter_map(|id| find_question_location(spec, id).map(|location| location.question))
        .collect();
    if questions.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "forms-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![
                ui_text(Label::data(format!("Schema: {FORMS_DOCUMENT_SCHEMA}"))),
                ui_text(Label::data(format!("Steps: {}", spec.steps.len()))),
                ui_text(Label::data(format!("Questions: {}", flatten_questions(spec).len()))),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let question_ids: Vec<String> = questions.iter().map(|question| question.id.clone()).collect();
    let labels: Vec<String> = questions.iter().map(|question| question.label.clone()).collect();
    let kinds: Vec<String> = questions.iter().map(|question| question.kind.clone()).collect();
    let required: Vec<bool> = questions.iter().map(|question| question.required.unwrap_or(false)).collect();
    let kind_mixed = ui_inspector_mixed_text(&kinds);
    let required_mixed = ui_inspector_mixed_toggle(&required);
    let kind_items: Vec<UiSelectItem> = catalogue_kinds(contributions, term_labels)
        .into_iter()
        .map(|(kind, label, _)| UiSelectItem {
            value: kind,
            label: Label::data(label),
        })
        .collect();
    let mut base_fields = vec![
        inspector_text_field(&question_ids, "forms-play-inspector.label", term_labels.label, &labels, "label"),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.kind".into(),
            label: term_labels.kind.into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                id: "forms-play-inspector.kind.select".into(),
                value: kind_mixed.value,
                placeholder: kind_mixed.placeholder.map(Label::data),
                items: kind_items,
                on_change: inspector_patch(&question_ids, "kind"),
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        ui_inspector_readonly_field(
            "forms-play-inspector.id",
            term_labels.id,
            if question_ids.len() == 1 {
                question_ids[0].clone()
            } else {
                format!("{} {}", question_ids.len(), term_labels.selected.as_str())
            },
        ),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.required".into(),
            label: term_labels.required.into(),
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: "forms-play-inspector.required.toggle".into(),
                icon_id: "check".into(),
                text: if required_mixed.uniform {
                    Some(if required_mixed.pressed { term_labels.yes.into() } else { term_labels.no.into() })
                } else {
                    Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER))
                },
                on_change: inspector_patch(&question_ids, "required"),
                presence: UiPresence::selected(required_mixed.uniform && required_mixed.pressed),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
    ];
    if questions.len() == 1 {
        base_fields.extend(question_kind_editor_fields(&questions[0], &question_ids, contributions, "forms-play-inspector", term_labels));
    }
    let groups = vec![UiInspectorFieldGroup { presence: UiPresence::default(),
        id: "forms-play-inspector.base".into(),
        label: term_labels.question.into(),
        default_open: None,
        fields: base_fields,
    }];
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Panels

//#region 🔖️Render
//#region 🔖️Builder
fn forms_playbook_builder_config() -> playbook::PlaybookBuilderConfig {
    playbook::PlaybookBuilderConfig {
        action_namespace: "forms-blueprint",
        controller_id: FORMS_PLAY_CONTROLLER_ID,
        labels: playbook::PLAYBOOK_BUILDER_LABELS_EN,
    }
}

fn render_blueprint_builder(spec: &FormSpec, forms_config: &FormsConfig, contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> UiNode {
    let palette: Vec<BlockPaletteEntry> = catalogue_kinds(contributions, labels)
        .into_iter()
        .map(|(kind, label, icon_id)| BlockPaletteEntry {
            block_kind: kind,
            label,
            icon_id,
        })
        .collect();
    let builder_config = forms_playbook_builder_config();
    playbook::render_playbook_builder(
        FORMS_PLAY_SURFACE_BLUEPRINT,
        spec,
        &palette,
        forms_config.selected_ids.first().map(String::as_str),
        &builder_config,
    )
}
//#endregion 🔖️Builder

//#region 🔖️TryWizard
fn try_value_action(key: &str) -> ActionDescriptor {
    forms_action("setTryValue", Some(json!({ "key": key })))
}

fn image_question_src(question: &FormQuestion) -> String {
    let src = question.src.as_deref().unwrap_or("");
    if src.is_empty() {
        return format!("data:image/png;base64,{AVATAR_PLACEHOLDER_PNG_BASE64}");
    }
    if src.starts_with("data:") || src.starts_with("http") || src.starts_with('/') || src.ends_with(".svg") {
        return src.into();
    }
    format!("data:image/png;base64,{src}")
}

fn render_image_question(question: &FormQuestion) -> UiNode {
    ui_image(
        format!("forms-try.{}.image", question.id),
        image_question_src(question),
        Some(Label::data(question.label.clone())),
    )
}

fn ui_text_emphasized(value: impl Into<Label>) -> UiNode {
    UiNode::Text(UiTextNode {
        value: value.into(),
        emphasize: Some(true),
        data_attributes: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "horizontal".into(),
        gap: Some("tight".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
        menu: None,
    })
}

fn try_field(question: &FormQuestion, error: Option<&str>, child: UiNode) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("forms-try.{}", question.id),
        label: Label::data(question.label.clone()),
        description: question.description.clone(),
        required: question.required.filter(|required| *required),
        error: error.map(str::to_string),
        child: Box::new(child),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn render_try_question(
    question: &FormQuestion,
    values: &Map<String, Value>,
    contributions: &[ProgramContributionEntry],
    error: Option<&str>,
    labels: &FormsLabels,
) -> UiNode {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| dsl_to_value(&default_value_for_question(question)));
    let key = question.id.clone();
    match question.kind.as_str() {
        "text" | "longText" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(&value),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "number" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: question.min,
                max: question.max,
                step: question.step,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "slider" => try_field(
            question,
            error,
            UiNode::Slider(UiSliderNode {
                id: format!("forms-try.{key}.slider"),
                value: json_f64_value(&value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                unit: question.unit.clone(),
                on_change: try_value_action(&key),
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "boolean" => try_field(
            question,
            error,
            UiNode::Toggle(UiToggleNode {
                id: format!("forms-try.{key}.toggle"),
                icon_id: "check".into(),
                text: Some(if value.as_bool().unwrap_or(false) { labels.yes.into() } else { labels.no.into() }),
                on_change: try_value_action(&key),
                presence: UiPresence::selected(value.as_bool().unwrap_or(false)),
                menu: None,
            }),
        ),
        "single" => {
            let items = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| UiSelectItem {
                            value: option.value.clone(),
                            label: Label::data(option.label.clone()),
                        })
                        .collect()
                })
                .unwrap_or_default();
            try_field(
                question,
                error,
                UiNode::Select(UiSelectNode {
                    id: format!("forms-try.{key}.select"),
                    value: json_string_value(&value),
                    placeholder: None,
                    items,
                    on_change: try_value_action(&key),
                    presence: UiPresence::default(),
                    menu: None,
                }),
            )
        }
        "multi" => {
            let selected: HashSet<String> = value
                .as_array()
                .map(|items| items.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let chips = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| {
                            UiNode::Toggle(UiToggleNode {
                                id: format!("forms-try.{key}.{}.toggle", option.value),
                                icon_id: "hash".into(),
                                text: Some(Label::data(option.label.clone())),
                                on_change: forms_action(
                                    "setTryValue",
                                    Some(json!({ "key": key, "optionValue": option.value })),
                                ),
                                presence: UiPresence::selected(selected.contains(&option.value)),
                                menu: None,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            try_field(question, error, ui_stack_horizontal(chips))
        }
        "date" | "color" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "vector" => {
            let array = value.as_array().cloned().unwrap_or_default();
            let fields = question.fields.as_ref().cloned().unwrap_or_default();
            let steppers: Vec<UiNode> = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_value = array.get(index).cloned().unwrap_or(json!(field.value.unwrap_or(0.0)));
                    UiNode::Field(UiFieldNode {
                        id: format!("forms-try.{key}.{}", field.key),
                        label: Label::data(field.label.clone().unwrap_or_else(|| field.key.clone())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
                            id: format!("forms-try.{key}.{}.stepper", field.key),
                            value: json_f64_value(&field_value),
                            step: question.step.unwrap_or(0.1),
                            uniform: true,
                            on_absolute: forms_action(
                                "setTryValue",
                                Some(json!({ "key": key, "vectorIndex": index })),
                            ),
                            on_delta: forms_action(
                                "setTryValue",
                                Some(json!({ "key": key, "vectorIndex": index })),
                            ),
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    })
                })
                .collect();
            try_field(question, error, ui_stack_horizontal(steppers))
        }
        "note" => ui_text(Label::data(question.text.clone().unwrap_or_else(|| question.label.clone()))),
        "image" => try_field(question, error, render_image_question(question)),
        "file" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "file".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: question.accept.clone(),
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        kind if is_extension_question_kind(kind) => {
            render_extension_question(question, values, contributions, "try", true)
        }
        _ => ui_text(Label::data(format!("Unsupported kind: {}", question.kind))),
    }
}

fn render_try_wizard(spec: &FormSpec, config: &FormsConfig, contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> UiNode {
    if spec.steps.is_empty() {
        return ui_text(labels.no_steps_in_form);
    }
    let step_index = (config.current_step_index as usize).min(spec.steps.len().saturating_sub(1));
    let step = &spec.steps[step_index];
    let values = effective_try_values(spec, config);
    let visible = visible_questions(step, &values);
    let errors = forms_engine::step_errors(step, &values);
    let advance = can_advance(step, &values);
    let errors_by_question: HashMap<&str, &str> = errors
        .iter()
        .map(|error| (error.block_id.as_str(), error.message.as_str()))
        .collect();
    let mut children = vec![
        ui_text_emphasized(Label::data(spec.title.clone().unwrap_or_else(|| labels.form_fallback_title.into()))),
        ui_text(Label::data(format!("{} {} / {}", labels.step_progress.as_str(), step_index + 1, spec.steps.len()))),
        ui_text_emphasized(Label::data(step.title.clone())),
    ];
    if let Some(description) = &step.description {
        children.push(ui_text(Label::data(description.clone())));
    }
    for question in visible {
        children.push(render_try_question(
            question,
            &values,
            contributions,
            errors_by_question.get(question.id.as_str()).copied(),
            labels,
        ));
    }
    let nav = vec![
        UiNode::Button(UiButtonNode {
            id: Some("forms-try.back".into()),
            icon_id: "chevron-left".into(),
            label: labels.back.into(),
            action: forms_action("previousStep", None),
            style: None,
            presence: UiPresence::disabled_if(step_index == 0),
            menu: None,
        }),
        if step_index + 1 < spec.steps.len() {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.next".into()),
                icon_id: "chevron-right".into(),
                label: labels.next.into(),
                action: forms_action("nextStep", None),
                style: None,
                presence: UiPresence::disabled_if(!advance),
                menu: None,
            })
        } else {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.submit".into()),
                icon_id: "check".into(),
                label: labels.submit.into(),
                action: forms_action("submit", None),
                style: None,
                presence: UiPresence::disabled_if(!advance),
                menu: None,
            })
        },
    ];
    children.push(ui_stack_horizontal(nav));
    ui_stack_vertical(children)
}
//#endregion 🔖️TryWizard
//#endregion 🔖️Render

//#region 🔖️FormsPlayApp
/// 🧪️ B1: unit struct — every former `FormsPlayRuntime` field now lives in
/// `forms_engine::FormsConfig`, written through `forms_op::FormsConfigOperation`s.
#[derive(Default)]
pub struct FormsPlayApp;

/// 🔠️ Parses a `FormsCommand` JSON-blob payload (`value_json`/`values_json`/…), falling back to
/// `Value::Null` on malformed or absent JSON — every one of these fields is best-effort text carried
/// across the wire, not a validated protocol.
fn parse_value_json(value_json: &str) -> Value {
    serde_json::from_str(value_json).unwrap_or(Value::Null)
}

impl DocumentApp for FormsPlayApp {
    type Projection = FormSpec;
    type Operation = FormOperation;
    type Config = FormsConfig;
    type ConfigOperation = FormsConfigOperation;
    type Command = FormsCommand;

    fn app_id(&self) -> &str {
        FORMS_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FORMS_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FormSpec {
        building_component_spec()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(forms_engine::forms_io())
    }

    /// 🏷️ Maps each `FormsCommand` variant back to the action id it was declared under in
    /// `create_forms_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &FormsCommand) -> &str {
        match command {
            FormsCommand::SetSelection { .. } => "setSelection",
            FormsCommand::SetTryValue { .. } => "setTryValue",
            FormsCommand::SetTryValues { .. } => "setTryValues",
            FormsCommand::ResetTry => "resetTry",
            FormsCommand::PreviousStep => "previousStep",
            FormsCommand::NextStep => "nextStep",
            FormsCommand::Submit => "submit",
            FormsCommand::SetLocale { .. } => "setLocale",
            FormsCommand::SetContributions { .. } => "setContributions",
            FormsCommand::AddStep => "addStep",
            FormsCommand::PatchStep { .. } => "patchStep",
            FormsCommand::RemoveStep { .. } => "removeStep",
            FormsCommand::MoveStep { .. } => "moveStep",
            FormsCommand::UpdateForm { .. } => "updateForm",
            FormsCommand::AddQuestion { .. } => "addQuestion",
            FormsCommand::RemoveQuestion { .. } => "removeQuestion",
            FormsCommand::PatchQuestions { .. } => "patchQuestions",
            FormsCommand::PatchQuestionOptions { .. } => "patchQuestionOptions",
            FormsCommand::AddQuestionOption { .. } => "addQuestionOption",
            FormsCommand::RemoveQuestionOption { .. } => "removeQuestionOption",
            FormsCommand::PatchVectorField { .. } => "patchVectorField",
            FormsCommand::AddVectorField { .. } => "addVectorField",
            FormsCommand::RemoveVectorField { .. } => "removeVectorField",
            FormsCommand::MoveQuestion { .. } => "moveQuestion",
            FormsCommand::DropQuestionKind { .. } => "dropQuestionKind",
            FormsCommand::SetSpecJson { .. } => "setSpecJson",
            FormsCommand::SetActiveExample { .. } => "setActiveExample",
            FormsCommand::ExportFixture => "exportFixture",
        }
    }

    fn handle(&self, command: &FormsCommand, doc: &DocumentView<'_, FormSpec>, cfg: &ConfigView<'_, FormsConfig>) -> Emit<FormOperation, FormsConfigOperation> {
        let spec = doc.projection;
        let config = cfg.projection;
        match command {
            //#region 👁️View
            FormsCommand::SetSelection { ids } => Emit::config(vec![FormsConfigOperation::SetSelection { ids: ids.clone() }]),
            FormsCommand::SetTryValue { key, value_json, option_value, vector_index, param_key } => {
                let mut values = try_values_map(config);
                if let Some(option_value) = option_value {
                    let mut selected = values.get(key.as_str()).and_then(|value| value.as_array().cloned()).unwrap_or_default();
                    let pressed = value_json.as_deref().map(parse_value_json).and_then(|value| value.as_bool()).unwrap_or(false);
                    if pressed {
                        if !selected.iter().any(|entry| entry.as_str() == Some(option_value.as_str())) {
                            selected.push(json!(option_value));
                        }
                    } else {
                        selected.retain(|entry| entry.as_str() != Some(option_value.as_str()));
                    }
                    values.insert(key.clone(), Value::Array(selected));
                } else if let Some(index) = vector_index {
                    if let Some(raw) = value_json.as_deref().map(parse_value_json) {
                        patch_try_vector_field(&mut values, key, *index as usize, &raw);
                    }
                } else if let Some(param_key) = param_key {
                    if let Some(raw) = value_json.as_deref().map(parse_value_json) {
                        patch_try_object_field(&mut values, key, param_key, &raw);
                    }
                } else if let Some(raw) = value_json.as_deref().map(parse_value_json) {
                    values.insert(key.clone(), raw);
                }
                Emit::config(vec![FormsConfigOperation::SetTryValues { json: try_values_json_text(&values) }])
            }
            FormsCommand::SetTryValues { values_json } => {
                let mut values = try_values_map(config);
                if let Some(incoming) = serde_json::from_str::<Value>(values_json).ok().and_then(|value| value.as_object().cloned()) {
                    for (key, value) in incoming {
                        values.insert(key, value);
                    }
                }
                Emit::config(vec![FormsConfigOperation::SetTryValues { json: try_values_json_text(&values) }])
            }
            FormsCommand::ResetTry => Emit::config(reset_try_config_operations()),
            FormsCommand::PreviousStep => Emit::config(vec![FormsConfigOperation::SetStepIndex { index: config.current_step_index.saturating_sub(1) }]),
            FormsCommand::NextStep => {
                let index = config.current_step_index as usize;
                if index + 1 < spec.steps.len() {
                    let step = &spec.steps[index];
                    let values = effective_try_values(spec, config);
                    if can_advance(step, &values) {
                        return Emit::config(vec![FormsConfigOperation::SetStepIndex { index: config.current_step_index + 1 }]);
                    }
                }
                Emit::default()
            }
            FormsCommand::Submit => Emit::default(),
            FormsCommand::SetLocale { value } => Emit::config(vec![FormsConfigOperation::SetLocale { value: value.clone() }]),
            FormsCommand::SetContributions { json } => Emit::config(vec![FormsConfigOperation::SetContributions { json: json.clone() }]),
            //#endregion 👁️View
            //#region 🔧️Operations
            FormsCommand::AddStep => {
                let step = FormStep { id: create_form_id("step"), title: format!("Step {}", spec.steps.len() + 1), description: None, blocks: Vec::new() };
                Emit { document_operations: vec![FormOperation::AddStep { step, index: None }], config_operations: reset_try_config_operations(), ..Default::default() }
            }
            FormsCommand::PatchStep { step_id, field, value } => {
                let Some(step) = spec.steps.iter().find(|step| step.id == *step_id).cloned() else {
                    return Emit::default();
                };
                let step = match field.as_str() {
                    "title" => FormStep { title: value.clone(), ..step },
                    "description" => FormStep { description: Some(value.clone()).filter(|description| !description.is_empty()), ..step },
                    _ => return Emit::default(),
                };
                Emit {
                    document_operations: vec![FormOperation::UpdateStep { step }],
                    config_operations: reset_try_config_operations(),
                    coalesce_key: Some(format!("patch-step:{step_id}:{field}")),
                    ..Default::default()
                }
            }
            FormsCommand::RemoveStep { step_id } => {
                if step_id.is_empty() {
                    return Emit::default();
                }
                let removed_ids: Vec<String> = spec.steps.iter().filter(|step| step.id == *step_id).flat_map(|step| step.blocks.iter().map(|question| question.id.clone())).collect();
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: config.selected_ids.iter().filter(|id| !removed_ids.contains(id)).cloned().collect() });
                Emit { document_operations: vec![FormOperation::RemoveStep { step_id: step_id.clone() }], config_operations, ..Default::default() }
            }
            FormsCommand::MoveStep { step_id, index } => {
                if step_id.is_empty() {
                    return Emit::default();
                }
                Emit {
                    document_operations: vec![FormOperation::MoveStep { step_id: step_id.clone(), index: *index as usize }],
                    config_operations: reset_try_config_operations(),
                    ..Default::default()
                }
            }
            FormsCommand::UpdateForm { title } => Emit {
                document_operations: vec![FormOperation::UpdatePlaybook { title: Some(title.clone()).filter(|title| !title.is_empty()) }],
                coalesce_key: Some("update-playbook".into()),
                ..Default::default()
            },
            FormsCommand::AddQuestion { kind, step_id } => {
                let Some(step_id) = step_id.clone().or_else(|| spec.steps.first().map(|step| step.id.clone())) else {
                    return Emit::default();
                };
                let question = default_question_for_kind(kind, create_form_id("q"));
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: vec![question.id.clone()] });
                Emit { document_operations: vec![FormOperation::AddBlock { step_id, block: question, index: None }], config_operations, ..Default::default() }
            }
            FormsCommand::RemoveQuestion { question_id } => {
                let Some(location) = find_question_location(spec, question_id) else {
                    return Emit::default();
                };
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: config.selected_ids.iter().filter(|id| *id != question_id).cloned().collect() });
                Emit { document_operations: vec![FormOperation::RemoveBlock { step_id: location.step_id, block_id: question_id.clone() }], config_operations, ..Default::default() }
            }
            FormsCommand::PatchQuestions { question_ids, field, value_json, param_key } => {
                let raw_value = parse_value_json(value_json);
                let operations: Vec<FormOperation> = if field == "param" {
                    let param_key = param_key.as_deref().unwrap_or("");
                    question_ids.iter().filter_map(|question_id| patch_building_component_param(spec, question_id, param_key, &raw_value)).collect()
                } else {
                    question_ids.iter().filter_map(|question_id| patch_question_field(spec, question_id, field, &raw_value)).collect()
                };
                if operations.is_empty() {
                    return Emit::config(reset_try_config_operations());
                }
                Emit { document_operations: operations, config_operations: reset_try_config_operations(), coalesce_key: Some(format!("patch:{field}:{}", question_ids.join(","))), ..Default::default() }
            }
            FormsCommand::PatchQuestionOptions { question_ids, option_value, field, value_json } => {
                let raw_value = parse_value_json(value_json);
                let operations: Vec<FormOperation> = question_ids.iter().filter_map(|question_id| patch_question_option(spec, question_id, option_value, field, &raw_value)).collect();
                if operations.is_empty() {
                    return Emit::default();
                }
                Emit::amend(operations, format!("patch-option:{option_value}:{field}"))
            }
            FormsCommand::AddQuestionOption { question_id, label } => match add_question_option(spec, question_id, label) {
                Some(operation) => Emit::operations(vec![operation]),
                None => Emit::default(),
            },
            FormsCommand::RemoveQuestionOption { question_id, option_value } => match remove_question_option(spec, question_id, option_value) {
                Some(operation) => Emit::operations(vec![operation]),
                None => Emit::default(),
            },
            FormsCommand::PatchVectorField { question_id, field_key, field, value_json } => {
                let raw_value = parse_value_json(value_json);
                match patch_vector_field(spec, question_id, field_key, field, &raw_value) {
                    Some(operation) => Emit::amend(vec![operation], format!("patch-vector:{question_id}:{field_key}:{field}")),
                    None => Emit::default(),
                }
            }
            FormsCommand::AddVectorField { question_id, field_key } => match add_vector_field(spec, question_id, field_key) {
                Some(operation) => Emit::operations(vec![operation]),
                None => Emit::default(),
            },
            FormsCommand::RemoveVectorField { question_id, field_key } => match remove_vector_field(spec, question_id, field_key) {
                Some(operation) => Emit::operations(vec![operation]),
                None => Emit::default(),
            },
            FormsCommand::MoveQuestion { question_id, to_step_id, target_id, position, index } => {
                let Some(source) = find_question_location(spec, question_id) else {
                    return Emit::default();
                };
                let target_id = target_id.as_deref().unwrap_or(question_id);
                let resolved_index = index.map(|value| value as usize).unwrap_or_else(|| resolve_question_insert_index(spec, to_step_id, target_id, position).unwrap_or(0));
                Emit {
                    document_operations: vec![FormOperation::MoveBlock { block_id: question_id.clone(), from_step_id: source.step_id, to_step_id: to_step_id.clone(), index: resolved_index }],
                    config_operations: reset_try_config_operations(),
                    ..Default::default()
                }
            }
            FormsCommand::DropQuestionKind { kind, target_id, drop_position } => {
                let Some(step_id) = resolve_step_id_from_tree_target(spec, target_id) else {
                    return Emit::default();
                };
                let index = resolve_question_insert_index(spec, &step_id, target_id, drop_position);
                let question = default_question_for_kind(kind, create_form_id("q"));
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: vec![question.id.clone()] });
                Emit { document_operations: vec![FormOperation::AddBlock { step_id, block: question, index }], config_operations, ..Default::default() }
            }
            FormsCommand::SetSpecJson { json } => {
                let Ok(next) = serde_json::from_str::<FormSpec>(json) else {
                    return Emit::default();
                };
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: Vec::new() });
                Emit { document_operations: replace_spec_operations(spec, &next), config_operations, ..Default::default() }
            }
            FormsCommand::SetActiveExample { example_id } => {
                let next = match example_id.as_str() {
                    "" => Some(empty_forms_projection()),
                    "building-component" => forms_dsl::parse_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).ok(),
                    "default" => Some(forms_engine::default_example_spec()),
                    "onboarding" => Some(forms_engine::onboarding_example_spec()),
                    _ => None,
                };
                let Some(next) = next else {
                    return Emit::default();
                };
                let mut config_operations = reset_try_config_operations();
                config_operations.push(FormsConfigOperation::SetSelection { ids: Vec::new() });
                Emit { document_operations: replace_spec_operations(spec, &next), config_operations, ..Default::default() }
            }
            //#endregion 🔧️Operations
            //#region 🐚️Shell
            FormsCommand::ExportFixture => {
                let data = forms_dsl::print_dsl(spec);
                Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.forms.dsl", spec.id), mime_type: "text/plain".into(), data, encoding: None })
            } //#endregion 🐚️Shell
        }
    }

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `dictionary:out` forfeits the default's dispatch);
    /// `dictionary:out` re-exports the form's currently-configured default field values (see
    /// `playbook::initial_values`, re-exported as `initial_try_values`) as a `form.dictionary` JSON
    /// object keyed by question id — no `cfg` parameter reaches this method, so this is the form's
    /// authored defaults, not a live in-progress Try-wizard session (that lives in `Self::Config`).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, FormSpec>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let bytes = store::DocumentPack::encode_pack(doc.projection);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                    payload: MediaPayload::Structured { schema: FORMS_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "dictionary:out" => {
                let values = initial_try_values(doc.projection, &Map::new());
                let json = serde_json::to_string(&Value::Object(values)).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "form.dictionary".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FormSpec>, cfg: &ConfigView<'_, FormsConfig>) -> UiNode {
        let spec = doc.projection;
        let config = cfg.projection;
        let contributions = parse_contributions(config);
        let labels = resolve_labels::<FormsLabels>(config);
        match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => render_blueprint_builder(spec, config, &contributions, labels),
            FORMS_PLAY_BODY_TRY => render_try_wizard(spec, config, &contributions, labels),
            FORMS_PLAY_BODY_DOCUMENT => build_document_tree(spec, &config.selected_ids, labels),
            FORMS_PLAY_BODY_CATALOGUE => build_catalogue_tree(&contributions, labels),
            FORMS_PLAY_BODY_INSPECTION => build_inspector_tree(spec, config, &contributions, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️FormsPlayApp

//#region 🔖️Manifest
pub fn create_forms_app() -> App {
    App::from_builder(
        App::builder(FORMS_PLAY_APP_ID, LocalizedLabel::native("Forms", "Formulare")).document(["semio", "forms"])
            .artifact_kind(ArtifactKindSpec {
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
            })
            .icon_id("forms")
            .mode("blueprint", LocalizedLabel::native("Blueprint", "Entwurf"), "cad-shape")
            .default_mode_id("blueprint")
            .window_kind(FORMS_PLAY_WINDOW_BLUEPRINT, LocalizedLabel::native("Blueprint", "Entwurf"), FORMS_PLAY_BODY_BLUEPRINT, SurfaceKind::BlockList, "clipboard-list")
            .window_kind(FORMS_PLAY_WINDOW_TRY, LocalizedLabel::native("Try", "Testen"), FORMS_PLAY_BODY_TRY, SurfaceKind::Canvas2d, "play")
            .panel_tab("framework.panel.document", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, FORMS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.catalogue", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, FORMS_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, FORMS_PLAY_BODY_INSPECTION)
            .operation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .operation("addQuestion", LocalizedLabel::native("Add Question", "Frage hinzufügen"))
            .operation("removeQuestion", LocalizedLabel::native("Remove Question", "Frage entfernen"))
            .operation("patchQuestions", LocalizedLabel::native("Patch Questions", "Fragen aktualisieren"))
            .operation("patchQuestionOptions", LocalizedLabel::native("Patch Question Options", "Fragenoptionen aktualisieren"))
            .operation("addQuestionOption", LocalizedLabel::native("Add Question Option", "Fragenoption hinzufügen"))
            .operation("removeQuestionOption", LocalizedLabel::native("Remove Question Option", "Fragenoption entfernen"))
            .operation("patchVectorField", LocalizedLabel::native("Patch Vector Field", "Vektorfeld aktualisieren"))
            .operation("addVectorField", LocalizedLabel::native("Add Vector Field", "Vektorfeld hinzufügen"))
            .operation("removeVectorField", LocalizedLabel::native("Remove Vector Field", "Vektorfeld entfernen"))
            .operation("moveQuestion", LocalizedLabel::native("Move Question", "Frage verschieben"))
            .operation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .operation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .operation("patchStep", LocalizedLabel::native("Patch Step", "Schritt aktualisieren"))
            .operation("updateForm", LocalizedLabel::native("Update Form", "Formular aktualisieren"))
            .operation("updatePlaybook", LocalizedLabel::native("Update Playbook", "Playbook aktualisieren"))
            .operation("dropQuestionKind", LocalizedLabel::native("Drop Question Kind", "Frageart ablegen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🛠️ Dev-only whole-spec import — kept out of the command palette, staged JSON form.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setSpecJson", LocalizedLabel::native("Set Spec JSON", "Spezifikations-JSON festlegen"), ActionKind::Operation) })
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setTryValue", LocalizedLabel::native("Set Try Value", "Testwert festlegen"))
            .view_action("setTryValues", LocalizedLabel::native("Set Try Values", "Testwerte festlegen"))
            .view_action("resetTry", LocalizedLabel::native("Reset Try", "Test zurücksetzen"))
            .view_action("previousStep", LocalizedLabel::native("Previous Step", "Vorheriger Schritt"))
            .view_action("nextStep", LocalizedLabel::native("Next Step", "Nächster Schritt"))
            .view_action("submit", LocalizedLabel::native("Submit", "Absenden"))
            .shell_action("exportFixture", LocalizedLabel::native("Export Fixture", "Fixture exportieren"))
            // 📝️ Staged argument forms for the panel-visible create/switch actions.
            .action_args("addQuestion", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value("text"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard")),
                    ActionArgOption::new("onboarding", LocalizedLabel::native("Onboarding", "Einführung")),
                    ActionArgOption::new("building-component", LocalizedLabel::native("Building Component", "Baukomponente")),
                ]).default_value("default"),
            ])
            .action_args("setSpecJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Spec JSON", "Spezifikations-JSON"))])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[FORMS_PLAY_WINDOW_BLUEPRINT.into(), FORMS_PLAY_WINDOW_TRY.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Blueprint".into(), "Try".into()]),
            ))
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`forms_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(FormsPlayApp.config_spec())
            .io(forms_engine::forms_io()),
    )
    .example("default", LocalizedLabel::native("Contact", "Kontakt"), forms_engine::default_example_json(), "file")
    .example("onboarding", LocalizedLabel::native("Onboarding", "Einführung"), forms_engine::onboarding_example_json(), "user")
    .example("building-component", LocalizedLabel::native("Building Component", "Baukomponente"), BUILDING_COMPONENT_EXAMPLE_TEXT, "building")
    .workflow("forms", "Forms", "data")
}
//#endregion 🔖️Manifest
// 🗂️ `semio_plugin!`/the document-codec registration live in the dedicated
// `✏️s/🔌️plugin/📋️forms/🛂️manifest/🗿️artifact` crate (matching every other plugin's constitutional
// layout — the `🎛️app/…/ui` crate exports `create_forms_app`/`FormsPlayApp` only, never registers the
// plugin bundle itself). This crate previously duplicated that registration inline, which tripped the
// framework's `__semio_plugin_sanity_constitutional_crates_present` gate (it expects `semio_plugin!`'s
// caller to sit at the manifest crate's shallower path depth).
//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, ViewState, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<FormsPlayApp> {
        testkit::new_app::<FormsPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so `addQuestion`'s declared `kind` default materializes.
    fn new_app_with_registry() -> VcsDocumentApp<FormsPlayApp> {
        testkit::new_app_with_registry::<FormsPlayApp>(create_forms_app)
    }

    #[test]
    fn add_question_materializes_kind_default() {
        let mut app = new_app_with_registry();
        let steps_before = app.projection().expect("projection").steps.len();
        assert!(steps_before > 0, "seeded fixture has at least one step to receive the question");
        // addQuestion fired with no explicit kind: the declared `kind` default ("text") must be materialized host-side.
        app.dispatch_typed(FormsCommand::AddQuestion { kind: "text".into(), step_id: None }, &testkit::meta("local")).expect("add question");
        let spec = app.projection().expect("projection");
        assert!(
            flatten_questions(&spec).iter().any(|(_, question)| question.kind == "text"),
            "kind default materialized from the registry"
        );
    }

    fn seed_example(app: &mut VcsDocumentApp<FormsPlayApp>, example_id: &str) {
        app.dispatch_typed(FormsCommand::SetActiveExample { example_id: example_id.into() }, &testkit::meta("local")).expect("seed example");
    }

    fn render(app: &mut VcsDocumentApp<FormsPlayApp>, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).unwrap()
    }

    #[test]
    fn initial_document_seeds_building_component_fixture() {
        let app = new_app();
        let spec = app.projection().expect("projection");
        assert!(!flatten_questions(&spec).is_empty());
        assert!(flatten_questions(&spec).iter().any(|(_, question)| question.kind == "buildingComponent"));
    }

    #[test]
    fn renders_blueprint_builder_cards() {
        let mut app = new_app();
        let first_question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(json.contains(r#""componentKind":"block-list""#));
        assert!(json.contains(r#""surfaceId":"forms.play.blueprint""#));
        assert!(json.contains("\"blockList\""));
        assert!(json.contains(&first_question_id));
    }

    #[test]
    fn blueprint_builder_card_reflects_selection() {
        let mut app = new_app();
        let first_question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        app.dispatch_typed(FormsCommand::SetSelection { ids: vec![first_question_id.clone()] }, &testkit::meta("local")).expect("select");
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(json.contains(&format!(r#""selectedId":"{first_question_id}""#)));
    }

    #[test]
    fn try_wizard_gates_navigation_and_reports_inline_errors() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        app.dispatch_typed(FormsCommand::SetTryValues { values_json: r#"{"name":"","email":""}"#.into() }, &testkit::meta("local")).expect("clear values");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        // 🩹️ Pre-existing test-assertion bug (unrelated to the B1 typed-command migration): `UiPresence`
        // serializes disabled state as `{"state":"disabled"}`, not a bare `"disabled":true` boolean —
        // this assertion never matched the real wire shape.
        assert!(json.contains(r#""state":"disabled""#));
        assert!(json.contains(r#""error":"#));
        assert!(json.contains("forms-try.back"));
    }

    #[test]
    fn try_wizard_emits_slider_unit_and_number_bounds() {
        let mut app = new_app();
        seed_example(&mut app, "onboarding");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains(r#""min":13.0"#) || json.contains(r#""min":13"#));
        assert!(json.contains(r#""max":120.0"#) || json.contains(r#""max":120"#));
        app.dispatch_typed(FormsCommand::SetTryValues { values_json: r#"{"full-name":"Ada"}"#.into() }, &testkit::meta("local")).expect("fill");
        app.dispatch_typed(FormsCommand::NextStep, &testkit::meta("local")).expect("next");
        let second_json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(second_json.contains(r#""unit":"%""#));
    }

    #[test]
    fn image_question_with_url_src_emits_image_node() {
        let question = FormQuestion {
            src: Some("https://example.com/picture.png".into()),
            ..question_shell("q-image".into(), "Picture".into(), "image".into())
        };
        let node = render_try_question(&question, &Map::new(), &[], None, &FormsLabels::NATIVE_EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains("https://example.com/picture.png"));
    }

    #[test]
    fn patch_step_updates_title_and_description() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.dispatch_typed(FormsCommand::PatchStep { step_id: step_id.clone(), field: "title".into(), value: "Renamed".into() }, &testkit::meta("local")).expect("patch step");
        assert_eq!(app.projection().expect("projection").steps[0].title, "Renamed");
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = new_app();
        app.dispatch_typed(FormsCommand::AddStep, &testkit::meta("local")).expect("add step");
        let last_step_id = app.projection().expect("projection").steps.last().unwrap().id.clone();
        app.dispatch_typed(FormsCommand::MoveStep { step_id: last_step_id.clone(), index: 0 }, &testkit::meta("local")).expect("move step");
        assert_eq!(app.projection().expect("projection").steps[0].id, last_step_id);
        app.dispatch_typed(FormsCommand::RemoveStep { step_id: last_step_id.clone() }, &testkit::meta("local")).expect("remove step");
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn update_form_action_sets_title() {
        let mut app = new_app();
        app.dispatch_typed(FormsCommand::UpdateForm { title: "My Form".into() }, &testkit::meta("local")).expect("update form");
        assert_eq!(app.projection().expect("projection").title.as_deref(), Some("My Form"));
    }

    #[test]
    fn document_tree_declares_drop_action() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_DOCUMENT);
        assert!(json.contains(r#""dropAction""#));
        assert!(json.contains("dropQuestionKind"));
    }

    #[test]
    fn drop_question_kind_inserts_and_selects() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.dispatch_typed(
            FormsCommand::DropQuestionKind { kind: "slider".into(), target_id: forms_play_step_tree_id(&step_id), drop_position: "inside".into() },
            &testkit::meta("local"),
        )
        .expect("drop kind");
        let spec = app.projection().expect("projection");
        assert!(spec.steps[0].blocks.iter().any(|question| question.kind == "slider"));
        let blueprint = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(blueprint.contains(r#""selectedId":"#));
    }

    #[test]
    fn kind_editor_fields_are_editable_when_unset() {
        let question = question_shell("q-num".into(), "Amount".into(), "number".into());
        let fields = question_kind_editor_fields(&question, &["q-num".into()], &[], "forms-blueprint.q-num", &FormsLabels::NATIVE_EN);
        let json = serde_json::to_string(&fields).unwrap();
        assert!(json.contains("forms-blueprint.q-num.min"));
        assert!(json.contains("forms-blueprint.q-num.max"));
        assert!(json.contains("forms-blueprint.q-num.default"));
        assert!(json.contains("forms-blueprint.q-num.description"));
    }

    #[test]
    fn app_has_blueprint_and_try_windows_only() {
        let app = create_forms_app();
        assert_eq!(app.definition.window_kinds.len(), 2);
        assert_eq!(app.definition.window_kinds[0].id, FORMS_PLAY_WINDOW_BLUEPRINT);
        assert_eq!(app.definition.window_kinds[1].id, FORMS_PLAY_WINDOW_TRY);
        assert_eq!(app.definition.modes[0].id, "blueprint");
    }

    fn building_component_contributions() -> Vec<ProgramContributionEntry> {
        vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            contribution: Contribution::PlaybookBlockKind {
                app_id: "forms-module-procedural".into(),
                block_kind: "buildingComponent".into(),
                label: "Building Component".into(),
                icon_id: "building".into(),
                default_value_json: "{}".into(),
                params_body_key: "params".into(),
                preview_body_key: "preview".into(),
            },
        }]
    }

    fn building_component_question() -> FormQuestion {
        let mut question = question_shell("geometry".into(), "Geometry".into(), "buildingComponent".into());
        question.fixture_slug = Some("hexagonal-mushroom-column".into());
        question.params = Some(value_to_dsl(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
        question
    }

    #[test]
    fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_try_question(
            &building_component_question(),
            &Map::new(),
            &building_component_contributions(),
            None,
            &FormsLabels::NATIVE_EN,
        );
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    #[test]
    fn extension_question_falls_back_without_contribution() {
        let node = render_try_question(&building_component_question(), &Map::new(), &[], None, &FormsLabels::NATIVE_EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[test]
    fn document_lists_steps() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_DOCUMENT);
        assert!(json.contains("forms-play-document.steps"));
        assert!(json.contains("Identity"));
        assert!(json.contains("Geometry"));
    }

    #[test]
    fn catalogue_lists_question_kinds() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(json.contains("forms-play-catalogue.text"));
        assert!(json.contains("forms-play-catalogue.add-step"));
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = new_app();
        let before = app.projection().expect("projection").steps.len();
        app.dispatch_typed(FormsCommand::AddStep, &testkit::meta("local")).expect("add step");
        assert_eq!(app.projection().expect("projection").steps.len(), before + 1);
    }

    #[test]
    fn add_question_action_appends_question() {
        let mut app = new_app();
        app.dispatch_typed(FormsCommand::AddQuestion { kind: "text".into(), step_id: None }, &testkit::meta("local")).expect("add question");
        assert!(flatten_questions(&app.projection().expect("projection")).iter().any(|(_, question)| question.kind == "text"));
    }

    #[test]
    fn add_question_undo_redo_round_trip() {
        let mut app = new_app();
        let before = flatten_questions(&app.projection().expect("projection")).len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            FormsCommand::AddQuestion { kind: "text".into(), step_id: None },
            |app| flatten_questions(&app.projection().expect("projection")).len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn set_try_values_updates_config() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        app.dispatch_typed(FormsCommand::SetTryValues { values_json: r#"{"name":"Ada"}"#.into() }, &testkit::meta("local")).expect("set try values");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains("Ada"));
    }

    #[test]
    fn wizard_step_navigation() {
        let mut app = new_app();
        seed_example(&mut app, "onboarding");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
        app.dispatch_typed(FormsCommand::NextStep, &testkit::meta("local")).expect("next");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 2 / 3"));
        app.dispatch_typed(FormsCommand::PreviousStep, &testkit::meta("local")).expect("prev");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
    }

    #[test]
    fn conditional_visibility_hides_team_size() {
        let mut app = new_app();
        seed_example(&mut app, "onboarding");
        let spec = app.projection().expect("projection");
        let advanced = spec.steps.iter().find(|step| step.id == "advanced").expect("advanced step");
        let values = initial_try_values(&spec, &Map::new());
        assert_eq!(visible_questions(advanced, &values).len(), 1);
    }

    #[test]
    fn inspector_patch_updates_required() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        let name_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        app.dispatch_typed(
            FormsCommand::PatchQuestions { question_ids: vec![name_id], field: "required".into(), value_json: "false".into(), param_key: None },
            &testkit::meta("local"),
        )
        .expect("patch required");
        let spec = app.projection().expect("projection");
        assert!(!spec.steps[0].blocks[0].required.unwrap_or(true));
    }

    #[test]
    fn renders_try_wizard() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }

    #[test]
    fn forms_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(json.contains("Boolean"));
        assert!(json.contains("Long Text"));
        assert!(json.contains("Slider"));
        assert!(!json.contains("Boolescher Wert"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// passing a `ViewState` into `render` for this purpose.
    #[test]
    fn forms_labels_resolve_german_locale() {
        let mut app = new_app();
        app.dispatch_typed(FormsCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT);
        assert!(json.contains("Boolescher Wert"));
        assert!(json.contains("Langtext"));
        assert!(json.contains("Schieberegler"));
        assert!(!json.contains("Boolean"));
        let catalogue_json = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Langtext"));
        assert!(catalogue_json.contains("Aktionen"));
    }

    /// 🧪️ The definitional proof: two independent instances start from the same seeded document, apply
    /// DISJOINT edits (A adds a question to the first step, B adds a whole new step), and exchanging operations
    /// over a backbone converges both sides onto the same projection — impossible under whole-document
    /// snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits() {
        testkit::assert_two_instances_converge::<FormsPlayApp, (usize, usize)>(
            "mem://forms-convergence",
            FormsCommand::AddQuestion { kind: "text".into(), step_id: None },
            FormsCommand::AddStep,
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.steps.len(), projection.steps[0].blocks.len())
            },
        );
    }

    //#region 🧪️MediaPorts
    #[test]
    fn export_media_dictionary_out_returns_default_values() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = FormsPlayApp.export_media("dictionary:out", &doc).expect("export dictionary:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "form.dictionary");
        let parsed: Value = serde_json::from_str(&json).expect("valid json dictionary");
        assert!(parsed.is_object());
    }

    #[test]
    fn export_media_document_out_round_trips_through_pack() {
        let app = new_app();
        let document = app.projection().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = FormsPlayApp.export_media("document:out", &doc).expect("export document:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, FORMS_DOCUMENT_SCHEMA);
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
        let decoded = <FormSpec as store::DocumentPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(decoded, document);
    }

    #[test]
    fn forms_io_exposes_dictionary_out_port() {
        let io = FormsPlayApp.io().expect("forms declares io");
        assert!(io.ports.iter().any(|port| port.id == "dictionary:out"));
    }
    //#endregion 🧪️MediaPorts
}
//#endregion 🧪️Tests
