//! 🧩️ Playbook procedural block-kind module — flow-backed building component params + live 3D preview.

use flow_core::{flow_neuron_kind_infos_json, forms_bridge::flow_fixture_to_form_spec, FlowFixture, FlowHost, Widget};
use flow_module_brep::{export_solid_json, import_solid_json, tessellate_geometry};
use playbook::{visible_blocks, PlaybookBlock};
use semio_framework_core::mesh_from_indexed;
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, world3d_default_camera, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App, Contribution, DocumentApp,
    DocumentView, PluginBundle, SurfaceKind, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSliderNode, UiToggleNode, ViewState, WorldSunConfig,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use protocol::{Operation, OperationDiff};

//#region 🔖️Constants
const MODULE_PLUGIN_ID: &str = "playbook-module-procedural";
const MODULE_APP_ID: &str = "playbook-module-procedural";
const MODULE_DOCUMENT_SCHEMA: &str = "playbook.module.procedural.payload";
const BODY_PARAMS: &str = "params";
const BODY_PREVIEW: &str = "preview";
const MODULE_WINDOW_PARAMS: &str = "playbook-module-procedural-params";
const MODULE_WINDOW_PREVIEW: &str = "playbook-module-procedural-preview";
const PREVIEW_SURFACE: &str = "playbook.module.procedural.preview";
const PREVIEW_FALLBACK_MESH_KIND: &str = "box";
const ACTION_EXPORT_SOLID: &str = "exportSolidGeometry";
const ACTION_IMPORT_SOLID: &str = "importSolidGeometry";
const SOLID_MEDIA_FORMATS: [&str; 4] = ["step", "obj", "stl", "glb"];
const SOLID_EXPORT_DEFLECTION: f64 = 0.1;
const SOLID_IMPORT_TOLERANCE: f64 = 0.1;
// 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
// handcrafted DSL (`store::DocumentDsl`) that this module (which parses the content as a raw
// `FlowFixture`, not a `Procedural3dDocument`) doesn't read — inlined the same flow-fixture JSON
// this module actually needs, decoupled from procedural's document format.
const HEX_COLUMN_FIXTURE_JSON: &str = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
//#endregion 🔖️Constants

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the procedural module; one field per label makes every locale combination compile-checked.
struct ModuleLabels {
    no_flow_inputs: &'static str,
    no_procedural_parameters: &'static str,
}

const MODULE_LABELS_NATIVE_EN: ModuleLabels = ModuleLabels { no_flow_inputs: "No flow inputs.", no_procedural_parameters: "No procedural parameters." };
const MODULE_LABELS_NATIVE_DE: ModuleLabels = ModuleLabels { no_flow_inputs: "Keine Flow-Eingaben.", no_procedural_parameters: "Keine prozeduralen Parameter." };

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown/missing locale falls back to native English.
fn module_labels(view_state: &ViewState) -> &'static ModuleLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &MODULE_LABELS_NATIVE_DE
    } else {
        &MODULE_LABELS_NATIVE_EN
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "procmodule")]
struct ModuleRenderPayload {
    #[serde(default)]
    fixture_slug: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch because the key
    /// set is driven entirely by whichever `Widget::InputSlider`/`Widget::Neuron` ids the referenced
    /// `fixture_slug`'s flow graph happens to define (see `apply_flow_params`, which walks `params` as
    /// an arbitrary `key -> f64` map and forwards every entry to `FlowHost::set_slider_value`) — no
    /// fixed schema spans all fixtures, so a typed `dsl::DslDocument` derive doesn't apply here.
    #[serde(default = "default_params_field")]
    #[dsl(value)]
    params: dsl::DslValue,
    #[serde(default)]
    question_id: String,
    #[serde(default)]
    controller_id: String,
    #[serde(default)]
    surface: String,
    #[serde(default)]
    interactive: bool,
}

fn default_params_field() -> dsl::DslValue {
    dsl::DslValue::Null
}

/// 🌱️ The module's default document — the hex-column fixture with its stock procedural params. Used
/// as `DocumentApp::initial_projection`; live slot renders override it with the forms-supplied payload.
fn default_payload() -> ModuleRenderPayload {
    ModuleRenderPayload {
        fixture_slug: "hexagonal-mushroom-column".into(),
        params: dsl::to_dsl_value(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })).expect("default params"),
        question_id: String::new(),
        controller_id: String::new(),
        surface: "try".into(),
        interactive: true,
    }
}

fn params_as_json(params: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(params.clone()).unwrap_or(Value::Null)
}

//#region 🔖️DocumentOperation
/// ✏️ Whole-payload replace operation for the procedural block-kind slot document. The module's document is a
/// transient render/params payload (not a collaboratively-edited structure), so its single operation
/// swaps the payload wholesale — export/import stash their results on `params` and re-emit it. The VCS
/// store still records the pre-operation payload as a true inverse, so undo works.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
enum ModulePayloadOperation {
    SetPayload {
        #[dsl(block)]
        payload: ModuleRenderPayload,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModulePayloadDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    payload: Option<ModuleRenderPayload>,
}

impl OperationDiff<ModuleRenderPayload> for ModulePayloadDiff {
    fn apply(&self, projection: &ModuleRenderPayload) -> ModuleRenderPayload {
        self.payload.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.payload.is_some() {
            *self = other;
        }
    }
}

impl Operation<ModuleRenderPayload> for ModulePayloadOperation {
    type Diff = ModulePayloadDiff;

    fn diff(&self, _projection: &ModuleRenderPayload) -> ModulePayloadDiff {
        match self {
            ModulePayloadOperation::SetPayload { payload } => ModulePayloadDiff { payload: Some(payload.clone()) },
        }
    }

    fn backwards(&self, projection: &ModuleRenderPayload) -> Vec<Self> {
        vec![ModulePayloadOperation::SetPayload { payload: projection.clone() }]
    }
}
//#endregion 🔖️DocumentOperation

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
    ActionDescriptor { controller_id: payload.controller_id.clone(), action: action.into(), args: Some(args) }
}
//#endregion 🔖️Payload

//#region 🔖️Preview
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
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
    let positions: Vec<f32> =
        parsed.get("position").or_else(|| parsed.get("positions")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect()).filter(|items: &Vec<f32>| !items.is_empty())?;
    let normals: Vec<f32> = parsed.get("normal").or_else(|| parsed.get("normals")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect()).unwrap_or_default();
    let indices: Vec<u32> =
        parsed.get("index").or_else(|| parsed.get("indices")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_u64().map(|number| number as u32)).collect()).filter(|items: &Vec<u32>| !items.is_empty())?;
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
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
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
            if let Ok(data) = tessellate_geometry(&handle, 0.05) {
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
        return (serde_json::to_string(&fallback).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&fallback_instances).unwrap_or_else(|_| "[]".into()));
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

fn render_preview_body(payload: &ModuleRenderPayload) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let params = params_as_json(&payload.params);
    let (meshes_json, instances_json) = evaluated_preview_payload(&fixture, &params);
    build_world_3d_scene(PREVIEW_SURFACE, MODULE_APP_ID, world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("single", &[], None), &WorldSunConfig::default()))
}
//#endregion 🔖️Preview

//#region 🔖️MediaExport
/// 🧵️ Collects every distinct brep geometry handle exposed by the fixture's preview-flagged widgets, evaluated against the current param overrides — same eval pass as `evaluated_preview_payload`, minus the tessellation step.
fn evaluated_preview_geometry_handles(fixture: &FlowFixture, params: &Value) -> Vec<String> {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
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

/// 📤️ Handles `ACTION_EXPORT_SOLID`: re-evaluates the active fixture, exports every preview geometry handle through `flow_module_brep`'s STEP/OBJ/STL kernel codecs (GLB bridges through mesh tessellation), and stashes the JSON result on `params.__solidExport` for the host shell to read back.
fn handle_export_solid(payload: &mut ModuleRenderPayload, args: Option<&Value>) {
    let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("obj");
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return;
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let handles = evaluated_preview_geometry_handles(&fixture, &params_as_json(&payload.params));
    let result_json = if handles.is_empty() { json!({ "error": "no procedural solid geometry to export" }) } else { serde_json::from_str(&export_solid_json(&handles, format, SOLID_EXPORT_DEFLECTION)).unwrap_or(json!({ "error": "export failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidExport".into(), result_json);
    payload.params = dsl::to_dsl_value(&object).expect("params object");
}

/// 📥️ Handles `ACTION_IMPORT_SOLID`: imports `args.data` (UTF-8 text for STEP/OBJ, base64 for STL/GLB) as `args.format` through `flow_module_brep`'s in-process kernel (GLB bridges through mesh tessellation into an OBJ ingestion) and stashes the resulting geometry handles on `params.__solidImport`.
fn handle_import_solid(payload: &mut ModuleRenderPayload, args: Option<&Value>) {
    let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("obj");
    let data = args.and_then(|value| value.get("data")).and_then(|value| value.as_str()).unwrap_or("");
    let result_json = if data.is_empty() { json!({ "error": "no import data provided" }) } else { serde_json::from_str(&import_solid_json(format, data, SOLID_IMPORT_TOLERANCE)).unwrap_or(json!({ "error": "import failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidImport".into(), result_json);
    payload.params = dsl::to_dsl_value(&object).expect("params object");
}

fn export_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("playbook-module.export.{format}")),
        icon_id: "export".into(),
        label: format!("Export {}", format.to_uppercase()),
        action: module_action(payload, ACTION_EXPORT_SOLID, json!({ "format": format })),
        style: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn import_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("playbook-module.import.{format}")),
        icon_id: "import".into(),
        label: format!("Import {}", format.to_uppercase()),
        action: module_action(payload, ACTION_IMPORT_SOLID, json!({ "format": format })),
        style: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 🎛️ One export + import button pair per solid interchange format, wired to `ACTION_EXPORT_SOLID`/`ACTION_IMPORT_SOLID` question-type actions.
fn render_media_export_buttons(payload: &ModuleRenderPayload) -> Vec<UiNode> {
    let mut buttons: Vec<UiNode> = Vec::new();
    for format in SOLID_MEDIA_FORMATS {
        buttons.push(export_solid_button(payload, format));
        buttons.push(import_solid_button(payload, format));
    }
    buttons
}
//#endregion 🔖️MediaExport

//#region 🔖️Params
fn render_question_control(question: &PlaybookBlock, value: &Value, payload: &ModuleRenderPayload) -> UiNode {
    let key = &question.id;
    let patch_field = if payload.surface == "blueprint" { "param" } else { "tryParam" };
    let patch_cmd = |param_key: &str| {
        module_action(
            payload,
            if payload.surface == "blueprint" { "patchQuestions" } else { "setTryValue" },
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
            id: format!("playbook-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("playbook-module.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: patch_cmd(key),
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
        }),
        "number" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("playbook-module.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: patch_cmd(key),
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
        }),
        "slider" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Slider(UiSliderNode {
                id: format!("playbook-module.{key}.slider"),
                value: json_f64_value(value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: patch_cmd(key),
                unit: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        "boolean" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: question.label.clone(),
            child: Box::new(UiNode::Toggle(UiToggleNode { id: format!("playbook-module.{key}.toggle"), icon_id: "check".into(), text: None, on_change: patch_cmd(key), presence: UiPresence::selected(value.as_bool().unwrap_or(false)),
        menu: None,
    })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        _ => ui_text(format!("Unsupported param kind: {}", question.kind)),
    }
}

fn render_params_body(payload: &ModuleRenderPayload, labels: &ModuleLabels) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let spec = flow_fixture_to_form_spec(&fixture);
    let values: Map<String, Value> = params_as_json(&payload.params).as_object().cloned().unwrap_or_default();
    let step = spec.steps.first();
    let Some(step) = step else {
        return ui_text(labels.no_flow_inputs.to_string());
    };
    let visible = visible_blocks(step, &values);
    let mut children: Vec<UiNode> = visible
        .iter()
        .map(|question| {
            let value = values.get(&question.id).cloned().unwrap_or_else(|| json!(0));
            render_question_control(question, &value, payload)
        })
        .collect();
    if children.is_empty() {
        children.push(ui_text(labels.no_procedural_parameters.to_string()));
    }
    children.extend(render_media_export_buttons(payload));
    ui_stack_vertical(children)
}
//#endregion 🔖️Params

//#region 🔖️App
#[derive(Default)]
struct ModuleApp;

impl DocumentApp for ModuleApp {
    type Projection = ModuleRenderPayload;
    type Operation = ModulePayloadOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        MODULE_APP_ID
    }

    fn document_schema(&self) -> &str {
        MODULE_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> ModuleRenderPayload {
        default_payload()
    }

    fn handle_action(&self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, ModuleRenderPayload>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, _view_state: &ViewState) -> ActionEmit<ModulePayloadOperation> {
        match action {
            ACTION_EXPORT_SOLID => {
                let mut payload = doc.projection.clone();
                handle_export_solid(&mut payload, args);
                ActionEmit::operations(vec![ModulePayloadOperation::SetPayload { payload }])
            }
            ACTION_IMPORT_SOLID => {
                let mut payload = doc.projection.clone();
                handle_import_solid(&mut payload, args);
                ActionEmit::operations(vec![ModulePayloadOperation::SetPayload { payload }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, ModuleRenderPayload>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, view_state: &ViewState) -> UiNode {
        let labels = module_labels(view_state);
        match body_key {
            BODY_PARAMS => render_params_body(doc.projection, labels),
            BODY_PREVIEW => render_preview_body(doc.projection),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn create_module_app() -> App {
    App::from_builder(
        App::builder(MODULE_APP_ID, "Playbook Module Procedural")
            .document(["semio", "forms"])
            .mode("edit", "Edit", "pencil")
            .window_kind(MODULE_WINDOW_PARAMS, "Params", BODY_PARAMS, SurfaceKind::NodeGraph, "clipboard-list")
            .window_kind(MODULE_WINDOW_PREVIEW, "Preview", BODY_PREVIEW, SurfaceKind::World3d, "preview")
            .default_layout(create_default_layout(
                &[MODULE_WINDOW_PARAMS.into(), MODULE_WINDOW_PREVIEW.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Params".into(), "Preview".into()]),
            ))
            // 🔧️ Whole-payload import/export of the block's solid geometry — legitimate coarse-grained
            // operations for this non-collaborative render slot (not the deleted framework `setDocument`).
            .operation(ACTION_EXPORT_SOLID, "Export Solid")
            .operation(ACTION_IMPORT_SOLID, "Import Solid")
            // 📝️ Only the interchange `format` is a user-facing panel choice; the import `data` payload
            // arrives through the host file-open callback, so it is deliberately not a declared arg.
            .action_args(ACTION_EXPORT_SOLID, vec![solid_format_arg()])
            .action_args(ACTION_IMPORT_SOLID, vec![solid_format_arg()]),
    )
}

/// 🎛️ The shared `format` Select over the solid interchange formats, defaulting to OBJ (the handlers' default).
fn solid_format_arg() -> ActionArgDef {
    ActionArgDef::select("format", "Format", SOLID_MEDIA_FORMATS.iter().map(|format| ActionArgOption::new(*format, format.to_uppercase())).collect()).default_value("obj")
}

/// 🗂️ Registers `ModuleRenderPayload`'s pack<->dsl codec under its real `document_schema()` string
/// so `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// this module's render payload without depending on this crate's concrete `Projection`/`Operation`
/// types.
fn register_module_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<ModuleApp>(MODULE_DOCUMENT_SCHEMA);
}

fn module_bundle() -> PluginBundle {
    register_module_exports();
    PluginBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", "0.1.0")
        .contributes(Contribution::PlaybookBlockKind {
            app_id: MODULE_APP_ID.into(),
            block_kind: "buildingComponent".into(),
            label: "Building Component".into(),
            icon_id: "building".into(),
            default_value_json: r#"{"height":6,"radius":0.5,"sides":6}"#.into(),
            params_body_key: BODY_PARAMS.into(),
            preview_body_key: BODY_PREVIEW.into(),
        })
        .register_document_app(create_module_app(), ModuleApp::default)
}

semio_framework_plugin::plugin_exports!(module_bundle);
//#endregion 🔖️App

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, Plugin, PluginApp, VcsDocumentApp};

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<ModuleApp> {
        VcsDocumentApp::new(ModuleApp)
    }

    fn payload_json(params: Value) -> String {
        serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: dsl::to_dsl_value(&params).expect("params"),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap()
    }

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
        let Contribution::PlaybookBlockKind { block_kind, params_body_key, preview_body_key, .. } = &manifest.contributions[0] else {
            panic!("expected a PlaybookBlockKind contribution");
        };
        assert_eq!(block_kind, "buildingComponent");
        assert_eq!(params_body_key, BODY_PARAMS);
        assert_eq!(preview_body_key, BODY_PREVIEW);
    }

    #[test]
    fn preview_body_emits_world_scene() {
        let mut app = new_app();
        let document = payload_json(json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
        let node = app.render(BODY_PREVIEW, Some(&document), &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn params_body_lists_flow_inputs() {
        let mut app = new_app();
        let node = app.render(BODY_PARAMS, None, &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::Stack(_)));
    }

    #[test]
    fn params_body_includes_media_export_buttons() {
        let mut app = new_app();
        let node = app.render(BODY_PARAMS, None, &ViewState::default()).expect("render");
        let UiNode::Stack(stack) = node else {
            panic!("expected a stack node");
        };
        let button_count = stack.children.iter().filter(|child| matches!(child, UiNode::Button(_))).count();
        assert_eq!(button_count, SOLID_MEDIA_FORMATS.len() * 2);
    }

    #[test]
    fn export_solid_action_stashes_result_and_is_undoable() {
        let mut app = new_app();
        assert!(app.projection().expect("projection").params.get("__solidExport").is_none());
        // The export action emits a whole-payload `SetPayload` operation; the store applies it and the
        // stashed result is read back through the materialized projection.
        app.handle_action(ACTION_EXPORT_SOLID, Some(&json!({ "format": "obj" })), &ViewState::default(), &meta()).expect("export");
        assert!(app.projection().expect("projection").params.get("__solidExport").is_some(), "export result stashed on params via the SetPayload operation");
        // The operation carries a true inverse (the pre-operation payload), so undo removes the stashed result.
        app.handle_action("undo", None, &ViewState::default(), &meta()).expect("undo");
        assert!(app.projection().expect("projection").params.get("__solidExport").is_none(), "undo restores the pre-operation payload");
    }

    #[test]
    fn import_solid_action_stashes_result_on_params() {
        let mut app = new_app();
        app.handle_action(ACTION_IMPORT_SOLID, Some(&json!({ "format": "obj", "data": "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n" })), &ViewState::default(), &meta()).expect("import");
        assert!(app.projection().expect("projection").params.get("__solidImport").is_some(), "import result stashed on params via the SetPayload operation");
    }

    #[test]
    fn import_solid_action_reports_error_when_no_data_given() {
        let mut app = new_app();
        app.handle_action(ACTION_IMPORT_SOLID, Some(&json!({ "format": "obj" })), &ViewState::default(), &meta()).expect("import");
        let payload = app.projection().expect("projection");
        let import = payload.params.get("__solidImport").expect("import result present");
        assert!(import.get("error").is_some());
    }

    #[test]
    fn export_solid_declares_only_format_arg_and_materializes_default() {
        use semio_framework_plugin::app::AppActionRegistry;
        let definition = create_module_app().definition;
        let import = definition.actions.iter().find(|action| action.id == ACTION_IMPORT_SOLID).expect("import declared");
        assert!(import.args.iter().all(|arg| arg.id == "format"), "only `format` is a user-facing arg; `data` is file-callback populated");
        let export = definition.actions.iter().find(|action| action.id == ACTION_EXPORT_SOLID).expect("export declared");
        assert_eq!(export.args.len(), 1, "export exposes exactly the format choice");
        let registry = AppActionRegistry::from_definition(&definition);
        let mut app = VcsDocumentApp::with_registry(ModuleApp, registry);
        // exportSolid fired with no args: the declared `format` default is materialized before dispatch,
        // so the whole-payload operation still applies and stashes a result.
        app.handle_action(ACTION_EXPORT_SOLID, None, &ViewState::default(), &meta()).expect("export");
        assert!(app.projection().expect("projection").params.get("__solidExport").is_some(), "export result stashed under the materialized format");
    }

    #[test]
    fn unknown_action_yields_no_document_change() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        app.handle_action("noSuchAction", None, &ViewState::default(), &meta()).expect("noOperation");
        assert_eq!(app.projection().expect("projection"), before);
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

    //#region 🔖️DslAndOpText
    #[test]
    fn module_render_payload_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&default_payload());
        store::test_support::assert_dsl_pack_equivalence(&default_payload());
    }

    #[test]
    fn module_payload_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&ModulePayloadOperation::SetPayload { payload: default_payload() });
    }
    //#endregion 🔖️DslAndOpText
}
//#endregion 🧪️Tests
