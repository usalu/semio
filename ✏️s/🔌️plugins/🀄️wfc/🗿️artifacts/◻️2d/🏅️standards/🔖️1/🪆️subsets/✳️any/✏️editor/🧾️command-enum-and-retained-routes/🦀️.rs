//! 🧾️ Command enum and retained routes: the wfc2d editor command with its op binary and ids, the retained tool ids and publication contracts, the command work and the retained job factory. See [`super`].

use super::*;

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Wfc2dMutation` kind a UI can
/// trigger, plus the three non-document verbs (camera, armed tile, solve result).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
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
    /// 🕹️ The NodeGraph canvas' OWN gesture channel: a released node drag (the node-graph gesture record), a completed,
    /// cut or deleted wire and a deleted node all arrive as `nodeGraphEdit` rows, never as
    /// `move-slot`/`connect-slots` directly. One release is ONE drag-tool transaction of relative `drag-slots` leaves.
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

pub(super) const WFC_2D_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const WFC_2D_RETAINED_WORK_ITEMS: usize = 4_096;

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
pub(super) fn wfc2d_retained_extent(command: &Wfc2dEditorCommand, snapshot: &Wfc2dSnapshot) -> Option<usize> {
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
pub(super) struct Wfc2dCommandWork {
    pub(super) tool_id: &'static str,
    completed: bool,
}

impl Wfc2dCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
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

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Wfc2dEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Wfc2dEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.completed || wfc2d_command_id(input.command) != self.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.retained.route"), "the bounded WFC 2D work rejects an undeclared or completed route"));
        }
        let emit = dispatch(input.command, input.snapshot, input.config, &input.operation.authoring_seed)?;
        let transient = match input.command {
            Wfc2dEditorCommand::CommitFill { payload_json } => {
                let payload = fill::decode_fill_payload(payload_json.as_bytes()).ok_or_else(|| Fault::from("wfc2d-commit-fill-payload"))?;
                payload.set_solve_mutations()
            }
            _ => Vec::new(),
        };
        self.completed = true;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient, window_transient: Vec::new() } })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 🏭️ The app-owned retained command job factory — `factory_type:` in the proof block binds this exact
/// Rust type to `EditorApp<Wfc2dEditor>`, which is what turns a bare (and therefore dispatch-dead)
/// bounded proof into an exact-owner proof.
pub struct Wfc2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Wfc2dRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: WFC_2D_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Wfc2dRetainedCommandJobFactory {
    type Owner = EditorApp<Wfc2dEditor>;
    const TOOL_IDS: &'static [&'static str] = WFC_2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = WFC_2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️Retained
