//! ✒️ Writer editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/*/🪟️windows/*`, chrome measures in that window's `🎚️options/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, document-side pure compute
//! in the artifact's `🧬️schema`, and this app's own typed media I/O surface + plugin registration
//! (below — constitutional: general, an artifact must never depend on an app, so both live here rather
//! than under `🗿️artifacts`). This file is a routing table: `handle` → `WriterCommand::dispatch`,
//! `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::writer::commands::set_camera;
use crate::editor::writer::commands::set_editor_selection;
use crate::editor::writer::commands::set_locale;
use crate::editor::writer::commands::{commit_rename, format_document, open_document, set_active_example, set_fixture_json, set_snapshot, set_snapshot_json, set_text, text_edit};
use crate::editor::writer::commands::{engagement_input, engagement_submit};
use crate::editor::writer::commands::{lint_document, request_completions};
use crate::editor::writer::commands::{set_font_px, set_line_height, set_tab_size, toggle_line_numbers};
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::editor::writer::modes::edit;
use crate::editor::writer::modes::edit::windows::main;
use crate::editor::writer::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::writer::presence::{WriterPresence, WriterPresenceMutation};
use crate::editor::writer::terminology::writer_play_labels;
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, AppActionRegistry, AppIo, ArtifactEditor, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, ContextMenuTextContext, Dialect, DomainTopology,
    DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, MergeMode,
    NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode, WindowMeasure,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const WRITER_PLAY_APP_ID: &str = "writer-play";
pub use catalogue_panel::WRITER_PLAY_BODY_CATALOGUE;
pub use document_panel::WRITER_PLAY_BODY_ARTIFACT;
pub use inspection_panel::WRITER_PLAY_BODY_INSPECTION;
pub use main::{WRITER_PLAY_BODY_MAIN, WRITER_PLAY_WINDOW_KIND};

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub async fn writer_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionFactory::new(WRITER_PLAY_APP_ID).action(action, args)
}

/// 🙈️ An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
async fn writer_hidden_operation(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️ An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
async fn writer_hidden_view(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra output, `text:out` (Text×Document, kind `text.document`, `Many` — a workflow may fan this
/// writer's text out to several consumers, e.g. `playbook`'s `chapters:in`).
pub async fn writer_io() -> AppIo {
    AppIo {
        document_schema: WRITER_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "text:out".into(),
            label: "Text".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
            kind_id: Some("text.document".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "text.document".into(), name: "Text Document".into(), dimension: "text".into(), component_kind: "writer".into() },
    }
}

/// 📤️ The JSON shape `"text:out"` exports and `playbook`'s `"chapters:in"` imports — one writer
/// document's text as one "chapter" (`title` mirrors the document id, `language_id` lets an importer
/// route jack/wire content differently from prose if it ever wants to). Single consumer (this file's
/// `export_media`), so it lives here rather than in the artifact's `🧬️schema`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    pub language_id: String,
}

/// 🎞️ Projects a `WriterSnapshot` onto the `"text:out"` chapter payload shape.
pub async fn writer_chapter_payload(document: &WriterSnapshot) -> WriterChapterPayload {
    WriterChapterPayload { id: document.id.clone(), title: document.id.clone(), text: writer_text(document), language_id: document.language_id.clone() }
}

/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE history —
/// the sanctioned non-mutation path for a whole-document replace (open file, load example, dev JSON
/// setters). Per the SMO-agreed mutation taxonomy, whole-document replace has NO mutation-enum
/// representative (`SetSnapshot` is banned outright); every former "replace the whole document"
/// gesture builds this effect instead of an `Emit::mutations([...])` — mirrors `📐️cad`'s identical
/// `reset_document_effect` (`📓️wave3-reports/cad-report.md`). The spr is a fresh, edit-free op-log
/// for `scene`'s own `schema`/`id` — a genesis envelope with no history to encode.
pub async fn reset_document_effect(scene: &WriterSnapshot) -> Effect {
    let pack = <WriterSnapshot as ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&scene.schema, &scene.id, scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("writer document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Io

//#region 🔖️Interaction
/// 🕹️ `ast` domain topology from the jack AST's own parent links — `HierarchyProvider::Topology`, so
/// this is the framework's ONLY source of truth for that domain's membership/hierarchy (selection
/// pruning after a document edit, `selectAll`, range-select, transitive descendant-closure
/// hover/selection). Empty for a non-jack document (nothing to select) or a document with no AST.
async fn writer_ast_topology(document: &WriterSnapshot) -> DomainTopology {
    use crate::artifacts::writer::schema::{parse_jack_ast, JackAstNode};

    async fn visit(node: &JackAstNode, parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        out.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: parent.map(str::to_string) });
        for child in &node.children {
            visit(child, Some(node.id.as_str()), out);
        }
    }

    let mut ordered = Vec::new();
    if document.language_id == "jack" {
        let root = parse_jack_ast(&writer_text(document));
        visit(&root, None, &mut ordered);
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

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
        "setEditorSelection" as "editor-selection" => set_editor_selection::SetEditorSelection,
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
async fn writer_context_menu_items(registry: &AppActionRegistry, text: Option<&ContextMenuTextContext>, is_de: bool) -> Vec<ContextMenuItemSpec> {
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

impl ArtifactEditor for WriterPlayApp {
    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Config = WriterConfig;
    type ConfigMutation = WriterConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = WriterPresence;
    type PresenceMutation = WriterPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = WriterCommand;

    const DIALECT: Dialect = crate::artifacts::writer::WRITER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::writer::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::schema::empty_writer_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(writer_io())
    }

    // 🌱️ No `whole_document_operation` override: per the SMO-agreed mutation taxonomy
    // (`📌️important.md`'s "Forbidden vocabulary"), whole-document replace has NO mutation-enum
    // representative — `SetSnapshot` is banned outright — so this falls back to the trait's own
    // default (`None`), matching `📐️cad`/`💠️lowpoly`'s identical ruling. Every former "replace the
    // whole document" gesture (`setSnapshot`/`openDocument`/JSON setters/`setActiveExample`) now
    // builds `reset_document_effect` (a `Effect::LoadDocument`, outside undo history) instead.

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &WriterCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &WriterCommand,
        doc: &ArtifactView<'_, WriterSnapshot>,
        cfg: &ConfigView<'_, WriterConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WriterMutation, WriterConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `ast` domain: `HierarchyProvider::Topology` from the jack AST's own parent links — see
    /// `writer_ast_topology`'s doc comment.
    async fn interaction_topology(doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("ast".to_string(), writer_ast_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"text:out"` exports the writer document's current text as one "chapter" payload (see
    /// `writer_chapter_payload`) — `playbook`'s `"chapters:in"` is the intended consumer. Falls through
    /// to the default whole-document-pack export for `"document:out"` (duplicated inline, not delegated
    /// — Rust traits have no `super` call for an overridden default).
    async fn export_media(port: &str, doc: &ArtifactView<'_, WriterSnapshot>) -> Result<Media, MediaError> {
        if port == "text:out" {
            let payload = writer_chapter_payload(doc.snapshot);
            let json = serde_json::to_string(&payload).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json } });
        }
        if port != "document:out" {
            return Err(MediaError::NotImplemented);
        }
        let bytes = doc.snapshot.encode_pack();
        Ok(Media { media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> UiNode {
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

    async fn window_engagements(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
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

    async fn window_measures(_doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), main::window_measures(config, writer_play_labels(config)))])
    }

    async fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
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
pub async fn create_writer_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::writer::WRITER_DIALECT)
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
            // 🙈️ Internal View measures — editor caret/range, completions, editor settings. AST
            // selection/hover no longer declared here: the framework auto-injects
            // interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity for every domain declared via `.interaction(...)` below.
            .action_with(writer_hidden_view("requestCompletions", LocalizedLabel::native("Request Completions", "Vervollständigungen anfordern")).with_category("tools"))
            .action_with(writer_hidden_view("setEditorSelection", LocalizedLabel::native("Set Editor Selection", "Editor-Auswahl festlegen")))
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
            // 🕹️ THE TRANSITIVE TEMPLATE (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // `ast` is `HierarchyProvider::Topology` from the jack AST's own parent links
            // (`writer_ast_topology`/`WriterPlayApp::interaction_topology`); `hover.transitive`/
            // `selection.transitive` both `true` — dispatch the deepest AST node at the caret and
            // transitivity produces the covering behavior the old `jack_ast_node_for_selection`
            // covering-node search used to compute by hand.
            .interaction(InteractionDefinition {
                id: "ast".into(),
                label: LocalizedLabel::native("AST", "AST"),
                granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Single, SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(WRITER_PLAY_WINDOW_KIND, vec![InteractionRef::new("ast")])
            // 🎯️ Typed channel surface (mirrors `shooting_ui::create_shooting_app`'s identical wiring) —
            // `writer_io()` is the single source of truth for both the trait's `io()` override and this
            // manifest declaration.
            .config(WriterPlayApp::config_spec())
            .io(writer_io())
            // SDK GAP (contract 2.4, Editor/Viewer builder split): .example(...)/.workflow(...) do
            // not exist on Editor::builder's type -- the two example registrations ("jack",
            // "dag.jack", label + JSON fixture + icon) and the "writer" workflow entry ("writer",
            // "Writer", "text.document") that used to chain here are dropped, not silently lost.
            // The subset-level examples facet (moved whole into this editor) still ships the demo
            // fixture; the app-level example PICKER UI is the actual regression until this SDK gap closes.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as framework_new_app, new_app_with_registry as framework_new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// WriterPlayApp implements the AUTHORING trait ArtifactEditor, not the runtime ArtifactApp --
    /// EditorApp<WriterPlayApp> (SDK adapter, contract 2.1) is the real ArtifactApp implementor
    /// VcsArtifactApp wraps, the same way PluginBuilder::editor::<WriterPlayApp> builds it.
    pub type WriterApp = VcsArtifactApp<EditorApp<WriterPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> WriterApp {
        framework_new_app::<EditorApp<WriterPlayApp>>()
    }

    /// Adapts create_writer_app's AppDefinition (contract 2.4) into the App { definition, examples }
    /// shape testkit::new_app_with_registry/assert_declared_actions_bridge_to_commands still expect --
    /// framework testkit gap (framework crate outside this packet's lease), not modifiable here.
    async fn writer_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_writer_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> WriterApp {
        framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_testkit)
    }

    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    /// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `reset_document_effect`'s doc comment), so `setActiveExample` no longer lands via
    /// `dispatch_typed` alone; this loads the same document pack a real host would apply from that
    /// command's `Effect::LoadDocument`, via `PluginApp::load_document_pack` directly — the same
    /// technique `📐️cad`'s own `two_instances_converge_disjoint_edits_via_backbone` test uses.
    pub async fn app_with_jack() -> WriterApp {
        let mut app = new_app();
        let document = crate::artifacts::writer::dsl::jack_example_document();
        let (schema, id) = (document.schema.clone(), document.id.clone());
        let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&schema, &id, document, None);
        let files = store::print_document_pack(&envelope).expect("print jack document pack");
        app.load_document_pack(&files).expect("load jack");
        app
    }

    pub async fn dispatch(app: &mut WriterApp, command: WriterCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut WriterApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub async fn main_window_measures(app: &mut WriterApp) -> Vec<WindowMeasure> {
        app.window_measures().get(WRITER_PLAY_WINDOW_KIND).cloned().expect("main window measures")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::testkit::{new_app_with_registry, WriterApp};
    use semio_framework_plugin::PluginApp;

    async fn context_menu_items(app: &mut WriterApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "writer.play".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    #[test]
    async fn jack_completions_use_example_fixture() {
        let json = crate::artifacts::writer::standards::v1::subsets::any::schema::jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row (`setEditorSetting`
    /// legitimately covers three rows — see the `app_commands!` doc comment above), and every row's wire
    /// keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 20, "every WriterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
    async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
        let expectations: Vec<(&str, WriterCommand)> = vec![
            ("text-edit", WriterCommand::TextEdit(text_edit::TextEdit { text: "x".into() })),
            ("set-text", WriterCommand::SetText(set_text::SetText { text: "x".into() })),
            ("set-snapshot", WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "{}".into() })),
            ("open-document", WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: "x".into() })),
            ("document-json", WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() })),
            ("fixture-json", WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() })),
            ("active-example", WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() })),
            ("format-document", WriterCommand::FormatDocument(format_document::FormatDocument {})),
            ("commit-rename", WriterCommand::CommitRename(commit_rename::CommitRename { text: "x".into() })),
            ("camera", WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::writer::WriterCamera::default() })),
            ("request-completions", WriterCommand::RequestCompletions(request_completions::RequestCompletions {})),
            ("lint-document", WriterCommand::LintDocument(lint_document::LintDocument {})),
            ("editor-selection", WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 0, end: 1 })),
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
    async fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<WriterCommand> {
        vec![
            WriterCommand::TextEdit(text_edit::TextEdit { text: "hello".into() }),
            WriterCommand::SetText(set_text::SetText { text: "MATCH (a) RETURN a".into() }),
            WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "{}".into() }),
            WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: String::new() }),
            WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() }),
            WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
            WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }),
            WriterCommand::FormatDocument(format_document::FormatDocument {}),
            WriterCommand::CommitRename(commit_rename::CommitRename { text: "piece".into() }),
            WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::writer::WriterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            WriterCommand::RequestCompletions(request_completions::RequestCompletions {}),
            WriterCommand::LintDocument(lint_document::LintDocument {}),
            WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 7 }),
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
    /// `🧪️wire-baseline-before.txt`, row 22 — rows 15/16 (`ast-hover`/`text-hover`) dissolved into the
    /// framework's own `ast` interaction domain and no longer exist as writer commands). A regression
    /// here is a real format break, not a test-fixture mismatch.
    #[test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(WriterCommand, &str, &str); 1] = [(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }), "engagement-submit engagement-submit", "01120000")];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_writer_app()).expect("app definition json");
        assert!(json.contains(WRITER_PLAY_WINDOW_KIND), "window kind missing from the manifest: {json}");
        assert!(json.contains(edit::WRITER_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [WRITER_PLAY_BODY_ARTIFACT, WRITER_PLAY_BODY_CATALOGUE, WRITER_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("text.document"), "artifact kind missing from the manifest");
    }

    /// SDK GAP (contract 2.4): create_writer_app() now returns a bare AppDefinition -- .example(...)
    /// does not exist on Editor::builder, so the "jack"/"dag.jack" example registrations this test
    /// used to assert on no longer exist to assert on (App{definition,examples} no longer flows through
    /// this builder path at all). Deleted rather than left compiling against a field that is gone;
    /// tracked as a real regression in the migration notes, not silently dropped.
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Interaction
    /// 🕹️ The `ast` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
    /// selection, and scoped to writer's one window kind — the manifest side of THE TRANSITIVE
    /// TEMPLATE (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    #[test]
    async fn ast_interaction_domain_is_declared_topology_and_transitive_on_the_main_window() {
        let definition = create_writer_app();
        let ast = definition.interactions.iter().find(|interaction| interaction.id == "ast").expect("ast interaction domain declared");
        assert!(matches!(ast.hierarchy, HierarchyProvider::Topology));
        assert!(ast.hover.transitive, "ast hover must be transitive for the covering-node behavior");
        assert!(ast.selection.transitive, "ast selection must be transitive for the covering-node behavior");
        let main_window = definition.window_kinds.iter().find(|window| window.id == WRITER_PLAY_WINDOW_KIND).expect("main window kind declared");
        assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == "ast"), "main window must reference the ast interaction domain");
    }

    /// 🌳️ `interaction_topology` walks the jack AST's own `children` into `TopologyNode.parent` links —
    /// root has no parent, every child's parent is its syntactic parent's id.
    #[test]
    async fn interaction_topology_walks_the_jack_ast_into_parent_links() {
        let document = crate::artifacts::writer::dsl::jack_example_document();
        let config = WriterConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = WriterPlayApp::interaction_topology(&doc, &cfg);
        let ast = topology.domains.get("ast").expect("ast domain present in topology");
        assert!(!ast.ordered.is_empty(), "jack document must produce a non-empty ast topology");
        let root = &ast.ordered[0];
        assert!(root.parent.is_none(), "the first (pre-order) node is the AST root and has no parent");
        assert!(ast.ordered.iter().skip(1).all(|node| node.parent.is_some()), "every non-root node must carry its syntactic parent's id");
    }

    /// 🌱️ A non-jack document has no AST to select — an empty topology, matching `Flat`-vs-empty
    /// pruning semantics: every stale `ast` selection id gets pruned for a document with no AST.
    #[test]
    async fn interaction_topology_is_empty_for_non_jack_documents() {
        let document = crate::artifacts::writer::schema::empty_writer_snapshot();
        let config = WriterConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = WriterPlayApp::interaction_topology(&doc, &cfg);
        assert!(topology.domains.get("ast").expect("ast domain present in topology").ordered.is_empty());
    }
    //#endregion 🔖️Interaction

    //#region 🔖️PortTests
    #[test]
    async fn writer_io_declares_the_extra_text_out_port() {
        let io = writer_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let text_out = ports.iter().find(|port| port.id == "text:out").expect("text:out port declared");
        assert_eq!(text_out.kind_id.as_deref(), Some("text.document"));
        assert_eq!(text_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }

    #[test]
    async fn export_media_text_out_projects_the_document_as_a_chapter() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::dsl::jack_example_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::resolve_ready(WriterPlayApp::export_media("text:out", &doc_view)).expect("export text:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "text.document");
        let payload: WriterChapterPayload = serde_json::from_str(&json).expect("decode chapter payload");
        assert_eq!(payload.text, writer_text(&document));
        assert_eq!(payload.language_id, document.language_id);
    }

    #[test]
    async fn export_media_rejects_unknown_ports() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::schema::empty_writer_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView::new(&document, &history);
        assert!(matches!(semio_framework_plugin::resolve_ready(WriterPlayApp::export_media("nonsense:out", &doc_view)), Err(MediaError::NotImplemented)));
    }
    //#endregion 🔖️PortTests

    //#region 🔖️ContextMenu
    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the writer text-editor context menu stays a
    /// shallow, disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall
    /// of rows, and the destructive `cut` row stays the trailing item.
    #[test]
    async fn context_menu_is_grouped_and_keeps_cut_last_and_destructive() {
        let app = WriterPlayApp;
        let document = crate::artifacts::writer::dsl::jack_example_document();
        let config = WriterConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let registry = AppActionRegistry::from_definition(&create_writer_app());
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
    async fn context_menu_via_the_registry_still_starts_with_select_token() {
        let mut app = new_app_with_registry();
        let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "writer.play".into(), kind: "textEditor".into(), hits: vec![], selection: vec![], text: None }));
        assert!(menu.to_string().contains("writer-select-token"), "menu should be {menu}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️CrossCutting
    #[test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::writer::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "writer.play.nope").contains("Unknown body"));
    }

    /// 🌱️ `SetSnapshot` is banned outright (see `whole_document_operation`'s doc comment) — the
    /// trait default correctly returns `None`; whole-document replace goes through
    /// `reset_document_effect` instead, exercised by `📚️examples/🎬️demo-session`'s own command
    /// tests and by `commands::text`'s `set_active_example`/`open_document` tests.
    #[test]
    async fn whole_document_operation_stays_the_trait_default_none() {
        let replacement = jack_snapshot();
        assert_eq!(WriterPlayApp::whole_document_operation(replacement), None);
    }

    #[test]
    async fn window_engagements_expose_format_lint_placeholder() {
        let mut app = testkit::new_app();
        let engagements = app.window_engagements();
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    #[test]
    async fn window_engagements_include_format_and_lint_possible_engagements() {
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
    async fn writer_labels_resolve_native_english_by_default_across_every_surface() {
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
    async fn writer_labels_resolve_german_locale_across_every_surface() {
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
