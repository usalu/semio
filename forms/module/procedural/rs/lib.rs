//! 🧩 Forms procedural question-kind module — flow-backed building component params + live 3D preview.

use flow_core::{forms_bridge::flow_fixture_to_form_spec, FlowFixture, FlowHost, Widget};
use flow_module_brep::tessellate_geometry_json;
use forms::{visible_questions, FormQuestion};
use semio_framework_core::mesh_from_indexed;
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, App,
    CommandDescriptor, Contribution, PluginApp, PluginBundle, SurfaceKind, UiControlNode,
    UiFieldNode, UiInputNode, UiNode, UiSliderNode, UiToggleNode, ViewState, world3d_default_camera,
    world3d_scene, world3d_selection_json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖Constants
const MODULE_PLUGIN_ID: &str = "forms-module-procedural";
const MODULE_APP_ID: &str = "forms-module-procedural";
const BODY_PARAMS: &str = "params";
const BODY_PREVIEW: &str = "preview";
const MODULE_WINDOW_PARAMS: &str = "forms-module-procedural-params";
const MODULE_WINDOW_PREVIEW: &str = "forms-module-procedural-preview";
const PREVIEW_SURFACE: &str = "forms.module.procedural.preview";
const PREVIEW_FALLBACK_MESH_KIND: &str = "box";
const HEX_COLUMN_FIXTURE_JSON: &str =
    include_str!("../../../../procedural/3d/example/hexagonal-mushroom-column.procedural.json");
//#endregion 🔖Constants

//#region 🔖Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModuleRenderPayload {
    #[serde(default)]
    fixture_slug: String,
    #[serde(default)]
    params: Value,
    #[serde(default)]
    question_id: String,
    #[serde(default)]
    controller_id: String,
    #[serde(default)]
    surface: String,
    #[serde(default)]
    interactive: bool,
}

fn parse_payload(document_json: &str) -> ModuleRenderPayload {
    serde_json::from_str(document_json).unwrap_or(ModuleRenderPayload {
        fixture_slug: "hexagonal-mushroom-column".into(),
        params: json!({}),
        question_id: String::new(),
        controller_id: String::new(),
        surface: "try".into(),
        interactive: true,
    })
}

fn fixture_json_for_slug(slug: &str) -> Option<&'static str> {
    match slug {
        "hexagonal-mushroom-column" => Some(HEX_COLUMN_FIXTURE_JSON),
        _ => None,
    }
}

fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
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

fn module_cmd(payload: &ModuleRenderPayload, command: &str, args: Value) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: payload.controller_id.clone(),
        command: command.into(),
        args: Some(args),
    }
}
//#endregion 🔖Payload

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
        let fallback = json!([{ "id": PREVIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(PREVIEW_FALLBACK_MESH_KIND) }]);
        let fallback_instances = json!([{
            "id": "preview",
            "meshId": PREVIEW_FALLBACK_MESH_KIND,
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

fn render_preview_body(payload: &ModuleRenderPayload) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() {
        "hexagonal-mushroom-column"
    } else {
        payload.fixture_slug.as_str()
    };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let params = payload.params.clone();
    let (meshes_json, instances_json) = evaluated_preview_payload(&fixture, &params);
    build_world_3d_scene(
        PREVIEW_SURFACE,
        MODULE_APP_ID,
        world3d_scene(
            world3d_default_camera(),
            meshes_json,
            instances_json,
            world3d_selection_json("single", &[], None),
        ),
    )
}
//#endregion 🔖Preview

//#region 🔖Params
fn render_question_control(
    question: &FormQuestion,
    value: &Value,
    payload: &ModuleRenderPayload,
) -> UiNode {
    let key = &question.id;
    let patch_field = if payload.surface == "blueprint" {
        "param"
    } else {
        "tryParam"
    };
    let patch_cmd = |param_key: &str| {
        module_cmd(
            payload,
            if payload.surface == "blueprint" {
                "patchQuestions"
            } else {
                "setTryValue"
            },
            json!({
                "questionIds": [payload.question_id],
                "field": patch_field,
                "paramKey": param_key,
                "key": payload.question_id,
            }),
        )
    };
    match question.kind.as_str() {
        "text" | "longText" => UiNode::Field(UiFieldNode {
            id: format!("forms-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("forms-module.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: patch_cmd(key),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        }),
        "number" => UiNode::Field(UiFieldNode {
            id: format!("forms-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("forms-module.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: patch_cmd(key),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
        }),
        "slider" => UiNode::Field(UiFieldNode {
            id: format!("forms-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Slider(UiSliderNode {
                id: format!("forms-module.{key}.slider"),
                value: json_f64_value(value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: patch_cmd(key),
                unit: None,
            })),
            description: None,
            required: None,
            error: None,
        }),
        "boolean" => UiNode::Field(UiFieldNode {
            id: format!("forms-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: format!("forms-module.{key}.toggle"),
                icon_id: "check".into(),
                pressed: value.as_bool().unwrap_or(false),
                text: None,
                on_change: patch_cmd(key),
            })),
            description: None,
            required: None,
            error: None,
        }),
        _ => ui_text(format!("Unsupported param kind: {}", question.kind)),
    }
}

fn render_params_body(payload: &ModuleRenderPayload) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() {
        "hexagonal-mushroom-column"
    } else {
        payload.fixture_slug.as_str()
    };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let spec = flow_fixture_to_form_spec(&fixture);
    let values: Map<String, Value> = payload
        .params
        .as_object()
        .cloned()
        .unwrap_or_default();
    let step = spec.steps.first();
    let Some(step) = step else {
        return ui_text("No flow inputs.");
    };
    let visible = visible_questions(step, &values);
    let mut children: Vec<UiNode> = visible
        .iter()
        .map(|question| {
            let value = values
                .get(&question.id)
                .cloned()
                .unwrap_or_else(|| json!(0));
            render_question_control(question, &value, payload)
        })
        .collect();
    if children.is_empty() {
        children.push(ui_text("No procedural parameters."));
    }
    ui_stack_vertical(children)
}

fn render_flow3d_question(payload: &ModuleRenderPayload) -> UiNode {
    ui_stack_vertical(vec![
        render_params_body(payload),
        render_preview_body(payload),
    ])
}
//#endregion 🔖Params

//#region 🔖App
struct ModuleApp;

impl PluginApp for ModuleApp {
    fn app_id(&self) -> &str {
        MODULE_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }),
            question_id: String::new(),
            controller_id: String::new(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap_or_else(|_| "{}".into())
    }

    fn handle_command_patch_ops(
        &mut self,
        _command: &str,
        _args: Option<&Value>,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let payload = parse_payload(document_json);
        match body_key {
            BODY_PARAMS => render_params_body(&payload),
            BODY_PREVIEW => render_preview_body(&payload),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn create_module_app() -> App {
    App::from_builder(
        App::builder(MODULE_APP_ID, "Forms Module Procedural")
            .document(["semio", "forms"])
            .window_kind(MODULE_WINDOW_PARAMS, "Params", BODY_PARAMS, SurfaceKind::NodeGraph)
            .window_kind(MODULE_WINDOW_PREVIEW, "Preview", BODY_PREVIEW, SurfaceKind::World3d)
            .default_layout(create_default_layout(
                &[MODULE_WINDOW_PARAMS.into(), MODULE_WINDOW_PREVIEW.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Params".into(), "Preview".into()]),
            )),
    )
}

fn module_bundle() -> PluginBundle {
    PluginBundle::new(MODULE_PLUGIN_ID, "Forms Module Procedural", "0.1.0")
        .contributes(Contribution::FormsQuestionKind {
            app_id: MODULE_APP_ID.into(),
            question_kind: "buildingComponent".into(),
            label: "Building Component".into(),
            icon_id: "building".into(),
            default_value_json: r#"{"height":6,"radius":0.5,"sides":6}"#.into(),
            params_body_key: BODY_PARAMS.into(),
            preview_body_key: BODY_PREVIEW.into(),
        })
        .register_app(create_module_app(), || Box::new(ModuleApp))
}

semio_framework_plugin::plugin_exports!(module_bundle);
//#endregion 🔖App

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::Plugin;

    #[test]
    fn module_app_declares_window_kinds() {
        let app = create_module_app();
        assert_eq!(app.definition.window_kinds.len(), 2);
        assert_eq!(app.definition.window_kinds[0].id, MODULE_WINDOW_PARAMS);
        assert_eq!(app.definition.window_kinds[0].body_key, BODY_PARAMS);
        assert_eq!(app.definition.window_kinds[1].id, MODULE_WINDOW_PREVIEW);
        assert_eq!(app.definition.window_kinds[1].body_key, BODY_PREVIEW);
    }

    #[test]
    fn module_manifest_contributes_building_component() {
        let bundle = module_bundle();
        let manifest = bundle.manifest();
        assert_eq!(manifest.contributions.len(), 1);
        let Contribution::FormsQuestionKind {
            question_kind,
            params_body_key,
            preview_body_key,
            ..
        } = &manifest.contributions[0];
        assert_eq!(question_kind, "buildingComponent");
        assert_eq!(params_body_key, BODY_PARAMS);
        assert_eq!(preview_body_key, BODY_PREVIEW);
    }

    #[test]
    fn preview_body_emits_world_scene() {
        let app = ModuleApp;
        let document = serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap();
        let node = app.render(BODY_PREVIEW, &document, &ViewState::default());
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn params_body_lists_flow_inputs() {
        let app = ModuleApp;
        let document = app.initial_document_json();
        let node = app.render(BODY_PARAMS, &document, &ViewState::default());
        assert!(matches!(node, UiNode::Stack(_)));
    }
}
//#endregion 🧪Tests
