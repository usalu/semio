//! 🧩 Protocol procedural block-kind module — flow-backed building component params + live 3D preview.

use flow_core::{forms_bridge::flow_fixture_to_form_spec, FlowFixture, FlowHost, Widget};
use flow_module_brep::{export_solid_json, import_solid_json, tessellate_geometry_json};
use protocol::{visible_blocks, ProtocolBlock};
use semio_framework_core::mesh_from_indexed;
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, App,
    ActionDescriptor, Contribution, PluginApp, PluginBundle, SurfaceKind, UiButtonNode,
    UiFieldNode, UiInputNode, UiNode, UiSliderNode, UiToggleNode, ViewState, world3d_default_camera,
    world3d_scene, world3d_selection_json, WorldSunConfig,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖Constants
const MODULE_PLUGIN_ID: &str = "protocol-module-procedural";
const MODULE_APP_ID: &str = "protocol-module-procedural";
const BODY_PARAMS: &str = "params";
const BODY_PREVIEW: &str = "preview";
const MODULE_WINDOW_PARAMS: &str = "protocol-module-procedural-params";
const MODULE_WINDOW_PREVIEW: &str = "protocol-module-procedural-preview";
const PREVIEW_SURFACE: &str = "protocol.module.procedural.preview";
const PREVIEW_FALLBACK_MESH_KIND: &str = "box";
const ACTION_EXPORT_SOLID: &str = "exportSolidGeometry";
const ACTION_IMPORT_SOLID: &str = "importSolidGeometry";
const SOLID_MEDIA_FORMATS: [&str; 4] = ["step", "obj", "stl", "glb"];
const SOLID_EXPORT_DEFLECTION: f64 = 0.1;
const SOLID_IMPORT_TOLERANCE: f64 = 0.1;
const HEX_COLUMN_FIXTURE_JSON: &str =
    include_str!("../../../../procedural/3d/example/hexagonal-mushroom-column.procedural.json");
//#endregion 🔖Constants

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the procedural module; one field per label makes every locale combination compile-checked.
struct ModuleLabels {
    no_flow_inputs: &'static str,
    no_procedural_parameters: &'static str,
}

const MODULE_LABELS_NATIVE_EN: ModuleLabels =
    ModuleLabels { no_flow_inputs: "No flow inputs.", no_procedural_parameters: "No procedural parameters." };
const MODULE_LABELS_NATIVE_DE: ModuleLabels =
    ModuleLabels { no_flow_inputs: "Keine Flow-Eingaben.", no_procedural_parameters: "Keine prozeduralen Parameter." };

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown/missing locale falls back to native English.
fn module_labels(view_state: &ViewState) -> &'static ModuleLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &MODULE_LABELS_NATIVE_DE
    } else {
        &MODULE_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

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

fn module_action(payload: &ModuleRenderPayload, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: payload.controller_id.clone(),
        action: action.into(),
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
            &WorldSunConfig::default(),
        ),
    )
}
//#endregion 🔖Preview

//#region 🔖MediaExport
/// 🧵 Collects every distinct brep geometry handle exposed by the fixture's preview-flagged widgets, evaluated against the current param overrides — same eval pass as `evaluated_preview_payload`, minus the tessellation step.
fn evaluated_preview_geometry_handles(fixture: &FlowFixture, params: &Value) -> Vec<String> {
    let mut host = FlowHost::from_fixture(fixture.clone());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
    let mut handles: Vec<String> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        if let Some(handle) = geometry_handle_for_widget(&eval, &id) {
            if !handles.contains(&handle) {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📤 Handles `ACTION_EXPORT_SOLID`: re-evaluates the active fixture, exports every preview geometry handle through `flow_module_brep`'s STEP/OBJ/STL kernel codecs (GLB bridges through mesh tessellation), and stashes the JSON result on `params.__solidExport` for the host shell to read back.
fn handle_export_solid(payload: &mut ModuleRenderPayload, args: Option<&Value>) {
    let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("obj");
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return;
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let handles = evaluated_preview_geometry_handles(&fixture, &payload.params);
    let result_json = if handles.is_empty() {
        json!({ "error": "no procedural solid geometry to export" })
    } else {
        serde_json::from_str(&export_solid_json(&handles, format, SOLID_EXPORT_DEFLECTION)).unwrap_or(json!({ "error": "export failed" }))
    };
    let mut params = payload.params.as_object().cloned().unwrap_or_default();
    params.insert("__solidExport".into(), result_json);
    payload.params = Value::Object(params);
}

/// 📥 Handles `ACTION_IMPORT_SOLID`: imports `args.data` (UTF-8 text for STEP/OBJ, base64 for STL/GLB) as `args.format` through `flow_module_brep`'s in-process kernel (GLB bridges through mesh tessellation into an OBJ ingestion) and stashes the resulting geometry handles on `params.__solidImport`.
fn handle_import_solid(payload: &mut ModuleRenderPayload, args: Option<&Value>) {
    let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("obj");
    let data = args.and_then(|value| value.get("data")).and_then(|value| value.as_str()).unwrap_or("");
    let result_json = if data.is_empty() {
        json!({ "error": "no import data provided" })
    } else {
        serde_json::from_str(&import_solid_json(format, data, SOLID_IMPORT_TOLERANCE)).unwrap_or(json!({ "error": "import failed" }))
    };
    let mut params = payload.params.as_object().cloned().unwrap_or_default();
    params.insert("__solidImport".into(), result_json);
    payload.params = Value::Object(params);
}

fn export_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("protocol-module.export.{format}")),
        icon_id: "export".into(),
        label: format!("Export {}", format.to_uppercase()),
        action: module_action(payload, ACTION_EXPORT_SOLID, json!({ "format": format })),
        style: None,
        disabled: None,
    })
}

fn import_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("protocol-module.import.{format}")),
        icon_id: "import".into(),
        label: format!("Import {}", format.to_uppercase()),
        action: module_action(payload, ACTION_IMPORT_SOLID, json!({ "format": format })),
        style: None,
        disabled: None,
    })
}

/// 🎛 One export + import button pair per solid interchange format, wired to `ACTION_EXPORT_SOLID`/`ACTION_IMPORT_SOLID` question-type actions.
fn render_media_export_buttons(payload: &ModuleRenderPayload) -> Vec<UiNode> {
    let mut buttons: Vec<UiNode> = Vec::new();
    for format in SOLID_MEDIA_FORMATS {
        buttons.push(export_solid_button(payload, format));
        buttons.push(import_solid_button(payload, format));
    }
    buttons
}
//#endregion 🔖MediaExport

//#region 🔖Params
fn render_question_control(
    question: &ProtocolBlock,
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
        module_action(
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
            id: format!("protocol-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("protocol-module.{key}.input"),
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
            id: format!("protocol-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("protocol-module.{key}.input"),
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
            id: format!("protocol-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Slider(UiSliderNode {
                id: format!("protocol-module.{key}.slider"),
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
            id: format!("protocol-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Toggle(UiToggleNode {
                id: format!("protocol-module.{key}.toggle"),
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

fn render_params_body(payload: &ModuleRenderPayload, labels: &ModuleLabels) -> UiNode {
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
        return ui_text(labels.no_flow_inputs.to_string());
    };
    let visible = visible_blocks(step, &values);
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
        children.push(ui_text(labels.no_procedural_parameters.to_string()));
    }
    children.extend(render_media_export_buttons(payload));
    ui_stack_vertical(children)
}

fn render_flow3d_question(payload: &ModuleRenderPayload, labels: &ModuleLabels) -> UiNode {
    let mut children = vec![render_params_body(payload, labels)];
    children.extend(render_media_export_buttons(payload));
    children.push(render_preview_body(payload));
    ui_stack_vertical(children)
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

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        match action {
            ACTION_EXPORT_SOLID => {
                let mut payload = parse_payload(document_json);
                handle_export_solid(&mut payload, args);
                vec![serde_json::to_string(&payload).unwrap_or_else(|_| document_json.into())]
            }
            ACTION_IMPORT_SOLID => {
                let mut payload = parse_payload(document_json);
                handle_import_solid(&mut payload, args);
                vec![serde_json::to_string(&payload).unwrap_or_else(|_| document_json.into())]
            }
            _ => Vec::new(),
        }
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let payload = parse_payload(document_json);
        let labels = module_labels(view_state);
        match body_key {
            BODY_PARAMS => render_params_body(&payload, labels),
            BODY_PREVIEW => render_preview_body(&payload),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn create_module_app() -> App {
    App::from_builder(
        App::builder(MODULE_APP_ID, "Protocol Module Procedural")
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
    PluginBundle::new(MODULE_PLUGIN_ID, "Protocol Module Procedural", "0.1.0")
        .contributes(Contribution::ProtocolBlockKind {
            app_id: MODULE_APP_ID.into(),
            block_kind: "buildingComponent".into(),
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
        let Contribution::ProtocolBlockKind {
            block_kind,
            params_body_key,
            preview_body_key,
            ..
        } = &manifest.contributions[0];
        assert_eq!(block_kind, "buildingComponent");
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

    #[test]
    fn params_body_includes_media_export_buttons() {
        let app = ModuleApp;
        let document = app.initial_document_json();
        let node = app.render(BODY_PARAMS, &document, &ViewState::default());
        let UiNode::Stack(stack) = node else {
            panic!("expected a stack node");
        };
        let button_count = stack.children.iter().filter(|child| matches!(child, UiNode::Button(_))).count();
        assert_eq!(button_count, SOLID_MEDIA_FORMATS.len() * 2);
    }

    #[test]
    fn export_solid_action_stashes_result_on_params() {
        let mut app = ModuleApp;
        let document = serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap();
        let ops = app.handle_action_patch_ops(ACTION_EXPORT_SOLID, Some(&json!({ "format": "obj" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let payload: ModuleRenderPayload = serde_json::from_str(&ops[0]).unwrap();
        let export = payload.params.get("__solidExport").expect("export result present");
        assert!(export.get("error").is_none(), "{export:?}");
        assert_eq!(export.get("binary").and_then(|value| value.as_bool()), Some(false));
    }

    #[test]
    fn export_then_import_solid_round_trips_through_actions() {
        let mut app = ModuleApp;
        let document = serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap();
        let export_ops = app.handle_action_patch_ops(ACTION_EXPORT_SOLID, Some(&json!({ "format": "stl" })), &document, &ViewState::default());
        let exported: ModuleRenderPayload = serde_json::from_str(&export_ops[0]).unwrap();
        let export_result = exported.params.get("__solidExport").expect("export result present");
        let data = export_result.get("data").and_then(|value| value.as_str()).expect("export data").to_string();

        let import_ops = app.handle_action_patch_ops(ACTION_IMPORT_SOLID, Some(&json!({ "format": "stl", "data": data })), &document, &ViewState::default());
        assert_eq!(import_ops.len(), 1);
        let imported: ModuleRenderPayload = serde_json::from_str(&import_ops[0]).unwrap();
        let import_result = imported.params.get("__solidImport").expect("import result present");
        assert!(import_result.get("error").is_none(), "{import_result:?}");
        assert_eq!(import_result.get("handles").and_then(|value| value.as_array()).map(|handles| handles.len()), Some(1));
    }

    #[test]
    fn import_solid_action_reports_error_when_no_data_given() {
        let mut app = ModuleApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(ACTION_IMPORT_SOLID, Some(&json!({ "format": "obj" })), &document, &ViewState::default());
        let payload: ModuleRenderPayload = serde_json::from_str(&ops[0]).unwrap();
        let import = payload.params.get("__solidImport").expect("import result present");
        assert!(import.get("error").is_some());
    }

    #[test]
    fn unknown_action_yields_no_patch_ops() {
        let mut app = ModuleApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("noSuchAction", None, &document, &ViewState::default());
        assert!(ops.is_empty());
    }

    #[test]
    fn module_labels_resolve_native_english_by_default() {
        let labels = module_labels(&ViewState::default());
        assert_eq!(labels.no_flow_inputs, "No flow inputs.");
        assert_eq!(labels.no_procedural_parameters, "No procedural parameters.");
        let node = ui_text(labels.no_procedural_parameters.to_string());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No procedural parameters."));
    }

    #[test]
    fn module_labels_resolve_german_locale() {
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let labels = module_labels(&view_state);
        assert_eq!(labels.no_flow_inputs, "Keine Flow-Eingaben.");
        assert_eq!(labels.no_procedural_parameters, "Keine prozeduralen Parameter.");
        let node = ui_text(labels.no_procedural_parameters.to_string());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Keine prozeduralen Parameter."));
        assert!(!json.contains("No procedural parameters."));
    }
}
//#endregion 🧪Tests
