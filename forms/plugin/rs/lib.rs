//! 📋 Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.

use forms::{
    can_advance, default_value_for_question, empty_forms_projection, initial_try_values, is_extension_question_kind,
    visible_questions, FormOp, FormQuestion, FormQuestionOption, FormSpec, FormStep, FormVectorField,
    FORM_BUILTIN_KINDS, FORMS_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind,
    create_default_layout, is_de_locale, localized_label_map, resolve_labels, selection_ids, tree_item_with_action,
    ui_external_slot, ui_image, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text,
    ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionArgDef, ActionArgOption,
    ActionDefinition, ActionKind, ActionEmit, App, AppLabelsOverlay, AppLabelsOverlayExt, BlockPaletteEntry, Contribution,
    DocumentApp, DocumentView, HostEffect, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, ResourceKindSpec, ActionDescriptor,
    UiButtonNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode, UiPresence,
    UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode,
    ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖Constants
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
const BUILDING_COMPONENT_EXAMPLE_JSON: &str = include_str!("../../example/building-component.forms.json");
const DEFAULT_EXAMPLE_JSON: &str = r##"{
  "schema": "forms.form",
  "id": "default",
  "version": "1",
  "title": "Contact",
  "steps": [
    {
      "id": "contact",
      "title": "Contact",
      "blocks": [
        { "id": "name", "kind": "text", "label": "Name", "required": true, "placeholder": "Your name" },
        { "id": "email", "kind": "text", "label": "Email", "required": true, "placeholder": "you@example.com" },
        { "id": "message", "kind": "longText", "label": "Message", "placeholder": "How can we help?" }
      ]
    }
  ]
}"##;
const ONBOARDING_EXAMPLE_JSON: &str = r##"{
  "schema": "forms.form",
  "id": "onboarding",
  "version": "1",
  "title": "Product Onboarding",
  "steps": [
    {
      "id": "profile",
      "title": "Profile",
      "description": "Tell us about yourself.",
      "blocks": [
        { "id": "full-name", "kind": "text", "label": "Full name", "required": true, "default": "Alex Example" },
        { "id": "bio", "kind": "longText", "label": "Bio", "placeholder": "Short introduction" },
        { "id": "age", "kind": "number", "label": "Age", "min": 13, "max": 120, "default": 28 },
        { "id": "avatar", "kind": "image", "label": "Avatar", "src": "" },
        { "id": "resume", "kind": "file", "label": "Resume", "accept": ".pdf,.doc,.docx" }
      ]
    },
    {
      "id": "preferences",
      "title": "Preferences",
      "description": "Customize your experience.",
      "blocks": [
        { "id": "theme-color", "kind": "color", "label": "Accent color", "default": "#336699" },
        { "id": "start-date", "kind": "date", "label": "Start date", "default": "2026-07-01" },
        { "id": "notifications", "kind": "boolean", "label": "Enable notifications", "default": true },
        { "id": "volume", "kind": "slider", "label": "Notification volume", "min": 0, "max": 100, "step": 5, "default": 60, "unit": "%" },
        {
          "id": "plan",
          "kind": "single",
          "label": "Plan",
          "required": true,
          "default": "pro",
          "options": [
            { "value": "free", "label": "Free" },
            { "value": "pro", "label": "Pro" },
            { "value": "team", "label": "Team" }
          ]
        },
        {
          "id": "features",
          "kind": "multi",
          "label": "Features",
          "default": ["analytics"],
          "options": [
            { "value": "analytics", "label": "Analytics" },
            { "value": "automation", "label": "Automation" },
            { "value": "collab", "label": "Collaboration" }
          ]
        },
        {
          "id": "offset",
          "kind": "vector",
          "label": "Workspace offset",
          "schema": "vec3",
          "step": 0.5,
          "fields": [
            { "key": "x", "label": "X", "value": 0 },
            { "key": "y", "label": "Y", "value": 0 },
            { "key": "z", "label": "Z", "value": 0 }
          ]
        },
        { "id": "welcome-note", "kind": "note", "label": "Welcome", "text": "Thanks for trying every question kind in one fixture." }
      ]
    },
    {
      "id": "advanced",
      "title": "Advanced",
      "blocks": [
        { "id": "show-team-size", "kind": "boolean", "label": "Specify team size", "default": false },
        {
          "id": "team-size",
          "kind": "slider",
          "label": "Team size",
          "min": 1,
          "max": 50,
          "step": 1,
          "default": 5,
          "condition": { "kind": "truthy", "expr": { "kind": "var", "name": "show-team-size" } }
        },
        {
          "id": "team-role",
          "kind": "single",
          "label": "Primary role",
          "options": [
            { "value": "design", "label": "Design" },
            { "value": "engineering", "label": "Engineering" },
            { "value": "product", "label": "Product" }
          ],
          "condition": { "kind": "truthy", "expr": { "kind": "var", "name": "show-team-size" } }
        }
      ]
    }
  ]
}"##;
const AVATAR_PLACEHOLDER_PNG_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

static FORM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Types
/// 👁️ Ephemeral per-session view state — never part of the persisted `FormSpec` document. Blueprint
/// selection, the Try wizard's active step, and its in-progress answer values all live here on the
/// app struct, out of the VCS document.
#[derive(Clone, Debug, Default)]
struct FormsPlayRuntime {
    selected_ids: Vec<String>,
    current_step_index: usize,
    try_values: HashMap<String, Value>,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
/// 🌱 The forms app's default document — the building-component fixture. Used as
/// `DocumentApp::initial_projection`.
fn building_component_spec() -> FormSpec {
    serde_json::from_str(BUILDING_COMPONENT_EXAMPLE_JSON).unwrap_or_else(|_| empty_forms_projection())
}

fn forms_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: FORMS_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn effective_try_values(spec: &FormSpec, runtime: &FormsPlayRuntime) -> Map<String, Value> {
    let overrides: Map<String, Value> = runtime.try_values.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
    initial_try_values(spec, &overrides)
}

fn reset_try_runtime(runtime: &mut FormsPlayRuntime) {
    runtime.try_values.clear();
    runtime.current_step_index = 0;
}

/// ✏️ Emits the ops that replace the current form spec's title + steps with those of `next` — a
/// legitimate whole-document swap for import/example-switch, expressed granularly through the
/// existing `FormOp` vocabulary (remove every current step, retitle, re-add the new steps) so it
/// still records a true inverse.
fn replace_spec_ops(current: &FormSpec, next: &FormSpec) -> Vec<FormOp> {
    let mut ops: Vec<FormOp> = current
        .steps
        .iter()
        .map(|step| FormOp::RemoveStep { step_id: step.id.clone() })
        .collect();
    if next.title != current.title {
        ops.push(FormOp::UpdateProtocol { title: next.title.clone() });
    }
    for step in &next.steps {
        ops.push(FormOp::AddStep { step: step.clone(), index: None });
    }
    ops
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

//#region 🔖Contributions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PluginContributionEntry {
    plugin_id: String,
    contribution: Contribution,
}

fn parse_contributions(view_state: &ViewState) -> Vec<PluginContributionEntry> {
    view_state
        .contributions_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Vec<PluginContributionEntry>>(json).ok())
        .unwrap_or_default()
}

fn find_question_kind_contribution<'a>(
    contributions: &'a [PluginContributionEntry],
    kind: &str,
) -> Option<(&'a str, &'a Contribution)> {
    contributions.iter().find_map(|entry| {
        if let Contribution::ProtocolBlockKind { block_kind, .. } = &entry.contribution {
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
        .or_else(|| question.params.clone())
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
    contributions: &[PluginContributionEntry],
    surface: &str,
    interactive: bool,
) -> UiNode {
    let Some((plugin_id, contribution)) = find_question_kind_contribution(contributions, &question.kind) else {
        return ui_text(format!("Extension unavailable: {}", question.kind));
    };
    let Contribution::ProtocolBlockKind {
        app_id,
        params_body_key,
        preview_body_key,
        ..
    } = contribution
    else {
        return ui_text(format!("Extension unavailable: {}", question.kind));
    };
    let params = extension_params_value(question, values);
    let payload = extension_render_payload(question, &params, surface, interactive);
    ui_stack_vertical(vec![
        ui_external_slot(plugin_id, app_id, params_body_key, &payload),
        ui_external_slot(plugin_id, app_id, preview_body_key, &payload),
    ])
}
//#endregion 🔖Contributions

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

fn patch_try_object_field(runtime: &mut FormsPlayRuntime, key: &str, field: &str, raw: &Value) {
    let mut object = runtime.try_values.get(key).cloned().unwrap_or_else(|| json!({}));
    if let Some(map) = object.as_object_mut() {
        map.insert(field.into(), raw.clone());
        runtime.try_values.insert(key.into(), object);
    }
}

fn patch_try_vector_field(runtime: &mut FormsPlayRuntime, key: &str, index: usize, raw: &Value) {
    let mut array = runtime
        .try_values
        .get(key)
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default();
    while array.len() <= index {
        array.push(json!(0.0));
    }
    array[index] = raw.clone();
    runtime.try_values.insert(key.into(), Value::Array(array));
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
            question.default = Some(json!(0));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "slider" => {
            let mut question = question_shell(id, "Slider".into(), "slider".into());
            question.default = Some(json!(50));
            question.min = Some(0.0);
            question.max = Some(100.0);
            question.step = Some(1.0);
            question
        }
        "boolean" => {
            let mut question = question_shell(id, "Boolean".into(), "boolean".into());
            question.default = Some(json!(false));
            question
        }
        "single" | "multi" => {
            let mut question = question_shell(
                id,
                if kind == "single" { "Single Select" } else { "Multi Select" }.into(),
                kind.into(),
            );
            question.default = if kind == "multi" { Some(json!([])) } else { None };
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
            question.default = Some(json!("2026-01-01"));
            question
        }
        "color" => {
            let mut question = question_shell(id, "Color".into(), "color".into());
            question.default = Some(json!("#336699"));
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
            question.params = Some(json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
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

/// ✏️ Locates `question_id` in `spec`, applies `mutate` to a clone, and returns the `UpdateBlock` op
/// that records the edit — the single seam every inspector patch flows through. Returns `None` if the
/// question no longer exists.
fn update_block_op(spec: &FormSpec, question_id: &str, mutate: impl FnOnce(&mut FormQuestion)) -> Option<FormOp> {
    let location = find_question_location(spec, question_id)?;
    let mut question = location.question;
    mutate(&mut question);
    Some(FormOp::UpdateBlock { step_id: location.step_id, block: question })
}

fn patch_question_field(spec: &FormSpec, question_id: &str, field: &str, raw_value: &Value) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| match field {
        "label" => question.label = raw_value.as_str().unwrap_or("").to_string(),
        "kind" => question.kind = raw_value.as_str().unwrap_or("text").to_string(),
        "description" => question.description = raw_value.as_str().map(str::to_string),
        "placeholder" => question.placeholder = raw_value.as_str().map(str::to_string),
        "required" => question.required = Some(raw_value.as_bool().unwrap_or(false)),
        "text" => question.text = raw_value.as_str().map(str::to_string),
        "default" => question.default = Some(raw_value.clone()),
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

fn patch_question_option(spec: &FormSpec, question_id: &str, option_value: &str, field: &str, raw_value: &Value) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        if let Some(option) = options.iter_mut().find(|entry| entry.value == option_value) {
            if field == "label" {
                option.label = raw_value.as_str().unwrap_or("").to_string();
            }
        }
        question.options = Some(options);
    })
}

fn add_question_option(spec: &FormSpec, question_id: &str, label: &str) -> Option<FormOp> {
    let value = create_form_id("opt");
    update_block_op(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.push(FormQuestionOption { value, label: label.into() });
        question.options = Some(options);
    })
}

fn remove_question_option(spec: &FormSpec, question_id: &str, option_value: &str) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.retain(|entry| entry.value != option_value);
        question.options = Some(options);
    })
}

fn patch_vector_field(spec: &FormSpec, question_id: &str, field_key: &str, field: &str, raw_value: &Value) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| {
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

fn add_vector_field(spec: &FormSpec, question_id: &str, key: &str) -> Option<FormOp> {
    let location = find_question_location(spec, question_id)?;
    if location.question.fields.iter().flatten().any(|entry| entry.key == key) {
        return None;
    }
    update_block_op(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.push(FormVectorField { key: key.into(), label: Some(key.into()), value: Some(0.0) });
        question.fields = Some(fields);
    })
}

fn remove_vector_field(spec: &FormSpec, question_id: &str, field_key: &str) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.retain(|entry| entry.key != field_key);
        question.fields = Some(fields);
    })
}

fn patch_building_component_param(spec: &FormSpec, question_id: &str, param_key: &str, raw_value: &Value) -> Option<FormOp> {
    update_block_op(spec, question_id, |question| {
        let mut params = question.params.take().unwrap_or_else(|| json!({}));
        if let Some(map) = params.as_object_mut() {
            map.insert(param_key.into(), raw_value.clone());
        }
        question.params = Some(params);
    })
}

fn catalogue_kinds(contributions: &[PluginContributionEntry], labels: &FormsLabels) -> Vec<(String, String, String)> {
    let mut kinds: Vec<(String, String, String)> = FORM_BUILTIN_KINDS
        .iter()
        .map(|kind| {
            let (label, icon): (&str, &str) = match *kind {
                "text" => (labels.kind_text, "type"),
                "longText" => (labels.kind_long_text, "align-left"),
                "number" => (labels.kind_number, "hash"),
                "slider" => (labels.kind_slider, "sliders-horizontal"),
                "boolean" => (labels.kind_boolean, "toggle-left"),
                "single" => (labels.kind_single, "circle-dot"),
                "multi" => (labels.kind_multi, "list-checks"),
                "date" => (labels.kind_date, "calendar"),
                "color" => (labels.kind_color, "palette"),
                "image" => (labels.kind_image, "image"),
                "file" => (labels.kind_file, "file"),
                "vector" => (labels.kind_vector, "move-3d"),
                "note" => (labels.kind_note, "sticky-note"),
                other => (other, "help-circle"),
            };
            (kind.to_string(), label.into(), icon.into())
        })
        .collect();
    for entry in contributions {
        if let Contribution::ProtocolBlockKind {
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
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the forms app; one field per label makes every locale combination compile-checked.
    struct FormsLabels {
        label: &'static str = en: "Label", de: "Bezeichnung";
        kind: &'static str = en: "Kind", de: "Art";
        id: &'static str = en: "Id", de: "Id";
        required: &'static str = en: "Required", de: "Erforderlich";
        description: &'static str = en: "Description", de: "Beschreibung";
        placeholder: &'static str = en: "Placeholder", de: "Platzhalter";
        default: &'static str = en: "Default", de: "Standard";
        min: &'static str = en: "Min", de: "Min";
        max: &'static str = en: "Max", de: "Max";
        step_field: &'static str = en: "Step", de: "Schrittweite";
        unit: &'static str = en: "Unit", de: "Einheit";
        schema: &'static str = en: "Schema", de: "Schema";
        text: &'static str = en: "Text", de: "Text";
        src: &'static str = en: "Src", de: "Quelle";
        accept: &'static str = en: "Accept", de: "Akzeptierte Dateien";
        yes: &'static str = en: "Yes", de: "Ja";
        no: &'static str = en: "No", de: "Nein";
        option: &'static str = en: "Option", de: "Option";
        remove: &'static str = en: "Remove", de: "Entfernen";
        add_option: &'static str = en: "Add Option", de: "Option hinzufügen";
        remove_option: &'static str = en: "Remove Option", de: "Option entfernen";
        add_vector_field: &'static str = en: "Add Vector Field", de: "Vektorfeld hinzufügen";
        vector_field_label_suffix: &'static str = en: "label", de: "Bezeichnung";
        vector_field_value_suffix: &'static str = en: "value", de: "Wert";
        add_step: &'static str = en: "Add Step", de: "Schritt hinzufügen";
        add_text_question: &'static str = en: "Add Text Question", de: "Textfrage hinzufügen";
        question: &'static str = en: "Question", de: "Frage";
        selected: &'static str = en: "selected", de: "ausgewählt";
        no_steps_in_form: &'static str = en: "No steps in this form.", de: "Keine Schritte in diesem Formular.";
        form_fallback_title: &'static str = en: "Form", de: "Formular";
        step_progress: &'static str = en: "Step", de: "Schritt";
        back: &'static str = en: "Back", de: "Zurück";
        next: &'static str = en: "Next", de: "Weiter";
        submit: &'static str = en: "Submit", de: "Absenden";
        fixture_slug: &'static str = en: "Fixture Slug", de: "Fixture-Slug";
        no_steps_tree_item: &'static str = en: "(no steps)", de: "(keine Schritte)";
        actions: &'static str = en: "Actions", de: "Aktionen";
        kind_text: &'static str = en: "Text", de: "Text";
        kind_long_text: &'static str = en: "Long Text", de: "Langtext";
        kind_number: &'static str = en: "Number", de: "Zahl";
        kind_slider: &'static str = en: "Slider", de: "Schieberegler";
        kind_boolean: &'static str = en: "Boolean", de: "Boolescher Wert";
        kind_single: &'static str = en: "Single Select", de: "Einzelauswahl";
        kind_multi: &'static str = en: "Multi Select", de: "Mehrfachauswahl";
        kind_date: &'static str = en: "Date", de: "Datum";
        kind_color: &'static str = en: "Color", de: "Farbe";
        kind_image: &'static str = en: "Image", de: "Bild";
        kind_file: &'static str = en: "File", de: "Datei";
        kind_vector: &'static str = en: "Vector", de: "Vektor";
        kind_note: &'static str = en: "Note", de: "Notiz";
        window_blueprint: &'static str = en: "Blueprint", de: "Entwurf";
        window_try: &'static str = en: "Try", de: "Testen";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in `create_forms_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn forms_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addStep", "Add Step", "Schritt hinzufügen"),
        ("addQuestion", "Add Question", "Frage hinzufügen"),
        ("removeQuestion", "Remove Question", "Frage entfernen"),
        ("patchQuestions", "Patch Questions", "Fragen aktualisieren"),
        ("patchQuestionOptions", "Patch Question Options", "Fragenoptionen aktualisieren"),
        ("addQuestionOption", "Add Question Option", "Fragenoption hinzufügen"),
        ("removeQuestionOption", "Remove Question Option", "Fragenoption entfernen"),
        ("patchVectorField", "Patch Vector Field", "Vektorfeld aktualisieren"),
        ("addVectorField", "Add Vector Field", "Vektorfeld hinzufügen"),
        ("removeVectorField", "Remove Vector Field", "Vektorfeld entfernen"),
        ("moveQuestion", "Move Question", "Frage verschieben"),
        ("moveStep", "Move Step", "Schritt verschieben"),
        ("removeStep", "Remove Step", "Schritt entfernen"),
        ("patchStep", "Patch Step", "Schritt aktualisieren"),
        ("updateForm", "Update Form", "Formular aktualisieren"),
        ("updateProtocol", "Update Protocol", "Protokoll aktualisieren"),
        ("dropQuestionKind", "Drop Question Kind", "Frageart ablegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setSpecJson", "Set Spec JSON", "Spezifikations-JSON festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setTryValue", "Set Try Value", "Testwert festlegen"),
        ("setTryValues", "Set Try Values", "Testwerte festlegen"),
        ("resetTry", "Reset Try", "Test zurücksetzen"),
        ("previousStep", "Previous Step", "Vorheriger Schritt"),
        ("nextStep", "Next Step", "Nächster Schritt"),
        ("submit", "Submit", "Absenden"),
        ("editEngagementInput", "Edit Engagement Input", "Bearbeitungseingabe"),
        ("tryEngagementInput", "Try Engagement Input", "Testeingabe"),
        ("exportFixture", "Export Fixture", "Fixture exportieren"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_forms_app`.
/// The forms manifest declares no utilities for the utility bar, so this always resolves empty; kept for parity with the
/// other play apps' terminology plumbing.
fn forms_utility_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[])
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
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
                    ..tree_item_with_action(
                        question.id.clone(),
                        question.label.clone(),
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
                ..tree_item_with_action(
                    forms_play_step_tree_id(&step.id),
                    step.title.clone(),
                    Some(format!("{} questions", step.blocks.len())),
                    forms_action("setSelection", Some(json!({ "ids": [] }))),
                )
            }
        })
        .collect();
    PanelTreeBuilder::new("forms-play-document")
        .section_or_placeholder(
            "forms-play-document.steps",
            Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            true,
            step_items,
            labels.no_steps_tree_item,
        )
        .selected(selected_ids.to_vec())
        .selection_change(forms_action("setSelection", None))
        .drop_action(forms_action("dropQuestionKind", None))
        .build()
}

fn build_catalogue_tree(contributions: &[PluginContributionEntry], labels: &FormsLabels) -> UiNode {
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds(contributions, labels)
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                icon_id: Some(icon),
                draggable: Some(true),
                drag_data: Some(drag_data),
                ..tree_item_with_action(
                    format!("forms-play-catalogue.{kind}"),
                    label,
                    Some(kind.clone()),
                    forms_action("addQuestion", Some(json!({ "kind": kind }))),
                )
            }
        })
        .collect();
    let action_items = vec![
        UiTreeItemNode {
            icon_id: Some("plus".into()),
            ..tree_item_with_action("forms-play-catalogue.add-step", labels.add_step, None, forms_action("addStep", None))
        },
        UiTreeItemNode {
            icon_id: Some("type".into()),
            ..tree_item_with_action(
                "forms-play-catalogue.add-question",
                labels.add_text_question,
                None,
                forms_action("addQuestion", Some(json!({ "kind": "text" }))),
            )
        },
    ];
    PanelTreeBuilder::new("forms-play-catalogue")
        .section("forms-play-catalogue.kinds", Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()), true, kind_items)
        .section("forms-play-catalogue.actions", Some(labels.actions.into()), true, action_items)
        .build()
}

fn inspector_patch(question_ids: &[String], field: &str) -> ActionDescriptor {
    forms_action("patchQuestions", Some(json!({ "questionIds": question_ids, "field": field })))
}

fn inspector_text_field(question_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(question_ids, field),
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

fn inspector_number_field(question_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
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
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
    })
}

fn question_kind_editor_fields(
    question: &FormQuestion,
    question_ids: &[String],
    contributions: &[PluginContributionEntry],
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
                &[question.default.as_ref().map(json_string_value).unwrap_or_default()],
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
                &[question.default.as_ref().map(json_f64_value).unwrap_or(0.0)],
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
            let pressed = question.default.as_ref().and_then(|default| default.as_bool()).unwrap_or(false);
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
                })),
            }));
        }
        "single" | "multi" => {
            if let Some(options) = &question.options {
                for option in options {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("option.{}", option.value)),
                        label: format!("{} {}", labels.option, option.value),
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
                        })),
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
            }));
        }
        "date" | "color" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("default"),
                labels.default,
                &[question.default.as_ref().map(json_string_value).unwrap_or_default()],
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
                        label: format!("{} {}", field.key, labels.vector_field_label_suffix),
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
                        })),
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.value", field.key)),
                        label: format!("{} {}", field.key, labels.vector_field_value_suffix),
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
                        })),
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("vector.{}.remove", field.key))),
                        icon_id: "trash-2".into(),
                        label: format!("{} {}", labels.remove, field.key),
                        action: forms_action(
                            "removeVectorField",
                            Some(json!({ "questionId": question.id, "fieldKey": field.key })),
                        ),
                        style: None,
                        presence: UiPresence::default(),
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
    runtime: &FormsPlayRuntime,
    contributions: &[PluginContributionEntry],
    term_labels: &FormsLabels,
) -> UiNode {
    let questions: Vec<FormQuestion> = runtime
        .selected_ids
        .iter()
        .filter_map(|id| find_question_location(spec, id).map(|location| location.question))
        .collect();
    if questions.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {FORMS_DOCUMENT_SCHEMA}")),
            ui_text(format!("Steps: {}", spec.steps.len())),
            ui_text(format!("Questions: {}", flatten_questions(spec).len())),
        ]);
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
            label,
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
                placeholder: kind_mixed.placeholder,
                items: kind_items,
                on_change: inspector_patch(&question_ids, "kind"),
                presence: UiPresence::default(),
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        }),
        ui_inspector_readonly_field(
            "forms-play-inspector.id",
            term_labels.id,
            if question_ids.len() == 1 {
                question_ids[0].clone()
            } else {
                format!("{} {}", question_ids.len(), term_labels.selected)
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
                    Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
                on_change: inspector_patch(&question_ids, "required"),
                presence: UiPresence::selected(required_mixed.uniform && required_mixed.pressed),
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
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
//#endregion 🔖Panels

//#region 🔖Render
//#region 🔖Builder
fn forms_protocol_builder_config() -> protocol::ProtocolBuilderConfig {
    protocol::ProtocolBuilderConfig {
        action_namespace: "forms-blueprint",
        controller_id: FORMS_PLAY_CONTROLLER_ID,
        labels: protocol::PROTOCOL_BUILDER_LABELS_EN,
    }
}

fn render_blueprint_builder(spec: &FormSpec, runtime: &FormsPlayRuntime, contributions: &[PluginContributionEntry], labels: &FormsLabels) -> UiNode {
    let palette: Vec<BlockPaletteEntry> = catalogue_kinds(contributions, labels)
        .into_iter()
        .map(|(kind, label, icon_id)| BlockPaletteEntry {
            block_kind: kind,
            label,
            icon_id,
        })
        .collect();
    let config = forms_protocol_builder_config();
    protocol::render_protocol_builder(
        FORMS_PLAY_SURFACE_BLUEPRINT,
        spec,
        &palette,
        runtime.selected_ids.first().map(String::as_str),
        &config,
    )
}
//#endregion 🔖Builder

//#region 🔖TryWizard
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
        Some(question.label.clone()),
    )
}

fn ui_text_emphasized(value: impl Into<String>) -> UiNode {
    UiNode::Text(UiTextNode {
        value: value.into(),
        emphasize: Some(true),
        data_attributes: None,
        presence: UiPresence::default(),
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
    })
}

fn try_field(question: &FormQuestion, error: Option<&str>, child: UiNode) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("forms-try.{}", question.id),
        label: question.label.clone(),
        description: question.description.clone(),
        required: question.required.filter(|required| *required),
        error: error.map(str::to_string),
        child: Box::new(child),
        presence: UiPresence::default(),
    })
}

fn render_try_question(
    question: &FormQuestion,
    values: &Map<String, Value>,
    contributions: &[PluginContributionEntry],
    error: Option<&str>,
    labels: &FormsLabels,
) -> UiNode {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| default_value_for_question(question));
    let key = question.id.clone();
    match question.kind.as_str() {
        "text" | "longText" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(&value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
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
                            label: option.label.clone(),
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
                                text: Some(option.label.clone()),
                                on_change: forms_action(
                                    "setTryValue",
                                    Some(json!({ "key": key, "optionValue": option.value })),
                                ),
                                presence: UiPresence::selected(selected.contains(&option.value)),
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
                        label: field.label.clone().unwrap_or_else(|| field.key.clone()),
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
                        })),
                    })
                })
                .collect();
            try_field(question, error, ui_stack_horizontal(steppers))
        }
        "note" => ui_text(question.text.clone().unwrap_or_else(|| question.label.clone())),
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
            }),
        ),
        kind if is_extension_question_kind(kind) => {
            render_extension_question(question, values, contributions, "try", true)
        }
        _ => ui_text(format!("Unsupported kind: {}", question.kind)),
    }
}

fn render_try_wizard(spec: &FormSpec, runtime: &FormsPlayRuntime, contributions: &[PluginContributionEntry], labels: &FormsLabels) -> UiNode {
    if spec.steps.is_empty() {
        return ui_text(labels.no_steps_in_form);
    }
    let step_index = runtime.current_step_index.min(spec.steps.len().saturating_sub(1));
    let step = &spec.steps[step_index];
    let values = effective_try_values(spec, runtime);
    let visible = visible_questions(step, &values);
    let errors = forms::step_errors(step, &values);
    let advance = can_advance(step, &values);
    let errors_by_question: HashMap<&str, &str> = errors
        .iter()
        .map(|error| (error.block_id.as_str(), error.message.as_str()))
        .collect();
    let mut children = vec![
        ui_text_emphasized(spec.title.clone().unwrap_or_else(|| labels.form_fallback_title.into())),
        ui_text(format!("{} {} / {}", labels.step_progress, step_index + 1, spec.steps.len())),
        ui_text_emphasized(step.title.clone()),
    ];
    if let Some(description) = &step.description {
        children.push(ui_text(description.clone()));
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
        }),
        if step_index + 1 < spec.steps.len() {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.next".into()),
                icon_id: "chevron-right".into(),
                label: labels.next.into(),
                action: forms_action("nextStep", None),
                style: None,
                presence: UiPresence::disabled_if(!advance),
            })
        } else {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.submit".into()),
                icon_id: "check".into(),
                label: labels.submit.into(),
                action: forms_action("submit", None),
                style: None,
                presence: UiPresence::disabled_if(!advance),
            })
        },
    ];
    children.push(ui_stack_horizontal(nav));
    ui_stack_vertical(children)
}
//#endregion 🔖TryWizard
//#endregion 🔖Render

//#region 🔖FormsPlayApp
#[derive(Default)]
struct FormsPlayApp {
    runtime: FormsPlayRuntime,
}

impl DocumentApp for FormsPlayApp {
    type Projection = FormSpec;
    type Op = FormOp;

    fn app_id(&self) -> &str {
        FORMS_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FORMS_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FormSpec {
        building_component_spec()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, FormSpec>,
        _view_state: &ViewState,
    ) -> ActionEmit<FormOp> {
        let spec = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no ops.
            "setSelection" => {
                self.runtime.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "editEngagementInput" | "tryEngagementInput" => ActionEmit::default(),
            "setTryValue" => {
                let key = args.and_then(|value| value.get("key")).and_then(|value| value.as_str());
                let raw_value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned();
                let option_value = args.and_then(|value| value.get("optionValue")).and_then(|value| value.as_str());
                let vector_index = args.and_then(|value| value.get("vectorIndex")).and_then(|value| value.as_u64()).map(|index| index as usize);
                let param_key = args.and_then(|value| value.get("paramKey")).and_then(|value| value.as_str());
                if let Some(key) = key {
                    if let Some(option_value) = option_value {
                        let mut selected = self
                            .runtime
                            .try_values
                            .get(key)
                            .and_then(|value| value.as_array().cloned())
                            .unwrap_or_default();
                        let pressed = raw_value.as_ref().and_then(|value| value.as_bool()).unwrap_or(false);
                        if pressed {
                            if !selected.iter().any(|entry| entry.as_str() == Some(option_value)) {
                                selected.push(json!(option_value));
                            }
                        } else {
                            selected.retain(|entry| entry.as_str() != Some(option_value));
                        }
                        self.runtime.try_values.insert(key.into(), Value::Array(selected));
                    } else if let Some(index) = vector_index {
                        if let Some(raw) = raw_value {
                            patch_try_vector_field(&mut self.runtime, key, index, &raw);
                        }
                    } else if let Some(param_key) = param_key {
                        if let Some(raw) = raw_value {
                            patch_try_object_field(&mut self.runtime, key, param_key, &raw);
                        }
                    } else if let Some(raw) = raw_value {
                        self.runtime.try_values.insert(key.into(), raw);
                    }
                }
                ActionEmit::default()
            }
            "setTryValues" => {
                if let Some(values) = args.and_then(|value| value.get("values")).and_then(|value| value.as_object()) {
                    for (key, value) in values {
                        self.runtime.try_values.insert(key.clone(), value.clone());
                    }
                }
                ActionEmit::default()
            }
            "resetTry" => {
                reset_try_runtime(&mut self.runtime);
                ActionEmit::default()
            }
            "previousStep" => {
                self.runtime.current_step_index = self.runtime.current_step_index.saturating_sub(1);
                ActionEmit::default()
            }
            "nextStep" => {
                if self.runtime.current_step_index + 1 < spec.steps.len() {
                    let step = &spec.steps[self.runtime.current_step_index];
                    let values = effective_try_values(spec, &self.runtime);
                    if can_advance(step, &values) {
                        self.runtime.current_step_index += 1;
                    }
                }
                ActionEmit::default()
            }
            "submit" => ActionEmit::default(),
            // ✏️ Operations — read the current spec, emit typed ops with a true inverse.
            "addStep" => {
                let step = FormStep {
                    id: create_form_id("step"),
                    title: format!("Step {}", spec.steps.len() + 1),
                    description: None,
                    blocks: Vec::new(),
                };
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::AddStep { step, index: None }])
            }
            "patchStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let Some(step) = spec.steps.iter().find(|step| step.id == step_id).cloned() else {
                    return ActionEmit::default();
                };
                let step = match field {
                    "title" => FormStep { title: raw_value.into(), ..step },
                    "description" => FormStep {
                        description: Some(raw_value.to_string()).filter(|description| !description.is_empty()),
                        ..step
                    },
                    _ => return ActionEmit::default(),
                };
                self.runtime.try_values.clear();
                ActionEmit::amend(vec![FormOp::UpdateStep { step }], format!("patch-step:{step_id}:{field}"))
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                let removed_ids: Vec<String> = spec
                    .steps
                    .iter()
                    .filter(|step| step.id == step_id)
                    .flat_map(|step| step.blocks.iter().map(|question| question.id.clone()))
                    .collect();
                self.runtime.selected_ids.retain(|id| !removed_ids.contains(id));
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::RemoveStep { step_id: step_id.into() }])
            }
            "moveStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if step_id.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::MoveStep { step_id: step_id.into(), index }])
            }
            "updateForm" | "updateProtocol" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                ActionEmit::amend(
                    vec![FormOp::UpdateProtocol { title: Some(title.to_string()).filter(|title| !title.is_empty()) }],
                    "update-protocol",
                )
            }
            "addQuestion" | "addBlock" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("text");
                let step_id = args
                    .and_then(|value| value.get("stepId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| spec.steps.first().map(|step| step.id.clone()));
                let Some(step_id) = step_id else {
                    return ActionEmit::default();
                };
                let question = default_question_for_kind(kind, create_form_id("q"));
                self.runtime.selected_ids = vec![question.id.clone()];
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::AddBlock { step_id, block: question, index: None }])
            }
            "removeQuestion" | "removeBlock" => {
                let question_id = args
                    .and_then(|value| value.get("blockId").or_else(|| value.get("questionId")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let Some(location) = find_question_location(spec, question_id) else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.retain(|id| id != question_id);
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::RemoveBlock {
                    step_id: location.step_id,
                    block_id: question_id.into(),
                }])
            }
            "patchQuestions" => {
                let question_ids: Vec<String> = args
                    .and_then(|value| value.get("questionIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if question_ids.is_empty() || field.is_empty() {
                    return ActionEmit::default();
                }
                let ops: Vec<FormOp> = if field == "param" {
                    let param_key = args
                        .and_then(|value| value.get("paramKey"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("");
                    question_ids
                        .iter()
                        .filter_map(|question_id| patch_building_component_param(spec, question_id, param_key, &raw_value))
                        .collect()
                } else {
                    question_ids
                        .iter()
                        .filter_map(|question_id| patch_question_field(spec, question_id, field, &raw_value))
                        .collect()
                };
                self.runtime.try_values.clear();
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(ops, format!("patch:{field}:{}", question_ids.join(",")))
            }
            "patchQuestionOptions" => {
                let question_ids: Vec<String> = args
                    .and_then(|value| value.get("questionIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let option_value = args
                    .and_then(|value| value.get("optionValue"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                let ops: Vec<FormOp> = question_ids
                    .iter()
                    .filter_map(|question_id| patch_question_option(spec, question_id, option_value, field, &raw_value))
                    .collect();
                self.runtime.try_values.clear();
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(ops, format!("patch-option:{option_value}:{field}"))
            }
            "addQuestionOption" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args
                    .and_then(|value| value.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("New option");
                match add_question_option(spec, question_id, label) {
                    Some(op) => ActionEmit::ops(vec![op]),
                    None => ActionEmit::default(),
                }
            }
            "removeQuestionOption" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let option_value = args
                    .and_then(|value| value.get("optionValue"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                match remove_question_option(spec, question_id, option_value) {
                    Some(op) => ActionEmit::ops(vec![op]),
                    None => ActionEmit::default(),
                }
            }
            "patchVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args.and_then(|value| value.get("fieldKey")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                match patch_vector_field(spec, question_id, field_key, field, &raw_value) {
                    Some(op) => ActionEmit::amend(vec![op], format!("patch-vector:{question_id}:{field_key}:{field}")),
                    None => ActionEmit::default(),
                }
            }
            "addVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args
                    .and_then(|value| value.get("fieldKey"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("field");
                match add_vector_field(spec, question_id, field_key) {
                    Some(op) => ActionEmit::ops(vec![op]),
                    None => ActionEmit::default(),
                }
            }
            "removeVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args.and_then(|value| value.get("fieldKey")).and_then(|value| value.as_str()).unwrap_or("");
                match remove_vector_field(spec, question_id, field_key) {
                    Some(op) => ActionEmit::ops(vec![op]),
                    None => ActionEmit::default(),
                }
            }
            "moveQuestion" | "moveBlock" => {
                let question_id = args
                    .and_then(|value| value.get("blockId").or_else(|| value.get("questionId")))
                    .and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let target_id = args.and_then(|value| value.get("targetId")).and_then(|value| value.as_str());
                let position = args
                    .and_then(|value| value.get("position"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let explicit_index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).map(|index| index as usize);
                let (Some(question_id), Some(to_step_id)) = (question_id, to_step_id) else {
                    return ActionEmit::default();
                };
                let Some(source) = find_question_location(spec, question_id) else {
                    return ActionEmit::default();
                };
                let target_id = target_id.unwrap_or(question_id);
                let index = explicit_index
                    .unwrap_or_else(|| resolve_question_insert_index(spec, to_step_id, target_id, position).unwrap_or(0));
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::MoveBlock {
                    block_id: question_id.into(),
                    from_step_id: source.step_id,
                    to_step_id: to_step_id.into(),
                    index,
                }])
            }
            "dropQuestionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                let target_id = args.and_then(|value| value.get("targetId")).and_then(|value| value.as_str());
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let (Some(kind), Some(target_id)) = (kind, target_id) else {
                    return ActionEmit::default();
                };
                let Some(step_id) = resolve_step_id_from_tree_target(spec, target_id) else {
                    return ActionEmit::default();
                };
                let index = resolve_question_insert_index(spec, &step_id, target_id, drop_position);
                let question = default_question_for_kind(kind, create_form_id("q"));
                self.runtime.selected_ids = vec![question.id.clone()];
                self.runtime.try_values.clear();
                ActionEmit::ops(vec![FormOp::AddBlock { step_id, block: question, index }])
            }
            "setSpecJson" => {
                let Some(next) = args
                    .and_then(|value| value.get("json"))
                    .and_then(|value| value.as_str())
                    .and_then(|json_text| serde_json::from_str::<FormSpec>(json_text).ok())
                else {
                    return ActionEmit::default();
                };
                reset_try_runtime(&mut self.runtime);
                self.runtime.selected_ids.clear();
                ActionEmit::ops(replace_spec_ops(spec, &next))
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let json_text = match example_id {
                    "" => return ActionEmit::ops(replace_spec_ops(spec, &empty_forms_projection())),
                    "building-component" => BUILDING_COMPONENT_EXAMPLE_JSON,
                    "default" => DEFAULT_EXAMPLE_JSON,
                    "onboarding" => ONBOARDING_EXAMPLE_JSON,
                    _ => return ActionEmit::default(),
                };
                let Ok(next) = serde_json::from_str::<FormSpec>(json_text) else {
                    return ActionEmit::default();
                };
                reset_try_runtime(&mut self.runtime);
                self.runtime.selected_ids.clear();
                ActionEmit::ops(replace_spec_ops(spec, &next))
            }
            // 🐚 Shell action — download the current form spec as JSON.
            "exportFixture" => {
                let json = serde_json::to_string_pretty(spec).unwrap_or_default();
                ActionEmit::effect(HostEffect::DownloadMediaExport {
                    filename: format!("{}.forms.json", spec.id),
                    mime_type: "application/json".into(),
                    data: json,
                    encoding: None,
                })
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FormSpec>, view_state: &ViewState) -> UiNode {
        let spec = doc.projection;
        let contributions = parse_contributions(view_state);
        let labels = resolve_labels::<FormsLabels>(view_state);
        match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => render_blueprint_builder(spec, &self.runtime, &contributions, labels),
            FORMS_PLAY_BODY_TRY => render_try_wizard(spec, &self.runtime, &contributions, labels),
            FORMS_PLAY_BODY_DOCUMENT => build_document_tree(spec, &self.runtime.selected_ids, labels),
            FORMS_PLAY_BODY_CATALOGUE => build_catalogue_tree(&contributions, labels),
            FORMS_PLAY_BODY_INSPECTION => build_inspector_tree(spec, &self.runtime, &contributions, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<FormsLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::with_framework_panel_tabs(
            ["framework.panel.document", "framework.panel.catalogue", "framework.panel.inspection"],
            is_de,
        )
        .window_kind_label(FORMS_PLAY_WINDOW_BLUEPRINT, labels.window_blueprint)
        .window_kind_label(FORMS_PLAY_WINDOW_TRY, labels.window_try)
        .mode_label("blueprint", labels.window_blueprint)
        .action_labels(forms_action_labels(is_de))
        .utility_labels(forms_utility_labels(is_de))
        .example_labels(HashMap::from([
            ("default".to_string(), (if is_de { "Kontakt" } else { "Contact" }).to_string()),
            ("onboarding".to_string(), "Onboarding".to_string()),
            ("building-component".to_string(), (if is_de { "Baukomponente" } else { "Building Component" }).to_string()),
        ]))
    }
}
//#endregion 🔖FormsPlayApp

//#region 🔖Manifest
fn create_forms_app() -> App {
    App::from_builder(
        App::builder(FORMS_PLAY_APP_ID, "Forms").document(["semio", "forms"])
            .resource_kind(ResourceKindSpec {
                id: "form.dictionary".into(),
                name: "Form Dictionary".into(),
                source_format: "forms.dictionary".into(),
                component_kind: "forms".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::Brep,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                schema: "forms.dictionary".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("forms")
            .mode("blueprint", "Blueprint")
            .default_mode_id("blueprint")
            .window_kind(FORMS_PLAY_WINDOW_BLUEPRINT, "Blueprint", FORMS_PLAY_BODY_BLUEPRINT, SurfaceKind::BlockList)
            .window_kind(FORMS_PLAY_WINDOW_TRY, "Try", FORMS_PLAY_BODY_TRY, SurfaceKind::Canvas2d)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, FORMS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.catalogue", "Catalogue", PanelGroup::Workbench, FORMS_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, FORMS_PLAY_BODY_INSPECTION)
            .operation("addStep", "Add Step")
            .operation("addQuestion", "Add Question")
            .operation("removeQuestion", "Remove Question")
            .operation("patchQuestions", "Patch Questions")
            .operation("patchQuestionOptions", "Patch Question Options")
            .operation("addQuestionOption", "Add Question Option")
            .operation("removeQuestionOption", "Remove Question Option")
            .operation("patchVectorField", "Patch Vector Field")
            .operation("addVectorField", "Add Vector Field")
            .operation("removeVectorField", "Remove Vector Field")
            .operation("moveQuestion", "Move Question")
            .operation("moveStep", "Move Step")
            .operation("removeStep", "Remove Step")
            .operation("patchStep", "Patch Step")
            .operation("updateForm", "Update Form")
            .operation("updateProtocol", "Update Protocol")
            .operation("dropQuestionKind", "Drop Question Kind")
            .operation("setActiveExample", "Set Active Example")
            // 🛠️ Dev-only whole-spec import — kept out of the command palette, staged JSON form.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSpecJson", "Set Spec JSON", ActionKind::Operation) })
            .view_action("setSelection", "Set Selection")
            .view_action("setTryValue", "Set Try Value")
            .view_action("setTryValues", "Set Try Values")
            .view_action("resetTry", "Reset Try")
            .view_action("previousStep", "Previous Step")
            .view_action("nextStep", "Next Step")
            .view_action("submit", "Submit")
            .view_action("editEngagementInput", "Edit Engagement Input")
            .view_action("tryEngagementInput", "Try Engagement Input")
            .shell_action("exportFixture", "Export Fixture")
            // 📝 Staged argument forms for the panel-visible create/switch actions.
            .action_args("addQuestion", vec![
                ActionArgDef::select(
                    "kind",
                    "Kind",
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, *kind)).collect(),
                )
                .default_value("text"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("default", "Default"),
                    ActionArgOption::new("onboarding", "Onboarding"),
                    ActionArgOption::new("building-component", "Building Component"),
                ]).default_value("default"),
            ])
            .action_args("setSpecJson", vec![ActionArgDef::text("json", "Spec JSON")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[FORMS_PLAY_WINDOW_BLUEPRINT.into(), FORMS_PLAY_WINDOW_TRY.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Blueprint".into(), "Try".into()]),
            )),
    )
    .example("default", "Contact", DEFAULT_EXAMPLE_JSON)
    .example("onboarding", "Onboarding", ONBOARDING_EXAMPLE_JSON)
    .example("building-component", "Building Component", BUILDING_COMPONENT_EXAMPLE_JSON)
    .program("forms", "Forms", "data")
}

fn register_forms_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "forms",
    label: "Forms",
    version: "0.1.0",
    setup: register_forms_exports,
    apps: [ create_forms_app => FormsPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::apply_form_edit_op;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<FormsPlayApp> {
        testkit::new_app::<FormsPlayApp>()
    }

    /// 🧬 A wrapper carrying the real action registry so `addQuestion`'s declared `kind` default materializes.
    fn new_app_with_registry() -> VcsDocumentApp<FormsPlayApp> {
        testkit::new_app_with_registry::<FormsPlayApp>(create_forms_app)
    }

    #[test]
    fn add_question_materializes_kind_default() {
        let mut app = new_app_with_registry();
        let steps_before = app.projection().expect("projection").steps.len();
        assert!(steps_before > 0, "seeded fixture has at least one step to receive the question");
        // addQuestion fired with no args: the declared `kind` default ("text") must be materialized host-side.
        app.handle_action("addQuestion", None, &ViewState::default(), &testkit::meta("local")).expect("add question");
        let spec = app.projection().expect("projection");
        assert!(
            flatten_questions(&spec).iter().any(|(_, question)| question.kind == "text"),
            "kind default materialized from the registry"
        );
    }

    fn seed_example(app: &mut VcsDocumentApp<FormsPlayApp>, example_id: &str) {
        app.handle_action(
            "setActiveExample",
            Some(&json!({ "exampleId": example_id })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("seed example");
    }

    fn render(app: &mut VcsDocumentApp<FormsPlayApp>, body_key: &str, view_state: &ViewState) -> String {
        serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
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
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT, &ViewState::default());
        assert!(json.contains(r#""componentKind":"block-list""#));
        assert!(json.contains(r#""surfaceId":"forms.play.blueprint""#));
        assert!(json.contains("\"blockList\""));
        assert!(json.contains(&first_question_id));
    }

    #[test]
    fn blueprint_builder_card_reflects_selection() {
        let mut app = new_app();
        let first_question_id = app.projection().expect("projection").steps[0].blocks[0].id.clone();
        app.handle_action(
            "setSelection",
            Some(&json!({ "ids": [first_question_id.clone()] })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("select");
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT, &ViewState::default());
        assert!(json.contains(&format!(r#""selectedId":"{first_question_id}""#)));
    }

    #[test]
    fn try_wizard_gates_navigation_and_reports_inline_errors() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        app.handle_action(
            "setTryValues",
            Some(&json!({ "values": { "name": "", "email": "" } })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("clear values");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default());
        assert!(json.contains(r#""disabled":true"#));
        assert!(json.contains(r#""error":"#));
        assert!(json.contains("forms-try.back"));
    }

    #[test]
    fn try_wizard_emits_slider_unit_and_number_bounds() {
        let mut app = new_app();
        seed_example(&mut app, "onboarding");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default());
        assert!(json.contains(r#""min":13.0"#) || json.contains(r#""min":13"#));
        assert!(json.contains(r#""max":120.0"#) || json.contains(r#""max":120"#));
        app.handle_action(
            "setTryValues",
            Some(&json!({ "values": { "full-name": "Ada" } })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("fill");
        app.handle_action("nextStep", None, &ViewState::default(), &testkit::meta("local")).expect("next");
        let second_json = render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default());
        assert!(second_json.contains(r#""unit":"%""#));
    }

    #[test]
    fn image_question_with_url_src_emits_image_node() {
        let question = FormQuestion {
            src: Some("https://example.com/picture.png".into()),
            ..question_shell("q-image".into(), "Picture".into(), "image".into())
        };
        let node = render_try_question(&question, &Map::new(), &[], None, &FormsLabels::EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains("https://example.com/picture.png"));
    }

    #[test]
    fn patch_step_updates_title_and_description() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.handle_action(
            "patchStep",
            Some(&json!({ "stepId": step_id, "field": "title", "value": "Renamed" })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("patch step");
        assert_eq!(app.projection().expect("projection").steps[0].title, "Renamed");
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = new_app();
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        let last_step_id = app.projection().expect("projection").steps.last().unwrap().id.clone();
        app.handle_action(
            "moveStep",
            Some(&json!({ "stepId": last_step_id, "index": 0 })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("move step");
        assert_eq!(app.projection().expect("projection").steps[0].id, last_step_id);
        app.handle_action(
            "removeStep",
            Some(&json!({ "stepId": last_step_id })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("remove step");
        assert!(app.projection().expect("projection").steps.iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn update_form_action_sets_title() {
        let mut app = new_app();
        app.handle_action(
            "updateForm",
            Some(&json!({ "field": "title", "value": "My Form" })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("update form");
        assert_eq!(app.projection().expect("projection").title.as_deref(), Some("My Form"));
    }

    #[test]
    fn document_tree_declares_drop_action() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(json.contains(r#""dropAction""#));
        assert!(json.contains("dropQuestionKind"));
    }

    #[test]
    fn drop_question_kind_inserts_and_selects() {
        let mut app = new_app();
        let step_id = app.projection().expect("projection").steps[0].id.clone();
        app.handle_action(
            "dropQuestionKind",
            Some(&json!({ "kind": "slider", "targetId": forms_play_step_tree_id(&step_id), "dropPosition": "inside" })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("drop kind");
        let spec = app.projection().expect("projection");
        assert!(spec.steps[0].blocks.iter().any(|question| question.kind == "slider"));
        let blueprint = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT, &ViewState::default());
        assert!(blueprint.contains(r#""selectedId":"#));
    }

    #[test]
    fn kind_editor_fields_are_editable_when_unset() {
        let question = question_shell("q-num".into(), "Amount".into(), "number".into());
        let fields = question_kind_editor_fields(&question, &["q-num".into()], &[], "forms-blueprint.q-num", &FormsLabels::EN);
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

    fn building_component_contributions() -> Vec<PluginContributionEntry> {
        vec![PluginContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            contribution: Contribution::ProtocolBlockKind {
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
        question.params = Some(json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
        question
    }

    #[test]
    fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_try_question(
            &building_component_question(),
            &Map::new(),
            &building_component_contributions(),
            None,
            &FormsLabels::EN,
        );
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    #[test]
    fn extension_question_falls_back_without_contribution() {
        let node = render_try_question(&building_component_question(), &Map::new(), &[], None, &FormsLabels::EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[test]
    fn document_lists_steps() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(json.contains("forms-play-document.steps"));
        assert!(json.contains("Identity"));
        assert!(json.contains("Geometry"));
    }

    #[test]
    fn catalogue_lists_question_kinds() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_CATALOGUE, &ViewState::default());
        assert!(json.contains("forms-play-catalogue.text"));
        assert!(json.contains("forms-play-catalogue.add-step"));
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = new_app();
        let before = app.projection().expect("projection").steps.len();
        app.handle_action("addStep", None, &ViewState::default(), &testkit::meta("local")).expect("add step");
        assert_eq!(app.projection().expect("projection").steps.len(), before + 1);
    }

    #[test]
    fn add_question_action_appends_question() {
        let mut app = new_app();
        app.handle_action("addQuestion", Some(&json!({ "kind": "text" })), &ViewState::default(), &testkit::meta("local")).expect("add question");
        assert!(flatten_questions(&app.projection().expect("projection")).iter().any(|(_, question)| question.kind == "text"));
    }

    #[test]
    fn add_question_undo_redo_round_trip() {
        let mut app = new_app();
        let before = flatten_questions(&app.projection().expect("projection")).len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "addQuestion",
            Some(&json!({ "kind": "text" })),
            |app| flatten_questions(&app.projection().expect("projection")).len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn set_try_values_updates_runtime() {
        let mut app = new_app();
        seed_example(&mut app, "default");
        app.handle_action(
            "setTryValues",
            Some(&json!({ "values": { "name": "Ada" } })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("set try values");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default());
        assert!(json.contains("Ada"));
    }

    #[test]
    fn apply_form_edit_op_roundtrip() {
        let spec = empty_forms_projection();
        let step = FormStep {
            id: "step-test".into(),
            title: "Review".into(),
            description: None,
            blocks: Vec::new(),
        };
        let next = apply_form_edit_op(&spec, &FormOp::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }

    #[test]
    fn wizard_step_navigation() {
        let mut app = new_app();
        seed_example(&mut app, "onboarding");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default()).contains("Step 1 / 3"));
        app.handle_action("nextStep", None, &ViewState::default(), &testkit::meta("local")).expect("next");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default()).contains("Step 2 / 3"));
        app.handle_action("previousStep", None, &ViewState::default(), &testkit::meta("local")).expect("prev");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default()).contains("Step 1 / 3"));
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
        app.handle_action(
            "patchQuestions",
            Some(&json!({ "questionIds": [name_id], "field": "required", "pressed": false })),
            &ViewState::default(),
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
        let json = render(&mut app, FORMS_PLAY_BODY_TRY, &ViewState::default());
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }

    #[test]
    fn forms_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT, &ViewState::default());
        assert!(json.contains("Boolean"));
        assert!(json.contains("Long Text"));
        assert!(json.contains("Slider"));
        assert!(!json.contains("Boolescher Wert"));
    }

    #[test]
    fn forms_labels_resolve_german_locale() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let json = render(&mut app, FORMS_PLAY_BODY_BLUEPRINT, &view_state);
        assert!(json.contains("Boolescher Wert"));
        assert!(json.contains("Langtext"));
        assert!(json.contains("Schieberegler"));
        assert!(!json.contains("Boolean"));
        let catalogue_json = render(&mut app, FORMS_PLAY_BODY_CATALOGUE, &view_state);
        assert!(catalogue_json.contains("Langtext"));
        assert!(catalogue_json.contains("Aktionen"));
    }

    /// 🧪 The definitional proof: two independent instances start from the same seeded document, apply
    /// DISJOINT edits (A adds a question to the first step, B adds a whole new step), and exchanging ops
    /// over a backbone converges both sides onto the same projection — impossible under whole-document
    /// snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits() {
        testkit::assert_two_instances_converge::<FormsPlayApp, (usize, usize)>(
            "mem://forms-convergence",
            ("addQuestion", Some(&json!({ "kind": "text" }))),
            ("addStep", None),
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.steps.len(), projection.steps[0].blocks.len())
            },
        );
    }
}
//#endregion 🧪Tests
