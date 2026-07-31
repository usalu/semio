//! ⚙️ Flow app — headless compute (constitutional: engine).
//!
//! Every function here is pure over `flow_core` types (`FlowHost`, `Widget`, `DagFixture`) and takes no
//! app-runtime (`FlowPlayRuntime`)/label (`FlowPlayLabels`) parameter — those types are `ui`-owned, and
//! `ui` depends on `engine`, so a dependency the other way would be circular. Compute that DOES need the
//! runtime (`host_from_fixture`, `host_operations`, `apply_canvas_options`, the context-menu builder)
//! stays in `ui`.

use flow_core::{dag::DagFixture, neural::NeuralCache, FlowHost, Widget};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};

//#region 🔖️Constants
pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDiagramPortRecord {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNodeRecord {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub inputs: Vec<WorkflowDiagramPortRecord>,
    pub outputs: Vec<WorkflowDiagramPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEdgeRecord {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}
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
    let mut sections: Vec<Value> = serde_json::from_str(&flow_core::flow_operator_catalogue_json()).unwrap_or_default();
    if let Ok(extra) = serde_json::from_str::<Vec<Value>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

pub fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    if selected.is_empty() {
        let _ = host.dag.cancel_area_select();
    } else {
        host.dag.set_selection(selected);
    }
}

pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "out".into()))
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
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| WorkflowDiagramPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| WorkflowDiagramPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
        })
        .collect();
    let edges: Vec<WorkflowEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            WorkflowEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id }
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
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
    fn flow_play_neural_cache_returns_the_same_process_wide_instance() {
        assert!(Arc::ptr_eq(&flow_play_neural_cache(), &flow_play_neural_cache()));
    }
}
//#endregion 🧪️Tests
