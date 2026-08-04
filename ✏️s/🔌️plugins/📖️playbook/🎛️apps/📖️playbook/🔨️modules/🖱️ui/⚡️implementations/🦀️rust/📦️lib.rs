//! 🧩️ Playbook-play app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the
//! pure-trait flip — `PlaybookPlayApp` is a unit struct; the former app-struct `RefCell<Vec<String>>`
//! selection now lives in `playbook_engine::PlaybookConfig`, written via
//! `playbook_op::PlaybookConfigOperation`s (real `backwards`, no ad hoc `InverseAction`); every action
//! dispatches through the single typed `playbook_protocol::PlaybookCommand` channel via
//! `DocumentApp::handle` — mirrors `writer_ui::WriterPlayApp`/`shooting_ui::ShootingPlayApp`.

use playbook::{empty_playbook_projection, flatten_playbook_blocks, PlaybookSpec, PlaybookStep, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
use playbook_engine::{default_block, playbook_io, PlaybookChapterPayload, PlaybookConfig};
use playbook_kernel::{render_playbook_builder, PlaybookBuilderConfig, PLAYBOOK_BUILDER_LABELS_EN};
use playbook_op::{add_block_operation, add_step_operation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation, PlaybookConfigOperation, PlaybookOperation};
use playbook_protocol::PlaybookCommand;
use semio_framework_plugin::{app_labels, create_default_layout, ActionArgDef, ActionArgOption};
use semio_framework_plugin::{
    ui_text, App, AppIo, ArtifactKindSpec, BlockPaletteEntry, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, SurfaceKind, UiNode,
};

//#region 🔖️Constants
const PLAYBOOK_PLAY_APP_ID: &str = "playbook-play";
const PLAYBOOK_PLAY_CONTROLLER_ID: &str = "playbook-play";
const PLAYBOOK_PLAY_SURFACE_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_BODY_BUILDER: &str = "playbook.play.builder";
const PLAYBOOK_PLAY_WINDOW_BUILDER: &str = "playbook-builder";
/// 📥️ The step `"chapters:in"` imports land in — created on first import, reused on every later one.
const PLAYBOOK_IMPORTED_STEP_ID: &str = "imported";
//#endregion 🔖️Constants

//#region 🔖️Terminology
// 🗣️ Complete UI label set for the playbook-play app; one field per label makes every locale combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
app_labels! {
    struct PlaybookPlayLabels {
        window_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        mode_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        kind_arg: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Render
fn playbook_builder_config() -> PlaybookBuilderConfig {
    PlaybookBuilderConfig { action_namespace: "playbook-builder", controller_id: PLAYBOOK_PLAY_CONTROLLER_ID, labels: PLAYBOOK_BUILDER_LABELS_EN }
}

fn builtin_palette() -> Vec<BlockPaletteEntry> {
    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| BlockPaletteEntry { block_kind: (*kind).into(), label: (*kind).into(), icon_id: "circle".into() }).collect()
}

fn render_builder(spec: &PlaybookSpec, selected_id: Option<&str>) -> UiNode {
    render_playbook_builder(PLAYBOOK_PLAY_SURFACE_BUILDER, spec, &builtin_palette(), selected_id, &playbook_builder_config())
}
//#endregion 🔖️Render

//#region 🔖️PlaybookPlayApp
/// 🧪️ B1: unit struct — the former app-struct `RefCell<Vec<String>>` selection now lives in
/// `playbook_engine::PlaybookConfig` (see `DocumentApp::Config`), written through
/// `playbook_op::PlaybookConfigOperation`s.
#[derive(Default)]
pub struct PlaybookPlayApp;

impl DocumentApp for PlaybookPlayApp {
    type Projection = PlaybookSpec;
    type Operation = PlaybookOperation;
    type Config = PlaybookConfig;
    type ConfigOperation = PlaybookConfigOperation;
    type Command = PlaybookCommand;

    fn app_id(&self) -> &str {
        PLAYBOOK_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PLAYBOOK_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> PlaybookSpec {
        empty_playbook_projection()
    }

    fn io(&self) -> Option<AppIo> {
        Some(playbook_io())
    }

    /// 🏷️ Maps each `PlaybookCommand` variant back to the action id it was declared under in
    /// `create_playbook_play_app` — used by `VcsDocumentApp` for command-log labeling and the
    /// registry's View/Shell kind-discipline check.
    fn command_id(&self, command: &PlaybookCommand) -> &str {
        match command {
            PlaybookCommand::AddStep => "addStep",
            PlaybookCommand::RemoveStep { .. } => "removeStep",
            PlaybookCommand::MoveStep { .. } => "moveStep",
            PlaybookCommand::AddBlock { .. } => "addBlock",
            PlaybookCommand::RemoveBlock { .. } => "removeBlock",
            PlaybookCommand::MoveBlock { .. } => "moveBlock",
            PlaybookCommand::UpdatePlaybook { .. } => "updatePlaybook",
            PlaybookCommand::SetSelection { .. } => "setSelection",
            PlaybookCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &PlaybookCommand, doc: &DocumentView<'_, PlaybookSpec>, cfg: &ConfigView<'_, PlaybookConfig>) -> Result<Emit<PlaybookOperation, PlaybookConfigOperation>, Fault> {
        let spec = doc.projection;
        let config = cfg.projection;
        match command {
            PlaybookCommand::AddStep => {
                let step_id = format!("step-{}", spec.steps.len() + 1);
                Ok(Emit::operations(vec![add_step_operation(spec, step_id)])
            }
            PlaybookCommand::RemoveStep { step_id } => {
                if step_id.is_empty() {
                    return Ok(Emit::default();
                }
                Ok(Emit::operations(vec![remove_step_operation(step_id)])
            }
            PlaybookCommand::MoveStep { step_id, index } => {
                if step_id.is_empty() {
                    return Ok(Emit::default();
                }
                Ok(Emit::operations(vec![move_step_operation(step_id, *index)])
            }
            PlaybookCommand::AddBlock { kind, step_id } => {
                let Some(step_id) = step_id.clone().or_else(|| spec.steps.first().map(|step| step.id.clone())) else {
                    return Ok(Emit::default();
                };
                let block_id = format!("block-{}", spec.steps.iter().map(|step| step.blocks.len()).sum::<usize>() + 1);
                Emit { document_operations: vec![add_block_operation(&step_id, default_block(block_id.clone(), kind), None)], config_operations: vec![PlaybookConfigOperation::SetSelectedIds { ids: vec![block_id] }], ..Default::default() }
            }
            PlaybookCommand::RemoveBlock { step_id, block_id } => {
                if step_id.is_empty() || block_id.is_empty() {
                    return Ok(Emit::default();
                }
                let remaining: Vec<String> = config.selected_ids.iter().filter(|id| *id != block_id).cloned().collect();
                Emit { document_operations: vec![remove_block_operation(step_id, block_id)], config_operations: vec![PlaybookConfigOperation::SetSelectedIds { ids: remaining }], ..Default::default() }
            }
            PlaybookCommand::MoveBlock { block_id, from_step_id, to_step_id, index } => Ok(Emit::operations(vec![move_block_operation(block_id, from_step_id, to_step_id, *index)]),
            PlaybookCommand::UpdatePlaybook { value } => Ok(Emit::amend(vec![update_playbook_title_operation(Some(value.clone()).filter(|title| !title.is_empty()))], "playbook.title"),
            PlaybookCommand::SetSelection { ids } => Ok(Emit::config(vec![PlaybookConfigOperation::SetSelectedIds { ids: ids.clone() }])),
            PlaybookCommand::SetLocale { value } => Ok(Emit::config(vec![PlaybookConfigOperation::SetLocale { value: value.clone() }])),
        }
    }

    /// 🎞️ `"chapters:in"` (Text×Document, `Many`) — decodes a `writer`-shaped chapter payload (see
    /// `writer_engine::WriterChapterPayload`/`playbook_engine::PlaybookChapterPayload`) and inserts it
    /// as a `"note"` block (free-form `text` field, non-interactive — see `PLAYBOOK_BUILTIN_KINDS` and
    /// `default_value_for_block`'s `"note"` arm in the kernel crate) into a dedicated `"imported"` step,
    /// created on first import and reused on every later one (idempotent step creation).
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, PlaybookSpec>) -> Result<Emit<PlaybookOperation, PlaybookConfigOperation>, MediaError> {
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
        operations.push(add_block_operation(PLAYBOOK_IMPORTED_STEP_ID, block, None));
        Ok(Emit::operations(operations))
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, PlaybookSpec>, cfg: &ConfigView<'_, PlaybookConfig>) -> UiNode {
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => render_builder(doc.projection, cfg.projection.selected_ids.first().map(String::as_str)),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaybookPlayApp

//#region 🔖️Manifest
pub fn create_playbook_play_app() -> App {
    App::from_builder(
        App::builder(PLAYBOOK_PLAY_APP_ID, LocalizedLabel::native("Playbook", "Playbook"))
            .document(["semio", "playbook"])
            .artifact_kind(ArtifactKindSpec {
                id: "text.playbook".into(),
                name: "Playbook".into(),
                source_format: PLAYBOOK_DOCUMENT_SCHEMA.into(),
                component_kind: "playbook".into(),
                dimension: "text".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
                schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .mode("builder", LocalizedLabel::native("Builder", "Builder"), "component")
            .default_mode_id("builder")
            .window_kind(PLAYBOOK_PLAY_WINDOW_BUILDER, LocalizedLabel::native("Builder", "Builder"), PLAYBOOK_PLAY_BODY_BUILDER, SurfaceKind::BlockList, "clipboard-list")
            .default_layout(create_default_layout(&[PLAYBOOK_PLAY_WINDOW_BUILDER.into()], "row", None, None))
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
                    PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value("text"),
            ])
            // 🎯️ Typed channel surface (mirrors `writer_ui::create_writer_app`'s identical wiring) —
            // `playbook_engine::playbook_io()` is the single source of truth for both the trait's
            // `io()` override and this manifest declaration.
            .config(PlaybookPlayApp.config_spec())
            .io(playbook_io()),
    )
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{
        testkit::{self, meta},
        PluginApp, VcsDocumentApp, ViewState,
    };

    fn new_app() -> VcsDocumentApp<PlaybookPlayApp> {
        testkit::new_app::<PlaybookPlayApp>()
    }

    fn new_app_with_registry() -> VcsDocumentApp<PlaybookPlayApp> {
        testkit::new_app_with_registry::<PlaybookPlayApp>(create_playbook_play_app)
    }

    #[test]
    fn add_block_materializes_declared_kind_default() {
        let mut app = new_app_with_registry();
        app.dispatch_typed(PlaybookCommand::AddStep, &meta("local")).expect("add step");
        // addBlock fired with the declared default `kind` ("text").
        app.dispatch_typed(PlaybookCommand::AddBlock { kind: "text".into(), step_id: None }, &meta("local")).expect("add block");
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.last().unwrap().kind, "text", "kind default materialized from the registry");
    }

    #[test]
    fn playbook_play_app_declares_builder_window() {
        let app = create_playbook_play_app();
        assert_eq!(app.definition.window_kinds.len(), 1);
        assert_eq!(app.definition.window_kinds[0].id, PLAYBOOK_PLAY_WINDOW_BUILDER);
        assert_eq!(app.definition.window_kinds[0].body_key, PLAYBOOK_PLAY_BODY_BUILDER);
    }

    #[test]
    fn add_step_action_grows_projection() {
        let mut app = new_app();
        app.dispatch_typed(PlaybookCommand::AddStep, &meta("local")).expect("add step");
        assert_eq!(app.projection().expect("materialize projection").steps.len(), 2);
    }

    #[test]
    fn add_block_action_appends_and_selects_block() {
        let mut app = new_app();
        let result = app.dispatch_typed(PlaybookCommand::AddBlock { kind: "text".into(), step_id: None }, &meta("local")).expect("add block");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("materialize projection");
        assert_eq!(projection.steps[0].blocks.len(), 1);
        assert_eq!(projection.steps[0].blocks[0].kind, "text");
        let node = app.render(PLAYBOOK_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&projection.steps[0].blocks[0].id));
    }

    #[test]
    fn set_selection_is_a_view_command_without_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(PlaybookCommand::SetSelection { ids: vec!["block-1".into()] }, &meta("local")).expect("set selection");
        assert!(result.operations.is_empty(), "selection is ephemeral config state, not a document operation");
    }

    #[test]
    fn render_builder_emits_playbook_list_component_scene() {
        let mut app = new_app();
        let node = app.render(PLAYBOOK_PLAY_BODY_BUILDER, None, &ViewState::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(&mut app, PlaybookCommand::AddStep, |app| app.projection().expect("materialize projection").steps.len(), 1, 2);
    }

    #[test]
    fn update_playbook_title_coalesces_into_one_undo_step() {
        let mut app = new_app();
        for title in ["R", "Re", "Recipe"] {
            app.dispatch_typed(PlaybookCommand::UpdatePlaybook { value: title.into() }, &meta("local")).expect("type title");
        }
        assert_eq!(app.projection().expect("materialize projection").title.as_deref(), Some("Recipe"));
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("materialize projection").title, None, "coalesced typing is one undo step");
    }

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations over
    /// a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<PlaybookPlayApp, (usize, usize)>("mem://playbook-convergence", PlaybookCommand::AddStep, PlaybookCommand::AddBlock { kind: "number".into(), step_id: None }, |app| {
            let projection = app.projection().expect("materialize projection");
            (projection.steps.len(), projection.steps[0].blocks.len())
        });
    }

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
        Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: serde_json::to_string(&payload).unwrap() } }
    }

    #[test]
    fn import_media_creates_the_imported_step_and_a_note_block() {
        let app = PlaybookPlayApp;
        let spec = empty_playbook_projection();
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
        let mut spec = empty_playbook_projection();
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
        let spec = empty_playbook_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = DocumentView { projection: &spec, history: &history };
        assert!(matches!(app.import_media("nonsense:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::NotImplemented)));
        let bad_media = Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: "not json".into() } };
        assert!(matches!(app.import_media("chapters:in", &bad_media, &doc_view), Err(MediaError::Payload(..))));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
