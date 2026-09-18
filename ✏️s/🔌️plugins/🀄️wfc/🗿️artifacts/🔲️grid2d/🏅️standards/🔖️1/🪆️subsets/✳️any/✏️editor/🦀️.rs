//! ✏️ 2D-grid editor — the authored `ArtifactEditor` surface for `s.wfc.grid2d@1/*`. Two panes:
//! `🔲️grid` (the interactive Canvas2d over the authored grid) and `👁️preview` (the Canvas2d render of
//! the SOLVED assignment). Every document command maps 1:1 onto a real `Grid2dMutation` builder
//! from the schema tree — no synthetic "set field" indirection, because the domain's own mutations
//! are already exactly that granular. Camera/grid/active-tile/solve-cache commands write the pane's
//! own `Grid2dWindowConfig` instead, never the document.

use crate::editor::grid2d::modes::edit;
use crate::editor::grid2d::modes::edit::windows::{grid, preview};
use crate::editor::grid2d::window::{self, Grid2dWindowConfig};
use crate::mutations::{
    change_cell_size, change_periodicity, change_seed, change_tile_media, change_tile_weight, create_rule, create_tile, delete_rule, delete_tile, mask_cell, pin_cell, resize_grid, unmask_cell, unpin_cell,
};
use crate::schema::inferences::solve_with_job;
use crate::schema::snapshot::{WfcAdjacencyRule2d, WfcTile2d, WfcTileMedia2d};
use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect,
    DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient,
    NoTransientMutation, ViewModel,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

//#region 🔖️ActiveUtility
/// 🧰️ The utility armed for the window being rendered or dispatched: the React host arms utilities
/// per window INSTANCE (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window
/// into the flat `active_utility_id`, so a pane that is not the active window would otherwise read
/// `None` and run every click as a plain pick.
pub fn grid2d_active_utility(view: &ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or(grid::UTILITY_SELECT)
}
//#endregion 🔖️ActiveUtility

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `Grid2dMutation` kind plus the
/// pane-local view verbs (camera, grid chrome, active tile, solve).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Grid2dEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "resize-grid")]
    ResizeGrid { width: u32, height: u32 },
    #[dsl(key = "change-cell-size")]
    ChangeCellSize { cell_width: f64, cell_height: f64 },
    #[dsl(key = "change-periodicity")]
    ChangePeriodicity { periodic_x: bool, periodic_y: bool },
    #[dsl(key = "create-tile")]
    CreateTile { id: String, label: Option<String>, weight: f64 },
    #[dsl(key = "delete-tile")]
    DeleteTile { id: String },
    #[dsl(key = "change-tile-weight")]
    ChangeTileWeight { id: String, weight: f64 },
    #[dsl(key = "change-tile-media")]
    ChangeTileMedia { id: String, media_json: String },
    #[dsl(key = "create-rule")]
    CreateRule { id: String, tile_a_id: String, tile_b_id: String, direction: String, allowed: bool },
    #[dsl(key = "delete-rule")]
    DeleteRule { id: String },
    #[dsl(key = "pin-cell")]
    PinCell { x: u32, y: u32, tile_id: String },
    #[dsl(key = "unpin-cell")]
    UnpinCell { x: u32, y: u32 },
    #[dsl(key = "mask-cell")]
    MaskCell { x: u32, y: u32 },
    #[dsl(key = "unmask-cell")]
    UnmaskCell { x: u32, y: u32 },
    /// 🖱️ One board click, resolved against the pane's ARMED utility: `pin` writes the active tile,
    /// `mask` toggles the hole, `select` changes nothing.
    #[dsl(key = "pick-cell")]
    PickCell { x: u32, y: u32 },
    #[dsl(key = "set-active-tile")]
    SetActiveTile { tile_id: String },
    #[dsl(key = "set-camera")]
    SetCamera { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "set-grid-visible")]
    SetGridVisible { visible: bool },
    #[dsl(key = "set-grid-snap-enabled")]
    SetGridSnapEnabled { enabled: bool },
    #[dsl(key = "set-grid-factor")]
    SetGridFactor { factor: f64 },
    /// 🏁 Runs the `s.wfc.grid2d.solve` inference and caches its commit in THIS pane's window
    /// config. The document is never touched: the solve is derived, never persisted.
    #[dsl(key = "solve")]
    Solve,
    /// 🗃️ Loads one bundled example by its registered id — the navbar switcher's verb. A whole
    /// document replacement is not expressible as a `Grid2dMutation`, so this emits
    /// `Effect::LoadDocument` and journals NO history patch: re-picking the boot example leaves
    /// `canUndo` false instead of minting a phantom edit.
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    /// 🖱️ One pointer press on a canvas surface, in that surface's CSS pixels. Only the GRID pane's
    /// surface resolves to a cell; a press on the read-only preview is inert.
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown { surface_id: String, x: f64, y: f64, width: f64, height: f64 },
    /// 🖱️ Every other canvas gesture the host dispatches unprompted. Declared so it is never
    /// `refused: undeclared-action`, inert because this editor tracks no hover state.
    #[dsl(key = "canvas-gesture")]
    CanvasGesture { action: String },
    /// 🎥️ `Canvas2dHost`'s own debounced camera echo. A SEPARATE variant from `SetCamera` although
    /// both write the same config: `dispatch_typed_command_inner` refuses a command whose
    /// `command_id` is not the action id it was admitted under, so two action ids may never collapse
    /// onto one command.
    #[dsl(key = "sync-camera")]
    SyncCamera { x: f64, y: f64, zoom: f64 },
}

impl protocol::OpBinary for Grid2dEditorCommand {
    /// 🎯️ The typed tool ids this command channel generates. WITHOUT this the trait default
    /// (`["typed-command"]`) makes `validate_tool_job_rows` compute an EMPTY expected set, every
    /// bounded-first-step proof below is dropped, and `qualified_tool_proof` then refuses every UI
    /// dispatch with `interactive-job.missing-factory` — a complete, correctly classified, completely
    /// dead action roster.
    const TOOL_JOB_IDS: &'static [&'static str] = GRID2D_TOOL_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🏷️ Every manifest action id this editor dispatches, in `Grid2dEditorCommand` row order. The
/// `#[dsl(key)]` spelling IS the manifest action id for every document verb; the camelCase rows are
/// framework/host-owned ids (the navbar picker and `Canvas2dHost` dispatch those exact strings).
pub const GRID2D_TOOL_IDS: &[&str] = &[
    "change-seed",
    "resize-grid",
    "change-cell-size",
    "change-periodicity",
    "create-tile",
    "delete-tile",
    "change-tile-weight",
    "change-tile-media",
    "create-rule",
    "delete-rule",
    "pin-cell",
    "unpin-cell",
    "mask-cell",
    "unmask-cell",
    "pick-cell",
    "set-active-tile",
    "set-camera",
    "set-grid-visible",
    "set-grid-snap-enabled",
    "set-grid-factor",
    "solve",
    "setActiveExample",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "canvasDoubleClick",
    "setCamera",
];

/// 🏷️ The manifest action id one command was declared under — command-log labelling and the
/// registry's kind-discipline lookup both read it. A raw canvas gesture carries its own host id, so
/// three inert verbs share one command variant without losing their identity.
pub fn grid2d_command_id(command: &Grid2dEditorCommand) -> &'static str {
    match command {
        Grid2dEditorCommand::ChangeSeed { .. } => "change-seed",
        Grid2dEditorCommand::ResizeGrid { .. } => "resize-grid",
        Grid2dEditorCommand::ChangeCellSize { .. } => "change-cell-size",
        Grid2dEditorCommand::ChangePeriodicity { .. } => "change-periodicity",
        Grid2dEditorCommand::CreateTile { .. } => "create-tile",
        Grid2dEditorCommand::DeleteTile { .. } => "delete-tile",
        Grid2dEditorCommand::ChangeTileWeight { .. } => "change-tile-weight",
        Grid2dEditorCommand::ChangeTileMedia { .. } => "change-tile-media",
        Grid2dEditorCommand::CreateRule { .. } => "create-rule",
        Grid2dEditorCommand::DeleteRule { .. } => "delete-rule",
        Grid2dEditorCommand::PinCell { .. } => "pin-cell",
        Grid2dEditorCommand::UnpinCell { .. } => "unpin-cell",
        Grid2dEditorCommand::MaskCell { .. } => "mask-cell",
        Grid2dEditorCommand::UnmaskCell { .. } => "unmask-cell",
        Grid2dEditorCommand::PickCell { .. } => "pick-cell",
        Grid2dEditorCommand::SetActiveTile { .. } => "set-active-tile",
        Grid2dEditorCommand::SetCamera { .. } => "set-camera",
        Grid2dEditorCommand::SetGridVisible { .. } => "set-grid-visible",
        Grid2dEditorCommand::SetGridSnapEnabled { .. } => "set-grid-snap-enabled",
        Grid2dEditorCommand::SetGridFactor { .. } => "set-grid-factor",
        Grid2dEditorCommand::Solve => "solve",
        Grid2dEditorCommand::SetActiveExample { .. } => "setActiveExample",
        Grid2dEditorCommand::CanvasPointerDown { .. } => "canvasPointerDown",
        Grid2dEditorCommand::CanvasGesture { action } => match action.as_str() {
            "canvasPointerUp" => "canvasPointerUp",
            "canvasDoubleClick" => "canvasDoubleClick",
            _ => "canvasPointerMove",
        },
        Grid2dEditorCommand::SyncCamera { .. } => "setCamera",
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedCommands
/// 🧬️ The payload schema id every 2D-grid retained command job is admitted under.
const GRID2D_RETAINED_COMMAND_SCHEMA: &str = "s.wfc.grid2d/v1.tool-command.v1";
/// 🎒️ Wire ceiling for one dispatch. The widest payload is `change-tile-media`, whose JSON carries a
/// whole tile's vector paths or its base64 pixel block.
const GRID2D_RETAINED_RAW_BYTES: usize = 65_536;
/// 📬️ Admission envelope for one published document edit — `change-tile-media` again.
const GRID2D_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 262_144;

const fn artifact_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact] }
}

const fn window_config_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::WindowConfig] }
}

const fn host_only_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::HostOnly] }
}

/// 🚦️ Per-tool publication lanes, read straight off each command's own emit: the fourteen mutation
/// verbs and the two click routes publish document edits, the camera/grid/active-tile/solve verbs
/// publish this pane's window config, and the example switch plus the inert gestures publish nothing.
const GRID2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("resize-grid"),
    artifact_route("change-cell-size"),
    artifact_route("change-periodicity"),
    artifact_route("create-tile"),
    artifact_route("delete-tile"),
    artifact_route("change-tile-weight"),
    artifact_route("change-tile-media"),
    artifact_route("create-rule"),
    artifact_route("delete-rule"),
    artifact_route("pin-cell"),
    artifact_route("unpin-cell"),
    artifact_route("mask-cell"),
    artifact_route("unmask-cell"),
    artifact_route("pick-cell"),
    artifact_route("canvasPointerDown"),
    window_config_route("set-active-tile"),
    window_config_route("set-camera"),
    window_config_route("setCamera"),
    window_config_route("set-grid-visible"),
    window_config_route("set-grid-snap-enabled"),
    window_config_route("set-grid-factor"),
    window_config_route("solve"),
    host_only_route("setActiveExample"),
    host_only_route("canvasPointerMove"),
    host_only_route("canvasPointerUp"),
    host_only_route("canvasDoubleClick"),
];

fn grid2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GRID2D_RETAINED_RAW_BYTES, 64, 1, GRID2D_ARTIFACT_MUTATION_MAXIMUM_BYTES, 7_500)
}

/// 🧵️ The one retained work every 2D-grid verb rides: one bounded first step that runs the editor's
/// own dispatch and publishes its emit.
struct Grid2dCommandWork {
    tool_id: &'static str,
    consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Grid2dEditor>> for Grid2dCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Grid2dEditorCommand,
        _snapshot: &Grid2dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Grid2dEditor>>>,
    ) -> Option<usize> {
        GRID2D_TOOL_IDS.contains(&grid2d_command_id(command)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Grid2dEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<Grid2dEditor>>, Fault> {
        if self.consumed {
            return Err(Fault::from("wfc-grid2d-retained-work-repeated"));
        }
        self.consumed = true;
        if grid2d_command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc-grid2d-retained-route-mismatch"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let view_state = input.context.and_then(|context| context.view_state.as_ref());
        Ok(ArtifactCommandWorkStep::Complete(Grid2dEditor::dispatch(input.command, &doc, &cfg, view_state)?))
    }
}

/// 🏭️ The app-owned bounded tool-job factory. `qualified_tool_proof` refuses every typed command that
/// resolves to anything else, so this registration is what makes the whole roster reachable.
pub struct Grid2dCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Grid2dCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GRID2D_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Grid2dEditor>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Self::new(&controller))
    }
}

impl semio_framework::ToolJobFactory for Grid2dCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Grid2dEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Grid2dEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        GRID2D_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        grid2d_retained_contract()
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
        if input.declared_bytes() > GRID2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded wfc grid2d command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Grid2dCommandJobFactory {
    type Owner = EditorApp<Grid2dEditor>;
    const TOOL_IDS: &'static [&'static str] = GRID2D_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = GRID2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

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

fn arg_u32(args: Option<&dsl::DslValue>, key: &str, fallback: u32) -> u32 {
    let value = arg_f64(args, key, f64::from(fallback));
    if value.is_finite() && value >= 0.0 {
        value as u32
    } else {
        fallback
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

/// 🔑️ The first of several spellings that carries a non-empty string — the navbar sends
/// `exampleId`, a palette form may send `id`, an inspector row `value`.
fn arg_string_any(args: Option<&dsl::DslValue>, keys: &[&str]) -> String {
    keys.iter().map(|key| arg_string(args, key)).find(|value| !value.is_empty()).unwrap_or_default()
}
//#endregion 🔖️Args

//#region 🔖️Examples
/// 📚️ One bundled example by the id the navbar switcher sends — the same ids the subset publishes
/// through `SubsetDeclaration.examples`, so a picker row and this lookup can never drift.
pub fn example_document(example_id: &str) -> Option<Grid2dSnapshot> {
    match example_id {
        crate::examples::grid2d::pipes::ID => Some(crate::examples::grid2d::pipes::document()),
        crate::examples::grid2d::terrain::ID => Some(crate::examples::grid2d::terrain::document()),
        _ => None,
    }
}

/// 🌱️ The sanctioned whole-document replacement: a fresh, edit-free op log for `document`. A
/// document swap is deliberately NOT a `Grid2dMutation` — it carries no inverse and belongs in no
/// undo ladder — so it rides `Effect::LoadDocument` and leaves the history exactly as it was.
pub fn reset_document_effect(document: &Grid2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Grid2dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("wfc-grid2d", WFC_GRID2D_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Examples

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Grid2dEditor;

impl Grid2dEditor {
    /// 🀄️ The tile a pin gesture writes: the pane's active tile when it still exists, else the first
    /// authored tile — a cursor that has gone stale must fall back to a real entry, never render
    /// nothing (the "Empty canvas despite real data" fault class).
    fn active_tile(document: &Grid2dSnapshot, config: &Grid2dWindowConfig) -> Option<String> {
        if !config.active_tile_id.is_empty() && document.tiles.iter().any(|tile| tile.id == config.active_tile_id) {
            return Some(config.active_tile_id.clone());
        }
        document.tiles.first().map(|tile| tile.id.clone())
    }

    /// 🖱️ One resolved cell gesture run through the ARMED utility: `pin` writes the active tile,
    /// `mask` toggles the hole, `select` emits nothing. Shared by the explicit `pick-cell` verb and
    /// by a raw canvas press, so a click and a scripted pick can never diverge.
    fn armed_pick(document: &Grid2dSnapshot, config: &Grid2dWindowConfig, utility: &str, x: u32, y: u32) -> Result<Option<(Grid2dMutation, String)>, Fault> {
        Ok(match utility {
            grid::UTILITY_PIN => {
                let tile = Self::active_tile(document, config).ok_or_else(|| Fault::from("wfc-grid2d-no-tile-to-pin"))?;
                Some((pin_cell(x, y, tile), format!("Pin cell ({x}, {y})")))
            }
            grid::UTILITY_MASK => {
                if document.masked.iter().any(|cell| cell.x == x && cell.y == y) {
                    Some((unmask_cell(x, y), format!("Unmask cell ({x}, {y})")))
                } else {
                    Some((mask_cell(x, y), format!("Mask cell ({x}, {y})")))
                }
            }
            _ => None,
        })
    }

    fn config_emit(view_state: Option<&ViewModel>, config: Grid2dWindowConfig, description: &str) -> Result<Emit<Grid2dMutation>, Fault> {
        let view = view_state.ok_or_else(|| Fault::from("wfc-grid2d-window-required"))?;
        let mutation = window::addressed_config(view, config)?;
        Ok(Emit { window_config_mutations: vec![mutation], description: Some(description.to_string()), ..Default::default() })
    }

    /// ✏️ The editor's whole command behaviour, reachable from BOTH the `ArtifactEditor::handle`
    /// entry point and the retained tool job that carries a real UI dispatch.
    pub fn dispatch(command: &Grid2dEditorCommand, doc: &ArtifactView<'_, Grid2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: Option<&ViewModel>) -> Result<Emit<Grid2dMutation>, Fault> {
        let window_config = window::config_from_view(cfg);
        let (mutation, description) = match command {
            Grid2dEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            Grid2dEditorCommand::ResizeGrid { width, height } => (resize_grid(*width, *height), format!("Resize grid to {width}×{height}")),
            Grid2dEditorCommand::ChangeCellSize { cell_width, cell_height } => (change_cell_size(*cell_width, *cell_height), "Change cell size".to_string()),
            Grid2dEditorCommand::ChangePeriodicity { periodic_x, periodic_y } => (change_periodicity(*periodic_x, *periodic_y), "Change periodicity".to_string()),
            Grid2dEditorCommand::CreateTile { id, label, weight } => {
                (create_tile(WfcTile2d { id: id.clone(), label: label.clone(), weight: *weight, media: WfcTileMedia2d::default() }), format!("Create tile {id}"))
            }
            Grid2dEditorCommand::DeleteTile { id } => (delete_tile(id.clone()), format!("Delete tile {id}")),
            Grid2dEditorCommand::ChangeTileWeight { id, weight } => (change_tile_weight(id.clone(), *weight), format!("Change weight of {id}")),
            Grid2dEditorCommand::ChangeTileMedia { id, media_json } => {
                let media: WfcTileMedia2d = protocol::json::from_json_str(media_json).map_err(|error| Fault::from(format!("wfc-grid2d-invalid-media:{error}")))?;
                (change_tile_media(id.clone(), media), format!("Change media of {id}"))
            }
            Grid2dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, direction, allowed } => {
                let direction = crate::schema::snapshot::text::direction_from_token(direction).map_err(|error| Fault::from(format!("wfc-grid2d-invalid-direction:{error}")))?;
                (create_rule(WfcAdjacencyRule2d { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), direction, allowed: *allowed }), format!("Create rule {id}"))
            }
            Grid2dEditorCommand::DeleteRule { id } => (delete_rule(id.clone()), format!("Delete rule {id}")),
            Grid2dEditorCommand::PinCell { x, y, tile_id } => (pin_cell(*x, *y, tile_id.clone()), format!("Pin cell ({x}, {y})")),
            Grid2dEditorCommand::UnpinCell { x, y } => (unpin_cell(*x, *y), format!("Unpin cell ({x}, {y})")),
            Grid2dEditorCommand::MaskCell { x, y } => (mask_cell(*x, *y), format!("Mask cell ({x}, {y})")),
            Grid2dEditorCommand::UnmaskCell { x, y } => (unmask_cell(*x, *y), format!("Unmask cell ({x}, {y})")),
            Grid2dEditorCommand::PickCell { x, y } => {
                let utility = view_state.map_or(grid::UTILITY_SELECT, grid2d_active_utility);
                match Self::armed_pick(doc.snapshot, &window_config, utility, *x, *y)? {
                    Some(picked) => picked,
                    None => return Ok(Emit::default()),
                }
            }
            Grid2dEditorCommand::CanvasPointerDown { surface_id, x, y, width, height } => {
                if !grid::owns_surface(surface_id) {
                    return Ok(Emit::default());
                }
                let utility = view_state.map_or(grid::UTILITY_SELECT, grid2d_active_utility);
                let Some((column, row)) = grid::cell_at(doc.snapshot, &window_config, *x, *y, *width, *height) else {
                    return Ok(Emit::default());
                };
                match Self::armed_pick(doc.snapshot, &window_config, utility, column, row)? {
                    Some(picked) => picked,
                    None => return Ok(Emit::default()),
                }
            }
            Grid2dEditorCommand::CanvasGesture { .. } => return Ok(Emit::default()),
            Grid2dEditorCommand::SetActiveExample { example_id } => {
                let next = example_document(example_id).ok_or_else(|| Fault::from(format!("wfc-grid2d-unknown-example:{example_id}")))?;
                return Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() });
            }
            Grid2dEditorCommand::SetActiveTile { tile_id } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { active_tile_id: tile_id.clone(), ..window_config }, "Set active tile");
            }
            Grid2dEditorCommand::SetCamera { x, y, zoom } | Grid2dEditorCommand::SyncCamera { x, y, zoom } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { camera_x: *x, camera_y: *y, camera_zoom: if *zoom > 0.0 { *zoom } else { 1.0 }, ..window_config }, "Set camera");
            }
            Grid2dEditorCommand::SetGridVisible { visible } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_visible: *visible, ..window_config }, "Set grid visibility");
            }
            Grid2dEditorCommand::SetGridSnapEnabled { enabled } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_snap_enabled: *enabled, ..window_config }, "Set grid snapping");
            }
            Grid2dEditorCommand::SetGridFactor { factor } => {
                return Self::config_emit(view_state, Grid2dWindowConfig { grid_factor: if *factor > 0.0 { *factor } else { 1.0 }, ..window_config }, "Set grid factor");
            }
            Grid2dEditorCommand::Solve => {
                let commit = solve_with_job(doc.snapshot).map_err(|error| Fault::from(format!("wfc-grid2d-solve:{error}")))?;
                return Self::config_emit(view_state, Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&commit), ..window_config }, "Solve");
            }
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }
}

impl ArtifactEditor for Grid2dEditor {
    type Snapshot = Grid2dSnapshot;
    type Mutation = Grid2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Grid2dEditorCommand;

    const DIALECT: Dialect = WFC_GRID2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;

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
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
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
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn initial_snapshot() -> Grid2dSnapshot {
        crate::examples::grid2d::pipes::document()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        Grid2dCommandJobFactory::register(registry)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Grid2dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.grid2d@1/*#editor",
        artifact_schema: "s.wfc.grid2d",
        factory: "Grid2dCommandJobFactory",
        factory_type: Grid2dCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
        tools: [
            "change-seed",
            "resize-grid",
            "change-cell-size",
            "change-periodicity",
            "create-tile",
            "delete-tile",
            "change-tile-weight",
            "change-tile-media",
            "create-rule",
            "delete-rule",
            "pin-cell",
            "unpin-cell",
            "mask-cell",
            "unmask-cell",
            "pick-cell",
            "set-active-tile",
            "set-camera",
            "set-grid-visible",
            "set-grid-snap-enabled",
            "set-grid-factor",
            "solve",
            "setActiveExample",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "canvasDoubleClick",
            "setCamera"
        ]
    }

    /// 📬️ Publication authority for the document lane. Without it every `Artifact`-lane verb is
    /// refused closed with `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("wfc-grid2d-artifact-retained", GRID2D_ARTIFACT_MUTATION_MAXIMUM_BYTES))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GRID2D_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = grid2d_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::from("wfc-grid2d-command-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            grid2d_command_id,
            GRID2D_RETAINED_RAW_BYTES,
            1,
            Box::new(Grid2dCommandWork { tool_id, consumed: false }),
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        grid2d_command_id(command)
    }

    /// 🎥️ The canvas host syncs its own camera as one nested `camera` object; the palette's
    /// `set-camera` form states the three scalars flat. Both reach the same command.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        Ok(match action {
            "change-seed" => Grid2dEditorCommand::ChangeSeed { seed: arg_u64(args, "seed", 0) },
            "resize-grid" => Grid2dEditorCommand::ResizeGrid { width: arg_u32(args, "width", 1), height: arg_u32(args, "height", 1) },
            "change-cell-size" => Grid2dEditorCommand::ChangeCellSize { cell_width: arg_f64(args, "cellWidth", 1.0), cell_height: arg_f64(args, "cellHeight", 1.0) },
            "change-periodicity" => Grid2dEditorCommand::ChangePeriodicity { periodic_x: arg_bool(args, "periodicX", false), periodic_y: arg_bool(args, "periodicY", false) },
            "create-tile" => Grid2dEditorCommand::CreateTile { id: arg_string(args, "id"), label: None, weight: arg_f64(args, "weight", 1.0) },
            "delete-tile" => Grid2dEditorCommand::DeleteTile { id: arg_string(args, "id") },
            "change-tile-weight" => Grid2dEditorCommand::ChangeTileWeight { id: arg_string(args, "id"), weight: arg_f64(args, "weight", 1.0) },
            "change-tile-media" => Grid2dEditorCommand::ChangeTileMedia { id: arg_string(args, "id"), media_json: arg_string(args, "mediaJson") },
            "create-rule" => Grid2dEditorCommand::CreateRule {
                id: arg_string(args, "id"),
                tile_a_id: arg_string(args, "tileAId"),
                tile_b_id: arg_string(args, "tileBId"),
                direction: arg_string(args, "direction"),
                allowed: arg_bool(args, "allowed", true),
            },
            "delete-rule" => Grid2dEditorCommand::DeleteRule { id: arg_string(args, "id") },
            "pin-cell" => Grid2dEditorCommand::PinCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0), tile_id: arg_string(args, "tileId") },
            "unpin-cell" => Grid2dEditorCommand::UnpinCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "mask-cell" => Grid2dEditorCommand::MaskCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "unmask-cell" => Grid2dEditorCommand::UnmaskCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "pick-cell" => Grid2dEditorCommand::PickCell { x: arg_u32(args, "x", 0), y: arg_u32(args, "y", 0) },
            "set-active-tile" => Grid2dEditorCommand::SetActiveTile { tile_id: arg_string(args, "tileId") },
            "set-camera" => Grid2dEditorCommand::SetCamera { x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0), zoom: arg_f64(args, "zoom", 1.0) },
            "set-grid-visible" => Grid2dEditorCommand::SetGridVisible { visible: arg_bool(args, "visible", true) },
            "set-grid-snap-enabled" => Grid2dEditorCommand::SetGridSnapEnabled { enabled: arg_bool(args, "enabled", true) },
            "set-grid-factor" => Grid2dEditorCommand::SetGridFactor { factor: arg_f64(args, "factor", 1.0) },
            "solve" => Grid2dEditorCommand::Solve,
            "setActiveExample" => Grid2dEditorCommand::SetActiveExample { example_id: arg_string_any(args, &["exampleId", "id", "value"]) },
            "setCamera" => {
                let camera = arg(args, "camera");
                Grid2dEditorCommand::SyncCamera { x: arg_f64(camera, "x", 0.0), y: arg_f64(camera, "y", 0.0), zoom: arg_f64(camera, "zoom", 1.0) }
            }
            "canvasPointerDown" => Grid2dEditorCommand::CanvasPointerDown {
                surface_id: arg_string(args, "surfaceId"),
                x: arg_f64(args, "x", -1.0),
                y: arg_f64(args, "y", -1.0),
                width: arg_f64(args, "width", 0.0),
                height: arg_f64(args, "height", 0.0),
            },
            "canvasPointerMove" | "canvasPointerUp" | "canvasDoubleClick" => Grid2dEditorCommand::CanvasGesture { action: action.to_string() },
            other => return Err(Fault::from(format!("wfc-grid2d-unknown-action:{other}"))),
        })
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        Self::dispatch(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = window::config_from_view(cfg);
        match body_key {
            grid::BODY_KEY => grid::render(doc.snapshot, &config, grid2d_active_utility(view_state)).map(semio_framework_plugin::built_to_component_tree),
            preview::BODY_KEY => preview::render(doc.snapshot, &config).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🏗️ Admits the whole-document replacement every `Effect::LoadDocument` this editor emits
    /// (`reset_document_effect`: the example switcher) through the host's persisted-envelope
    /// replacement — the trait default REFUSES the envelope, which faults every document swap at
    /// the archive-load boundary instead of loading the picked example.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, WFC_GRID2D_DOCUMENT_SCHEMA, operation, generation))
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window::register_config(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window::register_transient(registry)
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_grid2d_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(WFC_GRID2D_DIALECT)
        .document(["semio", "wfc", "grid2d"])
        .icon_id("puzzle")
        .mode_def(edit::definition())
        .default_mode_id(edit::GRID2D_EDIT_MODE_ID)
        .window_kind_def(grid::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout());
    for utility in edit::utilities() {
        builder = builder.utility(utility);
    }
    builder.window_kind_utilities(grid::WINDOW_KIND_ID, vec![grid::UTILITY_SELECT.into(), grid::UTILITY_PIN.into(), grid::UTILITY_MASK.into()]).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
