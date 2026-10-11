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

use crate::editor::equation::commands::edit_equation;
use crate::editor::equation::commands::edit_points;
use crate::editor::equation::commands::node_graph_edit::EquationEditOperation;
use crate::editor::equation::commands::{add_node, node_graph_edit, node_graph_viewport, set_active_example, set_algorithm, set_directed};
use crate::editor::equation::modes::edit;
use crate::editor::equation::modes::edit::windows::graph::config::{EquationCamera, EquationGraphWindowConfigMutation, EquationGraphWindowConfigOwner};
use crate::editor::equation::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::op::EquationMutation;
use crate::{EquationGeometry, EquationGraph, EquationSnapshot, EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
use semio_framework_pack_json::Value;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::ArtifactDisposal;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use std::sync::Arc;
use store::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "equation-play";

/// 🧬️ The whole-document replacement `setActiveExample` emits. The spr is built with
/// `store::empty_document_spr`, NEVER by minting an `ArtifactEnvelope` to print one: an envelope is
/// a terminal store shell whose `Drop` asserts that its app-owned bounded retirement authority
/// detached every nested owner first, and nothing on this path ever mounts or retires it — the
/// envelope route panics inside the guest (`🗒️note`/`✒️writer`/`🕸️dag` all take this route). The log
/// is edit-free by construction, which is exactly what a whole-document replace carries.
pub fn reset_equation_document_effect(document: &EquationSnapshot) -> semio_framework_plugin::Effect {
    let pack = EquationSnapshot::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(MATH_APP_ID, MATH_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
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
        artifact_schema: MATH_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
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

/// 🔌️ The one output and one input port every equation node exposes. The edges have always named them,
/// but the nodes declared no ports at all, so the node-graph engine could resolve no edge endpoint and the
/// mathematical play pane drew its four nodes without a single edge (measured live 2026-09-23).
pub const EQUATION_EDGE_SOURCE_PORT: &str = "out";
pub const EQUATION_EDGE_TARGET_PORT: &str = "in";

pub fn workflow_json(graph: &EquationGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<NodeGraphNodeRecord> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            NodeGraphNodeRecord {
                id: node.id.clone(),
                label: Some(format!("{}{}", node.label, suffix)),
                x: node.x,
                y: node.y,
                width: 72.0,
                height: 40.0,
                inputs: vec![NodeGraphPortRecord { id: EQUATION_EDGE_TARGET_PORT.into(), ..Default::default() }],
                outputs: vec![NodeGraphPortRecord { id: EQUATION_EDGE_SOURCE_PORT.into(), ..Default::default() }],
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> =
        graph.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.source.clone(), source_port_id: EQUATION_EDGE_SOURCE_PORT.into(), target_node_id: edge.target.clone(), target_port_id: EQUATION_EDGE_TARGET_PORT.into(), label: None }).collect();
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
        layers.push(semio_framework_pack_json::object([
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
            hull_points.push(semio_framework_pack_json::array([Value::from(a.x()), Value::from(a.y())]));
            hull_points.push(semio_framework_pack_json::array([Value::from(b.x()), Value::from(b.y())]));
        }
        layers.push(semio_framework_pack_json::object([("kind".to_string(), Value::from("polyline")), ("id".to_string(), Value::from("hull")), ("points".to_string(), semio_framework_pack_json::array(hull_points)), ("color".to_string(), Value::from("#facc15"))]));
    }
    layers.push(semio_framework_pack_json::object([
        ("kind".to_string(), Value::from("circle")),
        ("id".to_string(), Value::from("centroid")),
        ("x".to_string(), Value::from(centroid.x() - 4.0)),
        ("y".to_string(), Value::from(centroid.y() - 4.0)),
        ("width".to_string(), Value::from(8.0)),
        ("height".to_string(), Value::from(8.0)),
        ("color".to_string(), Value::from("#f472b6")),
    ]));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::array(layers))
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
        "editEquation" as "edit-equation" => edit_equation::EditEquation,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "editPoints" as "edit-points" => edit_points::EditPoints,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const EQUATION_TOOL_IDS: &[&str] = &["editEquation", "setAlgorithm", "setDirected", "nodeGraphEdit", "nodeGraphViewport", "editPoints", "setActiveExample", "addNode"];
const EQUATION_RETAINED_PAYLOAD_SCHEMA: &str = "semio.equation/v1.tool-command.v1";
const EQUATION_RETAINED_RAW_BYTES: usize = 65_536;
const EQUATION_RETAINED_WORK_ITEMS: usize = 65_536;
const EQUATION_MAX_NODES: usize = 256;
const EQUATION_MAX_EDGES: usize = 512;
const EQUATION_MAX_POINTS: usize = 1_024;
const EQUATION_MAX_EDIT_JSON_BYTES: usize = 8_192;
const EQUATION_MAX_EDIT_OPERATIONS: usize = 16;
pub(crate) const EQUATION_MAX_DELETE_IDS: usize = 256;
pub(crate) const EQUATION_MAX_TEXT_BYTES: usize = 256;
/// 🧬️ The `nodeGraphEdit` argument the Actions pane stages by default — one node-graph gesture record `move` of the demo's
/// node `a`, in the exact shared row shape `EquationEditOperation::from_value` admits.
const EQUATION_DEFAULT_EDIT_OPERATIONS: &str = r#"[{"operation":"move","gestureId":"actions-pane","nodeIds":["a"],"dx":40.0,"dy":20.0}]"#;

const EQUATION_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "editEquation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAlgorithm", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setDirected", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "editPoints", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
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
    let values = semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec()))?;
    if values.len() > EQUATION_MAX_EDIT_OPERATIONS {
        return None;
    }
    for value in &values {
        EquationEditOperation::from_value(value).ok()?;
    }
    Some(values.len())
}

fn equation_command_extent(command: &EquationCommand, snapshot: &EquationSnapshot) -> Option<usize> {
    if let EquationCommand::SetActiveExample(payload) = command {
        return (payload.example_id.len() <= EQUATION_MAX_TEXT_BYTES).then_some(1);
    }
    if !equation_graph_shape_admitted(&snapshot.graph) || snapshot.geometry.points.len() > EQUATION_MAX_POINTS {
        return None;
    }
    let extent = match command {
        EquationCommand::SetActiveExample(_) => return None,
        EquationCommand::NodeGraphViewport(_) => 1,
        // 🧮️ `EquationRetainedCommandWork::step` charges ONE step per phase BOUNDARY on top of the
        // per-item steps: `Initialize`, then `nodes-complete`, then `edges-complete` (`Finish` itself
        // completes without a step). A graph-walking verb therefore costs `3 + nodes + edges`, not
        // `2 + …` — the old constant priced exactly one boundary (the shape `SetPoints` has) and every
        // `setAlgorithm`/`setDirected` overflowed its own extent by one step with
        // `equation-work-extent-overflow` on the very last boundary.
        EquationCommand::SetAlgorithm(payload) if payload.algorithm.len() <= EQUATION_MAX_TEXT_BYTES && payload.seed.as_ref().is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) => 1,
        EquationCommand::SetAlgorithm(_) => return None,
        EquationCommand::SetDirected(_) => 1,
        EquationCommand::AddNode(payload) if payload.x.is_finite() && payload.y.is_finite() => 1,
        EquationCommand::AddNode(_) => return None,
        EquationCommand::EditPoints(payload) if payload.geometry.points.len() <= EQUATION_MAX_POINTS => 2_usize.checked_add(payload.geometry.points.len())?,
        EquationCommand::EditPoints(_) => return None,
        EquationCommand::EditEquation(payload)
            if payload.graph.retained_node_count() <= EQUATION_MAX_NODES
                && payload.graph.retained_edge_count() <= EQUATION_MAX_EDGES
                && payload.geometry.points.len() <= EQUATION_MAX_POINTS
                && payload.graph.retained_metadata().1.len() <= EQUATION_MAX_TEXT_BYTES
                && payload.graph.retained_metadata().2.is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) =>
        {
            // 🗿️ `setArtifact` walks THREE phases (nodes, edges, points), so it pays `Initialize` plus
            // three boundaries — see the `setAlgorithm` note above.
            4_usize.checked_add(payload.graph.retained_node_count())?.checked_add(payload.graph.retained_edge_count())?.checked_add(payload.geometry.points.len())?
        }
        EquationCommand::EditEquation(_) => return None,
        EquationCommand::NodeGraphEdit(payload) => 4_usize.checked_add(payload.operations_json.len())?.checked_add(equation_edit_preflight(payload)?)?,
    };
    (extent != 0 && extent <= EQUATION_RETAINED_WORK_ITEMS).then_some(extent)
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

/// 📏️ The heap one yielded leaf retains while a retained job holds it.
fn equation_leaf_retained_bytes(leaf: &EquationMutation) -> usize {
    use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes::ConnectNodes, create_node::CreateNode, delete_node::DeleteNode, disconnect_nodes::DisconnectNodes, move_nodes::MoveNodes, update_graph_algorithm::UpdateGraphAlgorithm};
    size_of::<EquationMutation>()
        + match leaf {
            EquationMutation::CreateNode(CreateNode { id, label, .. }) => id.capacity() + label.capacity(),
            EquationMutation::MoveNodes(MoveNodes { ids, .. }) => ids.capacity() * size_of::<String>() + ids.iter().map(String::capacity).sum::<usize>(),
            EquationMutation::ConnectNodes(ConnectNodes { id, source, target, .. }) => id.capacity() + source.capacity() + target.capacity(),
            EquationMutation::DisconnectNodes(DisconnectNodes { id }) | EquationMutation::DeleteNode(DeleteNode { id }) => id.capacity(),
            EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm, new_algorithm_seed }) => new_algorithm.capacity() + new_algorithm_seed.as_ref().map_or(0, String::capacity),
            _ => 0,
        }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EquationCloseUnit {
    Operation,
    OperationsBacking,
    Leaf,
    LeavesBacking,
    Gesture,
    Point,
    PointsBacking,
    Edge,
    EdgesBacking,
    Node,
    NodesBacking,
    Graph,
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
    leaves: Vec<EquationMutation>,
    gesture: Option<String>,
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
            leaves: Vec::new(),
            gesture: None,
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

    fn initialize(&mut self, command: &EquationCommand, snapshot: &EquationSnapshot) -> Result<(), Fault> {
        let source = snapshot;
        match command {
            // 🎬️ Completed in `step` before any phase runs — it exists to CREATE the scene these
            // phases read, so reaching the phase machine at all is a routing defect.
            EquationCommand::SetActiveExample(_) => return Err(Fault::from("equation-set-active-example-is-not-a-phased-command")),
            EquationCommand::SetAlgorithm(payload) => {
                self.leaves = set_algorithm::set_algorithm_leaves(payload, &source.graph);
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::SetDirected(payload) => {
                self.leaves = set_directed::set_directed_leaves(payload, &source.graph);
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::AddNode(payload) => {
                self.leaves = add_node::add_node_leaves(payload, &source.graph)?;
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::NodeGraphEdit(_) => {
                self.graph = Some(source.graph.clone());
                self.phase = EquationWorkPhase::JsonBytes;
            }
            EquationCommand::EditEquation(payload) => {
                let (directed, algorithm, seed) = payload.graph.retained_metadata();
                let mut graph = EquationGraph { directed, nodes: Vec::new(), edges: Vec::new(), algorithm: algorithm.to_string(), algorithm_seed: seed.map(str::to_string) };
                graph.nodes.try_reserve_exact(payload.graph.retained_node_count()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(payload.graph.retained_edge_count()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::EditPoints(payload) => {
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.phase = EquationWorkPhase::Points;
            }
            EquationCommand::NodeGraphViewport(_) => self.phase = EquationWorkPhase::Finish,
        }
        Ok(())
    }

    fn finish(&mut self, command: &EquationCommand, source: &EquationSnapshot, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>, operation: &AppOperationContext) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
        Ok(match command {
            EquationCommand::SetActiveExample(_) => return Err(Fault::from("equation-set-active-example-is-not-a-phased-command")),
            EquationCommand::SetAlgorithm(_) | EquationCommand::SetDirected(_) | EquationCommand::AddNode(_) => Emit::mutations(std::mem::take(&mut self.leaves)),
            EquationCommand::NodeGraphEdit(_) => node_graph_edit::equation_edit_emit(&operation.authoring_seed, self.gesture.take().as_deref(), std::mem::take(&mut self.leaves)),
            EquationCommand::EditPoints(_) => Emit::mutations(edit_equation::equation_point_edit_leaves(&source.geometry.points, &std::mem::take(&mut self.points))),
            EquationCommand::EditEquation(_) => {
                let graph = self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                Emit::mutations(edit_equation::equation_graph_edit_leaves(&source.graph, &graph).into_iter().chain(edit_equation::equation_point_edit_leaves(&source.geometry.points, &std::mem::take(&mut self.points))).collect())
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

    fn backing_bytes<T>(values: &Vec<T>) -> usize {
        if values.is_empty() { values.capacity().saturating_mul(size_of::<T>()) } else { 0 }
    }

    fn next_close_unit(&self) -> Option<(EquationCloseUnit, usize)> {
        if let Some(operation) = self.operations.last() {
            return Some((EquationCloseUnit::Operation, operation.retained_bytes()));
        }
        let bytes = Self::backing_bytes(&self.operations);
        if bytes != 0 {
            return Some((EquationCloseUnit::OperationsBacking, bytes));
        }
        if let Some(leaf) = self.leaves.last() {
            return Some((EquationCloseUnit::Leaf, equation_leaf_retained_bytes(leaf)));
        }
        let bytes = Self::backing_bytes(&self.leaves);
        if bytes != 0 {
            return Some((EquationCloseUnit::LeavesBacking, bytes));
        }
        if let Some(gesture) = self.gesture.as_ref() {
            return Some((EquationCloseUnit::Gesture, gesture.capacity()));
        }
        if let Some(point) = self.points.last() {
            return Some((EquationCloseUnit::Point, size_of_val(point)));
        }
        let bytes = Self::backing_bytes(&self.points);
        if bytes != 0 {
            return Some((EquationCloseUnit::PointsBacking, bytes));
        }
        let graph = self.graph.as_ref()?;
        if let Some(edge) = graph.edges.last() {
            return Some((EquationCloseUnit::Edge, size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity()));
        }
        let bytes = Self::backing_bytes(&graph.edges);
        if bytes != 0 {
            return Some((EquationCloseUnit::EdgesBacking, bytes));
        }
        if let Some(node) = graph.nodes.last() {
            return Some((EquationCloseUnit::Node, size_of_val(node) + node.id.capacity() + node.label.capacity()));
        }
        let bytes = Self::backing_bytes(&graph.nodes);
        if bytes != 0 {
            return Some((EquationCloseUnit::NodesBacking, bytes));
        }
        Some((EquationCloseUnit::Graph, size_of::<EquationGraph>() + graph.algorithm.capacity() + graph.algorithm_seed.as_ref().map_or(0, String::capacity)))
    }

    fn close_unit(&mut self, unit: EquationCloseUnit) {
        match unit {
            EquationCloseUnit::Operation => drop(self.operations.pop()),
            EquationCloseUnit::OperationsBacking => self.operations = Vec::new(),
            EquationCloseUnit::Leaf => drop(self.leaves.pop()),
            EquationCloseUnit::LeavesBacking => self.leaves = Vec::new(),
            EquationCloseUnit::Gesture => self.gesture = None,
            EquationCloseUnit::Point => drop(self.points.pop()),
            EquationCloseUnit::PointsBacking => self.points = Vec::new(),
            EquationCloseUnit::Edge => drop(self.graph.as_mut().and_then(|graph| graph.edges.pop())),
            EquationCloseUnit::EdgesBacking => drop(self.graph.as_mut().map(|graph| std::mem::take(&mut graph.edges))),
            EquationCloseUnit::Node => drop(self.graph.as_mut().and_then(|graph| graph.nodes.pop())),
            EquationCloseUnit::NodesBacking => drop(self.graph.as_mut().map(|graph| std::mem::take(&mut graph.nodes))),
            EquationCloseUnit::Graph => self.graph = None,
        }
    }

    fn close_demand(&self) -> semio_framework_value::RetirementDemand {
        self.next_close_unit().map_or_else(Default::default, |(_, bytes)| semio_framework_value::RetirementDemand { release_bytes: bytes, depth: 1, ..Default::default() })
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

    /// 🧮️ One bounded microturn. The job owns its command and snapshot for the whole operation and the
    /// framework measures [`Self::extent`] once in its preflight phase, so a turn never re-derives it:
    /// doing so cloned the whole working scene and re-parsed the edit JSON on every per-item turn,
    /// which made each microturn cost O(document) and the operation quadratic.
    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<EquationPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<EquationPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation } = *input;
        if self.cursor > self.extent {
            return Err(Fault::from("equation-command-extent-overflow"));
        }
        // 🎬️ One step, no scene: the whole-document replacement is a host-applied effect.
        if let EquationCommand::SetActiveExample(payload) = command {
            return set_active_example::emit(&payload.example_id).map(ArtifactCommandWorkStep::Complete);
        }
        let source = snapshot;
        match self.phase {
            EquationWorkPhase::Initialize => {
                self.initialize(command, snapshot)?;
                self.progress::<EditorApp<EquationPlayApp>>(self.tool_id.as_bytes(), "equation-command-initialize")
            }
            EquationWorkPhase::Nodes => {
                let (count, node) = match command {
                    EquationCommand::EditEquation(payload) => (payload.graph.retained_node_count(), payload.graph.retained_node(self.item_cursor).cloned()),
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
                self.observe(node.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.nodes.push(node);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-node")
            }
            EquationWorkPhase::Edges => {
                let (count, edge) = match command {
                    EquationCommand::EditEquation(payload) => (payload.graph.retained_edge_count(), (self.item_cursor < payload.graph.retained_edge_count()).then(|| payload.graph.retained_edge(self.item_cursor)).transpose().map_err(Fault::from)?),
                    _ => (source.graph.edges.len(), source.graph.edges.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = match command {
                        EquationCommand::EditEquation(_) => EquationWorkPhase::Points,
                        _ => EquationWorkPhase::Finish,
                    };
                    return self.progress::<EditorApp<EquationPlayApp>>(b"edges-complete", "equation-command-edge-boundary");
                }
                let edge = edge.ok_or_else(|| Fault::from("equation-command-edge-cursor"))?;
                if edge.id.len() > EQUATION_MAX_TEXT_BYTES || edge.source.len() > EQUATION_MAX_TEXT_BYTES || edge.target.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-command-edge-text-capacity"));
                }
                self.observe(edge.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.edges.push(edge);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-edge")
            }
            EquationWorkPhase::Points => {
                let points = match command {
                    EquationCommand::EditEquation(payload) => &payload.geometry.points,
                    EquationCommand::EditPoints(payload) => &payload.geometry.points,
                    _ => return Err(Fault::from("equation-command-point-phase")),
                };
                if self.item_cursor >= points.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Finish;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"points-complete", "equation-command-point-boundary");
                }
                let point = points[self.item_cursor].clone();
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
                let values = semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
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
                let edit = self.operations[self.item_cursor].clone();
                let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                let (leaves, gesture) = node_graph_edit::equation_edit_operation_leaves(graph, &edit);
                self.leaves.try_reserve(leaves.len()).map_err(|_| Fault::from("equation-command-leaf-reserve"))?;
                self.leaves.extend(leaves);
                if self.gesture.is_none() {
                    self.gesture = gesture;
                }
                self.item_cursor += 1;
                let observed = [self.item_cursor.to_le_bytes(), self.leaves.len().to_le_bytes()].concat();
                self.progress::<EditorApp<EquationPlayApp>>(&observed, "equation-command-operation")
            }
            EquationWorkPhase::Finish => self.finish(command, source, context, operation).map(ArtifactCommandWorkStep::Complete),
        }
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0..=3=>Some(b"MRC1"[index]),4=>Some(self.phase as u8),5..=7=>Some(0),8..=15=>Some(((self.cursor as u64)>>((index-8)*8))as u8),16..=23=>Some((self.digest>>((index-16)*8))as u8),24..=31=>Some((self.operation_identity>>((index-24)*8))as u8),32..=39=>Some(((self.extent as u64)>>((index-32)*8))as u8),_=>None}}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40
            || &checkpoint[..4] != b"MRC1"
            || checkpoint[5..8] != [0, 0, 0]
            || self.graph.is_some()
            || !self.points.is_empty()
            || !self.operations.is_empty()
            || !self.leaves.is_empty()
            || self.gesture.is_some()
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
        self.cursor = 0;
        self.digest = 0xcbf2_9ce4_8422_2325;
        self.replay_target = (cursor != 0).then_some((cursor as usize, u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("equation-command-checkpoint-digest"))?)));
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        let Some((unit, bytes)) = self.next_close_unit() else { return InteractiveJobCloseStep::Complete { progress: Default::default() } };
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if grant.maximum_depth == 0 {
            return InteractiveJobCloseStep::Refused { kind: semio_framework_value::ValueRefusalKind::DepthLimit, progress: Default::default() };
        }
        self.close_unit(unit);
        InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() } }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().depth)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.graph.is_none()
            && self.points.is_empty()
            && self.points.capacity() == 0
            && self.operations.is_empty()
            && self.operations.capacity() == 0
            && self.leaves.is_empty()
            && self.leaves.capacity() == 0
            && self.gesture.is_none()
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
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct EquationStorePreparationFactory<P, M> {
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> Default for EquationStorePreparationFactory<P, M> {
    fn default() -> Self {
        Self { marker: std::marker::PhantomData }
    }
}

struct EquationStorePreparation<P, M> {
    base: std::mem::ManuallyDrop<Option<store::SnapshotRead<P>>>,

    mutation: std::mem::ManuallyDrop<Option<M>>,

    inverse: std::mem::ManuallyDrop<Option<Vec<M>>>,
    refused: std::mem::ManuallyDrop<Option<(protocol::Edit<M>, std::sync::Arc<P>)>>,
    apply_refusal: std::mem::ManuallyDrop<Option<protocol::MutationApplyError>>,
    authority: std::mem::ManuallyDrop<Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>>,

    prepared: std::mem::ManuallyDrop<Option<store::ArtifactStoreOneItemPrepared<P, M>>>,

    mutation_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<P>>>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}

impl<P, M> EquationStorePreparation<P, M>
where P: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static, M: semio_framework_value::retirement::RetireOwned + Send + 'static,
{
    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "preparation close depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return nested(store::artifact_retirement_box_demands(active, body)?); }
        if self.prepared.is_some() { let birth = store::ArtifactStoreOneItemPrepared::<P, M>::retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.mutation)?); }
        if self.inverse.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.inverse)?); }
        if self.refused.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.refused)?); }
        if self.apply_refusal.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.apply_refusal)?); }
        if self.base.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.base)?); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(semio_framework_value::RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth + 1, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        self.factories.iter().find_map(Option::as_ref).map_or(Ok(Default::default()), |factory| nested(factory.demands(body)?))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for EquationStorePreparationFactory<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + semio_framework_pack_json::ArtifactCanonicalJsonTree + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<M>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<M>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &M, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Equation Store preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &M, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand {capacity_bytes:std::mem::size_of::<EquationStorePreparation<P,M>>(),depth:1})
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
        let demand=match self.begin_demand(&request.mutation,request.lane){Ok(demand)=>demand,Err(error)=>return Err((error,request))};
        let progress=match demand.admit(grant.retained_grant()){Ok(progress)=>progress,Err(error)=>return Err((error,request))};
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preparation rejected original publication authority"),request));
        }
        Ok((Box::new(EquationStorePreparation {
            base: std::mem::ManuallyDrop::new(Some(request.base)),
            mutation: std::mem::ManuallyDrop::new(Some(request.mutation)),
            inverse: std::mem::ManuallyDrop::new(None),
            refused: std::mem::ManuallyDrop::new(None),
            apply_refusal: std::mem::ManuallyDrop::new(None),
            authority: std::mem::ManuallyDrop::new(Some(request.authority)),
            prepared: std::mem::ManuallyDrop::new(None),
            mutation_retirement: std::mem::ManuallyDrop::new(Some(request.mutation_retirement)),
            snapshot_retirement: std::mem::ManuallyDrop::new(Some(request.snapshot_retirement)),
                        active: std::mem::ManuallyDrop::new(None),
            factories: std::mem::ManuallyDrop::new(Default::default()),
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }),progress))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for EquationStorePreparation<P, M>
where
    P: Clone + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: protocol::Mutation<P> + semio_framework_value::retirement::RetireOwned + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.refused.is_some() || self.apply_refusal.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()));
        }
        let authority = self.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Equation preparation lost its Store authority"))?;
                let base = self.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Equation preparation lost its exact base root"))?;
        let mutation = self.mutation.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Equation preparation lost its mutation owner"))?;
        let inverse = match mutation.inverse(base.get()) {
                    Ok(inverse) => inverse,
                    Err(error) => { *self.mutation = Some(mutation); return Err(error); }
                };
        let post = match protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
                    Ok(post) => post,
                    Err(error) => {
                        *self.mutation = Some(mutation);
                        *self.inverse = Some(inverse);
                        *self.apply_refusal = Some(error);
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                    }
                };

        let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => { *self.refused = Some((edit, post)); return Err(error); }
                };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default(); let grant = grant.retained_grant();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "preparation close exceeds original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return store::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.prepared.is_some() {
            if self.mutation_retirement.is_none() || self.snapshot_retirement.is_none() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation retains its original installed issuers")); }
            let original = self.prepared.take().expect("observed original prepared candidate");
            let mutations = self.mutation_retirement.take().expect("original mutation issuer");
            let snapshots = self.snapshot_retirement.take().expect("original snapshot issuer");
            return match original.admit_retirement(mutations, snapshots, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original prepared close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation child changed its actual admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original, mutations, snapshots)) => { *self.prepared = Some(original); *self.mutation_retirement = Some(mutations); *self.snapshot_retirement = Some(snapshots); Err(error) },
            };
        }
        if self.mutation.is_some() { return store::artifact_retirement_admit_owned(&mut self.mutation, &mut self.active, child); }
        if self.inverse.is_some() { return store::artifact_retirement_admit_owned(&mut self.inverse, &mut self.active, child); }
        if self.refused.is_some() { return store::artifact_retirement_admit_owned(&mut self.refused, &mut self.active, child); }
        if self.apply_refusal.is_some() { return store::artifact_retirement_admit_owned(&mut self.apply_refusal, &mut self.active, child); }
        if self.base.is_some() { return store::artifact_retirement_admit_owned(&mut self.base, &mut self.active, child); }
        if let Some(authority) = self.authority.take() { return match authority.retire(child) { Ok((owner, progress)) => { *self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "original preparation authority close birth")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "preparation authority changed admitted birth")); } Ok(RetainedCloneStep::Progress(progress)) }, Err((error, original)) => { *self.authority = Some(original); Err(error) } }; }
        if let Some(factory) = self.mutation_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(factory) = self.snapshot_retirement.take() { let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory; self.factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(slot) = self.factories.iter_mut().find(|slot| slot.is_some()) { let factory = slot.as_mut().expect("original preparation factory alias"); let step = factory.step(child)?; let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original preparation factory close")?; if factory.terminal_is_empty() { *slot = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.active.is_none() && self.factories.iter().all(Option::is_none) && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

impl<P, M> Drop for EquationStorePreparation<P, M> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none() && self.inverse.is_none() && self.refused.is_none() && self.apply_refusal.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.active.is_none() && self.factories.iter().all(Option::is_none)), "preparation must retain original owners until supplied-grant terminal closure"); }
}
//#endregion 📬️StorePreparation

//#region 🔖️EquationPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// the registered graph-window configuration owner.
#[derive(Default)]
pub struct EquationPlayApp;

impl ArtifactEditor for EquationPlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children (`text`/`table`/`value`), so every bundle of this
    /// surface opens them through the same roster. The react shell's `loadDocumentPair` sends
    /// `members: []`, so an undeclared-but-derivable child makes the archive closure `Incomplete`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
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
    /// 🧬️ The crate's one loaded-parent child projection (`crate::equation_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::equation_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(EquationStorePreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> ArtifactDisposal<store::ConfigStore<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_disposer())
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
        artifact_schema: "semio.equation/v1",
        factory: "EquationCommandJobFactory",
        factory_type: EquationCommandJobFactory,
        tools: {
            "editEquation" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setAlgorithm" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setDirected" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphViewport" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "editPoints" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setActiveExample" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "addNode" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
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
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Equation command does not match its exact retained tool registration"));
        }
        let extent = equation_command_extent(&request.command, &request.snapshot).ok_or_else(|| Fault::from("equation-command-capacity"))?;
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(EquationRetainedCommandWork::new(tool_id, equation_operation_identity(tool_id, &operation_context), extent));
        let payload = ArtifactRetainedCommandPayload::new(
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
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🌱️ The derivable `notation`/`results`/`computed` members — see
    /// `crate::genesis_equation_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_equation_child_pack(snapshot, slot, child_id)
    
})())
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

    /// 🎯️ Maps a host action id + its staged args onto `EquationCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge every Actions-pane row and
    /// every graph gesture died before reaching `handle`. The three block-shaped payloads
    /// (`graph`/`geometry`/`viewport`) are decoded through `semio_framework_value::FromValue::from_value`, the same codec the
    /// typed channel uses, so the shell needs no staging shim.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<EquationCommand, Fault> {
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        fn decode<T: semio_framework_value::FromValue>(action: &str, args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<T, Fault> {
            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("equation {action} requires a '{key}' block")))?;
            semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid equation {action} '{key}': {error}")))
        }
        match action {
            "editEquation" => Ok(EquationCommand::EditEquation(edit_equation::EditEquation { graph: decode(action, args, "graph")?, geometry: decode(action, args, "geometry")? })),
            "setAlgorithm" => Ok(EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: text_arg(&["algorithm", "value"]).unwrap_or_default(), seed: text_arg(&["seed"]) })),
            "setDirected" => Ok(EquationCommand::SetDirected(set_directed::SetDirected {
                directed: args.and_then(|value| value.get("directed").or_else(|| value.get("value"))).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false),
            })),
            // 🧬️ A staged `json_text` argument arrives as a STRING already holding the JSON document
            // (the `loadDocumentJson.json`/`patchLayer.value` shape); re-stringifying it would wrap the
            // array in quotes and `equation_edit_preflight` would refuse it as a non-array. A
            // structured `DslValue` (an engagement or an MCP caller) still prints normally.
            "nodeGraphEdit" => Ok(EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), |value| value.as_str().map_or_else(|| semio_framework_pack_json::to_json_string(value), str::to_string)),
            })),
            "addNode" => {
                let coordinate = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64).ok_or_else(|| Fault::from(format!("equation addNode requires a numeric '{key}'")));
                Ok(EquationCommand::AddNode(add_node::AddNode { x: coordinate("x")?, y: coordinate("y")? }))
            }
            "nodeGraphViewport" => Ok(EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: decode(action, args, "viewport")? })),
            "editPoints" => Ok(EquationCommand::EditPoints(edit_points::EditPoints { geometry: decode(action, args, "geometry")? })),
            "setActiveExample" => Ok(EquationCommand::SetActiveExample(set_active_example::SetActiveExample {
                example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()),
            })),
            other => Err(Fault::from(format!("equation: unhandled action id {other}"))),
        }
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
    /// `"artifact:out"` replicates `ArtifactApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    fn export_media(port: &str, doc: &ArtifactView<'_, EquationSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let graph = doc.snapshot.graph.clone();
                let overlay = algorithm_overlay(&graph);
                let overlay_json = semio_framework_pack_json::object(overlay.iter().map(|(id, suffix)| (id.clone(), Value::from(suffix.as_str()))));
                let json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("algorithm".to_string(), Value::from(graph.algorithm.as_str())), ("overlay".to_string(), overlay_json)]));
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.equation".into(), json } })
            }
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, EquationSnapshot>, cfg: &ConfigView<'_, NoConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&doc.snapshot.graph.clone(), &graph_window::config::current(cfg).cloned().unwrap_or_default().camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&doc.snapshot.geometry.clone()),
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
        .mutation("editEquation", LocalizedLabel::native("Edit Equation", "Gleichung bearbeiten"))
        .mutation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
        .mutation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .action_with(semio_framework_plugin::ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), semio_framework_plugin::ActionKind::View, "camera"))
        .mutation("editPoints", LocalizedLabel::native("Edit Points", "Punkte bearbeiten"))
        .action_interactive_job("editEquation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setAlgorithm", InteractiveJobClassification::Migrated)
        .action_interactive_job("setDirected", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("editPoints", InteractiveJobClassification::Migrated)
        .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_args("setActiveExample", vec![
            ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])
                .required()
                .default_value(&crate::examples::demo::ID),
        ])
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
        // 🧬️ The equation's own EDIT verb, staged as JSON text — the only document verb whose whole
        // payload the Actions pane can carry (`editEquation`/`editPoints`/`nodeGraphViewport` take
        // structured `<block>` arguments no pane control produces). The default is one `move` gesture
        // record, so the verb is dispatchable, invertible and visible in the graph window
        // straight from the pane instead of only from a canvas engagement.
        .action_args("addNode", vec![
            ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&120.0),
            ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&80.0),
        ])
        .action_args("nodeGraphEdit", vec![
            ActionArgDef::json_text("operations", LocalizedLabel::native("Operations", "Operationen"))
                .required()
                .default_value(&EQUATION_DEFAULT_EDIT_OPERATIONS),
        ])
        // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
        // WORKFLOWS-END-TO-END-TYPED-PORTS) — `equation_io()` (this file's own `🔖️Io` region) is
        // this port information's single source of truth, reused here rather than duplicated.
        .io(equation_io())
        .action_describe("editEquation", LocalizedLabel::native("Applies the supplied graph and point geometry as the concrete node, edge and point changes that differ from the equation; nothing else is touched.", "Wendet den übergebenen Graphen und die Punktgeometrie als die konkreten Knoten-, Kanten- und Punktänderungen an, die von der Gleichung abweichen; sonst wird nichts angefasst."))
        .action_describe("setAlgorithm", LocalizedLabel::native("Chooses the graph algorithm the equation evaluates: topological order, connected components, strongly connected components or breadth-first distances.", "Wählt den Graphalgorithmus, den die Gleichung auswertet: topologische Ordnung, Zusammenhangskomponenten, starke Zusammenhangskomponenten oder Breitensuche-Distanzen."))
        .action_describe("setDirected", LocalizedLabel::native("Sets whether the equation's graph is treated as directed or undirected, which changes every algorithm result.", "Legt fest, ob der Graph der Gleichung gerichtet oder ungerichtet behandelt wird, was jedes Algorithmusergebnis ändert."))
        .action_describe("nodeGraphEdit", LocalizedLabel::native("Applies a JSON list of node-graph rows (move; connect; disconnect; delete) to the equation's graph in one step.", "Wendet eine JSON-Liste von Knotengraph-Zeilen (move; connect; disconnect; delete) in einem Schritt auf den Graphen der Gleichung an."))
        .action_describe("addNode", LocalizedLabel::native("Adds one node to the equation's graph at x, y, named with the first free id.", "Fügt dem Graphen der Gleichung einen Knoten an x, y hinzu, benannt mit der ersten freien Id."))
        .action_describe("editPoints", LocalizedLabel::native("Applies the supplied points as the point repositionings, insertions and removals that differ from the equation's geometry.", "Wendet die übergebenen Punkte als die Punktverschiebungen, -einfügungen und -entfernungen an, die von der Geometrie der Gleichung abweichen."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole equation document with the bundled demo example; any other example id changes nothing.", "Ersetzt das gesamte Gleichungsdokument durch das mitgelieferte Demo-Beispiel; jede andere Beispiel-Id ändert nichts."))
        .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
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

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
