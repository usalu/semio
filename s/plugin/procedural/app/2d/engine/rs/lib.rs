//! ⚙️ Procedural 2D app — headless compute (constitutional: engine).

use flow_core::dag::DagFixture;
use flow_core::forms_bridge::apply_generation_values_to_fixture;
use flow_core::{flow_neuron_kind_infos_json, CameraJson, FlowEvalDriver, FlowFixture, FlowHost};
use flow_module_draw::render_scene_json;
use playbook::{selected_generation, GenerationPlayState};
use procedural_2d::Procedural2dDocument;
use serde::Serialize;
use serde_json::{json, Value};
use store::DocumentDsl;

//#region 🔖Types
/// 👁️ Ephemeral per-session view state — never part of the persisted document. Selection, the
/// graph camera, the active show mode, the off-main-thread eval driver, and the derived generation
/// preview all live here on the app struct, out of the VCS document.
#[derive(Clone, Debug)]
pub struct Procedural2dPlayRuntime {
    pub selected_ids: Vec<String>,
    pub camera: CameraJson,
    pub show_mode: String,
    pub eval_driver: FlowEvalDriver,
    pub selected_generation_id: Option<String>,
    pub generation_preview_text: Option<String>,
}

impl Default for Procedural2dPlayRuntime {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            show_mode: default_show_mode(),
            eval_driver: FlowEvalDriver::default(),
            selected_generation_id: None,
            generation_preview_text: None,
        }
    }
}

pub fn default_show_mode() -> String {
    "preview".into()
}
//#endregion 🔖Types

//#region 🔖EvalCache
/// 🧠 Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions —
/// lets a `flowEvalTick` chain's per-tick host rebuild pick up earlier ticks' cached node outputs
/// instead of recomputing the whole graph from scratch every tick.
static PROCEDURAL2D_NEURAL_CACHE: std::sync::OnceLock<std::sync::Arc<flow_core::neural::NeuralCache>> = std::sync::OnceLock::new();

pub fn procedural2d_neural_cache() -> std::sync::Arc<flow_core::neural::NeuralCache> {
    PROCEDURAL2D_NEURAL_CACHE.get_or_init(|| std::sync::Arc::new(flow_core::neural::NeuralCache::new())).clone()
}
//#endregion 🔖EvalCache

//#region 🔖DocumentHelpers
pub fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    host_from_fixture_with_driver(fixture, None)
}

pub fn host_from_fixture_with_driver(fixture: &FlowFixture, driver: Option<&FlowEvalDriver>) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), procedural2d_neural_cache());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowDiagramPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<WorkflowDiagramPortRecord>,
    outputs: Vec<WorkflowDiagramPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<WorkflowNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| WorkflowNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node
                .inputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| WorkflowDiagramPortRecord {
                    id: format!("{}@{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
            outputs: node
                .outputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| WorkflowDiagramPortRecord {
                    id: format!("{}@{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
        })
        .collect();
    let edges: Vec<WorkflowEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            WorkflowEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

pub fn collect_drawing_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if handle.starts_with("drawing-") {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub fn affine_transform_array(value: &Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

pub fn path_segments_from_node(node: &Value) -> Vec<Value> {
    if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
        return segments.clone();
    }
    for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
        if let Some(inner) = node.get(key) {
            if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                return segments.clone();
            }
        }
    }
    Vec::new()
}

pub fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = serde_json::from_str::<Value>(&scene_json) else {
        return Vec::new();
    };
    if scene.get("error").is_some() {
        return Vec::new();
    }
    let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
        return Vec::new();
    };
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let node_body = node.get("node").unwrap_or(node);
            json!({
                "id": format!("{prefix}-{handle}-{index}"),
                "transform": affine_transform_array(node.get("transform").unwrap_or(&Value::Null)),
                "segments": path_segments_from_node(node_body),
                "fill": node.get("fill").cloned().unwrap_or(Value::Null),
                "stroke": node.get("stroke").cloned().unwrap_or(Value::Null),
                "opacity": node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0),
                "blendMode": "normal",
                "visible": true,
                "needsKernel": false,
            })
        })
        .collect()
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.evaluate().unwrap_or_default()
}

pub fn generation_preview_layers(eval_json: &str) -> String {
    let prefix = "procedural2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = serde_json::from_str::<Value>(eval_json) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Recomputes the ephemeral generation preview for the currently selected generation and
/// stores it on the runtime (never on the persisted document).
pub fn refresh_generation_preview(runtime: &mut Procedural2dPlayRuntime, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = selected_generation(generation) else {
        runtime.generation_preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(fixture, &selected.values);
    runtime.generation_preview_text = Some(preview.clone());
    runtime.eval_driver.set_eval_json(preview);
}

/// 📄 The `procedural2d-play` "default" document — parsed from the bundled `.procedural2d` example
/// fixture (see `procedural_2d_dsl::PROCEDURAL2D_EXAMPLE_TEXT`), falling back to the empty document
/// if the fixture ever fails to parse.
pub fn default_projection() -> Procedural2dDocument {
    Procedural2dDocument::parse_dsl(procedural_2d_dsl::PROCEDURAL2D_EXAMPLE_TEXT).unwrap_or_default()
}

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument::default()
}

pub fn procedural2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Procedural 2D", 1024, 768)
}

pub fn procedural2d_document_from_dwg(_drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    serde_json::to_value(default_projection()).map_err(|err| err.to_string())
}
//#endregion 🔖DocumentHelpers
