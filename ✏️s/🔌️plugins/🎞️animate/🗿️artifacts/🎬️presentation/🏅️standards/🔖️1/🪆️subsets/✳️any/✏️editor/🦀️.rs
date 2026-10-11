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
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::build_tile_morph_prompt;
use crate::{default_presentation_snapshot, FigureTileDraft, PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPublicationKind, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPublication, StepContext};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion};
use semio_framework_plugin::ArtifactReservedJob;
// 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
// `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
// `.workflow(...)` on this builder (see `🔖️Manifest` below for what got dropped, not silently).
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ComponentTree;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use std::collections::HashSet;
use semio_framework_2d::compute::EngineHandles;

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
    let targets = ids.iter().map(|id| semio_framework_pack_json::object([("granularity".to_string(), semio_framework_pack_json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)), ("id".to_string(), semio_framework_pack_json::Value::from(id.clone()))])).collect();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(targets))
}

/// 🕹️ Requests the shell to redispatch the framework-owned `interactionSelect` verb through its
/// normal action funnel — the only way `canvas-pointer-down`'s hit test can drive selection now that
/// it is framework-owned state, never a `PresentationConfigMutation` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), semio_framework_value::DslValue::String(interaction_targets_json(ids))),
            ("merge".to_string(), semio_framework_value::DslValue::String(merge.into())),
            ("method".to_string(), semio_framework_value::DslValue::String("pick".into())),
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

/// 🧱️ The concrete rows that make the tile roster `next`: one `delete-tiles` of every tile the deck holds, then one position-exact
/// `create-tile` per new tile — never a whole-roster replace.
pub(crate) fn tile_roster_mutations(deck: &PresentationSnapshot, next: Vec<FigureTileDraft>) -> Vec<PresentationMutation> {
    let (_, held) = crate::presentation_working_scene(deck);
    let cleared = (!held.is_empty()).then(|| PresentationMutation::DeleteTiles(crate::mutations::delete_tiles::DeleteTiles { ids: held.iter().map(|tile| tile.id.clone()).collect() }));
    cleared.into_iter().chain(next.into_iter().enumerate().map(|(index, tile)| PresentationMutation::CreateTile(CreateTile { index, tile }))).collect()
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
            let value = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            Ok(value.get("name").and_then(|v| v.as_str()).or_else(|| value.get("src").and_then(|v| v.as_str())).map_or_else(|| "Imported frame".into(), str::to_string))
        }
        MediaPayload::Binary { blob_hash, .. } => Ok(format!("frame-{}", &blob_hash[..blob_hash.len().min(8)])),
        MediaPayload::Intrinsic { value, .. } => Ok(value.get("name").and_then(semio_framework_value::DslValue::as_str).or_else(|| value.get("src").and_then(semio_framework_value::DslValue::as_str)).map_or_else(|| "Imported frame".into(), str::to_string)),
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
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("presentation", PRESENTATION_DOCUMENT_SCHEMA));
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
    "exportVideoFromDeck",
];
const ANIMATE_PRESENTATION_RETAINED_PAYLOAD_SCHEMA: &str = "animate.presentation.tool-command.v1";
const ANIMATE_PRESENTATION_RETAINED_RAW_BYTES: usize = 8_192;
const ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS: usize = 1;
const ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES: usize = 512;
const ANIMATE_PRESENTATION_CONFIG_BASE_BYTES: usize = 512;
const ANIMATE_PRESENTATION_CONFIG_STEP_BYTES: usize = 4_096;
/// 🀄️ The largest tile roster one retained verb may build. A roster is staged as one `delete-tiles` row
/// plus one `create-tile` row per tile, so nothing downstream bounds its length — this constant is the only ceiling.
const ANIMATE_PRESENTATION_MAXIMUM_TILES: usize = 256;
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
    ArtifactToolPublicationContract { tool_id: "exportVideoFromDeck", lanes: &[ArtifactToolPublicationLane::HostOnly] },
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
/// single `BoundedArtifactCommandWork` step, and `ArtifactRetainedCommandPayload::new` is handed
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
        PresentationCommand::ExportVideoFromDeck(payload) => bounded(payload.scene_json.len()),
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
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct AnimatePresentationConfigPreparationFactory;

struct AnimatePresentationConfigPreparation {
    owners: store::OneItemOwners<PresentationConfig, PresentationConfigMutation>,
    serialized_bytes: Option<usize>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
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
    counter.write_all(semio_framework_pack_json::to_json_string(edit).as_bytes()).map_err(|_| "Animate Presentation config edit exceeds its serialized byte envelope".to_string())?;
    Ok(counter.bytes)
}

fn animate_presentation_config_fault(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

impl store::ArtifactStoreOneItemPreparationFactory<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<PresentationConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<PresentationConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &PresentationConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            PresentationConfigMutation::SetEngagementInput(payload) => payload.value.len(),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES {
            return Err("Animate Presentation config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, ANIMATE_PRESENTATION_CONFIG_STEP_BYTES))
    }

    fn begin_demand(&self, mutation: &PresentationConfigMutation, lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        self.preflight(mutation, lane).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config preparation has no admissible lane or byte envelope"))?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<AnimatePresentationConfigPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation, PresentationConfigMutation>,
        grant: store::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation, PresentationConfigMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((animate_presentation_config_fault("Animate Presentation config preparation original request refused"), request));
        }
        Ok((Box::new(AnimatePresentationConfigPreparation {
            owners: store::OneItemOwners::from_request(request),
            serialized_bytes: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !grant.permits_one() || grant.maximum_copy_bytes < ANIMATE_PRESENTATION_CONFIG_STEP_BYTES || self.cancelled || self.owners.is_closing() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() || self.owners.failure.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        if self.owners.candidate.is_none() && self.owners.sealed.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its exact base root"))?.get();
            let base_bytes = base.engagement_input.len();
            if base_bytes > ANIMATE_PRESENTATION_CONFIG_BASE_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config base exceeds retained byte capacity"));
            }
            let mutation = self.owners.mutation.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its mutation owner"))?;
            let mut post = base.clone();
            let inverse = match &mutation {
                PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value }) => {
                    post.engagement_input = value.clone();
                    PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: base.engagement_input.clone() })
                }
            };
            *self.owners.candidate = Some((post, vec![inverse], mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, copied_bytes: base_bytes, ..Default::default() }));
        }
        if self.owners.sealed.is_none() {
            let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its candidate"))?;
            let Some(authority) = self.owners.authority.as_ref() else {
                *self.owners.candidate = Some((post, inverse, forward));
                return Err(animate_presentation_config_fault("Animate Presentation config preparation lost its Store authority"));
            };
            *self.owners.sealed = Some((post, authority.next_edit(forward, inverse)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.owners.sealed.as_ref().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its semantic edit"))?;
            let bytes = animate_presentation_config_edit_bytes(edit).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config edit exceeds its serialized byte envelope"))?;
            if bytes.saturating_add(post.engagement_input.len()).saturating_add(512) > ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config publication exceeds the 4096-byte complete envelope"));
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        let (post, edit) = self.owners.sealed.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its validated edit"))?;
        let Some(authority) = self.owners.authority.as_ref() else {
            *self.owners.sealed = Some((post, edit));
            return Err(animate_presentation_config_fault("Animate Presentation config preparation lost its Store authority"));
        };
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
        self.owners.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.owners.begin_close();
    }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
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
    port: Option<String>,
    media: Option<Media>,
    snapshot: Option<std::sync::Arc<PresentationSnapshot>>,
    history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    mutations: Option<Vec<PresentationMutation>>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<AnimatePresentationPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<AnimatePresentationPlayApp>>>,
    publication: RetainedJobPublication,
    publishing: Option<JobPublicationKind>,
    source: Vec<u8>,
    delivered: bool,
    active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl PresentationImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<AnimatePresentationPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port: Some(port),
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            publication: RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
        }
    }

    /// 🧯️ Stages one bounded fault description for the next publication turns.
    fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(JobPublicationKind::Fault);
    }

    fn decode(&mut self) -> bool {
        if self.port.as_deref() != Some(PRESENTATION_IMPORT_PORT) {
            self.stage_fault("presentation import only implements frames:in");
            return false;
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                self.stage_fault("presentation import lost its media, snapshot or history authority");
                return false;
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            AnimatePresentationPlayApp::import_media(PRESENTATION_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = Some(emit.artifact_mutations);
                self.decoded = true;
                true
            }
            Err(error) => {
                self.stage_fault(&error.to_string());
                false
            }
        }
    }

    /// ⏭️ Runs the next bounded unit of work and reports what the step must publish.
    fn advance(&mut self) -> PresentationImportAction {
        if self.pending_completion_rejection.is_some() {
            self.stage_fault("presentation import completion remains rejected");
            return PresentationImportAction::Publish;
        }
        if !self.decoded {
            if !self.decode() {
                return PresentationImportAction::Publish;
            }
            self.source = vec![1];
            self.publishing = Some(JobPublicationKind::Checkpoint { applied_progress: 1 });
            return PresentationImportAction::Publish;
        }
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let Some(completion) = self.completion.as_ref() else {
                self.stage_fault("presentation import lost its completion authority");
                return PresentationImportAction::Publish;
            };
            if !completion.has_mounted_consumer() {
                self.stage_fault("presentation import completion consumer is absent");
                return PresentationImportAction::Publish;
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                self.stage_fault(&message);
                return PresentationImportAction::Publish;
            }
            self.completed = true;
        }
        PresentationImportAction::Complete
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let nested = |mut demand: semio_framework_value::RetirementDemand| -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "presentation import close depth overflow"))?; Ok(demand) };
        if !self.publication.terminal_is_empty() { return self.publication.retirement_demands(); }
        if let Some(active) = self.active.as_ref() { return nested(store::artifact_retirement_box_demands(active, body)?); }
        if self.pending_completion_rejection.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection)?); }
        if self.mutations.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.mutations)?); }
        if self.media.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.media)?); }
        if self.port.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.port)?); }
        if self.history.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.history)?); }
        if self.snapshot.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.snapshot)?); }
        if self.completion.is_some() { return nested(store::artifact_retirement_owned_birth_demands(&self.completion)?); }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

enum PresentationImportAction {
    Publish,
    Complete,
}

impl InteractiveJob for PresentationImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.delivered {
            let step = self.publication.close_step(cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.delivered = false;
            }
            return Ok(None);
        }
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.publishing.is_none() {
            cx.set_stage(if self.decoded { "presentation-import-publish" } else { "presentation-import-decode" });
            if matches!(self.advance(), PresentationImportAction::Complete) {
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            cx.consume_fuel(1);
            return Ok(None);
        }
        let Some(kind) = self.publishing else { return Ok(None) };
        let result = self.publication.advance_from_source(kind, &self.source, cx)?;
        if result.is_some() {
            self.delivered = true;
            if matches!(kind, JobPublicationKind::Checkpoint { .. }) {
                self.publishing = None;
            }
        }
        Ok(result)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let refused = |error: semio_framework_value::ValueError| InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() };
        let demand = match self.close_demands(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => return refused(error) };
        if self.terminal_is_empty() { return InteractiveJobCloseStep::Complete { progress: Default::default() }; }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return InteractiveJobCloseStep::Pending { progress: Default::default() }; }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        macro_rules! owned {
            ($slot:expr) => {
                if $slot.is_some() { return match store::artifact_retirement_admit_owned(&mut $slot, &mut self.active, child) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => refused(error) }; }
            };
        }
        if !self.publication.terminal_is_empty() { return match self.publication.close_step(grant) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => refused(error) }; }
        if self.delivered { self.delivered = false; return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }; }
        if self.active.is_some() { return match store::artifact_retirement_box_close_step(&mut self.active, child) { Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() }, Err(error) => refused(error) }; }
        owned!(self.pending_completion_rejection);
        owned!(self.mutations);
        owned!(self.media);
        owned!(self.port);
        owned!(self.history);
        owned!(self.snapshot);
        owned!(self.completion);
        InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && !self.delivered
            && self.publication.terminal_is_empty()
            && self.active.is_none()
            && self.port.is_none()
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}

impl ArtifactReservedJob for PresentationImportJob {}
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

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_presentation_child_pack(snapshot, slot, child_id)
    
})())
}

    /// 🧺️ Without these owners the document store holds no `initial_snapshot_retirement_factory`, so
    /// the boot `setActiveExample` archive load is refused with `module.vcs: validation failed:
    /// returned snapshot read requires its exact owned-snapshot retirement factory` — measured
    /// against the live React playground.

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ Exact one-item PUBLICATION authority for the DOCUMENT store. Without it every tool
    /// declaring the `artifact` lane is marked `unsupported-publication-contract` at boot and fails
    /// closed with `interactive-job.publication-authority-missing`, which is why this app could own
    /// no retained document verb at all (B1a: "promoting a document verb is a two-part change").
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(AnimatePresentationConfigPreparationFactory))
    }

    /// 🧹️ Every store an instance owns closes through its bounded disposer before drop; without these the
    /// instance close faults `interactive-job.close-owned-disposer-missing` at the config-store lane.

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
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
            "copyPrompt",
            "exportVideoFromDeck"
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
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "animate-presentation-command-tool-mismatch"));
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
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
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
        );
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
        Some(crate::host::owned::presentation_envelope_decode_owner_bundle())
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::animate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> PresentationSnapshot {
        default_presentation_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(presentation_io())
    }

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
    /// `semio_framework_value::FromValue::from_value`, the same codec the typed channel uses.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<PresentationCommand, Fault> {
        let text_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string));
        let number_arg = |keys: &[&str]| keys.iter().find_map(|key| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64)).unwrap_or_default();
        let id_list = |keys: &[&str]| match keys.iter().find_map(|key| args.and_then(|value| value.get(key))) {
            Some(semio_framework_value::DslValue::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
            Some(semio_framework_value::DslValue::String(raw)) if !raw.is_empty() => vec![raw.clone()],
            _ => Vec::new(),
        };
        fn decode<T: semio_framework_value::FromValue>(action: &str, args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<T, Fault> {
            let value = args.and_then(|value| value.get(key)).cloned().ok_or_else(|| Fault::from(format!("presentation {action} requires a '{key}' block")))?;
            semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid presentation {action} '{key}': {error}")))
        }
        match action {
            "seedGrid" => Ok(PresentationCommand::SeedGrid(seed_grid::SeedGrid { rows: number_arg(&["rows"]) as u32, columns: number_arg(&["columns", "cols"]) as u32 })),
            "addTile" => Ok(PresentationCommand::AddTile(add_tile::AddTile { crop: args.and_then(|value| value.get("crop")).cloned().map(semio_framework_value::FromValue::from_value).transpose().map_err(|error| Fault::from(format!("invalid presentation addTile 'crop': {error}")))? })),
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
            "exportVideoFromDeck" => Ok(PresentationCommand::ExportVideoFromDeck(export_video_from_deck::ExportVideoFromDeck {
                scene_json: match args.and_then(|value| value.get("scene")) {
                    Some(semio_framework_value::DslValue::String(text)) => text.clone(),
                    Some(semio_framework_value::DslValue::Null) | None => String::new(),
                    Some(value) => semio_framework_pack_json::to_json_string(value),
                },
            })),
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
            .action_args("exportVideoFromDeck", vec![ActionArgDef::text("scene", LocalizedLabel::native("Presentation Scene (JSON)", "Präsentationsszene (JSON)"))])
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
            .action_destructive("resetGrid")
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("copyPrompt", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportVideoFromDeck", InteractiveJobClassification::Migrated)
            .action_destructive("exportVideoFromDeck")
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
            .action_describe("seedGrid", LocalizedLabel::native("Replaces every tile of the deck with a fresh grid of the given rows and columns cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch ein neues Raster mit den angegebenen Zeilen und Spalten aus dem Quellbild; die bisherigen Kacheln werden verworfen."))
            .action_describe("addTile", LocalizedLabel::native("Adds one new tile cropping the given region of the source image (a small square near the top left when none is given) and selects it.", "Fügt eine neue Kachel hinzu, die den angegebenen Bereich des Quellbilds ausschneidet (ohne Angabe ein kleines Quadrat oben links), und wählt sie aus."))
            .action_describe("deleteTile", LocalizedLabel::native("Deletes one tile by id from the deck; its crop and name are gone unless the edit is undone.", "Löscht eine Kachel anhand ihrer Id aus dem Deck; Zuschnitt und Name sind fort, sofern die Änderung nicht rückgängig gemacht wird."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every currently selected tile from the deck.", "Löscht alle aktuell ausgewählten Kacheln aus dem Deck."))
            .action_describe("renameTiles", LocalizedLabel::native("Gives every tile with the given ids the same new name; an empty name changes nothing.", "Gibt allen Kacheln mit den angegebenen Ids denselben neuen Namen; ein leerer Name ändert nichts."))
            .action_describe("patchTileCrops", LocalizedLabel::native("Sets one crop coordinate (x, y, width or height, as a fraction of the source image) on every tile with the given ids.", "Setzt eine Zuschnittkoordinate (x, y, Breite oder Höhe, als Anteil des Quellbilds) auf allen Kacheln mit den angegebenen Ids."))
            .action_describe("setSource", LocalizedLabel::native("Sets the source image the tiles are cut from; choosing a different image discards every existing tile.", "Legt das Quellbild fest, aus dem die Kacheln geschnitten werden; ein anderes Bild verwirft alle vorhandenen Kacheln."))
            .action_describe("setFrame", LocalizedLabel::native("Moves and resizes the frame the source image is shown in within the presentation (x, y, width, height).", "Verschiebt und skaliert den Rahmen, in dem das Quellbild in der Präsentation erscheint (x, y, Breite, Höhe)."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole presentation with the bundled demo deck; any other example id changes nothing.", "Ersetzt die gesamte Präsentation durch das mitgelieferte Demo-Deck; jede andere Beispiel-Id ändert nichts."))
            .action_describe("clearTiles", LocalizedLabel::native("Removes every tile from the deck and keeps only the source image.", "Entfernt alle Kacheln aus dem Deck und behält nur das Quellbild."))
            .action_describe("copyPrompt", LocalizedLabel::native("Writes a tile-morph prompt describing the source image and every tile to a downloaded tile-morph-prompt.md file on the user's machine.", "Schreibt einen Tile-Morph-Prompt, der das Quellbild und jede Kachel beschreibt, in eine heruntergeladene Datei tile-morph-prompt.md auf dem Rechner des Nutzers."))
            .action_describe("exportVideoFromDeck", LocalizedLabel::native("Renders the deck (one slide per tile), or the presentation scene given as JSON in scene, to an H.264 MP4 video on this device and downloads it; its progress and a cancel control are in the Task Manager.", "Rendert das Deck (eine Folie pro Kachel) oder die als JSON in scene übergebene Präsentationsszene auf diesem Gerät als H.264-MP4-Video und lädt es herunter; Fortschritt und Abbruch stehen im Task-Manager."))
            .action_describe("resetGrid", LocalizedLabel::native("Replaces every tile of the deck with the default 3 by 5 grid cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch das Standardraster von 3 mal 5 aus dem Quellbild; die bisherigen Kacheln werden verworfen."))
            .action_audience("noMutation", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("seedGrid")
            .action_destructive("setSource")
            .action_destructive("copyPrompt")
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
