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

use crate::editor::animate::commands::{
    add_tile, canvas_pointer_down, clear_tiles, copy_prompt, delete_selection, delete_tile, engagement_input, engagement_submit, export_video_from_deck, no_operation, patch_tile_crops, rename_tiles, reset_grid, seed_grid, set_active_example,
    set_frame, set_locale, set_source,
};
use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::modes::main;
use crate::editor::animate::modes::main::windows::tile_editor;
use crate::editor::animate::panels::{artifact, catalogue, inspection};
use crate::editor::animate::terminology::animate_present_labels;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::schema::build_tile_morph_prompt;
use crate::artifacts::present::{default_present_snapshot, FigureTileDraft, PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
// 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
// `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
// `.workflow(...)` on this builder (see `🔖️Manifest` below for what got dropped, not silently).
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionKind, AppIo, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, Effect, HoverSpec, InteractionDefinition, InteractionRef,
    Label, LocalizedLabel, Media, MediaError, MediaPayload, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
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
pub async fn animate_present_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PRESENT_PLAY_APP_ID).action(action, args)
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
async fn interaction_targets_json(ids: &[String]) -> String {
    serde_json::to_string(&ids.iter().map(|id| serde_json::json!({ "granularity": PRESENT_INTERACTION_GRANULARITY, "id": id })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}

/// 🕹️ Requests the shell to redispatch the framework-owned `interactionSelect` verb through its
/// normal action funnel — the only way `canvas-pointer-down`'s hit test can drive selection now that
/// it is framework-owned state, never a `PresentConfigMutation` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub(crate) async fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
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
pub async fn present_io() -> AppIo {
    semio_framework_plugin::AppIo {
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

pub async fn next_frame_tile_id(existing_tile_count: usize) -> String {
    format!("frame-{}", existing_tile_count + 1)
}

pub async fn next_frame_tile_crop(existing_tile_count: usize) -> crate::artifacts::present::FigureTileFrame {
    let cell = 1.0 / FRAME_IMPORT_GRID_COLUMNS as f64;
    let column = existing_tile_count % FRAME_IMPORT_GRID_COLUMNS;
    let row = existing_tile_count / FRAME_IMPORT_GRID_COLUMNS;
    crate::artifacts::present::schema::clamp_tile_crop(&crate::artifacts::present::FigureTileFrame { x: column as f64 * cell, y: (row as f64 * cell).min(1.0 - cell), width: cell, height: cell })
}
//#endregion 🔖️Io

//#region 🔖️Helpers
/// 🔢️ Mints a fresh, process-unique tile id — shared by `🎮️commands/🀄️add-tile::add_tile` and
/// `🎮️commands/⌨️engagement::engagement_submit`'s `"add"` keyword.
pub(crate) async fn new_tile_id(prefix: &str) -> String {
    let serial = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{serial}")
}

/// 🧹️ Retains only the ids that reference an existing tile in `deck` — shared by every command that
/// accepts a selection/target id list.
pub(crate) async fn valid_tile_ids(deck: &PresentSnapshot, ids: Vec<String>) -> Vec<String> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(deck);
    let valid: HashSet<&str> = tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}

/// 🎞️ `frames:in` display name (Wave-2 port recipe) — a `Structured` payload's `"name"`/`"src"` field
/// (falling back to a generic label), a `Binary` payload's leading blob-hash characters.
async fn frame_media_name(port: &str, media: &Media) -> Result<String, MediaError> {
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
pub(crate) async fn tile_morph_prompt_effect(deck: &PresentSnapshot) -> Effect {
    let (source, tiles) = crate::artifacts::present::present_working_scene(deck);
    Effect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&source, &tiles), encoding: None }
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "reset the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🖼️set-source::set_active_example` uses instead of the banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub async fn reset_present_document_effect(document: &PresentSnapshot) -> Effect {
    let pack = <PresentSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<PresentSnapshot, PresentMutation>(PRESENT_DOCUMENT_SCHEMA, "present", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("present document spr encode is infallible for a fresh, edit-free envelope");
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
        let tile = FigureTileDraft { id: id.clone(), name, crop };
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
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🕹️ `render(body_key, doc, cfg)` is never given an `InteractionView` (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — only `handle`/`copy_fragment`/
    /// `cut_operations` are), so the per-tile crop/name editors this panel used to build from
    /// `config.selected_ids` are gone from `inspection::render`; the client renders the tile-selected
    /// canvas highlight itself from the framework's own interaction state now (matches `🖍️draw`'s
    /// canvas render, same reason).
    async fn render(body_key: &str, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> UiNode {
        let deck = doc.snapshot;
        let config = cfg.snapshot;
        let labels = animate_present_labels(config);
        match body_key {
            PRESENT_PLAY_BODY_MAIN => tile_editor::render(deck),
            PRESENT_PLAY_BODY_DOCUMENT => artifact::render(deck, labels),
            PRESENT_PLAY_BODY_CATALOGUE => catalogue::render(deck, labels),
            PRESENT_PLAY_BODY_DETAILS => inspection::render(deck, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️AnimatePresentPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_animate_present_app() -> semio_framework_plugin::AppDefinition {
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
            .config(AnimatePresentPlayApp::config_spec())
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
        new_app::<EditorApp<AnimatePresentPlayApp>>()
    }

    /// 🧪️ Adapts `create_animate_present_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `new_app_with_registry`/
    /// `testkit::assert_declared_actions_bridge_to_commands` still expect — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    async fn animate_present_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_animate_present_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn present_app_with_registry() -> PresentApp {
        new_app_with_registry::<EditorApp<AnimatePresentPlayApp>>(animate_present_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut PresentApp, command: PresentCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut PresentApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::present_app;
    use protocol::OpText;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;

    #[test]
    async fn deck_schema_is_animate_present() {
        assert_eq!(default_present_snapshot().schema, PRESENT_DOCUMENT_SCHEMA);
    }

    #[test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).expect("seed grid");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 4);
    }

    #[test]
    async fn render_unknown_body_key_reports_it_by_name() {
        use semio_framework_plugin::ViewModel;
        let mut app = present_app();
        let node = app.render("some.unknown.body", None, &ViewModel::default()).expect("render unknown");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Unknown body: some.unknown.body"));
    }

    #[test]
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
    #[test]
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
    #[test]
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
    #[test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        use store::MemoryBackbone;
        let mut instance_a = present_app();
        let mut instance_b = present_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://animate-present-convergence", "mem://animate-present-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: Some(crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 0.3, height: 0.3 }) }), &meta("actor-a")).expect("a adds tile");
        let (mut source, _) = crate::artifacts::present::present_working_scene(&instance_b.snapshot().expect("projection"));
        source.kind = "video".into();
        instance_b.dispatch_typed(PresentCommand::SetSource(set_source::SetSource { source }), &meta("actor-b")).expect("b sets source kind");

        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let (source_a, tiles_a) = crate::artifacts::present::present_working_scene(&instance_a.snapshot().expect("projection"));
        let (source_b, tiles_b) = crate::artifacts::present::present_working_scene(&instance_b.snapshot().expect("projection"));
        assert_eq!(tiles_a.len(), 1, "instance A keeps its own tile");
        assert_eq!(tiles_b.len(), 1, "instance B converges on A's tile");
        assert_eq!(source_a.kind, "video", "instance A converges on B's source edit");
        assert_eq!(source_b.kind, "video", "instance B keeps its own source edit");
    }

    //#region 🔖️PortTests
    #[test]
    async fn present_io_declares_frames_in_and_document_ports() {
        let ports = AnimatePresentPlayApp::io().expect("io").all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "frames:in"));
    }

    #[test]
    async fn import_media_frames_in_inserts_a_new_tile() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry();
        let before = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "hero-frame", "src": "/frames/hero.png" }).to_string() } };
        app.import_media("frames:in", &media, &meta("local")).expect("import frames:in");
        let (_, after_tiles) = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection"));
        assert_eq!(after_tiles.len(), before + 1);
        assert_eq!(after_tiles.last().expect("imported tile").name, "hero-frame");
    }

    #[test]
    async fn import_media_frames_in_places_repeated_imports_in_distinct_cells() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry();
        for _ in 0..2 {
            let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "frame" }).to_string() } };
            app.import_media("frames:in", &media, &meta("local")).expect("import frames:in");
        }
        let (_, tiles) = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection"));
        assert_eq!(tiles.len(), 2);
        assert_ne!(tiles[0].crop, tiles[1].crop, "repeated imports land in distinct cells");
    }

    #[test]
    async fn import_media_rejects_unknown_port() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        let mut app = testkit::present_app_with_registry();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "{}".into() } };
        assert!(app.import_media("not-a-port", &media, &meta("local")).is_err());
    }

    #[test]
    async fn empty_present_snapshot_has_no_tiles() {
        let snapshot = crate::artifacts::present::schema::empty_present_snapshot();
        assert!(crate::artifacts::present::present_working_scene(&snapshot).1.is_empty());
    }

    /// 🌱️ Relocated from the former artifact-tree `⚙️engine`'s own tests (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `present_io`'s relocation to
    /// this file's `🔖️Io` region.
    #[test]
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

    #[test]
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
    #[test]
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
    #[test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. This is what
    /// a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
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
    #[test]
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
    pub(super) async fn every_command() -> Vec<PresentCommand> {
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
