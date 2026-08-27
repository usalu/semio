//! 🎞️ Animate editor — the `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET), the aggregated command enum and the manifest
//! stitch. B1: the pure-trait pivot — `AnimatePresentPlayApp` is a unit struct; every former
//! `AnimatePresentPlayRuntime` field (selection, engagement draft) now lives in
//! `crate::editor::animate::config::PresentConfig`, written via `PresentConfigMutation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `PresentCommand` channel via `ArtifactEditor::handle`. MUST NOT be imported from the sibling
//! `👁️viewer` module (`policyViewerPurityBreaches`).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🖊️main/🪟️windows/🖼️tile-editor`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, pure document helpers in
//! `crate::artifacts::present::schema`, and stateful behaviour (the Manim-class animation core + the
//! headless video renderer, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) in this app's
//! own `⚙️engine`. This file is a routing table: `handle` → `PresentCommand::dispatch`, `render` →
//! body-key → node, `🔖️Io`/`🔌️Registration` regions below, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::schema::build_tile_morph_prompt;
use crate::artifacts::present::{default_present_snapshot, FigureTileDraft, PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use crate::editor::animate::commands::{
    add_tile, canvas_pointer_down, clear_tiles, copy_prompt, delete_selection, delete_tile, engagement_input, engagement_submit, export_video_from_deck, no_operation, patch_tile_crops, rename_tiles, reset_grid, seed_grid, set_active_example,
    set_frame, set_locale, set_source,
};
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::modes::main;
use crate::editor::animate::modes::main::windows::tile_editor;
use crate::editor::animate::panels::{artifact, catalogue, inspection};
use crate::editor::animate::terminology::animate_present_labels;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
// 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
// `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
// `.workflow(...)` on this builder (see `🔖️Manifest` below for what got dropped, not silently).
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionKind, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ComponentTree, ConfigView, Dialect, DraftView, Editor, EditorApp, Effect, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaError, MediaPayload, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use serde_json::Value;
use std::collections::HashSet;
use store::EngineHandles;

//#region 🔖️Constants
pub const PRESENT_PLAY_APP_ID: &str = "animate-present-play";
pub use artifact::PRESENT_PLAY_BODY_DOCUMENT;
pub use catalogue::PRESENT_PLAY_BODY_CATALOGUE;
pub use inspection::PRESENT_PLAY_BODY_DETAILS;
pub use tile_editor::PRESENT_PLAY_BODY_MAIN;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn animate_present_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PRESENT_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "tiles" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Flat` over the tile grid (a document panel tree binds it
/// directly; the canvas hit-tests a click and asks the framework to apply it, mirroring `🖍️draw`'s
/// "strokes" domain).
pub const PRESENT_INTERACTION_DOMAIN: &str = "tiles";
pub const PRESENT_INTERACTION_GRANULARITY: &str = "tile";

/// 🕹️ Per-dispatch scratch: the "tiles" domain's current selection, resolved once by
/// `ArtifactApp::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself.
pub struct PresentDispatchCtx {
    pub selected_ids: Vec<String>,
}

/// 🕹️ JSON-encodes `ids` as the `Vec<InteractionTarget>` string the framework's `interactionSelect`
/// action requires in its `targets` arg — every hit id shares the domain's one granularity.
fn interaction_targets_json(ids: &[String]) -> String {
    serde_json::to_string(&ids.iter().map(|id| serde_json::json!({ "granularity": PRESENT_INTERACTION_GRANULARITY, "id": id })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}

/// 🕹️ Requests the shell to redispatch the framework-owned `interactionSelect` verb through its
/// normal action funnel — the only way `canvas-pointer-down`'s hit test can drive selection now that
/// it is framework-owned state, never a `PresentConfigMutation` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ "domainId": PRESENT_INTERACTION_DOMAIN, "targets": interaction_targets_json(ids), "merge": merge, "method": "pick" }))),
    }
}
//#endregion 🔖️Interaction

//#region 🔖️Io
/// 🔌️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors `create_animate_present_app`'s `.artifact_kind(...)` literal (schema/
/// media type copied verbatim) plus the extra `frames:in` input port (Wave-2 port recipe).
pub fn present_io() -> AppIo {
    AppIo {
        document_schema: PRESENT_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Presentation, form: semio_framework_plugin::MediaForm::Deck },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "frames:in".into(),
            label: "Frames".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Raster },
            kind_id: Some("2d.image".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: PRESENT_DOCUMENT_SCHEMA.into(), name: "Animate Present Deck".into(), dimension: "2d".into(), component_kind: "panel".into() },
    }
}

/// 🎞️ `frames:in` placement (Wave-2 port recipe) — `PresentSnapshot` models one shared background
/// `source` image with named crop-`tiles` over it; there is no per-tile independent raster payload in
/// this schema, so an incoming `2d.image` frame becomes a new tile positioned in a deterministic
/// contact-sheet grid (4 columns) rather than replacing `source` — exactly the surface `seedGrid`/
/// `addTile` (see the app's `🎮️commands/🀄️add-tile`/`🎮️commands/🌐️seed-grid`) already let a user crop/arrange
/// candidate frames on. Pure: both functions depend only on the current tile COUNT, so repeated imports
/// land in distinct, stable cells without needing a live host/counter.
const FRAME_IMPORT_GRID_COLUMNS: usize = 4;

pub fn next_frame_tile_id(existing_tile_count: usize) -> String {
    format!("frame-{}", existing_tile_count + 1)
}

pub fn next_frame_tile_crop(existing_tile_count: usize) -> crate::artifacts::present::FigureTileFrame {
    let cell = 1.0 / FRAME_IMPORT_GRID_COLUMNS as f64;
    let column = existing_tile_count % FRAME_IMPORT_GRID_COLUMNS;
    let row = existing_tile_count / FRAME_IMPORT_GRID_COLUMNS;
    crate::artifacts::present::schema::clamp_tile_crop(&crate::artifacts::present::FigureTileFrame { x: column as f64 * cell, y: (row as f64 * cell).min(1.0 - cell), width: cell, height: cell })
}
//#endregion 🔖️Io

//#region 🔖️Helpers
/// 🔢️ Mints a fresh, process-unique tile id — shared by `🎮️commands/🀄️add-tile::add_tile` and
/// `🎮️commands/⌨️engagement::engagement_submit`'s `"add"` keyword.
pub(crate) fn new_tile_id(prefix: &str) -> String {
    let serial = {
        let hex = framework_hash::hash_bytes(concat!(file!(), line!()).as_bytes());
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{serial}")
}

/// 🧹️ Retains only the ids that reference an existing tile in `deck` — shared by every command that
/// accepts a selection/target id list.
pub(crate) fn valid_tile_ids(deck: &PresentSnapshot, ids: Vec<String>) -> Vec<String> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(deck);
    let valid: HashSet<&str> = tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}

/// 🎞️ `frames:in` display name (Wave-2 port recipe) — a `Structured` payload's `"name"`/`"src"` field
/// (falling back to a generic label), a `Binary` payload's leading blob-hash characters.
fn frame_media_name(port: &str, media: &Media) -> Result<String, MediaError> {
    match &media.payload {
        MediaPayload::Structured { json, .. } => {
            let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            Ok(value.get("name").and_then(|v| v.as_str()).or_else(|| value.get("src").and_then(|v| v.as_str())).map_or_else(|| "Imported frame".into(), str::to_string))
        }
        MediaPayload::Binary { blob_hash, .. } => Ok(format!("frame-{}", &blob_hash[..blob_hash.len().min(8)])),
    }
}

/// 📋️ Host effect delivering the generated tile-morph prompt to the user as a downloadable markdown
/// file — the genuine shell side-effect that replaces the retired ephemeral clipboard scratch (the
/// landed `Effect` contract carries no clipboard variant, so the prompt is exported as media).
/// Shared by `🎮️commands/🐚️copy-prompt::copy_prompt` and `🎮️commands/⌨️engagement::engagement_submit`'s
/// `"copy"`/`"copy prompt"` keywords.
pub(crate) fn tile_morph_prompt_effect(deck: &PresentSnapshot) -> Effect {
    let (source, tiles) = crate::artifacts::present::present_working_scene(deck);
    Effect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&source, &tiles), encoding: None }
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "reset the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🖼️set-source::set_active_example` uses instead of the banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_present_document_effect(document: &PresentSnapshot) -> Effect {
    let pack = <PresentSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<PresentSnapshot, PresentMutation>(PRESENT_DOCUMENT_SCHEMA, "present", document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("present document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Helpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `AnimatePresentPlayApp::Command` — the SOLE dispatch surface for animate present's own
    /// behavior, assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest
    /// action id (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire
    /// keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// `"resetGrid" as "reset-grid"` is the row that proves it. The app owner is carried by the
    /// qualified command address rather than repeated in this local id. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PresentCommand for PresentSnapshot, PresentMutation, PresentConfig, PresentConfigMutation, ctx = PresentDispatchCtx {
        "seedGrid" as "seed-grid" => seed_grid::SeedGrid,
        "addTile" as "add-tile" => add_tile::AddTile,
        "deleteTile" as "delete-tile" => delete_tile::DeleteTile,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "renameTiles" as "rename-tiles" => rename_tiles::RenameTiles,
        "patchTileCrops" as "patch-tile-crops" => patch_tile_crops::PatchTileCrops,
        "setSource" as "set-source" => set_source::SetSource,
        "setFrame" as "set-frame" => set_frame::SetFrame,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "clearTiles" as "clear-tiles" => clear_tiles::ClearTiles,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "resetGrid" as "reset-grid" => reset_grid::ResetGrid,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "setLocale" as "set-locale" => set_locale::SetLocale,
        "noMutation" as "no-op" => no_operation::NoOperation,
        "copyPrompt" as "copy-prompt" => copy_prompt::CopyPrompt,
        "exportVideoFromDeck" as "export-video-from-deck" => export_video_from_deck::ExportVideoFromDeck,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const ANIMATE_PRESENT_RETAINED_TOOL_IDS: &[&str] = &["setActiveExample", "engagementInput", "setLocale", "noMutation"];
const ANIMATE_PRESENT_RETAINED_PAYLOAD_SCHEMA: &str = "animate.present.tool-command.v1";
const ANIMATE_PRESENT_RETAINED_RAW_BYTES: usize = 8_192;
const ANIMATE_PRESENT_RETAINED_WORK_ITEMS: usize = 1;
const ANIMATE_PRESENT_CONFIG_VALUE_BYTES: usize = 512;
const ANIMATE_PRESENT_CONFIG_BASE_BYTES: usize = 512;
const ANIMATE_PRESENT_CONFIG_STEP_BYTES: usize = 4_096;
const ANIMATE_PRESENT_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn animate_present_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(ANIMATE_PRESENT_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500, 1, 1)
}

fn animate_present_retained_extent(command: &PresentCommand, _snapshot: &PresentSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        PresentCommand::SetActiveExample(payload) if payload.example_id.len() <= ANIMATE_PRESENT_RETAINED_RAW_BYTES => Some(1),
        PresentCommand::EngagementInput(payload) if payload.value.len() <= ANIMATE_PRESENT_CONFIG_VALUE_BYTES => Some(1),
        PresentCommand::SetLocale(payload) if payload.value.len() <= ANIMATE_PRESENT_CONFIG_VALUE_BYTES => Some(1),
        PresentCommand::NoOperation(_) => Some(1),
        _ => None,
    }
}

fn animate_present_retained_reduce(
    command: &PresentCommand,
    snapshot: &PresentSnapshot,
    config: &PresentConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<PresentMutation, PresentConfigMutation, NoDraftMutation>, Fault> {
    let document = ArtifactView::with_operation(snapshot, history, operation.clone());
    let config = ConfigView { snapshot: config };
    let mut context = PresentDispatchCtx { selected_ids: Vec::new() };
    match command {
        PresentCommand::SetActiveExample(payload) if payload.example_id.len() <= ANIMATE_PRESENT_RETAINED_RAW_BYTES => set_active_example::handle(payload, &document, &config, &mut context),
        PresentCommand::EngagementInput(payload) if payload.value.len() <= ANIMATE_PRESENT_CONFIG_VALUE_BYTES => engagement_input::handle(payload, &document, &config, &mut context),
        PresentCommand::SetLocale(payload) if payload.value.len() <= ANIMATE_PRESENT_CONFIG_VALUE_BYTES => set_locale::handle(payload, &document, &config, &mut context),
        PresentCommand::NoOperation(payload) => no_operation::handle(payload, &document, &config, &mut context),
        _ => Err(Fault::from("animate-present-retained-route-mismatch")),
    }
}

struct AnimatePresentRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl AnimatePresentRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ANIMATE_PRESENT_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for AnimatePresentRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<AnimatePresentPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<AnimatePresentPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { ANIMATE_PRESENT_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { animate_present_retained_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > ANIMATE_PRESENT_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Animate Present retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for AnimatePresentRetainedCommandJobFactory {
    type Owner = EditorApp<AnimatePresentPlayApp>;
    const TOOL_IDS: &'static [&'static str] = ANIMATE_PRESENT_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PRESENT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = ANIMATE_PRESENT_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
struct AnimatePresentConfigPreparationFactory;

struct AnimatePresentConfigPreparation {
    base: Option<store::SnapshotRead<PresentConfig>>,
    mutation: Option<PresentConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(PresentConfig, PresentConfigMutation, PresentConfigMutation)>,
    sealed_candidate: Option<(PresentConfig, protocol::Edit<PresentConfigMutation>)>,
    serialized_bytes: Option<usize>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<PresentConfig, PresentConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn animate_present_config_edit(forward: PresentConfigMutation, inverse: PresentConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<PresentConfigMutation> {
    let id = format!("animate-present-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse: vec![inverse],
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

struct AnimatePresentConfigByteCounter { bytes: usize }

impl std::io::Write for AnimatePresentConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > ANIMATE_PRESENT_CONFIG_STEP_BYTES { return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)); }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn animate_present_config_edit_bytes(edit: &protocol::Edit<PresentConfigMutation>) -> Result<usize, String> {
    let mut counter = AnimatePresentConfigByteCounter { bytes: 0 };
    serde_json::to_writer(&mut counter, edit).map_err(|_| "Animate Present config edit exceeds its serialized byte envelope".to_string())?;
    Ok(counter.bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<PresentConfig, PresentConfigMutation> for AnimatePresentConfigPreparationFactory {
    fn preflight(&self, mutation: &PresentConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            PresentConfigMutation::SetEngagementInput { value } | PresentConfigMutation::SetLocale { value } => value.len(),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > ANIMATE_PRESENT_CONFIG_VALUE_BYTES || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Animate Present config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: ANIMATE_PRESENT_CONFIG_STEP_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<PresentConfig, PresentConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<PresentConfig, PresentConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<PresentConfig, PresentConfigMutation>> {
        let mutation_bytes = match &request.mutation {
            PresentConfigMutation::SetEngagementInput { value } | PresentConfigMutation::SetLocale { value } => value.len(),
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > ANIMATE_PRESENT_CONFIG_VALUE_BYTES || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(AnimatePresentConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<PresentConfig, PresentConfigMutation> for AnimatePresentConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < ANIMATE_PRESENT_CONFIG_STEP_BYTES || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() && self.sealed_candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Animate Present config preparation lost its exact base root".to_string())?.get();
            let base_bytes = base.engagement_input.len().saturating_add(base.locale.len());
            if base_bytes > ANIMATE_PRESENT_CONFIG_BASE_BYTES { return Err("Animate Present config base exceeds retained byte capacity".into()); }
            let mutation = self.mutation.take().ok_or_else(|| "Animate Present config preparation lost its mutation owner".to_string())?;
            let mut post = base.clone();
            let inverse = match &mutation {
                PresentConfigMutation::SetEngagementInput { value } => {
                    post.engagement_input = value.clone();
                    PresentConfigMutation::SetEngagementInput { value: base.engagement_input.clone() }
                }
                PresentConfigMutation::SetLocale { value } => {
                    post.locale = value.clone();
                    PresentConfigMutation::SetLocale { value: base.locale.clone() }
                }
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Animate Present config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "Animate Present config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, animate_present_config_edit(forward, inverse, self.description.take(), authority)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.sealed_candidate.as_ref().ok_or_else(|| "Animate Present config preparation lost its semantic edit".to_string())?;
            let bytes = animate_present_config_edit_bytes(edit)?;
            if bytes.saturating_add(post.engagement_input.len().saturating_add(post.locale.len())).saturating_add(512) > ANIMATE_PRESENT_CONFIG_STEP_BYTES {
                return Err("Animate Present config publication exceeds the 4096-byte complete envelope".into());
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, edit) = self.sealed_candidate.take().ok_or_else(|| "Animate Present config preparation lost its validated edit".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Animate Present config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<PresentConfig, PresentConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<PresentConfig, PresentConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if (self.prepared.is_some() || self.sealed_candidate.is_some() || self.candidate.is_some() || self.mutation.is_some() || self.description.is_some()) && grant.maximum_bytes < ANIMATE_PRESENT_CONFIG_STEP_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: ANIMATE_PRESENT_CONFIG_STEP_BYTES }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Animate Present config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🔖️AnimatePresentPlayApp
/// 🧪️ B1: unit struct — every former `AnimatePresentPlayRuntime` field now lives in
/// `crate::editor::animate::config::PresentConfig` (see `ArtifactApp::Config`), written through
/// `PresentConfigMutation`s.
#[derive(Default)]
pub struct AnimatePresentPlayApp;

impl ArtifactEditor for AnimatePresentPlayApp {
    type Snapshot = PresentSnapshot;
    type Mutation = PresentMutation;
    type Config = PresentConfig;
    type ConfigMutation = PresentConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::animate::presence::PresentPresence;
    type PresenceMutation = crate::editor::animate::presence::PresentPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = PresentCommand;

    const DIALECT: Dialect = crate::artifacts::present::ANIMATE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PRESENT_DOCUMENT_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(AnimatePresentConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<AnimatePresentPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.animate.present@1/*#editor",
        document_schema: "animate.present",
        factory: "AnimatePresentRetainedCommandJobFactory",
        factory_type: AnimatePresentRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample", "engagementInput", "setLocale", "noMutation"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(AnimatePresentRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !ANIMATE_PRESENT_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("animate-present-command-tool-mismatch"));
        }
        if animate_present_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("animate-present-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, animate_present_retained_reduce, animate_present_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            PresentCommand::command_id,
            ANIMATE_PRESENT_RETAINED_RAW_BYTES,
            ANIMATE_PRESENT_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::present::spr::present_envelope_decode_owner_bundle())
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::animate::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> PresentSnapshot {
        default_present_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(present_io())
    }

    /// 🌱️ `whole_document_operation` stays the trait default (`None`): per `📓️taxonomy.md`, whole-
    /// document replace has no in-history mutation at all (there is no import mutation by locked
    /// decision — see `🎮️commands/🖼️set-source::set_active_example`'s `Effect::LoadDocument` instead).
    /// 🎞️ `frames:in` (Wave-2 port recipe): inserts an incoming raster frame as a new tile in a
    /// deterministic contact-sheet grid (see `next_frame_tile_crop`'s doc comment below for why this
    /// schema's single shared `source` means tiles, not `source`, are the natural insertion point).
    /// Never mutates anything directly: the caller applies the returned `Tiles(Add)` through the
    /// ordinary, undoable document store.
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PresentSnapshot>) -> Result<Emit<PresentMutation, PresentConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "frames:in" {
            return Err(MediaError::NotImplemented);
        }
        let deck = doc.snapshot;
        let count = crate::artifacts::present::present_working_scene(deck).1.len();
        let id = next_frame_tile_id(count);
        let crop = next_frame_tile_crop(count);
        let name = frame_media_name(port, media)?;
        let tile = FigureTileDraft { id, name, crop };
        Ok(Emit::mutations(vec![PresentMutation::CreateTile(CreateTile { index: count, tile })]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &PresentCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &PresentCommand,
        doc: &ArtifactView<'_, PresentSnapshot>,
        cfg: &ConfigView<'_, PresentConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PresentMutation, PresentConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = PresentDispatchCtx { selected_ids: interaction.selection(PRESENT_INTERACTION_DOMAIN).ids.clone() };
        match command {
            PresentCommand::ExportVideoFromDeck(payload) => export_video_from_deck::handle_async(payload).await,
            _ => command.dispatch(doc, cfg, &mut ctx),
        }
    }

    /// 🕹️ `render(body_key, doc, cfg)` is never given an `InteractionView` (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — only `handle`/`copy_fragment`/
    /// `cut_operations` are), so the per-tile crop/name editors this panel used to build from
    /// `config.selected_ids` are gone from `inspection::render`; the client renders the tile-selected
    /// canvas highlight itself from the framework's own interaction state now (matches `🖍️draw`'s
    /// canvas render, same reason).
    async fn render(body_key: &str, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> ComponentTree {
        let deck = doc.snapshot;
        let config = cfg.snapshot;
        let labels = animate_present_labels(config);
        semio_framework_plugin::built_to_component_tree(match body_key {
            PRESENT_PLAY_BODY_MAIN => tile_editor::render(deck),
            PRESENT_PLAY_BODY_DOCUMENT => artifact::render(deck, labels),
            PRESENT_PLAY_BODY_CATALOGUE => catalogue::render(deck, labels),
            PRESENT_PLAY_BODY_DETAILS => inspection::render(deck, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))),
        })
    }
}
//#endregion 🔖️AnimatePresentPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_animate_present_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::present::ANIMATE_DIALECT)
            .document(["semio", "animate"])
            .artifact_kind(crate::artifacts::present::artifact_kind())
            .icon_id("animate")
            .mode_def(main::definition())
            .default_mode_id(main::PRESENT_PLAY_MODE_MAIN)
            .window_kind_def(tile_editor::definition())
            .default_layout(main::layout())
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            .mutation("seedGrid", LocalizedLabel::native("Seed Grid", "Raster erzeugen"))
            .mutation("addTile", LocalizedLabel::native("Add Tile", "Kachel hinzufügen"))
            .mutation("deleteTile", LocalizedLabel::native("Delete Tile", "Kachel löschen"))
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .mutation("renameTiles", LocalizedLabel::native("Rename Tiles", "Kacheln umbenennen"))
            .mutation("patchTileCrops", LocalizedLabel::native("Patch Tile Crops", "Kachelzuschnitte aktualisieren"))
            .mutation("setSource", LocalizedLabel::native("Set Source", "Quelle festlegen"))
            .mutation("setFrame", LocalizedLabel::native("Set Frame", "Rahmen festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("clearTiles", LocalizedLabel::native("Clear Tiles", "Kacheln leeren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            // 🐚️ Host side-effect — exports the generated tile-morph prompt to the user (no document mutation).
            .shell_action("copyPrompt", LocalizedLabel::native("Copy Prompt", "Prompt kopieren"))
            .shell_action("exportVideoFromDeck", LocalizedLabel::native("Export Video From Deck", "Video aus Deck exportieren"))
            // 👁️ Ephemeral view state — engagement draft, locale. Selection/hover are framework-owned
            // now (see `.interaction(...)` below): interactionSelect/interactionHover/clearSelection/
            // selectAll/setSelectionMode/setInteractionGranularity auto-inject, never declared here
            // (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("noMutation", LocalizedLabel::native("No Operation", "Keine Aktion"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 🎛️ Declared arg schemas for palette-parametric actions (materialized before dispatch).
            .action_args("seedGrid", vec![
                ActionArgDef::number("rows", LocalizedLabel::native("Rows", "Zeilen")).required().default_value(2),
                ActionArgDef::number("columns", LocalizedLabel::native("Columns", "Spalten")).required().default_value(2),
            ])
            .action_args("setSource", vec![ActionArgDef::text("src", LocalizedLabel::native("Source", "Quelle")).required()])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("demo", LocalizedLabel::native("Demo", "Demo"))])
                    .required()
                    .default_value("demo"),
            ])
            // 🎛️ App-scope command — see `🎮️commands/🌐️seed-grid::reset_grid`'s doc comment for why this
            // isn't `seedGrid`/`clearTiles`.
            .app_command("resetGrid", LocalizedLabel::native("Reset to Default Grid", "Auf Standardraster zurücksetzen"), "document", ActionKind::Mutation)
            .action_interactive_job("seedGrid", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addTile", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteTile", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("renameTiles", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchTileCrops", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setSource", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFrame", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearTiles", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("resetGrid", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("copyPrompt", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("exportVideoFromDeck", InteractiveJobClassification::BatchOnlyPendingRewrite)
            // 🕹️ The framework-owned "tiles" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the document panel
            // tree (`.interaction_domain("tiles")`) and the tile-editor canvas's pick selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity, replacing the deleted bespoke `setSelectedIds` view action.
            .interaction(InteractionDefinition {
                id: PRESENT_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Tiles", "Kacheln"),
                granularities: vec![GranularityDefinition { id: PRESENT_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Tile", "Kachel"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(tile_editor::PRESENT_PLAY_WINDOW_MAIN, vec![InteractionRef::new(PRESENT_INTERACTION_DOMAIN)])
            .config(semio_framework_plugin::resolve_ready(AnimatePresentPlayApp::config_spec()))
            .io(present_io())
            // 🚧️ SDK GAP (contract §2.4): no `.example(...)`/`.workflow(...)` on `EditorBuilder` — the
            // old `crate::examples::art_present_demo::source()` app-level example registration and the
            // no-op `.workflow("animate", "Animate", "deck")` call are dropped here (not silently:
            // reported in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::artifacts::present::examples::...`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
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

    pub type PresentApp = VcsArtifactApp<EditorApp<AnimatePresentPlayApp>>;

    /// ✏️ `AnimatePresentPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<AnimatePresentPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<E>`
    /// builds it.
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn present_app() -> PresentApp {
        new_app::<EditorApp<AnimatePresentPlayApp>>().await
    }

    /// 🧪️ Adapts `create_animate_present_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `new_app_with_registry`/
    /// `testkit::assert_declared_actions_bridge_to_commands` still expect — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    fn animate_present_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_animate_present_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn present_app_with_registry() -> PresentApp {
        new_app_with_registry::<EditorApp<AnimatePresentPlayApp>>(animate_present_app_manifest_for_testkit).await
    }

    pub async fn dispatch(app: &mut PresentApp, command: PresentCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }

    pub async fn render(app: &mut PresentApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️RetainedCommandEnvelope
    #[test]
    fn retained_command_fixture_matches_exact_routes_and_serde_json_boundaries() {
        use store::ArtifactStoreOneItemPreparationFactory as _;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🎯️retained-command-limits.json")).expect("language-neutral retained fixture");
        let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
        assert_eq!(migrated, ANIMATE_PRESENT_RETAINED_TOOL_IDS);
        assert_eq!(ANIMATE_PRESENT_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
        assert_eq!(fixture["limits"]["configValueBytes"].as_u64(), Some(ANIMATE_PRESENT_CONFIG_VALUE_BYTES as u64));
        assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(ANIMATE_PRESENT_CONFIG_STEP_BYTES as u64));
        let factory = AnimatePresentConfigPreparationFactory;
        for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
            let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
            let mutation = PresentConfigMutation::SetEngagementInput { value };
            let encoded = serde_json::to_vec(&mutation).expect("third-party JSON encode");
            let decoded: PresentConfigMutation = serde_json::from_slice(&encoded).expect("third-party JSON decode");
            assert_eq!(decoded, mutation);
            assert_eq!(factory.preflight(&decoded, None, store::HistoryLane::Document).is_ok(), case["accepted"].as_bool().expect("admission oracle"));
        }
    }

    #[test]
    fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
        use std::io::Write as _;
        use store::ArtifactStoreOneItemPreparation as _;
        let value = "x".repeat(ANIMATE_PRESENT_CONFIG_VALUE_BYTES);
        let mut preparation = AnimatePresentConfigPreparation {
            base: None, mutation: Some(PresentConfigMutation::SetEngagementInput { value }), description: None, authority: None, candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
        preparation.cancel();
        assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
        preparation.begin_close();
        assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
        assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 }));
        assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
        assert!(preparation.terminal_is_empty());
        let mut counter = AnimatePresentConfigByteCounter { bytes: 0 };
        assert_eq!(counter.write(&[0; 4_096]).expect("maximum serialized envelope"), 4_096);
        assert!(counter.write(&[0]).is_err());
    }
    //#endregion 🧪️RetainedCommandEnvelope

    use crate::editor::animate::testkit::present_app;
    use protocol::OpText;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;

    #[semio_framework_async_macros::async_test]
    async fn deck_schema_is_animate_present() {
        assert_eq!(default_present_snapshot().schema, PRESENT_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = present_app().await;
        app.dispatch_typed(PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).await.expect("seed grid");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection")).1.len(), 4);
        app.handle_action("undo", None, &meta("local")).await.expect("undo");
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection")).1.is_empty());
        app.handle_action("redo", None, &meta("local")).await.expect("redo");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection")).1.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_unknown_body_key_reports_it_by_name() {
        use semio_framework_plugin::ViewModel;
        let mut app = present_app().await;
        let node = app.render("some.unknown.body", None, &ViewModel::default()).await.expect("render unknown");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Unknown body: some.unknown.body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn app_manifest_declares_expected_operations_and_shell_actions() {
        use semio_framework_plugin::ActionKind;
        let definition = create_animate_present_app();
        let operation_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).filter(|action| matches!(action.kind, ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
        for expected in ["seedGrid", "addTile", "deleteTile", "deleteSelection", "renameTiles", "patchTileCrops", "setSource", "setFrame", "setActiveExample", "clearTiles", "engagementSubmit"] {
            assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
        }
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "exportVideoFromDeck" && matches!(action.kind, ActionKind::Shell)));
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "engagementInput" && matches!(action.kind, ActionKind::View)));
    }

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_animate_present_app()).expect("app definition json");
        assert!(json.contains(tile_editor::PRESENT_PLAY_WINDOW_MAIN), "window kind missing from the manifest: {json}");
        assert!(json.contains(main::PRESENT_PLAY_MODE_MAIN), "mode missing from the manifest");
        for body in [PRESENT_PLAY_BODY_DOCUMENT, PRESENT_PLAY_BODY_CATALOGUE, PRESENT_PLAY_BODY_DETAILS] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains(PRESENT_DOCUMENT_SCHEMA), "artifact kind missing from the manifest");
    }

    /// 🕹️ The `tiles` domain is declared `HierarchyProvider::Flat`, non-transitive, broadcast, and
    /// bound to the tile-editor window (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_declares_the_tiles_interaction_domain() {
        let definition = create_animate_present_app();
        let domain = definition.interactions.iter().find(|interaction| interaction.id == PRESENT_INTERACTION_DOMAIN).expect("tiles interaction domain declared");
        assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
        assert!(domain.selection.broadcast);
        assert!(!domain.selection.transitive);
        let canvas_window = definition.window_kinds.iter().find(|window| window.id == tile_editor::PRESENT_PLAY_WINDOW_MAIN).expect("tile-editor window declared");
        assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == PRESENT_INTERACTION_DOMAIN));
    }
    //#endregion 🔖️ManifestSanity

    /// 🧬️ Two independent instances start empty, apply DISJOINT edits (A adds a tile, B sets the
    /// source), and exchanging operations over a `MemoryBackbone` converges both sides to contain BOTH
    /// edits — impossible with whole-document snapshots, which would clobber one another.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        use store::MemoryBackbone;
        let mut instance_a = present_app().await;
        let mut instance_b = present_app().await;
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://animate-present-convergence", "mem://animate-present-convergence").await;
        instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
        instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

        instance_a.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: Some(crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 0.3, height: 0.3 }) }), &meta("actor-a")).await.expect("a adds tile");
        let (mut source, _) = crate::artifacts::present::present_working_scene(&instance_b.snapshot().await.expect("projection"));
        source.kind = "video".into();
        instance_b.dispatch_typed(PresentCommand::SetSource(set_source::SetSource { source }), &meta("actor-b")).await.expect("b sets source kind");

        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

        let (source_a, tiles_a) = crate::artifacts::present::present_working_scene(&instance_a.snapshot().await.expect("projection"));
        let (source_b, tiles_b) = crate::artifacts::present::present_working_scene(&instance_b.snapshot().await.expect("projection"));
        assert_eq!(tiles_a.len(), 1, "instance A keeps its own tile");
        assert_eq!(tiles_b.len(), 1, "instance B converges on A's tile");
        assert_eq!(source_a.kind, "video", "instance A converges on B's source edit");
        assert_eq!(source_b.kind, "video", "instance B keeps its own source edit");
    }

    //#region 🔖️PortTests
    #[semio_framework_async_macros::async_test]
    async fn present_io_declares_frames_in_and_document_ports() {
        let ports = AnimatePresentPlayApp::io().await.expect("io").all_ports().await;
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "frames:in"));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_frames_in_inserts_a_new_tile() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry().await;
        let before = crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection")).1.len();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "hero-frame", "src": "/frames/hero.png" }).to_string() } };
        app.import_media("frames:in", &media, &meta("local")).await.expect("import frames:in");
        let (_, after_tiles) = crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection"));
        assert_eq!(after_tiles.len(), before + 1);
        assert_eq!(after_tiles.last().expect("imported tile").name, "hero-frame");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_frames_in_places_repeated_imports_in_distinct_cells() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry().await;
        for _ in 0..2 {
            let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "frame" }).to_string() } };
            app.import_media("frames:in", &media, &meta("local")).await.expect("import frames:in");
        }
        let (_, tiles) = crate::artifacts::present::present_working_scene(&app.snapshot().await.expect("projection"));
        assert_eq!(tiles.len(), 2);
        assert_ne!(tiles[0].crop, tiles[1].crop, "repeated imports land in distinct cells");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_rejects_unknown_port() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        let mut app = testkit::present_app_with_registry().await;
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "{}".into() } };
        assert!(app.import_media("not-a-port", &media, &meta("local")).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_present_snapshot_has_no_tiles() {
        let snapshot = crate::artifacts::present::schema::empty_present_snapshot();
        assert!(crate::artifacts::present::present_working_scene(&snapshot).1.is_empty());
    }

    /// 🌱️ Relocated from the former artifact-tree `⚙️engine`'s own tests (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `present_io`'s relocation to
    /// this file's `🔖️Io` region.
    #[semio_framework_async_macros::async_test]
    async fn present_io_declares_the_frames_in_port() {
        let io = present_io();
        assert_eq!(io.document_schema, PRESENT_DOCUMENT_SCHEMA);
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "frames:in");
        assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(port.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[semio_framework_async_macros::async_test]
    async fn frame_import_placement_is_deterministic_and_non_overlapping() {
        let first = next_frame_tile_crop(0);
        let second = next_frame_tile_crop(1);
        assert_ne!(first, second);
        assert_eq!(next_frame_tile_id(0), "frame-1");
        assert_eq!(next_frame_tile_id(1), "frame-2");
        // 🧮️ Pure function of the count, not a mutating counter.
        assert_eq!(next_frame_tile_crop(0), first);
    }
    //#endregion 🔖️PortTests

    //#region 🔖️CommandSurfaceTests
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 18, "every PresentCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. This is what
    /// a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keywords: [(&str, &str); 18] = [
            ("seedGrid", "seed-grid"),
            ("addTile", "add-tile"),
            ("deleteTile", "delete-tile"),
            ("deleteSelection", "delete-selection"),
            ("renameTiles", "rename-tiles"),
            ("patchTileCrops", "patch-tile-crops"),
            ("setSource", "set-source"),
            ("setFrame", "set-frame"),
            ("setActiveExample", "set-active-example"),
            ("clearTiles", "clear-tiles"),
            ("engagementSubmit", "engagement-submit"),
            ("resetGrid", "reset-grid"),
            ("engagementInput", "engagement-input"),
            ("canvasPointerDown", "canvas-pointer-down"),
            ("setLocale", "set-locale"),
            ("noMutation", "no-op"),
            ("copyPrompt", "copy-prompt"),
            ("exportVideoFromDeck", "export-video-from-deck"),
        ];
        for (command, (id, keyword)) in every_command().into_iter().zip(expected_keywords) {
            assert_eq!(command.command_id(), id);
            let printed = OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), keyword, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
    /// bytes captured from the pre-merge `present_protocol` crate. A regression here is a real format
    /// break, not a test-fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let with_crop = PresentCommand::AddTile(add_tile::AddTile { crop: Some(crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 }) });
        let without_crop = PresentCommand::AddTile(add_tile::AddTile { crop: None });
        assert!(OpText::print_op(&with_crop).starts_with("add-tile"));
        assert!(OpText::print_op(&without_crop).starts_with("add-tile"));
        store::os_store::test_support::assert_op_text_binary_equivalence(&with_crop);
        store::os_store::test_support::assert_op_text_binary_equivalence(&without_crop);

        let with_layer = PresentCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some("t1".into()) });
        let without_layer = PresentCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: None });
        store::os_store::test_support::assert_op_text_binary_equivalence(&with_layer);
        store::os_store::test_support::assert_op_text_binary_equivalence(&without_layer);
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<PresentCommand> {
        vec![
            PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 3 }),
            PresentCommand::AddTile(add_tile::AddTile { crop: Some(crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 }) }),
            PresentCommand::DeleteTile(delete_tile::DeleteTile { id: "t1".into() }),
            PresentCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec!["t1".into(), "t2".into()], value: "Hero".into() }),
            PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec!["t1".into()], field: "width".into(), value: 0.4 }),
            PresentCommand::SetSource(set_source::SetSource { source: crate::artifacts::present::default_figure_tile_source() }),
            PresentCommand::SetFrame(set_frame::SetFrame { frame: crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }),
            PresentCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
            PresentCommand::ClearTiles(clear_tiles::ClearTiles {}),
            PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "2x2".into() }),
            PresentCommand::ResetGrid(reset_grid::ResetGrid {}),
            PresentCommand::EngagementInput(engagement_input::EngagementInput { value: "add".into() }),
            PresentCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: Some("t1".into()) }),
            PresentCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            PresentCommand::NoOperation(no_operation::NoOperation {}),
            PresentCommand::CopyPrompt(copy_prompt::CopyPrompt {}),
            PresentCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: "output/x".into(), scene_json: "{}".into() }),
        ]
    }
    //#endregion 🔖️CommandSurfaceTests
}
//#endregion 🧪️Tests
