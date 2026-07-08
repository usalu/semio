//! 📋 Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.

use base64::Engine;
use flow_core::{FlowFixture, FlowHost, Widget};
use flow_module_brep::tessellate_geometry_json;
use forms::{
    can_advance, default_value_for_question, empty_forms_projection, flatten_form_questions,
    initial_try_values, is_extension_question_kind, visible_questions, FormOp, FormQuestion,
    FormQuestionOption, FormSpec, FormStep, FormVectorField, FormsEnvelope, FormsStore,
    FORMS_DOCUMENT_SCHEMA,
};
use image::RgbaImage;
use semio_framework_core::mesh_from_indexed;
use semio_framework_plugin::{
    build_raster_scene, build_table_scene, build_world_3d_scene, create_default_layout,
    mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text,
    ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, PanelGroup,
    CommandDescriptor, PluginApp, PluginBundle, RasterScene, TableScene, UiButtonNode,
    UiControlNode, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiNumberStepperNode,
    UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, ViewState, world3d_default_camera, world3d_scene, world3d_selection_json,
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
const FORMS_PLAY_SURFACE_EDIT: &str = "forms.play.edit";
const FORMS_PLAY_SURFACE_TRY: &str = "forms.play.try";
const FORMS_PLAY_SURFACE_PREVIEW: &str = "forms.play.preview";
const FORMS_PLAY_BODY_EDIT: &str = "forms.play.edit";
const FORMS_PLAY_BODY_TRY: &str = "forms.play.try";
const FORMS_PLAY_BODY_PREVIEW: &str = "forms.play.preview";
const FORMS_PLAY_BODY_DOCUMENT: &str = "forms.play.document";
const FORMS_PLAY_BODY_CATALOGUE: &str = "forms.play.catalogue";
const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
const FORMS_PLAY_WINDOW_EDIT: &str = "forms-edit";
const FORMS_PLAY_WINDOW_TRY: &str = "forms-try";
const FORMS_PLAY_WINDOW_PREVIEW: &str = "forms-preview";
const FORMS_QUESTION_DRAG_MIME: &str = "application/x-semio-forms-question-kind";
const FORMS_PREVIEW_FALLBACK_MESH_KIND: &str = "box";
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
const HEX_COLUMN_FIXTURE_JSON: &str = include_str!("../../../procedural/3d/example/hexagonal-mushroom-column.procedural.json");
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

fn parse_envelope(document_json: &str) -> FormsPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &FormsPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn forms_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: FORMS_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
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

fn fixture_json_for_slug(slug: &str) -> Option<&'static str> {
    match slug {
        "hexagonal-mushroom-column" => Some(HEX_COLUMN_FIXTURE_JSON),
        _ => None,
    }
}

fn first_building_component_question<'a>(spec: &'a FormSpec) -> Option<&'a FormQuestion> {
    flatten_form_questions(spec)
        .into_iter()
        .find(|question| question.kind == "buildingComponent")
}

fn building_component_params(question: &FormQuestion, values: &Map<String, Value>) -> Value {
    values
        .get(&question.id)
        .cloned()
        .or_else(|| question.params.clone())
        .unwrap_or_else(|| json!({}))
}

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

fn apply_store_command(play: &mut FormsPlayEnvelope, store: &mut FormsStore) -> Vec<String> {
    *play = sync_store_to_envelope(store, play);
    vec![set_document_op(play)]
}

fn catalogue_kinds() -> [(&'static str, &'static str, &'static str); 14] {
    [
        ("text", "Text", "type"),
        ("number", "Number", "hash"),
        ("boolean", "Boolean", "toggle-left"),
        ("single", "Single Select", "circle-dot"),
        ("multi", "Multi Select", "list-checks"),
        ("slider", "Slider", "sliders-horizontal"),
        ("longText", "Long Text", "align-left"),
        ("note", "Note", "sticky-note"),
        ("date", "Date", "calendar"),
        ("color", "Color", "palette"),
        ("image", "Image", "image"),
        ("file", "File", "file"),
        ("vector", "Vector", "move-3d"),
        ("buildingComponent", "Building Component", "building"),
    ]
}
//#endregion 🔖Helpers

//#region 🔖Preview
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputStepper { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn is_brep_geometry_handle(handle: &str) -> bool {
    handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
}

fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
    let widget_eval = eval.get(widget_id)?;
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles.into_iter().next()
}

fn mesh_from_tessellation_json(mesh_json: &str) -> Option<semio_framework_plugin::MeshData> {
    let parsed: Value = serde_json::from_str(mesh_json).ok()?;
    if parsed.get("error").is_some() {
        return None;
    }
    let positions: Vec<f32> = parsed
        .get("position")
        .or_else(|| parsed.get("positions"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
        .filter(|items: &Vec<f32>| !items.is_empty())?;
    let normals: Vec<f32> = parsed
        .get("normal")
        .or_else(|| parsed.get("normals"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect())
        .unwrap_or_default();
    let indices: Vec<u32> = parsed
        .get("index")
        .or_else(|| parsed.get("indices"))
        .and_then(|entry| entry.as_array())
        .map(|items| items.iter().filter_map(|value| value.as_u64().map(|number| number as u32)).collect())
        .filter(|items: &Vec<u32>| !items.is_empty())?;
    Some(mesh_from_indexed(&positions, &normals, &indices))
}

fn apply_flow_params(host: &mut FlowHost, fixture: &FlowFixture, params: &Value) {
    let Some(object) = params.as_object() else {
        return;
    };
    for (key, value) in object {
        if let Some(number) = value.as_f64() {
            host.set_slider_value(key, number);
        }
    }
    if let Ok(params_json) = serde_json::to_string(object) {
        for widget in &fixture.widgets {
            if let Widget::Neuron { id, .. } = widget {
                let _ = host.set_neuron_params(id, &params_json);
            }
        }
    }
}

fn evaluated_preview_payload(fixture: &FlowFixture, params: &Value) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let Some(handle) = geometry_handle_for_widget(&eval, &id) else {
            continue;
        };
        let mesh_id = format!("eval-{id}");
        if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            let tessellation = tessellate_geometry_json(&handle, 0.05);
            if let Some(data) = mesh_from_tessellation_json(&tessellation) {
                meshes.push(json!({ "id": mesh_id, "data": data }));
            }
        }
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            instances.push(json!({
                "id": id,
                "meshId": mesh_id,
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": false,
                "hovered": false,
            }));
        }
    }
    if meshes.is_empty() {
        let fallback = json!([{ "id": FORMS_PREVIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(FORMS_PREVIEW_FALLBACK_MESH_KIND) }]);
        let fallback_instances = json!([{
            "id": "preview",
            "meshId": FORMS_PREVIEW_FALLBACK_MESH_KIND,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": "preview",
            "selected": false,
            "hovered": false,
        }]);
        return (
            serde_json::to_string(&fallback).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&fallback_instances).unwrap_or_else(|_| "[]".into()),
        );
    }
    (
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()),
    )
}

fn render_preview_body(spec: &FormSpec, play: &FormsPlayEnvelope) -> UiNode {
    let Some(question) = first_building_component_question(spec) else {
        return ui_text("No building component question in this form.");
    };
    let slug = question.fixture_slug.as_deref().unwrap_or("hexagonal-mushroom-column");
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let values = effective_try_values(spec, play);
    let params = building_component_params(question, &values);
    let (meshes_json, instances_json) = evaluated_preview_payload(&fixture, &params);
    build_world_3d_scene(
        FORMS_PLAY_SURFACE_PREVIEW,
        FORMS_PLAY_CONTROLLER_ID,
        world3d_scene(
            world3d_default_camera(),
            meshes_json,
            instances_json,
            world3d_selection_json("single", &[], None),
        ),
    )
}
//#endregion 🔖Preview

//#region 🔖Tables
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EditQuestionRow {
    id: String,
    label: String,
    kind: String,
    step: String,
}

fn edit_table_rows(spec: &FormSpec) -> String {
    let rows: Vec<EditQuestionRow> = flatten_questions(spec)
        .into_iter()
        .map(|(step, question)| EditQuestionRow {
            id: question.id.clone(),
            label: question.label.clone(),
            kind: question.kind.clone(),
            step,
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn render_edit_table(spec: &FormSpec) -> UiNode {
    build_table_scene(
        FORMS_PLAY_SURFACE_EDIT,
        FORMS_PLAY_CONTROLLER_ID,
        TableScene {
            columns_json: json!([
                {"id":"id","label":"Id"},
                {"id":"label","label":"Label"},
                {"id":"kind","label":"Kind"},
                {"id":"step","label":"Step"}
            ])
            .to_string(),
            rows_json: edit_table_rows(spec),
        },
    )
}
//#endregion 🔖Tables

//#region 🔖TryWizard
fn try_value_cmd(key: &str) -> CommandDescriptor {
    forms_cmd("setTryValue", Some(json!({ "key": key })))
}

fn decode_image_src_to_rgba(src: &str) -> Option<RgbaImage> {
    if src.is_empty() || src.ends_with(".svg") {
        return None;
    }
    let bytes = if let Some(rest) = src.strip_prefix("data:") {
        let payload = rest.split(',').nth(1)?;
        base64::engine::general_purpose::STANDARD.decode(payload).ok()?
    } else if src.starts_with("http") || src.starts_with('/') {
        return None;
    } else {
        base64::engine::general_purpose::STANDARD.decode(src).ok()?
    };
    image::load_from_memory(&bytes).ok().map(|img| img.to_rgba8())
}

fn render_image_question(question: &FormQuestion) -> UiNode {
    let src = question.src.as_deref().unwrap_or("");
    let effective_src = if src.is_empty() { AVATAR_PLACEHOLDER_PNG_BASE64 } else { src };
    if let Some(image) = decode_image_src_to_rgba(effective_src) {
        return build_raster_scene(
            format!("{}.{}", FORMS_PLAY_SURFACE_TRY, question.id),
            FORMS_PLAY_CONTROLLER_ID,
            RasterScene {
                width: image.width(),
                height: image.height(),
                pixels_base64: base64::engine::general_purpose::STANDARD.encode(image.into_raw()),
            },
        );
    }
    ui_text("No image")
}

fn render_try_question(question: &FormQuestion, values: &Map<String, Value>) -> UiNode {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| default_value_for_question(question));
    let key = question.id.clone();
    match question.kind.as_str() {
        "text" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "text".into(),
                value: json_string_value(&value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        "longText" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "longText".into(),
                value: json_string_value(&value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        "number" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        "slider" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Slider(UiSliderNode {
                id: format!("forms-try.{key}.slider"),
                value: json_f64_value(&value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: try_value_cmd(&key),
            }),
        }),
        "boolean" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Toggle(UiToggleNode {
                id: format!("forms-try.{key}.toggle"),
                icon_id: "toggle-left".into(),
                pressed: value.as_bool().unwrap_or(false),
                text: Some(if value.as_bool().unwrap_or(false) { "Yes".into() } else { "No".into() }),
                on_change: try_value_cmd(&key),
            }),
        }),
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
            UiNode::Field(UiFieldNode {
                id: format!("forms-try.{key}"),
                label: question.label.clone(),
                child: UiControlNode::Select(UiSelectNode {
                    id: format!("forms-try.{key}.select"),
                    value: json_string_value(&value),
                    placeholder: None,
                    items,
                    on_change: try_value_cmd(&key),
                }),
            })
        }
        "multi" => {
            let selected: HashSet<String> = value
                .as_array()
                .map(|items| items.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            let toggles = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| {
                            UiNode::Field(UiFieldNode {
                                id: format!("forms-try.{key}.{}", option.value),
                                label: option.label.clone(),
                                child: UiControlNode::Toggle(UiToggleNode {
                                    id: format!("forms-try.{key}.{}.toggle", option.value),
                                    icon_id: "check".into(),
                                    pressed: selected.contains(&option.value),
                                    text: Some(option.label.clone()),
                                    on_change: forms_cmd(
                                        "setTryValue",
                                        Some(json!({ "key": key, "optionValue": option.value })),
                                    ),
                                }),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            UiNode::Section(semio_framework_plugin::UiSectionNode {
                id: format!("forms-try.{key}.section"),
                label: Some(question.label.clone()),
                default_open: Some(true),
                children: toggles,
            })
        }
        "date" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "date".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        "color" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "color".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        "vector" => {
            let array = value.as_array().cloned().unwrap_or_default();
            let fields = question.fields.as_ref().cloned().unwrap_or_default();
            let children: Vec<UiNode> = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_value = array.get(index).cloned().unwrap_or(json!(field.value.unwrap_or(0.0)));
                    UiNode::Field(UiFieldNode {
                        id: format!("forms-try.{key}.{}", field.key),
                        label: field.label.clone().unwrap_or_else(|| field.key.clone()),
                        child: UiControlNode::NumberStepper(UiNumberStepperNode {
                            id: format!("forms-try.{key}.{}.stepper", field.key),
                            value: json_f64_value(&field_value),
                            step: question.step.unwrap_or(0.1),
                            uniform: true,
                            on_absolute: forms_cmd(
                                "setTryValue",
                                Some(json!({ "key": key, "vectorIndex": index })),
                            ),
                            on_delta: forms_cmd(
                                "setTryValue",
                                Some(json!({ "key": key, "vectorIndex": index })),
                            ),
                        }),
                    })
                })
                .collect();
            UiNode::Section(semio_framework_plugin::UiSectionNode {
                id: format!("forms-try.{key}.section"),
                label: Some(question.label.clone()),
                default_open: Some(true),
                children,
            })
        }
        "note" => ui_text(question.text.clone().unwrap_or_else(|| question.label.clone())),
        "image" => UiNode::Section(semio_framework_plugin::UiSectionNode {
            id: format!("forms-try.{key}.section"),
            label: Some(question.label.clone()),
            default_open: Some(true),
            children: vec![render_image_question(question)],
        }),
        "file" => UiNode::Field(UiFieldNode {
            id: format!("forms-try.{key}"),
            label: question.label.clone(),
            child: UiControlNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "file".into(),
                value: json_string_value(&value),
                placeholder: question.accept.clone(),
                commit: None,
                on_change: try_value_cmd(&key),
            }),
        }),
        kind if is_extension_question_kind(kind) => {
            let mut params = value.as_object().cloned().unwrap_or_default();
            if params.is_empty() {
                if let Some(default_params) = question.params.as_ref().and_then(|entry| entry.as_object()) {
                    params = default_params.clone();
                }
            }
            let children: Vec<UiNode> = params
                .iter()
                .map(|(param_key, param_value)| {
                    UiNode::Field(UiFieldNode {
                        id: format!("forms-try.{key}.{param_key}"),
                        label: param_key.clone(),
                        child: UiControlNode::NumberStepper(UiNumberStepperNode {
                            id: format!("forms-try.{key}.{param_key}.stepper"),
                            value: json_f64_value(param_value),
                            step: 0.1,
                            uniform: true,
                            on_absolute: forms_cmd(
                                "setTryValue",
                                Some(json!({ "key": key, "paramKey": param_key })),
                            ),
                            on_delta: forms_cmd(
                                "setTryValue",
                                Some(json!({ "key": key, "paramKey": param_key })),
                            ),
                        }),
                    })
                })
                .collect();
            UiNode::Section(semio_framework_plugin::UiSectionNode {
                id: format!("forms-try.{key}.section"),
                label: Some(question.label.clone()),
                default_open: Some(true),
                children,
            })
        }
        _ => ui_text(format!("Unsupported kind: {}", question.kind)),
    }
}

fn render_try_wizard(spec: &FormSpec, play: &FormsPlayEnvelope) -> UiNode {
    if spec.steps.is_empty() {
        return ui_text("No steps in this form.");
    }
    let step_index = play.current_step_index.min(spec.steps.len().saturating_sub(1));
    let step = &spec.steps[step_index];
    let values = effective_try_values(spec, play);
    let visible = visible_questions(step, &values);
    let errors = forms::step_errors(step, &values);
    let _advance = can_advance(step, &values);
    let mut children = vec![
        ui_text(spec.title.clone().unwrap_or_else(|| "Form".into())),
        ui_text(format!("Step {} / {}", step_index + 1, spec.steps.len())),
    ];
    if let Some(description) = &step.description {
        children.push(ui_text(description.clone()));
    }
    for question in visible {
        children.push(render_try_question(question, &values));
    }
    for error in &errors {
        children.push(ui_text(format!("⚠ {}", error.message)));
    }
    let mut nav = Vec::new();
    if step_index > 0 {
        nav.push(UiNode::Button(UiButtonNode {
            id: Some("forms-try.back".into()),
            icon_id: "arrow-left".into(),
            label: "Back".into(),
            command: forms_cmd("previousStep", None),
            style: None,
        }));
    }
    if step_index + 1 < spec.steps.len() {
        nav.push(UiNode::Button(UiButtonNode {
            id: Some("forms-try.next".into()),
            icon_id: "arrow-right".into(),
            label: "Next".into(),
            command: forms_cmd("nextStep", None),
            style: None,
        }));
    } else {
        nav.push(UiNode::Button(UiButtonNode {
            id: Some("forms-try.submit".into()),
            icon_id: "check".into(),
            label: "Submit".into(),
            command: forms_cmd("submit", None),
            style: None,
        }));
    }
    children.push(ui_stack_vertical(nav));
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
            command: Some(forms_cmd("setSelection", Some(json!({ "ids": [] })))),
            hover_command: None,
            unhover_command: None,
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
                        command: Some(forms_cmd("setSelection", Some(json!({ "ids": [question.id.clone()] })))),
                        hover_command: None,
                        unhover_command: None,
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
                    command: None,
        hover_command: None,
        unhover_command: None,
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
        selection_change: Some(forms_cmd("setSelection", None)),
    })
}

fn build_catalogue_tree() -> UiNode {
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds()
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                id: format!("forms-play-catalogue.{kind}"),
                label: label.into(),
                description: Some(kind.into()),
                icon_id: Some(icon.into()),
                selected: None,
                default_open: None,
                command: Some(forms_cmd("addQuestion", Some(json!({ "kind": kind })))),
                hover_command: None,
                unhover_command: None,
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
                        command: Some(forms_cmd("addStep", None)),
                        hover_command: None,
                        unhover_command: None,
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
                        command: Some(forms_cmd("addQuestion", Some(json!({ "kind": "text" })))),
                        hover_command: None,
                        unhover_command: None,
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
    })
}

fn inspector_patch(question_ids: &[String], field: &str) -> CommandDescriptor {
    forms_cmd("patchQuestions", Some(json!({ "questionIds": question_ids, "field": field })))
}

fn inspector_text_field(question_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(question_ids, field),
        }),
    })
}

fn inspector_number_field(question_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::NumberStepper(UiNumberStepperNode {
            id: format!("{field_id}.stepper"),
            value: mixed.value,
            step: 0.1,
            uniform: mixed.uniform,
            on_absolute: inspector_patch(question_ids, field),
            on_delta: inspector_patch(question_ids, field),
        }),
    })
}

fn inspector_kind_fields(question: &FormQuestion, question_ids: &[String]) -> Vec<UiNode> {
    let mut fields = Vec::new();
    if let Some(description) = &question.description {
        fields.push(inspector_text_field(question_ids, "forms-play-inspector.description", "Description", &[description.clone()], "description"));
    }
    match question.kind.as_str() {
        "text" | "longText" => {
            if let Some(placeholder) = &question.placeholder {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.placeholder", "Placeholder", &[placeholder.clone()], "placeholder"));
            }
            if let Some(default) = &question.default {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.default", "Default", &[json_string_value(default)], "default"));
            }
        }
        "number" | "slider" => {
            if let Some(min) = question.min {
                fields.push(inspector_number_field(question_ids, "forms-play-inspector.min", "Min", &[min], "min"));
            }
            if let Some(max) = question.max {
                fields.push(inspector_number_field(question_ids, "forms-play-inspector.max", "Max", &[max], "max"));
            }
            if let Some(step) = question.step {
                fields.push(inspector_number_field(question_ids, "forms-play-inspector.step", "Step", &[step], "step"));
            }
            if let Some(default) = &question.default {
                fields.push(inspector_number_field(question_ids, "forms-play-inspector.default", "Default", &[json_f64_value(default)], "default"));
            }
            if question.kind == "slider" {
                if let Some(unit) = &question.unit {
                    fields.push(inspector_text_field(question_ids, "forms-play-inspector.unit", "Unit", &[unit.clone()], "unit"));
                }
            }
        }
        "boolean" => {
            if let Some(default) = &question.default {
                let pressed = default.as_bool().unwrap_or(false);
                fields.push(UiNode::Field(UiFieldNode {
                    id: "forms-play-inspector.default".into(),
                    label: "Default".into(),
                    child: UiControlNode::Toggle(UiToggleNode {
                        id: "forms-play-inspector.default.toggle".into(),
                        icon_id: "toggle-left".into(),
                        pressed,
                        text: Some(if pressed { "Yes".into() } else { "No".into() }),
                        on_change: inspector_patch(question_ids, "default"),
                    }),
                }));
            }
        }
        "single" | "multi" => {
            if let Some(options) = &question.options {
                for option in options {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: format!("forms-play-inspector.option.{}", option.value),
                        label: format!("Option {}", option.value),
                        child: UiControlNode::Input(UiInputNode {
                            id: format!("forms-play-inspector.option.{}.input", option.value),
                            input_kind: "text".into(),
                            value: option.label.clone(),
                            placeholder: None,
                            commit: None,
                            on_change: forms_cmd(
                                "patchQuestionOptions",
                                Some(json!({ "questionIds": question_ids, "optionValue": option.value, "field": "label" })),
                            ),
                        }),
                    }));
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(format!("forms-play-inspector.option.{}.remove", option.value)),
                        icon_id: "trash-2".into(),
                        label: "Remove Option".into(),
                        command: forms_cmd(
                            "removeQuestionOption",
                            Some(json!({ "questionId": question.id, "optionValue": option.value })),
                        ),
                        style: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some("forms-play-inspector.option.add".into()),
                icon_id: "plus".into(),
                label: "Add Option".into(),
                command: forms_cmd("addQuestionOption", Some(json!({ "questionId": question.id, "label": "New option" }))),
                style: None,
            }));
        }
        "date" | "color" => {
            if let Some(default) = &question.default {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.default", "Default", &[json_string_value(default)], "default"));
            }
        }
        "vector" => {
            if let Some(schema) = &question.schema {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.schema", "Schema", &[schema.clone()], "schema"));
            }
            if let Some(step) = question.step {
                fields.push(inspector_number_field(question_ids, "forms-play-inspector.step", "Step", &[step], "step"));
            }
            if let Some(vector_fields) = &question.fields {
                for field in vector_fields {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: format!("forms-play-inspector.vector.{}.label", field.key),
                        label: format!("{} label", field.key),
                        child: UiControlNode::Input(UiInputNode {
                            id: format!("forms-play-inspector.vector.{}.label.input", field.key),
                            input_kind: "text".into(),
                            value: field.label.clone().unwrap_or_else(|| field.key.clone()),
                            placeholder: None,
                            commit: None,
                            on_change: forms_cmd(
                                "patchVectorField",
                                Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "label" })),
                            ),
                        }),
                    }));
                    if let Some(value) = field.value {
                        fields.push(UiNode::Field(UiFieldNode {
                            id: format!("forms-play-inspector.vector.{}.value", field.key),
                            label: format!("{} value", field.key),
                            child: UiControlNode::NumberStepper(UiNumberStepperNode {
                                id: format!("forms-play-inspector.vector.{}.value.stepper", field.key),
                                value,
                                step: question.step.unwrap_or(0.1),
                                uniform: true,
                                on_absolute: forms_cmd(
                                    "patchVectorField",
                                    Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" })),
                                ),
                                on_delta: forms_cmd(
                                    "patchVectorField",
                                    Some(json!({ "questionId": question.id, "fieldKey": field.key, "field": "value" })),
                                ),
                            }),
                        }));
                    }
                    fields.push(UiNode::Button(UiButtonNode {
                        id: Some(format!("forms-play-inspector.vector.{}.remove", field.key)),
                        icon_id: "trash-2".into(),
                        label: format!("Remove {}", field.key),
                        command: forms_cmd(
                            "removeVectorField",
                            Some(json!({ "questionId": question.id, "fieldKey": field.key })),
                        ),
                        style: None,
                    }));
                }
            }
            fields.push(UiNode::Button(UiButtonNode {
                id: Some("forms-play-inspector.vector.add".into()),
                icon_id: "plus".into(),
                label: "Add Vector Field".into(),
                command: forms_cmd(
                    "addVectorField",
                    Some(json!({ "questionId": question.id, "fieldKey": "field" })),
                ),
                style: None,
            }));
        }
        "note" => {
            if let Some(text) = &question.text {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.text", "Text", &[text.clone()], "text"));
            }
        }
        "image" => {
            if let Some(src) = &question.src {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.src", "Src", &[src.clone()], "src"));
            } else {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.src", "Src", &[String::new()], "src"));
            }
        }
        "file" => {
            if let Some(accept) = &question.accept {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.accept", "Accept", &[accept.clone()], "accept"));
            } else {
                fields.push(inspector_text_field(question_ids, "forms-play-inspector.accept", "Accept", &[String::new()], "accept"));
            }
        }
        "buildingComponent" => {
            if let Some(slug) = &question.fixture_slug {
                fields.push(ui_inspector_readonly_field("forms-play-inspector.fixtureSlug", "Fixture Slug", slug));
            }
            if let Some(params) = question.params.as_ref().and_then(|value| value.as_object()) {
                for (param_key, param_value) in params {
                    fields.push(UiNode::Field(UiFieldNode {
                        id: format!("forms-play-inspector.param.{param_key}"),
                        label: param_key.clone(),
                        child: UiControlNode::NumberStepper(UiNumberStepperNode {
                            id: format!("forms-play-inspector.param.{param_key}.stepper"),
                            value: json_f64_value(param_value),
                            step: 0.1,
                            uniform: true,
                            on_absolute: forms_cmd(
                                "patchQuestions",
                                Some(json!({ "questionIds": question_ids, "field": "param", "paramKey": param_key })),
                            ),
                            on_delta: forms_cmd(
                                "patchQuestions",
                                Some(json!({ "questionIds": question_ids, "field": "param", "paramKey": param_key })),
                            ),
                        }),
                    }));
                }
            }
        }
        _ => {}
    }
    fields
}

fn build_inspector_tree(spec: &FormSpec, play: &FormsPlayEnvelope) -> UiNode {
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
    let kind_items: Vec<UiSelectItem> = catalogue_kinds()
        .into_iter()
        .map(|(kind, label, _)| UiSelectItem {
            value: kind.into(),
            label: label.into(),
        })
        .collect();
    let mut base_fields = vec![
        inspector_text_field(&question_ids, "forms-play-inspector.label", "Label", &labels, "label"),
        UiNode::Field(UiFieldNode {
            id: "forms-play-inspector.kind".into(),
            label: "Kind".into(),
            child: UiControlNode::Select(UiSelectNode {
                id: "forms-play-inspector.kind.select".into(),
                value: kind_mixed.value,
                placeholder: kind_mixed.placeholder,
                items: kind_items,
                on_change: inspector_patch(&question_ids, "kind"),
            }),
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
            child: UiControlNode::Toggle(UiToggleNode {
                id: "forms-play-inspector.required.toggle".into(),
                icon_id: "check".into(),
                pressed: required_mixed.uniform && required_mixed.pressed,
                text: if required_mixed.uniform {
                    Some(if required_mixed.pressed { "Yes".into() } else { "No".into() })
                } else {
                    Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
                on_change: inspector_patch(&question_ids, "required"),
            }),
        }),
    ];
    if questions.len() == 1 {
        base_fields.extend(inspector_kind_fields(&questions[0], &question_ids));
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
        serde_json::to_string(&default_envelope()).expect("forms envelope json")
    }

    fn handle_command_patch_ops(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match command {
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
                return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
            }
            "addQuestionOption" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let label = args
                    .and_then(|value| value.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("New option");
                if !question_id.is_empty() {
                    add_question_option(&mut play, &mut store, question_id, label);
                    return apply_store_command(&mut play, &mut store);
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
                    return apply_store_command(&mut play, &mut store);
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
                    return apply_store_command(&mut play, &mut store);
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
                    return apply_store_command(&mut play, &mut store);
                }
            }
            "removeVectorField" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                let field_key = args.and_then(|value| value.get("fieldKey")).and_then(|value| value.as_str()).unwrap_or("");
                if !question_id.is_empty() && !field_key.is_empty() {
                    remove_vector_field(&mut play, &mut store, question_id, field_key);
                    return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
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
                        return apply_store_command(&mut play, &mut store);
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
                return apply_store_command(&mut play, &mut store);
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

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let spec = materialized_projection(&play);
        match body_key {
            FORMS_PLAY_BODY_EDIT => render_edit_table(&spec),
            FORMS_PLAY_BODY_TRY => render_try_wizard(&spec, &play),
            FORMS_PLAY_BODY_PREVIEW => render_preview_body(&spec, &play),
            FORMS_PLAY_BODY_DOCUMENT => build_document_tree(&spec, &play.selected_ids),
            FORMS_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            FORMS_PLAY_BODY_INSPECTION => build_inspector_tree(&spec, &play),
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
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FORMS_PLAY_WINDOW_EDIT, "Edit", FORMS_PLAY_BODY_EDIT)
            .window_kind(FORMS_PLAY_WINDOW_TRY, "Try", FORMS_PLAY_BODY_TRY)
            .window_kind(FORMS_PLAY_WINDOW_PREVIEW, "Preview", FORMS_PLAY_BODY_PREVIEW)
            .panel_tab("framework.panel.document", "Document", PanelGroup::Workbench, FORMS_PLAY_BODY_DOCUMENT)
            .panel_tab("framework.panel.catalogue", "Catalogue", PanelGroup::Workbench, FORMS_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", PanelGroup::Details, FORMS_PLAY_BODY_INSPECTION)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[
                    FORMS_PLAY_WINDOW_EDIT.into(),
                    FORMS_PLAY_WINDOW_TRY.into(),
                    FORMS_PLAY_WINDOW_PREVIEW.into(),
                ],
                "row",
                Some(&[40.0, 35.0, 25.0]),
                Some(&["Edit".into(), "Try".into(), "Preview".into()]),
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

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(forms_bundle()));

semio_framework_plugin::plugin_exports!();
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
    fn renders_edit_table() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_EDIT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn document_lists_steps() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-play-document.steps"));
        assert!(json.contains("Inputs"));
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
    fn add_step_command_appends_step() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document)).steps.len();
        let ops = app.handle_command_patch_ops("addStep", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let next = apply_ops(&document, &ops);
        assert_eq!(materialized_projection(&next).steps.len(), before + 1);
    }

    #[test]
    fn add_question_command_appends_question() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops(
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
        let ops = app.handle_command_patch_ops(
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
        let ops = app.handle_command_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "onboarding" })),
            &document,
            &ViewState::default(),
        );
        let play = apply_ops(&document, &ops);
        assert_eq!(play.current_step_index, 0);
        let next_ops = app.handle_command_patch_ops("nextStep", None, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let next = apply_ops(&document, &next_ops);
        assert_eq!(next.current_step_index, 1);
        let back_ops = app.handle_command_patch_ops("previousStep", None, &serde_json::to_string(&next).unwrap(), &ViewState::default());
        let back = apply_ops(&document, &back_ops);
        assert_eq!(back.current_step_index, 0);
    }

    #[test]
    fn conditional_visibility_hides_team_size() {
        let mut app = FormsPlayApp;
        let ops = app.handle_command_patch_ops(
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
        let document = app.handle_command_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "default" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &document);
        let spec = materialized_projection(&play);
        let name_id = spec.steps[0].questions[0].id.clone();
        let ops = app.handle_command_patch_ops(
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
    fn preview_mesh_json_non_empty_for_building_component() {
        let mut app = FormsPlayApp;
        let ops = app.handle_command_patch_ops(
            "setActiveExample",
            Some(&json!({ "exampleId": "building-component" })),
            &app.initial_document_json(),
            &ViewState::default(),
        );
        let play = apply_ops(&app.initial_document_json(), &ops);
        let spec = materialized_projection(&play);
        let node = app.render(FORMS_PLAY_BODY_PREVIEW, &serde_json::to_string(&play).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world3d"), "preview should render world3d scene");
        let question = first_building_component_question(&spec).expect("building component question");
        let values = effective_try_values(&spec, &play);
        let params = building_component_params(question, &values);
        let fixture: FlowFixture = serde_json::from_str(HEX_COLUMN_FIXTURE_JSON).expect("fixture json");
        let (meshes_json, _) = evaluated_preview_payload(&fixture, &params);
        assert!(!meshes_json.is_empty());
        assert_ne!(meshes_json, "[]");
    }

    #[test]
    fn renders_try_wizard() {
        let mut app = FormsPlayApp;
        let ops = app.handle_command_patch_ops(
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
