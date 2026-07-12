//! 📋 Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.

use forms::{
    can_advance, default_value_for_question, empty_forms_projection, flatten_form_questions,
    initial_try_values, is_extension_question_kind, visible_questions, FormOp, FormQuestion,
    FormQuestionOption, FormSpec, FormStep, FormVectorField, FormsEnvelope, FormsStore,
    FORM_BUILTIN_KINDS, FORMS_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{SurfaceKind,
    create_default_layout,
    ui_external_slot, ui_image, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text,
    ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, Contribution,
    PanelGroup, ActionDescriptor, PluginApp, PluginBundle, UiButtonNode,
    UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode,
    UiSectionNode, UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const FORMS_PLAY_APP_ID: &str = "forms-play";
const FORMS_PLAY_CONTROLLER_ID: &str = "forms-play";
const FORMS_PLAY_SURFACE_BLUEPRINT: &str = "forms.play.blueprint";
const FORMS_PLAY_SURFACE_TRY: &str = "forms.play.try";
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
      "questions": [
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
      "questions": [
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
      "questions": [
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
      "questions": [
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

//#region 🔖Envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormsPlayEnvelope {
    envelope: FormsEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    current_step_index: usize,
    #[serde(default)]
    try_values: HashMap<String, Value>,
}

fn default_envelope() -> FormsPlayEnvelope {
    let store = FormsStore::new(create_document_vcs_envelope(
        FORMS_DOCUMENT_SCHEMA,
        "forms-play",
        empty_forms_projection(),
        None,
    ));
    FormsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: Vec::new(),
        current_step_index: 0,
        try_values: HashMap::new(),
    }
}

fn building_component_envelope() -> FormsPlayEnvelope {
    let spec: FormSpec =
        serde_json::from_str(BUILDING_COMPONENT_EXAMPLE_JSON).unwrap_or_else(|_| empty_forms_projection());
    let store = FormsStore::new(create_document_vcs_envelope(
        FORMS_DOCUMENT_SCHEMA,
        "forms-play",
        spec,
        None,
    ));
    FormsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: Vec::new(),
        current_step_index: 0,
        try_values: HashMap::new(),
    }
}

fn parse_envelope(document_json: &str) -> FormsPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &FormsPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn forms_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: FORMS_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn store_from_envelope(play: &FormsPlayEnvelope) -> FormsStore {
    let mut store = FormsStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(store: &FormsStore, play: &FormsPlayEnvelope) -> FormsPlayEnvelope {
    FormsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: play.selected_ids.clone(),
        current_step_index: play.current_step_index,
        try_values: play.try_values.clone(),
    }
}

fn effective_try_values(spec: &FormSpec, play: &FormsPlayEnvelope) -> Map<String, Value> {
    let overrides: Map<String, Value> = play.try_values.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
    initial_try_values(spec, &overrides)
}

fn reset_try_runtime(play: &mut FormsPlayEnvelope) {
    play.try_values.clear();
    play.current_step_index = 0;
}

fn materialized_projection(play: &FormsPlayEnvelope) -> FormSpec {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}
//#endregion 🔖Envelope

//#region 🔖Helpers
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
        if let Some(question) = step.questions.iter().find(|question| question.id == question_id) {
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
        if let Contribution::FormsQuestionKind { question_kind, .. } = &entry.contribution {
            if question_kind == kind {
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
    let Contribution::FormsQuestionKind {
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
        .flat_map(|step| step.questions.iter().map(|question| (step.title.clone(), question.clone())))
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

fn patch_try_object_field(play: &mut FormsPlayEnvelope, key: &str, field: &str, raw: &Value) {
    let mut object = play.try_values.get(key).cloned().unwrap_or_else(|| json!({}));
    if let Some(map) = object.as_object_mut() {
        map.insert(field.into(), raw.clone());
        play.try_values.insert(key.into(), object);
    }
}

fn patch_try_vector_field(play: &mut FormsPlayEnvelope, key: &str, index: usize, raw: &Value) {
    let mut array = play
        .try_values
        .get(key)
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default();
    while array.len() <= index {
        array.push(json!(0.0));
    }
    array[index] = raw.clone();
    play.try_values.insert(key.into(), Value::Array(array));
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
        return Some(if drop_position == "before" { 0 } else { step.questions.len() });
    }
    let target_index = step.questions.iter().position(|question| question.id == target_id)?;
    Some(match drop_position {
        "before" => target_index,
        "after" => target_index + 1,
        _ => step.questions.len(),
    })
}

fn update_question(store: &mut FormsStore, step_id: &str, question: FormQuestion) {
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![FormOp::UpdateQuestion { step_id: step_id.into(), question }],
        description: None,
    });
}

fn patch_question_field(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, field: &str, raw_value: &Value) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    match field {
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
    }
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn patch_question_option(
    play: &mut FormsPlayEnvelope,
    store: &mut FormsStore,
    question_id: &str,
    option_value: &str,
    field: &str,
    raw_value: &Value,
) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut options = question.options.take().unwrap_or_default();
    if let Some(option) = options.iter_mut().find(|entry| entry.value == option_value) {
        if field == "label" {
            option.label = raw_value.as_str().unwrap_or("").to_string();
        }
    }
    question.options = Some(options);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn add_question_option(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, label: &str) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut options = question.options.take().unwrap_or_default();
    let value = create_form_id("opt");
    options.push(FormQuestionOption {
        value,
        label: label.into(),
    });
    question.options = Some(options);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn remove_question_option(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, option_value: &str) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut options = question.options.take().unwrap_or_default();
    options.retain(|entry| entry.value != option_value);
    question.options = Some(options);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn patch_vector_field(
    play: &mut FormsPlayEnvelope,
    store: &mut FormsStore,
    question_id: &str,
    field_key: &str,
    field: &str,
    raw_value: &Value,
) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut fields = question.fields.take().unwrap_or_default();
    if let Some(entry) = fields.iter_mut().find(|item| item.key == field_key) {
        match field {
            "label" => entry.label = raw_value.as_str().map(str::to_string),
            "value" => entry.value = raw_value.as_f64(),
            _ => {}
        }
    }
    question.fields = Some(fields);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn add_vector_field(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, key: &str) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut fields = question.fields.take().unwrap_or_default();
    if fields.iter().any(|entry| entry.key == key) {
        return;
    }
    fields.push(FormVectorField {
        key: key.into(),
        label: Some(key.into()),
        value: Some(0.0),
    });
    question.fields = Some(fields);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn remove_vector_field(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, field_key: &str) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut fields = question.fields.take().unwrap_or_default();
    fields.retain(|entry| entry.key != field_key);
    question.fields = Some(fields);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn patch_building_component_param(
    play: &mut FormsPlayEnvelope,
    store: &mut FormsStore,
    question_id: &str,
    param_key: &str,
    raw_value: &Value,
) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    let mut question = location.question;
    let mut params = question.params.take().unwrap_or_else(|| json!({}));
    if let Some(map) = params.as_object_mut() {
        map.insert(param_key.into(), raw_value.clone());
    }
    question.params = Some(params);
    update_question(store, &location.step_id, question);
    reset_try_runtime(play);
}

fn apply_store_action(play: &mut FormsPlayEnvelope, store: &mut FormsStore) -> Vec<String> {
    *play = sync_store_to_envelope(store, play);
    vec![set_document_op(play)]
}

fn catalogue_kinds(contributions: &[PluginContributionEntry]) -> Vec<(String, String, String)> {
    let mut kinds: Vec<(String, String, String)> = FORM_BUILTIN_KINDS
        .iter()
        .map(|kind| {
            let (label, icon) = match *kind {
                "text" => ("Text", "type"),
                "longText" => ("Long Text", "align-left"),
                "number" => ("Number", "hash"),
                "slider" => ("Slider", "sliders-horizontal"),
                "boolean" => ("Boolean", "toggle-left"),
                "single" => ("Single Select", "circle-dot"),
                "multi" => ("Multi Select", "list-checks"),
                "date" => ("Date", "calendar"),
                "color" => ("Color", "palette"),
                "image" => ("Image", "image"),
                "file" => ("File", "file"),
                "vector" => ("Vector", "move-3d"),
                "note" => ("Note", "sticky-note"),
                other => (other, "help-circle"),
            };
            (kind.to_string(), label.into(), icon.into())
        })
        .collect();
    for entry in contributions {
        if let Contribution::FormsQuestionKind {
            question_kind,
            label,
            icon_id,
            ..
        } = &entry.contribution
        {
            kinds.push((question_kind.clone(), label.clone(), icon_id.clone()));
        }
    }
    kinds
}
//#endregion 🔖Helpers

//#region 🔖Builder
fn builder_text_editor(id: String, label: &str, value: String, on_change: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value,
            placeholder: None,
            commit: None,
            on_change,
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
    })
}

fn builder_button(id: String, icon_id: &str, label: &str, action: ActionDescriptor, disabled: bool) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(id),
        icon_id: icon_id.into(),
        label: label.into(),
        action,
        style: None,
        disabled: Some(disabled).filter(|disabled| *disabled),
    })
}

fn builder_question_card(
    question: &FormQuestion,
    step: &FormStep,
    index: usize,
    selected_ids: &[String],
    contributions: &[PluginContributionEntry],
) -> UiNode {
    let question_ids = vec![question.id.clone()];
    let prefix = format!("forms-blueprint.{}", question.id);
    let kind_label = catalogue_kinds(contributions)
        .into_iter()
        .find(|(kind, _, _)| *kind == question.kind)
        .map(|(_, label, _)| label)
        .unwrap_or_else(|| question.kind.clone());
    let required = question.required.unwrap_or(false);
    let mut children = vec![
        ui_text_emphasized(format!("{kind_label} · {}", question.id)),
        builder_text_editor(
            format!("{prefix}.label"),
            "Label",
            question.label.clone(),
            forms_action("patchQuestions", Some(json!({ "questionIds": question_ids, "field": "label" }))),
        ),
        UiNode::Field(UiFieldNode {
            id: format!("{prefix}.required"),
            label: "Required".into(),
            description: None,
            required: None,
            error: None,
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: format!("{prefix}.required.toggle"),
                icon_id: "check".into(),
                pressed: required,
                text: Some(if required { "Yes".into() } else { "No".into() }),
                on_change: forms_action("patchQuestions", Some(json!({ "questionIds": question_ids, "field": "required" }))),
            })),
        }),
    ];
    children.extend(question_kind_editor_fields(question, &question_ids, contributions, &prefix));
    children.push(ui_stack_horizontal(vec![
        builder_button(
            format!("{prefix}.remove"),
            "trash-2",
            "Remove Question",
            forms_action("removeQuestion", Some(json!({ "questionId": question.id }))),
            false,
        ),
        builder_button(
            format!("{prefix}.move-up"),
            "arrow-up",
            "Move Up",
            forms_action(
                "moveQuestion",
                Some(json!({ "questionId": question.id, "toStepId": step.id, "index": index.saturating_sub(1) })),
            ),
            index == 0,
        ),
        builder_button(
            format!("{prefix}.move-down"),
            "arrow-down",
            "Move Down",
            forms_action(
                "moveQuestion",
                Some(json!({ "questionId": question.id, "toStepId": step.id, "index": index + 1 })),
            ),
            index + 1 >= step.questions.len(),
        ),
    ]));
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("tight".into()),
        padding: None,
        id: Some(format!("forms-blueprint.card.{}", question.id)),
        selected: Some(selected_ids.contains(&question.id)).filter(|selected| *selected),
        activate: Some(forms_action("setSelection", Some(json!({ "ids": [question.id] })))),
        drop_action: Some(forms_action(
            "dropQuestionKind",
            Some(json!({ "targetId": question.id, "dropPosition": "after" })),
        )),
        children,
    })
}

fn builder_step_section(
    spec: &FormSpec,
    step: &FormStep,
    step_index: usize,
    selected_ids: &[String],
    contributions: &[PluginContributionEntry],
) -> UiNode {
    let prefix = format!("forms-blueprint.step.{}", step.id);
    let mut children = vec![
        builder_text_editor(
            format!("{prefix}.title"),
            "Title",
            step.title.clone(),
            forms_action("patchStep", Some(json!({ "stepId": step.id, "field": "title" }))),
        ),
        builder_text_editor(
            format!("{prefix}.description"),
            "Description",
            step.description.clone().unwrap_or_default(),
            forms_action("patchStep", Some(json!({ "stepId": step.id, "field": "description" }))),
        ),
    ];
    for (index, question) in step.questions.iter().enumerate() {
        children.push(builder_question_card(question, step, index, selected_ids, contributions));
    }
    children.push(UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("tight".into()),
        padding: None,
        id: Some(format!("{prefix}.dropzone")),
        selected: None,
        activate: None,
        drop_action: Some(forms_action(
            "dropQuestionKind",
            Some(json!({ "targetId": forms_play_step_tree_id(&step.id), "dropPosition": "inside" })),
        )),
        children: vec![ui_text("Drop a question kind here")],
    }));
    children.push(ui_stack_horizontal(vec![
        builder_button(
            format!("{prefix}.add-question"),
            "plus",
            "Add Question",
            forms_action("addQuestion", Some(json!({ "stepId": step.id, "kind": "text" }))),
            false,
        ),
        builder_button(
            format!("{prefix}.remove"),
            "trash-2",
            "Remove Step",
            forms_action("removeStep", Some(json!({ "stepId": step.id }))),
            false,
        ),
        builder_button(
            format!("{prefix}.move-up"),
            "arrow-up",
            "Move Up",
            forms_action("moveStep", Some(json!({ "stepId": step.id, "index": step_index.saturating_sub(1) }))),
            step_index == 0,
        ),
        builder_button(
            format!("{prefix}.move-down"),
            "arrow-down",
            "Move Down",
            forms_action("moveStep", Some(json!({ "stepId": step.id, "index": step_index + 1 }))),
            step_index + 1 >= spec.steps.len(),
        ),
    ]));
    UiNode::Section(UiSectionNode {
        id: prefix,
        label: Some(step.title.clone()),
        default_open: Some(true),
        children,
    })
}

fn render_blueprint_builder(spec: &FormSpec, play: &FormsPlayEnvelope, contributions: &[PluginContributionEntry]) -> UiNode {
    let mut children = vec![builder_text_editor(
        "forms-blueprint.title".into(),
        "Form Title",
        spec.title.clone().unwrap_or_default(),
        forms_action("updateForm", Some(json!({ "field": "title" }))),
    )];
    for (step_index, step) in spec.steps.iter().enumerate() {
        children.push(builder_step_section(spec, step, step_index, &play.selected_ids, contributions));
    }
    children.push(builder_button(
        "forms-blueprint.add-step".into(),
        "plus",
        "Add Step",
        forms_action("addStep", None),
        false,
    ));
    ui_stack_vertical(children)
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
    })
}

fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "horizontal".into(),
        gap: Some("tight".into()),
        padding: Some("none".into()),
        id: None,
        selected: None,
        activate: None,
        children,
        drop_action: None,
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
    })
}

fn render_try_question(
    question: &FormQuestion,
    values: &Map<String, Value>,
    contributions: &[PluginContributionEntry],
    error: Option<&str>,
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
            }),
        ),
        "boolean" => try_field(
            question,
            error,
            UiNode::Toggle(UiToggleNode {
                id: format!("forms-try.{key}.toggle"),
                icon_id: "check".into(),
                pressed: value.as_bool().unwrap_or(false),
                text: Some(if value.as_bool().unwrap_or(false) { "Yes".into() } else { "No".into() }),
                on_change: try_value_action(&key),
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
                                pressed: selected.contains(&option.value),
                                text: Some(option.label.clone()),
                                on_change: forms_action(
                                    "setTryValue",
                                    Some(json!({ "key": key, "optionValue": option.value })),
                                ),
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
            }),
        ),
        kind if is_extension_question_kind(kind) => {
            render_extension_question(question, values, contributions, "try", true)
        }
        _ => ui_text(format!("Unsupported kind: {}", question.kind)),
    }
}

fn render_try_wizard(spec: &FormSpec, play: &FormsPlayEnvelope, contributions: &[PluginContributionEntry]) -> UiNode {
    if spec.steps.is_empty() {
        return ui_text("No steps in this form.");
    }
    let step_index = play.current_step_index.min(spec.steps.len().saturating_sub(1));
    let step = &spec.steps[step_index];
    let values = effective_try_values(spec, play);
    let visible = visible_questions(step, &values);
    let errors = forms::step_errors(step, &values);
    let advance = can_advance(step, &values);
    let errors_by_question: HashMap<&str, &str> = errors
        .iter()
        .map(|error| (error.question_id.as_str(), error.message.as_str()))
        .collect();
    let mut children = vec![
        ui_text_emphasized(spec.title.clone().unwrap_or_else(|| "Form".into())),
        ui_text(format!("Step {} / {}", step_index + 1, spec.steps.len())),
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
        ));
    }
    let nav = vec![
        UiNode::Button(UiButtonNode {
            id: Some("forms-try.back".into()),
            icon_id: "chevron-left".into(),
            label: "Back".into(),
            action: forms_action("previousStep", None),
            style: None,
            disabled: Some(step_index == 0).filter(|disabled| *disabled),
        }),
        if step_index + 1 < spec.steps.len() {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.next".into()),
                icon_id: "chevron-right".into(),
                label: "Next".into(),
                action: forms_action("nextStep", None),
                style: None,
                disabled: Some(!advance).filter(|disabled| *disabled),
            })
        } else {
            UiNode::Button(UiButtonNode {
                id: Some("forms-try.submit".into()),
                icon_id: "check".into(),
                label: "Submit".into(),
                action: forms_action("submit", None),
                style: None,
                disabled: Some(!advance).filter(|disabled| *disabled),
            })
        },
    ];
    children.push(ui_stack_horizontal(nav));
    ui_stack_vertical(children)
}
//#endregion 🔖TryWizard

//#region 🔖Panels
fn build_document_tree(spec: &FormSpec, selected_ids: &[String]) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = spec
        .steps
        .iter()
        .map(|step| UiTreeItemNode {
            id: forms_play_step_tree_id(&step.id),
            label: step.title.clone(),
            description: Some(format!("{} questions", step.questions.len())),
            icon_id: Some("list-tree".into()),
            selected: None,
            default_open: Some(true),
            action: Some(forms_action("setSelection", Some(json!({ "ids": [] })))),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: Some(true),
            drag_data: None,
            items: Some(
                step.questions
                    .iter()
                    .map(|question| UiTreeItemNode {
                        id: question.id.clone(),
                        label: question.label.clone(),
                        description: Some(question.kind.clone()),
                        icon_id: Some("help-circle".into()),
                        selected: None,
                        default_open: None,
                        action: Some(forms_action("setSelection", Some(json!({ "ids": [question.id.clone()] })))),
                        hover_action: None,
                        unhover_action: None,
                        actions: None,
                        draggable: Some(true),
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    })
                    .collect(),
            ),
            control: None,
            is_hidden: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "forms-play-document.steps".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if step_items.is_empty() {
                vec![UiTreeItemNode {
                    id: "forms-play-document.empty".into(),
                    label: "(no steps)".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }]
            } else {
                step_items
            },
        }],
        selected_ids: Some(selected_ids.to_vec()),
        highlighted_ids: None,
        selection_change: Some(forms_action("setSelection", None)),
        drop_action: Some(forms_action("dropQuestionKind", None)),
    })
}

fn build_catalogue_tree(contributions: &[PluginContributionEntry]) -> UiNode {
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds(contributions)
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                id: format!("forms-play-catalogue.{kind}"),
                label: label.clone(),
                description: Some(kind.clone()),
                icon_id: Some(icon.clone()),
                selected: None,
                default_open: None,
                action: Some(forms_action("addQuestion", Some(json!({ "kind": kind })))),
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: Some(true),
                drag_data: Some(drag_data),
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "forms-play-catalogue.kinds".into(),
                label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
                default_open: Some(true),
                items: kind_items,
            },
            UiTreeSectionNode {
                id: "forms-play-catalogue.actions".into(),
                label: Some("Actions".into()),
                default_open: Some(true),
                items: vec![
                    UiTreeItemNode {
                        id: "forms-play-catalogue.add-step".into(),
                        label: "Add Step".into(),
                        description: None,
                        icon_id: Some("plus".into()),
                        selected: None,
                        default_open: None,
                        action: Some(forms_action("addStep", None)),
                        hover_action: None,
                        unhover_action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    },
                    UiTreeItemNode {
                        id: "forms-play-catalogue.add-question".into(),
                        label: "Add Text Question".into(),
                        description: None,
                        icon_id: Some("type".into()),
                        selected: None,
                        default_open: None,
                        action: Some(forms_action("addQuestion", Some(json!({ "kind": "text" })))),
                        hover_action: None,
                        unhover_action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    },
                ],
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
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
        })),
        description: None,
        required: None,
        error: None,
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
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn question_kind_editor_fields(
    question: &FormQuestion,
    question_ids: &[String],
    contributions: &[PluginContributionEntry],
    id_prefix: &str,
) -> Vec<UiNode> {
    let fid = |suffix: &str| format!("{id_prefix}.{suffix}");
    let mut fields = Vec::new();
    fields.push(inspector_text_field(
        question_ids,
        &fid("description"),
        "Description",
        &[question.description.clone().unwrap_or_default()],
        "description",
    ));
    match question.kind.as_str() {
        "text" | "longText" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("placeholder"),
                "Placeholder",
                &[question.placeholder.clone().unwrap_or_default()],
                "placeholder",
            ));
            fields.push(inspector_text_field(
                question_ids,
                &fid("default"),
                "Default",
                &[question.default.as_ref().map(json_string_value).unwrap_or_default()],
                "default",
            ));
        }
        "number" | "slider" => {
            fields.push(inspector_number_field(question_ids, &fid("min"), "Min", &[question.min.unwrap_or(0.0)], "min"));
            fields.push(inspector_number_field(question_ids, &fid("max"), "Max", &[question.max.unwrap_or(100.0)], "max"));
            fields.push(inspector_number_field(question_ids, &fid("step"), "Step", &[question.step.unwrap_or(1.0)], "step"));
            fields.push(inspector_number_field(
                question_ids,
                &fid("default"),
                "Default",
                &[question.default.as_ref().map(json_f64_value).unwrap_or(0.0)],
                "default",
            ));
            if question.kind == "slider" {
                fields.push(inspector_text_field(
                    question_ids,
                    &fid("unit"),
                    "Unit",
                    &[question.unit.clone().unwrap_or_default()],
                    "unit",
                ));
            }
        }
        "boolean" => {
            let pressed = question.default.as_ref().and_then(|default| default.as_bool()).unwrap_or(false);
            fields.push(UiNode::Field(UiFieldNode {
                id: fid("default"),
                label: "Default".into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Toggle(UiToggleNode {
                    id: fid("default.toggle"),
                    icon_id: "check".into(),
                    pressed,
                    text: Some(if pressed { "Yes".into() } else { "No".into() }),
                    on_change: inspector_patch(question_ids, "default"),
                })),
            }));
        }
        "single" | "multi" => {
            if let Some(options) = &question.options {
                for option in options {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("option.{}", option.value)),
                        label: format!("Option {}", option.value),
                        description: None,
                        required: None,
                        error: None,
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
                        })),
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("option.{}.remove", option.value))),
                        icon_id: "trash-2".into(),
                        label: "Remove Option".into(),
                        action: forms_action(
                            "removeQuestionOption",
                            Some(json!({ "questionId": question.id, "optionValue": option.value })),
                        ),
                        style: None,
                        disabled: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("option.add")),
                icon_id: "plus".into(),
                label: "Add Option".into(),
                action: forms_action("addQuestionOption", Some(json!({ "questionId": question.id, "label": "New option" }))),
                style: None,
                disabled: None,
            }));
        }
        "date" | "color" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("default"),
                "Default",
                &[question.default.as_ref().map(json_string_value).unwrap_or_default()],
                "default",
            ));
        }
        "vector" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("schema"),
                "Schema",
                &[question.schema.clone().unwrap_or_default()],
                "schema",
            ));
            fields.push(inspector_number_field(question_ids, &fid("step"), "Step", &[question.step.unwrap_or(0.1)], "step"));
            if let Some(vector_fields) = &question.fields {
                for field in vector_fields {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.label", field.key)),
                        label: format!("{} label", field.key),
                        description: None,
                        required: None,
                        error: None,
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
                        })),
                    }));
                    fields.push(UiNode::Field(UiFieldNode {
                        id: fid(&format!("vector.{}.value", field.key)),
                        label: format!("{} value", field.key),
                        description: None,
                        required: None,
                        error: None,
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
                        })),
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(fid(&format!("vector.{}.remove", field.key))),
                        icon_id: "trash-2".into(),
                        label: format!("Remove {}", field.key),
                        action: forms_action(
                            "removeVectorField",
                            Some(json!({ "questionId": question.id, "fieldKey": field.key })),
                        ),
                        style: None,
                        disabled: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some(fid("vector.add")),
                icon_id: "plus".into(),
                label: "Add Vector Field".into(),
                action: forms_action(
                    "addVectorField",
                    Some(json!({ "questionId": question.id, "fieldKey": "field" })),
                ),
                style: None,
                disabled: None,
            }));
        }
        "note" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("text"),
                "Text",
                &[question.text.clone().unwrap_or_default()],
                "text",
            ));
        }
        "image" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("src"),
                "Src",
                &[question.src.clone().unwrap_or_default()],
                "src",
            ));
        }
        "file" => {
            fields.push(inspector_text_field(
                question_ids,
                &fid("accept"),
                "Accept",
                &[question.accept.clone().unwrap_or_default()],
                "accept",
            ));
        }
        kind if is_extension_question_kind(kind) => {
            let values = serde_json::Map::new();
            fields.push(render_extension_question(question, &values, contributions, "blueprint", true));
            if let Some(slug) = &question.fixture_slug {
                fields.push(ui_inspector_readonly_field(fid("fixtureSlug"), "Fixture Slug", slug));
            }
        }
        _ => {}
    }
    fields
}

fn build_inspector_tree(
    spec: &FormSpec,
    play: &FormsPlayEnvelope,
    contributions: &[PluginContributionEntry],
) -> UiNode {
    let questions: Vec<FormQuestion> = play
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
    let kind_items: Vec<UiSelectItem> = catalogue_kinds(contributions)
        .into_iter()
        .map(|(kind, label, _)| UiSelectItem {
            value: kind,
            label,
        })
        .collect();
    let mut base_fields = vec![
        inspector_text_field(&question_ids, "forms-play-inspector.label", "Label", &labels, "label"),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.kind".into(),
            label: "Kind".into(),
            child: Box::new(UiNode::Select(UiSelectNode {
                id: "forms-play-inspector.kind.select".into(),
                value: kind_mixed.value,
                placeholder: kind_mixed.placeholder,
                items: kind_items,
                on_change: inspector_patch(&question_ids, "kind"),
            })),
            description: None,
            required: None,
            error: None,
        }),
        ui_inspector_readonly_field(
            "forms-play-inspector.id",
            "Id",
            if question_ids.len() == 1 {
                question_ids[0].clone()
            } else {
                format!("{} selected", question_ids.len())
            },
        ),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.required".into(),
            label: "Required".into(),
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: "forms-play-inspector.required.toggle".into(),
                icon_id: "check".into(),
                pressed: required_mixed.uniform && required_mixed.pressed,
                text: if required_mixed.uniform {
                    Some(if required_mixed.pressed { "Yes".into() } else { "No".into() })
                } else {
                    Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
                on_change: inspector_patch(&question_ids, "required"),
            })),
            description: None,
            required: None,
            error: None,
        }),
    ];
    if questions.len() == 1 {
        base_fields.extend(question_kind_editor_fields(&questions[0], &question_ids, contributions, "forms-play-inspector"));
    }
    let groups = vec![UiInspectorFieldGroup {
        id: "forms-play-inspector.base".into(),
        label: "Question".into(),
        default_open: None,
        fields: base_fields,
    }];
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖FormsPlayApp
struct FormsPlayApp;

impl PluginApp for FormsPlayApp {
    fn app_id(&self) -> &str {
        FORMS_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&building_component_envelope()).expect("forms envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match action {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<FormsPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    play.selected_ids = ids
                        .iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect();
                    return vec![set_document_op(&play)];
                }
            }
            "addStep" => {
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let step = FormStep {
                    id: create_form_id("step"),
                    title: format!("Step {}", projection.steps.len() + 1),
                    description: None,
                    questions: Vec::new(),
                };
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddStep { step, index: None }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "patchStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(step) = projection.steps.iter().find(|step| step.id == step_id).cloned() else {
                    return Vec::new();
                };
                let step = match field {
                    "title" => FormStep {
                        title: raw_value.into(),
                        ..step
                    },
                    "description" => FormStep {
                        description: Some(raw_value.to_string()).filter(|description| !description.is_empty()),
                        ..step
                    },
                    _ => return Vec::new(),
                };
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::UpdateStep { step }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "removeStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                if step_id.is_empty() {
                    return Vec::new();
                }
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let removed_ids: Vec<String> = projection
                    .steps
                    .iter()
                    .filter(|step| step.id == step_id)
                    .flat_map(|step| step.questions.iter().map(|question| question.id.clone()))
                    .collect();
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::RemoveStep { step_id: step_id.into() }],
                    description: None,
                });
                play.selected_ids.retain(|id| !removed_ids.contains(id));
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "moveStep" => {
                let step_id = args.and_then(|value| value.get("stepId")).and_then(|value| value.as_str()).unwrap_or("");
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if step_id.is_empty() {
                    return Vec::new();
                }
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::MoveStep {
                        step_id: step_id.into(),
                        index,
                    }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "updateForm" => {
                let title = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::UpdateForm {
                        title: Some(title.to_string()).filter(|title| !title.is_empty()),
                    }],
                    description: None,
                });
                return apply_store_action(&mut play, &mut store);
            }
            "addQuestion" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("text");
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let step_id = args
                    .and_then(|value| value.get("stepId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| projection.steps.first().map(|step| step.id.clone()));
                let Some(step_id) = step_id else {
                    return Vec::new();
                };
                let question = default_question_for_kind(kind, create_form_id("q"));
                let select_id = question.id.clone();
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddQuestion {
                        step_id,
                        question,
                        index: None,
                    }],
                    description: None,
                });
                play.try_values.clear();
                play.selected_ids = vec![select_id];
                return apply_store_action(&mut play, &mut store);
            }
            "removeQuestion" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                if question_id.is_empty() {
                    return Vec::new();
                }
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(location) = find_question_location(&projection, question_id) else {
                    return Vec::new();
                };
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::RemoveQuestion {
                        step_id: location.step_id,
                        question_id: question_id.into(),
                    }],
                    description: None,
                });
                play.selected_ids.retain(|id| id != question_id);
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
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
                    return Vec::new();
                }
                if field == "param" {
                    let param_key = args
                        .and_then(|value| value.get("paramKey"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("");
                    for question_id in question_ids {
                        patch_building_component_param(&mut play, &mut store, &question_id, param_key, &raw_value);
                    }
                } else {
                    for question_id in question_ids {
                        patch_question_field(&mut play, &mut store, &question_id, field, &raw_value);
                    }
                }
                return apply_store_action(&mut play, &mut store);
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
                for question_id in question_ids {
                    patch_question_option(&mut play, &mut store, &question_id, option_value, field, &raw_value);
                }
                return apply_store_action(&mut play, &mut store);
            }
            "addQuestionOption" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args
                    .and_then(|value| value.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("New option");
                if !question_id.is_empty() {
                    add_question_option(&mut play, &mut store, question_id, label);
                    return apply_store_action(&mut play, &mut store);
                }
            }
            "removeQuestionOption" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let option_value = args
                    .and_then(|value| value.get("optionValue"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !question_id.is_empty() && !option_value.is_empty() {
                    remove_question_option(&mut play, &mut store, question_id, option_value);
                    return apply_store_action(&mut play, &mut store);
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
                if !question_id.is_empty() && !field_key.is_empty() {
                    patch_vector_field(&mut play, &mut store, question_id, field_key, field, &raw_value);
                    return apply_store_action(&mut play, &mut store);
                }
            }
            "addVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args
                    .and_then(|value| value.get("fieldKey"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("field");
                if !question_id.is_empty() {
                    add_vector_field(&mut play, &mut store, question_id, field_key);
                    return apply_store_action(&mut play, &mut store);
                }
            }
            "removeVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args.and_then(|value| value.get("fieldKey")).and_then(|value| value.as_str()).unwrap_or("");
                if !question_id.is_empty() && !field_key.is_empty() {
                    remove_vector_field(&mut play, &mut store, question_id, field_key);
                    return apply_store_action(&mut play, &mut store);
                }
            }
            "moveQuestion" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let target_id = args
                    .and_then(|value| value.get("targetId"))
                    .and_then(|value| value.as_str());
                let position = args
                    .and_then(|value| value.get("position"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let (Some(question_id), Some(to_step_id)) = (question_id, to_step_id) else {
                    return Vec::new();
                };
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(source) = find_question_location(&projection, question_id) else {
                    return Vec::new();
                };
                let target_id = target_id.unwrap_or(question_id);
                let index = resolve_question_insert_index(&projection, to_step_id, target_id, position).unwrap_or(0);
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::MoveQuestion {
                        question_id: question_id.into(),
                        from_step_id: source.step_id,
                        to_step_id: to_step_id.into(),
                        index,
                    }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "dropQuestionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                let target_id = args.and_then(|value| value.get("targetId")).and_then(|value| value.as_str());
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let (Some(kind), Some(target_id)) = (kind, target_id) else {
                    return Vec::new();
                };
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(step_id) = resolve_step_id_from_tree_target(&projection, target_id) else {
                    return Vec::new();
                };
                let index = resolve_question_insert_index(&projection, &step_id, target_id, drop_position);
                let question = default_question_for_kind(kind, create_form_id("q"));
                let select_id = question.id.clone();
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddQuestion {
                        step_id,
                        question,
                        index,
                    }],
                    description: None,
                });
                play.try_values.clear();
                play.selected_ids = vec![select_id];
                return apply_store_action(&mut play, &mut store);
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                play.try_values.clear();
                return apply_store_action(&mut play, &mut store);
            }
            "exportFixture" => {
                let spec = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let _ = serde_json::to_string_pretty(&spec).unwrap_or_default();
                return Vec::new();
            }
            "setSpecJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(spec) = serde_json::from_str::<FormSpec>(json_text) {
                        let document_id = spec.id.clone();
                        let envelope = create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, &document_id, spec, None);
                        store = FormsStore::new(envelope);
                        reset_try_runtime(&mut play);
                        play.selected_ids.clear();
                        return apply_store_action(&mut play, &mut store);
                    }
                }
            }
            "editEngagementInput" | "tryEngagementInput" => {}
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let json_text = match example_id {
                    "building-component" => BUILDING_COMPONENT_EXAMPLE_JSON,
                    "default" => DEFAULT_EXAMPLE_JSON,
                    "onboarding" => ONBOARDING_EXAMPLE_JSON,
                    _ => return Vec::new(),
                };
                let spec: FormSpec = serde_json::from_str(json_text).unwrap_or_else(|_| materialized_projection(&play));
                let document_id = spec.id.clone();
                let envelope = create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, &document_id, spec, None);
                store = FormsStore::new(envelope);
                reset_try_runtime(&mut play);
                play.selected_ids.clear();
                return apply_store_action(&mut play, &mut store);
            }
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
                        let mut selected = play
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
                        play.try_values.insert(key.into(), Value::Array(selected));
                    } else if let Some(index) = vector_index {
                        if let Some(raw) = raw_value {
                            patch_try_vector_field(&mut play, key, index, &raw);
                        }
                    } else if let Some(param_key) = param_key {
                        if let Some(raw) = raw_value {
                            patch_try_object_field(&mut play, key, param_key, &raw);
                        }
                    } else if let Some(raw) = raw_value {
                        play.try_values.insert(key.into(), raw);
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "setTryValues" => {
                if let Some(values) = args.and_then(|value| value.get("values")).and_then(|value| value.as_object()) {
                    for (key, value) in values {
                        play.try_values.insert(key.clone(), value.clone());
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "resetTry" => {
                reset_try_runtime(&mut play);
                return vec![set_document_op(&play)];
            }
            "previousStep" => {
                if play.current_step_index > 0 {
                    play.current_step_index -= 1;
                    return vec![set_document_op(&play)];
                }
            }
            "nextStep" => {
                let spec = materialized_projection(&play);
                if play.current_step_index + 1 < spec.steps.len() {
                    let step = &spec.steps[play.current_step_index];
                    let values = effective_try_values(&spec, &play);
                    if can_advance(step, &values) {
                        play.current_step_index += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "submit" => {
                let spec = materialized_projection(&play);
                if !spec.steps.is_empty() {
                    let step_index = play.current_step_index.min(spec.steps.len() - 1);
                    let step = &spec.steps[step_index];
                    let values = effective_try_values(&spec, &play);
                    if can_advance(step, &values) {
                        return vec![set_document_op(&play)];
                    }
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let spec = materialized_projection(&play);
        let contributions = parse_contributions(view_state);
        match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => render_blueprint_builder(&spec, &play, &contributions),
            FORMS_PLAY_BODY_TRY => render_try_wizard(&spec, &play, &contributions),
            FORMS_PLAY_BODY_DOCUMENT => build_document_tree(&spec, &play.selected_ids),
            FORMS_PLAY_BODY_CATALOGUE => build_catalogue_tree(&contributions),
            FORMS_PLAY_BODY_INSPECTION => build_inspector_tree(&spec, &play, &contributions),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖FormsPlayApp

//#region 🔖AppFactory
fn create_forms_app() -> App {
    App::from_builder(
        App::builder(FORMS_PLAY_APP_ID, "Forms").document(["semio", "forms"])
            .icon_id("forms")
            .mode("blueprint", "Blueprint")
            .default_mode_id("blueprint")
            .window_kind(FORMS_PLAY_WINDOW_BLUEPRINT, "Blueprint", FORMS_PLAY_BODY_BLUEPRINT, SurfaceKind::NodeGraph)
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
            .operation("dropQuestionKind", "Drop Question Kind")
            .view_action("setSelection", "Set Selection")
            .view_action("setActiveExample", "Set Active Example")
            .view_action("setTryValue", "Set Try Value")
            .view_action("setTryValues", "Set Try Values")
            .view_action("resetTry", "Reset Try")
            .view_action("previousStep", "Previous Step")
            .view_action("nextStep", "Next Step")
            .view_action("submit", "Submit")
            .view_action("editEngagementInput", "Edit Engagement Input")
            .view_action("tryEngagementInput", "Try Engagement Input")
            .shell_action("setDocument", "Set Document")
            .shell_action("exportFixture", "Export Fixture")
            .shell_action("setSpecJson", "Set Spec JSON")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[FORMS_PLAY_WINDOW_BLUEPRINT.into(), FORMS_PLAY_WINDOW_TRY.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Blueprint".into(), "Try".into()]),
            )),
    )
    .example("empty", "Empty", serde_json::to_string(&default_envelope()).unwrap())
    .example("default", "Contact", DEFAULT_EXAMPLE_JSON)
    .example("onboarding", "Onboarding", ONBOARDING_EXAMPLE_JSON)
    .example("building-component", "Building Component", BUILDING_COMPONENT_EXAMPLE_JSON)
    .program("forms", "Forms", "data")
}

fn forms_bundle() -> PluginBundle {
    PluginBundle::new("forms", "Forms", "0.1.0").register_app(create_forms_app(), || Box::new(FormsPlayApp))
}

semio_framework_plugin::plugin_exports!(forms_bundle);
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::apply_form_edit_op;
    use semio_framework_plugin::PluginApp;

    fn apply_ops(document_json: &str, ops: &[String]) -> FormsPlayEnvelope {
        let mut play = parse_envelope(document_json);
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                    if let Some(document) = op.get("document") {
                        if let Ok(parsed) = serde_json::from_value(document.clone()) {
                            play = parsed;
                        }
                    }
                }
            }
        }
        play
    }

    #[test]
    fn initial_document_seeds_building_component_fixture() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let play = parse_envelope(&document);
        let spec = materialized_projection(&play);
        assert!(!flatten_questions(&spec).is_empty());
        assert!(
            flatten_questions(&spec)
                .iter()
                .any(|(_, question)| question.kind == "buildingComponent")
        );
    }

    #[test]
    fn renders_blueprint_builder_cards() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let play = parse_envelope(&document);
        let spec = materialized_projection(&play);
        let first_question_id = spec.steps[0].questions[0].id.clone();
        let node = app.render(FORMS_PLAY_BODY_BLUEPRINT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-blueprint.title"));
        assert!(json.contains(&format!("forms-blueprint.card.{first_question_id}")));
        assert!(json.contains("setSelection"));
        assert!(json.contains("forms-blueprint.add-step"));
        assert!(json.contains(&format!("forms-blueprint.{first_question_id}.label")));
    }

    #[test]
    fn blueprint_builder_card_reflects_selection() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let mut play = parse_envelope(&document);
        let spec = materialized_projection(&play);
        let first_question_id = spec.steps[0].questions[0].id.clone();
        play.selected_ids = vec![first_question_id.clone()];
        let node = app.render(FORMS_PLAY_BODY_BLUEPRINT, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""selected":true"#));
    }

    #[test]
    fn try_wizard_gates_navigation_and_reports_inline_errors() {
        let mut app = FormsPlayApp;
        let ops = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "default" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &ops);
        let clear_ops = app.handle_action_patch_ops(
            "setTryValues",
            Some(&json!({ "values": { "name": "", "email": "" } })),
            &serde_json::to_string(&play).unwrap(),
            &ViewState::default(),
        );
        let cleared = apply_ops(&serde_json::to_string(&play).unwrap(), &clear_ops);
        let node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&cleared).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""disabled":true"#));
        assert!(json.contains(r#""error":"#));
        assert!(json.contains("forms-try.back"));
    }

    #[test]
    fn try_wizard_emits_slider_unit_and_number_bounds() {
        let mut app = FormsPlayApp;
        let ops = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "onboarding" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &ops);
        let node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""min":13.0"#) || json.contains(r#""min":13"#));
        assert!(json.contains(r#""max":120.0"#) || json.contains(r#""max":120"#));
        let next_ops = app.handle_action_patch_ops(
            "setTryValues",
            Some(&json!({ "values": { "full-name": "Ada" } })),
            &serde_json::to_string(&play).unwrap(),
            &ViewState::default(),
        );
        let filled = apply_ops(&serde_json::to_string(&play).unwrap(), &next_ops);
        let step_ops = app.handle_action_patch_ops("nextStep", None, &serde_json::to_string(&filled).unwrap(), &ViewState::default());
        let second = apply_ops(&serde_json::to_string(&filled).unwrap(), &step_ops);
        let second_node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&second).unwrap(), &ViewState::default());
        let second_json = serde_json::to_string(&second_node).unwrap();
        assert!(second_json.contains(r#""unit":"%""#));
    }

    #[test]
    fn image_question_with_url_src_emits_image_node() {
        let app = FormsPlayApp;
        let question = FormQuestion {
            src: Some("https://example.com/picture.png".into()),
            ..question_shell("q-image".into(), "Picture".into(), "image".into())
        };
        let node = render_try_question(&question, &Map::new(), &[], None);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains("https://example.com/picture.png"));
        let _ = app;
    }

    #[test]
    fn patch_step_updates_title_and_description() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let step_id = materialized_projection(&parse_envelope(&document)).steps[0].id.clone();
        let ops = app.handle_action_patch_ops(
            "patchStep",
            Some(&json!({ "stepId": step_id, "field": "title", "value": "Renamed" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert_eq!(materialized_projection(&next).steps[0].title, "Renamed");
    }

    #[test]
    fn remove_and_move_step_actions() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let add_ops = app.handle_action_patch_ops("addStep", None, &document, &ViewState::default());
        let with_step = apply_ops(&document, &add_ops);
        let with_step_json = serde_json::to_string(&with_step).unwrap();
        let spec = materialized_projection(&with_step);
        let last_step_id = spec.steps.last().unwrap().id.clone();
        let move_ops = app.handle_action_patch_ops(
            "moveStep",
            Some(&json!({ "stepId": last_step_id, "index": 0 })),
            &with_step_json,
            &ViewState::default(),
        );
        let moved = apply_ops(&with_step_json, &move_ops);
        assert_eq!(materialized_projection(&moved).steps[0].id, last_step_id);
        let remove_ops = app.handle_action_patch_ops(
            "removeStep",
            Some(&json!({ "stepId": last_step_id })),
            &serde_json::to_string(&moved).unwrap(),
            &ViewState::default(),
        );
        let removed = apply_ops(&serde_json::to_string(&moved).unwrap(), &remove_ops);
        assert!(materialized_projection(&removed).steps.iter().all(|step| step.id != last_step_id));
    }

    #[test]
    fn update_form_action_sets_title() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "updateForm",
            Some(&json!({ "field": "title", "value": "My Form" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert_eq!(materialized_projection(&next).title.as_deref(), Some("My Form"));
    }

    #[test]
    fn document_tree_declares_drop_action() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""dropAction""#));
        assert!(json.contains("dropQuestionKind"));
    }

    #[test]
    fn drop_question_kind_inserts_and_selects() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let step_id = materialized_projection(&parse_envelope(&document)).steps[0].id.clone();
        let ops = app.handle_action_patch_ops(
            "dropQuestionKind",
            Some(&json!({ "kind": "slider", "targetId": forms_play_step_tree_id(&step_id), "dropPosition": "inside" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        let spec = materialized_projection(&next);
        assert!(spec.steps[0].questions.iter().any(|question| question.kind == "slider"));
        assert_eq!(next.selected_ids.len(), 1);
    }

    #[test]
    fn kind_editor_fields_are_editable_when_unset() {
        let question = question_shell("q-num".into(), "Amount".into(), "number".into());
        let fields = question_kind_editor_fields(&question, &["q-num".into()], &[], "forms-blueprint.q-num");
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

    #[test]
    fn extension_question_emits_external_slot_when_contribution_registered() {
        let app = FormsPlayApp;
        let contributions = vec![PluginContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            contribution: Contribution::FormsQuestionKind {
                app_id: "forms-module-procedural".into(),
                question_kind: "buildingComponent".into(),
                label: "Building Component".into(),
                icon_id: "building".into(),
                default_value_json: "{}".into(),
                params_body_key: "params".into(),
                preview_body_key: "preview".into(),
            },
        }];
        let mut view_state = ViewState::default();
        view_state.contributions_json = Some(serde_json::to_string(&contributions).unwrap());
        let mut play_app = FormsPlayApp;
        let ops = play_app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "building-component" })),
            &app.initial_document_json(),
            &view_state,
        );
        let mut play = apply_ops(&app.initial_document_json(), &ops);
        play.current_step_index = 1;
        let node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&play).unwrap(), &view_state);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    #[test]
    fn extension_question_falls_back_without_contribution() {
        let app = FormsPlayApp;
        let mut play_app = FormsPlayApp;
        let ops = play_app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "building-component" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let mut play = apply_ops(&app.initial_document_json(), &ops);
        play.current_step_index = 1;
        let node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[test]
    fn document_lists_steps() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-play-document.steps"));
        assert!(json.contains("Identity"));
        assert!(json.contains("Geometry"));
    }

    #[test]
    fn catalogue_lists_question_kinds() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-play-catalogue.text"));
        assert!(json.contains("forms-play-catalogue.add-step"));
    }

    #[test]
    fn add_step_action_appends_step() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document)).steps.len();
        let ops = app.handle_action_patch_ops("addStep", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let next = apply_ops(&document, &ops);
        assert_eq!(materialized_projection(&next).steps.len(), before + 1);
    }

    #[test]
    fn add_question_action_appends_question() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "addQuestion",
            Some(&json!({ "kind": "text" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert!(flatten_questions(&materialized_projection(&next))
            .iter()
            .any(|(_, question)| question.kind == "text"));
    }

    #[test]
    fn set_try_values_updates_runtime() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "setTryValues",
            Some(&json!({ "values": { "q-text": "Ada" } })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let next = apply_ops(&document, &ops);
        assert_eq!(next.try_values.get("q-text").and_then(|v| v.as_str()), Some("Ada"));
    }

    #[test]
    fn apply_form_edit_op_roundtrip() {
        let spec = empty_forms_projection();
        let step = FormStep {
            id: "step-test".into(),
            title: "Review".into(),
            description: None,
            questions: Vec::new(),
        };
        let next = apply_form_edit_op(&spec, &FormOp::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }

    #[test]
    fn wizard_step_navigation() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "onboarding" })),
            &document,
            &ViewState::default(),
        );
        let play = apply_ops(&document, &ops);
        assert_eq!(play.current_step_index, 0);
        let next_ops = app.handle_action_patch_ops("nextStep", None, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let next = apply_ops(&document, &next_ops);
        assert_eq!(next.current_step_index, 1);
        let back_ops = app.handle_action_patch_ops("previousStep", None, &serde_json::to_string(&next).unwrap(), &ViewState::default());
        let back = apply_ops(&document, &back_ops);
        assert_eq!(back.current_step_index, 0);
    }

    #[test]
    fn conditional_visibility_hides_team_size() {
        let mut app = FormsPlayApp;
        let ops = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "onboarding" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &ops);
        let spec = materialized_projection(&play);
        let advanced = spec.steps.iter().find(|step| step.id == "advanced").expect("advanced step");
        let values = effective_try_values(&spec, &play);
        assert_eq!(visible_questions(advanced, &values).len(), 1);
    }

    #[test]
    fn inspector_patch_updates_required() {
        let mut app = FormsPlayApp;
        let document = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "default" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &document);
        let spec = materialized_projection(&play);
        let name_id = spec.steps[0].questions[0].id.clone();
        let ops = app.handle_action_patch_ops(
            "patchQuestions",
            Some(&json!({ "questionIds": [name_id.clone()], "field": "required", "pressed": false })),
            &serde_json::to_string(&play).unwrap(),
            &ViewState::default(),
        );
        let next = apply_ops(&serde_json::to_string(&play).unwrap(), &ops);
        let next_spec = materialized_projection(&next);
        let question = &next_spec.steps[0].questions[0];
        assert!(!question.required.unwrap_or(true));
    }

    #[test]
    fn renders_try_wizard() {
        let mut app = FormsPlayApp;
        let ops = app.handle_action_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "default" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &ops);
        let node = app.render(FORMS_PLAY_BODY_TRY, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }
}
//#endregion 🧪Tests
