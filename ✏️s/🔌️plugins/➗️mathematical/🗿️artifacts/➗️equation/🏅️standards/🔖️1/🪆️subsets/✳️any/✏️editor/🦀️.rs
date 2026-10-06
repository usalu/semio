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
use semio_framework_plugin::Dialect;
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
        "setDocument" as "set-artifact" => set_artifact::SetArtifact,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setPoints" as "set-points" => set_points::SetPoints,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const EQUATION_TOOL_IDS: &[&str] = &["setDocument", "setAlgorithm", "setDirected", "nodeGraphEdit", "nodeGraphViewport", "setPoints", "setActiveExample", "addNode"];
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
    ArtifactToolPublicationContract { tool_id: "setDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAlgorithm", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setDirected", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setPoints", lanes: &[ArtifactToolPublicationLane::Artifact] },
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
        EquationCommand::SetPoints(payload) if payload.geometry.points.len() <= EQUATION_MAX_POINTS => 2_usize.checked_add(payload.geometry.points.len())?,
        EquationCommand::SetPoints(_) => return None,
        EquationCommand::SetArtifact(payload)
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
        EquationCommand::SetArtifact(_) => return None,
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
            leaves: Vec::new(),
            gesture: None,
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

    fn finish(&mut self, command: &EquationCommand, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>, operation: &AppOperationContext) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
        use crate::standards::v1::subsets::geometry::schema::mutations::replace_points::ReplacePoints;
        use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
        Ok(match command {
            EquationCommand::SetActiveExample(_) => return Err(Fault::from("equation-set-active-example-is-not-a-phased-command")),
            EquationCommand::SetAlgorithm(_) | EquationCommand::SetDirected(_) | EquationCommand::AddNode(_) => Emit::mutations(std::mem::take(&mut self.leaves)),
            EquationCommand::NodeGraphEdit(_) => node_graph_edit::equation_edit_emit(&operation.authoring_seed, self.gesture.take().as_deref(), std::mem::take(&mut self.leaves)),
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
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation } = *input;
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
            EquationWorkPhase::Finish => self.finish(command, context, operation).map(ArtifactCommandWorkStep::Complete),
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
        if let Some(leaf) = self.leaves.last() {
            let bytes = equation_leaf_retained_bytes(leaf);
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.leaves.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(step) = Self::close_vec_capacity(&mut self.leaves, maximum_items, maximum_bytes) {
            return step;
        }
        if let Some(gesture) = self.gesture.as_ref() {
            let bytes = gesture.capacity();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.gesture = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
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
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for EquationStorePreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, mutation: &M, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Equation Store preparation rejected its lane".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<P, M>(mutation, store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
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
        let inverse = mutation.inverse(base.get()).map_err(semio_framework_value::ValueError::into_message)?;
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Equation preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(authority.next_edit(mutation, inverse), Arc::new(post))?;
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

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "Equation preparation could not return its exact base root"));
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
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
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
        artifact_schema: "semio.equation/v1",
        factory: "EquationCommandJobFactory",
        factory_type: EquationCommandJobFactory,
        tools: {
            "setDocument" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setAlgorithm" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setDirected" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphViewport" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setPoints" => ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
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
            authoring_seed: request.authoring_seed.clone(),
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
            "setDocument" => Ok(EquationCommand::SetArtifact(set_artifact::SetArtifact { graph: decode(action, args, "graph")?, geometry: decode(action, args, "geometry")? })),
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
            "setPoints" => Ok(EquationCommand::SetPoints(set_points::SetPoints { geometry: decode(action, args, "geometry")? })),
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
        .mutation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"))
        .action_destructive("setDocument")
        .mutation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
        .mutation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .action_with(semio_framework_plugin::ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), semio_framework_plugin::ActionKind::View, "camera"))
        .mutation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
        .action_interactive_job("setDocument", InteractiveJobClassification::Migrated)
        .action_interactive_job("setAlgorithm", InteractiveJobClassification::Migrated)
        .action_interactive_job("setDirected", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setPoints", InteractiveJobClassification::Migrated)
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
        // payload the Actions pane can carry (`setDocument`/`setPoints`/`nodeGraphViewport` take
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
        .action_describe("setDocument", LocalizedLabel::native("Replaces the equation's whole graph and point geometry with the supplied ones; parts that differ are overwritten.", "Ersetzt den gesamten Graphen und die Punktgeometrie der Gleichung durch die übergebenen; abweichende Teile werden überschrieben."))
        .action_describe("setAlgorithm", LocalizedLabel::native("Chooses the graph algorithm the equation evaluates: topological order, connected components, strongly connected components or breadth-first distances.", "Wählt den Graphalgorithmus, den die Gleichung auswertet: topologische Ordnung, Zusammenhangskomponenten, starke Zusammenhangskomponenten oder Breitensuche-Distanzen."))
        .action_describe("setDirected", LocalizedLabel::native("Sets whether the equation's graph is treated as directed or undirected, which changes every algorithm result.", "Legt fest, ob der Graph der Gleichung gerichtet oder ungerichtet behandelt wird, was jedes Algorithmusergebnis ändert."))
        .action_describe("nodeGraphEdit", LocalizedLabel::native("Applies a JSON list of node-graph rows (move; connect; disconnect; delete) to the equation's graph in one step.", "Wendet eine JSON-Liste von Knotengraph-Zeilen (move; connect; disconnect; delete) in einem Schritt auf den Graphen der Gleichung an."))
        .action_describe("addNode", LocalizedLabel::native("Adds one node to the equation's graph at x, y, named with the first free id.", "Fügt dem Graphen der Gleichung einen Knoten an x, y hinzu, benannt mit der ersten freien Id."))
        .action_describe("setPoints", LocalizedLabel::native("Replaces the point set of the equation's geometry with the supplied points.", "Ersetzt die Punktmenge der Geometrie der Gleichung durch die übergebenen Punkte."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole equation document with the bundled demo example; any other example id changes nothing.", "Ersetzt das gesamte Gleichungsdokument durch das mitgelieferte Demo-Beispiel; jede andere Beispiel-Id ändert nichts."))
        .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_destructive("setPoints")
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
