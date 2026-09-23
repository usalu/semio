//! 🎞️ Animate editor — the `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET), the aggregated command enum and the manifest
//! stitch. B1: the pure-trait pivot — `AnimatePresentationPlayApp` is a unit struct; every former
//! `AnimatePresentationPlayRuntime` field (selection, engagement draft) now lives in
//! `crate::editor::animate::config::PresentationConfig`, written via `PresentationConfigMutation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `PresentationCommand` channel via `ArtifactEditor::handle`. MUST NOT be imported from the sibling
//! `👁️viewer` module (`policyViewerPurityBreaches`).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🖊️main/🪟️windows/🖼️tile-editor`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, pure document helpers in
//! `crate::schema`, and stateful behaviour (the Manim-class animation core + the
//! headless video renderer, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) in this app's
//! own `⚙️engine`. This file is a routing table: `handle` → `PresentationCommand::dispatch`, `render` →
//! body-key → node, `🔖️Io`/`🔌️Registration` regions below, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::editor::animate::commands::{
    add_tile, canvas_pointer_down, clear_tiles, copy_prompt, delete_selection, delete_tile, engagement_input, engagement_submit, export_video_from_deck, no_operation, patch_tile_crops, rename_tiles, reset_grid, seed_grid, set_active_example,
    set_frame, set_source,
};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::modes::main;
use crate::editor::animate::modes::main::windows::tile_editor;
use crate::editor::animate::panels::{artifact, catalogue, inspection};
use crate::editor::animate::terminology::animate_presentation_labels;
use crate::mutations::create_tile::CreateTile;
use crate::op::PresentationMutation;
use crate::standards::v1::subsets::any::schema::build_tile_morph_prompt;
use crate::{default_presentation_snapshot, FigureTileDraft, PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion};
use semio_framework_plugin::ArtifactReservedJob;
// 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
// `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
// `.workflow(...)` on this builder (see `🔖️Manifest` below for what got dropped, not silently).
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionKind, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane,
    ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, EditorApp, Effect, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaError,
    MediaPayload, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use std::collections::HashSet;
use store::EngineHandles;

//#region 🔖️Constants
pub const PRESENTATION_PLAY_APP_ID: &str = "s.animate.presentation@1/*#editor";
pub use artifact::PRESENTATION_PLAY_BODY_ARTIFACT;
pub use catalogue::PRESENTATION_PLAY_BODY_CATALOGUE;
pub use inspection::PRESENTATION_PLAY_BODY_DETAILS;
pub use tile_editor::PRESENTATION_PLAY_BODY_MAIN;

/// 🎯️ Binds catalogue actions to the canonical presentation editor.
pub fn animate_presentation_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(PRESENTATION_PLAY_APP_ID).action(action, args)
}

/// 🌳️ The framework's one node-list admission — the panel-local copy is gone (ticket
/// 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.3).
pub use semio_framework_plugin::ui_node_list;

/// 🏷️ Admits a semantic presentation label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| ui_capacity_error())
}

/// 📝️ Admits a fixed presentation UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a presentation UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of presentation UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🗺️ Admits structured presentation action arguments.
pub fn ui_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(ui_capacity_error)?;
    for (key, value) in values {
        builder.push(key.to_string(), value).map_err(|_| ui_capacity_error())?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("animate.ui.capacity", "presentation UI admission failed")
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "tiles" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Flat` over the tile grid (a document panel tree binds it
/// directly; the canvas hit-tests a click and asks the framework to apply it, mirroring `🖍️draw`'s
/// "strokes" domain).
pub const PRESENTATION_INTERACTION_DOMAIN: &str = "tiles";
pub const PRESENTATION_INTERACTION_GRANULARITY: &str = "tile";

/// 🕹️ Per-dispatch scratch: the "tiles" domain's current selection, resolved once by
/// `ArtifactApp::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself.
pub struct PresentationDispatchCtx {
    pub selected_ids: Vec<String>,
}

/// 🕹️ JSON-encodes `ids` as the `Vec<InteractionTarget>` string the framework's `interactionSelect`
/// action requires in its `targets` arg — every hit id shares the domain's one granularity.
fn interaction_targets_json(ids: &[String]) -> String {
    let targets = ids.iter().map(|id| dsl::os_pack::json::object([("granularity".to_string(), dsl::os_pack::json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)), ("id".to_string(), dsl::os_pack::json::Value::from(id.clone()))])).collect();
    dsl::os_pack::json::to_string(&dsl::os_pack::json::Value::Array(targets))
}

/// 🕹️ Requests the shell to redispatch the framework-owned `interactionSelect` verb through its
/// normal action funnel — the only way `canvas-pointer-down`'s hit test can drive selection now that
/// it is framework-owned state, never a `PresentationConfigMutation` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(dsl::DslValue::object([
            ("domainId".to_string(), dsl::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), dsl::DslValue::String(interaction_targets_json(ids))),
            ("merge".to_string(), dsl::DslValue::String(merge.into())),
            ("method".to_string(), dsl::DslValue::String("pick".into())),
        ])),
    }
}
//#endregion 🔖️Interaction

//#region 🔖️Io
/// 🔌️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors `create_animate_presentation_app`'s `.artifact_kind(...)` literal (schema/
/// media type copied verbatim) plus the extra `frames:in` input port (Wave-2 port recipe).
pub fn presentation_io() -> AppIo {
    AppIo {
        artifact_schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        artifact_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Presentation, form: semio_framework_plugin::MediaForm::Deck },
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
        artifact: semio_framework_plugin::ArtifactPresentation { id: PRESENTATION_DOCUMENT_SCHEMA.into(), name: "Animate Presentation Deck".into(), dimension: "2d".into(), component_kind: "panel".into() },
    }
}

/// 🎞️ `frames:in` placement (Wave-2 port recipe) — `PresentationSnapshot` models one shared background
/// `source` image with named crop-`tiles` over it; there is no per-tile independent raster payload in
/// this schema, so an incoming `2d.image` frame becomes a new tile positioned in a deterministic
/// contact-sheet grid (4 columns) rather than replacing `source` — exactly the surface `seedGrid`/
/// `addTile` (see the app's `🎮️commands/🀄️add-tile`/`🎮️commands/🌱️seed-grid`) already let a user crop/arrange
/// candidate frames on. Pure: both functions depend only on the current tile COUNT, so repeated imports
/// land in distinct, stable cells without needing a live host/counter.
const FRAME_IMPORT_GRID_COLUMNS: usize = 4;

pub fn next_frame_tile_id(existing_tile_count: usize) -> String {
    format!("frame-{}", existing_tile_count + 1)
}

pub fn next_frame_tile_crop(existing_tile_count: usize) -> crate::FigureTileFrame {
    let cell = 1.0 / FRAME_IMPORT_GRID_COLUMNS as f64;
    let column = existing_tile_count % FRAME_IMPORT_GRID_COLUMNS;
    let row = existing_tile_count / FRAME_IMPORT_GRID_COLUMNS;
    crate::standards::v1::subsets::any::schema::clamp_tile_crop(&crate::FigureTileFrame { x: column as f64 * cell, y: (row as f64 * cell).min(1.0 - cell), width: cell, height: cell })
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
pub(crate) fn valid_tile_ids(deck: &PresentationSnapshot, ids: Vec<String>) -> Vec<String> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let valid: HashSet<&str> = tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}

/// 🎞️ `frames:in` display name (Wave-2 port recipe) — a `Structured` payload's `"name"`/`"src"` field
/// (falling back to a generic label), a `Binary` payload's leading blob-hash characters.
fn frame_media_name(port: &str, media: &Media) -> Result<String, MediaError> {
    match &media.payload {
        MediaPayload::Structured { json, .. } => {
            let value = dsl::os_pack::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            Ok(value.get("name").and_then(|v| v.as_str()).or_else(|| value.get("src").and_then(|v| v.as_str())).map_or_else(|| "Imported frame".into(), str::to_string))
        }
        MediaPayload::Binary { blob_hash, .. } => Ok(format!("frame-{}", &blob_hash[..blob_hash.len().min(8)])),
    }
}

/// 📋️ Host effect delivering the generated tile-morph prompt to the user as a downloadable markdown
/// file — the genuine shell side-effect that replaces the retired ephemeral clipboard scratch (the
/// landed `Effect` contract carries no clipboard variant, so the prompt is exported as media).
/// Shared by `🎮️commands/📋️copy-prompt::copy_prompt` and `🎮️commands/⌨️engagement::engagement_submit`'s
/// `"copy"`/`"copy prompt"` keywords.
pub(crate) fn tile_morph_prompt_effect(deck: &PresentationSnapshot) -> Effect {
    let (source, tiles) = crate::presentation_working_scene(deck);
    Effect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&source, &tiles), encoding: None }
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "reset the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/📥️set-source::set_active_example` uses instead of the banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_presentation_document_effect(document: &PresentationSnapshot) -> Effect {
    let pack = <PresentationSnapshot as store::ArtifactPack>::encode_pack(document);
    // 🪦️ NOT `create_document_envelope` + `print_document_spr`: an envelope is a terminal store shell
    // whose `Drop` asserts that its app-owned bounded retirement authority detached every nested
    // owner first, and nothing here ever mounts or retires it — so building this effect panicked the
    // guest (`unreachable`) at boot, before any window kind existed. `🗒️note`/`✒️writer`/`📐️cad` all
    // take the `empty_document_spr` route; the log is edit-free by construction either way.
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("presentation", PRESENTATION_DOCUMENT_SCHEMA));
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Helpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `AnimatePresentationPlayApp::Command` — the SOLE dispatch surface for animate presentation's own
    /// behavior, assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest
    /// action id (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire
    /// keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// `"resetGrid" as "reset-grid"` is the row that proves it. The app owner is carried by the
    /// qualified command address rather than repeated in this local id. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PresentationCommand for PresentationSnapshot, PresentationMutation, PresentationConfig, PresentationConfigMutation, ctx = PresentationDispatchCtx {
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
        "noMutation" as "no-op" => no_operation::NoOperation,
        "copyPrompt" as "copy-prompt" => copy_prompt::CopyPrompt,
        "exportVideoFromDeck" as "export-video-from-deck" => export_video_from_deck::ExportVideoFromDeck,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const ANIMATE_PRESENTATION_RETAINED_TOOL_IDS: &[&str] = &[
    "seedGrid",
    "addTile",
    "deleteTile",
    "deleteSelection",
    "renameTiles",
    "patchTileCrops",
    "setSource",
    "setFrame",
    "setActiveExample",
    "clearTiles",
    "engagementSubmit",
    "resetGrid",
    "engagementInput",
    "canvasPointerDown",
    "noMutation",
    "copyPrompt",
];
const ANIMATE_PRESENTATION_RETAINED_PAYLOAD_SCHEMA: &str = "animate.presentation.tool-command.v1";
const ANIMATE_PRESENTATION_RETAINED_RAW_BYTES: usize = 8_192;
const ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS: usize = 1;
const ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES: usize = 512;
const ANIMATE_PRESENTATION_CONFIG_BASE_BYTES: usize = 512;
const ANIMATE_PRESENTATION_CONFIG_STEP_BYTES: usize = 4_096;
/// 🀄️ The largest tile roster one retained verb may build. A `ReplaceTiles` is a single store row
/// whatever its length, so nothing downstream bounds it — this constant is the only ceiling.
const ANIMATE_PRESENTATION_MAXIMUM_TILES: usize = 256;
/// 📦️ The admission envelope ONE encoded document mutation may occupy on the retained artifact lane.
/// A `ReplaceTiles` carrying the contract's whole 64-cell work ceiling is the largest row this app
/// can stage (id + name + four crop floats per tile), which is an order of magnitude under this.
const ANIMATE_PRESENTATION_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 65_536;
const ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "seedGrid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addTile", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteTile", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "renameTiles", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchTileCrops", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSource", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "clearTiles", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "resetGrid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "copyPrompt", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

/// 🧾️ The ONE execution contract this app's retained factory publishes AND declares in its
/// `bounded_first_step_tool_proofs!` row. `validate_tool_job_rows`'s `typed_join` compares the two
/// for equality, so a second literal in the proof row is a rejection waiting to happen — a
/// `bounded_first_step(…)` literal there against this `resumable(…)` factory shape is exactly what
/// used to fail the whole catalog (`interactive-job.catalog-authority`, `typed_join=false`).
fn animate_presentation_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(ANIMATE_PRESENTATION_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500, 1, 1)
}

/// 🕹️ The tile ids the framework-owned `tiles` interaction domain holds selected, read straight off
/// the retained job's own `InteractionState` — the retained lane used to hand every handler an EMPTY
/// selection, which silently turned `deleteSelection`/`renameTiles`/`patchTileCrops` into no-ops the
/// moment they became retained jobs. `ArtifactApp::handle` reads the same domain (`:830`).
fn animate_presentation_selected_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction.selection.get(PRESENTATION_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

/// 📏️ Every retained verb here is ONE semantic work item — the job reduces the whole command in a
/// single `BoundedArtifactCommandWork` step, and `ArtifactRetainedCommandPayload::try_new` is handed
/// `ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS` (1) as its preflight ceiling, so an extent priced per
/// emitted ROW is refused outright (`retained command exceeds semantic work capacity` — measured on
/// `seedGrid` 2×2 before this was corrected). Fan-out is therefore bounded HERE, by refusing the
/// command whose payload would build more than this app's tile ceiling, not by the extent number.
fn animate_presentation_retained_extent(command: &PresentationCommand, _snapshot: &PresentationSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bounded = |length: usize| (length <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES).then_some(1);
    match command {
        PresentationCommand::SeedGrid(payload) => (payload.rows as usize).checked_mul(payload.columns as usize).filter(|cells| *cells > 0 && *cells <= ANIMATE_PRESENTATION_MAXIMUM_TILES).map(|_| 1),
        PresentationCommand::AddTile(_) | PresentationCommand::DeleteSelection(_) | PresentationCommand::ClearTiles(_) | PresentationCommand::ResetGrid(_) | PresentationCommand::NoOperation(_) | PresentationCommand::CopyPrompt(_) | PresentationCommand::SetFrame(_) => Some(1),
        PresentationCommand::DeleteTile(payload) => bounded(payload.id.len()),
        PresentationCommand::RenameTiles(payload) => (payload.value.len() <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES && payload.ids.len() <= ANIMATE_PRESENTATION_MAXIMUM_TILES).then_some(1),
        PresentationCommand::PatchTileCrops(payload) => (payload.field.len() <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES && payload.ids.len() <= ANIMATE_PRESENTATION_MAXIMUM_TILES).then_some(1),
        PresentationCommand::SetSource(payload) => bounded(payload.source.src.len()),
        PresentationCommand::SetActiveExample(payload) => bounded(payload.example_id.len()),
        PresentationCommand::EngagementSubmit(payload) => bounded(payload.value.len()),
        PresentationCommand::EngagementInput(payload) => (payload.value.len() <= ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES).then_some(1),
        PresentationCommand::CanvasPointerDown(payload) => bounded(payload.layer_id.as_ref().map_or(0, String::len)),
        PresentationCommand::ExportVideoFromDeck(_) => None,
    }
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn animate_presentation_retained_reduce(
    command: &PresentationCommand,
    snapshot: &PresentationSnapshot,
    config: &PresentationConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<AnimatePresentationPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PresentationMutation, PresentationConfigMutation, NoDraftMutation>, Fault> {
    if !ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.contains(&command.command_id()) || animate_presentation_retained_extent(command, snapshot, interaction).is_none() {
        return Err(Fault::from("animate-presentation-retained-route-mismatch"));
    }
    let document = ArtifactView::with_operation(snapshot, history, operation.clone());
    let config = ConfigView { snapshot: config, window: None };
    let mut context = PresentationDispatchCtx { selected_ids: animate_presentation_selected_ids(interaction) };
    command.dispatch(&document, &config, &mut context)
}

struct AnimatePresentationRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl AnimatePresentationRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for AnimatePresentationRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<AnimatePresentationPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<AnimatePresentationPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        ANIMATE_PRESENTATION_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        animate_presentation_retained_contract()
    }
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
        if input.declared_bytes() > ANIMATE_PRESENTATION_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Animate Presentation retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for AnimatePresentationRetainedCommandJobFactory {
    type Owner = EditorApp<AnimatePresentationPlayApp>;
    const TOOL_IDS: &'static [&'static str] = ANIMATE_PRESENTATION_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PRESENTATION_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
struct AnimatePresentationConfigPreparationFactory;

struct AnimatePresentationConfigPreparation {
    base: Option<store::SnapshotRead<PresentationConfig>>,
    mutation: Option<PresentationConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(PresentationConfig, PresentationConfigMutation, PresentationConfigMutation)>,
    sealed_candidate: Option<(PresentationConfig, protocol::Edit<PresentationConfigMutation>)>,
    serialized_bytes: Option<usize>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn animate_presentation_config_edit(forward: PresentationConfigMutation, inverse: PresentationConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<PresentationConfigMutation> {
    let id = format!("animate-presentation-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse: vec![inverse],
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

struct AnimatePresentationConfigByteCounter {
    bytes: usize,
}

impl std::io::Write for AnimatePresentationConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn animate_presentation_config_edit_bytes(edit: &protocol::Edit<PresentationConfigMutation>) -> Result<usize, String> {
    let mut counter = AnimatePresentationConfigByteCounter { bytes: 0 };
    use std::io::Write as _;
    counter.write_all(dsl::json::to_json_string(edit).as_bytes()).map_err(|_| "Animate Presentation config edit exceeds its serialized byte envelope".to_string())?;
    Ok(counter.bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparationFactory {
    fn preflight(&self, mutation: &PresentationConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            PresentationConfigMutation::SetEngagementInput(payload) => payload.value.len(),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Animate Presentation config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: ANIMATE_PRESENTATION_CONFIG_STEP_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation>> {
        let mutation_bytes = match &request.mutation {
            PresentationConfigMutation::SetEngagementInput(payload) => payload.value.len(),
        };
        if request.lane != store::HistoryLane::Document
            || mutation_bytes > ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES
            || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES)
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(AnimatePresentationConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            sealed_candidate: None,
            serialized_bytes: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < ANIMATE_PRESENTATION_CONFIG_STEP_BYTES || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        if self.candidate.is_none() && self.sealed_candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Animate Presentation config preparation lost its exact base root".to_string())?.get();
            let base_bytes = base.engagement_input.len();
            if base_bytes > ANIMATE_PRESENTATION_CONFIG_BASE_BYTES {
                return Err("Animate Presentation config base exceeds retained byte capacity".into());
            }
            let mutation = self.mutation.take().ok_or_else(|| "Animate Presentation config preparation lost its mutation owner".to_string())?;
            let mut post = base.clone();
            let inverse = match &mutation {
                PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value }) => {
                    post.engagement_input = value.clone();
                    PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: base.engagement_input.clone() })
                }
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Animate Presentation config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "Animate Presentation config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, animate_presentation_config_edit(forward, inverse, self.description.take(), authority)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.sealed_candidate.as_ref().ok_or_else(|| "Animate Presentation config preparation lost its semantic edit".to_string())?;
            let bytes = animate_presentation_config_edit_bytes(edit)?;
            if bytes.saturating_add(post.engagement_input.len()).saturating_add(512) > ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
                return Err("Animate Presentation config publication exceeds the 4096-byte complete envelope".into());
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, edit) = self.sealed_candidate.take().ok_or_else(|| "Animate Presentation config preparation lost its validated edit".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Animate Presentation config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
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
        if (self.prepared.is_some() || self.sealed_candidate.is_some() || self.candidate.is_some() || self.mutation.is_some() || self.description.is_some()) && grant.maximum_bytes < ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: ANIMATE_PRESENTATION_CONFIG_STEP_BYTES });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Animate Presentation config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 🎞️ReservedImport
/// 🎞️ The framework registers the reserved `import-media` factory for every app but never a concrete
/// job, so `VcsArtifactApp::import_media` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from [`ArtifactEditor::build_reserved_tool_job`]. Presentation's one
/// inbound port is `frames:in`. Same shape as `📏️layout`'s `LayoutImportJob`.
const PRESENTATION_IMPORT_TOOL_ID: &str = "import-media";
const PRESENTATION_IMPORT_PORT: &str = "frames:in";

/// 🎞️ Presentation's ONE concrete resumable importer. Two bounded steps: decode the `frames:in` frame
/// through [`AnimatePresentationPlayApp::import_media`] — the single decoding authority, shared with
/// every non-interactive caller — then publish its tile mutation through the completion authority.
struct PresentationImportJob {
    port: String,
    media: Option<Media>,
    snapshot: Option<std::sync::Arc<PresentationSnapshot>>,
    history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    mutations: Vec<PresentationMutation>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<AnimatePresentationPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<AnimatePresentationPlayApp>>>,
}

fn presentation_job_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn presentation_job_fault(cx: &mut StepContext<'_>, detail: &str) -> StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: presentation_job_payload(cx, JobPayloadStream::Fault, bounded) })
}

impl PresentationImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port,
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: Vec::new(),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    fn decode(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.port != PRESENTATION_IMPORT_PORT {
            return Some(presentation_job_fault(cx, "presentation import only implements frames:in"));
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                return Some(presentation_job_fault(cx, "presentation import lost its media, snapshot or history authority"));
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            AnimatePresentationPlayApp::import_media(PRESENTATION_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = emit.artifact_mutations;
                self.decoded = true;
                None
            }
            Err(error) => Some(presentation_job_fault(cx, &error.to_string())),
        }
    }
}

impl InteractiveJob for PresentationImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return presentation_job_fault(cx, "presentation import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("presentation-import-decode");
            if let Some(outcome) = self.decode(cx) {
                return outcome;
            }
            cx.consume_fuel(1);
            return StepOutcome::CheckpointReady(Checkpoint { state: presentation_job_payload(cx, JobPayloadStream::CheckpointState, &[1]), applied_progress: 1 });
        }
        cx.set_stage("presentation-import-publish");
        if !self.completed {
            let mutations = std::mem::take(&mut self.mutations);
            let Some(completion) = self.completion.as_ref() else {
                return presentation_job_fault(cx, "presentation import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return presentation_job_fault(cx, "presentation import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return presentation_job_fault(cx, &message);
            }
            self.completed = true;
        }
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) if ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for PresentationImportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        // 🧊️ An imported frame only ever yields `CreateTile`, whose draft is plain owned text and
        // floats, so a popped mutation closes on drop.
        if self.mutations.pop().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutations.capacity() > 0 {
            self.mutations = Vec::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.media.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.port.is_empty() || self.port.capacity() > 0 {
            self.port = String::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.history.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "presentation import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "presentation import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}
//#endregion 🎞️ReservedImport

//#region 🔖️AnimatePresentationPlayApp
/// 🧪️ B1: unit struct — every former `AnimatePresentationPlayRuntime` field now lives in
/// `crate::editor::animate::config::PresentationConfig` (see `ArtifactApp::Config`), written through
/// `PresentationConfigMutation`s.
#[derive(Default)]
pub struct AnimatePresentationPlayApp;

impl ArtifactEditor for AnimatePresentationPlayApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🧬️ The loaded-parent child projection, read straight off the snapshot's own `#[child]` fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("animate.child-projection"), error.to_string()))
    }

    type Snapshot = PresentationSnapshot;
    type Mutation = PresentationMutation;
    type Config = PresentationConfig;
    type ConfigMutation = PresentationConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::animate::presence::PresentationPresence;
    type PresenceMutation = crate::editor::animate::presence::PresentationPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = PresentationCommand;

    const DIALECT: Dialect = crate::ANIMATE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PRESENTATION_DOCUMENT_SCHEMA;

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::genesis_presentation_child_pack(snapshot, slot, child_id)
    }

    /// 🧺️ Without these owners the document store holds no `initial_snapshot_retirement_factory`, so
    /// the boot `setActiveExample` archive load is refused with `module.vcs: validation failed:
    /// returned snapshot read requires its exact owned-snapshot retirement factory` — measured
    /// against the live React playground.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📥️ The trait default owns no retained initialization authority, so the host refuses every
    /// archive this app hands back (`artifact-store.persisted-initializer-refused`) — which is
    /// exactly what `setActiveExample`'s `Effect::LoadDocument` is.
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, PRESENTATION_DOCUMENT_SCHEMA, operation, generation))
    }

    /// 📬️ Exact one-item PUBLICATION authority for the DOCUMENT store. Without it every tool
    /// declaring the `artifact` lane is marked `unsupported-publication-contract` at boot and fails
    /// closed with `interactive-job.publication-authority-missing`, which is why this app could own
    /// no retained document verb at all (B1a: "promoting a document verb is a two-part change").
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>(
            "animate-presentation-artifact-retained",
            ANIMATE_PRESENTATION_ARTIFACT_MUTATION_MAXIMUM_BYTES,
        ))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(AnimatePresentationConfigPreparationFactory))
    }

    /// 🧹️ Every store an instance owns closes through its bounded disposer before drop; without these the
    /// instance close faults `interactive-job.close-owned-disposer-missing` at the config-store lane.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(
            semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default())
                .expect("default presentation presence is the exact empty terminal"),
        ))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<AnimatePresentationPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.animate.presentation@1/*#editor",
        artifact_schema: "animate.presentation",
        factory: "AnimatePresentationRetainedCommandJobFactory",
        factory_type: AnimatePresentationRetainedCommandJobFactory,
        contract: animate_presentation_retained_contract(),
        tools: [
            "seedGrid",
            "addTile",
            "deleteTile",
            "deleteSelection",
            "renameTiles",
            "patchTileCrops",
            "setSource",
            "setFrame",
            "setActiveExample",
            "clearTiles",
            "engagementSubmit",
            "resetGrid",
            "engagementInput",
            "canvasPointerDown",
            "noMutation",
            "copyPrompt"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(AnimatePresentationRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("animate-presentation-command-tool-mismatch"));
        }
        if animate_presentation_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("animate-presentation-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, animate_presentation_retained_reduce, animate_presentation_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: None,
                operation: operation_context,
                completion: request.completion,
            },
            PresentationCommand::command_id,
            ANIMATE_PRESENTATION_RETAINED_RAW_BYTES,
            ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🎞️ `import-media` is the only reserved route presentation owns: every inbound `frames:in`
    /// delivery is routed through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`).
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != PRESENTATION_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("presentation import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("presentation import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(PresentationImportJob::new(request, port, media))))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::presentation_envelope_decode_owner_bundle())
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::animate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> PresentationSnapshot {
        default_presentation_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(presentation_io())
    }

    /// 🌱️ `whole_document_operation` stays the trait default (`None`): per `📓️taxonomy.md`, whole-
    /// document replace has no in-history mutation at all (there is no import mutation by locked
    /// decision — see `🎮️commands/📥️set-source::set_active_example`'s `Effect::LoadDocument` instead).
    /// 🎞️ `frames:in` (Wave-2 port recipe): inserts an incoming raster frame as a new tile in a
    /// deterministic contact-sheet grid (see `next_frame_tile_crop`'s doc comment below for why this
    /// schema's single shared `source` means tiles, not `source`, are the natural insertion point).
    /// Never mutates anything directly: the caller applies the returned `Tiles(Add)` through the
    /// ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PresentationSnapshot>) -> Result<Emit<PresentationMutation, PresentationConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "frames:in" {
            return Err(MediaError::NotImplemented);
        }
        let deck = doc.snapshot;
        let count = crate::presentation_working_scene(deck).1.len();
        let id = next_frame_tile_id(count);
        let crop = next_frame_tile_crop(count);
        let name = frame_media_name(port, media)?;
        let tile = FigureTileDraft { id, name, crop };
        Ok(Emit::mutations(vec![PresentationMutation::CreateTile(CreateTile { index: count, tile })]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PresentationCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps a host action id + its staged args onto `PresentationCommand`. The React/wgpu shells
    /// still dispatch `{action, args}` while the guest channel is typed-only, and the trait default
    /// refuses EVERY id (`app.command.unsupported`) — without this bridge the boot
    /// `setActiveExample`, every Actions-pane row and every canvas pick died before reaching
    /// `handle`. The `#[dsl(block)]` payloads (`crop`/`frame`/`source`) decode through
    /// `dsl::from_dsl_value`, the same codec the typed channel uses.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<PresentationCommand, Fault> {
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64)).unwrap_or_default();
        let id_list = |keys: &[&str]| match keys.iter().find_map(|key| args.and_then(|value| value.get(key))) {
            Some(dsl::DslValue::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
            Some(dsl::DslValue::String(raw)) if !raw.is_empty() => vec![raw.clone()],
            _ => Vec::new(),
        };
        fn decode<T: dsl::FromValue>(action: &str, args: Option<&dsl::DslValue>, key: &str) -> Result<T, Fault> {
            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("presentation {action} requires a '{key}' block")))?;
            dsl::from_dsl_value(value).map_err(|error| Fault::from(format!("invalid presentation {action} '{key}': {error}")))
        }
        match action {
            "seedGrid" => Ok(PresentationCommand::SeedGrid(seed_grid::SeedGrid { rows: number_arg(&["rows"]) as u32, columns: number_arg(&["columns", "cols"]) as u32 })),
            "addTile" => Ok(PresentationCommand::AddTile(add_tile::AddTile { crop: args.and_then(|value| value.get("crop")).cloned().map(dsl::from_dsl_value).transpose().map_err(|error| Fault::from(format!("invalid presentation addTile 'crop': {error}")))? })),
            "deleteTile" => Ok(PresentationCommand::DeleteTile(delete_tile::DeleteTile { id: text_arg(&["id", "tileId", "value"]).unwrap_or_default() })),
            "deleteSelection" => Ok(PresentationCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "renameTiles" => Ok(PresentationCommand::RenameTiles(rename_tiles::RenameTiles { ids: id_list(&["ids", "id"]), value: text_arg(&["value", "name"]).unwrap_or_default() })),
            "patchTileCrops" => Ok(PresentationCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: id_list(&["ids", "id"]), field: text_arg(&["field"]).unwrap_or_default(), value: number_arg(&["value"]) })),
            "setSource" => Ok(PresentationCommand::SetSource(set_source::SetSource { source: decode(action, args, "source")? })),
            "setFrame" => Ok(PresentationCommand::SetFrame(set_frame::SetFrame { frame: decode(action, args, "frame")? })),
            "setActiveExample" => Ok(PresentationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text_arg(&["exampleId", "example_id", "id", "value"]).unwrap_or_else(|| crate::examples::demo::ID.into()) })),
            "clearTiles" => Ok(PresentationCommand::ClearTiles(clear_tiles::ClearTiles {})),
            "engagementSubmit" => Ok(PresentationCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: text_arg(&["value", "text"]).unwrap_or_default() })),
            "resetGrid" => Ok(PresentationCommand::ResetGrid(reset_grid::ResetGrid {})),
            "engagementInput" => Ok(PresentationCommand::EngagementInput(engagement_input::EngagementInput { value: text_arg(&["value", "text"]).unwrap_or_default() })),
            "canvasPointerDown" => Ok(PresentationCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { layer_id: text_arg(&["layerId", "layer_id", "id"]) })),
            "noMutation" => Ok(PresentationCommand::NoOperation(no_operation::NoOperation {})),
            "copyPrompt" => Ok(PresentationCommand::CopyPrompt(copy_prompt::CopyPrompt {})),
            "exportVideoFromDeck" => Ok(PresentationCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck { output_dir: text_arg(&["outputDir", "output_dir"]).unwrap_or_default(), scene_json: args.and_then(|value| value.get("scene")).map_or_else(|| "null".into(), dsl::json::to_json_string) })),
            other => Err(Fault::from(format!("presentation: unhandled action id {other}"))),
        }
    }

    fn handle(
        command: &PresentationCommand,
        doc: &ArtifactView<'_, PresentationSnapshot>,
        cfg: &ConfigView<'_, PresentationConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<PresentationMutation, PresentationConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = PresentationDispatchCtx { selected_ids: interaction.selection(PRESENTATION_INTERACTION_DOMAIN).ids.clone() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ `render(body_key, doc, cfg)` is never given an `InteractionView` (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — only `handle`/`copy_fragment`/
    /// `cut_operations` are), so the per-tile crop/name editors this panel used to build from
    /// `config.selected_ids` are gone from `inspection::render`; the client renders the tile-selected
    /// canvas highlight itself from the framework's own interaction state now (matches `🖍️draw`'s
    /// canvas render, same reason).
    fn render(body_key: &str, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        let deck = doc.snapshot;
        let labels = animate_presentation_labels(view_state);
        (match body_key {
            PRESENTATION_PLAY_BODY_MAIN => tile_editor::render(deck),
            PRESENTATION_PLAY_BODY_ARTIFACT => artifact::render(deck, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, PRESENTATION_PLAY_BODY_ARTIFACT)),
            PRESENTATION_PLAY_BODY_CATALOGUE => catalogue::render(deck, labels),
            PRESENTATION_PLAY_BODY_DETAILS => inspection::render(deck, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        })
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️AnimatePresentationPlayApp

//#region 🔖️Manifest
/// 🖼️ The four fields of `FigureTileFrame`, as the declared record shape of every `#[dsl(block)]`
/// frame argument (`setFrame.frame`, `setSource.source.frame`) — one source for both declarations so
/// the published input schema cannot drift from the struct `command_from_action` decodes into.
fn figure_tile_frame_arg_fields() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
        ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
        ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required().default_value(&1.0),
        ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required().default_value(&1.0),
    ]
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_animate_presentation_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::ANIMATE_DIALECT)
            .document(["semio", "animate"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("animate")
            .mode_def(main::definition())
            .default_mode_id(main::PRESENTATION_PLAY_MODE_MAIN)
            .window_kind_def(tile_editor::definition())
            .default_layout(main::layout())
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            .mutation("seedGrid", LocalizedLabel::native("Seed Grid", "Raster erzeugen"))
            .mutation("addTile", LocalizedLabel::native("Add Tile", "Kachel hinzufügen"))
            .mutation("deleteTile", LocalizedLabel::native("Delete Tile", "Kachel löschen"))
            .action_destructive("deleteTile")
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_destructive("deleteSelection")
            .mutation("renameTiles", LocalizedLabel::native("Rename Tiles", "Kacheln umbenennen"))
            .mutation("patchTileCrops", LocalizedLabel::native("Patch Tile Crops", "Kachelzuschnitte aktualisieren"))
            .mutation("setSource", LocalizedLabel::native("Set Source", "Quelle festlegen"))
            .mutation("setFrame", LocalizedLabel::native("Set Frame", "Rahmen festlegen"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("clearTiles", LocalizedLabel::native("Clear Tiles", "Kacheln leeren"))
            .action_destructive("clearTiles")
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            // 🐚️ Host side-effect — exports the generated tile-morph prompt to the user (no document mutation).
            .action_with(semio_framework_plugin::ActionDefinition::new("copyPrompt", LocalizedLabel::native("Copy Prompt", "Prompt kopieren"), ActionKind::Shell, "copy"))
            .action_with(semio_framework_plugin::ActionDefinition::new("exportVideoFromDeck", LocalizedLabel::native("Export Video From Deck", "Video aus Deck exportieren"), ActionKind::Shell, "download"))
            // 👁️ Ephemeral view state — engagement draft, locale. Selection/hover are framework-owned
            // now (see `.interaction(...)` below): interactionSelect/interactionHover/clearSelection/
            // selectAll/setSelectionMode/setInteractionGranularity auto-inject, never declared here
            // (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("noMutation", LocalizedLabel::native("No Operation", "Keine Aktion"))
            // 🎛️ Declared arg schemas for palette-parametric actions (materialized before dispatch).
            .action_args("seedGrid", vec![
                ActionArgDef::number("rows", LocalizedLabel::native("Rows", "Zeilen")).required().default_value(&2),
                ActionArgDef::number("columns", LocalizedLabel::native("Columns", "Spalten")).required().default_value(&2),
            ])
            // 🧱️ `setSource`/`setFrame` decode `#[dsl(block)]` payloads (`source`, `frame`) — the arg
            // ids and the record shapes are the reducer's, so an agent reading the published input
            // schema sends what `command_from_action` actually decodes.
            .action_args("setSource", vec![
                ActionArgDef::object(
                    "source",
                    LocalizedLabel::native("Source", "Quelle"),
                    vec![
                        ActionArgDef::text("src", LocalizedLabel::native("Source URL", "Quell-URL")).required(),
                        ActionArgDef::text("kind", LocalizedLabel::native("Kind", "Art")).required().default_value(&"image"),
                        ActionArgDef::object("frame", LocalizedLabel::native("Frame", "Rahmen"), figure_tile_frame_arg_fields()).required(),
                    ],
                )
                .required(),
            ])
            .action_args("setFrame", vec![
                ActionArgDef::object("frame", LocalizedLabel::native("Frame", "Rahmen"), figure_tile_frame_arg_fields()).required(),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("demo", LocalizedLabel::native("Demo", "Demo"))])
                    .required()
                    .default_value(&"demo"),
            ])
            // 🎛️ App-scope command — see `🎮️commands/🌱️seed-grid::reset_grid`'s doc comment for why this
            // isn't `seedGrid`/`clearTiles`.
            .app_command("resetGrid", LocalizedLabel::native("Reset to Default Grid", "Auf Standardraster zurücksetzen"), "document", ActionKind::Mutation)
            .action_interactive_job("seedGrid", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameTiles", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchTileCrops", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSource", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearTiles", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetGrid", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("copyPrompt", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportVideoFromDeck", InteractiveJobClassification::BatchOnlyPendingRewrite)
            // 🕹️ The framework-owned "tiles" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the document panel
            // tree (`.interaction_domain("tiles")`) and the tile-editor canvas's pick selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity, replacing the deleted bespoke `setSelectedIds` view action.
            .interaction(InteractionDefinition {
                id: PRESENTATION_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Tiles", "Kacheln"),
                granularities: vec![GranularityDefinition { id: PRESENTATION_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Tile", "Kachel"), icon_id: "square".into() }],
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
            .window_kind_interactions(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN, vec![InteractionRef::new(PRESENTATION_INTERACTION_DOMAIN)])
            .config(AnimatePresentationPlayApp::config_spec())
            .io(presentation_io())
            // 🚧️ SDK GAP (contract §2.4): no `.example(...)`/`.workflow(...)` on `EditorBuilder` — the
            // old `crate::examples::art_presentation_demo::source()` app-level example registration and the
            // no-op `.workflow("animate", "Animate", "deck")` call are dropped here (not silently:
            // reported in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::examples::...`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
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
