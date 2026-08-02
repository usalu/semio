//! ⚙️ Procedural 3D app — headless compute (constitutional: engine).

use flow_core::dag::DagFixture;
use flow_core::forms_bridge::apply_generation_values_to_fixture;
use flow_core::{CameraJson, FlowEvalDriver, FlowFixture, FlowHost, Widget};
use flow_module_brep::tessellate_geometry;
use playbook::{selected_generation, GenerationPlayState};
use procedural_3d::{widget_id, Procedural3dDocument};
use semio_framework_plugin::SelectionSet;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use store::DocumentDsl;

//#region 🔖️Constants
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";
//#endregion 🔖️Constants

//#region 🔖️EvalCache
/// 🧠️ Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions.
///
/// `Procedural3dPlayView` is a stateless serde value rebuilt from `document_json` on every
/// plugin dispatch, so a fresh `FlowHost::from_fixture` would otherwise discard per-node
/// memoization (and the geometry handle stability that lets `flow_module_brep`'s mesh cache
/// hit) on every single edit. Mirrors `flow_module_brep`'s single-instance `KERNEL`/`MESH_CACHE`
/// `OnceLock` pattern — one shared cache per WASM instance, which matches one editor session.
static PROCEDURAL_NEURAL_CACHE: std::sync::OnceLock<std::sync::Arc<flow_core::neural::NeuralCache>> = std::sync::OnceLock::new();

pub fn procedural_neural_cache() -> std::sync::Arc<flow_core::neural::NeuralCache> {
    PROCEDURAL_NEURAL_CACHE.get_or_init(|| std::sync::Arc::new(flow_core::neural::NeuralCache::new())).clone()
}
//#endregion 🔖️EvalCache

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    #[serde(default = "default_preview_cam_pos")]
    pub position: [f64; 3],
    #[serde(default = "default_preview_cam_target")]
    pub target: [f64; 3],
    #[serde(default = "default_preview_fov")]
    pub fov: f64,
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

pub fn default_preview_cam_pos() -> [f64; 3] {
    [4.0, -4.0, 3.0]
}

pub fn default_preview_cam_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

pub fn default_preview_fov() -> f64 {
    45.0
}

/// 👁️ Ephemeral per-session view state — never part of the persisted document. Selection, hover,
/// graph camera, preview camera, sun/LOD display options, the derived mesh preview caches, and the
/// active generation selection/preview all live here on the app struct, out of the VCS document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dRuntime {
    pub selected_node_ids: SelectionSet,
    pub lod_mode: String,
    pub show_mode: String,
    pub selection_method: String,
    pub hovered_node_id: Option<String>,
    pub camera: CameraJson,
    pub preview_camera: Procedural3dPreviewCamera,
    pub preview_cache: Option<Procedural3dPreviewCache>,
    pub generation_preview_cache: Option<Procedural3dPreviewCache>,
    pub sun: semio_framework_plugin::WorldSunConfig,
    pub selected_generation_id: Option<String>,
    pub generation_preview_text: Option<String>,
    /// 🧵️ Off-main-thread evaluation state — see `FlowEvalDriver`.
    pub eval_driver: FlowEvalDriver,
}

impl Default for Procedural3dRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: SelectionSet::default(),
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            selection_method: default_selection_method(),
            hovered_node_id: None,
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            preview_cache: None,
            generation_preview_cache: None,
            sun: semio_framework_plugin::WorldSunConfig::default(),
            selected_generation_id: None,
            generation_preview_text: None,
            eval_driver: FlowEvalDriver::default(),
        }
    }
}

pub fn default_show_mode() -> String {
    "shaded".into()
}

pub fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCache {
    pub signature: u64,
    pub meshes_json: String,
    pub instances_json: String,
    pub status_json: Option<String>,
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 📄️ The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example fixture (see `procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT`).
pub fn default_projection() -> Procedural3dDocument {
    Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument::default()
}

/// 🧾️ Builds the initial projection for a named example (or the empty/default fixture).
pub fn example_projection(example_id: &str) -> Procedural3dDocument {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT),
        "" => None,
        _ => None,
    };
    dsl.and_then(|text| Procedural3dDocument::parse_dsl(text).ok()).unwrap_or_default()
}

/// 🧾️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    serde_json::to_string(&example_projection(example_id)).unwrap_or_default()
}

pub fn fixture_signature(fixture: &FlowFixture) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    if let Ok(json) = serde_json::to_string(&fixture.widgets) {
        json.hash(&mut hasher);
    }
    if let Ok(json) = serde_json::to_string(&fixture.synapses) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn generation_preview_signature(fixture: &FlowFixture, generation: &GenerationPlayState) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture_signature(fixture).hash(&mut hasher);
    if let Some(selected) = selected_generation(generation) {
        selected.id.hash(&mut hasher);
        if let Ok(json) = serde_json::to_string(&selected.values) {
            json.hash(&mut hasher);
        }
    }
    hasher.finish()
}

pub fn generation_fixture_for(fixture: &FlowFixture, generation: &GenerationPlayState) -> FlowFixture {
    if let Some(selected) = selected_generation(generation) {
        let patched = apply_generation_values_to_fixture(
            &serde_json::to_string(fixture).unwrap_or_default(),
            &selected.values,
        );
        FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone())
    } else {
        fixture.clone()
    }
}

fn preview_cache_signature(fixture: &FlowFixture, eval_json: &str, runtime: &Procedural3dRuntime) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture_signature(fixture).hash(&mut hasher);
    eval_json.hash(&mut hasher);
    runtime.lod_mode.hash(&mut hasher);
    runtime.show_mode.hash(&mut hasher);
    hasher.finish()
}

pub fn preview_tolerance(lod_mode: &str) -> f64 {
    match lod_mode {
        "coarse" => 0.15,
        "fine" => 0.02,
        _ => 0.05,
    }
}

pub fn refresh_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture) {
    let eval_json = runtime.eval_driver.eval_json().to_string();
    let signature = preview_cache_signature(fixture, &eval_json, runtime);
    if runtime.preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
        return;
    }
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, fixture, runtime);
    let status_json = preview_status_json(&eval_json, fixture);
    runtime.preview_cache = Some(Procedural3dPreviewCache {
        signature,
        meshes_json,
        instances_json,
        status_json,
    });
}

pub fn refresh_generation_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = selected_generation(generation) else {
        runtime.generation_preview_cache = None;
        return;
    };
    let eval_json = runtime
        .generation_preview_text
        .clone()
        .unwrap_or_else(|| evaluate_generation_preview(fixture, &selected.values));
    let signature = preview_cache_signature(fixture, &eval_json, runtime);
    if runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
        return;
    }
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, fixture, runtime);
    let status_json = preview_status_json(&eval_json, fixture);
    runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
        signature,
        meshes_json,
        instances_json,
        status_json,
    });
}

/// 🧵️ Never evaluates: returns the last converged cache or empty scene JSON while `flowEvalTick` runs.
pub fn preview_payload_cached(runtime: &Procedural3dRuntime, _fixture: &FlowFixture) -> (String, String) {
    if let Some(cache) = &runtime.preview_cache {
        return (cache.meshes_json.clone(), cache.instances_json.clone());
    }
    ("[]".into(), "[]".into())
}

pub fn preview_status_cached(runtime: &Procedural3dRuntime) -> Option<String> {
    runtime.preview_cache.as_ref().and_then(|cache| cache.status_json.clone())
}

pub fn generation_preview_payload_cached(runtime: &Procedural3dRuntime) -> (String, String) {
    if let Some(cache) = &runtime.generation_preview_cache {
        return (cache.meshes_json.clone(), cache.instances_json.clone());
    }
    ("[]".into(), "[]".into())
}

/// 🗂️ Refreshes the ephemeral base + generation mesh preview caches after a mutation, so the next
/// render hits instead of recomputing. `generation` carries the active selection from the runtime.
pub fn refresh_all_caches(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    refresh_preview_cache(runtime, fixture);
    if selected_generation(generation).is_none() {
        // 🪞️ No active generation: `generation_fixture_for` would just return a clone of `fixture`,
        // so the generation preview is identical to the base preview — reuse the result just
        // computed above instead of evaluating the same fixture twice.
        let signature = generation_preview_signature(fixture, generation);
        let already_cached = runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature);
        if !already_cached {
            if let Some(base) = runtime.preview_cache.clone() {
                runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
                    signature,
                    meshes_json: base.meshes_json,
                    instances_json: base.instances_json,
                    status_json: base.status_json,
                });
            }
        }
    } else {
        let generation_fixture = generation_fixture_for(fixture, generation);
        refresh_generation_preview_cache(runtime, &generation_fixture, generation);
    }
}

pub fn preview_camera_json(runtime: &Procedural3dRuntime) -> String {
    ui_wgpu::world3d_camera_json(
        runtime.preview_camera.position,
        runtime.preview_camera.target,
        runtime.preview_camera.fov,
    )
}

/// 🧭️ World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow `view_state.active_utility_id` instead of any document/runtime-stored utility.
pub fn preview_selection_json(runtime: &Procedural3dRuntime, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&semio_framework_plugin::world3d_selection_json(
        &runtime.selection_method,
        runtime.selected_node_ids.as_slice(),
        runtime.hovered_node_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    let show_mode = if runtime.show_mode.is_empty() { "shaded" } else { runtime.show_mode.as_str() };
    let (show_edges, selection_mode, targets) = match show_mode {
        "wireframe" => (true, "mesh", json!({ "mesh": false, "vertex": false, "edge": true, "face": false })),
        "points" => (false, "vertex", json!({ "mesh": false, "vertex": true, "edge": false, "face": false })),
        "shaded+edges" => (true, "mesh", json!({ "mesh": true, "vertex": false, "edge": true, "face": false })),
        _ => (false, "mesh", json!({ "mesh": true, "vertex": false, "edge": false, "face": false })),
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("gumballActive".into(), json!(!runtime.selected_node_ids.is_empty()));
        object.insert("showEdges".into(), json!(show_edges));
        object.insert("selectionMode".into(), json!(selection_mode));
        object.insert("granularity".into(), json!(selection_mode));
        object.insert("targets".into(), targets);
        object.insert("componentIds".into(), json!([]));
    }
    value.to_string()
}

fn merge_status_json(computing: Option<String>, preview_status: Option<String>) -> Option<String> {
    match (computing, preview_status) {
        (Some(c), Some(p)) => {
            let mut computing_val: Value = serde_json::from_str(&c).unwrap_or(json!({ "computing": true }));
            let preview_val: Value = serde_json::from_str(&p).unwrap_or(json!({}));
            if let (Some(c_obj), Some(p_obj)) = (computing_val.as_object_mut(), preview_val.as_object()) {
                for (k, v) in p_obj {
                    c_obj.insert(k.clone(), v.clone());
                }
            }
            Some(computing_val.to_string())
        }
        (Some(c), None) => Some(c),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

pub fn preview_scene_status_json(runtime: &Procedural3dRuntime) -> Option<String> {
    let computing = runtime.eval_driver.pending().then(|| r#"{"computing":true}"#.to_string());
    merge_status_json(computing, preview_status_cached(runtime))
}

pub fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    host_from_fixture_with_driver(fixture, None)
}

pub fn host_from_fixture_with_driver(fixture: &FlowFixture, driver: Option<&FlowEvalDriver>) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural_neural_cache());
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    if let Some(driver) = driver {
        driver.install_baseline_into(&mut host);
    }
    host
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once('@')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (String, String) {
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
                "inputs": node.inputs().iter().filter(|port| port.visible).map(|port| json!({
                    "id": format!("{}@{}", node.id, port.id),
                    "label": port.label,
                })).collect::<Vec<_>>(),
                "outputs": node.outputs().iter().filter(|port| port.visible).map(|port| json!({
                    "id": format!("{}@{}", node.id, port.id),
                    "label": port.label,
                })).collect::<Vec<_>>(),
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

pub fn widget_id_from_instance_id(instance_id: &str) -> &str {
    instance_id.split('#').next().unwrap_or(instance_id)
}

pub fn is_brep_geometry_handle(handle: &str) -> bool {
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

pub fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
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

pub fn geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"));
    let Some(channels) = channels else {
        return Vec::new();
    };
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles
}

fn mesh_has_preview_geometry(data: &semio_framework_plugin::MeshData) -> bool {
    (!data.indices.is_empty() && data.positions.len() >= 9) || data.edge_positions.len() >= 6 || (data.positions.len() >= 3 && data.indices.is_empty())
}

fn apply_show_mode_mesh(mut data: semio_framework_plugin::MeshData, show_mode: &str) -> semio_framework_plugin::MeshData {
    let show_mode = match show_mode {
        "solid" | "shaded" | "shaded+edges" | "wireframe" | "points" => show_mode,
        _ => "shaded",
    };
    match show_mode {
        "wireframe" => {
            data.positions.clear();
            data.normals.clear();
            data.indices.clear();
            data.face_ids.clear();
            data
        }
        "points" => {
            data.indices.clear();
            data.normals.clear();
            data.edge_positions.clear();
            data
        }
        _ => data,
    }
}

pub fn preview_status_json(eval_json: &str, fixture: &FlowFixture) -> Option<String> {
    let eval: Value = serde_json::from_str(eval_json).ok()?;
    if eval.get("error").and_then(Value::as_str).is_some() {
        return Some(json!({ "error": eval.get("error") }).to_string());
    }
    let mut errors = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let Some(entry) = eval.get(&id) else { continue };
        if let Some(error) = entry.get("error").and_then(Value::as_str) {
            errors.insert(id, Value::String(error.to_string()));
        }
    }
    if errors.is_empty() {
        None
    } else {
        Some(json!({ "widgetErrors": errors }).to_string())
    }
}

pub fn preview_payload_from_eval(eval_json: &str, fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
    if eval_json.is_empty() {
        return ("[]".into(), "[]".into());
    }
    if let Ok(parsed) = serde_json::from_str::<Value>(eval_json) {
        if parsed.get("error").and_then(Value::as_str).is_some() {
            return ("[]".into(), "[]".into());
        }
    }
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let tolerance = preview_tolerance(&runtime.lod_mode);
    let show_mode = if runtime.show_mode.is_empty() { "solid" } else { runtime.show_mode.as_str() };
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. });
        if !preview {
            continue;
        }
        let handles = geometry_handles_for_widget(&eval, &id);
        if handles.is_empty() {
            continue;
        }
        let selected = runtime.selected_node_ids.contains(&id);
        let hovered = runtime.hovered_node_id.as_deref() == Some(id.as_str());
        for (index, handle) in handles.iter().enumerate() {
            let mesh_id = if handles.len() == 1 {
                format!("eval-{id}")
            } else {
                format!("eval-{id}#{index}")
            };
            let instance_id = if handles.len() == 1 {
                id.clone()
            } else {
                format!("{id}#{index}")
            };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let Ok(data) = tessellate_geometry(handle, tolerance) {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        meshes.push(json!({ "id": mesh_id, "data": data }));
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                instances.push(json!({
                    "id": instance_id,
                    "meshId": mesh_id,
                    "position": [0.0, 0.0, 0.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered,
                }));
            }
        }
    }
    (
        serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()),
    )
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture_with_cache(patched_fixture, procedural_neural_cache());
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    host.evaluate().unwrap_or_default()
}

/// 👁️ Recomputes the ephemeral generation preview text for the selected generation and stores it
/// on the runtime (never on the persisted document).
pub fn refresh_generation_preview(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = selected_generation(generation) else {
        runtime.generation_preview_text = None;
        return;
    };
    runtime.generation_preview_text = Some(evaluate_generation_preview(fixture, &selected.values));
}

pub fn merge_preview_meshes(meshes: &[semio_framework_plugin::MeshData]) -> semio_framework_plugin::MeshData {
    let mut merged = semio_framework_plugin::MeshData::default();
    for mesh in meshes {
        let vertex_offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(&mesh.positions);
        merged.normals.extend(&mesh.normals);
        merged.colors.extend(&mesh.colors);
        merged.indices.extend(mesh.indices.iter().map(|index| index + vertex_offset));
        merged.edge_positions.extend(&mesh.edge_positions);
        if !mesh.edge_ids.is_empty() {
            let edge_base = merged.edge_ids.len() as u32;
            merged.edge_ids.extend(mesh.edge_ids.iter().map(|id| id + edge_base));
        }
    }
    merged
}

pub fn export_mesh_from_document(projection: &Procedural3dDocument) -> semio_framework_plugin::MeshData {
    let mut runtime = Procedural3dRuntime::default();
    let mut host = host_from_fixture(&projection.fixture);
    let eval_json = host.evaluate().unwrap_or_default();
    runtime.eval_driver.set_eval_json(eval_json.clone());
    let (meshes_json, _) = preview_payload_from_eval(&eval_json, &projection.fixture, &runtime);
    let meshes: Vec<semio_framework_plugin::MeshData> = serde_json::from_str::<Vec<Value>>(&meshes_json)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| serde_json::from_value(entry.get("data").cloned().unwrap_or(Value::Null)).ok())
        .collect();
    merge_preview_meshes(&meshes)
}

pub fn procedural3d_mesh_from_document(doc: &Value) -> Result<semio_framework_plugin::MeshData, String> {
    let projection: Procedural3dDocument = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_document(&projection))
}

pub fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(default_projection()).map_err(|err| err.to_string())
}

//#region 🔖️GumballTransforms
/// 🧭️ Maps a gumball drag operation to the flow-graph transform neuron kind that persists it.
pub fn gumball_xform_kind(operation: &str) -> &'static str {
    match operation {
        "rotate" => "brep.xform.rotate",
        "scale" => "brep.xform.scale",
        _ => "brep.xform.translate",
    }
}

/// 🪪️ Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `operation`.
pub fn gumball_widget_id(source_id: &str, operation: &str) -> String {
    format!("{source_id}__gumball_{operation}")
}

pub fn gumball_widget_json(host: &FlowHost, widget_id_str: &str) -> Option<Value> {
    host.fixture
        .widgets
        .iter()
        .find(|widget| widget_id(widget) == widget_id_str)
        .and_then(|widget| serde_json::to_value(widget).ok())
}

pub fn gumball_widget_offset(host: &FlowHost, widget_id_str: &str) -> [f64; 3] {
    let offset = gumball_widget_json(host, widget_id_str).and_then(|widget_json| widget_json.get("params").and_then(|params| params.get("offset")).cloned());
    [
        offset.as_ref().and_then(|value| value.get("x")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("y")).and_then(Value::as_f64).unwrap_or(0.0),
        offset.as_ref().and_then(|value| value.get("z")).and_then(Value::as_f64).unwrap_or(0.0),
    ]
}

pub fn gumball_widget_number_param(host: &FlowHost, widget_id_str: &str, key: &str, default: f64) -> f64 {
    gumball_widget_json(host, widget_id_str)
        .and_then(|widget_json| widget_json.get("params").and_then(|params| params.get(key)).and_then(|entry| entry.get("value")).and_then(Value::as_f64))
        .unwrap_or(default)
}

pub fn gumball_translate_params_json(offset: [f64; 3]) -> String {
    json!({ "offset": { "$schema": "vector", "x": offset[0], "y": offset[1], "z": offset[2] } }).to_string()
}

pub fn gumball_rotate_params_json(axis: [f64; 3], angle: f64) -> String {
    json!({
        "axis": { "$schema": "vector", "x": axis[0], "y": axis[1], "z": axis[2] },
        "angle": { "$schema": "number", "value": angle },
    })
    .to_string()
}

pub fn gumball_scale_params_json(factor: f64) -> String {
    json!({
        "factor": { "$schema": "number", "value": factor },
        "center": { "$schema": "point", "x": 0.0, "y": 0.0, "z": 0.0 },
    })
    .to_string()
}

/// 🔀️ Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for `operation` into the flow graph, rewiring downstream consumers so the transformed geometry is what actually evaluates and exports.
pub fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, operation: &str) -> Result<String, String> {
    let own_suffix = format!("__gumball_{operation}");
    if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
        return Ok(selected_id.to_string());
    }
    let transform_id = gumball_widget_id(selected_id, operation);
    if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
        return Ok(transform_id);
    }
    let (source_x, source_y) = host
        .fixture
        .layout
        .get(selected_id)
        .map(|layout| (layout.x, layout.y))
        .unwrap_or((0.0, 0.0));
    let descriptor = json!({ "kind": "neuron", "id": transform_id, "neuronKind": gumball_xform_kind(operation) }).to_string();
    host.add_widget(&descriptor, source_x + 220.0, source_y).map_err(|err| err.to_string())?;
    let outgoing_port = host.fixture.synapses.iter().find(|synapse| synapse.from == selected_id).map(|synapse| synapse.from_port.clone());
    if let Some(port) = outgoing_port {
        host.insert_between(selected_id, &port, &transform_id, "geometry", "geometry").map_err(|err| err.to_string())?;
    } else {
        host.connect(selected_id, &transform_id).map_err(|err| err.to_string())?;
    }
    if let Some(Widget::Neuron { preview, .. }) = host.fixture.widgets.iter_mut().find(|widget| widget_id(widget) == selected_id) {
        *preview = false;
    }
    Ok(transform_id)
}
//#endregion 🔖️GumballTransforms
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3};
    use std::sync::{Mutex, MutexGuard};

    static TEST_SERIAL: Mutex<()> = Mutex::new(());

    fn test_serial() -> MutexGuard<'static, ()> {
        TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn preview_payload_from_evaluated_fixture(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
        let mut host = FlowHost::from_fixture(fixture.clone());
        host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
        let eval_json = host.evaluate().unwrap_or_default();
        preview_payload_from_eval(&eval_json, fixture, runtime)
    }

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let _serial = test_serial();
        let projection = default_projection();
        let runtime = Procedural3dRuntime::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &runtime);
        assert_ne!(meshes_json, "[]", "meshes_json was empty");
        assert_ne!(instances_json, "[]", "instances_json was empty");
        let meshes: Vec<serde_json::Value> = serde_json::from_str(&meshes_json).expect("meshes json");
        let instances: Vec<serde_json::Value> = serde_json::from_str(&instances_json).expect("instances json");
        assert!(!meshes.is_empty());
        assert!(!instances.is_empty());
        for mesh in &meshes {
            let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
            assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
            let data: semio_framework_core::MeshData =
                serde_json::from_value(mesh.get("data").cloned().unwrap_or_default()).expect("mesh data");
            assert!(data.positions.len() >= 9, "mesh has too few positions");
            assert!(data.indices.len() >= 3, "mesh has too few indices");
            assert!(!data.edge_positions.is_empty(), "brep preview should include edge geometry");
        }
        let camera = Camera3d {
            position: Vec3::from_array([
                runtime.preview_camera.position[0] as f32,
                runtime.preview_camera.position[1] as f32,
                runtime.preview_camera.position[2] as f32,
            ]),
            target: Vec3::from_array([
                runtime.preview_camera.target[0] as f32,
                runtime.preview_camera.target[1] as f32,
                runtime.preview_camera.target[2] as f32,
            ]),
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: runtime.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
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
                .unwrap_or("eval-missing");
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
            assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
            let (min, max) = transform_aabb(model, mesh3d.aabb_min, mesh3d.aabb_max);
            if aabb_intersects_frustum(&planes, min, max) {
                visible += 1;
            }
        }
        assert!(visible > 0, "no preview instances intersect camera frustum");
    }

    #[test]
    fn document_from_mesh_returns_valid_default_projection() {
        let _serial = test_serial();
        let mesh = semio_framework_plugin::MeshData::default();
        let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
        let projection: Procedural3dDocument = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
        let _serial = test_serial();
        use semio_framework_plugin::{
            GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter,
        };
        let document_json = serde_json::to_value(default_projection()).expect("projection json");
        let mesh = procedural3d_mesh_from_document(&document_json).expect("mesh from document");
        assert!(!mesh.positions.is_empty());

        let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
        let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
        let obj_document = procedural3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
        let _: Procedural3dDocument = serde_json::from_value(obj_document).expect("parseable obj projection");

        let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
        let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
        let glb_document = procedural3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
        let _: Procedural3dDocument = serde_json::from_value(glb_document).expect("parseable glb projection");

        let stl_bytes = StlExporter.export(&mesh).expect("stl export");
        let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
        let stl_document = procedural3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
        let _: Procedural3dDocument = serde_json::from_value(stl_document).expect("parseable stl projection");
    }

    #[test]
    fn rectangle_wire_preview_emits_edge_only_mesh() {
        let _serial = test_serial();
        let projection = Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
        let runtime = Procedural3dRuntime::default();
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &runtime);
        let meshes: Vec<serde_json::Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
        let data: semio_framework_core::MeshData =
            serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
        assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
        assert!(!instances_json.is_empty());
    }

    #[test]
    fn preview_tolerance_follows_lod_mode() {
        assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
        assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
        assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
    }

    #[test]
    fn wireframe_show_mode_strips_shaded_triangles() {
        let _serial = test_serial();
        let projection = default_projection();
        let mut runtime = Procedural3dRuntime::default();
        runtime.show_mode = "wireframe".into();
        let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &runtime);
        let meshes: Vec<serde_json::Value> = serde_json::from_str(&meshes_json).expect("meshes");
        assert!(!meshes.is_empty());
        let data: semio_framework_core::MeshData =
            serde_json::from_value(meshes[0].get("data").cloned().unwrap_or_default()).expect("mesh data");
        assert!(data.indices.is_empty());
        assert!(!data.edge_positions.is_empty());
    }
}
//#endregion 🧪️Tests
