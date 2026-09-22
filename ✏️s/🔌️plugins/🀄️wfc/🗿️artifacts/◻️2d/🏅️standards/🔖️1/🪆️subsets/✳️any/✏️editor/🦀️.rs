//! ✏️ WFC 2D editor — the `ArtifactEditor` for `s.wfc.wfc2d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas and the read-only `wfc-2d-preview`.
//!
//! Every command maps 1:1 onto a real `Wfc2dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The two command families that are NOT document mutations (`change-camera`, `change-active-tile`)
//! go to the per-pane config instead: a camera is per-window view state. The SOLVE goes to the app
//! transient — it is an INFERENCE, so letting it reach the document lanes would make a derived value
//! look persisted and would pollute undo — and reaches the preview window through
//! `ArtifactEditor::render_with_request_context`, the only render entry point that carries that lane.

#![allow(clippy::result_large_err)]

use crate::editor::wfc2d::config::{wfc2d_active_tile_id, Wfc2dConfig, Wfc2dConfigMutation};
use crate::editor::wfc2d::modes::edit;
use crate::editor::wfc2d::modes::edit::tools::fill;
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use crate::editor::wfc2d::transient::{Wfc2dTransient, Wfc2dTransientMutation};
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dTile, Wfc2dTileMedia};
use crate::{Wfc2dMutation, Wfc2dSnapshot, WFC_2D_DIALECT, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ActionArgDef, Effect, ActionArgOption, ActionDefinition, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ToolRunJob, ToolRunJobPurpose, ToolRunJobRequest,
    ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractiveJobClassification, Label, LocalizedLabel, MergeMode, NoDraft,
    NoDraftMutation, NoPresence, NoPresenceMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc2dMutation` kind a UI can
/// trigger, plus the three non-document verbs (camera, armed tile, solve result).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Wfc2dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "create-slot")]
    CreateSlot { id: String, x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    #[dsl(key = "move-slot")]
    MoveSlot { id: String, x: f64, y: f64 },
    #[dsl(key = "resize-slot")]
    ResizeSlot { id: String, width: f64, height: f64 },
    #[dsl(key = "connect-slots")]
    ConnectSlots { id: String, from_slot_id: String, to_slot_id: String, relation: String },
    #[dsl(key = "disconnect-slots")]
    DisconnectSlots { id: String },
    #[dsl(key = "pin-slot")]
    PinSlot { id: String, tile_id: String },
    #[dsl(key = "unpin-slot")]
    UnpinSlot { id: String },
    #[dsl(key = "create-tile")]
    CreateTile { id: String, label: Option<String>, weight: f64 },
    #[dsl(key = "delete-tile")]
    DeleteTile { id: String },
    #[dsl(key = "change-tile-weight")]
    ChangeTileWeight { tile_id: String, weight: f64 },
    #[dsl(key = "change-tile-media")]
    ChangeTileMedia { tile_id: String },
    #[dsl(key = "create-rule")]
    CreateRule { id: String, tile_a_id: String, tile_b_id: String, relation: Option<String>, allowed: bool },
    #[dsl(key = "delete-rule")]
    DeleteRule { id: String },
    #[dsl(key = "change-camera")]
    ChangeCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "change-active-tile")]
    ChangeActiveTile { tile_id: String },
    #[dsl(key = "solve")]
    Solve,
    #[dsl(key = "commit-fill")]
    CommitFill { payload_json: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    /// 🕹️ The NodeGraph canvas' OWN gesture channel: dragging a node and completing a wire both arrive
    /// as `nodeGraphEdit` carrying one `operations` entry, never as `move-slot`/`connect-slots`
    /// directly. Coalescing is the canvas': `onNodeDragStop` fires once per gesture, so one drag lands
    /// exactly one `move-slot`.
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
}

impl protocol::OpBinary for Wfc2dEditorCommand {
    /// 🧵️ The list `validate_tool_job_rows` joins against: `TOOL_JOB_IDS ∩ migrated` must EQUAL the
    /// `bounded_first_step_tool_proofs!` rows, or the app boots with `interactive-job.catalog-incomplete`.
    const TOOL_JOB_IDS: &'static [&'static str] = WFC_2D_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🪪️ The exact tool id one command dispatches as — `dispatch_typed_command_inner` rejects a command
/// whose id does not equal the admitted action, so this is the single naming authority for the
/// manifest actions, the retained roster and the publication contracts alike.
pub fn wfc2d_command_id(command: &Wfc2dEditorCommand) -> &'static str {
    match command {
        Wfc2dEditorCommand::ChangeSeed { .. } => "change-seed",
        Wfc2dEditorCommand::CreateSlot { .. } => "create-slot",
        Wfc2dEditorCommand::DeleteSlot { .. } => "delete-slot",
        Wfc2dEditorCommand::MoveSlot { .. } => "move-slot",
        Wfc2dEditorCommand::ResizeSlot { .. } => "resize-slot",
        Wfc2dEditorCommand::ConnectSlots { .. } => "connect-slots",
        Wfc2dEditorCommand::DisconnectSlots { .. } => "disconnect-slots",
        Wfc2dEditorCommand::PinSlot { .. } => "pin-slot",
        Wfc2dEditorCommand::UnpinSlot { .. } => "unpin-slot",
        Wfc2dEditorCommand::CreateTile { .. } => "create-tile",
        Wfc2dEditorCommand::DeleteTile { .. } => "delete-tile",
        Wfc2dEditorCommand::ChangeTileWeight { .. } => "change-tile-weight",
        Wfc2dEditorCommand::ChangeTileMedia { .. } => "change-tile-media",
        Wfc2dEditorCommand::CreateRule { .. } => "create-rule",
        Wfc2dEditorCommand::DeleteRule { .. } => "delete-rule",
        Wfc2dEditorCommand::ChangeCamera { .. } => "change-camera",
        Wfc2dEditorCommand::ChangeActiveTile { .. } => "change-active-tile",
        Wfc2dEditorCommand::Solve => "solve",
        Wfc2dEditorCommand::CommitFill { .. } => fill::COMMIT_FILL_ACTION_ID,
        Wfc2dEditorCommand::SetActiveExample { .. } => WFC_2D_SET_ACTIVE_EXAMPLE,
        Wfc2dEditorCommand::NodeGraphEdit { .. } => WFC_2D_NODE_GRAPH_EDIT,
    }
}
//#endregion 🔖️Command

//#region 🧵️Retained
/// 🎨️ The action id the shell's own navbar example picker dispatches — fixed by
/// `SET_ACTIVE_EXAMPLE_ACTION_ID` in `🛠️ShellHelpers`, so it is camelCase where every wfc verb is
/// kebab-case. Declaring it is what keeps the picker alive: an action no window kind of this app
/// declares is dropped as `undeclared-action` before it ever reaches a guest.
pub const WFC_2D_SET_ACTIVE_EXAMPLE: &str = "setActiveExample";

/// 🕹️ The action id the NodeGraph canvas host dispatches every node drag and every completed wire
/// under (`nodeGraphActions.edit`) — likewise fixed by the renderer, likewise camelCase.
pub const WFC_2D_NODE_GRAPH_EDIT: &str = "nodeGraphEdit";

/// 🧵️ Every verb a pane may dispatch. An id absent here carries NO app-owned retained factory, and a
/// bare bounded proof faults in `typed-command-reducer` with "generic bounded proof has no resumable
/// app-owned reducer job" — the verb is then dead in the running app however it is classified.
pub const WFC_2D_RETAINED_TOOL_IDS: &[&str] = &[
    "change-seed",
    "create-slot",
    "delete-slot",
    "move-slot",
    "resize-slot",
    "connect-slots",
    "disconnect-slots",
    "pin-slot",
    "unpin-slot",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
    "change-camera",
    "change-active-tile",
    "solve",
    fill::COMMIT_FILL_ACTION_ID,
    WFC_2D_SET_ACTIVE_EXAMPLE,
    WFC_2D_NODE_GRAPH_EDIT,
];

const WFC_2D_RETAINED_PAYLOAD_SCHEMA: &str = "wfc.wfc2d.tool-command.v1";
const WFC_2D_RETAINED_RAW_BYTES: usize = 65_536;
const WFC_2D_RETAINED_WORK_ITEMS: usize = 4_096;

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact] }
}

/// 🧮️ One pane-config publication row — camera and armed tile are view state, never document state.
const fn config_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Config] }
}

/// 🛣️ One lane per route, read off each arm of `dispatch` and `solve_transient`. `solve` is the one
/// TRANSIENT route: the assignment is inferred, so it must never reach the document lanes. The example
/// picker is `HostOnly` — it answers a whole-document `Effect::LoadDocument`, which is not an edit, so
/// re-picking the boot example mints no phantom undo entry.
const WFC_2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("create-slot"),
    artifact_route("delete-slot"),
    artifact_route("move-slot"),
    artifact_route("resize-slot"),
    artifact_route("connect-slots"),
    artifact_route("disconnect-slots"),
    artifact_route("pin-slot"),
    artifact_route("unpin-slot"),
    artifact_route("create-tile"),
    artifact_route("delete-tile"),
    artifact_route("change-tile-weight"),
    artifact_route("change-tile-media"),
    artifact_route("create-rule"),
    artifact_route("delete-rule"),
    config_route("change-camera"),
    config_route("change-active-tile"),
    ArtifactToolPublicationContract { tool_id: "solve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: fill::COMMIT_FILL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: WFC_2D_SET_ACTIVE_EXAMPLE, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    artifact_route(WFC_2D_NODE_GRAPH_EDIT),
];

fn wfc2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(WFC_2D_RETAINED_RAW_BYTES, WFC_2D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per route, admitted only while the WHOLE addressable document plus this
/// edit fit inside `WFC_2D_RETAINED_WORK_ITEMS`.
fn wfc2d_retained_extent(command: &Wfc2dEditorCommand, snapshot: &Wfc2dSnapshot) -> Option<usize> {
    if !WFC_2D_RETAINED_TOOL_IDS.contains(&wfc2d_command_id(command)) {
        return None;
    }
    let items = snapshot.slots.len().checked_add(snapshot.edges.len())?.checked_add(snapshot.tiles.len())?.checked_add(snapshot.rules.len())?.checked_add(1)?;
    (items <= WFC_2D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🏁 Runs `s.wfc.wfc2d.solve` to completion and answers the ONE transient mutation the preview paints
/// from. The engine job is driven through the artifact's own headless adapter, so the live app and the
/// crate's tests solve through exactly the same ladder.
pub fn solve_transient(document: &Wfc2dSnapshot) -> Result<Vec<Wfc2dTransientMutation>, Fault> {
    let commit = crate::inferences::solve_with_job(document).map_err(|error| Fault::from(format!("wfc2d-solve:{error}")))?;
    let assignments = document
        .slots
        .iter()
        .filter_map(|slot| commit.assignments.get(&slot.id).map(|tile_id| crate::editor::wfc2d::transient::Wfc2dAssignment { slot_id: slot.id.clone(), tile_id: tile_id.clone() }))
        .collect();
    Ok(vec![crate::editor::wfc2d::transient::SetSolve { assignments, contradiction: commit.contradiction }.into()])
}

/// 🧵️ The ONE bounded work step every wfc2d route runs — the pure `dispatch` reducer, plus the
/// transient lane for `solve`.
struct Wfc2dCommandWork {
    tool_id: &'static str,
    completed: bool,
}

impl Wfc2dCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, completed: false }
    }
}

impl ArtifactCommandWork<EditorApp<Wfc2dEditor>> for Wfc2dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Wfc2dEditorCommand,
        snapshot: &Wfc2dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Wfc2dEditor>>>,
    ) -> Option<usize> {
        (!self.completed && wfc2d_command_id(command) == self.tool_id).then(|| wfc2d_retained_extent(command, snapshot)).flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Wfc2dEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<Wfc2dEditor>>, Fault> {
        if self.completed || wfc2d_command_id(input.command) != self.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.retained.route"), "the bounded WFC 2D work rejects an undeclared or completed route"));
        }
        let emit = dispatch(input.command, input.snapshot, input.config)?;
        let transient = match input.command {
            Wfc2dEditorCommand::CommitFill { payload_json } => {
                let payload = fill::decode_fill_payload(payload_json.as_bytes()).ok_or_else(|| Fault::from("wfc2d-commit-fill-payload"))?;
                payload.set_solve_mutations()
            }
            _ => Vec::new(),
        };
        self.completed = true;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient, window_transient: Vec::new() } })
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block binds this exact
/// Rust type to `EditorApp<Wfc2dEditor>`, which is what turns a bare (and therefore dispatch-dead)
/// bounded proof into an exact-owner proof.
struct Wfc2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Wfc2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: WFC_2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Wfc2dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Wfc2dEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Wfc2dEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC_2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        wfc2d_retained_contract()
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
        if input.declared_bytes() > WFC_2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded WFC 2D command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Wfc2dRetainedCommandJobFactory {
    type Owner = EditorApp<Wfc2dEditor>;
    const TOOL_IDS: &'static [&'static str] = WFC_2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = WFC_2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️Retained

//#region 🔖️GraphView
/// 🔭️ Canvas world units per DOMAIN unit on the slot graph. Slot coordinates are authored small (a
/// corridor two units wide); the node canvas sizes a node box in the tens of units, so at 1:1 every
/// node of every example lands inside one overlapping speck at the origin — unreadable, and
/// impossible to drag one node out of. This is the one place that scaling lives: the window file
/// stays artifact-agnostic (and copyable by `wfc3d`, which declares its own constant), and
/// `wfc2d_host_snapshot_edit` divides by exactly this when a gesture comes back.
pub const WFC_2D_GRAPH_VIEW_SCALE: f64 = 140.0;

/// 🕸️ The `wfc-graph` window's whole view of this document — see that window's own doc comment for
/// why the projection lives here and not there (so `wfc3d` can copy the window file verbatim).
pub struct Wfc2dGraphView<'a>(pub &'a Wfc2dSnapshot);

impl graph::SlotGraphView for Wfc2dGraphView<'_> {
    fn graph_slots(&self) -> Vec<graph::GraphSlotView> {
        self.0
            .slots
            .iter()
            .map(|slot| graph::GraphSlotView {
                id: slot.id.clone(),
                x: slot.x * WFC_2D_GRAPH_VIEW_SCALE,
                y: slot.y * WFC_2D_GRAPH_VIEW_SCALE,
                width: slot.width * WFC_2D_GRAPH_VIEW_SCALE,
                height: slot.height * WFC_2D_GRAPH_VIEW_SCALE,
                pinned_tile_id: slot.pinned_tile_id.clone(),
            })
            .collect()
    }
    fn graph_edges(&self) -> Vec<graph::GraphEdgeView> {
        self.0.edges.iter().map(|edge| graph::GraphEdgeView { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }).collect()
    }
}
//#endregion 🔖️GraphView

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Wfc2dEditor;

//#region 🔖️Args
fn arg<'a>(args: Option<&'a dsl::DslValue>, key: &str) -> Option<&'a dsl::DslValue> {
    match args {
        Some(dsl::DslValue::Object(entries)) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
        _ => None,
    }
}

fn arg_f64(args: Option<&dsl::DslValue>, key: &str, fallback: f64) -> f64 {
    match arg(args, key) {
        Some(dsl::DslValue::Number(number)) => number.as_f64(),
        Some(dsl::DslValue::String(text)) => text.parse().unwrap_or(fallback),
        _ => fallback,
    }
}

fn arg_u64(args: Option<&dsl::DslValue>, key: &str, fallback: u64) -> u64 {
    let value = arg_f64(args, key, fallback as f64);
    if value.is_finite() && value >= 0.0 {
        value as u64
    } else {
        fallback
    }
}

fn arg_bool(args: Option<&dsl::DslValue>, key: &str, fallback: bool) -> bool {
    match arg(args, key) {
        Some(dsl::DslValue::Bool(value)) => *value,
        Some(dsl::DslValue::String(text)) => text == "true",
        _ => fallback,
    }
}

fn arg_string(args: Option<&dsl::DslValue>, key: &str) -> String {
    match arg(args, key) {
        Some(dsl::DslValue::String(text)) => text.clone(),
        Some(other) => dsl::json::to_json_string(other),
        None => String::new(),
    }
}

/// 🏷️ An optional string argument: absent or empty answers `None`, so a rule with no relation scope
/// and a rule whose relation is the empty string cannot be confused.
fn arg_optional_string(args: Option<&dsl::DslValue>, key: &str) -> Option<String> {
    let value = arg_string(args, key);
    (!value.is_empty()).then_some(value)
}
//#endregion 🔖️Args

//#region 🕹️GraphGestures
/// 🆔️ A fresh edge id for a wire the canvas just drew — `{from}-{to}`, suffixed until it is free, so
/// the same gesture repeated twice cannot collide with the edge it already made.
fn wfc2d_fresh_edge_id(document: &Wfc2dSnapshot, from: &str, to: &str) -> String {
    let base = format!("{from}-{to}");
    if !document.edges.iter().any(|edge| edge.id == base) {
        return base;
    }
    let mut index = 2u32;
    loop {
        let candidate = format!("{base}-{index}");
        if !document.edges.iter().any(|edge| edge.id == candidate) {
            return candidate;
        }
        index += 1;
    }
}

/// 🕹️ Translates ONE `nodeGraphEdit` gesture into this artifact's own mutation. The canvas sends
/// `move` on drag release and `connect` on a completed wire; anything else (a wasm-canvas
/// `setHostSnapshot`, a future verb) is a no-op rather than a fault, because a gesture channel must
/// never fail a pane.
fn wfc2d_node_graph_edit(document: &Wfc2dSnapshot, operations_json: &str) -> Option<(Wfc2dMutation, String)> {
    let operations: Vec<pack::JsonValue> = pack::from_json_str(operations_json).ok()?;
    for operation in &operations {
        let text = |key: &str| operation.get(key).and_then(pack::JsonValue::as_str).map(str::to_string);
        let number = |key: &str| operation.get(key).and_then(pack::JsonValue::as_f64);
        match operation.get("operation").and_then(pack::JsonValue::as_str).unwrap_or_default() {
            "move" => {
                let id = text("nodeId")?;
                if !document.slots.iter().any(|slot| slot.id == id) {
                    continue;
                }
                let (x, y) = (number("x")?, number("y")?);
                return Some((move_slot(id.clone(), x, y), format!("Move slot {id}")));
            }
            "connect" => {
                let (from, to) = (text("sourceNodeId")?, text("targetNodeId")?);
                if from == to || !document.slots.iter().any(|slot| slot.id == from) || !document.slots.iter().any(|slot| slot.id == to) {
                    continue;
                }
                if document.edges.iter().any(|edge| edge.from_slot_id == from && edge.to_slot_id == to) {
                    continue;
                }
                let edge = Wfc2dSlotEdge { id: wfc2d_fresh_edge_id(document, &from, &to), from_slot_id: from.clone(), to_slot_id: to.clone(), relation: crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() };
                return Some((connect_slots(edge), format!("Connect {from} to {to}")));
            }
            "setHostSnapshot" => {
                let snapshot = operation.get("hostSnapshotJson").and_then(pack::JsonValue::as_str)?;
                return wfc2d_host_snapshot_edit(document, snapshot);
            }
            _ => continue,
        }
    }
    None
}

/// 🔗️ Whether two endpoint pairs name the SAME adjacency. `wfc-graph`'s two ports exist only to give
/// the canvas something to drag a wire between — the relation itself is undirected and the solver
/// symmetrises it, so `(a, b)` and `(b, a)` are one adjacency and never a removal plus an addition.
fn same_adjacency(left: (&str, &str), right: (&str, &str)) -> bool {
    (left.0 == right.0 && left.1 == right.1) || (left.0 == right.1 && left.1 == right.0)
}

/// 🔌️ Splits the canvas' `"{nodeId}@{portId}"` endpoint grammar down to the node id.
fn wfc2d_endpoint_node(endpoint: &str) -> &str {
    endpoint.rsplit_once('@').map_or(endpoint, |(node, _)| node)
}

/// 🕸️ The wasm node-graph surface hands back its WHOLE graph rather than an edit journal, so one
/// gesture is read as the difference between that graph and the document: a new wire is a
/// `connect-slots`, a removed wire a `disconnect-slots`, and a moved node the `move-slot` with the
/// largest displacement (the canvas may relayout the rest, and a gesture is one edit).
/// ✂️ A wire the canvas dropped. Two guards, both learned the hard way (and confirmed by wfc3d):
/// adjacency is UNDIRECTED, so an endpoint pair the canvas reports the other way round is the SAME
/// adjacency and not a removal; and a snapshot that carries no wires at all while the document
/// carries several is a canvas that has not finished syncing, never a user who deleted every edge
/// in one gesture.
fn wfc2d_host_snapshot_edit(document: &Wfc2dSnapshot, host_snapshot_json: &str) -> Option<(Wfc2dMutation, String)> {
    let snapshot: pack::JsonValue = pack::from_json_str(host_snapshot_json).ok()?;
    let nodes = snapshot.get("nodes").and_then(pack::JsonValue::as_array)?;
    let edges = snapshot.get("edges").and_then(pack::JsonValue::as_array).cloned().unwrap_or_default();
    let wires: Vec<(String, String)> = edges
        .iter()
        .filter_map(|edge| {
            let source = edge.get("source").and_then(pack::JsonValue::as_str)?;
            let target = edge.get("target").and_then(pack::JsonValue::as_str)?;
            Some((wfc2d_endpoint_node(source).to_string(), wfc2d_endpoint_node(target).to_string()))
        })
        .collect();
    for (from, to) in &wires {
        if from == to || !document.slots.iter().any(|slot| &slot.id == from) || !document.slots.iter().any(|slot| &slot.id == to) {
            continue;
        }
        if document.edges.iter().any(|edge| same_adjacency((&edge.from_slot_id, &edge.to_slot_id), (from, to))) {
            continue;
        }
        let edge = Wfc2dSlotEdge { id: wfc2d_fresh_edge_id(document, from, to), from_slot_id: from.clone(), to_slot_id: to.clone(), relation: crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() };
        return Some((connect_slots(edge), format!("Connect {from} to {to}")));
    }
    if !(wires.is_empty() && document.edges.len() > 1) {
        if let Some(edge) = document.edges.iter().find(|edge| !wires.iter().any(|wire| same_adjacency((&edge.from_slot_id, &edge.to_slot_id), (&wire.0, &wire.1)))) {
            return Some((disconnect_slots(edge.id.clone()), format!("Disconnect {}", edge.id)));
        }
    }
    let mut moved: Option<(f64, String, f64, f64)> = None;
    for node in nodes {
        let Some(id) = node.get("id").and_then(pack::JsonValue::as_str) else { continue };
        let Some(slot) = document.slots.iter().find(|slot| slot.id == id) else { continue };
        let (Some(x), Some(y)) = (node.get("x").and_then(pack::JsonValue::as_f64), node.get("y").and_then(pack::JsonValue::as_f64)) else { continue };
        let (x, y) = (x / WFC_2D_GRAPH_VIEW_SCALE, y / WFC_2D_GRAPH_VIEW_SCALE);
        let delta = (x - slot.x).abs() + (y - slot.y).abs();
        if delta > 1e-4 && moved.as_ref().is_none_or(|(best, ..)| delta > *best) {
            moved = Some((delta, id.to_string(), x, y));
        }
    }
    let (_, id, x, y) = moved?;
    Some((move_slot(id.clone(), x, y), format!("Move slot {id}")))
}
//#endregion 🕹️GraphGestures

//#region 🗃️Examples
/// 🗃️ The document one registered example id names, or `None` for an id this subset never shipped —
/// the picker offers exactly `crate::examples::sources()`, so an unknown id is a real fault, not a
/// silent blank document.
pub fn wfc2d_example_document(example_id: &str) -> Option<Wfc2dSnapshot> {
    match example_id {
        crate::examples::two_room_corridor::ID => Some(crate::examples::two_room_corridor::document()),
        crate::examples::wall_roof_facade_strip::ID => Some(crate::examples::wall_roof_facade_strip::document()),
        crate::examples::hex_ring::ID => Some(crate::examples::hex_ring::document()),
        crate::examples::terrain_ring::ID => Some(crate::examples::terrain_ring::document()),
        _ => None,
    }
}

/// 🗃️ The whole-document load the picker answers with. A `LoadDocument` effect is NOT an edit, so
/// re-selecting the example the app already booted on leaves `canUndo` false instead of minting the
/// phantom history entry a "replace every collection" mutation set would.
fn wfc2d_load_document_effect(document: &Wfc2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Wfc2dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("wfc", WFC_2D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🗃️Examples

//#region 🛂️Guards
/// 🛂️ Refuses a verb that names an entity the open document does not hold. A palette row carries a
/// STATIC argument default, so a default authored against one example reaches the guest unchanged
/// after the picker loads another — and a mutation against a missing id is either a fatal outcome the
/// ledger still journals or a silent no-op. Both read to a user as "the button did nothing"; a named
/// domain fault reads as what it is.
fn require_slot(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.slots.iter().any(|slot| slot.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.slot.unknown-slot"), format!("this document declares no slot '{id}'"))),
    }
}

fn require_tile(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.tiles.iter().any(|tile| tile.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.tile.unknown-tile"), format!("this document declares no tile '{id}'"))),
    }
}

fn require_edge(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.edges.iter().any(|edge| edge.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.edge.unknown-edge"), format!("this document declares no adjacency '{id}'"))),
    }
}

fn require_rule(document: &Wfc2dSnapshot, id: &str) -> Result<(), Fault> {
    match document.rules.iter().any(|rule| rule.id == id) {
        true => Ok(()),
        false => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.rule.unknown-rule"), format!("this document declares no rule '{id}'"))),
    }
}

fn require_fresh(taken: bool, entity: &'static str, id: &str) -> Result<(), Fault> {
    match taken {
        false => Ok(()),
        true => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.id.taken"), format!("this document already holds a {entity} '{id}'"))),
    }
}
//#endregion 🛂️Guards

/// ✏️ The whole command→emission rule set, as a free function over plain values. `ArtifactEditor::handle`
/// only adapts the framework's view bundle onto this: `InteractionView` has `pub(crate)` fields, so
/// this crate's own tests cannot construct one and would otherwise have no way to exercise dispatch
/// at all (cad's own test-harness precedent, `📐️cad/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`).
///
/// A pin gesture reads the PANE's armed tile rather than a global one, so two panes can pin different
/// tiles. Camera and armed-tile verbs go to the pane config and never touch the document lanes.
pub fn dispatch(command: &Wfc2dEditorCommand, document: &Wfc2dSnapshot, config: &Wfc2dConfig) -> Result<Emit<Wfc2dMutation, Wfc2dConfigMutation>, Fault> {
    let (mutation, description) = match command {
        Wfc2dEditorCommand::ChangeCamera { x, y, zoom } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeCamera(crate::editor::wfc2d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
            return Ok(Emit { config_mutations: mutations, description: Some("Change camera".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::ChangeActiveTile { tile_id } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeActiveTile(crate::editor::wfc2d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
            return Ok(Emit { config_mutations: mutations, description: Some("Arm tile".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::Solve => {
            return Ok(Emit { effects: fill::start_fill_effects(), description: Some("Solve".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::CommitFill { .. } => {
            return Ok(Emit { description: Some("Commit fill".into()), ..Default::default() });
        }
        Wfc2dEditorCommand::NodeGraphEdit { operations_json } => {
            let Some((mutation, description)) = wfc2d_node_graph_edit(document, operations_json) else {
                return Ok(Emit::default());
            };
            return Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() });
        }
        Wfc2dEditorCommand::SetActiveExample { example_id } => {
            let Some(next) = wfc2d_example_document(example_id) else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.example.unknown"), format!("wfc 2d has no example '{example_id}'")));
            };
            return Ok(Emit { effects: vec![wfc2d_load_document_effect(&next)], description: Some(format!("Load example {example_id}")), ..Default::default() });
        }
        Wfc2dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
        Wfc2dEditorCommand::CreateSlot { id, x, y, width, height } => {
            require_fresh(document.slots.iter().any(|slot| &slot.id == id), "slot", id)?;
            (create_slot(Wfc2dSlot { id: id.clone(), x: *x, y: *y, width: *width, height: *height, pinned_tile_id: None }), format!("Create slot {id}"))
        }
        Wfc2dEditorCommand::DeleteSlot { id } => {
            require_slot(document, id)?;
            (delete_slot(id.clone()), format!("Delete slot {id}"))
        }
        Wfc2dEditorCommand::MoveSlot { id, x, y } => {
            require_slot(document, id)?;
            (move_slot(id.clone(), *x, *y), format!("Move slot {id}"))
        }
        Wfc2dEditorCommand::ResizeSlot { id, width, height } => {
            require_slot(document, id)?;
            (resize_slot(id.clone(), *width, *height), format!("Resize slot {id}"))
        }
        Wfc2dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => {
            require_slot(document, from_slot_id)?;
            require_slot(document, to_slot_id)?;
            require_fresh(document.edges.iter().any(|edge| &edge.id == id), "adjacency", id)?;
            (
                connect_slots(Wfc2dSlotEdge {
                    id: id.clone(),
                    from_slot_id: from_slot_id.clone(),
                    to_slot_id: to_slot_id.clone(),
                    relation: if relation.is_empty() { crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() } else { relation.clone() },
                }),
                format!("Connect {from_slot_id} to {to_slot_id}"),
            )
        }
        Wfc2dEditorCommand::DisconnectSlots { id } => {
            require_edge(document, id)?;
            (disconnect_slots(id.clone()), format!("Disconnect {id}"))
        }
        Wfc2dEditorCommand::PinSlot { id, tile_id } => {
            require_slot(document, id)?;
            let tile = if tile_id.is_empty() { wfc2d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
            let Some(tile) = tile else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::Plugin, semio_framework_plugin::FaultCode::new("wfc2d.tile.unknown-pin"), "No tile is armed and the document declares none."));
            };
            require_tile(document, &tile)?;
            (pin_slot(id.clone(), tile.clone()), format!("Pin {id} to {tile}"))
        }
        Wfc2dEditorCommand::UnpinSlot { id } => {
            require_slot(document, id)?;
            (unpin_slot(id.clone()), format!("Unpin slot {id}"))
        }
        Wfc2dEditorCommand::CreateTile { id, label, weight } => {
            require_fresh(document.tiles.iter().any(|tile| &tile.id == id), "tile", id)?;
            (create_tile(Wfc2dTile { id: id.clone(), label: label.clone(), weight: *weight, media: Wfc2dTileMedia::default() }), format!("Create tile {id}"))
        }
        Wfc2dEditorCommand::DeleteTile { id } => {
            require_tile(document, id)?;
            (delete_tile(id.clone()), format!("Delete tile {id}"))
        }
        Wfc2dEditorCommand::ChangeTileWeight { tile_id, weight } => {
            require_tile(document, tile_id)?;
            (change_tile_weight(tile_id.clone(), *weight), format!("Change weight of {tile_id}"))
        }
        Wfc2dEditorCommand::ChangeTileMedia { tile_id } => {
            require_tile(document, tile_id)?;
            (change_tile_media(tile_id.clone(), Wfc2dTileMedia::default()), format!("Clear media of {tile_id}"))
        }
        Wfc2dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => {
            require_tile(document, tile_a_id)?;
            require_tile(document, tile_b_id)?;
            require_fresh(document.rules.iter().any(|rule| &rule.id == id), "rule", id)?;
            (
                create_rule(Wfc2dRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed }),
                format!("Create rule {id}"),
            )
        }
        Wfc2dEditorCommand::DeleteRule { id } => {
            require_rule(document, id)?;
            (delete_rule(id.clone()), format!("Delete rule {id}"))
        }
    };
    Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
}

/// 🖼️ The whole render rule set, likewise free of the framework's view bundle so a test can call it.
pub fn render_body(
    body_key: &str,
    document: &Wfc2dSnapshot,
    config: &Wfc2dConfig,
    transient: &Wfc2dTransient,
    tool_run: Option<&semio_framework_plugin::ToolRunView>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc2dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_2D_PREVIEW_BODY => preview::render(document, transient, tool_run, config.camera_x, config.camera_y, config.camera_zoom).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Wfc2dEditor {
    type Snapshot = Wfc2dSnapshot;
    type Mutation = Wfc2dMutation;
    type Config = Wfc2dConfig;
    type ConfigMutation = Wfc2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = Wfc2dTransient;
    type TransientMutation = Wfc2dTransientMutation;
    type Command = Wfc2dEditorCommand;

    const DIALECT: Dialect = WFC_2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    /// 🗃️ The bounded retirement catalog every store lane is released THROUGH: a store built without
    /// owners answers `artifact store has no owner-supplied bounded disposer` the moment the close
    /// ladder reaches it, so the owners and the disposer below are one declaration in two halves.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    /// ♻️ The instance close ladder walks one owned store lane per stage and faults the whole close
    /// with `interactive-job.close-owned-disposer-missing` the moment a lane answers `None`, so an
    /// editor must declare how EVERY store it types is released, not only the ones it edits.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    /// 🫧️ `Wfc2dTransient` carries the solved assignment, so its lane needs the BOUNDED disposer that
    /// retires a real root, not the `NoTransient` shim.
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    /// 📬️ The document lane's one-item retained preparation. Without one, EVERY route declaring
    /// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract
    /// and stays dispatch-dead however it is classified.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("wfc2d-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 📬️ The same, for the pane config the camera and armed-tile verbs publish into.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("wfc2d-config", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🫧️ The transient lane's own preparation and root retirement — `solve` declares
    /// `ArtifactToolPublicationLane::Transient`, and the lane is refused as unsupported unless BOTH
    /// exist.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    /// 🗃️ The retained initialization authority a whole-document `Effect::LoadDocument` needs. Without
    /// it the framework refuses every persisted replacement with
    /// `artifact-store.persisted-initializer-refused`, which is what the example picker's own load is.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, WFC_2D_DOCUMENT_SCHEMA, operation, generation))
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        fill::build_tool_run_job(request)
    }

    fn initial_snapshot() -> Wfc2dSnapshot {
        crate::examples::two_room_corridor::document()
    }

    fn command_id(command: &Self::Command) -> &'static str {
        wfc2d_command_id(command)
    }

    /// 🕹️ The `slot` domain's addressable ids — the framework validates every selection write against
    /// this, so a node the canvas picks is only ever selectable while the document still declares it.
    /// Flat: adjacency is a peer relation, never a parent one.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> protocol::InteractionTopology {
        let ordered = doc.snapshot.slots.iter().map(|slot| TopologyNode { id: slot.id.clone(), granularity: "slot".into(), parent: None }).collect();
        protocol::InteractionTopology { domains: [("slot".to_string(), protocol::DomainTopology { ordered })].into_iter().collect() }
    }

    /// 🌉️ The args bridge every UI dispatch crosses — typed against `dsl::DslValue`, never
    /// `serde_json::Value`: without this override the trait's own default answers
    /// `app.command.unsupported` and every button in every pane is inert.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        Ok(match action {
            "change-seed" => Wfc2dEditorCommand::ChangeSeed { seed: arg_u64(args, "seed", 0) },
            "create-slot" => Wfc2dEditorCommand::CreateSlot {
                id: arg_string(args, "id"),
                x: arg_f64(args, "x", 0.0),
                y: arg_f64(args, "y", 0.0),
                width: arg_f64(args, "width", 1.0),
                height: arg_f64(args, "height", 1.0),
            },
            "delete-slot" => Wfc2dEditorCommand::DeleteSlot { id: arg_string(args, "id") },
            "move-slot" => Wfc2dEditorCommand::MoveSlot { id: arg_string(args, "id"), x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0) },
            "resize-slot" => Wfc2dEditorCommand::ResizeSlot { id: arg_string(args, "id"), width: arg_f64(args, "width", 1.0), height: arg_f64(args, "height", 1.0) },
            "connect-slots" => Wfc2dEditorCommand::ConnectSlots {
                id: arg_string(args, "id"),
                from_slot_id: arg_string(args, "fromSlotId"),
                to_slot_id: arg_string(args, "toSlotId"),
                relation: arg_string(args, "relation"),
            },
            "disconnect-slots" => Wfc2dEditorCommand::DisconnectSlots { id: arg_string(args, "id") },
            "pin-slot" => Wfc2dEditorCommand::PinSlot { id: arg_string(args, "id"), tile_id: arg_string(args, "tileId") },
            "unpin-slot" => Wfc2dEditorCommand::UnpinSlot { id: arg_string(args, "id") },
            "create-tile" => Wfc2dEditorCommand::CreateTile { id: arg_string(args, "id"), label: arg_optional_string(args, "label"), weight: arg_f64(args, "weight", 1.0) },
            "delete-tile" => Wfc2dEditorCommand::DeleteTile { id: arg_string(args, "id") },
            "change-tile-weight" => Wfc2dEditorCommand::ChangeTileWeight { tile_id: arg_string(args, "tileId"), weight: arg_f64(args, "weight", 1.0) },
            "change-tile-media" => Wfc2dEditorCommand::ChangeTileMedia { tile_id: arg_string(args, "tileId") },
            "create-rule" => Wfc2dEditorCommand::CreateRule {
                id: arg_string(args, "id"),
                tile_a_id: arg_string(args, "tileAId"),
                tile_b_id: arg_string(args, "tileBId"),
                relation: arg_optional_string(args, "relation"),
                allowed: arg_bool(args, "allowed", true),
            },
            "delete-rule" => Wfc2dEditorCommand::DeleteRule { id: arg_string(args, "id") },
            "change-camera" => Wfc2dEditorCommand::ChangeCamera { x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0), zoom: arg_f64(args, "zoom", 1.0) },
            "change-active-tile" => Wfc2dEditorCommand::ChangeActiveTile { tile_id: arg_string(args, "tileId") },
            "solve" => Wfc2dEditorCommand::Solve,
            fill::COMMIT_FILL_ACTION_ID => {
                let payload_json = {
                    let camel = arg_string(args, "payloadJson");
                    if !camel.is_empty() {
                        camel
                    } else {
                        let snake = arg_string(args, "payload_json");
                        if !snake.is_empty() {
                            snake
                        } else {
                            arg_string(args, "value")
                        }
                    }
                };
                if payload_json.is_empty() {
                    return Err(Fault::from("wfc2d-commit-fill-args"));
                }
                Wfc2dEditorCommand::CommitFill { payload_json }
            },
            WFC_2D_NODE_GRAPH_EDIT => Wfc2dEditorCommand::NodeGraphEdit { operations_json: arg_string(args, "operations") },
            WFC_2D_SET_ACTIVE_EXAMPLE => {
                let requested = arg_string(args, "exampleId");
                let example_id = if requested.is_empty() { crate::examples::two_room_corridor::ID.to_string() } else { requested };
                Wfc2dEditorCommand::SetActiveExample { example_id }
            }
            other => return Err(Fault::from(format!("wfc2d-unknown-action:{other}"))),
        })
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Wfc2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !WFC_2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = wfc2d_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.retained.tool-mismatch"), "WFC 2D command does not match its exact registered tool"));
        }
        if wfc2d_retained_extent(&request.command, &request.snapshot).is_none() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.retained.extent"), "WFC 2D bounded route exceeded its declared work extent"));
        }
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(Wfc2dCommandWork::new(tool_id));
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
            wfc2d_command_id,
            WFC_2D_RETAINED_RAW_BYTES,
            WFC_2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Wfc2dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.wfc2d@1/*#editor",
        artifact_schema: "s.wfc.wfc2d",
        factory: "Wfc2dRetainedCommandJobFactory",
        factory_type: Wfc2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed", "create-slot", "delete-slot", "move-slot", "resize-slot", "connect-slots", "disconnect-slots", "pin-slot", "unpin-slot",
            "create-tile", "delete-tile", "change-tile-weight", "change-tile-media", "create-rule", "delete-rule",
            "change-camera", "change-active-tile", "solve", "commit-fill", "setActiveExample", "nodeGraphEdit"
        ]
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        dispatch(command, doc.snapshot, cfg.snapshot)
    }

    /// 🫧️ The bare `render` is the trait's transient-LESS entry point: its signature carries no
    /// transient lane at all, so this path can only paint what the DOCUMENT says (authored pins).
    /// Every host request goes through `render_with_request_context` below, which does carry the
    /// lane; this exists for callers that have no request context to offer.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, &Wfc2dTransient::default(), None)
    }

    /// 🫧️ The HOST-FACING render: the framework hands the app-local transient store in here, so this
    /// is where the solved assignment actually reaches `wfc-2d-preview`. Delegates to the free
    /// `render_with_transient` below so a test can drive the same code without an `InteractionView`
    /// (whose fields are `pub(crate)` in the framework crate and cannot be built from here).
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_with_transient(body_key, doc, cfg, transient)
    }
}

/// 🫧️ The exact projection `render_with_request_context` performs, minus the two arguments a test
/// cannot construct. `transient.snapshot` IS the app-local `Wfc2dTransient` the solve job publishes
/// into, so the preview window paints the LAST SOLVE here and authored pins only where the solve has
/// not reached yet.
pub fn render_with_transient(
    body_key: &str,
    doc: &ArtifactView<'_, Wfc2dSnapshot>,
    cfg: &ConfigView<'_, Wfc2dConfig>,
    transient: &semio_framework_plugin::TransientView<'_, Wfc2dTransient>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot, doc.tool_run())
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
/// 🀄️ One app-level catalogue action. `build_definition` pushes every action NO window kind owns
/// explicitly onto EVERY window kind, which is exactly what makes these reachable from either pane
/// and what stops `undeclaredActionDiagnostic` dropping them.
fn wfc2d_action(id: &'static str, en: &'static str, de: &'static str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), kind)
}

/// 🗃️ The example picker's own argument form — one option per `crate::examples::sources()` row, in
/// picker order, defaulting to the document the editor boots on.
fn wfc2d_example_arg() -> Vec<ActionArgDef> {
    vec![ActionArgDef::select(
        "exampleId",
        LocalizedLabel::native("Example", "Beispiel"),
        vec![
            ActionArgOption::new(crate::examples::two_room_corridor::ID, crate::examples::two_room_corridor::label()),
            ActionArgOption::new(crate::examples::wall_roof_facade_strip::ID, crate::examples::wall_roof_facade_strip::label()),
            ActionArgOption::new(crate::examples::hex_ring::ID, crate::examples::hex_ring::label()),
            ActionArgOption::new(crate::examples::terrain_ring::ID, crate::examples::terrain_ring::label()),
        ],
    )
    .required()
    .default_value(&crate::examples::two_room_corridor::ID)]
}

/// 📝️ Staged argument forms. An entity-naming verb with no form dispatches an EMPTY id the
/// instant its palette row is activated, which `dispatch`'s guards now refuse by name — the
/// form is what makes the row usable rather than merely honest.
/// 🕹️ Declaring ONE interaction domain is what injects the framework's own
/// `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll` verbs. Without it the
/// NodeGraph canvas' every node click is dropped as `undeclared-action`.
pub fn create_wfc2d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC_2D_DIALECT)
        .document(["semio", "wfc", "2d"])
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_2D_EDIT_MODE_ID)
        .tool(fill::definition())
        .window_kind_def(graph::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .window_kind_actions(preview::WFC_2D_PREVIEW_WINDOW, vec![wfc2d_action("solve", "Solve", "Lösen", ActionKind::Mutation)])
        .interaction(InteractionDefinition {
            id: "slot".into(),
            label: LocalizedLabel::native("Slots", "Slots"),
            granularities: vec![GranularityDefinition { id: "slot".into(), label: LocalizedLabel::native("Slot", "Slot"), icon_id: "square-dashed".into() }],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Single, SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: false },
        })
        .window_kind_interactions(graph::WFC_GRAPH_WINDOW, vec![InteractionRef::new("slot")])
        .action_with(ActionDefinition::new(WFC_2D_SET_ACTIVE_EXAMPLE, LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
        .action_args(WFC_2D_SET_ACTIVE_EXAMPLE, wfc2d_example_arg())
        .action_destructive("delete-slot")
        .action_destructive(WFC_2D_SET_ACTIVE_EXAMPLE)
        .action_with(wfc2d_action("create-tile", "Create Tile", "Kachel erstellen", ActionKind::Mutation))
        .action_with(wfc2d_action("delete-tile", "Delete Tile", "Kachel löschen", ActionKind::Mutation))
        .action_destructive("delete-tile")
        .action_with(wfc2d_action("change-tile-weight", "Change Tile Weight", "Kachelgewicht ändern", ActionKind::Mutation))
        .action_with(wfc2d_action("change-tile-media", "Clear Tile Media", "Kachelmedium leeren", ActionKind::Mutation))
        .action_with(wfc2d_action("create-rule", "Create Rule", "Regel erstellen", ActionKind::Mutation))
        .action_with(wfc2d_action("delete-rule", "Delete Rule", "Regel löschen", ActionKind::Mutation))
        .action_destructive("delete-rule")
        .action_with(wfc2d_action("change-camera", "Change Camera", "Kamera ändern", ActionKind::View))
        .action_with(wfc2d_action("change-active-tile", "Arm Tile", "Kachel aktivieren", ActionKind::View))
        .action_with(wfc2d_action(WFC_2D_NODE_GRAPH_EDIT, "Edit Graph", "Graph bearbeiten", ActionKind::Mutation))
        .action_args("create-tile", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).default_value(&1.0),
        ])
        .action_args("delete-tile", vec![ActionArgDef::text("id", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("change-tile-weight", vec![
            ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).default_value(&1.0),
        ])
        .action_args("change-tile-media", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("create-rule", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Rule", "Regel")).required(),
            ActionArgDef::text("tileAId", LocalizedLabel::native("Tile A", "Kachel A")).required(),
            ActionArgDef::text("tileBId", LocalizedLabel::native("Tile B", "Kachel B")).required(),
            ActionArgDef::text("relation", LocalizedLabel::native("Relation", "Relation")),
            ActionArgDef::toggle("allowed", LocalizedLabel::native("Allowed", "Erlaubt")).default_value(&true),
        ])
        .action_args("delete-rule", vec![ActionArgDef::text("id", LocalizedLabel::native("Rule", "Regel")).required()])
        .action_args("change-active-tile", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .interactive_jobs(InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
