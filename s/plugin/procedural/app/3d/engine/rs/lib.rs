//! ⚙️ Procedural 3D app — headless compute (constitutional: engine).

use flow_core::dag::DagFixture;
use flow_core::forms_bridge::apply_generation_values_to_fixture;
use flow_core::{CameraJson, FlowEvalDriver, FlowFixture, FlowHost, Widget};
use flow_module_brep::tessellate_geometry_json;
use playbook::{selected_generation, GenerationPlayState};
use procedural_3d::{widget_id, Procedural3dDocument};
use semio_framework_core::mesh_from_indexed;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use store::DocumentDsl;

//#region 🔖Constants
pub const PROCEDURAL_FALLBACK_MESH_KIND: &str = "box";
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
//#endregion 🔖Constants

//#region 🔖EvalCache
/// 🧠 Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions.
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
//#endregion 🔖EvalCache

//#region 🔖Types
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
    pub selected_node_ids: Vec<String>,
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
    /// 🧵 Off-main-thread evaluation state — see `FlowEvalDriver`.
    pub eval_driver: FlowEvalDriver,
}

impl Default for Procedural3dRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
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
    "solid".into()
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
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
/// 📄 The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example fixture (see `procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT`).
pub fn default_projection() -> Procedural3dDocument {
    Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument::default()
}

/// 🧾 Builds the initial projection for a named example (or the empty/default fixture).
pub fn example_projection(example_id: &str) -> Procedural3dDocument {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT),
        "" => None,
        _ => None,
    };
    dsl.and_then(|text| Procedural3dDocument::parse_dsl(text).ok()).unwrap_or_default()
}

/// 🧾 Serializes an example's bare projection for registration via `App::example`.
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

pub fn refresh_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture) {
    let signature = fixture_signature(fixture);
    if runtime.preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
        return;
    }
    let (meshes_json, instances_json) = evaluated_preview_payload(fixture, runtime);
    runtime.preview_cache = Some(Procedural3dPreviewCache {
        signature,
        meshes_json,
        instances_json,
    });
}

pub fn refresh_generation_preview_cache(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let signature = generation_preview_signature(fixture, generation);
    if runtime.generation_preview_cache.as_ref().is_some_and(|entry| entry.signature == signature) {
        return;
    }
    let (meshes_json, instances_json) = evaluated_preview_payload(fixture, runtime);
    runtime.generation_preview_cache = Some(Procedural3dPreviewCache {
        signature,
        meshes_json,
        instances_json,
    });
}

/// 🧵 Never evaluates: a signature mismatch (fixture changed since the cache was built) means a
/// `flowEvalTick` chain is converging on the new fixture — this returns the stale cache as-is
/// rather than blocking the render to recompute; the scene's `statusJson` reports "computing" in
/// the meantime (see `pending_effects`/`FlowEvalDriver`). Only a cold start (no cache at all) falls
/// back to a placeholder mesh per node kind.
pub fn preview_payload_cached(runtime: &Procedural3dRuntime, fixture: &FlowFixture) -> (String, String) {
    if let Some(cache) = &runtime.preview_cache {
        return (cache.meshes_json.clone(), cache.instances_json.clone());
    }
    (preview_meshes_json_fallback(fixture), preview_instances_json_fallback(fixture, runtime))
}

/// 🗂️ Refreshes the ephemeral base + generation mesh preview caches after a mutation, so the next
/// render hits instead of recomputing. `generation` carries the active selection from the runtime.
pub fn refresh_all_caches(runtime: &mut Procedural3dRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    refresh_preview_cache(runtime, fixture);
    if selected_generation(generation).is_none() {
        // 🪞 No active generation: `generation_fixture_for` would just return a clone of `fixture`,
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

/// 🧭 World-3d selection payload with the host-owned gumball utility spliced in, so the transform
/// handles follow `view_state.active_utility_id` instead of any document/runtime-stored utility.
pub fn preview_selection_json(runtime: &Procedural3dRuntime, active_utility: &str) -> String {
    let mut value: Value = serde_json::from_str(&semio_framework_plugin::world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_node_ids,
        runtime.hovered_node_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("gumballActive".into(), json!(!runtime.selected_node_ids.is_empty()));
    }
    value.to_string()
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

pub fn neuron_mesh_kind(neuron_kind: &str) -> &'static str {
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

pub fn widget_preview_mesh_kind(widget: &Widget) -> Option<&'static str> {
    match widget {
        Widget::Neuron { neuron_kind, preview, .. } if *preview => Some(neuron_mesh_kind(neuron_kind)),
        Widget::OutputPreview { .. } => Some(PROCEDURAL_FALLBACK_MESH_KIND),
        _ => None,
    }
}

pub fn widget_layout_position(fixture: &FlowFixture, widget_id: &str) -> (f64, f64) {
    fixture
        .layout
        .get(widget_id)
        .map(|layout| (layout.x, layout.y))
        .unwrap_or((0.0, 0.0))
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

pub fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
    let widget_eval = eval.get(widget_id)?;
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles.into_iter().next()
}

pub fn mesh_from_tessellation_json(mesh_json: &str) -> Option<semio_framework_plugin::MeshData> {
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

pub fn evaluated_preview_payload(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> (String, String) {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural_neural_cache());
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

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture_with_cache(patched_fixture, procedural_neural_cache());
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

pub fn preview_instances_json_fallback(fixture: &FlowFixture, runtime: &Procedural3dRuntime) -> String {
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

pub fn preview_meshes_json_fallback(fixture: &FlowFixture) -> String {
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
            let data = semio_framework_plugin::mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn export_mesh_from_document(projection: &Procedural3dDocument) -> semio_framework_plugin::MeshData {
    let runtime = Procedural3dRuntime::default();
    let (meshes_json, _) = evaluated_preview_payload(&projection.fixture, &runtime);
    if let Ok(meshes) = serde_json::from_str::<Vec<Value>>(&meshes_json) {
        if let Some(first) = meshes.first() {
            if let Ok(data) = serde_json::from_value(first.get("data").cloned().unwrap_or(Value::Null)) {
                return data;
            }
        }
    }
    let kind = projection
        .fixture
        .widgets
        .iter()
        .find_map(|widget| widget_preview_mesh_kind(widget))
        .unwrap_or(PROCEDURAL_FALLBACK_MESH_KIND);
    semio_framework_plugin::mesh_from_kind(kind)
}

pub fn procedural3d_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
    let projection: Procedural3dDocument = serde_json::from_value(doc.clone()).map_err(|err| err.to_string())?;
    Ok(export_mesh_from_document(&projection))
}

pub fn procedural3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(default_projection()).map_err(|err| err.to_string())
}

//#region 🔖GumballTransforms
/// 🧭 Maps a gumball drag operation to the flow-graph transform neuron kind that persists it.
pub fn gumball_xform_kind(operation: &str) -> &'static str {
    match operation {
        "rotate" => "brep.xform.rotate",
        "scale" => "brep.xform.scale",
        _ => "brep.xform.translate",
    }
}

/// 🪪 Deterministic id for the transform neuron generated by dragging `source_id`'s gumball for `operation`.
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

/// 🔀 Finds (or splices in) the transform neuron that persists `selected_id`'s gumball drag for `operation` into the flow graph, rewiring downstream consumers so the transformed geometry is what actually evaluates and exports.
pub fn ensure_gumball_node(host: &mut FlowHost, selected_id: &str, operation: &str) -> Result<String, String> {
    let own_suffix = format!("__gumball_{operation}");
    if selected_id.ends_with(&own_suffix) && host.fixture.widgets.iter().any(|widget| widget_id(widget) == selected_id) {
        return Ok(selected_id.to_string());
    }
    let transform_id = gumball_widget_id(selected_id, operation);
    if host.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id) {
        return Ok(transform_id);
    }
    let (source_x, source_y) = widget_layout_position(&host.fixture, selected_id);
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
//#endregion 🔖GumballTransforms
//#endregion 🔖DocumentHelpers

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3};

    #[test]
    fn preview_payload_has_meshes_and_instances() {
        let projection = default_projection();
        let runtime = Procedural3dRuntime::default();
        let (meshes_json, instances_json) = evaluated_preview_payload(&projection.fixture, &runtime);
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
    fn document_from_mesh_returns_valid_default_projection() {
        let mesh = semio_framework_plugin::MeshData::default();
        let document = procedural3d_document_from_mesh(&mesh).expect("dwg mesh import document");
        let projection: Procedural3dDocument = serde_json::from_value(document).expect("parseable projection");
        assert_eq!(projection.fixture.schema, "flow.fixture");
    }

    #[test]
    fn procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
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
}
//#endregion 🧪Tests
