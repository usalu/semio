//! ⚙️ Procedural 2D app — headless compute (constitutional: engine).

use flow_core::dag::DagFixture;
use flow_core::forms_bridge::apply_generation_values_to_fixture;
use flow_core::{flow_host_with_session, flow_neuron_kind_infos_json, CameraJson, FlowEvalSession, FlowFixture, FlowHost};
use flow_extension_draw::render_scene_json;
use playbook::{selected_generation, GenerationPlayState};
use procedural_2d::Procedural2dDocument;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use store::DocumentDsl;
use ui_wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Config
/// 🧮️ B1/Wave-2: `Procedural2dPlayApp::Config` — the pure-trait config artifact replacing the old
/// ad hoc `Procedural2dPlayRuntime` app-struct `RefCell`. Selection, the graph camera, the show-mode
/// display toggle, the off-main-thread eval-driver cursor, the derived generation selection/preview,
/// and locale all round-trip through the config `DocumentStore` exactly like document content now,
/// with a real `backwards` per `procedural_2d_op::Procedural2dConfigOperation` — see
/// `procedural_2d_ui::Procedural2dPlayApp::handle`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural2dcfg")]
#[dsl(layout = "lines")]
pub struct Procedural2dConfig {
    /// 👁️ Selected widget ids — was `Procedural2dPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗺️ The node-graph camera — was `Procedural2dPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 👁️ Display mode (`"preview"`/`"generate"`/`"wire"`) — was `Procedural2dPlayRuntime::show_mode`.
    pub show_mode: String,
    /// 👁️ Active generation selection — was `Procedural2dPlayRuntime::selected_generation_id`.
    pub selected_generation_id: Option<String>,
    /// 👁️ Derived generation preview text — was `Procedural2dPlayRuntime::generation_preview_text`.
    pub generation_preview_text: Option<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for Procedural2dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, show_mode: default_show_mode(), selected_generation_id: None, generation_preview_text: None, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Procedural2dConfig);

pub fn default_show_mode() -> String {
    "preview".into()
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_procedural2d_app`'s
/// `.artifact_kind(...)` document schema/media type verbatim, plus two workflow ports: `params:in`
/// (generic Data×Value parametric input, feeds `InputSlider` widget values — see
/// `procedural_2d_ui::Procedural2dPlayApp::import_media`) and `drawing:out` (TwoD×Vector, tagged with
/// draw's already-registered `2d.drawing` kind id — see `export_media`).
pub fn procedural2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.2d",
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "2d.procedural".into(), name: "2D Procedural".into(), dimension: "2d".into(), component_kind: "procedural2d".into() },
    )
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "drawing:out".into(),
            label: "Drawing".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
            kind_id: Some("2d.drawing".into()),
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::Many,
        },
    ])
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
pub fn host_from_fixture(fixture: &FlowFixture) -> FlowHost {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    host
}

pub fn host_from_fixture_with_session(fixture: &FlowFixture, session: &FlowEvalSession) -> FlowHost {
    flow_host_with_session(fixture, session)
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
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
/// stores it on the config (never on the persisted document).
pub fn refresh_generation_preview(config: &mut Procedural2dConfig, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = selected_generation(generation) else {
        config.generation_preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(fixture, &selected.values);
    config.generation_preview_text = Some(preview);
}

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.procedural2d` example
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
//#endregion 🔖️DocumentHelpers
