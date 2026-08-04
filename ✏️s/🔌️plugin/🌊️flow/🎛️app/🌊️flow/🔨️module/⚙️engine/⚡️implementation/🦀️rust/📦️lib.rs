//! ⚙️ Flow app — headless compute (constitutional: engine).
//!
//! Every function here is pure over `flow_core` types (`FlowHost`, `Widget`, `DagFixture`) and takes no
//! app-runtime (`FlowPlayRuntime`)/label (`FlowPlayLabels`) parameter — those types are `ui`-owned, and
//! `ui` depends on `engine`, so a dependency the other way would be circular. Compute that DOES need the
//! runtime (`host_from_fixture`, `host_operations`, `apply_canvas_options`, the context-menu builder)
//! stays in `ui`.

use flow_core::{dag::DagFixture, neural::NeuralCache, CameraJson, FlowEvalDriver, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC};
use playbook::GenerationPlayState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use ui_wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
/// 🖱️ Default proximity-select distance — was `FLOW_DEFAULT_PROXIMITY_DISTANCE` in the ui crate.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor — was `FLOW_DEFAULT_GRID_FACTOR` in the ui crate.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
//#endregion 🔖️Constants

//#region 🔖️Config
/// 🧮️ `FlowPlayApp::Config` — the pure-trait `DocumentApp::Config` for the flow app. Absorbs
/// everything that used to live in the ui crate's `FlowPlayRuntime` (an app-struct `RefCell`) AND the
/// locale the flow UI read off the deleted host-pushed `ViewState` — session-only view/generate-mode
/// state now round-trips through the config `DocumentStore` exactly like document content, with a real
/// `backwards` per `flow_op::FlowConfigOperation` instead of never being VCS'd at all.
///
/// `eval_driver_json`/`extension_enabled_json`/`generation_json` hold JSON-encoded
/// `flow_core::FlowEvalDriver`/`HashMap<String, bool>`/`playbook::GenerationPlayState` payloads rather
/// than nested `#[dsl(block)]`/`#[dsl(table)]` fields: none of those three types derive
/// `dsl::DslRecord`, mirroring `procedural_3d_engine::Procedural3dConfig`'s identical
/// `eval_driver_json`/`sun_json` escape hatch for the same reason. `generation_json` stays
/// config-tracked rather than becoming a document operation (unlike the sibling `procedural_3d`/
/// `procedural_2d` apps' `GenerationOperation`-backed generations): flow's document model
/// (`flow_core::FlowOperation`) is a shared kernel crate out of scope for this conversion.
/// `camera` stays a real `#[dsl(block)]` field since `flow_core::CameraJson` DOES derive
/// `dsl::DslRecord`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "flowcfg")]
#[dsl(layout = "lines")]
pub struct FlowConfig {
    /// 👁️ Selected widget ids — was `FlowPlayRuntime::selected_node_ids`.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Selected synapse ids — was `FlowPlayRuntime::selected_edge_ids`.
    pub selected_edge_ids: Vec<String>,
    /// 👁️ Selected handle ids — was `FlowPlayRuntime::selected_handle_ids`.
    pub selected_handle_ids: Vec<String>,
    /// 👁️ Widget ids with their live-eval preview disabled — was `FlowPlayRuntime::preview_off_node_ids`.
    pub preview_off_node_ids: Vec<String>,
    /// 🎥️ The node-graph viewport camera — was `FlowPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 🧵️ JSON-encoded `flow_core::FlowEvalDriver` (off-main-thread evaluation state) — was
    /// `FlowPlayRuntime::eval_driver`; see `eval_driver`/`flow_op::FlowConfigOperation::SetEvalDriver`.
    pub eval_driver_json: String,
    /// 🎚️ LOD mode id (or `flow_core::FLOW_LOD_MODE_AUTOMATIC`) — was `FlowPlayRuntime::lod_mode`.
    pub lod_mode: String,
    /// 🖱️ Proximity-select distance — was `FlowPlayRuntime::proximity_distance`.
    pub proximity_distance: f64,
    /// 🔳️ Canvas grid visibility — was `FlowPlayRuntime::grid_visible`.
    pub grid_visible: bool,
    /// 🧲️ Canvas grid snap toggle — was `FlowPlayRuntime::grid_snap_enabled`.
    pub grid_snap_enabled: bool,
    /// 🔳️ Canvas grid factor — was `FlowPlayRuntime::grid_factor`.
    pub grid_factor: f64,
    /// 📚️ JSON-encoded extra catalogue sections — was `FlowPlayRuntime::catalogue_sections_json`.
    pub catalogue_sections_json: String,
    /// 🧩️ JSON-encoded `(extension id) -> enabled` map — was `FlowPlayRuntime::extension_enabled`.
    pub extension_enabled_json: String,
    /// 🧬️ JSON-encoded `playbook::GenerationPlayState` (Generate-mode exploration surface) — was
    /// `FlowPlayRuntime::generation`.
    pub generation_json: String,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState::locale`.
    pub locale: String,
}

impl Default for FlowConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            eval_driver_json: String::new(),
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            extension_enabled_json: String::new(),
            generation_json: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl FlowConfig {
    /// 🧵️ Parses `eval_driver_json` — falls back to `FlowEvalDriver::default()` on any malformed/empty value.
    pub fn eval_driver(&self) -> FlowEvalDriver {
        serde_json::from_str(&self.eval_driver_json).unwrap_or_default()
    }

    /// 🧩️ Parses `extension_enabled_json` — falls back to an empty map.
    pub fn extension_enabled(&self) -> HashMap<String, bool> {
        serde_json::from_str(&self.extension_enabled_json).unwrap_or_default()
    }

    /// 🧬️ Parses `generation_json` — falls back to `GenerationPlayState::default()`.
    pub fn generation(&self) -> GenerationPlayState {
        serde_json::from_str(&self.generation_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(FlowConfig);

//#endregion 🔖️Config

//#region 🔖️Types
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 🧠️ Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions —
/// lets a `flowEvalTick` chain's per-tick host rebuild pick up earlier ticks' cached node outputs
/// instead of recomputing the whole graph from scratch every tick.
static FLOW_PLAY_NEURAL_CACHE: OnceLock<Arc<NeuralCache>> = OnceLock::new();

pub fn flow_play_neural_cache() -> Arc<NeuralCache> {
    FLOW_PLAY_NEURAL_CACHE.get_or_init(|| Arc::new(NeuralCache::new())).clone()
}

pub fn seed_host_catalogue(host: &mut FlowHost, extra_sections_json: &str) {
    let mut sections = flow_core::flow_catalogue_sections();
    if let Ok(extra) = serde_json::from_str::<Vec<flow_core::CatalogueSection>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

pub fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    sync_host_selection_domains(host, selected, &[], &[]);
}

pub fn sync_host_selection_domains(host: &mut FlowHost, nodes: &[String], edges: &[String], handles: &[String]) {
    if nodes.is_empty() && edges.is_empty() && handles.is_empty() {
        let _ = host.dag.cancel_area_select();
        return;
    }
    let json = serde_json::json!({ "nodes": nodes, "edges": edges, "handles": handles });
    host.dag.set_selection_domains_json(&json.to_string());
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

pub fn widget_kind_label(widget: &Widget) -> &'static str {
    match widget {
        Widget::Neuron { .. } => "neuron",
        Widget::InputSlider { .. } => "inputSlider",
        Widget::InputNote { .. } => "inputNote",
        Widget::InputImage { .. } => "inputImage",
        Widget::Variable { .. } => "variable",
        Widget::OutputPreview { .. } => "outputPreview",
        Widget::OutputAction { .. } => "outputAction",
        Widget::OutputExport { .. } => "outputExport",
        Widget::Cluster { .. } => "cluster",
    }
}

pub fn widget_tree_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { id, neuron_kind, .. } => format!("{id} ({neuron_kind})"),
        Widget::InputSlider { id, .. } => format!("{id} (slider)"),
        Widget::InputNote { id, .. } => format!("{id} (note)"),
        Widget::OutputPreview { id, .. } => format!("{id} (preview)"),
        Widget::Variable { id, name, .. } => format!("{id} ({name})"),
        widget => format!("{} ({})", widget_id(widget), widget_kind_label(widget)),
    }
}

pub fn widget_id(widget: &Widget) -> &str {
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

pub fn flow_widget_descriptor(kind: &str, neuron_kind: Option<&str>) -> Value {
    if kind == "neuron" {
        json!({ "kind": "neuron", "neuronKind": neuron_kind.unwrap_or(kind) })
    } else {
        json!({ "kind": kind })
    }
}

/// 🪢️ Wraps a widget descriptor into the `{mime: payload}` JSON shape `tree_item_with_action_draggable`
/// expects for its drag-data map.
pub fn flow_widget_drag_json(descriptor: &Value) -> Value {
    json!({ FLOW_WIDGET_DRAG_MIME: descriptor.to_string() })
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_endpoint_defaults_port_to_out() {
        assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
        assert_eq!(split_endpoint("node"), ("node".to_string(), "out".to_string()));
    }

    #[test]
    fn widget_id_and_kind_label_agree_across_variants() {
        let widget = Widget::InputSlider { id: "slider".into(), value: 3.0, min: 0.0, max: 10.0, step: 0.1 };
        assert_eq!(widget_id(&widget), "slider");
        assert_eq!(widget_kind_label(&widget), "inputSlider");
        assert_eq!(widget_tree_label(&widget), "slider (slider)");
    }

    #[test]
    fn flow_widget_drag_json_wraps_descriptor_under_drag_mime() {
        let descriptor = flow_widget_descriptor("neuron", Some("math.add"));
        let drag = flow_widget_drag_json(&descriptor);
        assert!(drag.get(FLOW_WIDGET_DRAG_MIME).is_some());
    }

    #[test]
    fn flow_config_default_matches_flow_play_runtime_defaults() {
        let config = FlowConfig::default();
        assert_eq!(config.camera, CameraJson { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.lod_mode, FLOW_LOD_MODE_AUTOMATIC);
        assert_eq!(config.proximity_distance, FLOW_DEFAULT_PROXIMITY_DISTANCE);
        assert!(config.grid_visible);
        assert!(!config.grid_snap_enabled);
        assert_eq!(config.grid_factor, FLOW_DEFAULT_GRID_FACTOR);
        assert_eq!(config.catalogue_sections_json, "[]");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.eval_driver(), FlowEvalDriver::default());
        assert_eq!(config.extension_enabled(), HashMap::new());
        assert_eq!(config.generation(), GenerationPlayState::default());
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `FlowConfig`.
    #[test]
    fn flow_config_dsl_pack_round_trip() {
        let config = FlowConfig {
            selected_node_ids: vec!["n1".into(), "n2".into()],
            selected_edge_ids: vec!["e1".into()],
            selected_handle_ids: vec!["h1".into()],
            preview_off_node_ids: vec!["n2".into()],
            camera: CameraJson { x: 12.5, y: -3.0, zoom: 2.25 },
            eval_driver_json: serde_json::to_string(&FlowEvalDriver::default()).unwrap_or_default(),
            lod_mode: "micro".into(),
            proximity_distance: 96.0,
            grid_visible: false,
            grid_snap_enabled: true,
            grid_factor: 5.0,
            catalogue_sections_json: "[{\"id\":\"custom\"}]".into(),
            extension_enabled_json: "{\"auto-layout\":true}".into(),
            generation_json: "{\"generations\":[]}".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn flow_play_neural_cache_returns_the_same_process_wide_instance() {
        assert!(Arc::ptr_eq(&flow_play_neural_cache(), &flow_play_neural_cache()));
    }
}
//#endregion 🧪️Tests
