//! ✒️ Writer play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/*/🪟️windows/*`, chrome measures in that window's `🎚️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the
//! artifact's `⚙️engine`. This file is a routing table: `handle` → `WriterCommand::dispatch`, `render` →
//! body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::writer::commands::camera::set_camera;
use crate::apps::writer::commands::editor_settings::{set_font_px, set_line_height, set_tab_size, toggle_line_numbers};
use crate::apps::writer::commands::engagement::{engagement_input, engagement_submit};
use crate::apps::writer::commands::inspect::{lint_document, request_completions};
use crate::apps::writer::commands::locale::set_locale;
use crate::apps::writer::commands::selection::{select_ast_node, set_ast_hover, set_ast_selection, set_editor_selection, text_hover, text_select};
use crate::apps::writer::commands::text::{commit_rename, format_document, open_document, set_active_example, set_snapshot, set_snapshot_json, set_fixture_json, set_text, text_edit};
use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::apps::writer::presence::{WriterPresence, WriterPresenceMutation};
use crate::apps::writer::modes::edit;
use crate::apps::writer::modes::edit::windows::main;
use crate::apps::writer::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::writer::terminology::writer_play_labels;
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, App, AppActionRegistry, AppIo, ConfigView, ContextMenuItemSpec, ContextMenuRequest, ContextMenuTextContext, ArtifactApp, ArtifactView, Emit, Fault, Label,
    LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, UiNode, WindowMeasure,
};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;
use store::ArtifactPack;

//#region 🔖️Constants
pub const WRITER_PLAY_APP_ID: &str = "writer-play";
pub use main::{WRITER_PLAY_BODY_MAIN, WRITER_PLAY_WINDOW_KIND};
pub use catalogue_panel::WRITER_PLAY_BODY_CATALOGUE;
pub use document_panel::WRITER_PLAY_BODY_ARTIFACT;
pub use inspection_panel::WRITER_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn writer_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionFactory::new(WRITER_PLAY_APP_ID).action(action, args)
}

/// 🙈️ An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
fn writer_hidden_operation(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️ An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
fn writer_hidden_view(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 🐁️ (highlighted AST id, tree-hover span, hover occurrences) — the tuple [`editor_hover_context`] resolves.
type HoverContext = (Option<String>, Option<(usize, usize)>, Vec<(usize, usize)>);

/// 🐁️ Resolves tree/editor hover cross-highlighting. Lives at APP level, not the artifact's `⚙️engine`,
/// even though it has two consumers (the main window and the document panel) — it takes `WriterConfig`,
/// an app-only view-state type, and artifacts must never depend on apps.
pub fn editor_hover_context(document: &WriterSnapshot, config: &WriterConfig) -> HoverContext {
    use crate::artifacts::writer::engine::{find_deepest_jack_ast_node_at, jack_ast_node_by_id, jack_symbol_at_offset, parse_jack_ast, JackSymbolKind};

    if document.language_id != "jack" {
        return (None, None, Vec::new());
    }
    let root = parse_jack_ast(&document.text);
    let tree_span = config.tree_hovered_ast_id.as_ref().and_then(|id| jack_ast_node_by_id(&root, id)).map(|node| (node.start, node.end));
    let editor_hovered_ast_id = config.editor_hover_offset.and_then(|offset| find_deepest_jack_ast_node_at(&root, offset)).map(|node| node.id.clone());
    let highlighted = config.tree_hovered_ast_id.clone().or(editor_hovered_ast_id);
    let hover_occurrences = config.editor_hover_offset.and_then(|offset| jack_symbol_at_offset(&document.text, offset)).filter(|symbol| symbol.kind == JackSymbolKind::Variable).map(|symbol| symbol.occurrences).unwrap_or_default();
    (highlighted, tree_span, hover_occurrences)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `WriterPlayApp::Command` — the SOLE dispatch surface for writer's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies.
    /// `setLocale`/`locale` is the row that proves it; `setEditorSetting` is declared by THREE rows
    /// (font/line-height/tab-size) sharing one manifest action id but three distinct wire keys and
    /// payload types — mirrors the pre-migration `WriterCommand::command_id()` match arm that mapped all
    /// three variants to the same `"setEditorSetting"` string. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WriterCommand for WriterSnapshot, WriterMutation, WriterConfig, WriterConfigMutation {
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "setText" as "set-text" => set_text::SetText,
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "openDocument" as "open-document" => open_document::OpenDocument,
        "setSnapshotJson" as "document-json" => set_snapshot_json::SetSnapshotJson,
        "setFixtureJson" as "fixture-json" => set_fixture_json::SetFixtureJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "formatDocument" as "format-document" => format_document::FormatDocument,
        "commitRename" as "commit-rename" => commit_rename::CommitRename,
        "setCamera" as "camera" => set_camera::SetCamera,
        "requestCompletions" as "request-completions" => request_completions::RequestCompletions,
        "lintDocument" as "lint-document" => lint_document::LintDocument,
        "textSelect" as "text-select" => text_select::TextSelect,
        "setEditorSelection" as "editor-selection" => set_editor_selection::SetEditorSelection,
        "selectAstNode" as "select-ast-node" => select_ast_node::SelectAstNode,
        "setAstSelection" as "ast-selection" => set_ast_selection::SetAstSelection,
        "setAstHover" as "ast-hover" => set_ast_hover::SetAstHover,
        "textHover" as "text-hover" => text_hover::TextHover,
        "toggleLineNumbers" as "toggle-line-numbers" => toggle_line_numbers::ToggleLineNumbers,
        "setEditorSetting" as "font-px" => set_font_px::SetFontPx,
        "setEditorSetting" as "line-height" => set_line_height::SetLineHeight,
        "setEditorSetting" as "tab-size" => set_tab_size::SetTabSize,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
/// 🖱️ On-demand writer text-editor context menu from caret/selection/completions context — grouped/
/// progressively disclosed (GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS): a handful of top-level
/// verbs plus taxonomy groups for the rest, with the destructive `cut` row kept trailing. `selectToken`/
/// `selectLine`/`selectAll`/`cut`/`copy`/`paste` are not declared `ActionDefinition`s (no palette/undo
/// entry makes sense for them), so they stay bespoke `.item(...)` rows per `Menu::of`'s escape hatch;
/// `requestCompletions`/`lintDocument`/`formatDocument`/`commitRename` are declared actions and resolve
/// through `.action(...)` against `registry`.
fn writer_context_menu_items(registry: &AppActionRegistry, text: Option<&ContextMenuTextContext>, is_de: bool) -> Vec<ContextMenuItemSpec> {
    let can_suggest = text.is_some_and(|t| t.has_completions);
    let has_selection = text.is_some_and(|t| t.has_selection);
    let can_rename = text.is_some_and(|t| t.can_rename);
    let bespoke = |id: &str, label: &str, icon: &str, action: &str, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    Menu::of(registry)
        .item(bespoke("writer-select-token", if is_de { "Token auswählen" } else { "Select token" }, "text-cursor", "selectToken", false))
        .item(bespoke("writer-copy", if is_de { "Kopieren" } else { "Copy" }, "copy", "copy", !has_selection))
        .item(bespoke("writer-paste", if is_de { "Einfügen" } else { "Paste" }, "clipboard", "paste", false))
        .group("selection", |m| {
            m.item(bespoke("writer-select-line", if is_de { "Zeile auswählen" } else { "Select line" }, "list-ordered", "selectLine", false)).item(bespoke(
                "writer-select-all",
                if is_de { "Alles auswählen" } else { "Select All" },
                "select-all",
                "selectAll",
                false,
            ))
        })
        .group("tools", |m| {
            let m = m.action("lintDocument");
            if can_suggest {
                m.action("requestCompletions")
            } else {
                m
            }
        })
        .group("transform", |m| {
            let m = m.action("formatDocument");
            if can_rename {
                m.action("commitRename")
            } else {
                m
            }
        })
        .item(ContextMenuItemSpec { destructive: Some(true), ..bespoke("writer-cut", if is_de { "Ausschneiden" } else { "Cut" }, "scissors", "cut", !has_selection) })
        .build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️WriterPlayApp
/// 🧪️ B1: unit struct — every former `WriterPlayRuntime` field now lives in [`WriterConfig`], written
/// through [`WriterConfigMutation`]s.
#[derive(Default)]
pub struct WriterPlayApp;

impl ArtifactApp for WriterPlayApp {
    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Config = WriterConfig;
    type ConfigMutation = WriterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = WriterPresence;
    type PresenceMutation = WriterPresenceMutation;

    type Command = WriterCommand;

    const APP_ID: &'static str = WRITER_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::engine::empty_writer_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(crate::artifacts::writer::engine::writer_io())
    }

    fn whole_document_operation(snapshot: WriterSnapshot) -> Option<WriterMutation> {
        Some(WriterMutation::SetSnapshot { snapshot })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &WriterCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &WriterCommand, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<WriterMutation, WriterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"text:out"` exports the writer document's current text as one "chapter" payload (see
    /// `crate::artifacts::writer::engine::writer_chapter_payload`) — `playbook`'s `"chapters:in"` is the
    /// intended consumer. Falls through to the default whole-document-pack export for `"document:out"`
    /// (duplicated inline, not delegated — Rust traits have no `super` call for an overridden default).
    fn export_media(port: &str, doc: &ArtifactView<'_, WriterSnapshot>) -> Result<Media, MediaError> {
        if port == "text:out" {
            let payload = crate::artifacts::writer::engine::writer_chapter_payload(doc.snapshot);
            let json = serde_json::to_string(&payload).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json } });
        }
        if port != "document:out" {
            return Err(MediaError::NotImplemented);
        }
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = writer_play_labels(config);
        match body_key {
            WRITER_PLAY_BODY_MAIN => main::render(document, config),
            WRITER_PLAY_BODY_ARTIFACT => document_panel::render(document, config, labels),
            WRITER_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            WRITER_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        use semio_framework_plugin::{WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus};

        let config = cfg.snapshot;
        let labels = writer_play_labels(config);
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: Some(vec![WindowEngagementOption {
                id: "writer-line-numbers".into(),
                label: Some(labels.line_numbers.into()),
                icon_id: Some("list-ordered".into()),
                pressed: Some(config.editor_settings.show_line_numbers),
                disabled: None,
                action: Some(writer_action("toggleLineNumbers", None)),
            }]),
            input: Some(WindowEngagementInput {
                id: Some("writer-engagement-input".into()),
                value: Some(config.engagement_input.clone()),
                placeholder: Some(labels.engagement_placeholder.into()),
                disabled: None,
                on_change: Some(writer_action("engagementInput", None)),
                on_submit: Some(writer_action("engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "writer-editor-mode".into(), text: labels.editor_mode_status.into() }]),
            possible_engagements: Some(vec![
                WindowEngagementPossible { id: "writer-format".into(), label: labels.format.into(), detail: None, action: Some(writer_action("formatDocument", None)) },
                WindowEngagementPossible { id: "writer-lint".into(), label: labels.lint.into(), detail: None, action: Some(writer_action("lintDocument", None)) },
                WindowEngagementPossible { id: "writer-line-numbers".into(), label: labels.line_numbers.into(), detail: None, action: Some(writer_action("toggleLineNumbers", None)) },
            ]),
        };
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), main::window_measures(config, writer_play_labels(config)))])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let is_de = cfg.snapshot.locale.starts_with("de");
        let text = request.surface.as_ref().and_then(|surface| surface.text.as_ref());
        writer_context_menu_items(registry, text, is_de)
    }
}
//#endregion 🔖️WriterPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, LocalizedLabel::native("Writer", "Writer"))
            .document(["semio", "writer"])
            .artifact_kind(crate::artifacts::writer::artifact_kind())
            .icon_id("writer")
            .mode_def(edit::definition())
            .default_mode_id(edit::WRITER_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 🔧️ Panel-visible P0 effects: format rewrites the buffer (Mutation), lint re-runs
            // diagnostics into runtime (View — an effect, not a document operation). Categorized for
            // `Menu::group`'s ribbon-parent taxonomy (GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS).
            .action_with(ActionDefinition::new_catalog("formatDocument", LocalizedLabel::native("Format Document", "Dokument formatieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::new_catalog("lintDocument", LocalizedLabel::native("Lint Document", "Dokument prüfen"), ActionKind::View).with_category("tools"))
            // 🔧️ P1 example switch (whole-document load) with a staged example choice.
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🙈️ Internal document operations — text edits (coalesced), aliases, camera, rename, engagement,
            // and dev-only whole-document JSON setters.
            .action_with(writer_hidden_operation("textEdit", LocalizedLabel::native("Edit Text", "Text bearbeiten")))
            .action_with(writer_hidden_operation("setText", LocalizedLabel::native("Set Text", "Text festlegen")))
            .action_with(writer_hidden_view("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen")))
            .action_with(writer_hidden_operation("commitRename", LocalizedLabel::native("Commit Rename", "Umbenennung übernehmen")).with_category("transform"))
            .action_with(writer_hidden_operation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen")))
            .action_with(writer_hidden_operation("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen")))
            .action_with(writer_hidden_operation("setSnapshotJson", LocalizedLabel::native("Set Document JSON", "Dokument-JSON festlegen")))
            .action_with(writer_hidden_operation("setFixtureJson", LocalizedLabel::native("Set Fixture JSON", "Fixture-JSON festlegen")))
            // 🙈️ Internal View measures — selection, hover, AST navigation, completions, editor settings.
            .action_with(writer_hidden_view("requestCompletions", LocalizedLabel::native("Request Completions", "Vervollständigungen anfordern")).with_category("tools"))
            .action_with(writer_hidden_view("textSelect", LocalizedLabel::native("Text Select", "Text auswählen")))
            .action_with(writer_hidden_view("setEditorSelection", LocalizedLabel::native("Set Editor Selection", "Editor-Auswahl festlegen")))
            .action_with(writer_hidden_view("selectAstNode", LocalizedLabel::native("Select Ast Node", "AST-Knoten auswählen")))
            .action_with(writer_hidden_view("setAstSelection", LocalizedLabel::native("Set Ast Selection", "AST-Auswahl festlegen")))
            .action_with(writer_hidden_view("setAstHover", LocalizedLabel::native("Set Ast Hover", "Überfahren (AST) festlegen")))
            .action_with(writer_hidden_view("textHover", LocalizedLabel::native("Text Hover", "Text-Hover")))
            .action_with(writer_hidden_view("toggleLineNumbers", LocalizedLabel::native("Toggle Line Numbers", "Zeilennummern umschalten")))
            .action_with(writer_hidden_view("setEditorSetting", LocalizedLabel::native("Set Editor Setting", "Editor-Einstellung festlegen")))
            .action_with(writer_hidden_view("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe")))
            // 📝️ Staged argument forms: example choice + the dev JSON setters.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("jack", LocalizedLabel::native("Jack", "Jack")),
                    ActionArgOption::new("dag.jack", LocalizedLabel::native("Dag Jack", "Dag Jack")),
                ]).default_value("jack"),
            ])
            .action_args("setSnapshotJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Document JSON", "Dokument-JSON"))])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Fixture JSON", "Fixture-JSON"))])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (mirrors `shooting_ui::create_shooting_app`'s identical wiring) —
            // `crate::artifacts::writer::engine::writer_io()` is the single source of truth for both the
            // trait's `io()` override and this manifest declaration.
            .config(WriterPlayApp::config_spec())
            .io(crate::artifacts::writer::engine::writer_io()),
    )
    .example("jack", LocalizedLabel::native("Jack", "Jack"), crate::artifacts::writer::engine::jack_example_json(), "file-text")
    .example("dag.jack", LocalizedLabel::native("Dag Jack", "Dag Jack"), crate::artifacts::writer::engine::dag_jack_example_json(), "file-text")
    .workflow("writer", "Writer", "text.document")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as framework_new_app, new_app_with_registry as framework_new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type WriterApp = VcsArtifactApp<WriterPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> WriterApp {
        framework_new_app::<WriterPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry() -> WriterApp {
        framework_new_app_with_registry::<WriterPlayApp>(create_writer_app)
    }

    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    pub fn app_with_jack() -> WriterApp {
        let mut app = new_app();
        app.dispatch_typed(WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }), &meta("local")).expect("load jack");
        app
    }

    pub fn dispatch(app: &mut WriterApp, command: WriterCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut WriterApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut WriterApp) -> Vec<WindowMeasure> {
        app.window_measures().get(WRITER_PLAY_WINDOW_KIND).cloned().expect("main window measures")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::writer::testkit::{new_app_with_registry, WriterApp};
    use semio_framework_plugin::PluginApp;

    fn context_menu_items(app: &mut WriterApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "writer.play".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row (`setEditorSetting`
    /// legitimately covers three rows — see the `app_commands!` doc comment above), and every row's wire
    /// keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 25, "every WriterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
        let expectations: Vec<(&str, WriterCommand)> = vec![
            ("text-edit", WriterCommand::TextEdit(text_edit::TextEdit { text: "x".into() })),
            ("set-text", WriterCommand::SetText(set_text::SetText { text: "x".into() })),
            ("set-snapshot", WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: jack_snapshot() })),
            ("open-document", WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: "x".into() })),
            ("document-json", WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() })),
            ("fixture-json", WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() })),
            ("active-example", WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() })),
            ("format-document", WriterCommand::FormatDocument(format_document::FormatDocument {})),
            ("commit-rename", WriterCommand::CommitRename(commit_rename::CommitRename { text: "x".into() })),
            ("camera", WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::writer::WriterCamera::default() })),
            ("request-completions", WriterCommand::RequestCompletions(request_completions::RequestCompletions {})),
            ("lint-document", WriterCommand::LintDocument(lint_document::LintDocument {})),
            ("text-select", WriterCommand::TextSelect(text_select::TextSelect { start: 0, end: 1 })),
            ("editor-selection", WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 0, end: 1 })),
            ("select-ast-node", WriterCommand::SelectAstNode(select_ast_node::SelectAstNode { id: "n1".into(), start: 0, end: 1 })),
            ("ast-selection", WriterCommand::SetAstSelection(set_ast_selection::SetAstSelection { ids: vec!["n1".into()] })),
            ("ast-hover", WriterCommand::SetAstHover(set_ast_hover::SetAstHover { id: Some("n1".into()) })),
            ("text-hover", WriterCommand::TextHover(text_hover::TextHover { start: Some(0), end: Some(1) })),
            ("toggle-line-numbers", WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {})),
            ("font-px", WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 16 })),
            ("line-height", WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 24 })),
            ("tab-size", WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 })),
            ("engagement-input", WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "x".into() })),
            ("engagement-submit", WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("x".into()) })),
            ("locale", WriterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })),
        ];
        for (expected_keyword, command) in expectations {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
        }
    }

    /// ✍️ Hand-built representative document — used across the app's own command-surface tests.
    fn jack_snapshot() -> WriterSnapshot {
        WriterSnapshot { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<WriterCommand> {
        vec![
            WriterCommand::TextEdit(text_edit::TextEdit { text: "hello".into() }),
            WriterCommand::SetText(set_text::SetText { text: "MATCH (a) RETURN a".into() }),
            WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { snapshot: jack_snapshot() }),
            WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: String::new() }),
            WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() }),
            WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
            WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }),
            WriterCommand::FormatDocument(format_document::FormatDocument {}),
            WriterCommand::CommitRename(commit_rename::CommitRename { text: "piece".into() }),
            WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::writer::WriterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            WriterCommand::RequestCompletions(request_completions::RequestCompletions {}),
            WriterCommand::LintDocument(lint_document::LintDocument {}),
            WriterCommand::TextSelect(text_select::TextSelect { start: 3, end: 7 }),
            WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 7 }),
            WriterCommand::SelectAstNode(select_ast_node::SelectAstNode { id: "jack-ast-1".into(), start: 0, end: 5 }),
            WriterCommand::SetAstSelection(set_ast_selection::SetAstSelection { ids: vec!["a".into(), "b".into()] }),
            WriterCommand::SetAstHover(set_ast_hover::SetAstHover { id: Some("jack-ast-1".into()) }),
            WriterCommand::TextHover(text_hover::TextHover { start: Some(3), end: None }),
            WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {}),
            WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 16 }),
            WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 24 }),
            WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 }),
            WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "format".into() }),
            WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }),
            WriterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
    /// bytes captured from the pre-merge `writer_protocol` crate (this ticket's
    /// `🧪️wire-baseline-before.txt`, rows 15/16/22). A regression here is a real format break, not a
    /// test-fixture mismatch.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(WriterCommand, &str, &str); 3] = [
            (WriterCommand::SetAstHover(set_ast_hover::SetAstHover { id: Some("jack-ast-1".into()) }), "ast-hover ast-hover id=jack-ast-1", "0110010a6a61636b2d6173742d3101000600"),
            (WriterCommand::TextHover(text_hover::TextHover { start: Some(3), end: None }), "text-hover text-hover start=3", "01110001000403"),
            (WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }), "engagement-submit engagement-submit", "01170000"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_writer_app().definition).expect("app definition json");
        assert!(json.contains(WRITER_PLAY_WINDOW_KIND), "window kind missing from the manifest: {json}");
        assert!(json.contains(edit::WRITER_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [WRITER_PLAY_BODY_ARTIFACT, WRITER_PLAY_BODY_CATALOGUE, WRITER_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("text.document"), "artifact kind missing from the manifest");
    }

    /// 📄️ Both declared examples ("jack" and "dag.jack") must be registered on the app — the
    /// `semio_plugin!` macro's own generated sanity test only checks that the app id itself appears in
    /// the bundle manifest, not that a specific named example is registered.
    #[test]
    fn manifest_includes_both_examples() {
        let app = create_writer_app();
        assert!(app.examples.iter().any(|example| example.id == "jack"), "jack example missing from the manifest");
        assert!(app.examples.iter().any(|example| example.id == "dag.jack"), "dag.jack example missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️PortTests
    #[test]
    fn writer_io_declares_the_extra_text_out_port() {
        let io = crate::artifacts::writer::engine::writer_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let text_out = ports.iter().find(|port| port.id == "text:out").expect("text:out port declared");
        assert_eq!(text_out.kind_id.as_deref(), Some("text.document"));
        assert_eq!(text_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }

    #[test]
    fn export_media_text_out_projects_the_document_as_a_chapter() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::engine::jack_example_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView { snapshot: &document, history: &history };
        let media = WriterPlayApp::export_media("text:out", &doc_view).expect("export text:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "text.document");
        let payload: crate::artifacts::writer::engine::WriterChapterPayload = serde_json::from_str(&json).expect("decode chapter payload");
        assert_eq!(payload.text, document.text);
        assert_eq!(payload.language_id, document.language_id);
    }

    #[test]
    fn export_media_rejects_unknown_ports() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::engine::empty_writer_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView { snapshot: &document, history: &history };
        assert!(matches!(WriterPlayApp::export_media("nonsense:out", &doc_view), Err(MediaError::NotImplemented)));
    }
    //#endregion 🔖️PortTests

    //#region 🔖️ContextMenu
    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the writer text-editor context menu stays a
    /// shallow, disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall
    /// of rows, and the destructive `cut` row stays the trailing item.
    #[test]
    fn context_menu_is_grouped_and_keeps_cut_last_and_destructive() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::engine::jack_example_document();
        let config = WriterConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &document, history: &history };
        let cfg = ConfigView { snapshot: &config };
        let registry = AppActionRegistry::from_definition(&create_writer_app().definition);
        let request = ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: WRITER_PLAY_BODY_MAIN.into(), args: None },
            surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "writer.play".into(),
                kind: "textEditor".into(),
                hits: Vec::new(),
                selection: Vec::new(),
                text: Some(ContextMenuTextContext { caret: 0, has_selection: true, word: None, can_rename: true, has_completions: true }),
            }),
            window_instance_id: None,
            point: None,
        };
        let items = WriterPlayApp::context_menu(&request, &doc, &cfg, &registry);
        assert!(items.len() <= 9, "top-level writer context menu should stay progressively disclosed: {items:?}");
        assert_eq!(items.last().map(|item| item.id.as_str()), Some("writer-cut"), "cut must stay the trailing destructive item: {items:?}");
        assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing writer-cut must be marked destructive: {items:?}");
    }

    #[test]
    fn context_menu_via_the_registry_still_starts_with_select_token() {
        let mut app = new_app_with_registry();
        let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "writer.play".into(), kind: "textEditor".into(), hits: vec![], selection: vec![], text: None }));
        assert!(menu.to_string().contains("writer-select-token"), "menu should be {menu}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️CrossCutting
    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::writer::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "writer.play.nope").contains("Unknown body"));
    }

    #[test]
    fn whole_document_operation_replaces_the_snapshot() {
        let app = WriterPlayApp;
        let replacement = jack_snapshot();
        let operation = WriterPlayApp::whole_document_operation(replacement.clone()).expect("whole document operation");
        assert_eq!(operation, WriterMutation::SetSnapshot { snapshot: replacement });
    }

    #[test]
    fn window_engagements_expose_format_lint_placeholder() {
        let mut app = testkit::new_app();
        let engagements = app.window_engagements();
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    #[test]
    fn window_engagements_include_format_and_lint_possible_engagements() {
        let mut app = testkit::new_app();
        let engagements = app.window_engagements();
        let engagement = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("writer window engagement");
        let ids: Vec<&str> = engagement.possible_engagements.as_ref().expect("possible engagements").iter().map(|possible| possible.id.as_str()).collect();
        assert!(ids.contains(&"writer-format"));
        assert!(ids.contains(&"writer-lint"));
    }

    /// 🗣️ Cross-cutting locale check across every rendering surface (inspection, catalogue,
    /// engagements, measures) at once — narrower per-node locale tests live beside each node, but this
    /// is the integration-level guarantee that locale threads through the whole app consistently.
    #[test]
    fn writer_labels_resolve_native_english_by_default_across_every_surface() {
        let mut app = testkit::new_app();
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("\"Document\""));
        assert!(inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Language\""));
        assert!(catalogue_json.contains("Cypher-inspired"));
        let engagements = app.window_engagements();
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("\"Format\""));
        assert!(engagements_json.contains("\"Lint\""));
        let measures = app.window_measures();
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Font size"));
        assert!(measures_json.contains("Line numbers"));
        assert!(!measures_json.contains("Schriftgröße"));
    }

    #[test]
    fn writer_labels_resolve_german_locale_across_every_surface() {
        let mut app = testkit::new_app();
        app.dispatch_typed(WriterCommand::SetLocale(set_locale::SetLocale { value: "de".into() }), &semio_framework_plugin::testkit::meta("local")).expect("set locale");
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Dokument"));
        assert!(inspection_json.contains("Kamera"));
        assert!(!inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &semio_framework_plugin::ViewModel::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Sprache"));
        let measures = app.window_measures();
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Schriftgröße"));
        assert!(measures_json.contains("Zeilennummern"));
        let engagements = app.window_engagements();
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("Texteditor"));
        assert!(engagements_json.contains("Formatieren"));
        assert!(engagements_json.contains("Prüfen"));
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
