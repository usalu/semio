//! ✏️ WFC 3D editor — the `ArtifactEditor` for `s.wfc.wfc3d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas (copied verbatim from `wfc2d`, so both artifacts present ONE window kind
//! to the shell) and the read-only `wfc-3d-preview` World3d pane.
//!
//! Every command maps 1:1 onto a real `Wfc3dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The command families that are NOT document mutations (`change-camera`, `change-active-tile`) go to
//! the per-pane config instead: a camera is per-window view state, and the solve is an INFERENCE, so
//! letting either reach the document lanes would make a derived value look persisted and pollute undo.
//!
//! Collection inserts always land at the collection's CANONICAL SORTED position
//! (`canonical_*_index`), so a delete followed by its inverse puts the row back exactly where it was.

#![allow(clippy::result_large_err)]

use crate::editor::wfc3d::config::{wfc3d_active_tile_id, Wfc3dConfig, Wfc3dConfigMutation};
use crate::editor::wfc3d::modes::edit;
use crate::editor::wfc3d::modes::edit::tools::fill;
use crate::editor::wfc3d::modes::edit::windows::{graph, preview};
use crate::editor::wfc3d::transient::{SetSolve, Wfc3dTransient, Wfc3dTransientMutation};
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{canonical_edge_index, canonical_insertion_index, canonical_rule_index, canonical_slot_index, canonical_tile_index, GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d};
use crate::{Wfc3dMutation, Wfc3dSnapshot, WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{Effect, RequestId, ToolRef, ToolRunJob, ToolRunJobPurpose, ToolRunJobRequest, ToolRunView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef,
    Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc3dMutation` kind a UI can
/// trigger, plus the two non-document verbs (camera, armed tile).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Wfc3dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "create-slot")]
    CreateSlot { id: String, x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64 },
    #[dsl(key = "delete-slot")]
    DeleteSlot { id: String },
    /// 🚚️ `z` is OPTIONAL because the `wfc-graph` window is shared with `wfc2d` and its staged form
    /// therefore offers only `x`/`y`. An absent `z` means "keep the slot's authored height", never
    /// "zero" — a 2d form must not be able to flatten a stack it cannot even see.
    #[dsl(key = "move-slot")]
    MoveSlot { id: String, x: f64, y: f64, z: Option<f64> },
    /// 📐️ `depth` is optional for the same reason, with the same meaning.
    #[dsl(key = "resize-slot")]
    ResizeSlot { id: String, width: f64, height: f64, depth: Option<f64> },
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
    /// 🕹️ The NodeGraph canvas' OWN gesture channel: dragging a node and completing a wire both
    /// arrive as `nodeGraphEdit` carrying one `operations` entry, never as `move-slot`/`connect-slots`
    /// directly. Coalescing is the canvas': the drag settles once per gesture, so one drag lands
    /// exactly one `move-slot`.
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
    /// 🎥️ The canvas' settled camera. Its own verb rather than `change-camera`, because a command's
    /// id must equal the action it was admitted under.
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { x: f64, y: f64, zoom: f64 },
}

impl protocol::OpBinary for Wfc3dEditorCommand {
    /// 🧵️ The list `validate_tool_job_rows` joins against: `TOOL_JOB_IDS ∩ migrated` must EQUAL the
    /// `bounded_first_step_tool_proofs!` rows, or the app boots with `interactive-job.catalog-incomplete`.
    const TOOL_JOB_IDS: &'static [&'static str] = WFC_3D_RETAINED_TOOL_IDS;

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
pub fn wfc3d_command_id(command: &Wfc3dEditorCommand) -> &'static str {
    match command {
        Wfc3dEditorCommand::ChangeSeed { .. } => "change-seed",
        Wfc3dEditorCommand::CreateSlot { .. } => "create-slot",
        Wfc3dEditorCommand::DeleteSlot { .. } => "delete-slot",
        Wfc3dEditorCommand::MoveSlot { .. } => "move-slot",
        Wfc3dEditorCommand::ResizeSlot { .. } => "resize-slot",
        Wfc3dEditorCommand::ConnectSlots { .. } => "connect-slots",
        Wfc3dEditorCommand::DisconnectSlots { .. } => "disconnect-slots",
        Wfc3dEditorCommand::PinSlot { .. } => "pin-slot",
        Wfc3dEditorCommand::UnpinSlot { .. } => "unpin-slot",
        Wfc3dEditorCommand::CreateTile { .. } => "create-tile",
        Wfc3dEditorCommand::DeleteTile { .. } => "delete-tile",
        Wfc3dEditorCommand::ChangeTileWeight { .. } => "change-tile-weight",
        Wfc3dEditorCommand::ChangeTileMedia { .. } => "change-tile-media",
        Wfc3dEditorCommand::CreateRule { .. } => "create-rule",
        Wfc3dEditorCommand::DeleteRule { .. } => "delete-rule",
        Wfc3dEditorCommand::ChangeCamera { .. } => "change-camera",
        Wfc3dEditorCommand::ChangeActiveTile { .. } => "change-active-tile",
        Wfc3dEditorCommand::Solve => "solve",
        Wfc3dEditorCommand::CommitFill { .. } => fill::COMMIT_FILL_ACTION_ID,
        Wfc3dEditorCommand::SetActiveExample { .. } => WFC_3D_SET_ACTIVE_EXAMPLE,
        Wfc3dEditorCommand::NodeGraphEdit { .. } => WFC_3D_NODE_GRAPH_EDIT,
        Wfc3dEditorCommand::NodeGraphViewport { .. } => WFC_3D_NODE_GRAPH_VIEWPORT,
    }
}

/// 🔗 The relation a connect gesture authors when the caller names none. The graph canvas drags a
/// wire between two `adjacent` ports; that is the relation it means.
pub const WFC_3D_DEFAULT_RELATION: &str = "adjacent";

/// 📏️ Canvas units one authored slot unit occupies in the `wfc-graph` pane. A slot is a BOX in
/// metres, so a document authored at `width: 1.0` would paint a one-pixel node at viewport zoom 1 and
/// three slots a metre apart would overlap inside the canvas' own minimum node size — the pane was
/// legible only as a smudge. The scale is the caller's, never the shared window's: the window file
/// stays byte-identical to `wfc2d`'s, and its inverse ([`slot_coordinate`]) is what turns a released
/// drag back into a `move-slot` in document units.
pub const WFC_3D_GRAPH_UNIT: f64 = 120.0;

/// 📐️ The document coordinate one canvas coordinate means — [`WFC_3D_GRAPH_UNIT`]'s exact inverse.
pub fn slot_coordinate(canvas: f64) -> f64 {
    canvas / WFC_3D_GRAPH_UNIT
}

/// 🕹️ The interaction domain the graph canvas selects and hovers in. Declaring ONE domain is what
/// makes the framework inject `interactionSelect`/`interactionHover`/`clearSelection` onto every
/// window kind; without it the shell drops every pointer gesture as `undeclared-action`.
pub const WFC_3D_INTERACTION_GRAPH: &str = "slot";
//#endregion 🔖️Command

//#region 🧵️Retained
/// 🎨️ The action id the shell's own navbar example picker dispatches — fixed by
/// `SET_ACTIVE_EXAMPLE_ACTION_ID` in `🛠️ShellHelpers`, so it is camelCase where every wfc verb is
/// kebab-case.
pub const WFC_3D_SET_ACTIVE_EXAMPLE: &str = "setActiveExample";

/// 🕹️ The action id the NodeGraph canvas host dispatches every node drag and every completed wire
/// under (`nodeGraphActions.edit`) — likewise fixed by the renderer, likewise camelCase.
pub const WFC_3D_NODE_GRAPH_EDIT: &str = "nodeGraphEdit";

/// 🎥️ The action id the NodeGraph canvas host dispatches its SETTLED camera under — once per camera
/// gesture and once when the canvas first fits the document, never per wheel tick.
pub const WFC_3D_NODE_GRAPH_VIEWPORT: &str = "nodeGraphViewport";

/// 🧵️ Every verb a pane may dispatch. An id absent here carries NO app-owned retained factory, and a
/// bare bounded proof faults with `interactive-job.missing-owned-reducer` — the verb is then dead in
/// the running app however it is classified.
pub const WFC_3D_RETAINED_TOOL_IDS: &[&str] = &[
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
    WFC_3D_SET_ACTIVE_EXAMPLE,
    WFC_3D_NODE_GRAPH_EDIT,
    WFC_3D_NODE_GRAPH_VIEWPORT,
];

const WFC_3D_RETAINED_PAYLOAD_SCHEMA: &str = "wfc.wfc3d.tool-command.v1";
const WFC_3D_RETAINED_RAW_BYTES: usize = 65_536;
const WFC_3D_RETAINED_WORK_ITEMS: usize = 4_096;

/// 📄️ One document-lane publication row.
const fn artifact_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] }
}

/// 🧮️ One pane-config publication row — camera and armed tile are view state, never document state.
const fn config_route(tool_id: &'static str) -> semio_framework_plugin::ArtifactToolPublicationContract {
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] }
}

/// 🛣️ One lane per route, read off each arm of `command_emit`. The example picker is `HostOnly`: it
/// answers a whole-document `Effect::LoadDocument`, which is not an edit, so re-picking the boot
/// example mints no phantom undo entry.
const WFC_3D_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
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
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "solve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: fill::COMMIT_FILL_ACTION_ID, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: WFC_3D_SET_ACTIVE_EXAMPLE, lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    artifact_route(WFC_3D_NODE_GRAPH_EDIT),
    config_route(WFC_3D_NODE_GRAPH_VIEWPORT),
];

fn wfc3d_retained_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(WFC_3D_RETAINED_RAW_BYTES, WFC_3D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 📏️ One bounded first step per route, admitted only while the WHOLE addressable document plus this
/// edit fit inside `WFC_3D_RETAINED_WORK_ITEMS`.
fn wfc3d_retained_extent(command: &Wfc3dEditorCommand, snapshot: &Wfc3dSnapshot) -> Option<usize> {
    if !WFC_3D_RETAINED_TOOL_IDS.contains(&wfc3d_command_id(command)) {
        return None;
    }
    let items = snapshot.slots.len().checked_add(snapshot.edges.len())?.checked_add(snapshot.tiles.len())?.checked_add(snapshot.rules.len())?.checked_add(1)?;
    (items <= WFC_3D_RETAINED_WORK_ITEMS).then_some(1)
}

/// 🧵️ The ONE bounded work step every wfc3d route runs — the pure `command_emit` reducer.
struct Wfc3dCommandWork {
    tool_id: &'static str,
    completed: bool,
}

impl Wfc3dCommandWork {
    fn new(tool_id: &'static str) -> Self {
        Self { tool_id, completed: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Wfc3dEditor>> for Wfc3dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Wfc3dEditorCommand,
        snapshot: &Wfc3dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<Wfc3dEditor>>>,
    ) -> Option<usize> {
        (!self.completed && wfc3d_command_id(command) == self.tool_id).then(|| wfc3d_retained_extent(command, snapshot)).flatten()
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<Wfc3dEditor>>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<Wfc3dEditor>>, Fault> {
        if self.completed || wfc3d_command_id(input.command) != self.tool_id {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("wfc3d.retained.route"),
                "the bounded WFC 3D work rejects an undeclared or completed route",
            ));
        }
        let emit = command_emit(input.command, input.snapshot, input.config)?;
        let transient = match input.command {
            Wfc3dEditorCommand::CommitFill { payload_json } => {
                let payload = fill::decode_fill_payload(payload_json.as_bytes()).ok_or_else(|| Fault::from("wfc3d-commit-fill-payload"))?;
                payload.set_solve_mutations()
            }
            _ => Vec::new(),
        };
        self.completed = true;
        Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral {
            emit,
            ephemeral: EphemeralEmit { presence: Vec::new(), transient, window_transient: Vec::new() },
        })
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block binds this exact
/// Rust type to `EditorApp<Wfc3dEditor>`, which is what turns a bare (and therefore dispatch-dead)
/// bounded proof into an exact-owner proof.
struct Wfc3dRetainedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl Wfc3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: WFC_3D_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Wfc3dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<Wfc3dEditor>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<Wfc3dEditor>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WFC_3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        wfc3d_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > WFC_3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("bounded WFC 3D command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Wfc3dRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<Wfc3dEditor>;
    const TOOL_IDS: &'static [&'static str] = WFC_3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = WFC_3D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️Retained

//#region 🔖️GraphView
/// 🕸️ The `wfc-graph` window's whole view of this document — the projection lives HERE, not in the
/// window, which is what lets `wfc2d` and `wfc3d` share one window file byte for byte. `z` is
/// deliberately dropped: the graph canvas is a 2d plan of an arbitrary graph, and the third axis is
/// the preview pane's business.
pub struct Wfc3dGraphView<'a>(pub &'a Wfc3dSnapshot);

impl graph::SlotGraphView for Wfc3dGraphView<'_> {
    fn graph_slots(&self) -> Vec<graph::GraphSlotView> {
        self.0
            .slots
            .iter()
            .map(|slot| graph::GraphSlotView {
                id: slot.id.clone(),
                x: slot.x * WFC_3D_GRAPH_UNIT,
                y: slot.y * WFC_3D_GRAPH_UNIT,
                width: slot.width * WFC_3D_GRAPH_UNIT,
                height: slot.height * WFC_3D_GRAPH_UNIT,
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
pub struct Wfc3dEditor;

impl ArtifactEditor for Wfc3dEditor {
    type Snapshot = Wfc3dSnapshot;
    type Mutation = Wfc3dMutation;
    type Config = Wfc3dConfig;
    type ConfigMutation = Wfc3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = Wfc3dTransient;
    type TransientMutation = Wfc3dTransientMutation;
    type Command = Wfc3dEditorCommand;

    const DIALECT: Dialect = WFC3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
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

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    /// 🏗️ Admits the whole-document replacement `setActiveExample`'s [`Effect::LoadDocument`] drives
    /// through the host's persisted-envelope replacement. The trait default REFUSES the envelope, so
    /// without this every example switch faults at the archive-load boundary instead of swapping the
    /// document.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, WFC3D_DOCUMENT_SCHEMA, operation, generation))
    }

    /// 📬️ The document lane's one-item retained preparation. Without one, EVERY route declaring
    /// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract
    /// and stays dispatch-dead however it is classified.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("wfc3d-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 📬️ The same, for the pane config the camera and armed-tile verbs publish into.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("wfc3d-config", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn initial_snapshot() -> Wfc3dSnapshot {
        crate::examples::two_room_corridor::snapshot()
    }

    fn command_id(command: &Self::Command) -> &'static str {
        wfc3d_command_id(command)
    }

    /// 🕹️ The `slot` domain's addressable ids — the framework validates every selection write against
    /// this, so a node the canvas picks is only ever selectable while the document still declares it.
    /// Flat: adjacency is a peer relation, never a parent one.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> protocol::InteractionTopology {
        let ordered = doc.snapshot.slots.iter().map(|slot| protocol::TopologyNode { id: slot.id.clone(), granularity: "slot".into(), parent: None }).collect();
        protocol::InteractionTopology { domains: [(WFC_3D_INTERACTION_GRAPH.to_string(), protocol::DomainTopology { ordered })].into_iter().collect() }
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Wfc3dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !WFC_3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = wfc3d_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.retained.tool-mismatch"), "WFC 3D command does not match its exact registered tool"));
        }
        if wfc3d_retained_extent(&request.command, &request.snapshot).is_none() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.retained.extent"), "WFC 3D bounded route exceeded its declared work extent"));
        }
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = Box::new(Wfc3dCommandWork::new(tool_id));
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
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
            wfc3d_command_id,
            WFC_3D_RETAINED_RAW_BYTES,
            WFC_3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Wfc3dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.wfc3d@1/*#editor",
        artifact_schema: "s.wfc.wfc3d",
        factory: "Wfc3dRetainedCommandJobFactory",
        factory_type: Wfc3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed", "create-slot", "delete-slot", "move-slot", "resize-slot", "connect-slots", "disconnect-slots", "pin-slot", "unpin-slot",
            "create-tile", "delete-tile", "change-tile-weight", "change-tile-media", "create-rule", "delete-rule",
            "change-camera", "change-active-tile", "solve", "commit-fill", "setActiveExample", "nodeGraphEdit", "nodeGraphViewport"
        ]
    }

    /// 🎯️ Maps the shell's stringly `{action, args}` wire onto `Wfc3dEditorCommand`. The trait default
    /// REFUSES every app action ("dispatched exclusively through the typed command channel"), which is
    /// why — before this bridge existed — every Actions-pane row, the navbar example picker and every
    /// canvas gesture answered `dispatch-failed` in the live playground. Each key below is the
    /// `ActionArgDef.id` that action declares.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_bool);
        let viewport = |key: &str| args.and_then(|value| value.get("viewport")).and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64);
        match action {
            "setActiveExample" => Ok(Wfc3dEditorCommand::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() }),
            "nodeGraphEdit" => Ok(Wfc3dEditorCommand::NodeGraphEdit { operations_json: graph_operations_json(args) }),
            "nodeGraphViewport" => Ok(Wfc3dEditorCommand::NodeGraphViewport {
                x: viewport("x").or_else(|| number("x")).unwrap_or_default(),
                y: viewport("y").or_else(|| number("y")).unwrap_or_default(),
                zoom: viewport("zoom").or_else(|| number("zoom")).filter(|zoom| *zoom > 0.0).unwrap_or(1.0),
            }),
            "change-seed" => Ok(Wfc3dEditorCommand::ChangeSeed { seed: number("seed").filter(|seed| *seed >= 0.0).unwrap_or_default() as u64 }),
            "create-slot" => Ok(Wfc3dEditorCommand::CreateSlot {
                id: text("id").unwrap_or_default(),
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                z: number("z").unwrap_or_default(),
                width: number("width").unwrap_or(1.0),
                height: number("height").unwrap_or(1.0),
                depth: number("depth").unwrap_or(1.0),
            }),
            "delete-slot" => Ok(Wfc3dEditorCommand::DeleteSlot { id: text("id").unwrap_or_default() }),
            "move-slot" => Ok(Wfc3dEditorCommand::MoveSlot { id: text("id").unwrap_or_default(), x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), z: number("z") }),
            "resize-slot" => Ok(Wfc3dEditorCommand::ResizeSlot { id: text("id").unwrap_or_default(), width: number("width").unwrap_or(1.0), height: number("height").unwrap_or(1.0), depth: number("depth") }),
            "connect-slots" => Ok(Wfc3dEditorCommand::ConnectSlots {
                id: text("id").unwrap_or_default(),
                from_slot_id: text("fromSlotId").unwrap_or_default(),
                to_slot_id: text("toSlotId").unwrap_or_default(),
                relation: text("relation").unwrap_or_default(),
            }),
            "disconnect-slots" => Ok(Wfc3dEditorCommand::DisconnectSlots { id: text("id").unwrap_or_default() }),
            "pin-slot" => Ok(Wfc3dEditorCommand::PinSlot { id: text("id").unwrap_or_default(), tile_id: text("tileId").unwrap_or_default() }),
            "unpin-slot" => Ok(Wfc3dEditorCommand::UnpinSlot { id: text("id").unwrap_or_default() }),
            "create-tile" => Ok(Wfc3dEditorCommand::CreateTile { id: text("id").unwrap_or_default(), label: text("label").filter(|label| !label.is_empty()), weight: number("weight").unwrap_or(1.0) }),
            "delete-tile" => Ok(Wfc3dEditorCommand::DeleteTile { id: text("id").unwrap_or_default() }),
            "change-tile-weight" => Ok(Wfc3dEditorCommand::ChangeTileWeight { tile_id: text("tileId").unwrap_or_default(), weight: number("weight").unwrap_or(1.0) }),
            "change-tile-media" => Ok(Wfc3dEditorCommand::ChangeTileMedia { tile_id: text("tileId").unwrap_or_default() }),
            "create-rule" => Ok(Wfc3dEditorCommand::CreateRule {
                id: text("id").unwrap_or_default(),
                tile_a_id: text("tileAId").unwrap_or_default(),
                tile_b_id: text("tileBId").unwrap_or_default(),
                relation: text("relation").filter(|relation| !relation.is_empty()),
                allowed: flag("allowed").unwrap_or(true),
            }),
            "delete-rule" => Ok(Wfc3dEditorCommand::DeleteRule { id: text("id").unwrap_or_default() }),
            "change-camera" => Ok(Wfc3dEditorCommand::ChangeCamera { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), zoom: number("zoom").filter(|zoom| *zoom > 0.0).unwrap_or(1.0) }),
            "change-active-tile" => Ok(Wfc3dEditorCommand::ChangeActiveTile { tile_id: text("tileId").unwrap_or_default() }),
            "solve" => Ok(Wfc3dEditorCommand::Solve),
            action if action == fill::COMMIT_FILL_ACTION_ID => {
                let payload_json = text("payloadJson").or_else(|| text("payload_json")).or_else(|| text("value")).ok_or_else(|| Fault::from("wfc3d-commit-fill-args"))?;
                Ok(Wfc3dEditorCommand::CommitFill { payload_json })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.action.unknown"), format!("wfc3d declares no action '{other}'"))),
        }
    }

    /// ✏️ Dispatches straight onto the schema tree's own mutation builders. A pin gesture reads the
    /// PANE's armed tile rather than a global one, so two panes can pin different tiles.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        command_emit(command, doc.snapshot, cfg.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, &Wfc3dTransient::default(), None)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot, doc.tool_run())
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        fill::build_tool_run_job(request)
    }
}

/// ✏️ The whole command→mutation mapping, as a PURE function of the document and the pane config —
/// `handle` is the thin `ArtifactView`/`ConfigView` adapter over it. Split out so the mapping is
/// reachable from a unit test: `ArtifactView`'s own fields are private, so nothing outside the
/// framework can build one.
pub fn command_emit(command: &Wfc3dEditorCommand, document: &Wfc3dSnapshot, config: &Wfc3dConfig) -> Result<Emit<Wfc3dMutation, Wfc3dConfigMutation>, Fault> {
    {
        let (mutation, description) = match command {
            Wfc3dEditorCommand::ChangeCamera { x, y, zoom } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeCamera(crate::editor::wfc3d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
                return Ok(Emit { config_mutations: emitted, description: Some("Change camera".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::NodeGraphViewport { x, y, zoom } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeCamera(crate::editor::wfc3d::config::ChangeCamera { x: *x, y: *y, zoom: if *zoom > 0.0 { *zoom } else { 1.0 } })];
                return Ok(Emit { config_mutations: emitted, description: Some("Change camera".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::ChangeActiveTile { tile_id } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeActiveTile(crate::editor::wfc3d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
                return Ok(Emit { config_mutations: emitted, description: Some("Arm tile".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::Solve => {
                return Ok(Emit { effects: fill::start_fill_effects(), description: Some("Solve".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::CommitFill { .. } => {
                return Ok(Emit { description: Some("Commit fill".into()), ..Default::default() });
            }
            Wfc3dEditorCommand::SetActiveExample { example_id } => {
                let next = example_snapshot(example_id)?;
                return Ok(Emit { effects: vec![load_document_effect(&next)], description: Some(format!("Set active example {example_id}")), ..Default::default() });
            }
            Wfc3dEditorCommand::NodeGraphEdit { operations_json } => {
                let (mutations, description) = graph_edit_mutations(document, operations_json)?;
                if mutations.is_empty() {
                    return Ok(Emit::default());
                }
                return Ok(Emit { artifact_mutations: mutations, description: Some(description), ..Default::default() });
            }
            Wfc3dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            Wfc3dEditorCommand::CreateSlot { id, x, y, z, width, height, depth } => (
                create_slot(
                    canonical_slot_index(document, id),
                    Slot3d { id: id.clone(), x: *x, y: *y, z: *z, width: positive_or(*width), height: positive_or(*height), depth: positive_or(*depth), pinned_tile_id: None },
                ),
                format!("Create slot {id}"),
            ),
            Wfc3dEditorCommand::DeleteSlot { id } => (delete_slot(id.clone()), format!("Delete slot {id}")),
            Wfc3dEditorCommand::MoveSlot { id, x, y, z } => {
                let authored = document.slots.iter().find(|slot| &slot.id == id).map(|slot| slot.z).unwrap_or_default();
                (move_slot(id.clone(), *x, *y, z.unwrap_or(authored)), format!("Move slot {id}"))
            }
            Wfc3dEditorCommand::ResizeSlot { id, width, height, depth } => {
                let authored = document.slots.iter().find(|slot| &slot.id == id).map_or(1.0, |slot| slot.depth);
                (resize_slot(id.clone(), positive_or(*width), positive_or(*height), positive_or(depth.unwrap_or(authored))), format!("Resize slot {id}"))
            }
            Wfc3dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => (
                connect_slots(
                    canonical_edge_index(document, id),
                    SlotEdge { id: id.clone(), from_slot_id: from_slot_id.clone(), to_slot_id: to_slot_id.clone(), relation: if relation.is_empty() { WFC_3D_DEFAULT_RELATION.into() } else { relation.clone() } },
                ),
                format!("Connect {from_slot_id} to {to_slot_id}"),
            ),
            Wfc3dEditorCommand::DisconnectSlots { id } => (disconnect_slots(id.clone()), format!("Disconnect {id}")),
            Wfc3dEditorCommand::PinSlot { id, tile_id } => {
                let tile = if tile_id.is_empty() { wfc3d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
                let Some(tile) = tile else { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.tile.unknown-pin"), "No tile is armed and the document declares none.")) };
                (pin_slot(id.clone(), tile.clone()), format!("Pin {id} to {tile}"))
            }
            Wfc3dEditorCommand::UnpinSlot { id } => (unpin_slot(id.clone()), format!("Unpin slot {id}")),
            Wfc3dEditorCommand::CreateTile { id, label, weight } => (
                create_tile(canonical_tile_index(document, id), Tile { id: id.clone(), label: label.clone(), weight: positive_or(*weight), media: TileMedia3d::default() }),
                format!("Create tile {id}"),
            ),
            Wfc3dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
            Wfc3dEditorCommand::ChangeTileWeight { tile_id, weight } => (change_tile_weight(tile_id.clone(), positive_or(*weight)), format!("Change weight of {tile_id}")),
            Wfc3dEditorCommand::ChangeTileMedia { tile_id } => (change_tile_media(tile_id.clone(), crate::unit_box_media(None)), format!("Reset media of {tile_id}")),
            Wfc3dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => (
                create_rule(canonical_rule_index(document, id), GraphRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed }),
                format!("Create rule {id}"),
            ),
            Wfc3dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }
}

/// 🖼️ The whole per-window render, as a PURE function of the document and the pane config — the same
/// split, for the same reason.
/// 🎥️ The preview frames the document itself and deliberately does NOT read `camera_zoom`:
/// that field is the GRAPH pane's viewport, and the two panes share one config instance, so
/// feeding it here made a pan or a wheel on the graph canvas warp the 3d view (measured live:
/// one graph pan put the preview inside a tile). `camera_json`'s own framing distance is the
/// pose authority.
pub fn render_body(body_key: &str, document: &Wfc3dSnapshot, config: &Wfc3dConfig, transient: &Wfc3dTransient, tool_run: Option<&ToolRunView>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc3dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_3D_PREVIEW_BODY => {
            let fill = preview::live_fill_payload(tool_run);
            let paint = fill.as_ref().map(|payload| {
                let mut paint = payload.clone().into_transient();
                for event in &payload.trace {
                    paint.assignments.retain(|assignment| assignment.slot_id != event.slot_id);
                    paint.assignments.push(crate::editor::wfc3d::transient::Wfc3dAssignment { slot_id: event.slot_id.clone(), tile_id: event.tile_id.clone() });
                }
                paint
            }).unwrap_or_else(|| transient.clone());
            preview::render(document, &paint, 1.0).map(semio_framework_plugin::built_to_component_tree)
        }
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

//#region 🔖️Examples
/// 📚️ Every bundled example the navbar picker offers, as `(id, snapshot)` — the ONE roster
/// `setActiveExample` resolves against, in the order `crate::examples::sources()` declares them.
pub const WFC_3D_EXAMPLE_IDS: [&str; 3] = [crate::examples::two_room_corridor::ID, crate::examples::wall_roof_facade_strip::ID, crate::examples::tower_stack::ID];

/// 📚️ The document one example id names. An EMPTY id is the shell's own "the app's default document"
/// (the picker's `No example` row and the boot announcement both send it), so it resolves to the boot
/// example rather than faulting; an id this artifact never registered faults instead of silently
/// opening a blank document.
pub fn example_snapshot(example_id: &str) -> Result<Wfc3dSnapshot, Fault> {
    match example_id {
        "" => Ok(crate::examples::two_room_corridor::snapshot()),
        id if id == crate::examples::two_room_corridor::ID => Ok(crate::examples::two_room_corridor::snapshot()),
        id if id == crate::examples::wall_roof_facade_strip::ID => Ok(crate::examples::wall_roof_facade_strip::snapshot()),
        id if id == crate::examples::tower_stack::ID => Ok(crate::examples::tower_stack::snapshot()),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.example.unknown"), format!("wfc3d has no example '{other}'"))),
    }
}

/// 🌱️ Swaps the live document for `document` OUTSIDE the undo ledger — an example switch is a
/// different document, not an edit to this one, so it must never mint a history row (the "phantom
/// boot edit" the sibling artifacts hit when they replayed a whole example as mutations).
pub fn load_document_effect(document: &Wfc3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Wfc3dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("wfc", WFC3D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Examples

//#region 🔖️GraphEdit
/// 🕸️ The `operations` array a `nodeGraphEdit` dispatch carries, as JSON text. Both graph canvases —
/// the wasm dag surface and React Flow — speak this one vocabulary (`move`/`connect`/`disconnect`),
/// so the command carries it verbatim and the document-aware mapping happens in
/// [`graph_edit_mutations`], where `z` and the edge roster are actually readable.
pub fn graph_operations_json(args: Option<&dsl::DslValue>) -> String {
    args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), |operations| serde_json::Value::from(operations).to_string())
}

/// 🕸️ Lowers one released gesture onto real document mutations — ONE `Emit`, so a drag is ONE
/// undoable edit rather than one per pointer tick.
///
/// - `move` keeps the slot's authored `z`: the graph canvas is a plan and never saw the third axis,
///   so reading it back off the document is the only way a drag cannot silently flatten a stack.
/// - `connect` mints a deterministic edge id from its endpoints and is a no-op when that adjacency
///   already exists, because a duplicate id is a fatal mutation invariant, not a second edge.
/// - `disconnect` addresses an existing edge by id and is a no-op when it is already gone.
pub fn graph_edit_mutations(document: &Wfc3dSnapshot, operations_json: &str) -> Result<(Vec<Wfc3dMutation>, String), Fault> {
    let operations: Vec<serde_json::Value> = serde_json::from_str(operations_json).unwrap_or_default();
    let mut edge_ids: Vec<String> = document.edges.iter().map(|edge| edge.id.clone()).collect();
    let mut mutations = Vec::new();
    let mut description = String::new();
    for operation in &operations {
        let kind = operation.get("operation").and_then(serde_json::Value::as_str).unwrap_or_default();
        let string = |key: &str| operation.get(key).and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        let float = |key: &str| operation.get(key).and_then(serde_json::Value::as_f64).unwrap_or_default();
        match kind {
            "move" => {
                let node_id = string("nodeId");
                let Some(slot) = document.slots.iter().find(|slot| slot.id == node_id) else { continue };
                mutations.push(move_slot(slot.id.clone(), slot_coordinate(float("x")), slot_coordinate(float("y")), slot.z));
                description = format!("Move slot {node_id}");
            }
            "connect" => {
                let from = string("sourceNodeId");
                let to = string("targetNodeId");
                if from.is_empty() || to.is_empty() || from == to {
                    continue;
                }
                if !document.slots.iter().any(|slot| slot.id == from) || !document.slots.iter().any(|slot| slot.id == to) {
                    continue;
                }
                if document.edges.iter().any(|edge| edge.from_slot_id == from && edge.to_slot_id == to) {
                    continue;
                }
                let id = format!("edge-{from}-{to}");
                if edge_ids.contains(&id) {
                    continue;
                }
                mutations.push(connect_slots(
                    canonical_insertion_index(edge_ids.iter().cloned(), &id),
                    SlotEdge { id: id.clone(), from_slot_id: from.clone(), to_slot_id: to.clone(), relation: WFC_3D_DEFAULT_RELATION.into() },
                ));
                edge_ids.push(id);
                edge_ids.sort();
                description = format!("Connect {from} to {to}");
            }
            "disconnect" => {
                let id = string("synapseId");
                if !edge_ids.contains(&id) {
                    continue;
                }
                mutations.push(disconnect_slots(id.clone()));
                edge_ids.retain(|edge| edge != &id);
                description = format!("Disconnect {id}");
            }
            "setHostSnapshot" => {
                let snapshot = operation.get("hostSnapshotJson").and_then(serde_json::Value::as_str).unwrap_or_default();
                let (host_mutations, host_description) = host_snapshot_mutations(document, snapshot, &mut edge_ids);
                mutations.extend(host_mutations);
                if !host_description.is_empty() {
                    description = host_description;
                }
            }
            _ => continue,
        }
    }
    if description.is_empty() {
        description = "Edit graph".into();
    }
    Ok((mutations, description))
}

/// 🕸️ The WHOLE-graph commit the wasm node-graph surface answers a released gesture with
/// (`{"operation":"setHostSnapshot","hostSnapshotJson":…}`). That surface does not speak the narrow
/// `move`/`connect` vocabulary the React Flow fallback and the wgpu renderer do — it hands back its
/// own laid-out graph — so this diffs that graph against the document and emits ONLY what genuinely
/// changed. Without it a node drag on the live canvas moved the picture and nothing else: the guest
/// saw a commit it could not read and the node snapped back on the next publication.
///
/// The node/edge shape is the host's own (`{nodes:[{id,x,y}],edges:[{id,source,target}]}`), the very
/// fields `graphEditSignature` in `🕸️NodeGraph/🟦️.tsx` reads to decide a commit is owed at all.
/// ✂️ A removal is keyed on the ENDPOINT PAIR, never on the edge id. The canvas mints its own wire
/// ids (`wire-1`, …) rather than echoing the document's, so an id-keyed rule read every authored
/// edge as deleted and one wire gesture silently disconnected the whole graph — measured live:
/// drawing `room-a→room-b` dropped `corridor→room-b` and the solve quietly re-coloured around the
/// hole. The pair is unordered, because a wire drawn the other way round is the same adjacency.
/// 🔗 Unordered, like the removal rule below: the adjacency is what the solver reads, and a
/// wire the canvas hands back the other way round is the SAME adjacency, not a second one.
fn host_snapshot_mutations(document: &Wfc3dSnapshot, host_snapshot_json: &str, edge_ids: &mut Vec<String>) -> (Vec<Wfc3dMutation>, String) {
    let Ok(snapshot) = serde_json::from_str::<serde_json::Value>(host_snapshot_json) else {
        return (Vec::new(), String::new());
    };
    let mut mutations = Vec::new();
    let mut description = String::new();
    let empty = Vec::new();
    for node in snapshot.get("nodes").and_then(serde_json::Value::as_array).unwrap_or(&empty) {
        let (Some(id), Some(x), Some(y)) = (
            node.get("id").and_then(serde_json::Value::as_str),
            node.get("x").and_then(serde_json::Value::as_f64),
            node.get("y").and_then(serde_json::Value::as_f64),
        ) else {
            continue;
        };
        let Some(slot) = document.slots.iter().find(|slot| slot.id == id) else { continue };
        let (next_x, next_y) = (slot_coordinate(x), slot_coordinate(y));
        if (next_x - slot.x).abs() < WFC_3D_GRAPH_EPSILON && (next_y - slot.y).abs() < WFC_3D_GRAPH_EPSILON {
            continue;
        }
        mutations.push(move_slot(slot.id.clone(), next_x, next_y, slot.z));
        description = format!("Move slot {id}");
    }
    let host_edges: Vec<(String, String, String)> = snapshot
        .get("edges")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty)
        .iter()
        .filter_map(|edge| {
            Some((
                edge.get("id").and_then(serde_json::Value::as_str)?.to_string(),
                edge.get("source").and_then(serde_json::Value::as_str)?.to_string(),
                edge.get("target").and_then(serde_json::Value::as_str)?.to_string(),
            ))
        })
        .collect();
    for (id, source, target) in &host_edges {
        let (from, to) = (graph_node_of(source), graph_node_of(target));
        if from.is_empty() || to.is_empty() || from == to || edge_ids.iter().any(|edge| edge == id) {
            continue;
        }
        if document.edges.iter().any(|edge| (edge.from_slot_id == from && edge.to_slot_id == to) || (edge.from_slot_id == to && edge.to_slot_id == from)) {
            continue;
        }
        if !document.slots.iter().any(|slot| slot.id == from) || !document.slots.iter().any(|slot| slot.id == to) {
            continue;
        }
        let minted = format!("edge-{from}-{to}");
        if edge_ids.contains(&minted) {
            continue;
        }
        mutations.push(connect_slots(
            canonical_insertion_index(edge_ids.iter().cloned(), &minted),
            SlotEdge { id: minted.clone(), from_slot_id: from.clone(), to_slot_id: to.clone(), relation: WFC_3D_DEFAULT_RELATION.into() },
        ));
        edge_ids.push(minted);
        edge_ids.sort();
        description = format!("Connect {from} to {to}");
    }
    for edge in &document.edges {
        let present = host_edges.iter().any(|(_, source, target)| {
            let (from, to) = (graph_node_of(source), graph_node_of(target));
            (from == edge.from_slot_id && to == edge.to_slot_id) || (from == edge.to_slot_id && to == edge.from_slot_id)
        });
        if present {
            continue;
        }
        mutations.push(disconnect_slots(edge.id.clone()));
        edge_ids.retain(|id| id != &edge.id);
        description = format!("Disconnect {}", edge.id);
    }
    (mutations, description)
}

/// 📏️ How far a node may drift in DOCUMENT units before the commit counts it as a move. The canvas
/// round-trips through f32 pixels, so an untouched node comes back a hair off its authored position;
/// without this floor every gesture anywhere in the pane would mint a move for every slot.
const WFC_3D_GRAPH_EPSILON: f64 = 1e-6;

/// 🔌️ The node id inside a canvas endpoint: the surface addresses ports as `{node}@{port}`, so an
/// endpoint that already names a bare node passes through unchanged.
fn graph_node_of(endpoint: &str) -> String {
    endpoint.split('@').next().unwrap_or_default().to_string()
}
//#endregion 🔖️GraphEdit

/// 📐️ A slot extent or tile weight the UI left at zero still has to be positive, or the mutation's
/// own invariant guard refuses it — the editor supplies the honest unit default rather than emitting
/// a command it knows will be rejected.
fn positive_or(value: f64) -> f64 {
    if value > 0.0 {
        value
    } else {
        1.0
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
/// 🀄️ One app-level catalogue action. `build_definition` pushes every action NO window kind owns
/// explicitly onto EVERY window kind, which is exactly what makes these reachable from either pane
/// and what stops `undeclaredActionDiagnostic` dropping them.
fn wfc3d_action(id: &'static str, en: &'static str, de: &'static str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), kind)
}

/// 🀄️ The tile, rule and pane-state verbs the graph window does not own. `TOOL_JOB_IDS ∩
/// migrated` must EQUAL the proof rows exactly, so every retained id is declared here or on a
/// window kind — one missing row boots the app with `interactive-job.catalog-incomplete`.
/// 🕸️ The two verbs the NodeGraph canvas itself dispatches: a released drag/wire gesture and a
/// settled camera. Both are canvas vocabulary, not palette entries.
/// 🎨️ The navbar example picker dispatches `setActiveExample { exampleId }` at boot and on every
/// pick; it belongs to no window, so it is declared centrally and `build_definition`'s
/// unowned-action fallback attaches it to both window kinds.
/// 🕹️ ONE selection/hover domain over the slot nodes. Declaring it is what makes the framework
/// inject `interactionSelect`/`interactionHover`/`clearSelection` onto every window kind — the
/// shell drops an action no window kind declares, so without this domain every pointer move
/// over the graph canvas was refused `undeclared-action`.
pub fn create_wfc3d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC3D_DIALECT)
        .document(["semio", "wfc", "3d"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_3D_EDIT_MODE_ID)
        .tool(fill::definition())
        .mode_tools(edit::WFC_3D_EDIT_MODE_ID, vec![semio_framework_plugin::resolve_ready(ToolRef::new(fill::TOOL_ID))])
        .window_kind_def(graph::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .window_kind_actions(preview::WFC_3D_PREVIEW_WINDOW, vec![wfc3d_action("solve", "Solve", "Lösen", ActionKind::Mutation)])
        .interaction(InteractionDefinition {
            id: WFC_3D_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Slots", "Slots"),
            granularities: vec![GranularityDefinition { id: "slot".into(), label: LocalizedLabel::native("Slot", "Slot"), icon_id: "square-dashed".into() }],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        })
        .window_kind_interactions(graph::WFC_GRAPH_WINDOW, vec![InteractionRef::new(WFC_3D_INTERACTION_GRAPH)])
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_destructive("delete-slot")
        .action_args("setActiveExample", vec![
            ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                ActionArgOption::new(crate::examples::two_room_corridor::ID, crate::examples::two_room_corridor::label()),
                ActionArgOption::new(crate::examples::wall_roof_facade_strip::ID, crate::examples::wall_roof_facade_strip::label()),
                ActionArgOption::new(crate::examples::tower_stack::ID, crate::examples::tower_stack::label()),
            ])
            .required()
            .default_value(&crate::examples::two_room_corridor::ID),
        ])
        .action_interactive_job("setActiveExample", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation) })
        .action_interactive_job("nodeGraphEdit", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(WFC_3D_NODE_GRAPH_VIEWPORT, LocalizedLabel::native("Graph Viewport", "Graph-Ansicht"), ActionKind::View) })
        .action_interactive_job(WFC_3D_NODE_GRAPH_VIEWPORT, semio_framework::InteractiveJobClassification::Migrated)
        .action_with(wfc3d_action("create-tile", "Create Tile", "Kachel erstellen", ActionKind::Mutation))
        .action_with(wfc3d_action("delete-tile", "Delete Tile", "Kachel löschen", ActionKind::Mutation))
        .action_destructive("delete-tile")
        .action_with(wfc3d_action("change-tile-weight", "Change Tile Weight", "Kachelgewicht ändern", ActionKind::Mutation))
        .action_with(wfc3d_action("change-tile-media", "Reset Tile Media", "Kachelmedium zurücksetzen", ActionKind::Mutation))
        .action_with(wfc3d_action("create-rule", "Create Rule", "Regel erstellen", ActionKind::Mutation))
        .action_with(wfc3d_action("delete-rule", "Delete Rule", "Regel löschen", ActionKind::Mutation))
        .action_destructive("delete-rule")
        .action_with(wfc3d_action("change-camera", "Change Camera", "Kamera ändern", ActionKind::View))
        .action_with(wfc3d_action("change-active-tile", "Arm Tile", "Kachel aktivieren", ActionKind::View))
        .action_interactive_job("create-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("delete-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-tile-weight", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-tile-media", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("create-rule", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("delete-rule", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-camera", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-active-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("solve", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(fill::COMMIT_FILL_ACTION_ID, LocalizedLabel::native("Commit Fill", "Füllen übernehmen"), ActionKind::Mutation) })
        .action_interactive_job(fill::COMMIT_FILL_ACTION_ID, semio_framework::InteractiveJobClassification::Migrated)
        .action_args("create-tile", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required(),
            ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).required().default_value(&1.0),
        ])
        .action_args("delete-tile", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required()])
        .action_args("change-tile-weight", vec![
            ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).required().default_value(&1.0),
        ])
        .action_args("change-tile-media", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("create-rule", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required(),
            ActionArgDef::text("tileAId", LocalizedLabel::native("Tile A", "Kachel A")).required(),
            ActionArgDef::text("tileBId", LocalizedLabel::native("Tile B", "Kachel B")).required(),
            ActionArgDef::text("relation", LocalizedLabel::native("Relation", "Relation")),
        ])
        .action_args("delete-rule", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required()])
        .action_args("change-active-tile", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .default_layout(edit::layout())
        .action_describe("change-seed", LocalizedLabel::native("Sets the random seed the solve starts from; the same seed, slots and rules always produce the same assignment.", "Legt den Zufallsstartwert des Lösers fest; derselbe Startwert mit denselben Slots und Regeln ergibt immer dieselbe Belegung."))
        .action_describe("solve", LocalizedLabel::native("Starts the fill tool run that assigns a tile to every slot of the 3D slot graph along its adjacency edges; the result is a preview and does not change the document.", "Startet den Füll-Werkzeuglauf, der jedem Slot des 3D-Slotgraphen entlang seiner Nachbarschaftskanten eine Kachel zuweist; das Ergebnis ist eine Vorschau und ändert das Dokument nicht."))
        .action_describe("commit-fill", LocalizedLabel::native("Receives the result of a finished fill run for the preview window; nothing is written to the document.", "Nimmt das Ergebnis eines fertigen Füll-Laufs für das Vorschaufenster entgegen; ins Dokument wird nichts geschrieben."))
        .action_describe("create-slot", LocalizedLabel::native("Adds a new slot, a 3D box the solve assigns one tile to, with the given id, position and size; refused when the id is taken.", "Fügt einen neuen Slot hinzu, ein 3D-Feld, dem der Löser eine Kachel zuweist, mit der angegebenen Id, Position und Größe; eine vergebene Id wird abgelehnt."))
        .action_describe("delete-slot", LocalizedLabel::native("Removes a slot by id together with every adjacency edge connected to it.", "Entfernt einen Slot anhand seiner Id samt aller mit ihm verbundenen Nachbarschaftskanten."))
        .action_describe("move-slot", LocalizedLabel::native("Moves a slot so its lower corner sits at the given x, y and optional z; its size and connections stay the same.", "Verschiebt einen Slot so, dass seine untere Ecke bei x, y und optional z liegt; Größe und Verbindungen bleiben erhalten."))
        .action_describe("resize-slot", LocalizedLabel::native("Sets a slot's width, height and optional depth; the preview scales the assigned tile into this box, and a zero or negative size is refused.", "Legt Breite, Höhe und optional Tiefe eines Slots fest; die Vorschau skaliert die zugewiesene Kachel in dieses Feld, eine Größe von null oder weniger wird abgelehnt."))
        .action_describe("connect-slots", LocalizedLabel::native("Adds an adjacency edge with the given id and relation between two slots, so the solve must pick tiles the rules allow side by side there.", "Fügt zwischen zwei Slots eine Nachbarschaftskante mit der angegebenen Id und Relation hinzu, sodass der Löser dort nur laut Regeln verträgliche Kacheln wählt."))
        .action_describe("disconnect-slots", LocalizedLabel::native("Removes one adjacency edge by id, so its two slots no longer constrain each other.", "Entfernt eine Nachbarschaftskante anhand ihrer Id, sodass sich ihre beiden Slots nicht mehr gegenseitig einschränken."))
        .action_describe("pin-slot", LocalizedLabel::native("Fixes one slot to the given tile, or to the armed tile when none is given; the solve must keep it.", "Legt einen Slot auf die angegebene Kachel fest, ohne Angabe auf die gewählte Kachel; der Löser muss sie beibehalten."))
        .action_describe("unpin-slot", LocalizedLabel::native("Releases the tile pin of one slot so the solve may choose any tile for it again.", "Löst die Kachelanheftung eines Slots, sodass der Löser wieder jede Kachel dafür wählen darf."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document (slots, edges, tiles, rules and seed) with one of the plugin's bundled 3D slot-graph examples, by example id.", "Ersetzt das gesamte Dokument (Slots, Kanten, Kacheln, Regeln und Startwert) durch eines der mitgelieferten 3D-Slotgraph-Beispiele, anhand der Beispiel-Id."))
        .action_describe("create-tile", LocalizedLabel::native("Adds a new placeable tile with the given id, label and weight to the tile catalogue; refused when the id is taken.", "Fügt dem Kachelkatalog eine neue setzbare Kachel mit der angegebenen Id, Bezeichnung und Gewichtung hinzu; eine vergebene Id wird abgelehnt."))
        .action_describe("delete-tile", LocalizedLabel::native("Removes a tile from the catalogue together with every rule that names it and every slot pinned to it.", "Entfernt eine Kachel aus dem Katalog samt aller Regeln, die sie nennen, und aller Slots, die auf sie angeheftet sind."))
        .action_describe("change-tile-weight", LocalizedLabel::native("Sets how strongly the solve favours one tile when several fit a slot; a higher weight makes it more frequent.", "Legt fest, wie stark der Löser eine Kachel bevorzugt, wenn mehrere in einen Slot passen; ein höheres Gewicht macht sie häufiger."))
        .action_describe("change-tile-media", LocalizedLabel::native("Clears the look of one tile back to the plain default box; its id, weight and rules stay unchanged.", "Setzt das Aussehen einer Kachel auf den schlichten Standardkörper zurück; Id, Gewicht und Regeln bleiben unverändert."))
        .action_describe("create-rule", LocalizedLabel::native("Allows or forbids tile B next to tile A under a named relation; along each edge the solve only places tile pairs the rules allow.", "Erlaubt oder verbietet Kachel B neben Kachel A unter einer benannten Relation; entlang jeder Kante setzt der Löser nur erlaubte Kachelpaare."))
        .action_describe("delete-rule", LocalizedLabel::native("Removes one adjacency rule by id, withdrawing the permission or ban it stated.", "Entfernt eine Nachbarschaftsregel anhand ihrer Id und nimmt die Erlaubnis oder das Verbot zurück, das sie ausdrückte."))
        .action_describe("change-active-tile", LocalizedLabel::native("Arms the tile that a pin gesture in this window assigns; only the editor's view state changes, not the document.", "Wählt die Kachel, die eine Anheften-Geste in diesem Fenster zuweist; nur der Ansichtszustand des Editors ändert sich, nicht das Dokument."))
        .action_audience("change-camera", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
