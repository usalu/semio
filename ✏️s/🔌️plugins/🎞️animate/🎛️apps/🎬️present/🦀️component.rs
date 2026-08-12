//! 🎞️ Animate present app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. B1: the pure-trait pivot — `AnimatePresentPlayApp` is a unit struct; every former
//! `AnimatePresentPlayRuntime` field (selection, engagement draft) now lives in
//! `crate::apps::present::config::PresentConfig`, written via `PresentConfigMutation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `PresentCommand` channel via `ArtifactApp::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🖊️main/🪟️windows/🖼️tile-editor`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, pure document helpers in
//! `crate::artifacts::present::schema`, and stateful behaviour (the Manim-class animation core + the
//! headless video renderer, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) in this app's
//! own `⚙️engine`. This file is a routing table: `handle` → `PresentCommand::dispatch`, `render` →
//! body-key → node, `🔖️Io`/`🔌️Registration` regions below, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::apps::present::commands::{engagement, grid, shell, source, tile, view};
use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::modes::main;
use crate::apps::present::modes::main::windows::tile_editor;
use crate::apps::present::panels::{artifact, catalogue, inspection};
use crate::apps::present::terminology::animate_present_labels;
use crate::artifacts::present::schema::build_tile_morph_prompt;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{default_present_snapshot, FigureTileDraft, PresentSnapshot, PRESENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDescriptor, App, AppIo, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, HostEffect, Label, LocalizedLabel, Media, MediaError, MediaPayload, UiNode};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashSet;

//#region 🔖️Constants
pub const PRESENT_PLAY_APP_ID: &str = "animate-present-play";
pub use catalogue::PRESENT_PLAY_BODY_CATALOGUE;
pub use artifact::PRESENT_PLAY_BODY_DOCUMENT;
pub use inspection::PRESENT_PLAY_BODY_DETAILS;
pub use tile_editor::PRESENT_PLAY_BODY_MAIN;


/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn animate_present_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PRESENT_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors `create_animate_present_app`'s `.artifact_kind(...)` literal (schema/
/// media type copied verbatim) plus the extra `frames:in` input port (Wave-2 port recipe).
pub fn present_io() -> AppIo {
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
/// `addTile` (see the app's `🎮️commands/🀄️tile`/`🎮️commands/🌐️grid`) already let a user crop/arrange
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

//#region 🔌️Registration
/// 🔌️ Relocated verbatim from the former artifact-tree `⚙️engine`'s root `component.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — called by the plugin-root `📦️glue.rs`'s
/// `semio_plugin!{}` `setup:` field (`.setup(crate::apps::present::register)`).
pub fn register() {
    crate::artifacts::present::io_registry::register();

    register_pilot_languages();
    register_artifact_schema();
    register_artifact_inferences();
    crate::apps::present::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::present::AnimatePresentPlayApp>(PRESENT_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "present.document",
        extension: Some("present"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::present::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::present::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::present::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::present::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("present.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "present.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::present::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("present.spr"),
    });
}

/// 📌️ Registers the twenty handcrafted schema leaves for `s.animate.present`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::present::schema::present_artifact_schema_descriptor());
}

/// 💡️ Registers `s.animate.present.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::present::standards::v1::subsets::any::schema::inferences::present_artifact_inference_descriptor());
}
//#endregion 🔌️Registration

//#region 🔖️Helpers
/// 🔢️ Mints a fresh, process-unique tile id — shared by `🎮️commands/🀄️tile::add_tile` and
/// `🎮️commands/⌨️engagement::engagement_submit`'s `"add"` keyword.
pub(crate) fn new_tile_id(prefix: &str) -> String {
    let serial = {
        let hex = blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex();
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{serial}")
}

/// 🧹️ Retains only the ids that reference an existing tile in `deck` — shared by every command that
/// accepts a selection/target id list.
pub(crate) fn valid_tile_ids(deck: &PresentSnapshot, ids: Vec<String>) -> Vec<String> {
    let valid: HashSet<&str> = deck.tiles.iter().map(|tile| tile.id.as_str()).collect();
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
/// landed `HostEffect` contract carries no clipboard variant, so the prompt is exported as media).
/// Shared by `🎮️commands/🐚️shell::copy_prompt` and `🎮️commands/⌨️engagement::engagement_submit`'s
/// `"copy"`/`"copy prompt"` keywords.
pub(crate) fn tile_morph_prompt_effect(deck: &PresentSnapshot) -> HostEffect {
    HostEffect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&deck.source, &deck.tiles), encoding: None }
}

/// 🔁️ Builds a `HostEffect::LoadDocument` for `document` — the sanctioned non-history "reset the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🖼️source::set_active_example` uses instead of the banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_present_document_effect(document: &PresentSnapshot) -> HostEffect {
    let pack = <PresentSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<PresentSnapshot, PresentMutation>(PRESENT_DOCUMENT_SCHEMA, "present", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("present document spr encode is infallible for a fresh, edit-free envelope");
    HostEffect::LoadDocument { pack, spr }
}
//#endregion 🔖️Helpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `AnimatePresentPlayApp::Command` — the SOLE dispatch surface for animate present's own
    /// behavior, assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest
    /// action id (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire
    /// keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// `"animate.resetGrid" as "reset-grid"` is the row that proves it (mirrors the pre-B1
    /// `handle_command`-only `"animate.resetGrid"` app-scope command). **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PresentCommand for PresentSnapshot, PresentMutation, PresentConfig, PresentConfigMutation {
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
        "animate.resetGrid" as "reset-grid" => reset_grid::ResetGrid,
        "setSelectedIds" as "set-selected-ids" => set_selected_ids::SetSelectedIds,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "setLocale" as "set-locale" => set_locale::SetLocale,
        "noMutation" as "no-op" => no_operation::NoOperation,
        "copyPrompt" as "copy-prompt" => copy_prompt::CopyPrompt,
        "exportVideoFromDeck" as "export-video-from-deck" => export_video_from_deck::ExportVideoFromDeck,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use engagement::{engagement_input, engagement_submit};
use grid::{clear_tiles, reset_grid, seed_grid};
use shell::{copy_prompt, export_video_from_deck};
use source::{set_active_example, set_frame, set_source};
use tile::{add_tile, delete_selection, delete_tile, patch_tile_crops, rename_tiles};
use view::{canvas_pointer_down, no_operation, set_locale, set_selected_ids};
//#endregion 🔖️Commands

//#region 🔖️AnimatePresentPlayApp
/// 🧪️ B1: unit struct — every former `AnimatePresentPlayRuntime` field now lives in
/// `crate::apps::present::config::PresentConfig` (see `ArtifactApp::Config`), written through
/// `PresentConfigMutation`s.
#[derive(Default)]
pub struct AnimatePresentPlayApp;

impl ArtifactApp for AnimatePresentPlayApp {
    type Snapshot = PresentSnapshot;
    type Mutation = PresentMutation;
    type Config = PresentConfig;
    type ConfigMutation = PresentConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::apps::present::presence::PresentPresence;
    type PresenceMutation = crate::apps::present::presence::PresentPresenceMutation;

    type Command = PresentCommand;

    const APP_ID: &'static str = PRESENT_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = PRESENT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PresentSnapshot {
        default_present_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(present_io())
    }

    /// 🌱️ `whole_document_operation` stays the trait default (`None`): per `📓️taxonomy.md`, whole-
    /// document replace has no in-history mutation at all (there is no import mutation by locked
    /// decision — see `🎮️commands/🖼️source::set_active_example`'s `HostEffect::LoadDocument` instead).

    /// 🎞️ `frames:in` (Wave-2 port recipe): inserts an incoming raster frame as a new tile in a
    /// deterministic contact-sheet grid (see `next_frame_tile_crop`'s doc comment below for why this
    /// schema's single shared `source` means tiles, not `source`, are the natural insertion point).
    /// Never mutates anything directly: the caller applies the returned `Tiles(Add)` through the
    /// ordinary, undoable document store.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PresentSnapshot>) -> Result<Emit<PresentMutation, PresentConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "frames:in" {
            return Err(MediaError::NotImplemented);
        }
        let deck = doc.snapshot;
        let count = deck.tiles.len();
        let id = next_frame_tile_id(count);
        let crop = next_frame_tile_crop(count);
        let name = frame_media_name(port, media)?;
        let tile = FigureTileDraft { id: id.clone(), name, crop };
        Ok(Emit::mutations(vec![PresentMutation::CreateTile(CreateTile { index: count, tile })]))
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PresentCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &PresentCommand, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<PresentMutation, PresentConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> UiNode {
        let deck = doc.snapshot;
        let config = cfg.snapshot;
        let selected = &config.selected_ids;
        let labels = animate_present_labels(config);
        match body_key {
            PRESENT_PLAY_BODY_MAIN => tile_editor::render(deck, selected),
            PRESENT_PLAY_BODY_DOCUMENT => artifact::render(deck, selected, labels),
            PRESENT_PLAY_BODY_CATALOGUE => catalogue::render(deck, labels),
            PRESENT_PLAY_BODY_DETAILS => inspection::render(deck, selected, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️AnimatePresentPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_animate_present_app() -> App {
    App::from_builder(
        App::builder(PRESENT_PLAY_APP_ID, LocalizedLabel::native("Animate Present", "Animate Present"))
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
            // 👁️ Ephemeral view state — selection, engagement draft, locale.
            .view_action("setSelectedIds", LocalizedLabel::native("Set Selected Ids", "Auswahl-IDs festlegen"))
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
            // 🎛️ App-scope command — see `🎮️commands/🌐️grid::reset_grid`'s doc comment for why this
            // isn't `seedGrid`/`clearTiles`.
            .app_command("animate.resetGrid", LocalizedLabel::native("Reset to Default Grid", "Auf Standardraster zurücksetzen"), "document")
            .config(AnimatePresentPlayApp::config_spec())
            .io(present_io()),
    )
    .example_source(crate::examples::art_present_demo::source())
    .workflow("animate", "Animate", "deck")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type PresentApp = VcsArtifactApp<AnimatePresentPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn present_app() -> PresentApp {
        new_app::<AnimatePresentPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn present_app_with_registry() -> PresentApp {
        new_app_with_registry::<AnimatePresentPlayApp>(create_animate_present_app)
    }

    pub fn dispatch(app: &mut PresentApp, command: PresentCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut PresentApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::present_app;
    use protocol::OpText;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn deck_schema_is_animate_present() {
        assert_eq!(default_present_snapshot().schema, PRESENT_DOCUMENT_SCHEMA);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }), &meta("local")).expect("seed grid");
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert!(app.snapshot().expect("projection").tiles.is_empty());
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4);
    }

    #[test]
    fn render_unknown_body_key_reports_it_by_name() {
        use semio_framework_plugin::ViewModel;
        let mut app = present_app();
        let node = app.render("some.unknown.body", None, &ViewModel::default()).expect("render unknown");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Unknown body: some.unknown.body"));
    }

    #[test]
    fn app_manifest_declares_expected_operations_and_shell_actions() {
        use semio_framework_plugin::ActionKind;
        let definition = create_animate_present_app().definition;
        let operation_ids: Vec<&str> = definition.actions.iter().filter(|action| matches!(action.kind, ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
        for expected in ["seedGrid", "addTile", "deleteTile", "deleteSelection", "renameTiles", "patchTileCrops", "setSource", "setFrame", "setActiveExample", "clearTiles", "engagementSubmit"] {
            assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
        }
        assert!(definition.actions.iter().any(|action| action.id == "exportVideoFromDeck" && matches!(action.kind, ActionKind::Shell)));
        assert!(definition.actions.iter().any(|action| action.id == "setSelectedIds" && matches!(action.kind, ActionKind::View)));
    }

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_animate_present_app().definition).expect("app definition json");
        assert!(json.contains(tile_editor::PRESENT_PLAY_WINDOW_MAIN), "window kind missing from the manifest: {json}");
        assert!(json.contains(main::PRESENT_PLAY_MODE_MAIN), "mode missing from the manifest");
        for body in [PRESENT_PLAY_BODY_DOCUMENT, PRESENT_PLAY_BODY_CATALOGUE, PRESENT_PLAY_BODY_DETAILS] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains(PRESENT_DOCUMENT_SCHEMA), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    /// 🧬️ Two independent instances start empty, apply DISJOINT edits (A adds a tile, B sets the
    /// source), and exchanging operations over a `MemoryBackbone` converges both sides to contain BOTH
    /// edits — impossible with whole-document snapshots, which would clobber one another.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        use store::MemoryBackbone;
        let mut instance_a = present_app();
        let mut instance_b = present_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://animate-present-convergence", "mem://animate-present-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: Some(crate::artifacts::present::FigureTileFrame { x: 0.0, y: 0.0, width: 0.3, height: 0.3 }) }), &meta("actor-a")).expect("a adds tile");
        let mut source = instance_b.snapshot().expect("projection").source;
        source.kind = "video".into();
        instance_b.dispatch_typed(PresentCommand::SetSource(set_source::SetSource { source }), &meta("actor-b")).expect("b sets source kind");

        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("projection");
        let projection_b = instance_b.snapshot().expect("projection");
        assert_eq!(projection_a.tiles.len(), 1, "instance A keeps its own tile");
        assert_eq!(projection_b.tiles.len(), 1, "instance B converges on A's tile");
        assert_eq!(projection_a.source.kind, "video", "instance A converges on B's source edit");
        assert_eq!(projection_b.source.kind, "video", "instance B keeps its own source edit");
    }

    //#region 🔖️PortTests
    #[test]
    fn present_io_declares_frames_in_and_document_ports() {
        let ports = AnimatePresentPlayApp::io().expect("io").all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        assert!(ports.iter().any(|port| port.id == "frames:in"));
    }

    #[test]
    fn import_media_frames_in_inserts_a_new_tile() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry();
        let before = app.snapshot().expect("projection").tiles.len();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "hero-frame", "src": "/frames/hero.png" }).to_string() } };
        app.import_media("frames:in", &media, &meta("local")).expect("import frames:in");
        let after = app.snapshot().expect("projection");
        assert_eq!(after.tiles.len(), before + 1);
        assert_eq!(after.tiles.last().expect("imported tile").name, "hero-frame");
    }

    #[test]
    fn import_media_frames_in_places_repeated_imports_in_distinct_cells() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        use serde_json::json;
        let mut app = testkit::present_app_with_registry();
        for _ in 0..2 {
            let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: json!({ "name": "frame" }).to_string() } };
            app.import_media("frames:in", &media, &meta("local")).expect("import frames:in");
        }
        let tiles = app.snapshot().expect("projection").tiles;
        assert_eq!(tiles.len(), 2);
        assert_ne!(tiles[0].crop, tiles[1].crop, "repeated imports land in distinct cells");
    }

    #[test]
    fn import_media_rejects_unknown_port() {
        use semio_framework_plugin::{Media, MediaClass, MediaForm, MediaPayload, MediaType};
        let mut app = testkit::present_app_with_registry();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: "{}".into() } };
        assert!(app.import_media("not-a-port", &media, &meta("local")).is_err());
    }

    #[test]
    fn empty_present_snapshot_has_no_tiles() {
        assert!(crate::artifacts::present::schema::empty_present_snapshot().tiles.is_empty());
    }

    /// 🌱️ Relocated from the former artifact-tree `⚙️engine`'s own tests (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `present_io`'s relocation to
    /// this file's `🔖️Io` region.
    #[test]
    fn present_io_declares_the_frames_in_port() {
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
    fn frame_import_placement_is_deterministic_and_non_overlapping() {
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
    fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 19, "every PresentCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. This is what
    /// a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keywords: [(&str, &str); 19] = [
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
            ("animate.resetGrid", "reset-grid"),
            ("setSelectedIds", "set-selected-ids"),
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
    fn optional_field_rows_keep_their_pre_migration_bytes() {
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
            PresentCommand::SetSelectedIds(set_selected_ids::SetSelectedIds { ids: vec!["t1".into()] }),
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
