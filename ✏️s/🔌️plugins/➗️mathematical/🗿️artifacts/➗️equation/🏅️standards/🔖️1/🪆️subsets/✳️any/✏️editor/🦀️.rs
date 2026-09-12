//! 🧮️ Equation editor — `EquationPlayApp`'s `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1), the aggregated command enum and
//! the manifest stitch. B1: the pure-trait pilot for this plugin — `EquationPlayApp` is a unit
//! struct; the former `MathPlayRuntime` app-struct `RefCell` (the node-graph viewport camera) now lives in
//! `crate::editor::equation::config::EquationGraphWindowConfig`, written via `EquationGraphWindowConfigMutation`s (real
//! `backwards`, no ad hoc inverse tracking); every action dispatches through the single typed
//! `EquationCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config/🦀️.rs`. Shared compute with more than one
//! consumer across the taxonomy tree (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES dissolved
//! the former artifact-tree `⚙️engine`) lives HERE — `🔖️Io`, `🔖️Scene`, `🔖️GraphAlgorithms`, `🔖️Geometry` — since
//! an artifact is a `🧬️schema` + `🚪️io` system only, never an engine; behaviour belongs to the app.
//! This file is a routing table: `handle` → `EquationCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.
//!
//! The sibling read-only surface (`👁️viewer/🦀️.rs`) never imports from this module — see
//! that file's own doc header.

use crate::editor::equation::commands::set_artifact;
use crate::editor::equation::commands::set_points;
use crate::editor::equation::commands::{node_graph_edit, node_graph_viewport, set_algorithm, set_directed};
use crate::editor::equation::modes::edit;
use crate::editor::equation::modes::edit::windows::graph::config::{EquationGraphWindowConfigMutation, EquationGraphWindowConfigOwner};
use crate::editor::equation::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::op::EquationMutation;
use crate::{EquationGeometry, EquationGraph, EquationSnapshot, EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
use pack::json::{self, Value};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::ArtifactDisposal;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView,
    Dialect, DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation,
};
use std::collections::BTreeSet;
use std::sync::Arc;
use store::ArtifactPack;
use store::EngineHandles;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord};

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "equation-play";
pub use geometry_window::MATH_PLAY_BODY_GEOMETRY;
pub use graph_window::MATH_PLAY_BODY_GRAPH;
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_equation_app` declares via `.artifact_kind(...)` (`computation.equation`), plus one
/// extra output port: `result:out`, the current graph+geometry projection as a generic data value
/// (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn equation_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: MATH_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("computation.equation".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.equation".into(), name: "Equation".into(), dimension: "graph".into(), component_kind: "equation".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub fn algorithm_overlay(graph: &EquationGraph) -> std::collections::HashMap<String, String> {
    use graph::algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

    let index = IdIndex::from_ids(graph.nodes.iter().map(|n| n.id.as_str()));
    let edge_pairs: Vec<(usize, usize)> = graph.edges.iter().filter_map(|e| Some((index.index_of(&e.source)?, index.index_of(&e.target)?))).collect();
    let adj = adjacency(index.len(), &edge_pairs, graph.directed);
    let mut overlay = std::collections::HashMap::new();

    match graph.algorithm.as_str() {
        "topo" => match topo_sort(&adj) {
            Ok(order) => {
                for (rank, &i) in order.iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" #{rank}"));
                    }
                }
            }
            Err(_) => {
                for node in &graph.nodes {
                    overlay.insert(node.id.clone(), " ⟲".into());
                }
            }
        },
        "components" => {
            for (i, label) in connected_components(&adj).into_iter().enumerate() {
                if let Some(id) = index.id_of(i) {
                    overlay.insert(id.to_string(), format!(" ⬤️{label}"));
                }
            }
        }
        "scc" => {
            for (group, component) in strongly_connected_components(&adj).into_iter().enumerate() {
                for i in component {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" ⬤️{group}"));
                    }
                }
            }
        }
        "bfs" => {
            if let Some(seed) = graph.algorithm_seed.as_deref().and_then(|s| index.index_of(s)) {
                for (i, dist) in bfs_distances(&adj, seed).into_iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), dist.map_or_else(|| " ∞".into(), |d| format!(" d{d}")));
                    }
                }
            }
        }
        _ => {}
    }
    overlay
}

pub fn workflow_json(graph: &EquationGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<NodeGraphNodeRecord> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            NodeGraphNodeRecord { id: node.id.clone(), label: Some(format!("{}{}", node.label, suffix)), x: node.x, y: node.y, width: 72.0, height: 40.0, inputs: Vec::new(), outputs: Vec::new(), ..Default::default() }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> =
        graph.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.source.clone(), source_port_id: "out".into(), target_node_id: edge.target.clone(), target_port_id: "in".into(), label: None }).collect();
    (nodes, edges)
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
pub fn geometry_layers_json(geometry: &EquationGeometry) -> String {
    let points: Vec<geometry::Point> = geometry.points.iter().map(|p| geometry::Point::new(p.x, p.y)).collect();
    let hull = geometry::convex_hull(&points);
    let centroid = geometry::polygon_centroid(&hull);

    let mut layers: Vec<Value> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        layers.push(json::object([
            ("kind".to_string(), Value::from("circle")),
            ("id".to_string(), Value::from(format!("point-{i}"))),
            ("x".to_string(), Value::from(p.x() - 5.0)),
            ("y".to_string(), Value::from(p.y() - 5.0)),
            ("width".to_string(), Value::from(10.0)),
            ("height".to_string(), Value::from(10.0)),
            ("color".to_string(), Value::from("#38bdf8")),
        ]));
    }
    if hull.len() >= 2 {
        let mut hull_points: Vec<Value> = Vec::new();
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            hull_points.push(json::array([Value::from(a.x()), Value::from(a.y())]));
            hull_points.push(json::array([Value::from(b.x()), Value::from(b.y())]));
        }
        layers.push(json::object([("kind".to_string(), Value::from("polyline")), ("id".to_string(), Value::from("hull")), ("points".to_string(), json::array(hull_points)), ("color".to_string(), Value::from("#facc15"))]));
    }
    layers.push(json::object([
        ("kind".to_string(), Value::from("circle")),
        ("id".to_string(), Value::from("centroid")),
        ("x".to_string(), Value::from(centroid.x() - 4.0)),
        ("y".to_string(), Value::from(centroid.y() - 4.0)),
        ("width".to_string(), Value::from(8.0)),
        ("height".to_string(), Value::from(8.0)),
        ("color".to_string(), Value::from("#f472b6")),
    ]));
    json::to_string(&json::array(layers))
}
//#endregion 🔖️Geometry

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `EquationPlayApp::Command` — the SOLE dispatch surface for equation's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum EquationCommand for EquationSnapshot, EquationMutation, NoConfig, NoConfigMutation {
        "setDocument" as "set-artifact" => set_artifact::SetArtifact,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setPoints" as "set-points" => set_points::SetPoints,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const EQUATION_TOOL_IDS: &[&str] = &["setDocument", "setAlgorithm", "setDirected", "nodeGraphEdit", "nodeGraphViewport", "setPoints"];
const EQUATION_RETAINED_PAYLOAD_SCHEMA: &str = "semio.equation/v1.tool-command.v1";
const EQUATION_RETAINED_RAW_BYTES: usize = 65_536;
const EQUATION_RETAINED_WORK_ITEMS: usize = 65_536;
const EQUATION_MAX_NODES: usize = 256;
const EQUATION_MAX_EDGES: usize = 512;
const EQUATION_MAX_POINTS: usize = 1_024;
const EQUATION_MAX_EDIT_JSON_BYTES: usize = 8_192;
const EQUATION_MAX_EDIT_OPERATIONS: usize = 16;
const EQUATION_MAX_DELETE_IDS: usize = 256;
const EQUATION_MAX_TEXT_BYTES: usize = 256;

const EQUATION_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAlgorithm", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setDirected", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setPoints", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn equation_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(EQUATION_RETAINED_RAW_BYTES, 2_048, 1, EQUATION_RETAINED_WORK_ITEMS, 7_500, 1, 1)
}

fn equation_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

fn equation_operation_identity(tool_id: &str, operation: &AppOperationContext) -> u64 {
    let mut identity = equation_tool_identity(tool_id);
    let app_instance = operation.app_instance_id.to_le_bytes();
    let operation_id = operation.operation_id.to_le_bytes();
    let generation = operation.generation.to_le_bytes();
    for bytes in [app_instance.as_slice(), operation.parent_document_id.as_bytes(), operation_id.as_slice(), generation.as_slice(), operation.canonical_base_revision.as_slice()] {
        for byte in bytes {
            identity = (identity ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }
    identity
}

fn equation_graph_shape_admitted(graph: &EquationGraph) -> bool {
    graph.nodes.len() <= EQUATION_MAX_NODES && graph.edges.len() <= EQUATION_MAX_EDGES && graph.algorithm.len() <= EQUATION_MAX_TEXT_BYTES && graph.algorithm_seed.as_ref().is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES)
}

fn equation_edit_preflight(payload: &node_graph_edit::NodeGraphEdit) -> Option<usize> {
    if payload.operations_json.len() > EQUATION_MAX_EDIT_JSON_BYTES {
        return None;
    }
    let values = json::parse(&payload.operations_json).ok().and_then(|value| value.as_array().map(|values| values.to_vec()))?;
    if values.len() > EQUATION_MAX_EDIT_OPERATIONS {
        return None;
    }
    for value in &values {
        EquationEditOperation::from_value(value).ok()?;
    }
    Some(values.len())
}

fn equation_command_extent(command: &EquationCommand, snapshot: &EquationSnapshot) -> Option<usize> {
    let scene = crate::equation_scene_owner(snapshot)?;
    if !equation_graph_shape_admitted(&scene.graph) || scene.geometry.points.len() > EQUATION_MAX_POINTS {
        return None;
    }
    let extent = match command {
        EquationCommand::NodeGraphViewport(_) => 1,
        EquationCommand::SetAlgorithm(payload) if payload.algorithm.len() <= EQUATION_MAX_TEXT_BYTES && payload.seed.as_ref().is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) => {
            2_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?
        }
        EquationCommand::SetAlgorithm(_) => return None,
        EquationCommand::SetDirected(_) => 2_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?,
        EquationCommand::SetPoints(payload) if payload.geometry.points.len() <= EQUATION_MAX_POINTS => 2_usize.checked_add(payload.geometry.points.len())?,
        EquationCommand::SetPoints(_) => return None,
        EquationCommand::SetArtifact(payload)
            if payload.graph.retained_node_count() <= EQUATION_MAX_NODES
                && payload.graph.retained_edge_count() <= EQUATION_MAX_EDGES
                && payload.geometry.points.len() <= EQUATION_MAX_POINTS
                && payload.graph.retained_metadata().1.len() <= EQUATION_MAX_TEXT_BYTES
                && payload.graph.retained_metadata().2.is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) =>
        {
            2_usize.checked_add(payload.graph.retained_node_count())?.checked_add(payload.graph.retained_edge_count())?.checked_add(payload.geometry.points.len())?
        }
        EquationCommand::SetArtifact(_) => return None,
        EquationCommand::NodeGraphEdit(payload) => {
            let operation_count = equation_edit_preflight(payload)?;
            let delete_extent = operation_count.checked_mul(EQUATION_MAX_DELETE_IDS.checked_add(scene.graph.nodes.len().checked_mul(2)?)?.checked_add(scene.graph.edges.len().checked_mul(2)?)?.checked_add(8)?)?;
            4_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?.checked_add(payload.operations_json.len())?.checked_add(delete_extent)?
        }
    };
    (extent != 0 && extent <= EQUATION_RETAINED_WORK_ITEMS).then_some(extent)
}

#[derive(Clone)]
enum EquationEditOperation {
    AddNode { x: f64, y: f64 },
    Move { node_id: String, x: f64, y: f64 },
    Connect { source: String, target: String },
    DeleteSelection { ids: Vec<String> },
    Ignore,
}

impl EquationEditOperation {
    fn from_value(value: &Value) -> Result<Self, Fault> {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default();
        Ok(match text("operation") {
            "addNode" => Self::AddNode { x: value.get("x").and_then(Value::as_f64).unwrap_or(0.0), y: value.get("y").and_then(Value::as_f64).unwrap_or(0.0) },
            "move" => {
                let node_id = text("nodeId");
                if node_id.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-edit-node-id-capacity"));
                }
                match (value.get("x").and_then(Value::as_f64), value.get("y").and_then(Value::as_f64)) {
                    (Some(x), Some(y)) if !node_id.is_empty() => Self::Move { node_id: node_id.to_string(), x, y },
                    _ => Self::Ignore,
                }
            }
            "connect" => {
                let source = text("sourceNodeId");
                let target = text("targetNodeId");
                if source.len() > EQUATION_MAX_TEXT_BYTES || target.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-edit-edge-id-capacity"));
                }
                if source.is_empty() || target.is_empty() {
                    Self::Ignore
                } else {
                    Self::Connect { source: source.to_string(), target: target.to_string() }
                }
            }
            "deleteSelection" => {
                let Some(values) = value.get("nodeIds").and_then(Value::as_array) else { return Ok(Self::Ignore) };
                if values.len() > EQUATION_MAX_DELETE_IDS {
                    return Err(Fault::from("equation-edit-delete-id-capacity"));
                }
                let mut ids = Vec::new();
                ids.try_reserve_exact(values.len()).map_err(|_| Fault::from("equation-edit-delete-id-reserve"))?;
                for value in values {
                    let Some(id) = value.as_str() else { return Ok(Self::Ignore) };
                    if id.len() > EQUATION_MAX_TEXT_BYTES {
                        return Err(Fault::from("equation-edit-delete-id-capacity"));
                    }
                    ids.push(id.to_string());
                }
                Self::DeleteSelection { ids }
            }
            _ => Self::Ignore,
        })
    }

    fn retained_bytes(&self) -> usize {
        match self {
            Self::AddNode { .. } | Self::Ignore => size_of::<Self>(),
            Self::Move { node_id, .. } => size_of::<Self>() + node_id.capacity(),
            Self::Connect { source, target } => size_of::<Self>() + source.capacity() + target.capacity(),
            Self::DeleteSelection { ids } => size_of::<Self>() + ids.capacity() * size_of::<String>() + ids.iter().map(|id| id.capacity()).sum::<usize>(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EquationWorkPhase {
    Initialize,
    Nodes,
    Edges,
    Points,
    JsonBytes,
    JsonDecode,
    Operations,
    Finish,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum EquationOperationPhase {
    #[default]
    Start,
    MoveNodes,
    BuildDeleteIds,
    DeleteNodes,
    ReverseNodes,
    DeleteEdges,
    ReverseEdges,
}

struct EquationRetainedCommandWork {
    tool_id: &'static str,
    operation_identity: u64,
    extent: usize,
    phase: EquationWorkPhase,
    item_cursor: usize,
    cursor: usize,
    digest: u64,
    replay_target: Option<(usize, u64)>,
    graph: Option<EquationGraph>,
    points: Vec<crate::EquationPoint>,
    operations: Vec<EquationEditOperation>,
    operation_phase: EquationOperationPhase,
    operation_cursor: usize,
    rewrite_nodes: Vec<crate::EquationNode>,
    rewrite_edges: Vec<crate::EquationEdge>,
    delete_ids: BTreeSet<String>,
    graph_changed: bool,
    geometry_changed: bool,
    closing: bool,
}

impl EquationRetainedCommandWork {
    fn new(tool_id: &'static str, operation_identity: u64, extent: usize) -> Self {
        Self {
            tool_id,
            operation_identity,
            extent,
            phase: EquationWorkPhase::Initialize,
            item_cursor: 0,
            cursor: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            replay_target: None,
            graph: None,
            points: Vec::new(),
            operations: Vec::new(),
            operation_phase: EquationOperationPhase::Start,
            operation_cursor: 0,
            rewrite_nodes: Vec::new(),
            rewrite_edges: Vec::new(),
            delete_ids: BTreeSet::new(),
            graph_changed: false,
            geometry_changed: false,
            closing: false,
        }
    }

    fn observe(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }

    fn progress<A: semio_framework_plugin::ArtifactApp>(&mut self, bytes: &[u8], stage: &'static str) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        self.observe(bytes);
        self.cursor = self.cursor.checked_add(1).ok_or_else(|| Fault::from("equation-work-cursor-overflow"))?;
        if self.cursor > self.extent {
            return Err(Fault::from("equation-work-extent-overflow"));
        }
        if let Some((target, expected)) = self.replay_target {
            if self.cursor == target {
                if self.digest != expected {
                    return Err(Fault::from("equation-work-replay-drift"));
                }
                self.replay_target = None;
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "equation-command-replay", preview: br#"{"en":"Restoring Equation command","de":"Gleichungs-Befehl wird wiederhergestellt"}"# });
        }
        Ok(ArtifactCommandWorkStep::Progress { stage, preview: br#"{"en":"Preparing Equation command","de":"Gleichungs-Befehl wird vorbereitet"}"# })
    }

    fn source_scene(snapshot: &EquationSnapshot) -> Result<Arc<crate::EquationWorkingScene>, Fault> {
        crate::equation_scene_owner(snapshot).ok_or_else(|| Fault::from("equation-command-scene-unresolved"))
    }

    fn initialize(&mut self, command: &EquationCommand, snapshot: &EquationSnapshot) -> Result<(), Fault> {
        let source = Self::source_scene(snapshot)?;
        match command {
            EquationCommand::SetAlgorithm(payload) => {
                let mut graph = EquationGraph { directed: source.graph.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: payload.algorithm.clone(), algorithm_seed: payload.seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::SetDirected(payload) => {
                let mut graph = EquationGraph { directed: payload.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: source.graph.algorithm.clone(), algorithm_seed: source.graph.algorithm_seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::NodeGraphEdit(_) => {
                let mut graph = EquationGraph { directed: source.graph.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: source.graph.algorithm.clone(), algorithm_seed: source.graph.algorithm_seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::SetArtifact(payload) => {
                let (directed, algorithm, seed) = payload.graph.retained_metadata();
                let mut graph = EquationGraph { directed, nodes: Vec::new(), edges: Vec::new(), algorithm: algorithm.to_string(), algorithm_seed: seed.map(str::to_string) };
                graph.nodes.try_reserve_exact(payload.graph.retained_node_count()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(payload.graph.retained_edge_count()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.graph_changed = directed != source.graph.directed
                    || algorithm != source.graph.algorithm
                    || seed != source.graph.algorithm_seed.as_deref()
                    || payload.graph.retained_node_count() != source.graph.nodes.len()
                    || payload.graph.retained_edge_count() != source.graph.edges.len();
                self.geometry_changed = payload.geometry.points.len() != source.geometry.points.len();
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::SetPoints(payload) => {
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.phase = EquationWorkPhase::Points;
            }
            EquationCommand::NodeGraphViewport(_) => self.phase = EquationWorkPhase::Finish,
        }
        Ok(())
    }

    fn finish(&mut self, command: &EquationCommand, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
        use crate::standards::v1::subsets::geometry::schema::mutations::replace_points::ReplacePoints;
        use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
        Ok(match command {
            EquationCommand::SetAlgorithm(_) => Emit::commit(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))? })], "setAlgorithm"),
            EquationCommand::SetDirected(_) => Emit::mutations(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))? })]),
            EquationCommand::NodeGraphEdit(_) if self.graph_changed => Emit::mutations(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))? })]),
            EquationCommand::NodeGraphEdit(_) => Emit::default(),
            EquationCommand::SetPoints(_) => Emit::mutations(vec![EquationMutation::ReplacePoints(ReplacePoints { points: std::mem::take(&mut self.points) })]),
            EquationCommand::SetArtifact(_) => {
                let mut mutations = Vec::new();
                if self.graph_changed {
                    mutations.push(EquationMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))? }));
                }
                if self.geometry_changed {
                    mutations.push(EquationMutation::ReplacePoints(ReplacePoints { points: std::mem::take(&mut self.points) }));
                }
                Emit::mutations(mutations)
            }
            EquationCommand::NodeGraphViewport(payload) => {
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("equation-graph-window-context-required"))?;
                let mut emit = Emit::default();
                emit.window_config_mutations.push(graph_window::config::addressed(view, EquationGraphWindowConfigMutation::SetCamera(graph_window::config::SetCamera {
                    camera: EquationCamera { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom },
                }))?);
                emit
            }
        })
    }

    fn close_vec_capacity<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<InteractiveJobCloseStep> {
        if !values.is_empty() {
            return None;
        }
        let bytes = values.capacity().saturating_mul(size_of::<T>());
        if bytes == 0 {
            return None;
        }
        if maximum_items == 0 || maximum_bytes < bytes {
            return Some(InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        *values = Vec::new();
        Some(InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes })
    }

    fn advance_operation(&mut self) {
        self.item_cursor += 1;
        self.operation_cursor = 0;
        self.operation_phase = EquationOperationPhase::Start;
    }
}

impl ArtifactCommandWork<EditorApp<EquationPlayApp>> for EquationRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.operation_identity ^ (self.extent as u64).rotate_left(17)
    }

    fn extent(&self, command: &EquationCommand, snapshot: &EquationSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>) -> Option<usize> {
        let extent = equation_command_extent(command, snapshot)?;
        (extent == self.extent).then_some(extent)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<EquationPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<EquationPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation: _operation } = *input;
        if equation_command_extent(command, snapshot) != Some(self.extent) || self.cursor > self.extent {
            return Err(Fault::from("equation-command-extent-drift"));
        }
        let source = Self::source_scene(snapshot)?;
        match self.phase {
            EquationWorkPhase::Initialize => {
                self.initialize(command, snapshot)?;
                self.progress::<EditorApp<EquationPlayApp>>(self.tool_id.as_bytes(), "equation-command-initialize")
            }
            EquationWorkPhase::Nodes => {
                let (count, node) = match command {
                    EquationCommand::SetArtifact(payload) => (payload.graph.retained_node_count(), payload.graph.retained_node(self.item_cursor).cloned()),
                    _ => (source.graph.nodes.len(), source.graph.nodes.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Edges;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"nodes-complete", "equation-command-node-boundary");
                }
                let node = node.ok_or_else(|| Fault::from("equation-command-node-cursor"))?;
                if node.id.len() > EQUATION_MAX_TEXT_BYTES || node.label.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-command-node-text-capacity"));
                }
                if let EquationCommand::SetArtifact(_) = command {
                    self.graph_changed |= source.graph.nodes.get(self.item_cursor) != Some(&node);
                }
                self.observe(node.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.nodes.push(node);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-node")
            }
            EquationWorkPhase::Edges => {
                let (count, edge) = match command {
                    EquationCommand::SetArtifact(payload) => (payload.graph.retained_edge_count(), (self.item_cursor < payload.graph.retained_edge_count()).then(|| payload.graph.retained_edge(self.item_cursor)).transpose().map_err(Fault::from)?),
                    _ => (source.graph.edges.len(), source.graph.edges.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = match command {
                        EquationCommand::SetArtifact(_) => EquationWorkPhase::Points,
                        EquationCommand::NodeGraphEdit(_) => EquationWorkPhase::JsonBytes,
                        _ => EquationWorkPhase::Finish,
                    };
                    return self.progress::<EditorApp<EquationPlayApp>>(b"edges-complete", "equation-command-edge-boundary");
                }
                let edge = edge.ok_or_else(|| Fault::from("equation-command-edge-cursor"))?;
                if edge.id.len() > EQUATION_MAX_TEXT_BYTES || edge.source.len() > EQUATION_MAX_TEXT_BYTES || edge.target.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-command-edge-text-capacity"));
                }
                if let EquationCommand::SetArtifact(_) = command {
                    self.graph_changed |= source.graph.edges.get(self.item_cursor) != Some(&edge);
                }
                self.observe(edge.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.edges.push(edge);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-edge")
            }
            EquationWorkPhase::Points => {
                let points = match command {
                    EquationCommand::SetArtifact(payload) => &payload.geometry.points,
                    EquationCommand::SetPoints(payload) => &payload.geometry.points,
                    _ => return Err(Fault::from("equation-command-point-phase")),
                };
                if self.item_cursor >= points.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Finish;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"points-complete", "equation-command-point-boundary");
                }
                let point = points[self.item_cursor].clone();
                if matches!(command, EquationCommand::SetArtifact(_)) {
                    self.geometry_changed |= source.geometry.points.get(self.item_cursor) != Some(&point);
                }
                self.observe(&point.x.to_le_bytes());
                self.observe(&point.y.to_le_bytes());
                self.points.push(point);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-point")
            }
            EquationWorkPhase::JsonBytes => {
                let EquationCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("equation-command-json-phase")) };
                if self.item_cursor >= payload.operations_json.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::JsonDecode;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"json-complete", "equation-command-json-boundary");
                }
                let byte = payload.operations_json.as_bytes()[self.item_cursor];
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&[byte], "equation-command-json-byte")
            }
            EquationWorkPhase::JsonDecode => {
                let EquationCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("equation-command-json-decode")) };
                let values = json::parse(&payload.operations_json).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
                if values.len() > EQUATION_MAX_EDIT_OPERATIONS {
                    return Err(Fault::from("equation-command-operation-capacity"));
                }
                self.operations.try_reserve_exact(values.len()).map_err(|_| Fault::from("equation-command-operation-reserve"))?;
                for value in &values {
                    self.operations.push(EquationEditOperation::from_value(value)?);
                }
                self.item_cursor = 0;
                self.phase = EquationWorkPhase::Operations;
                self.progress::<EditorApp<EquationPlayApp>>(&(values.len() as u64).to_le_bytes(), "equation-command-json-decode")
            }
            EquationWorkPhase::Operations => {
                if self.item_cursor >= self.operations.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Finish;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"operations-complete", "equation-command-operation-boundary");
                }
                let operation = self.operations[self.item_cursor].clone();
                match (&operation, self.operation_phase) {
                    (_, EquationOperationPhase::Start) => match &operation {
                        EquationEditOperation::AddNode { x, y } => {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(crate::EquationNode { label: id.to_uppercase(), id, x: *x, y: *y });
                            self.graph_changed = true;
                            self.advance_operation();
                        }
                        EquationEditOperation::Move { .. } => self.operation_phase = EquationOperationPhase::MoveNodes,
                        EquationEditOperation::Connect { source, target } => {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                            let id = format!("e{}", graph.edges.len());
                            graph.edges.push(crate::EquationEdge { id, source: source.clone(), target: target.clone() });
                            self.graph_changed = true;
                            self.advance_operation();
                        }
                        EquationEditOperation::DeleteSelection { .. } => self.operation_phase = EquationOperationPhase::BuildDeleteIds,
                        EquationEditOperation::Ignore => self.advance_operation(),
                    },
                    (EquationEditOperation::Move { node_id, x, y }, EquationOperationPhase::MoveNodes) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                        if self.operation_cursor >= graph.nodes.len() {
                            self.advance_operation();
                        } else {
                            let node = &mut graph.nodes[self.operation_cursor];
                            self.operation_cursor += 1;
                            if node.id == *node_id {
                                node.x = *x;
                                node.y = *y;
                                self.graph_changed = true;
                                self.advance_operation();
                            }
                        }
                    }
                    (EquationEditOperation::DeleteSelection { ids }, EquationOperationPhase::BuildDeleteIds) => {
                        if self.operation_cursor < ids.len() {
                            self.delete_ids.insert(ids[self.operation_cursor].clone());
                            self.operation_cursor += 1;
                        } else {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                            self.rewrite_nodes = std::mem::take(&mut graph.nodes);
                            graph.nodes.try_reserve_exact(self.rewrite_nodes.len()).map_err(|_| Fault::from("equation-command-node-rewrite-reserve"))?;
                            self.operation_cursor = 0;
                            self.operation_phase = EquationOperationPhase::DeleteNodes;
                        }
                    }
                    (EquationEditOperation::DeleteSelection { .. }, EquationOperationPhase::DeleteNodes) => {
                        if let Some(node) = self.rewrite_nodes.pop() {
                            if self.delete_ids.contains(&node.id) {
                                self.graph_changed = true;
                            } else {
                                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.nodes.push(node);
                            }
                        } else {
                            self.operation_cursor = 0;
                            self.operation_phase = EquationOperationPhase::ReverseNodes;
                        }
                    }
                    (EquationEditOperation::DeleteSelection { .. }, EquationOperationPhase::ReverseNodes) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                        if self.operation_cursor < graph.nodes.len() / 2 {
                            let last = graph.nodes.len() - 1 - self.operation_cursor;
                            graph.nodes.swap(self.operation_cursor, last);
                            self.operation_cursor += 1;
                        } else {
                            self.rewrite_edges = std::mem::take(&mut graph.edges);
                            graph.edges.try_reserve_exact(self.rewrite_edges.len()).map_err(|_| Fault::from("equation-command-edge-rewrite-reserve"))?;
                            self.operation_cursor = 0;
                            self.operation_phase = EquationOperationPhase::DeleteEdges;
                        }
                    }
                    (EquationEditOperation::DeleteSelection { .. }, EquationOperationPhase::DeleteEdges) => {
                        if let Some(edge) = self.rewrite_edges.pop() {
                            if self.delete_ids.contains(&edge.source) || self.delete_ids.contains(&edge.target) {
                                self.graph_changed = true;
                            } else {
                                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.edges.push(edge);
                            }
                        } else {
                            self.operation_cursor = 0;
                            self.operation_phase = EquationOperationPhase::ReverseEdges;
                        }
                    }
                    (EquationEditOperation::DeleteSelection { .. }, EquationOperationPhase::ReverseEdges) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                        if self.operation_cursor < graph.edges.len() / 2 {
                            let last = graph.edges.len() - 1 - self.operation_cursor;
                            graph.edges.swap(self.operation_cursor, last);
                            self.operation_cursor += 1;
                        } else {
                            self.delete_ids.clear();
                            self.advance_operation();
                        }
                    }
                    _ => return Err(Fault::from("equation-command-operation-phase")),
                }
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-operation")
            }
            EquationWorkPhase::Finish => self.finish(command, context).map(ArtifactCommandWorkStep::Complete),
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 40 {
            return Err(Fault::from("equation-command-checkpoint-capacity"));
        }
        target[..40].fill(0);
        target[..4].copy_from_slice(b"MRC1");
        target[4] = self.phase as u8;
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.digest.to_le_bytes());
        target[24..32].copy_from_slice(&self.operation_identity.to_le_bytes());
        target[32..40].copy_from_slice(&(self.extent as u64).to_le_bytes());
        Ok(40)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40
            || &checkpoint[..4] != b"MRC1"
            || checkpoint[5..8] != [0, 0, 0]
            || self.graph.is_some()
            || !self.points.is_empty()
            || !self.operations.is_empty()
            || !self.rewrite_nodes.is_empty()
            || !self.rewrite_edges.is_empty()
            || !self.delete_ids.is_empty()
        {
            return Err(Fault::from("equation-command-checkpoint-invalid"));
        }
        let identity = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("equation-command-checkpoint-identity"))?);
        let extent = u64::from_le_bytes(checkpoint[32..40].try_into().map_err(|_| Fault::from("equation-command-checkpoint-extent"))?);
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("equation-command-checkpoint-cursor"))?);
        if identity != self.operation_identity || extent != self.extent as u64 || cursor > self.extent as u64 {
            return Err(Fault::from("equation-command-checkpoint-mismatch"));
        }
        self.phase = EquationWorkPhase::Initialize;
        self.item_cursor = 0;
        self.operation_cursor = 0;
        self.operation_phase = EquationOperationPhase::Start;
        self.cursor = 0;
        self.digest = 0xcbf2_9ce4_8422_2325;
        self.replay_target = (cursor != 0).then_some((cursor as usize, u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("equation-command-checkpoint-digest"))?)));
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if let Some(operation) = self.operations.last() {
            let bytes = operation.retained_bytes();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.operations.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(step) = Self::close_vec_capacity(&mut self.operations, maximum_items, maximum_bytes) {
            return step;
        }
        if let Some(id) = self.delete_ids.first() {
            let bytes = size_of::<String>() + id.capacity();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.delete_ids.pop_first();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(node) = self.rewrite_nodes.last() {
            let bytes = size_of_val(node) + node.id.capacity() + node.label.capacity();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.rewrite_nodes.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(step) = Self::close_vec_capacity(&mut self.rewrite_nodes, maximum_items, maximum_bytes) {
            return step;
        }
        if let Some(edge) = self.rewrite_edges.last() {
            let bytes = size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.rewrite_edges.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(step) = Self::close_vec_capacity(&mut self.rewrite_edges, maximum_items, maximum_bytes) {
            return step;
        }
        if let Some(point) = self.points.last() {
            let bytes = size_of_val(point);
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.points.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(step) = Self::close_vec_capacity(&mut self.points, maximum_items, maximum_bytes) {
            return step;
        }
        if let Some(graph) = self.graph.as_mut() {
            if let Some(edge) = graph.edges.last() {
                let bytes = size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity();
                if maximum_items == 0 || maximum_bytes < bytes {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                graph.edges.pop();
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
            }
            if let Some(step) = Self::close_vec_capacity(&mut graph.edges, maximum_items, maximum_bytes) {
                return step;
            }
            if let Some(node) = graph.nodes.last() {
                let bytes = size_of_val(node) + node.id.capacity() + node.label.capacity();
                if maximum_items == 0 || maximum_bytes < bytes {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                graph.nodes.pop();
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
            }
            if let Some(step) = Self::close_vec_capacity(&mut graph.nodes, maximum_items, maximum_bytes) {
                return step;
            }
            let bytes = size_of::<EquationGraph>() + graph.algorithm.capacity() + graph.algorithm_seed.as_ref().map_or(0, String::capacity);
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.graph = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.graph.is_none()
            && self.points.is_empty()
            && self.points.capacity() == 0
            && self.operations.is_empty()
            && self.operations.capacity() == 0
            && self.rewrite_nodes.is_empty()
            && self.rewrite_nodes.capacity() == 0
            && self.rewrite_edges.is_empty()
            && self.rewrite_edges.capacity() == 0
            && self.delete_ids.is_empty()
    }
}

struct EquationCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl EquationCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: EQUATION_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for EquationCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<EquationPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<EquationPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        EQUATION_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        equation_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > EQUATION_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Equation retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for EquationCommandJobFactory {
    type Owner = EditorApp<EquationPlayApp>;
    const TOOL_IDS: &'static [&'static str] = EQUATION_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = EQUATION_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
struct EquationStorePreparationFactory<P, M> {
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> Default for EquationStorePreparationFactory<P, M> {
    fn default() -> Self {
        Self { marker: std::marker::PhantomData }
    }
}

struct EquationStorePreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn equation_store_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("equation-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for EquationStorePreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, _mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Equation Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(EquationStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for EquationStorePreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Equation preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Equation preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Equation preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(equation_store_edit(mutation, inverse, self.description.take(), authority), Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Equation preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🔖️EquationPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// the registered graph-window configuration owner.
#[derive(Default)]
pub struct EquationPlayApp;

impl ArtifactEditor for EquationPlayApp {
    type Snapshot = EquationSnapshot;
    type Mutation = EquationMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = EquationCommand;

    const DIALECT: Dialect = EQUATION_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(EquationStorePreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> ArtifactDisposal<store::DraftStore<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_store_disposer() -> ArtifactDisposal<store::PresenceStore<Self::Presence, Self::PresenceMutation>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<EquationGraphWindowConfigOwner>()
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<EquationPlayApp>,
        owner_file: "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.mathematical.equation@1/*#editor",
        document_schema: "semio.equation/v1",
        factory: "EquationCommandJobFactory",
        factory_type: EquationCommandJobFactory,
        tools: {
            "setDocument" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setAlgorithm" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setDirected" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphViewport" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setPoints" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EquationCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !EQUATION_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("equation-command-tool-mismatch"));
        }
        let extent = equation_command_extent(&request.command, &request.snapshot).ok_or_else(|| Fault::from("equation-command-capacity"))?;
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(EquationRetainedCommandWork::new(tool_id, equation_operation_identity(tool_id, &operation_context), extent));
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            EquationCommand::command_id,
            EQUATION_RETAINED_RAW_BYTES,
            EQUATION_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> EquationSnapshot {
        EquationSnapshot::default()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(equation_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// not a user-facing action).
    fn command_id(command: &EquationCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &EquationCommand,
        doc: &ArtifactView<'_, EquationSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<EquationMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if let EquationCommand::NodeGraphViewport(payload) = command {
            let view = view_state.ok_or_else(|| Fault::from("equation-graph-window-context-required"))?;
            let mut emit = Emit::default();
            emit.window_config_mutations.push(graph_window::config::addressed(view, EquationGraphWindowConfigMutation::SetCamera(graph_window::config::SetCamera {
                camera: EquationCamera { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom },
            }))?);
            return Ok(emit);
        }
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.equation`-kinded output);
    /// `"document:out"` replicates `ArtifactApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    fn export_media(port: &str, doc: &ArtifactView<'_, EquationSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let graph = crate::equation_graph(doc.snapshot);
                let overlay = algorithm_overlay(&graph);
                let overlay_json = json::object(overlay.iter().map(|(id, suffix)| (id.clone(), Value::from(suffix.as_str()))));
                let json = json::to_string(&json::object([("algorithm".to_string(), Value::from(graph.algorithm.as_str())), ("overlay".to_string(), overlay_json)]));
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.equation".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, EquationSnapshot>, cfg: &ConfigView<'_, NoConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&crate::equation_graph(doc.snapshot), &graph_window::config::current(cfg).cloned().unwrap_or_default().camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&crate::equation_geometry(doc.snapshot)),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️EquationPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` —
/// `PluginBuilder::editor::<E>(def: AppDefinition)` only takes the bare definition, so the old
/// `.example_source(crate::examples::demo::source())` and
/// `.workflow("equation", "Equation", "graph")` calls are dropped here (not silently: noted
/// in the migration report). The subset's own `📚️examples/🎬️demo` facet
/// (`crate::examples::...`, real content, pre-existing) is the modern,
/// role-agnostic replacement surface for example registration.
pub fn create_equation_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(EQUATION_DIALECT)
        .document(["semio", "equation"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("math-app")
        .mode_def(edit::definition())
        .default_mode_id(edit::MATH_PLAY_MODE_EDIT)
        .window_kind_def(graph_window::definition())
        .window_kind_def(geometry_window::definition())
        .default_layout(edit::layout())
        // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
        .mutation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"))
        .mutation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
        .mutation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .action_with(semio_framework_plugin::ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), semio_framework_plugin::ActionKind::View, "camera"))
        .mutation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
        .action_interactive_job("setDocument", InteractiveJobClassification::Migrated)
        .action_interactive_job("setAlgorithm", InteractiveJobClassification::Migrated)
        .action_interactive_job("setDirected", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setPoints", InteractiveJobClassification::Migrated)
        // 📝️ Staged argument forms for the graph analysis controls.
        .action_args("setAlgorithm", vec![
            ActionArgDef::select("algorithm", LocalizedLabel::native("Algorithm", "Algorithmus"), vec![
                ActionArgOption::new("topo", LocalizedLabel::native("Topological Order", "Topologische Ordnung")),
                ActionArgOption::new("components", LocalizedLabel::native("Connected Components", "Zusammenhangskomponenten")),
                ActionArgOption::new("scc", LocalizedLabel::native("Strongly Connected Components", "Starke Zusammenhangskomponenten")),
                ActionArgOption::new("bfs", LocalizedLabel::native("Breadth-First Distances", "Breitensuche-Distanzen")),
            ]).required(),
        ])
        .action_args("setDirected", vec![
            ActionArgDef::toggle("directed", LocalizedLabel::native("Directed", "Gerichtet")).default_value(&true),
        ])
        // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
        // WORKFLOWS-END-TO-END-TYPED-PORTS) — `equation_io()` (this file's own `🔖️Io` region) is
        // this port information's single source of truth, reused here rather than duplicated.
        .io(equation_io())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests
