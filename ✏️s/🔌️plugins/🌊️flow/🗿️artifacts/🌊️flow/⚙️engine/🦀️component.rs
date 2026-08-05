//! ⚙️ Flow artifact — headless compute over the `FlowFixture` projection (constitutional: engine).
//!
//! Everything here is pure over `flow_core` types (`FlowHost`, `Widget`, `DagFixture`) plus the app's
//! view `FlowConfig`, and takes no label (`FlowPlayLabels`) parameter — labels are app chrome. The rule
//! for what lands here rather than next to a single caller: a helper with MORE THAN ONE consumer across
//! the taxonomy tree lives here; a helper with exactly one consumer lives in that consumer's component
//! file.

use crate::apps::flow::config::FlowConfig;
use crate::artifacts::flow::op::FlowOperation;
use crate::artifacts::flow::FlowFixture;
use flow_core::{
    dag::{DagDrawLod, DagFixture},
    flow_fixture_operations, flow_host_with_session,
    neural::NeuralCache,
    CameraJson, FlowEvalSession, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC,
};
use semio_framework_plugin::HostEffect;
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
use ui_wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
/// 🖱️ Default proximity-select distance.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `HostEffect` by `🎮️commands/🧮️evaluate` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";
//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers `FlowFixture`'s pack↔dsl codec under `FLOW_DOCUMENT_SCHEMA` so `framework/sync`'s folder
/// endpoints and any other schema-string-keyed caller can print/parse flow documents. Called from the
/// plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::flow::FlowPlayApp>(crate::artifacts::flow::FLOW_DOCUMENT_SCHEMA);
}
//#endregion 🔖️Register

//#region 🔖️Host
/// 🧠️ Process-wide [`flow_core::neural::NeuralCache`] shared across `FlowHost` reconstructions — lets a
/// `flowEvalTick` chain's per-tick host rebuild pick up earlier ticks' cached node outputs instead of
/// recomputing the whole graph from scratch every tick.
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

/// 🎚️ Pushes the view-state canvas options (LOD mode, proximity distance, grid) onto a freshly built host.
pub fn apply_canvas_options(host: &mut FlowHost, config: &FlowConfig) {
    if config.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&config.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&config.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(config.proximity_distance);
    host.set_grid_visible(config.grid_visible);
    host.set_grid_snap_enabled(config.grid_snap_enabled);
    let _ = host.set_grid_factor(config.grid_factor);
}

/// 🏗️ Rebuilds the stateful `FlowHost` from the document projection + view config + eval session — the
/// single entry point every command handler and every window renderer goes through.
pub fn host_from_fixture(fixture: &FlowFixture, config: &FlowConfig, session: &FlowEvalSession) -> FlowHost {
    let mut host = flow_host_with_session(fixture, session);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// ✏️ Runs a stateful `FlowHost` mutation and diffs the result back into granular `FlowOperation`s —
/// returns an empty vec when `mutate` reports "nothing changed".
pub fn host_operations(fixture: &FlowFixture, config: &FlowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowOperation> {
    let mut host = host_from_fixture(fixture, config, session);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow_fixture_operations(fixture, &host.fixture)
}

/// 🧵️ The `HostEffect` that arms/continues the off-main-thread `flowEvalTick` chain.
pub fn eval_tick_effect() -> HostEffect {
    HostEffect::DispatchAction { action: FLOW_EVAL_TICK_ACTION.into(), args: None, delay_ms: 0 }
}
//#endregion 🔖️Host

//#region 🔖️Selection
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

/// 🔍️ The camera that frames the current node selection, or `None` when nothing is selected.
pub fn focus_selection_camera(fixture: &FlowFixture, config: &FlowConfig, session: &FlowEvalSession) -> Option<CameraJson> {
    if config.selected_node_ids.is_empty() {
        return None;
    }
    let mut host = host_from_fixture(fixture, config, session);
    host.dag.set_viewport(1280, 800, 1.0);
    host.dag.set_selection(&config.selected_node_ids);
    host.focus_selection_camera(1.2)
}
//#endregion 🔖️Selection

//#region 🔖️Widgets
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
//#endregion 🔖️Widgets

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
    fn flow_play_neural_cache_returns_the_same_process_wide_instance() {
        assert!(Arc::ptr_eq(&flow_play_neural_cache(), &flow_play_neural_cache()));
    }

    #[test]
    fn host_from_fixture_deletes_edge_selected_by_synapse_domain() {
        let config = FlowConfig::default();
        let fixture = FlowFixture::default();
        let session = FlowEvalSession::new();
        let mut host = host_from_fixture(&fixture, &config, &session);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        assert!(host.has_selection(), "s1 must resolve through host_from_fixture edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    }
}
//#endregion 🧪️Tests
