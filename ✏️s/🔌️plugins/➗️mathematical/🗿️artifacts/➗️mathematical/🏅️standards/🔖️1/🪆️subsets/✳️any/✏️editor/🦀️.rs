//! 🧮️ Mathematical editor — `MathematicalPlayApp`'s `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1), the aggregated command enum and
//! the manifest stitch. B1: the pure-trait pilot for this plugin — `MathematicalPlayApp` is a unit
//! struct; the former `MathPlayRuntime` app-struct `RefCell` (the node-graph viewport camera) now lives in
//! `crate::editor::mathematical::config::MathematicalConfig`, written via `MathematicalConfigMutation`s (real
//! `backwards`, no ad hoc inverse tracking); every action dispatches through the single typed
//! `MathematicalCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config/🦀️.rs`. Shared compute with more than one
//! consumer across the taxonomy tree (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES dissolved
//! the former artifact-tree `⚙️engine`) lives HERE — `🔖️Io`, `🔖️Scene`, `🔖️GraphAlgorithms`, `🔖️Geometry` — since
//! an artifact is a `🧬️schema` + `🚪️io` system only, never an engine; behaviour belongs to the app.
//! This file is a routing table: `handle` → `MathematicalCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.
//!
//! The sibling read-only surface (`👁️viewer/🦀️.rs`) never imports from this module — see
//! that file's own doc header.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph, MathematicalSnapshot, MATHEMATICAL_DIALECT, MATH_DOCUMENT_SCHEMA};
use crate::editor::mathematical::commands::set_artifact;
use crate::editor::mathematical::commands::set_locale;
use crate::editor::mathematical::commands::set_points;
use crate::editor::mathematical::commands::{node_graph_edit, node_graph_viewport, set_algorithm, set_directed};
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::editor::mathematical::modes::edit;
use crate::editor::mathematical::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::editor::mathematical::presence::{MathematicalPresence, MathematicalPresenceMutation};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ui_text, ActionArgDef, ActionArgOption, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence,
};
use pack::json::{self, Value};
use std::collections::BTreeSet;
use std::sync::Arc;
use store::ArtifactPack;
use store::EngineHandles;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord};

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "mathematical-play";
pub use geometry_window::MATH_PLAY_BODY_GEOMETRY;
pub use graph_window::MATH_PLAY_BODY_GRAPH;
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_mathematical_app` declares via `.artifact_kind(...)` (`computation.mathematical`), plus one
/// extra output port: `result:out`, the current graph+geometry projection as a generic data value
/// (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub async fn mathematical_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: MATH_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("computation.mathematical".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.mathematical".into(), name: "Mathematical".into(), dimension: "graph".into(), component_kind: "mathematical".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Scene
/// 🖼️ An empty `UiComponentSceneNode` shell for a body key, ready for its `node_graph`/`canvas_2d` field
/// to be filled in — shared by both `🎭️modes/✏️edit/🪟️windows/*` renderers.
pub async fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: MATH_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}
//#endregion 🔖️Scene

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub async fn algorithm_overlay(graph: &MathematicalGraph) -> std::collections::HashMap<String, String> {
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

pub async fn workflow_json(graph: &MathematicalGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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
pub async fn geometry_layers_json(geometry: &MathematicalGeometry) -> String {
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
    /// 🎯️ `MathematicalPlayApp::Command` — the SOLE dispatch surface for mathematical's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum MathematicalCommand for MathematicalSnapshot, MathematicalMutation, MathematicalConfig, MathematicalConfigMutation {
        "setDocument" as "set-artifact" => set_artifact::SetArtifact,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setPoints" as "set-points" => set_points::SetPoints,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const MATHEMATICAL_TOOL_IDS: &[&str] = &["setDocument", "setAlgorithm", "setDirected", "nodeGraphEdit", "nodeGraphViewport", "setPoints", "setLocale"];
const MATHEMATICAL_RETAINED_PAYLOAD_SCHEMA: &str = "semio.mathematical/v1.tool-command.v1";
const MATHEMATICAL_RETAINED_RAW_BYTES: usize = 65_536;
const MATHEMATICAL_RETAINED_WORK_ITEMS: usize = 65_536;
const MATHEMATICAL_MAX_NODES: usize = 256;
const MATHEMATICAL_MAX_EDGES: usize = 512;
const MATHEMATICAL_MAX_POINTS: usize = 1_024;
const MATHEMATICAL_MAX_EDIT_JSON_BYTES: usize = 8_192;
const MATHEMATICAL_MAX_EDIT_OPERATIONS: usize = 16;
const MATHEMATICAL_MAX_DELETE_IDS: usize = 256;
const MATHEMATICAL_MAX_TEXT_BYTES: usize = 256;
const MATHEMATICAL_MAX_LOCALE_BYTES: usize = 64;

const MATHEMATICAL_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAlgorithm", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setDirected", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setPoints", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn mathematical_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(MATHEMATICAL_RETAINED_RAW_BYTES, 2_048, 1, MATHEMATICAL_RETAINED_WORK_ITEMS, 7_500, 1, 1)
}

fn mathematical_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

fn mathematical_operation_identity(tool_id: &str, operation: &AppOperationContext) -> u64 {
    let mut identity = mathematical_tool_identity(tool_id);
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

fn mathematical_graph_shape_admitted(graph: &MathematicalGraph) -> bool {
    graph.nodes.len() <= MATHEMATICAL_MAX_NODES && graph.edges.len() <= MATHEMATICAL_MAX_EDGES && graph.algorithm.len() <= MATHEMATICAL_MAX_TEXT_BYTES && graph.algorithm_seed.as_ref().is_none_or(|seed| seed.len() <= MATHEMATICAL_MAX_TEXT_BYTES)
}

fn mathematical_edit_preflight(payload: &node_graph_edit::NodeGraphEdit) -> Option<usize> {
    if payload.operations_json.len() > MATHEMATICAL_MAX_EDIT_JSON_BYTES {
        return None;
    }
    let values = json::parse(&payload.operations_json).ok().and_then(|value| value.as_array().map(<[Value]>::to_vec))?;
    if values.len() > MATHEMATICAL_MAX_EDIT_OPERATIONS {
        return None;
    }
    for value in &values {
        MathematicalEditOperation::from_value(value).ok()?;
    }
    Some(values.len())
}

fn mathematical_command_extent(command: &MathematicalCommand, snapshot: &MathematicalSnapshot) -> Option<usize> {
    let scene = crate::artifacts::mathematical::mathematical_scene_owner(snapshot)?;
    if !mathematical_graph_shape_admitted(&scene.graph) || scene.geometry.points.len() > MATHEMATICAL_MAX_POINTS {
        return None;
    }
    let extent = match command {
        MathematicalCommand::NodeGraphViewport(_) => 1,
        MathematicalCommand::SetLocale(payload) if payload.value.len() <= MATHEMATICAL_MAX_LOCALE_BYTES => 1,
        MathematicalCommand::SetLocale(_) => return None,
        MathematicalCommand::SetAlgorithm(payload) if payload.algorithm.len() <= MATHEMATICAL_MAX_TEXT_BYTES && payload.seed.as_ref().is_none_or(|seed| seed.len() <= MATHEMATICAL_MAX_TEXT_BYTES) => {
            2_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?
        }
        MathematicalCommand::SetAlgorithm(_) => return None,
        MathematicalCommand::SetDirected(_) => 2_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?,
        MathematicalCommand::SetPoints(payload) if payload.geometry.points.len() <= MATHEMATICAL_MAX_POINTS => 2_usize.checked_add(payload.geometry.points.len())?,
        MathematicalCommand::SetPoints(_) => return None,
        MathematicalCommand::SetArtifact(payload)
            if payload.graph.retained_node_count() <= MATHEMATICAL_MAX_NODES
                && payload.graph.retained_edge_count() <= MATHEMATICAL_MAX_EDGES
                && payload.geometry.points.len() <= MATHEMATICAL_MAX_POINTS
                && payload.graph.retained_metadata().1.len() <= MATHEMATICAL_MAX_TEXT_BYTES
                && payload.graph.retained_metadata().2.is_none_or(|seed| seed.len() <= MATHEMATICAL_MAX_TEXT_BYTES) =>
        {
            2_usize.checked_add(payload.graph.retained_node_count())?.checked_add(payload.graph.retained_edge_count())?.checked_add(payload.geometry.points.len())?
        }
        MathematicalCommand::SetArtifact(_) => return None,
        MathematicalCommand::NodeGraphEdit(payload) => {
            let operation_count = mathematical_edit_preflight(payload)?;
            let delete_extent = operation_count.checked_mul(MATHEMATICAL_MAX_DELETE_IDS.checked_add(scene.graph.nodes.len().checked_mul(2)?)?.checked_add(scene.graph.edges.len().checked_mul(2)?)?.checked_add(8)?)?;
            4_usize.checked_add(scene.graph.nodes.len())?.checked_add(scene.graph.edges.len())?.checked_add(payload.operations_json.len())?.checked_add(delete_extent)?
        }
    };
    (extent != 0 && extent <= MATHEMATICAL_RETAINED_WORK_ITEMS).then_some(extent)
}

#[derive(Clone)]
enum MathematicalEditOperation {
    AddNode { x: f64, y: f64 },
    Move { node_id: String, x: f64, y: f64 },
    Connect { source: String, target: String },
    DeleteSelection { ids: Vec<String> },
    Ignore,
}

impl MathematicalEditOperation {
    fn from_value(value: &Value) -> Result<Self, Fault> {
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or_default();
        Ok(match text("operation") {
            "addNode" => Self::AddNode { x: value.get("x").and_then(Value::as_f64).unwrap_or(0.0), y: value.get("y").and_then(Value::as_f64).unwrap_or(0.0) },
            "move" => {
                let node_id = text("nodeId");
                if node_id.len() > MATHEMATICAL_MAX_TEXT_BYTES {
                    return Err(Fault::from("mathematical-edit-node-id-capacity"));
                }
                match (value.get("x").and_then(Value::as_f64), value.get("y").and_then(Value::as_f64)) {
                    (Some(x), Some(y)) if !node_id.is_empty() => Self::Move { node_id: node_id.to_string(), x, y },
                    _ => Self::Ignore,
                }
            }
            "connect" => {
                let source = text("sourceNodeId");
                let target = text("targetNodeId");
                if source.len() > MATHEMATICAL_MAX_TEXT_BYTES || target.len() > MATHEMATICAL_MAX_TEXT_BYTES {
                    return Err(Fault::from("mathematical-edit-edge-id-capacity"));
                }
                if source.is_empty() || target.is_empty() {
                    Self::Ignore
                } else {
                    Self::Connect { source: source.to_string(), target: target.to_string() }
                }
            }
            "deleteSelection" => {
                let Some(values) = value.get("nodeIds").and_then(Value::as_array) else { return Ok(Self::Ignore) };
                if values.len() > MATHEMATICAL_MAX_DELETE_IDS {
                    return Err(Fault::from("mathematical-edit-delete-id-capacity"));
                }
                let mut ids = Vec::new();
                ids.try_reserve_exact(values.len()).map_err(|_| Fault::from("mathematical-edit-delete-id-reserve"))?;
                for value in values {
                    let Some(id) = value.as_str() else { return Ok(Self::Ignore) };
                    if id.len() > MATHEMATICAL_MAX_TEXT_BYTES {
                        return Err(Fault::from("mathematical-edit-delete-id-capacity"));
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
            Self::AddNode { .. } | Self::Ignore => std::mem::size_of::<Self>(),
            Self::Move { node_id, .. } => std::mem::size_of::<Self>() + node_id.capacity(),
            Self::Connect { source, target } => std::mem::size_of::<Self>() + source.capacity() + target.capacity(),
            Self::DeleteSelection { ids } => std::mem::size_of::<Self>() + ids.capacity() * std::mem::size_of::<String>() + ids.iter().map(|id| id.capacity()).sum::<usize>(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MathematicalWorkPhase {
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
enum MathematicalOperationPhase {
    #[default]
    Start,
    MoveNodes,
    BuildDeleteIds,
    DeleteNodes,
    ReverseNodes,
    DeleteEdges,
    ReverseEdges,
}

struct MathematicalRetainedCommandWork {
    tool_id: &'static str,
    operation_identity: u64,
    extent: usize,
    phase: MathematicalWorkPhase,
    item_cursor: usize,
    cursor: usize,
    digest: u64,
    replay_target: Option<(usize, u64)>,
    graph: Option<MathematicalGraph>,
    points: Vec<crate::artifacts::mathematical::MathematicalPoint>,
    operations: Vec<MathematicalEditOperation>,
    operation_phase: MathematicalOperationPhase,
    operation_cursor: usize,
    rewrite_nodes: Vec<crate::artifacts::mathematical::MathematicalNode>,
    rewrite_edges: Vec<crate::artifacts::mathematical::MathematicalEdge>,
    delete_ids: BTreeSet<String>,
    graph_changed: bool,
    geometry_changed: bool,
    closing: bool,
}

impl MathematicalRetainedCommandWork {
    fn new(tool_id: &'static str, operation_identity: u64, extent: usize) -> Self {
        Self {
            tool_id,
            operation_identity,
            extent,
            phase: MathematicalWorkPhase::Initialize,
            item_cursor: 0,
            cursor: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            replay_target: None,
            graph: None,
            points: Vec::new(),
            operations: Vec::new(),
            operation_phase: MathematicalOperationPhase::Start,
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
        self.cursor = self.cursor.checked_add(1).ok_or_else(|| Fault::from("mathematical-work-cursor-overflow"))?;
        if self.cursor > self.extent {
            return Err(Fault::from("mathematical-work-extent-overflow"));
        }
        if let Some((target, expected)) = self.replay_target {
            if self.cursor == target {
                if self.digest != expected {
                    return Err(Fault::from("mathematical-work-replay-drift"));
                }
                self.replay_target = None;
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "mathematical-command-replay", preview: br#"{"en":"Restoring Mathematical command","de":"Mathematik-Befehl wird wiederhergestellt"}"# });
        }
        Ok(ArtifactCommandWorkStep::Progress { stage, preview: br#"{"en":"Preparing Mathematical command","de":"Mathematik-Befehl wird vorbereitet"}"# })
    }

    fn source_scene(snapshot: &MathematicalSnapshot) -> Result<Arc<crate::artifacts::mathematical::MathematicalWorkingScene>, Fault> {
        crate::artifacts::mathematical::mathematical_scene_owner(snapshot).ok_or_else(|| Fault::from("mathematical-command-scene-unresolved"))
    }

    fn initialize(&mut self, command: &MathematicalCommand, snapshot: &MathematicalSnapshot) -> Result<(), Fault> {
        let source = Self::source_scene(snapshot)?;
        match command {
            MathematicalCommand::SetAlgorithm(payload) => {
                let mut graph = MathematicalGraph { directed: source.graph.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: payload.algorithm.clone(), algorithm_seed: payload.seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("mathematical-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("mathematical-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = MathematicalWorkPhase::Nodes;
            }
            MathematicalCommand::SetDirected(payload) => {
                let mut graph = MathematicalGraph { directed: payload.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: source.graph.algorithm.clone(), algorithm_seed: source.graph.algorithm_seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("mathematical-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("mathematical-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = MathematicalWorkPhase::Nodes;
            }
            MathematicalCommand::NodeGraphEdit(_) => {
                let mut graph = MathematicalGraph { directed: source.graph.directed, nodes: Vec::new(), edges: Vec::new(), algorithm: source.graph.algorithm.clone(), algorithm_seed: source.graph.algorithm_seed.clone() };
                graph.nodes.try_reserve_exact(source.graph.nodes.len()).map_err(|_| Fault::from("mathematical-command-node-reserve"))?;
                graph.edges.try_reserve_exact(source.graph.edges.len()).map_err(|_| Fault::from("mathematical-command-edge-reserve"))?;
                self.graph = Some(graph);
                self.phase = MathematicalWorkPhase::Nodes;
            }
            MathematicalCommand::SetArtifact(payload) => {
                let (directed, algorithm, seed) = payload.graph.retained_metadata();
                let mut graph = MathematicalGraph { directed, nodes: Vec::new(), edges: Vec::new(), algorithm: algorithm.to_string(), algorithm_seed: seed.map(str::to_string) };
                graph.nodes.try_reserve_exact(payload.graph.retained_node_count()).map_err(|_| Fault::from("mathematical-command-node-reserve"))?;
                graph.edges.try_reserve_exact(payload.graph.retained_edge_count()).map_err(|_| Fault::from("mathematical-command-edge-reserve"))?;
                self.graph_changed = directed != source.graph.directed
                    || algorithm != source.graph.algorithm
                    || seed != source.graph.algorithm_seed.as_deref()
                    || payload.graph.retained_node_count() != source.graph.nodes.len()
                    || payload.graph.retained_edge_count() != source.graph.edges.len();
                self.geometry_changed = payload.geometry.points.len() != source.geometry.points.len();
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("mathematical-command-point-reserve"))?;
                self.graph = Some(graph);
                self.phase = MathematicalWorkPhase::Nodes;
            }
            MathematicalCommand::SetPoints(payload) => {
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("mathematical-command-point-reserve"))?;
                self.phase = MathematicalWorkPhase::Points;
            }
            MathematicalCommand::NodeGraphViewport(_) | MathematicalCommand::SetLocale(_) => self.phase = MathematicalWorkPhase::Finish,
        }
        Ok(())
    }

    fn finish(&mut self, command: &MathematicalCommand) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
        use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
        use crate::artifacts::mathematical::schema::mutations::replace_points::mutation::ReplacePoints;
        Ok(match command {
            MathematicalCommand::SetAlgorithm(_) => Emit::commit(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))? })], "setAlgorithm"),
            MathematicalCommand::SetDirected(_) => Emit::mutations(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))? })]),
            MathematicalCommand::NodeGraphEdit(_) if self.graph_changed => Emit::mutations(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))? })]),
            MathematicalCommand::NodeGraphEdit(_) => Emit::default(),
            MathematicalCommand::SetPoints(_) => Emit::mutations(vec![MathematicalMutation::ReplacePoints(ReplacePoints { points: std::mem::take(&mut self.points) })]),
            MathematicalCommand::SetArtifact(_) => {
                let mut mutations = Vec::new();
                if self.graph_changed {
                    mutations.push(MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: self.graph.take().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))? }));
                }
                if self.geometry_changed {
                    mutations.push(MathematicalMutation::ReplacePoints(ReplacePoints { points: std::mem::take(&mut self.points) }));
                }
                Emit::mutations(mutations)
            }
            MathematicalCommand::NodeGraphViewport(payload) => Emit::config(vec![MathematicalConfigMutation::SetCamera { camera: payload.camera.clone() }]),
            MathematicalCommand::SetLocale(payload) => Emit::config(vec![MathematicalConfigMutation::SetLocale { value: payload.value.clone() }]),
        })
    }

    fn close_vec_capacity<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<InteractiveJobCloseStep> {
        if !values.is_empty() {
            return None;
        }
        let bytes = values.capacity().saturating_mul(std::mem::size_of::<T>());
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
        self.operation_phase = MathematicalOperationPhase::Start;
    }
}

impl ArtifactCommandWork<EditorApp<MathematicalPlayApp>> for MathematicalRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.operation_identity ^ (self.extent as u64).rotate_left(17)
    }

    fn extent(
        &self,
        command: &MathematicalCommand,
        snapshot: &MathematicalSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<MathematicalPlayApp>>>,
    ) -> Option<usize> {
        let extent = mathematical_command_extent(command, snapshot)?;
        (extent == self.extent).then_some(extent)
    }

    fn step(
        &mut self,
        command: &MathematicalCommand,
        snapshot: &MathematicalSnapshot,
        _config: &MathematicalConfig,
        _history: &semio_framework_plugin::HistoryView,
        _interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<MathematicalPlayApp>>>,
        _operation: &AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<EditorApp<MathematicalPlayApp>>, Fault> {
        if mathematical_command_extent(command, snapshot) != Some(self.extent) || self.cursor > self.extent {
            return Err(Fault::from("mathematical-command-extent-drift"));
        }
        let source = Self::source_scene(snapshot)?;
        match self.phase {
            MathematicalWorkPhase::Initialize => {
                self.initialize(command, snapshot)?;
                self.progress::<EditorApp<MathematicalPlayApp>>(self.tool_id.as_bytes(), "mathematical-command-initialize")
            }
            MathematicalWorkPhase::Nodes => {
                let (count, node) = match command {
                    MathematicalCommand::SetArtifact(payload) => (payload.graph.retained_node_count(), payload.graph.retained_node(self.item_cursor).cloned()),
                    _ => (source.graph.nodes.len(), source.graph.nodes.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = MathematicalWorkPhase::Edges;
                    return self.progress::<EditorApp<MathematicalPlayApp>>(b"nodes-complete", "mathematical-command-node-boundary");
                }
                let node = node.ok_or_else(|| Fault::from("mathematical-command-node-cursor"))?;
                if node.id.len() > MATHEMATICAL_MAX_TEXT_BYTES || node.label.len() > MATHEMATICAL_MAX_TEXT_BYTES {
                    return Err(Fault::from("mathematical-command-node-text-capacity"));
                }
                if let MathematicalCommand::SetArtifact(_) = command {
                    self.graph_changed |= source.graph.nodes.get(self.item_cursor) != Some(&node);
                }
                self.observe(node.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?.nodes.push(node);
                self.item_cursor += 1;
                self.progress::<EditorApp<MathematicalPlayApp>>(&self.item_cursor.to_le_bytes(), "mathematical-command-node")
            }
            MathematicalWorkPhase::Edges => {
                let (count, edge) = match command {
                    MathematicalCommand::SetArtifact(payload) => (payload.graph.retained_edge_count(), (self.item_cursor < payload.graph.retained_edge_count()).then(|| payload.graph.retained_edge(self.item_cursor)).transpose().map_err(Fault::from)?),
                    _ => (source.graph.edges.len(), source.graph.edges.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = match command {
                        MathematicalCommand::SetArtifact(_) => MathematicalWorkPhase::Points,
                        MathematicalCommand::NodeGraphEdit(_) => MathematicalWorkPhase::JsonBytes,
                        _ => MathematicalWorkPhase::Finish,
                    };
                    return self.progress::<EditorApp<MathematicalPlayApp>>(b"edges-complete", "mathematical-command-edge-boundary");
                }
                let edge = edge.ok_or_else(|| Fault::from("mathematical-command-edge-cursor"))?;
                if edge.id.len() > MATHEMATICAL_MAX_TEXT_BYTES || edge.source.len() > MATHEMATICAL_MAX_TEXT_BYTES || edge.target.len() > MATHEMATICAL_MAX_TEXT_BYTES {
                    return Err(Fault::from("mathematical-command-edge-text-capacity"));
                }
                if let MathematicalCommand::SetArtifact(_) = command {
                    self.graph_changed |= source.graph.edges.get(self.item_cursor) != Some(&edge);
                }
                self.observe(edge.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?.edges.push(edge);
                self.item_cursor += 1;
                self.progress::<EditorApp<MathematicalPlayApp>>(&self.item_cursor.to_le_bytes(), "mathematical-command-edge")
            }
            MathematicalWorkPhase::Points => {
                let points = match command {
                    MathematicalCommand::SetArtifact(payload) => &payload.geometry.points,
                    MathematicalCommand::SetPoints(payload) => &payload.geometry.points,
                    _ => return Err(Fault::from("mathematical-command-point-phase")),
                };
                if self.item_cursor >= points.len() {
                    self.item_cursor = 0;
                    self.phase = MathematicalWorkPhase::Finish;
                    return self.progress::<EditorApp<MathematicalPlayApp>>(b"points-complete", "mathematical-command-point-boundary");
                }
                let point = points[self.item_cursor].clone();
                if matches!(command, MathematicalCommand::SetArtifact(_)) {
                    self.geometry_changed |= source.geometry.points.get(self.item_cursor) != Some(&point);
                }
                self.observe(&point.x.to_le_bytes());
                self.observe(&point.y.to_le_bytes());
                self.points.push(point);
                self.item_cursor += 1;
                self.progress::<EditorApp<MathematicalPlayApp>>(&self.item_cursor.to_le_bytes(), "mathematical-command-point")
            }
            MathematicalWorkPhase::JsonBytes => {
                let MathematicalCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("mathematical-command-json-phase")) };
                if self.item_cursor >= payload.operations_json.len() {
                    self.item_cursor = 0;
                    self.phase = MathematicalWorkPhase::JsonDecode;
                    return self.progress::<EditorApp<MathematicalPlayApp>>(b"json-complete", "mathematical-command-json-boundary");
                }
                let byte = payload.operations_json.as_bytes()[self.item_cursor];
                self.item_cursor += 1;
                self.progress::<EditorApp<MathematicalPlayApp>>(&[byte], "mathematical-command-json-byte")
            }
            MathematicalWorkPhase::JsonDecode => {
                let MathematicalCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("mathematical-command-json-decode")) };
                let values = json::parse(&payload.operations_json).ok().and_then(|value| value.as_array().map(<[Value]>::to_vec)).unwrap_or_default();
                if values.len() > MATHEMATICAL_MAX_EDIT_OPERATIONS {
                    return Err(Fault::from("mathematical-command-operation-capacity"));
                }
                self.operations.try_reserve_exact(values.len()).map_err(|_| Fault::from("mathematical-command-operation-reserve"))?;
                for value in &values {
                    self.operations.push(MathematicalEditOperation::from_value(value)?);
                }
                self.item_cursor = 0;
                self.phase = MathematicalWorkPhase::Operations;
                self.progress::<EditorApp<MathematicalPlayApp>>(&(values.len() as u64).to_le_bytes(), "mathematical-command-json-decode")
            }
            MathematicalWorkPhase::Operations => {
                if self.item_cursor >= self.operations.len() {
                    self.item_cursor = 0;
                    self.phase = MathematicalWorkPhase::Finish;
                    return self.progress::<EditorApp<MathematicalPlayApp>>(b"operations-complete", "mathematical-command-operation-boundary");
                }
                let operation = self.operations[self.item_cursor].clone();
                match (&operation, self.operation_phase) {
                    (_, MathematicalOperationPhase::Start) => match &operation {
                        MathematicalEditOperation::AddNode { x, y } => {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(crate::artifacts::mathematical::MathematicalNode { label: id.to_uppercase(), id, x: *x, y: *y });
                            self.graph_changed = true;
                            self.advance_operation();
                        }
                        MathematicalEditOperation::Move { .. } => self.operation_phase = MathematicalOperationPhase::MoveNodes,
                        MathematicalEditOperation::Connect { source, target } => {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
                            let id = format!("e{}", graph.edges.len());
                            graph.edges.push(crate::artifacts::mathematical::MathematicalEdge { id, source: source.clone(), target: target.clone() });
                            self.graph_changed = true;
                            self.advance_operation();
                        }
                        MathematicalEditOperation::DeleteSelection { .. } => self.operation_phase = MathematicalOperationPhase::BuildDeleteIds,
                        MathematicalEditOperation::Ignore => self.advance_operation(),
                    },
                    (MathematicalEditOperation::Move { node_id, x, y }, MathematicalOperationPhase::MoveNodes) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
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
                    (MathematicalEditOperation::DeleteSelection { ids }, MathematicalOperationPhase::BuildDeleteIds) => {
                        if self.operation_cursor < ids.len() {
                            self.delete_ids.insert(ids[self.operation_cursor].clone());
                            self.operation_cursor += 1;
                        } else {
                            let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
                            self.rewrite_nodes = std::mem::take(&mut graph.nodes);
                            graph.nodes.try_reserve_exact(self.rewrite_nodes.len()).map_err(|_| Fault::from("mathematical-command-node-rewrite-reserve"))?;
                            self.operation_cursor = 0;
                            self.operation_phase = MathematicalOperationPhase::DeleteNodes;
                        }
                    }
                    (MathematicalEditOperation::DeleteSelection { .. }, MathematicalOperationPhase::DeleteNodes) => {
                        if let Some(node) = self.rewrite_nodes.pop() {
                            if self.delete_ids.contains(&node.id) {
                                self.graph_changed = true;
                            } else {
                                self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?.nodes.push(node);
                            }
                        } else {
                            self.operation_cursor = 0;
                            self.operation_phase = MathematicalOperationPhase::ReverseNodes;
                        }
                    }
                    (MathematicalEditOperation::DeleteSelection { .. }, MathematicalOperationPhase::ReverseNodes) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
                        if self.operation_cursor < graph.nodes.len() / 2 {
                            let last = graph.nodes.len() - 1 - self.operation_cursor;
                            graph.nodes.swap(self.operation_cursor, last);
                            self.operation_cursor += 1;
                        } else {
                            self.rewrite_edges = std::mem::take(&mut graph.edges);
                            graph.edges.try_reserve_exact(self.rewrite_edges.len()).map_err(|_| Fault::from("mathematical-command-edge-rewrite-reserve"))?;
                            self.operation_cursor = 0;
                            self.operation_phase = MathematicalOperationPhase::DeleteEdges;
                        }
                    }
                    (MathematicalEditOperation::DeleteSelection { .. }, MathematicalOperationPhase::DeleteEdges) => {
                        if let Some(edge) = self.rewrite_edges.pop() {
                            if self.delete_ids.contains(&edge.source) || self.delete_ids.contains(&edge.target) {
                                self.graph_changed = true;
                            } else {
                                self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?.edges.push(edge);
                            }
                        } else {
                            self.operation_cursor = 0;
                            self.operation_phase = MathematicalOperationPhase::ReverseEdges;
                        }
                    }
                    (MathematicalEditOperation::DeleteSelection { .. }, MathematicalOperationPhase::ReverseEdges) => {
                        let graph = self.graph.as_mut().ok_or_else(|| Fault::from("mathematical-command-graph-owner"))?;
                        if self.operation_cursor < graph.edges.len() / 2 {
                            let last = graph.edges.len() - 1 - self.operation_cursor;
                            graph.edges.swap(self.operation_cursor, last);
                            self.operation_cursor += 1;
                        } else {
                            self.delete_ids.clear();
                            self.advance_operation();
                        }
                    }
                    _ => return Err(Fault::from("mathematical-command-operation-phase")),
                }
                self.progress::<EditorApp<MathematicalPlayApp>>(&self.item_cursor.to_le_bytes(), "mathematical-command-operation")
            }
            MathematicalWorkPhase::Finish => self.finish(command).map(ArtifactCommandWorkStep::Complete),
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 40 {
            return Err(Fault::from("mathematical-command-checkpoint-capacity"));
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
            return Err(Fault::from("mathematical-command-checkpoint-invalid"));
        }
        let identity = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("mathematical-command-checkpoint-identity"))?);
        let extent = u64::from_le_bytes(checkpoint[32..40].try_into().map_err(|_| Fault::from("mathematical-command-checkpoint-extent"))?);
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("mathematical-command-checkpoint-cursor"))?);
        if identity != self.operation_identity || extent != self.extent as u64 || cursor > self.extent as u64 {
            return Err(Fault::from("mathematical-command-checkpoint-mismatch"));
        }
        self.phase = MathematicalWorkPhase::Initialize;
        self.item_cursor = 0;
        self.operation_cursor = 0;
        self.operation_phase = MathematicalOperationPhase::Start;
        self.cursor = 0;
        self.digest = 0xcbf2_9ce4_8422_2325;
        self.replay_target = (cursor != 0).then_some((cursor as usize, u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("mathematical-command-checkpoint-digest"))?)));
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
            let bytes = std::mem::size_of::<String>() + id.capacity();
            if maximum_items == 0 || maximum_bytes < bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.delete_ids.pop_first();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
        }
        if let Some(node) = self.rewrite_nodes.last() {
            let bytes = std::mem::size_of_val(node) + node.id.capacity() + node.label.capacity();
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
            let bytes = std::mem::size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity();
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
            let bytes = std::mem::size_of_val(point);
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
                let bytes = std::mem::size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity();
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
                let bytes = std::mem::size_of_val(node) + node.id.capacity() + node.label.capacity();
                if maximum_items == 0 || maximum_bytes < bytes {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                graph.nodes.pop();
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: bytes };
            }
            if let Some(step) = Self::close_vec_capacity(&mut graph.nodes, maximum_items, maximum_bytes) {
                return step;
            }
            let bytes = std::mem::size_of::<MathematicalGraph>() + graph.algorithm.capacity() + graph.algorithm_seed.as_ref().map_or(0, String::capacity);
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

struct MathematicalCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl MathematicalCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: MATHEMATICAL_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for MathematicalCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<MathematicalPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<MathematicalPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        MATHEMATICAL_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        mathematical_contract()
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
        if input.declared_bytes() > MATHEMATICAL_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Mathematical retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for MathematicalCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<MathematicalPlayApp>;
    const TOOL_IDS: &'static [&'static str] = MATHEMATICAL_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = MATHEMATICAL_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
struct MathematicalStorePreparationFactory<P, M> {
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> Default for MathematicalStorePreparationFactory<P, M> {
    fn default() -> Self {
        Self { marker: std::marker::PhantomData }
    }
}

struct MathematicalStorePreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn mathematical_store_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("mathematical-retained-{}", authority.next_sequence_number());
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

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for MathematicalStorePreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, _mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Mathematical Store preparation rejected its lane or description envelope".into());
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
        Ok(Box::new(MathematicalStorePreparation {
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

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for MathematicalStorePreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::{Mutation as _, MutationDiff as _};
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Mathematical preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Mathematical preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Mathematical preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(mathematical_store_edit(mutation, inverse, self.description.take(), authority), Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Mathematical preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
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

//#region 🔖️MathematicalPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// `crate::editor::mathematical::config::MathematicalConfig` (see `ArtifactEditor::Config`), written
/// through `MathematicalConfigMutation`s.
#[derive(Default)]
pub struct MathematicalPlayApp;

impl ArtifactEditor for MathematicalPlayApp {
    type Snapshot = MathematicalSnapshot;
    type Mutation = MathematicalMutation;
    type Config = MathematicalConfig;
    type ConfigMutation = MathematicalConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = MathematicalPresence;
    type PresenceMutation = MathematicalPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = MathematicalCommand;

    const DIALECT: Dialect = MATHEMATICAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(MathematicalStorePreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(Arc::new(MathematicalStorePreparationFactory::<Self::Config, Self::ConfigMutation>::default()))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<MathematicalPlayApp>,
        owner_file: "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.mathematical.mathematical@1/*#editor",
        document_schema: "semio.mathematical/v1",
        factory: "MathematicalCommandJobFactory",
        factory_type: MathematicalCommandJobFactory,
        tools: {
            "setDocument" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setAlgorithm" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setDirected" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "nodeGraphViewport" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setPoints" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1),
            "setLocale" => semio_framework::ToolExecutionContract::resumable(65_536, 2_048, 1, 65_536, 7_500, 1, 1)
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(MathematicalCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !MATHEMATICAL_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("mathematical-command-tool-mismatch"));
        }
        let extent = mathematical_command_extent(&request.command, &request.snapshot).ok_or_else(|| Fault::from("mathematical-command-capacity"))?;
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(MathematicalRetainedCommandWork::new(tool_id, mathematical_operation_identity(tool_id, &operation_context), extent));
        let payload = ArtifactRetainedCommandPayload::try_new(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            MathematicalCommand::command_id,
            MATHEMATICAL_RETAINED_RAW_BYTES,
            MATHEMATICAL_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::mathematical::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> MathematicalSnapshot {
        MathematicalSnapshot::default()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(mathematical_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    async fn command_id(command: &MathematicalCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &MathematicalCommand,
        doc: &ArtifactView<'_, MathematicalSnapshot>,
        cfg: &ConfigView<'_, MathematicalConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.mathematical`-kinded output);
    /// `"document:out"` replicates `ArtifactApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    async fn export_media(port: &str, doc: &ArtifactView<'_, MathematicalSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
                let overlay = algorithm_overlay(&graph);
                let overlay_json = json::object(overlay.iter().map(|(id, suffix)| (id.clone(), Value::from(suffix.as_str()))));
                let json = json::to_string(&json::object([("algorithm".to_string(), Value::from(graph.algorithm.as_str())), ("overlay".to_string(), overlay_json)]));
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.mathematical".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, MathematicalSnapshot>, cfg: &ConfigView<'_, MathematicalConfig>) -> UiNode {
        match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&crate::artifacts::mathematical::mathematical_graph(doc.snapshot), &cfg.snapshot.camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&crate::artifacts::mathematical::mathematical_geometry(doc.snapshot)),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️MathematicalPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` —
/// `PluginBuilder::editor::<E>(def: AppDefinition)` only takes the bare definition, so the old
/// `.example_source(crate::examples::art_mathematical_demo::source())` and
/// `.workflow("mathematical", "Mathematical", "graph")` calls are dropped here (not silently: noted
/// in the migration report). The subset's own `📚️examples/🎬️demo` facet
/// (`crate::artifacts::mathematical::examples::...`, real content, pre-existing) is the modern,
/// role-agnostic replacement surface for example registration.
pub async fn create_mathematical_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(MATHEMATICAL_DIALECT)
        .document(["semio", "mathematical"])
        .artifact_kind(crate::artifacts::mathematical::artifact_kind())
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
        .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
        .mutation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
        .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
        .action_interactive_job("setDocument", InteractiveJobClassification::Migrated)
        .action_interactive_job("setAlgorithm", InteractiveJobClassification::Migrated)
        .action_interactive_job("setDirected", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setPoints", InteractiveJobClassification::Migrated)
        .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
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
            ActionArgDef::toggle("directed", LocalizedLabel::native("Directed", "Gerichtet")).default_value(true),
        ])
        // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
        // WORKFLOWS-END-TO-END-TYPED-PORTS) — `mathematical_io()` (this file's own `🔖️Io` region) is
        // this port information's single source of truth, reused here rather than duplicated.
        .io(mathematical_io())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `MathematicalPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<MathematicalPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<MathematicalPlayApp>` builds it.
    pub type MathApp = VcsArtifactApp<EditorApp<MathematicalPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn math_app() -> MathApp {
        new_app::<EditorApp<MathematicalPlayApp>>()
    }

    /// ✏️ Adapts `create_mathematical_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
    /// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub async fn mathematical_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_mathematical_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn math_app_with_registry() -> MathApp {
        new_app_with_registry::<EditorApp<MathematicalPlayApp>>(mathematical_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut MathApp, command: MathematicalCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut MathApp, body_key: &str) -> String {
        // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
        // `Debug` gives every test caller here the same "does the render mention X" substring check.
        format!("{:?}", app.render(body_key, None, &ViewModel::default()).expect("render"))
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::mathematical::testkit::{math_app, math_app_with_registry};

    //#region 🔖️RetainedCommands
    fn retained_operation(generation: u64) -> AppOperationContext {
        AppOperationContext { app_instance_id: 7, parent_document_id: "mathematical-retained-test".into(), operation_id: 11, generation, canonical_base_revision: [17; 32] }
    }

    fn graph_with_shape(node_count: usize, edge_count: usize) -> MathematicalGraph {
        let nodes = (0..node_count).map(|index| crate::artifacts::mathematical::MathematicalNode { id: format!("n{index}"), label: format!("N{index}"), x: index as f64, y: -(index as f64) }).collect();
        let edges = (0..edge_count).map(|index| crate::artifacts::mathematical::MathematicalEdge { id: format!("e{index}"), source: format!("n{}", index % node_count.max(1)), target: format!("n{}", (index + 1) % node_count.max(1)) }).collect();
        MathematicalGraph { directed: true, nodes, edges, algorithm: "bfs".into(), algorithm_seed: Some("n0".into()) }
    }

    fn drive_retained(work: &mut MathematicalRetainedCommandWork, command: &MathematicalCommand, snapshot: &MathematicalSnapshot, operation: &AppOperationContext) -> protocol::DslValue {
        let config = MathematicalConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        loop {
            match work.step(command, snapshot, &config, &history, &interaction, &hover, None, operation).expect("retained Mathematical turn") {
                ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => {}
                // 🌱️ `ToValue`/`DslValue` in place of the old `serde_json::to_value` oracle: `DslValue`
                // already implements `PartialEq`, so the two runs compare directly with no JSON text
                // round trip needed.
                ArtifactCommandWorkStep::Complete(emit) => return protocol::ToValue::to_value(&emit.artifact_mutations),
                ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("Mathematical commands do not publish ephemeral state"),
            }
        }
    }

    #[test]
    fn retained_schema_contract_and_factory_identity_are_exact() {
        let fixture: Value = json::parse(include_str!("../../../../../fixturesmathematical-retained-command-law.json")).expect("language-neutral retained fixture");
        assert_eq!(fixture["contract"]["workItems"], 65_536);
        assert_eq!(fixture["contract"]["maximumStepMillis"], 8);
        assert_eq!(fixture["actions"], json::array(MATHEMATICAL_TOOL_IDS.iter().map(|id| Value::from(*id))));
        assert_eq!(fixture["hostileCases"].as_array().map(<[Value]>::len), Some(14));
        let factory = MathematicalCommandJobFactory::new("s.mathematical.mathematical@1/*#editor");
        let keys = <MathematicalCommandJobFactory as semio_framework::ToolJobFactory>::keys(&factory);
        assert_eq!(keys.len(), MATHEMATICAL_TOOL_IDS.len());
        for (key, tool_id) in keys.iter().zip(MATHEMATICAL_TOOL_IDS) {
            assert_eq!(key.controller_id, "s.mathematical.mathematical@1/*#editor");
            assert_eq!(key.tool_id, *tool_id);
        }
        assert_eq!(<MathematicalPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 7);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one() {
        let command = MathematicalCommand::SetDirected(set_directed::SetDirected { directed: false });
        let maximum_nodes = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph_with_shape(MATHEMATICAL_MAX_NODES, 0), MathematicalGeometry::default());
        let excessive_nodes = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph_with_shape(MATHEMATICAL_MAX_NODES + 1, 0), MathematicalGeometry::default());
        assert!(mathematical_command_extent(&command, &maximum_nodes).is_some());
        assert!(mathematical_command_extent(&command, &excessive_nodes).is_none());
        let maximum_edges = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph_with_shape(2, MATHEMATICAL_MAX_EDGES), MathematicalGeometry::default());
        let excessive_edges = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph_with_shape(2, MATHEMATICAL_MAX_EDGES + 1), MathematicalGeometry::default());
        assert!(mathematical_command_extent(&command, &maximum_edges).is_some());
        assert!(mathematical_command_extent(&command, &excessive_edges).is_none());

        let snapshot = crate::artifacts::mathematical::mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default());
        let point = crate::artifacts::mathematical::MathematicalPoint { x: 1.0, y: 2.0 };
        let maximum_points = MathematicalCommand::SetPoints(set_points::SetPoints { geometry: MathematicalGeometry { points: vec![point.clone(); MATHEMATICAL_MAX_POINTS] } });
        let excessive_points = MathematicalCommand::SetPoints(set_points::SetPoints { geometry: MathematicalGeometry { points: vec![point; MATHEMATICAL_MAX_POINTS + 1] } });
        assert!(mathematical_command_extent(&maximum_points, &snapshot).is_some());
        assert!(mathematical_command_extent(&excessive_points, &snapshot).is_none());
        let maximum_text = "a".repeat(MATHEMATICAL_MAX_TEXT_BYTES);
        let excessive_text = "a".repeat(MATHEMATICAL_MAX_TEXT_BYTES + 1);
        assert!(mathematical_command_extent(&MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: maximum_text, seed: None }), &snapshot).is_some());
        assert!(mathematical_command_extent(&MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: excessive_text, seed: None }), &snapshot).is_none());
        assert!(mathematical_command_extent(&MathematicalCommand::SetLocale(set_locale::SetLocale { value: "d".repeat(MATHEMATICAL_MAX_LOCALE_BYTES) }), &snapshot).is_some());
        assert!(mathematical_command_extent(&MathematicalCommand::SetLocale(set_locale::SetLocale { value: "d".repeat(MATHEMATICAL_MAX_LOCALE_BYTES + 1) }), &snapshot).is_none());

        let operations = |count: usize| json::to_string(&json::array(std::iter::repeat(json::object([])).take(count)));
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(MATHEMATICAL_MAX_EDIT_OPERATIONS) }), &snapshot).is_some());
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(MATHEMATICAL_MAX_EDIT_OPERATIONS + 1) }), &snapshot).is_none());
        let delete = |count: usize| {
            json::to_string(&json::array([json::object([
                ("operation".to_string(), Value::from("deleteSelection")),
                ("nodeIds".to_string(), json::array((0..count).map(|index| Value::from(format!("n{index}"))))),
            ])]))
        };
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(MATHEMATICAL_MAX_DELETE_IDS) }), &snapshot).is_some());
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(MATHEMATICAL_MAX_DELETE_IDS + 1) }), &snapshot).is_none());
        let exact_json = format!("[{}]", " ".repeat(MATHEMATICAL_MAX_EDIT_JSON_BYTES - 2));
        let excessive_json = format!("[{}]", " ".repeat(MATHEMATICAL_MAX_EDIT_JSON_BYTES - 1));
        assert_eq!(exact_json.len(), MATHEMATICAL_MAX_EDIT_JSON_BYTES);
        assert_eq!(excessive_json.len(), MATHEMATICAL_MAX_EDIT_JSON_BYTES + 1);
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: exact_json }), &snapshot).is_some());
        assert!(mathematical_command_extent(&MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: excessive_json }), &snapshot).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_interruption_replay_aba_cancel_and_repeated_close_are_exact() {
        let graph = graph_with_shape(8, 12);
        let snapshot = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph, MathematicalGeometry::default());
        let command = MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
            operations_json: json::to_string(&json::array([
                json::object([("operation".to_string(), Value::from("move")), ("nodeId".to_string(), Value::from("n7")), ("x".to_string(), Value::from(41.0)), ("y".to_string(), Value::from(42.0))]),
                json::object([("operation".to_string(), Value::from("deleteSelection")), ("nodeIds".to_string(), json::array([Value::from("n1"), Value::from("n3")]))]),
                json::object([("operation".to_string(), Value::from("addNode")), ("x".to_string(), Value::from(5.0)), ("y".to_string(), Value::from(6.0))]),
            ])),
        });
        let operation = retained_operation(13);
        let extent = mathematical_command_extent(&command, &snapshot).expect("retained extent");
        let identity = mathematical_operation_identity("nodeGraphEdit", &operation);
        let mut uninterrupted = MathematicalRetainedCommandWork::new("nodeGraphEdit", identity, extent);
        let config = MathematicalConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        for _ in 0..9 {
            assert!(matches!(uninterrupted.step(&command, &snapshot, &config, &history, &interaction, &hover, None, &operation).expect("checkpoint prefix"), ArtifactCommandWorkStep::Progress { .. }));
        }
        let mut checkpoint = [0_u8; 40];
        assert_eq!(uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint"), 40);
        let aba_operation = retained_operation(14);
        let mut stale_aba = MathematicalRetainedCommandWork::new("nodeGraphEdit", mathematical_operation_identity("nodeGraphEdit", &aba_operation), extent);
        assert!(stale_aba.restore(&checkpoint).is_err());
        let mut wrong_action = MathematicalRetainedCommandWork::new("setDirected", mathematical_operation_identity("setDirected", &operation), extent);
        assert!(wrong_action.restore(&checkpoint).is_err());

        let mut replayed = MathematicalRetainedCommandWork::new("nodeGraphEdit", identity, extent);
        replayed.restore(&checkpoint).expect("interrupted restore");
        let uninterrupted_output = drive_retained(&mut uninterrupted, &command, &snapshot, &operation);
        let replayed_output = drive_retained(&mut replayed, &command, &snapshot, &operation);
        assert_eq!(uninterrupted_output, replayed_output, "the DslValue-encoded mutation output must observe exact replay output");

        let mut cancelled_before = MathematicalRetainedCommandWork::new("nodeGraphEdit", identity, extent);
        assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Blocked);
        cancelled_before.begin_close();
        assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
        assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
        let mut cancelled_after = MathematicalRetainedCommandWork::new("nodeGraphEdit", identity, extent);
        assert!(matches!(cancelled_after.step(&command, &snapshot, &config, &history, &interaction, &hover, None, &operation).expect("cancel after admission"), ArtifactCommandWorkStep::Progress { .. }));
        cancelled_after.begin_close();
        assert!(matches!(cancelled_after.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        while !cancelled_after.terminal_is_empty() {
            let _ = cancelled_after.close_step(1, usize::MAX);
        }
        assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
        assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_maximum_microturns_stay_below_eight_milliseconds() {
        let graph = graph_with_shape(MATHEMATICAL_MAX_NODES, MATHEMATICAL_MAX_EDGES);
        let snapshot = crate::artifacts::mathematical::mathematical_snapshot_with_state(graph, MathematicalGeometry::default());
        let ids = (0..MATHEMATICAL_MAX_DELETE_IDS).map(|index| format!("n{index}")).collect::<Vec<_>>();
        let mut operations = vec![json::object([("operation".to_string(), Value::from("deleteSelection")), ("nodeIds".to_string(), json::array(ids.iter().map(|id| Value::from(id.as_str()))))])];
        operations.resize(MATHEMATICAL_MAX_EDIT_OPERATIONS, json::object([]));
        let command = MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
            operations_json: json::to_string(&json::array(operations)),
        });
        let operation = retained_operation(23);
        let extent = mathematical_command_extent(&command, &snapshot).expect("maximum retained extent");
        let mut work = MathematicalRetainedCommandWork::new("nodeGraphEdit", mathematical_operation_identity("nodeGraphEdit", &operation), extent);
        let config = MathematicalConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        loop {
            let started = std::time::Instant::now();
            let step = work.step(&command, &snapshot, &config, &history, &interaction, &hover, None, &operation).expect("maximum retained turn");
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Mathematical microturn exceeded 8 ms");
            if matches!(step, ArtifactCommandWorkStep::Complete(_)) {
                break;
            }
        }
        work.begin_close();
        while !work.terminal_is_empty() {
            let started = std::time::Instant::now();
            let _ = work.close_step(1, usize::MAX);
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum Mathematical close turn exceeded 8 ms");
        }
    }
    //#endregion 🔖️RetainedCommands

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_the_full_row_set_is_covered() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 7, "every MathematicalCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the two documented divergences: `setLocale` → `locale`
    /// (an undeclared host-pushed command) and `setDocument` → `set-artifact` (the `app_commands!`
    /// row's own `"setDocument" as "set-artifact" => set_artifact::SetArtifact` explicitly pins a
    /// non-kebab wire keyword, matching `SetArtifact`'s own `#[dsl(keyword = "set-artifact")]`).
    /// **Pre-existing bug, independently traced**: `git log -1 --date=iso -- 🎮️commands/📄️set-artifact/
    /// 🦀️.rs` shows `SetArtifact`'s explicit `set-artifact` keyword predates this ticket's
    /// own edits to this file (which only touched `render`/`export_media`); this test's hardcoded
    /// exception list simply never accounted for the second declared divergence. Fixed outright
    /// per this ticket's own "trivial, safe, unambiguous" guidance rather than left unresolved.
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setDocument" => "set-artifact".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<MathematicalCommand> {
        vec![
            MathematicalCommand::SetArtifact(set_artifact::SetArtifact {
                graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathematicalGraph::default()),
                geometry: crate::artifacts::mathematical::MathematicalGeometry::default(),
            }),
            MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }),
            MathematicalCommand::SetDirected(set_directed::SetDirected { directed: true }),
            MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() }),
            MathematicalCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: crate::artifacts::mathematical::MathematicalCamera { x: 5.0, y: 6.0, zoom: 2.0 } }),
            MathematicalCommand::SetPoints(set_points::SetPoints { geometry: crate::artifacts::mathematical::MathematicalGeometry::default() }),
            MathematicalCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ The row whose `Option` field makes `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `mathematical_protocol` crate (see the ticket's
    /// `🧪️wire-baseline-before.txt`).
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(MathematicalCommand, &str, &str); 2] = [
            (MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "topo".into(), seed: None }), "set-algorithm algorithm=topo", "01010104746f706f01000600"),
            (MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }), "set-algorithm algorithm=bfs seed=a", "01010201610362667302000601010600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        // 🌱️ `AppDefinition` (`semio-framework-plugin`, framework-owned) has not itself gained
        // `ToValue` — `Debug` gives the same "does the manifest mention X" substring check without
        // needing `serde_json` for a framework type this batch does not own.
        let debug = format!("{:?}", create_mathematical_app());
        for id in [graph_window::MATH_PLAY_WINDOW_GRAPH, geometry_window::MATH_PLAY_WINDOW_GEOMETRY] {
            assert!(debug.contains(id), "window kind {id} missing from the manifest: {debug}");
        }
        assert!(debug.contains(edit::MATH_PLAY_MODE_EDIT), "mode missing from the manifest");
        assert!(debug.contains("computation.mathematical"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn mathematical_io_is_declared_on_the_manifest() {
        let app = create_mathematical_app();
        assert_eq!(app.io.artifact.id, "computation.mathematical");
        assert_eq!(app.io.ports.len(), 1);
        assert_eq!(app.io.ports[0].id, "result:out");
    }

    #[semio_framework_async_macros::async_test]
    async fn create_mathematical_app_builds_a_definition_for_the_editor_role() {
        let def = create_mathematical_app();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, MATHEMATICAL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<MathematicalPlayApp as ArtifactEditor>::DIALECT, MATHEMATICAL_DIALECT);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::mathematical::testkit::render;
        let mut app = math_app();
        assert!(render(&mut app, "mathematical.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn command_surface_is_registry_clean() {
        let _app = math_app_with_registry();
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MathematicalIo
    #[semio_framework_async_macros::async_test]
    async fn mathematical_io_declares_result_out_with_the_computation_mathematical_kind() {
        let io = mathematical_io();
        assert_eq!(io.document_schema, "semio.mathematical/v1");
        assert_eq!(io.artifact.id, "computation.mathematical");
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "result:out");
        assert_eq!(port.kind_id.as_deref(), Some("computation.mathematical"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }
    //#endregion 🔖️MathematicalIo

    //#region 🔖️GraphAlgorithms
    #[semio_framework_async_macros::async_test]
    async fn topo_algorithm_overlay_orders_dag_nodes() {
        let graph = MathematicalGraph::default();
        let overlay = algorithm_overlay(&graph);
        assert!(overlay.get("a").unwrap().starts_with(" #0"));
        assert!(overlay.get("d").unwrap().starts_with(" #"));
    }

    #[semio_framework_async_macros::async_test]
    async fn components_algorithm_overlay_groups_disconnected_node() {
        use crate::artifacts::mathematical::MathematicalNode;
        let mut graph = MathematicalGraph { algorithm: "components".into(), ..MathematicalGraph::default() };
        graph.nodes.push(MathematicalNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
        let overlay = algorithm_overlay(&graph);
        assert_ne!(overlay.get("a"), overlay.get("z"));
    }

    #[semio_framework_async_macros::async_test]
    async fn bfs_algorithm_overlay_reports_hop_distance() {
        let graph = MathematicalGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathematicalGraph::default() };
        let overlay = algorithm_overlay(&graph);
        assert_eq!(overlay.get("a").unwrap(), " d0");
        assert_eq!(overlay.get("b").unwrap(), " d1");
    }

    #[semio_framework_async_macros::async_test]
    async fn workflow_json_round_trips_node_count() {
        let graph = MathematicalGraph::default();
        let (nodes, edges) = workflow_json(&graph);
        assert_eq!(nodes.len(), graph.nodes.len());
        assert_eq!(edges.len(), graph.edges.len());
    }
    //#endregion 🔖️GraphAlgorithms

    //#region 🔖️Geometry
    #[semio_framework_async_macros::async_test]
    async fn geometry_layers_include_hull_and_centroid() {
        let geometry = MathematicalGeometry::default();
        let layers_json = geometry_layers_json(&geometry);
        assert!(layers_json.contains("\"hull\""));
        assert!(layers_json.contains("\"centroid\""));
    }
    //#endregion 🔖️Geometry
}
//#endregion 🧪️Tests
