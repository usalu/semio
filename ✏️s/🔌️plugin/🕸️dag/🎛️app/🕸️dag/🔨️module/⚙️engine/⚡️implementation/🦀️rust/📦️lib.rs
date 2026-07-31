//! ⚙️ DAG app — headless compute (constitutional: engine).
//!
//! Every function here is pure over `infinite_board_port_directed_dag` types and takes no
//! app-runtime (`DagPlayRuntime`)/label (`DagPlayLabels`) parameter — those types are `ui`-owned, and
//! `ui` depends on `engine`, so a dependency the other way would be circular. Compute that constructs
//! `DagOperation` values (`remove_nodes_operations`) stays in `ui`, which already depends on `op`.

use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle, DagDocument, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building.
#[derive(Debug, thiserror::Error)]
pub enum DagPlayError {
    #[error("connection would create cycle")]
    CycleDetected,
}
//#endregion ⚠️ Errors

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
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

pub fn document_to_workflow(document: &DagDocument) -> (String, String) {
    let nodes: Vec<WorkflowNodeRecord> = document
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
    let edges: Vec<WorkflowEdgeRecord> = document
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

pub fn next_node_id(document: &DagDocument) -> String {
    let max = document.nodes.iter().filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0);
    format!("n{}", max + 1)
}

pub fn default_node_for_kind(kind: &str, id: &str, x: f64, y: f64) -> DagNodeSpec {
    let mut node = match kind {
        "slider" => DagNodeSpec {
            id: id.into(),
            name: "Slider".into(),
            abbreviation: "Sld".into(),
            icon: "emoji:🎚️".into(),
            x,
            y,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.1, value: 3.0, output: IoPortSpec::named("N", "Num", "number", "Number") },
            ..Default::default()
        },
        "select" => DagNodeSpec {
            id: id.into(),
            name: "Select".into(),
            abbreviation: "Sel".into(),
            icon: "emoji:📋️".into(),
            x,
            y,
            kind: DagNodeKind::Select { options: vec!["A".into(), "B".into(), "C".into()], selected: 0, output: IoPortSpec::named("V", "Val", "value", "Value") },
            ..Default::default()
        },
        "screen" => {
            DagNodeSpec { id: id.into(), name: "Screen".into(), abbreviation: "Scr".into(), icon: "emoji:🖥️".into(), x, y, kind: DagNodeKind::Screen { media: None, input: IoPortSpec::named("I", "In", "in", "Input") }, ..Default::default() }
        }
        "note" => {
            let text = String::new();
            let (width, height) = note_widget_size(&text);
            DagNodeSpec { id: id.into(), name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝️".into(), x, y, width, height, kind: DagNodeKind::Note { text, output: IoPortSpec::named("T", "Txt", "text", "Text") }, ..Default::default() }
        }
        "preview" => {
            let (width, height) = preview_widget_size(&DagPreviewContent::Scalar { text: String::new() }, &BTreeSet::new());
            DagNodeSpec {
                id: id.into(),
                name: "Preview".into(),
                abbreviation: "Prv".into(),
                icon: "emoji:👁️".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: String::new() }, expanded: BTreeSet::new(), input: IoPortSpec::named("I", "In", "in", "Input") },
                ..Default::default()
            }
        }
        _ => DagNodeSpec {
            id: id.into(),
            name: "Computation".into(),
            abbreviation: "Cmp".into(),
            icon: "emoji:⚙️".into(),
            x,
            y,
            operator_kind: Some("math.add".into()),
            kind: DagNodeKind::Computation {
                inputs: vec![IoPortSpec::named("A", "A", "a", "A"), IoPortSpec::named("B", "B", "b", "B")],
                outputs: vec![IoPortSpec::named("R", "R", "result", "Result")],
                variadic_inputs: false,
                variadic_outputs: false,
            },
            ..Default::default()
        },
    };
    fit_node_size(&mut node);
    node
}

/// 🔗️ Builds the `DagFixtureEdge` connecting two ports, or `Err` if it would introduce a cycle.
pub fn connect_edge(document: &DagDocument, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<DagFixtureEdge, DagPlayError> {
    let existing: Vec<(String, String)> = document
        .edges
        .iter()
        .map(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            (from, to)
        })
        .collect();
    if would_create_cycle(&existing, source_node_id, target_node_id) {
        return Err(DagPlayError::CycleDetected);
    }
    let edge_id = format!("e{}", document.edges.iter().filter_map(|edge| edge.id.strip_prefix('e').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0) + 1);
    Ok(DagFixtureEdge { id: edge_id, source: format!("{source_node_id}@{source_port_id}"), target: format!("{target_node_id}@{target_port_id}"), ..Default::default() })
}

/// 🩹️ Builds the `DagNodePatch` for a `patchDagNodes` field write (name, or a slider param that also
/// refits the widget size).
pub fn node_patch_for_field(node: &DagNodeSpec, field: &str, raw_value: Option<&Value>) -> Option<DagNodePatch> {
    match field {
        "name" => raw_value.and_then(|value| value.as_str()).map(|value| DagNodePatch { name: Some(value.into()), ..Default::default() }),
        "value" | "min" | "max" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
            let value = raw_value.and_then(|value| value.as_f64())?;
            let mut updated = node.clone();
            if let DagNodeKind::Slider { value: ref mut slider_value, min: ref mut slider_min, max: ref mut slider_max, .. } = updated.kind {
                match field {
                    "value" => *slider_value = value,
                    "min" => *slider_min = value,
                    _ => *slider_max = value,
                }
            }
            fit_node_size(&mut updated);
            Some(DagNodePatch { kind: Some(updated.kind.clone()), width: Some(updated.width), height: Some(updated.height), ..Default::default() })
        }
        _ => None,
    }
}
//#endregion 🔖️DocumentHelpers
