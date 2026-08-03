//! ⚙️ DAG app — headless compute (constitutional: engine).
//!
//! Every function here is pure over `infinite_board_port_directed_dag` types and takes no
//! app-runtime (`DagPlayRuntime`)/label (`DagPlayLabels`) parameter — those types are `ui`-owned, and
//! `ui` depends on `engine`, so a dependency the other way would be circular. Compute that constructs
//! `DagOperation` values (`remove_nodes_operations`) stays in `ui`, which already depends on `op`.

use infinite_board_port_directed_dag::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle, DagCamera, DagDocument, DagFixtureEdge, DagNodeKind, DagNodePatch, DagNodeSpec, DagPreviewContent, IoPortSpec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use ui_wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Config
/// 🧮️ `DagPlayApp::Config` — the pure-trait `DocumentApp::Config` for the dag app. Absorbs everything
/// that used to live in the ui crate's `DagPlayRuntime` (an app-struct `RefCell`) AND the two fields
/// the dag UI actually read off the deleted host-pushed `ViewState` (`locale`, via
/// `dag_play_labels`/`app_labels`/`context_menu`): the selected node ids, the free/live node-graph
/// viewport camera, and the BCP-47 locale tag — session-only view state now round-trips through the
/// config `DocumentStore` exactly like document content, with a real `backwards` per
/// `dag_op::DagConfigOperation` instead of never being VCS'd at all.
///
/// The camera is flattened to its three scalar fields (`camera_x`/`camera_y`/`camera_zoom`) rather than
/// embedding `infinite_board_port_directed_dag::DagCamera` as a `#[dsl(block)]`: that kernel type is
/// explicitly out of scope for this conversion and doesn't derive `dsl::DslRecord` (only
/// `Clone`/`Debug`/`PartialEq`/`Serialize`/`Deserialize`), so it can't satisfy a nested-block field —
/// three plain `f64` fields need no such support at all. See `dag_config_camera` below for the seam
/// back to the real `DagCamera` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "dagcfg")]
#[dsl(layout = "lines")]
pub struct DagConfig {
    /// 👁️ Selected node ids — was `DagPlayRuntime::selected_node_ids`.
    pub selected_node_ids: Vec<String>,
    /// 🎥️ Viewport camera x — was `DagPlayRuntime::camera.x`.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y — was `DagPlayRuntime::camera.y`.
    pub camera_y: f64,
    /// 🎥️ Viewport camera zoom — was `DagPlayRuntime::camera.zoom`.
    pub camera_zoom: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for DagConfig {
    fn default() -> Self {
        // 🎥️ Matches `DagCamera`'s own implicit default (`x: 0.0, y: 0.0, zoom: 1.0`, see
        // `DagFixture`'s `Default` impl in the kernel crate) without needing to parse the bundled demo
        // document just to read a trivial camera default.
        Self { selected_node_ids: Vec::new(), camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, locale: "en-US".into() }
    }
}

impl store::ConfigRecord for DagConfig {}

/// 🧮️ Whole-record diff for `dag_op::DagConfigOperation` (lives here, not in `dag_op`, since
/// `protocol::OperationDiff`/`DagConfig` are both foreign to that crate — the orphan rule requires at
/// least one local type). Mirrors `DagOperation::SetDocument`'s "whole-document replace" pattern:
/// `apply` ignores `base` entirely.
impl protocol::OperationDiff<DagConfig> for DagConfig {
    fn apply(&self, _base: &DagConfig) -> DagConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// 🎥️ Reassembles the kernel's `DagCamera` from `DagConfig`'s flattened scalar fields — the seam
/// `dag_ui` uses wherever the old `DagPlayRuntime::camera` field was read.
pub fn dag_config_camera(config: &DagConfig) -> DagCamera {
    DagCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom }
}
//#endregion 🔖️Config

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building.
#[derive(Debug, thiserror::Error)]
pub enum DagPlayError {
    #[error("connection would create cycle")]
    CycleDetected,
}
//#endregion ⚠️ Errors

//#region 🔖️DocumentHelpers
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

pub fn document_to_workflow(document: &DagDocument) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = document
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
    let edges: Vec<NodeGraphEdgeRecord> = document
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
/// refits the widget size). `raw_value` is the typed `DagCommand::PatchDagNodes.value` field verbatim
/// (a plain `&str`, not a `serde_json::Value` — B1's typed command carries the raw UI input string
/// directly, so numeric fields parse it themselves instead of round-tripping through a JSON value that
/// would always classify it as a JSON string).
pub fn node_patch_for_field(node: &DagNodeSpec, field: &str, raw_value: Option<&str>) -> Option<DagNodePatch> {
    match field {
        "name" => raw_value.map(|value| DagNodePatch { name: Some(value.into()), ..Default::default() }),
        "value" | "min" | "max" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
            let value = raw_value.and_then(|value| value.parse::<f64>().ok())?;
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_config_default_matches_dag_camera_implicit_default() {
        let config = DagConfig::default();
        assert!(config.selected_node_ids.is_empty());
        assert_eq!((config.camera_x, config.camera_y, config.camera_zoom), (0.0, 0.0, 1.0));
        assert_eq!(dag_config_camera(&config), DagCamera { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.locale, "en-US");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `DagConfig`.
    #[test]
    fn dag_config_dsl_pack_round_trip() {
        let config = DagConfig { selected_node_ids: vec!["n1".into(), "n2".into()], camera_x: 12.5, camera_y: -3.0, camera_zoom: 2.25, locale: "de-DE".into() };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }
}
//#endregion 🧪️Tests
