//! 📖️ Playbook play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🏗️builder/🪟️windows/🏗️builder`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`,
//! shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `PlaybookCommand::dispatch`, `render` → body-key → node, plus `import_media`'s `"chapters:in"`
//! importer (an app-level `DocumentApp` trait override, not a command).

use crate::apps::playbook::commands::{block, selection, step, locale};
use crate::apps::playbook::config::{PlaybookConfig, PlaybookConfigOperation};
use crate::apps::playbook::modes::builder;
use crate::apps::playbook::modes::builder::windows::builder as builder_window;
use crate::artifacts::playbook::engine::{default_block, flatten_playbook_blocks, playbook_io, PlaybookChapterPayload};
use crate::artifacts::playbook::op::PlaybookOperation;
use crate::artifacts::playbook::{artifact_kind, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, ActionArgDef, ActionArgOption, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaError, MediaPayload, UiNode};
use store::EngineHandles;

//#region 🔖️Constants
pub const PLAYBOOK_PLAY_APP_ID: &str = "playbook-play";
pub use builder_window::PLAYBOOK_PLAY_BODY_BUILDER;
pub use builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER;

/// 📥️ The step `"chapters:in"` imports land in — created on first import, reused on every later one.
const PLAYBOOK_IMPORTED_STEP_ID: &str = "imported";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `PlaybookPlayApp::Command` — the SOLE dispatch surface for playbook's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — copied verbatim off the pre-migration
    /// `playbook_protocol::PlaybookCommand`'s `#[dsl(key)]` attributes. **Row order is the binary
    /// variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PlaybookCommand for PlaybookSpec, PlaybookOperation, PlaybookConfig, PlaybookConfigOperation {
        "addStep" as "add-step" => add_step::AddStep,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "addBlock" as "add-block" => add_block::AddBlock,
        "removeBlock" as "remove-block" => remove_block::RemoveBlock,
        "moveBlock" as "move-block" => move_block::MoveBlock,
        "updatePlaybook" as "update-playbook" => update_playbook::UpdatePlaybook,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use step::{add_step, move_step, remove_step, update_playbook};
use block::{add_block, move_block, remove_block};
use selection::set_selection;
use locale::set_locale;
//#endregion 🔖️Commands

//#region 🔖️PlaybookPlayApp
/// 🧪️ B1: unit struct — the former app-struct `RefCell<Vec<String>>` selection now lives in
/// `PlaybookConfig` (see `DocumentApp::Config`), written through `PlaybookConfigOperation`s.
#[derive(Default)]
pub struct PlaybookPlayApp;

impl DocumentApp for PlaybookPlayApp {
    type Projection = PlaybookSpec;
    type Operation = PlaybookOperation;
    type Config = PlaybookConfig;
    type ConfigOperation = PlaybookConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = PlaybookCommand;

    const APP_ID: &'static str = PLAYBOOK_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = PLAYBOOK_DOCUMENT_SCHEMA;

    fn initial_projection() -> PlaybookSpec {
        crate::artifacts::playbook::engine::empty_playbook_projection()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(playbook_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &PlaybookCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &PlaybookCommand, doc: &DocumentView<'_, PlaybookSpec>, cfg: &ConfigView<'_, PlaybookConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<PlaybookOperation, PlaybookConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"chapters:in"` (Text×Document, `Many`) — decodes a `writer`-shaped chapter payload (see
    /// `writer_engine::WriterChapterPayload`/`PlaybookChapterPayload`) and inserts it as a `"note"` block
    /// (free-form `text` field, non-interactive) into a dedicated `"imported"` step, created on first
    /// import and reused on every later one (idempotent step creation).
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, PlaybookSpec>) -> Result<Emit<PlaybookOperation, PlaybookConfigOperation, Self::DraftOperation>, MediaError> {
        if port != "chapters:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "chapters:in importer only accepts a Structured payload".into()));
        };
        let chapter: PlaybookChapterPayload = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let spec = doc.projection;
        let mut operations = Vec::new();
        if !spec.steps.iter().any(|step| step.id == PLAYBOOK_IMPORTED_STEP_ID) {
            operations.push(PlaybookOperation::AddStep { step: PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() }, index: None });
        }
        let block_id = format!("chapter-{}", flatten_playbook_blocks(spec).len() + 1);
        let mut block = default_block(block_id, "note");
        block.label = chapter.title;
        block.text = Some(chapter.text);
        operations.push(crate::artifacts::playbook::op::add_block_operation(PLAYBOOK_IMPORTED_STEP_ID, block, None));
        Ok(Emit::operations(operations))
    }

    fn render(body_key: &str, doc: &DocumentView<'_, PlaybookSpec>, cfg: &ConfigView<'_, PlaybookConfig>) -> UiNode {
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => builder_window::render(doc.projection, cfg.projection),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaybookPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_playbook_play_app() -> App {
    App::from_builder(
        App::builder(PLAYBOOK_PLAY_APP_ID, LocalizedLabel::native("Playbook", "Playbook"))
            .document(["semio", "playbook"])
            .artifact_kind(artifact_kind())
            .mode_def(builder::definition())
            .default_mode_id(builder::PLAYBOOK_PLAY_MODE_BUILDER)
            .window_kind_def(builder_window::definition())
            .default_layout(builder::layout())
            .operation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .operation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .operation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .operation("addBlock", LocalizedLabel::native("Add Block", "Baustein hinzufügen"))
            .operation("removeBlock", LocalizedLabel::native("Remove Block", "Baustein entfernen"))
            .operation("moveBlock", LocalizedLabel::native("Move Block", "Baustein verschieben"))
            .operation("updatePlaybook", LocalizedLabel::native("Update Playbook", "Playbook aktualisieren"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            // 📝️ Staged argument form for the panel-visible create action (block kind is a choice).
            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    crate::artifacts::playbook::PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value("text"),
            ])
            // 🎯️ Typed channel surface (mirrors `writer_ui::create_writer_app`'s identical wiring) —
            // `crate::artifacts::playbook::engine::playbook_io()` is the single source of truth for both
            // the trait's `io()` override and this manifest declaration.
            .config(PlaybookPlayApp::config_spec())
            .io(playbook_io()),
    )
}
//#endregion 🔖️Manifest

//#region 🔖️Setup
/// 🗂️ Called from the plugin root's `semio_plugin!{ setup: … }` — re-exported so `📦️glue.rs` names one
/// symbol instead of reaching into the artifact `⚙️engine` node directly.
pub use crate::artifacts::playbook::engine::register as setup;
//#endregion 🔖️Setup

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type PlaybookApp = VcsDocumentApp<PlaybookPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn playbook_app() -> PlaybookApp {
        new_app::<PlaybookPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
    /// `kind` default declared on `addBlock` materializes host-side.
    pub fn playbook_app_with_registry() -> PlaybookApp {
        new_app_with_registry::<PlaybookPlayApp>(create_playbook_play_app)
    }

    pub fn dispatch(app: &mut PlaybookApp, command: PlaybookCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut PlaybookApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::playbook::testkit::playbook_app;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{MediaClass, MediaForm};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 9, "every PlaybookCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id for every row except `setLocale`, preserved exactly (VERBATIM off the
    /// pre-migration `playbook_protocol::PlaybookCommand`'s own `#[dsl(key = ..)]` attribute) so the wire
    /// format stays byte-identical across the migration; see TEMPLATE.md §5.1.
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<PlaybookCommand> {
        vec![
            PlaybookCommand::AddStep(add_step::AddStep {}),
            PlaybookCommand::RemoveStep(remove_step::RemoveStep { step_id: "s".into() }),
            PlaybookCommand::MoveStep(move_step::MoveStep { step_id: "s".into(), index: 2 }),
            PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: None }),
            PlaybookCommand::RemoveBlock(remove_block::RemoveBlock { step_id: "s".into(), block_id: "b".into() }),
            PlaybookCommand::MoveBlock(move_block::MoveBlock { block_id: "b".into(), from_step_id: "s1".into(), to_step_id: "s2".into(), index: 0 }),
            PlaybookCommand::UpdatePlaybook(update_playbook::UpdatePlaybook { value: "Recipe".into() }),
            PlaybookCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] }),
            PlaybookCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_playbook_play_app().definition).expect("app definition json");
        assert!(json.contains(PLAYBOOK_PLAY_WINDOW_BUILDER), "window kind missing from the manifest: {json}");
        assert!(json.contains(builder::PLAYBOOK_PLAY_MODE_BUILDER), "mode missing from the manifest");
        assert!(json.contains("text.playbook"), "artifact kind missing from the manifest");
    }

    #[test]
    fn playbook_play_app_declares_builder_window_only() {
        let app = create_playbook_play_app();
        assert_eq!(app.definition.window_kinds.len(), 1);
        assert_eq!(app.definition.window_kinds[0].id, PLAYBOOK_PLAY_WINDOW_BUILDER);
        assert_eq!(app.definition.window_kinds[0].body_key, PLAYBOOK_PLAY_BODY_BUILDER);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = playbook_app();
        testkit::assert_undo_redo_round_trip(&mut app, PlaybookCommand::AddStep(add_step::AddStep {}), |app| app.projection().expect("materialize projection").steps.len(), 1, 2);
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::playbook::testkit::render;
        let mut app = playbook_app();
        assert!(render(&mut app, "playbook.play.nope").contains("Unknown body"));
    }

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations
    /// over a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<PlaybookPlayApp, (usize, usize)>(
            "mem://playbook-convergence",
            PlaybookCommand::AddStep(add_step::AddStep {}),
            PlaybookCommand::AddBlock(add_block::AddBlock { kind: "number".into(), step_id: None }),
            |app| {
                let projection = app.projection().expect("materialize projection");
                (projection.steps.len(), projection.steps[0].blocks.len())
            },
        );
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️PortTests
    #[test]
    fn playbook_io_declares_the_extra_chapters_in_port_and_its_own_kind() {
        let io = playbook_io();
        assert_eq!(io.artifact.id, "text.playbook");
        let ports = io.all_ports();
        let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in declared");
        assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
    }

    fn chapter_media(text: &str, title: &str) -> Media {
        let payload = PlaybookChapterPayload { id: "jack".into(), title: title.into(), text: text.into(), language_id: "jack".into() };
        Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: serde_json::to_string(&payload).unwrap() } }
    }

    #[test]
    fn import_media_creates_the_imported_step_and_a_note_block() {
        let app = PlaybookPlayApp;
        let spec = crate::artifacts::playbook::engine::empty_playbook_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &spec, history: &history };
        let media = chapter_media("MATCH (a) RETURN a", "Jack Query");
        let emit = app.import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
        assert_eq!(emit.document_operations.len(), 2, "creates the imported step, then the note block");
        assert!(matches!(&emit.document_operations[0], PlaybookOperation::AddStep { step, .. } if step.id == PLAYBOOK_IMPORTED_STEP_ID));
        match &emit.document_operations[1] {
            PlaybookOperation::AddBlock { step_id, block, .. } => {
                assert_eq!(step_id, PLAYBOOK_IMPORTED_STEP_ID);
                assert_eq!(block.kind, "note");
                assert_eq!(block.label, "Jack Query");
                assert_eq!(block.text.as_deref(), Some("MATCH (a) RETURN a"));
            }
            other => panic!("expected AddBlock, got {other:?}"),
        }
    }

    #[test]
    fn import_media_reuses_the_imported_step_on_a_second_import() {
        let app = PlaybookPlayApp;
        let mut spec = crate::artifacts::playbook::engine::empty_playbook_projection();
        spec.steps.push(PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() });
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &spec, history: &history };
        let media = chapter_media("second chapter", "Second");
        let emit = app.import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
        assert_eq!(emit.document_operations.len(), 1, "the imported step already exists, only the block is added");
        assert!(matches!(&emit.document_operations[0], PlaybookOperation::AddBlock { step_id, .. } if step_id == PLAYBOOK_IMPORTED_STEP_ID));
    }

    #[test]
    fn import_media_rejects_unknown_ports_and_malformed_payloads() {
        let app = PlaybookPlayApp;
        let spec = crate::artifacts::playbook::engine::empty_playbook_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &spec, history: &history };
        assert!(matches!(app.import_media("nonsense:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::NotImplemented)));
        let bad_media = Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: "not json".into() } };
        assert!(matches!(app.import_media("chapters:in", &bad_media, &doc_view), Err(MediaError::Payload(..))));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
