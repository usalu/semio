//! 🧱 Procedural 3D plugin — flow-based procedural brep editor bundled as a hot-swappable WASM component.

use flow_core::{
    dag::DagFixture,
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    FlowFixture, FlowHost, Widget,
};
use flow_module_brep::tessellate_geometry_json;
use semio_framework_plugin::{
    build_node_graph_scene, build_world_3d_scene, create_default_layout, create_named_layout,
    export_mesh_glb_bytes, export_mesh_obj, handle_generation_command, merge_world_selection_ids,
    mesh_from_kind, render_generation_form_body, render_generation_preview_text, render_generations_tree,
    selected_generation, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, App, world3d_scene, world3d_selection_json,
    CommandDescriptor, GenerationPlayState, NodeGraphScene, PluginApp, PluginBundle, UiControlNode,
    UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_core::mesh_from_indexed;
use std::collections::HashSet;
use semio_framework_os::{register_os_media_export_handler, OsMediaExportFormat, OsMediaExportResult};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;

//#region 🔖Constants
const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_CONTROLLER_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
const PROCEDURAL_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
const PROCEDURAL_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
const PROCEDURAL_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
const PROCEDURAL_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
const PROCEDURAL_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
const PROCEDURAL_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS: &str = "procedural3d-generations";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural3d-generate-form";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural3d-generate-preview";
const PROCEDURAL_3D_PLAY_BODY_GENERATIONS: &str = "procedural.play.generations";
const PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM: &str = "procedural.play.generate-form";
const PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
const PROCEDURAL_3D_PLAY_SURFACE_GENERATIONS: &str = "procedural.play.generations";
const PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";

const PROCEDURAL_FALLBACK_MESH_KIND: &str = "box";
const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";

const HEX_COLUMN_EXAMPLE_JSON: &str = include_str!("../../example/hexagonal-mushroom-column.procedural.json");
const RECT_EXTRUDE_EXAMPLE_JSON: &str = include_str!("../../example/rectangle-extrude-volume.procedural.json");
const SPHERE_TORUS_EXAMPLE_JSON: &str = include_str!("../../example/sphere-cut-with-torus.procedural.json");

const WIDGET_CATALOG: &[(&str, &str, &str)] = &[
    ("neuron", "Neuron", "cpu"),
    ("inputSlider", "Slider", "sliders-horizontal"),
    ("inputNote", "Note", "file-text"),
    ("outputPreview", "Preview", "eye"),
];
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural3dPreviewCamera {
    #[serde(default = "default_preview_cam_pos")]
    position: [f64; 3],
    #[serde(default = "default_preview_cam_target")]
    target: [f64; 3],
    #[serde(default = "default_preview_fov")]
    fov: f64,
}

impl Default for Procedural3dPreviewCamera {
    fn default() -> Self {
        Self {
            position: default_preview_cam_pos(),
            target: default_preview_cam_target(),
            fov: default_preview_fov(),
        }
    }
}

fn default_preview_cam_pos() -> [f64; 3] {
    [4.0, -4.0, 3.0]
}

fn default_preview_cam_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_preview_fov() -> f64 {
    45.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural3dRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    lod_mode: String,
    #[serde(default = "default_show_mode")]
    show_mode: String,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_node_id: Option<String>,
    #[serde(default)]
    preview_camera: Procedural3dPreviewCamera,
    /// ⏮️ Flow-graph snapshots for undo, pushed before structural edits (add/remove/connect/gumball).
    #[serde(default)]
    undo_fixtures: Vec<FlowFixture>,
    /// ⏭️ Flow-graph snapshots for redo, cleared whenever a new edit is snapshotted.
    #[serde(default)]
    redo_fixtures: Vec<FlowFixture>,
}

impl Default for Procedural3dRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            selection_method: default_selection_method(),
            hovered_node_id: None,
            preview_camera: Procedural3dPreviewCamera::default(),
            undo_fixtures: Vec::new(),
            redo_fixtures: Vec::new(),
        }
    }
}

fn snapshot_procedural3d(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture) {
    runtime.undo_fixtures.push(fixture.clone());
    runtime.redo_fixtures.clear();
}

fn default_show_mode() -> String {
    "solid".into()
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Procedural3dEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: Procedural3dRuntime,
    #[serde(default)]
    generation: GenerationPlayState,
}

fn default_envelope() -> Procedural3dEnvelope {
    envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(|| Procedural3dEnvelope {
        fixture: FlowFixture::default(),
        runtime: Procedural3dRuntime::default(),
        generation: GenerationPlayState::default(),
    })
}

fn envelope_from_fixture_json(json_text: &str) -> Option<Procedural3dEnvelope> {
    serde_json::from_str::<FlowFixture>(json_text).ok().map(|fixture| Procedural3dEnvelope {
        fixture,
        runtime: Procedural3dRuntime::default(),
        generation: GenerationPlayState::default(),
    })
}

fn parse_envelope(document_json: &str) -> Procedural3dEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Procedural3dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn procedural_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: PROCEDURAL_3D_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn preview_camera_json(runtime: &Procedural3dRuntime) -> String {
    semio_framework_core::world3d_camera_json(
        runtime.preview_camera.position,
        runtime.preview_camera.target,
        runtime.preview_camera.fov,
    )
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|ids: &Vec<String>| !ids.is_empty())
        .unwrap_or_else(|| fallback.to_vec())
}

//#region 🔖GumballTransforms
/// 🧭 Maps a gumball drag op to the flow-graph transform neuron kind that persists it.
fn gumball_xform_kind(op: &str) -> &'static str {
    match op {
        "rotate" => "brep.xform.rotate",
        "scale" => "brep.xform.scale",
        _ => "brep.xform.translate",
    }
}

/// 🪪 Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `op`.
fn gumball_widget_id(source_id: &str, op: &str) -> String {
    format!("{source_id}__gumball_{op}")
}

fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<Value> {
    host.fixture
        .widgets
        .iter()
        .find(|widget| widget_id(widget) == widget_id_str)
        .and_then(|widget| serde_json::to_value(widget).ok())
}

fn gumball_widget_offset(host: &FlowHost, widget_id_str: &str) -> [f64; 3] {
    let offset = gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get("offset")).cloned());
    [
        offset.as_ref().and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("z")).and_then(Value::as_f64).unwrap_or(0.0),
    ]
}

fn gumball_widget_number_param(host: &FlowHost, widget_id_str: &str, key: &str, default: f64) -> f64 {
    gumball_widget_json(host, widget_id_str)
        .and_then(|widget_json| widget_json.get("params").and_then(|params| params.get(key)).and_then(|entry| entry.get("value")).and_then(Value::as_f64))
        .unwrap_or(default)
}

fn gumball_translate_params_json(offset: [f64; 3]) -> String {
    json!({ "offset": { "$schema": "vector", "x": offset[0], "y": offset[1], "z": offset[2] } }).to_string()
}

fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
    json!({
        "axis": { "$schema": "vector", "x": axis[0], "y": axis[1], "z": axis[2] },
        "angle": { "$schema": "number", "value": angle },
    })
    .to_string()
}

fn gumball_scale_params_json(factor: f64) -> String {
    json!({
        "factor": { "$schema": "number", "value": factor },
        "center": { "$schema": "point", "x": 0.0, "y": 0.0, "z": 0.0 },
    })
    .to_string()
}

/// 🔀 Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for `op` into the flow graph, rewiring downstream consumers so the transformed geometry is what actually evaluates and exports.
fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, op: &str) -> Result<String, String> {
    let own_suffix = format!("__gumball_{op}");
    if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
        return Ok(selected_id.to_string());
    }
    let transform_id = gumball_widget_id(selected_id, op);
    if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
        return Ok(transform_id);
    }
    let (source_x, source_y) = widget_layout_position(&host.fixture, selected_id);
    let descriptor = json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(op) }).to_string();
    host.add_widget(&descriptor, source_x + 220.0, source_y)?;
    let outgoing_port = host.fixture.synapses.iter().find(|synapse| synapse.from == selected_id).map(|synapse| synapse.from_port.clone());
    if let Some(port) = outgoing_port {
        host.insert_between(selected_id, &port, &transform_id, "geometry", "geometry")?;
    } else {
        host.connect(selected_id, &transform_id)?;
    }
    if let Some(Widget::Neuron { preview, .. }) = host.fixture.widgets.iter_mut().find(|widget| widget_id(widget) == selected_id) {
        *preview = false;
    }
    Ok(transform_id)
}
//#endregion 🔖GumballTransforms

fn host_from_envelope(envelope: &Procedural3dEnvelope) -> FlowHost {
    let mut host = FlowHost::from_fixture(envelope.fixture.clone());
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    host
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<Value> = fixture
        .nodes
        .iter()
        .map(|node| {
            json!({
                "id": node.id,
                "label": if node.name.is_empty() { &node.id } else { &node.name },
                "x": node.x,
                "y": node.y,
                "width": node.width,
                "height": node.height,
            })
        })
        .collect();
    let edges: Vec<Value> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            json!({
                "id": edge.id,
                "sourceNodeId": source_node_id,
                "sourcePortId": source_port_id,
                "targetNodeId": target_node_id,
                "targetPortId": target_port_id,
            })
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

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

fn neuron_mesh_kind(neuron_kind: &str) -> &'static str {
    match neuron_kind {
        "brep.prim3d.sphere" => "sphere",
        "brep.prim3d.cylinder" => "cylinder",
        "brep.prim3d.cone" => "cone",
        "brep.prim3d.torus" => "torus",
        "brep.prim3d.box" => "box",
        "brep.solid.extrude" | "brep.bool.cut" | "brep.bool.fuse" => "box",
        _ => PROCEDURAL_FALLBACK_MESH_KIND,
    }
}

fn widget_preview_mesh_kind(widget: &Widget) -> Option<&'static str> {
    match widget {
        Widget::Neuron { neuronKind, preview, .. } if *preview => Some(neuron_mesh_kind(neuronKind)),
        Widget::OutputPreview { .. } => Some(PROCEDURAL_FALLBACK_MESH_KIND),
        _ => None,
    }
}

fn widget_layout_position(fixture: &FlowFixture, widget_id: &str) -> (f64, f64) {
    fixture
        .layout
        .get(widget_id)
        .map(|layout| (layout.x, layout.y))
        .unwrap_or((0.0, 0.0))
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

fn evaluated_preview_payload(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
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
            let (x, y) = widget_layout_position(fixture, &id);
            let selected = runtime.selected_node_ids.contains(&id);
            let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
            let position = [x * 0.01, -y * 0.01, 0.0];
            instances.push(json!({
                "id": id,
                "meshId": mesh_id,
                "position": position,
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": selected,
                "hovered": hovered,
            }));
        }
    }
    if meshes.is_empty() {
        let fallback = preview_meshes_json_fallback(fixture);
        let fallback_instances = preview_instances_json_fallback(fixture, runtime);
        return (fallback, fallback_instances);
    }
    (
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()),
    )
}

fn evaluate_generation_preview(envelope: &Procedural3dEnvelope, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(&envelope.fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| envelope.fixture.clone());
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.evaluate().unwrap_or_default()
}

fn refresh_generation_preview(envelope: &mut Procedural3dEnvelope) {
    let Some(generation) = selected_generation(&envelope.generation) else {
        envelope.generation.preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(envelope, &generation.values);
    envelope.generation.preview_text = Some(preview);
}

fn generation_preview_payload(envelope: &Procedural3dEnvelope) -> (String, String) {
    let fixture = if let Some(generation) = selected_generation(&envelope.generation) {
        let patched = apply_generation_values_to_fixture(
            &serde_json::to_string(&envelope.fixture).unwrap_or_default(),
            &generation.values,
        );
        FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| envelope.fixture.clone())
    } else {
        envelope.fixture.clone()
    };
    evaluated_preview_payload(&fixture, &envelope.runtime)
}

fn preview_instances_json_fallback(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> String {
    let instances: Vec<Value> = fixture
        .widgets
        .iter()
        .filter_map(|widget| {
            let mesh_kind = widget_preview_mesh_kind(widget)?;
            let id = widget_id(widget).to_string();
            let (x, y) = widget_layout_position(fixture, &id);
            let selected = runtime.selected_node_ids.contains(&id);
            let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
            let position = [x * 0.01, -y * 0.01, 0.0];
            Some(json!({
                "id": id,
                "meshId": mesh_kind,
                "position": position,
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": selected,
                "hovered": hovered,
            }))
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn preview_meshes_json_fallback(fixture: &FlowFixture) -> String {
    let kinds: Vec<String> = fixture
        .widgets
        .iter()
        .filter_map(|widget| widget_preview_mesh_kind(widget).map(str::to_string))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let fallback_kinds = if kinds.is_empty() {
        vec![PROCEDURAL_FALLBACK_MESH_KIND.into()]
    } else {
        kinds
    };
    let meshes: Vec<Value> = fallback_kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn preview_selection_json(runtime: &Procedural3dRuntime) -> String {
    world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_node_ids,
        runtime.hovered_node_id.as_deref(),
    )
}

fn export_mesh_from_envelope(envelope: &Procedural3dEnvelope) -> semio_framework_plugin::MeshData {
    let (meshes_json, _) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
    if let Ok(meshes) = serde_json::from_str::<Vec<Value>>(&meshes_json) {
        if let Some(first) = meshes.first() {
            if let Ok(data) = serde_json::from_value(first.get("data").cloned().unwrap_or(Value::Null)) {
                return data;
            }
        }
    }
    let kind = envelope
        .fixture
        .widgets
        .iter()
        .find_map(|widget| widget_preview_mesh_kind(widget))
        .unwrap_or(PROCEDURAL_FALLBACK_MESH_KIND);
    mesh_from_kind(kind)
}
//#endregion 🔖Document

//#region 🔖Panels
fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: CommandDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command: Some(command),
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_document_tree(fixture: &FlowFixture, selected_node_ids: &[String]) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_command(
                format!("procedural-widget:{id}"),
                id.clone(),
                Some("cpu"),
                procedural_cmd("setSelection", Some(json!({ "ids": [id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural-play-document.widgets".into(),
            label: Some("Widgets".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: Some(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    let items: Vec<UiTreeItemNode> = WIDGET_CATALOG
        .iter()
        .map(|(kind, label, icon)| {
            tree_item_with_command(
                format!("procedural-play-catalogue.{kind}"),
                *label,
                Some(icon),
                procedural_cmd("addWidget", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "procedural-play-catalogue.widgets".into(),
            label: Some("Widgets".into()),
            default_open: Some(true),
            items,
        }],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(fixture: &FlowFixture, selected_node_ids: &[String]) -> UiNode {
    let Some(selected_id) = selected_node_ids.first() else {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {}", fixture.schema)),
            ui_text(format!("Widgets: {}", fixture.widgets.len())),
        ]);
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return ui_text("No selection".to_string());
    };
    let mut fields = vec![ui_inspector_readonly_field(
        "procedural-play-inspector.id",
        "Id",
        widget_id(widget),
    )];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let mixed = ui_inspector_mixed_number(&[*value]);
        fields.push(UiNode::Field(UiFieldNode {
            id: "procedural-play-inspector.value".into(),
            label: "Value".into(),
            child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                id: "procedural-play-inspector.value.input".into(),
                input_kind: "number".into(),
                value: mixed.value.to_string(),
                placeholder: None,
                commit: None,
                on_change: procedural_cmd(
                    "patchFlowWidgets",
                    Some(json!({ "widgetIds": [selected_id], "field": "value" })),
                ),
            }),
        }));
        fields.push(ui_inspector_readonly_field(
            "procedural-play-inspector.range",
            "Range",
            &format!("{min}..{max}"),
        ));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "procedural-play-inspector.widget".into(),
        label: "Widget".into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖Panels

//#region 🔖GenerateRender
fn render_generate_generations(envelope: &Procedural3dEnvelope) -> UiNode {
    render_generations_tree(
        PROCEDURAL_3D_PLAY_APP_ID,
        "procedural3d-play-generate",
        &envelope.generation.generations,
        envelope.generation.selected_generation_id.as_deref(),
    )
}

fn render_generate_form(envelope: &Procedural3dEnvelope) -> UiNode {
    let spec = flow_fixture_to_form_spec(&envelope.fixture);
    let Some(generation) = selected_generation(&envelope.generation) else {
        return ui_text("Add a generation to edit input values.");
    };
    render_generation_form_body(
        &spec,
        &generation.values,
        PROCEDURAL_3D_PLAY_APP_ID,
        "updateGenerationValues",
        &generation.id,
    )
}

fn render_generate_preview(envelope: &Procedural3dEnvelope) -> UiNode {
    let (meshes_json, instances_json) = generation_preview_payload(envelope);
    if meshes_json == "[]" && instances_json == "[]" {
        let text = envelope
            .generation
            .preview_text
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or("(evaluate a generation to preview output)");
        return render_generation_preview_text(
            PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
            PROCEDURAL_3D_PLAY_APP_ID,
            text,
        );
    }
    build_world_3d_scene(
        PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        PROCEDURAL_3D_PLAY_APP_ID,
        world3d_scene(
            preview_camera_json(&envelope.runtime),
            meshes_json,
            instances_json,
            preview_selection_json(&envelope.runtime),
        ),
    )
}
//#endregion 🔖GenerateRender

//#region 🔖Procedural3dPlayApp
struct Procedural3dPlayApp;

impl PluginApp for Procedural3dPlayApp {
    fn app_id(&self) -> &str {
        PROCEDURAL_3D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("procedural3d envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let mut host = host_from_envelope(&envelope);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("exampleId"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    Procedural3dEnvelope {
                        fixture: FlowFixture::default(),
                        runtime: Procedural3dRuntime::default(),
                        generation: GenerationPlayState::default(),
                    }
                } else if example_id == PROCEDURAL_EXAMPLE_HEX_COLUMN || example_id == "demo" {
                    envelope_from_fixture_json(HEX_COLUMN_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else if example_id == PROCEDURAL_EXAMPLE_RECT_EXTRUDE {
                    envelope_from_fixture_json(RECT_EXTRUDE_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else if example_id == PROCEDURAL_EXAMPLE_SPHERE_TORUS {
                    envelope_from_fixture_json(SPHERE_TORUS_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                envelope.runtime.selected_node_ids = node_graph_selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => return Vec::new(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str(viewport_json) {
                        envelope.fixture.camera = camera;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphEdit" => {
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut changed = false;
                for op in ops {
                    match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                    envelope.fixture = fixture;
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            for node_id in envelope.runtime.selected_node_ids.clone() {
                                if !changed {
                                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                                }
                                if host.remove_widget(&node_id).is_ok() {
                                    changed = true;
                                }
                            }
                            if changed {
                                envelope.runtime.selected_node_ids.clear();
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) =
                                (from, from_port, to, to_port)
                            {
                                snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                                if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                    changed = true;
                                } else {
                                    envelope.runtime.undo_fixtures.pop();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    envelope.fixture = host.fixture;
                    return vec![set_document_op(&envelope)];
                }
            }
            "deleteSelection" => {
                for node_id in envelope.runtime.selected_node_ids.clone() {
                    snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                    if host.remove_widget(&node_id).is_ok() {
                        envelope.fixture = host.fixture;
                        envelope.runtime.selected_node_ids.retain(|id| id != &node_id);
                        return vec![set_document_op(&envelope)];
                    }
                    envelope.runtime.undo_fixtures.pop();
                }
            }
            "removeWidget" => {
                let target_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str());
                if let Some(target_id) = target_id {
                    if envelope.fixture.widgets.iter().any(|widget| widget_id(widget) == target_id) {
                        snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                        if host.remove_widget(target_id).is_ok() {
                            envelope.fixture = host.fixture;
                            envelope.runtime.selected_node_ids.retain(|id| id != target_id);
                            return vec![set_document_op(&envelope)];
                        }
                        envelope.runtime.undo_fixtures.pop();
                    }
                }
            }
            "undo" => {
                if let Some(previous) = envelope.runtime.undo_fixtures.pop() {
                    envelope.runtime.redo_fixtures.push(envelope.fixture.clone());
                    envelope.fixture = previous;
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if let Some(next) = envelope.runtime.redo_fixtures.pop() {
                    envelope.runtime.undo_fixtures.push(envelope.fixture.clone());
                    envelope.fixture = next;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.lod_mode = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.show_mode = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if host.move_widget(node_id, x, y).is_ok() {
                        envelope.fixture = host.fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => json!({ "kind": "neuron", "neuronKind": "math.add" }).to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    envelope.fixture = host.fixture;
                    envelope.runtime.selected_node_ids = vec![id];
                    return vec![set_document_op(&envelope)];
                }
                envelope.runtime.undo_fixtures.pop();
            }
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                for widget in envelope.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    if let (Widget::InputSlider { value: ref mut slider_value, .. }, Some(value)) =
                        (widget, raw_value.and_then(|entry| entry.as_f64()))
                    {
                        if field == "value" {
                            *slider_value = value;
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "reorganize" => {
                snapshot_procedural3d(&mut envelope.runtime, &envelope.fixture);
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    envelope.fixture = host.fixture;
                    return vec![set_document_op(&envelope)];
                }
                envelope.runtime.undo_fixtures.pop();
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_node_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_node_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_node_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setSelectionMethod" => {
                let method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.runtime.preview_camera = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let mut new_selection = Vec::new();
                let mut changed = false;
                for id in &ids {
                    let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "translate")) && !id.ends_with("__gumball_translate");
                    if is_new {
                        snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                    }
                    match ensure_gumball_node(&mut host, id, "translate") {
                        Ok(transform_id) => {
                            let current = gumball_widget_offset(&host, &transform_id);
                            let next = [current[0] + dx, current[1] + dy, current[2] + dz];
                            if host.set_neuron_params(&transform_id, &gumball_translate_params_json(next)).is_ok() {
                                new_selection.push(transform_id);
                                changed = true;
                            } else if is_new {
                                envelope.runtime.undo_fixtures.pop();
                            }
                        }
                        Err(_) if is_new => {
                            envelope.runtime.undo_fixtures.pop();
                        }
                        Err(_) => {}
                    }
                }
                if changed {
                    envelope.fixture = host.fixture;
                    envelope.runtime.selected_node_ids = new_selection;
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let mut new_selection = Vec::new();
                let mut changed = false;
                for id in &ids {
                    let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "rotate")) && !id.ends_with("__gumball_rotate");
                    if is_new {
                        snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                    }
                    match ensure_gumball_node(&mut host, id, "rotate") {
                        Ok(transform_id) => {
                            let current_angle = gumball_widget_number_param(&host, &transform_id, "angle", 0.0);
                            let params = gumball_rotate_params_json([ax, ay, az], current_angle + angle);
                            if host.set_neuron_params(&transform_id, &params).is_ok() {
                                new_selection.push(transform_id);
                                changed = true;
                            } else if is_new {
                                envelope.runtime.undo_fixtures.pop();
                            }
                        }
                        Err(_) if is_new => {
                            envelope.runtime.undo_fixtures.pop();
                        }
                        Err(_) => {}
                    }
                }
                if changed {
                    envelope.fixture = host.fixture;
                    envelope.runtime.selected_node_ids = new_selection;
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_node_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let uniform_factor = (sx + sy + sz) / 3.0;
                let mut new_selection = Vec::new();
                let mut changed = false;
                for id in &ids {
                    let is_new = !host.fixture.widgets.iter().any(|widget| widget_id(widget) == gumball_widget_id(id, "scale")) && !id.ends_with("__gumball_scale");
                    if is_new {
                        snapshot_procedural3d(&mut envelope.runtime, &host.fixture);
                    }
                    match ensure_gumball_node(&mut host, id, "scale") {
                        Ok(transform_id) => {
                            let current_factor = gumball_widget_number_param(&host, &transform_id, "factor", 1.0);
                            let params = gumball_scale_params_json(current_factor * uniform_factor);
                            if host.set_neuron_params(&transform_id, &params).is_ok() {
                                new_selection.push(transform_id);
                                changed = true;
                            } else if is_new {
                                envelope.runtime.undo_fixtures.pop();
                            }
                        }
                        Err(_) if is_new => {
                            envelope.runtime.undo_fixtures.pop();
                        }
                        Err(_) => {}
                    }
                }
                if changed {
                    envelope.fixture = host.fixture;
                    envelope.runtime.selected_node_ids = new_selection;
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldPointerDown" | "graphPointerDown" => return Vec::new(),
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(&envelope.fixture);
                if handle_generation_command(command, args, &mut envelope.generation, &spec, PROCEDURAL_3D_PLAY_APP_ID)
                {
                    if matches!(command, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                        refresh_generation_preview(&mut envelope);
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let host = host_from_envelope(&envelope);
        match body_key {
            PROCEDURAL_3D_PLAY_BODY_MAIN => {
                let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
                let viewport_json =
                    serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
                let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
                    None
                } else {
                    serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
                };
                build_node_graph_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    NodeGraphScene {
                        editable: Some(true),
                        selection_json,
                        context_menu_json: Some(
                            r#"[{"id":"delete-selection","label":"Delete selection","command":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
                        ),
                        ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                let (meshes_json, instances_json) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
                build_world_3d_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    world3d_scene(
                        preview_camera_json(&envelope.runtime),
                        meshes_json,
                        instances_json,
                        preview_selection_json(&envelope.runtime),
                    ),
                )
            }
            PROCEDURAL_3D_PLAY_BODY_GENERATIONS => render_generate_generations(&envelope),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => render_generate_form(&envelope),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&envelope),
            PROCEDURAL_3D_PLAY_BODY_DOCUMENT => {
                build_document_tree(&envelope.fixture, &envelope.runtime.selected_node_ids)
            }
            PROCEDURAL_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            PROCEDURAL_3D_PLAY_BODY_INSPECTION => {
                build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids)
            }
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}

fn node_graph_selection_ids(args: Option<&Value>) -> Vec<String> {
    if let Some(ids) = args
        .and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
    {
        return ids;
    }
    selection_ids(args)
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}
//#endregion 🔖Procedural3dPlayApp

//#region 🔖Manifest
fn create_procedural3d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D").document(["semio", "procedural", "3d"])
            .icon_id("workflow")
            .mode("edit", "Edit")
            .mode("generate", "Generate")
            .default_mode_id("edit")
            .window_kind(PROCEDURAL_3D_PLAY_WINDOW_MAIN, "Flow", PROCEDURAL_3D_PLAY_BODY_MAIN)
            .window_kind(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, "Preview", PROCEDURAL_3D_PLAY_BODY_PREVIEW)
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS,
                "Generations",
                PROCEDURAL_3D_PLAY_BODY_GENERATIONS,
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM,
                "Form",
                PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM,
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW,
            )
            .default_layout(create_default_layout(
                &[PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "Preview".into()]),
            ))
            .named_layout(create_named_layout(
                "procedural3d-generate",
                "Generate",
                create_default_layout(
                    &[
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS.into(),
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM.into(),
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                    ],
                    "row",
                    Some(&[22.0, 43.0, 35.0]),
                    Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                ),
                "builtin",
                Some("sparkles".into()),
                None,
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                "workbench",
                PROCEDURAL_3D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                PROCEDURAL_3D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                PROCEDURAL_3D_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", HEX_COLUMN_EXAMPLE_JSON)
    .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", RECT_EXTRUDE_EXAMPLE_JSON)
    .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", SPHERE_TORUS_EXAMPLE_JSON)
    .program("procedural3d", "Procedural 3D", "brep")
}

fn bundle() -> PluginBundle {
    register_procedural3d_exports();
    PluginBundle::new("procedural3d", "Procedural 3D", "0.1.0")
        .register_app(create_procedural3d_app(), || Box::new(Procedural3dPlayApp))
}

fn register_procedural3d_exports() {
    register_os_media_export_handler("3d.procedural", OsMediaExportFormat::Obj, |doc| {
        let envelope: Procedural3dEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (data, mime_type) = export_mesh_obj(&mesh, "procedural");
        Ok(OsMediaExportResult {
            data,
            mime_type,
            file_name: "procedural.obj".into(),
        })
    });
    register_os_media_export_handler("3d.procedural", OsMediaExportFormat::Glb, |doc| {
        let envelope: Procedural3dEnvelope = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
        let mesh = export_mesh_from_envelope(&envelope);
        let (bytes, mime_type) = export_mesh_glb_bytes(&mesh);
        Ok(OsMediaExportResult {
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            mime_type,
            file_name: "procedural.glb".into(),
        })
    });
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::plugin_exports!();
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use kernel_3d_scene::{
        aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3,
    };
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_node_graph_scene() {
        let app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn set_lod_mode_reads_value_arg() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("setLodMode", Some(&json!({ "value": "wireframe" })), &document, &ViewState::default());
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.lod_mode, "wireframe");
    }

    #[test]
    fn set_active_example_loads_sphere_fixture() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &document,
            &ViewState::default(),
        );
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuronKind, .. } if neuronKind == "brep.prim3d.sphere")));
    }

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let envelope = default_envelope();
        let (meshes_json, instances_json) = evaluated_preview_payload(&envelope.fixture, &envelope.runtime);
        assert_ne!(meshes_json, "[]", "meshes_json was empty");
        assert_ne!(instances_json, "[]", "instances_json was empty");
        let meshes: Vec<serde_json::Value> = serde_json::from_str(&meshes_json).expect("meshes json");
        let instances: Vec<serde_json::Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert!(!meshes.is_empty());
        assert!(!instances.is_empty());
        for mesh in &meshes {
            let data: semio_framework_core::MeshData =
                serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            assert!(data.positions.len() >= 9, "mesh has too few positions");
            assert!(data.indices.len() >= 3, "mesh has too few indices");
        }
        let camera = Camera3d {
            position: Vec3::from_array([
                envelope.runtime.preview_camera.position[0] as f32,
                envelope.runtime.preview_camera.position[1] as f32,
                envelope.runtime.preview_camera.position[2] as f32,
            ]),
            target: Vec3::from_array([
                envelope.runtime.preview_camera.target[0] as f32,
                envelope.runtime.preview_camera.target[1] as f32,
                envelope.runtime.preview_camera.target[2] as f32,
            ]),
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: envelope.runtime.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
            near: 0.1,
            far: 1000.0,
        };
        let view_proj = camera.view_proj(0.6);
        let planes = frustum_planes(view_proj);
        let mut visible = 0usize;
        for instance in instances {
            let mesh_id = instance
                .get("meshId")
                .or_else(|| instance.get("mesh_id"))
                .and_then(|value| value.as_str())
                .unwrap_or("box");
            let mesh = meshes
                .iter()
                .find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id))
                .expect("mesh record");
            let data: semio_framework_core::MeshData =
                serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            let mesh3d = Mesh3d::from_buffers(data.positions, data.normals, data.indices);
            let position = instance
                .get("position")
                .and_then(|value| value.as_array())
                .map(|items| {
                    [
                        items[0].as_f64().unwrap_or(0.0) as f32,
                        items[1].as_f64().unwrap_or(0.0) as f32,
                        items[2].as_f64().unwrap_or(0.0) as f32,
                    ]
                })
                .unwrap_or([0.0, 0.0, 0.0]);
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
            let (min, max) = transform_aabb(model, mesh3d.aabb_min, mesh3d.aabb_max);
            if aabb_intersects_frustum(&planes, min, max) {
                visible += 1;
            }
        }
        assert!(visible > 0, "no preview instances intersect camera frustum");
    }

    #[test]
    fn renders_world_preview_scene() {
        let app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
        let parsed: semio_framework_core::UiNode = serde_json::from_str(&json).expect("preview ui json");
        match parsed {
            semio_framework_core::UiNode::ComponentScene(scene) => {
                assert_eq!(scene.component_kind, "world-3d");
                let world = scene.world_3d.expect("world_3d payload");
                assert_ne!(world.meshes_json, "[]");
                assert_ne!(world.instances_json, "[]");
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    #[test]
    fn add_widget_command_appends_widget() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let before = parse_envelope(&document).fixture.widgets.len();
        let ops = app.handle_command("addWidget", Some(&json!({ "kind": "inputNote" })), &document, &ViewState::default());
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.widgets.len() > before);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let generations = app.render(PROCEDURAL_3D_PLAY_BODY_GENERATIONS, &document, &ViewState::default());
        assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
    }

    #[test]
    fn add_generation_evaluates_preview() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addGeneration", None, &document, &ViewState::default());
        let envelope: Procedural3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.generation.generations.len(), 1);
        assert!(envelope.generation.preview_text.as_deref().unwrap_or("").len() > 2);
    }

    #[test]
    fn translate_selection_persists_transform_into_flow_graph() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let before = parse_envelope(&document);
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
        let ops = app.handle_command(
            "translateSelection",
            Some(&json!({ "ids": ["extrude"], "dx": 1.0, "dy": 2.0, "dz": 3.0 })),
            &document,
            &ViewState::default(),
        );
        let envelope = apply_ops(&before, &ops);
        let transform_id = "extrude__gumball_translate";
        let transform = envelope.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
        assert!(matches!(transform, Widget::Neuron { neuronKind, .. } if neuronKind == "brep.xform.translate"));
        let offset = gumball_widget_offset(&host_from_envelope(&envelope), transform_id);
        assert_eq!(offset, [1.0, 2.0, 3.0]);
        let source = envelope.fixture.widgets.iter().find(|widget| widget_id(widget) == "extrude").expect("source widget");
        assert!(matches!(source, Widget::Neuron { preview, .. } if !*preview), "source preview should turn off once gumball-transformed");
        assert!(envelope.fixture.synapses.iter().any(|synapse| synapse.from == transform_id && synapse.to == "column-preview"), "downstream rewired through transform node");
        assert!(!envelope.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"), "old direct edge removed");
        assert_eq!(envelope.runtime.selected_node_ids, vec![transform_id.to_string()]);
        assert_eq!(envelope.runtime.undo_fixtures.len(), 1);

        // Re-grabbing the same transform accumulates the delta instead of creating a second node.
        let document2 = serde_json::to_string(&envelope).unwrap();
        let ops2 = app.handle_command(
            "translateSelection",
            Some(&json!({ "ids": [transform_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
            &document2,
            &ViewState::default(),
        );
        let envelope2 = apply_ops(&envelope, &ops2);
        assert_eq!(envelope2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
        assert_eq!(gumball_widget_offset(&host_from_envelope(&envelope2), transform_id), [2.0, 2.0, 3.0]);
        assert_eq!(envelope2.runtime.undo_fixtures.len(), 1, "re-grab updates in place without an extra undo snapshot");
    }

    #[test]
    fn rotate_and_scale_selection_persist_into_flow_graph() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let rotate_ops = app.handle_command(
            "rotateSelection",
            Some(&json!({ "ids": ["extrude"], "angle": std::f64::consts::FRAC_PI_2 })),
            &document,
            &ViewState::default(),
        );
        let rotated = apply_ops(&envelope, &rotate_ops);
        let rotate_id = "extrude__gumball_rotate";
        assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuronKind, .. } if id == rotate_id && neuronKind == "brep.xform.rotate")));
        assert_eq!(gumball_widget_number_param(&host_from_envelope(&rotated), rotate_id, "angle", 0.0), std::f64::consts::FRAC_PI_2);

        let scale_ops = app.handle_command(
            "scaleSelection",
            Some(&json!({ "ids": ["extrude"], "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
            &document,
            &ViewState::default(),
        );
        let scaled = apply_ops(&envelope, &scale_ops);
        let scale_id = "extrude__gumball_scale";
        assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuronKind, .. } if id == scale_id && neuronKind == "brep.xform.scale")));
        assert_eq!(gumball_widget_number_param(&host_from_envelope(&scaled), scale_id, "factor", 1.0), 2.0);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let before = parse_envelope(&document);
        let add_ops = app.handle_command("addWidget", Some(&json!({ "kind": "inputNote" })), &document, &ViewState::default());
        let after_add = apply_ops(&before, &add_ops);
        assert!(after_add.fixture.widgets.len() > before.fixture.widgets.len());
        assert_eq!(after_add.runtime.undo_fixtures.len(), 1);

        let document_after_add = serde_json::to_string(&after_add).unwrap();
        let undo_ops = app.handle_command("undo", None, &document_after_add, &ViewState::default());
        let after_undo = apply_ops(&after_add, &undo_ops);
        assert_eq!(after_undo.fixture.widgets.len(), before.fixture.widgets.len());
        assert_eq!(after_undo.runtime.undo_fixtures.len(), 0);
        assert_eq!(after_undo.runtime.redo_fixtures.len(), 1);

        let document_after_undo = serde_json::to_string(&after_undo).unwrap();
        let redo_ops = app.handle_command("redo", None, &document_after_undo, &ViewState::default());
        let after_redo = apply_ops(&after_undo, &redo_ops);
        assert_eq!(after_redo.fixture.widgets.len(), after_add.fixture.widgets.len());
        assert_eq!(after_redo.runtime.redo_fixtures.len(), 0);
    }

    #[test]
    fn remove_widget_command_deletes_by_id_and_supports_undo() {
        let mut app = Procedural3dPlayApp;
        let document = app.initial_document_json();
        let before = parse_envelope(&document);
        assert!(before.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
        let ops = app.handle_command("removeWidget", Some(&json!({ "widgetId": "sides" })), &document, &ViewState::default());
        let after = apply_ops(&before, &ops);
        assert!(!after.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
        assert_eq!(after.runtime.undo_fixtures.len(), 1);

        let document_after = serde_json::to_string(&after).unwrap();
        let undo_ops = app.handle_command("undo", None, &document_after, &ViewState::default());
        let restored = apply_ops(&after, &undo_ops);
        assert!(restored.fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
    }

    fn apply_ops(envelope: &Procedural3dEnvelope, ops: &[String]) -> Procedural3dEnvelope {
        let mut next = envelope.clone();
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        next = parsed;
                    }
                }
            }
        }
        next
    }
}
//#endregion 🧪Tests
