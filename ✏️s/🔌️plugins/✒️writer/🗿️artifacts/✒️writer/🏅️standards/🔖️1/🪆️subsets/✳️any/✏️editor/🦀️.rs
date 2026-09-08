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

use crate::op::WriterMutation;
use crate::{writer_text, writer_text_owner, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
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
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::{kernel::Effect, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolCompletion, ArtifactToolCompletionRejection, ArtifactToolFactoryRegistry, EditorApp, EphemeralEmit, InteractionView};
use semio_framework_plugin::{
    engagement_token_matches, strip_engagement_prefix, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppIo, ArtifactEditor, ArtifactView, ConfigView, ContextMenuItemSpec,
    ContextMenuRequest, ContextMenuTextContext, Dialect, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, WindowMeasure,
};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const WRITER_PLAY_APP_ID: &str = "s.writer.writer@1/*#editor";
pub use catalogue_panel::WRITER_PLAY_BODY_CATALOGUE;
pub use document_panel::WRITER_PLAY_BODY_ARTIFACT;
pub use inspection_panel::WRITER_PLAY_BODY_INSPECTION;
pub use main::{WRITER_PLAY_BODY_MAIN, WRITER_PLAY_WINDOW_KIND};

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn writer_action(action: &str, args: Option<dsl::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: WRITER_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "writer label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

/// 🙈️ An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
fn writer_hidden_operation(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️ An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
fn writer_hidden_view(id: &str, label: LocalizedLabel) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::View) }
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra output, `text:out` (Text×Document, kind `text.document`, `Many` — a workflow may fan this
/// writer's text out to several consumers, e.g. `playbook`'s `chapters:in`).
pub fn writer_io() -> AppIo {
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
pub fn writer_chapter_payload(document: &WriterSnapshot) -> WriterChapterPayload {
    WriterChapterPayload { id: document.id.clone(), title: document.id.clone(), text: writer_text(document), language_id: document.language_id.clone() }
}

/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE history —
/// the sanctioned non-mutation path for a whole-document replace (open file, load example, dev JSON
/// setters). Per the SMO-agreed mutation taxonomy, whole-document replace has NO mutation-enum
/// representative (`SetSnapshot` is banned outright); every former "replace the whole document"
/// gesture builds this effect instead of an `Emit::mutations([...])` — mirrors `📐️cad`'s identical
/// `reset_document_effect` (`📓️wave3-reports/cad-report.md`). The spr is a fresh, edit-free op-log
/// for `scene`'s own `schema`/`id` — a genesis envelope with no history to encode.
fn reset_document_effect_now(scene: &WriterSnapshot) -> Effect {
    let pack = <WriterSnapshot as ArtifactPack>::encode_pack(scene);
    let spr = semio_framework::io::resolve_ready(store::empty_document_spr(&scene.id, &scene.schema));
    Effect::LoadDocument { pack, spr }
}

pub fn reset_document_effect(scene: &WriterSnapshot) -> Effect {
    reset_document_effect_now(scene)
}
//#endregion 🔖️Io

//#region 🔖️Interaction
/// 🕹️ `ast` domain topology from the jack AST's own parent links — `HierarchyProvider::Topology`, so
/// this is the framework's ONLY source of truth for that domain's membership/hierarchy (selection
/// pruning after a document edit, `selectAll`, range-select, transitive descendant-closure
/// hover/selection). Empty for a non-jack document (nothing to select) or a document with no AST.
fn writer_ast_topology(document: &WriterSnapshot) -> DomainTopology {
    use crate::schema::{parse_jack_ast, JackAstNode};

    fn visit(node: &JackAstNode, parent: Option<&str>, out: &mut Vec<TopologyNode>) {
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

//#region 🧵️InteractiveJobs
const WRITER_COMMAND_TOOL_IDS: &[&str] = &[
    "textEdit",
    "setText",
    "setActiveExample",
    "setSnapshot",
    "openDocument",
    "setSnapshotJson",
    "setFixtureJson",
    "formatDocument",
    "commitRename",
    "setCamera",
    "requestCompletions",
    "lintDocument",
    "setEditorSelection",
    "toggleLineNumbers",
    "setEditorSetting",
    "engagementInput",
    "engagementSubmit",
    "setLocale",
];
const WRITER_COMMAND_PAYLOAD_SCHEMA: &str = "writer.writer.tool-command.v1";
const MAX_WRITER_COMMAND_RAW_BYTES: usize = 4_096;
const MAX_WRITER_COMMAND_DECODED_ITEMS: usize = 4_096;
const MAX_WRITER_COMMAND_TEXT_BYTES: usize = 4_096;
const MAX_WRITER_COMMAND_URI_BYTES: usize = 1_024;
const MAX_WRITER_LOCALE_BYTES: usize = 64;
const MAX_WRITER_EXAMPLE_ID_BYTES: usize = 64;

struct WriterCommandToolPayload {
    command: WriterCommand,
    snapshot: Arc<WriterSnapshot>,
    text: Arc<str>,
    config: Arc<WriterConfig>,
    completion: Option<ArtifactToolCompletion<EditorApp<WriterPlayApp>>>,
}

struct WriterCommandToolJob {
    command: Option<WriterCommand>,
    snapshot: Option<Arc<WriterSnapshot>>,
    text: Option<Arc<str>>,
    config: Option<Arc<WriterConfig>>,
    completion: Option<ArtifactToolCompletion<EditorApp<WriterPlayApp>>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<WriterPlayApp>>>,
    raw_input: Option<RetainedToolWireInput>,
    raw_bytes: Vec<u8>,
    raw_page_cursor: usize,
    raw_scan_cursor: usize,
    raw_validated: bool,
    text_admitted: bool,
    completed: bool,
    closing: bool,
}

impl WriterCommandToolJob {
    fn checkpoint(&self, context: &mut StepContext<'_>) -> StepOutcome {
        let progress = self.raw_bytes.len().saturating_add(self.raw_scan_cursor).saturating_add(usize::from(self.raw_validated)).saturating_add(usize::from(self.text_admitted)) as u64;
        let state = context.payload_from_bytes(JobPayloadStream::CheckpointState, &progress.to_le_bytes()).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(JobPayloadStream::CheckpointState)
        });
        StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: progress })
    }

    fn admit_text(&mut self) -> bool {
        let Some(command) = self.command.as_ref() else { return false };
        let Some(config) = self.config.as_ref() else { return false };
        if matches!(command, WriterCommand::SetLocale(payload) if payload.value.len() > MAX_WRITER_LOCALE_BYTES) {
            return false;
        }
        if matches!(command, WriterCommand::EngagementInput(payload) if payload.value.len() > MAX_WRITER_COMMAND_TEXT_BYTES) {
            return false;
        }
        if matches!(command, WriterCommand::TextEdit(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
            || matches!(command, WriterCommand::SetText(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
            || matches!(command, WriterCommand::CommitRename(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
        {
            return false;
        }
        if matches!(command, WriterCommand::EngagementSubmit(payload) if payload.value.as_ref().unwrap_or(&config.engagement_input).len() > MAX_WRITER_COMMAND_TEXT_BYTES) {
            return false;
        }
        if matches!(command, WriterCommand::SetActiveExample(payload) if payload.example_id.len() > MAX_WRITER_EXAMPLE_ID_BYTES) {
            return false;
        }
        if matches!(command, WriterCommand::SetSnapshot(payload) if payload.json.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
            || matches!(command, WriterCommand::SetSnapshotJson(payload) if payload.json.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
            || matches!(command, WriterCommand::SetFixtureJson(payload) if payload.json.len() > MAX_WRITER_COMMAND_TEXT_BYTES)
        {
            return false;
        }
        if matches!(command, WriterCommand::OpenDocument(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES || payload.uri.len() > MAX_WRITER_COMMAND_URI_BYTES) {
            return false;
        }
        let requires_text = match command {
            WriterCommand::TextEdit(_) | WriterCommand::SetText(_) | WriterCommand::FormatDocument(_) | WriterCommand::CommitRename(_) => true,
            WriterCommand::EngagementSubmit(payload) => {
                let value = payload.value.as_deref().unwrap_or(&config.engagement_input);
                engagement_token_matches(value.trim(), "format")
            }
            _ => false,
        };
        if requires_text && self.text.as_ref().is_none_or(|text| text.len() > MAX_WRITER_COMMAND_TEXT_BYTES) {
            return false;
        }
        self.text_admitted = true;
        true
    }

    fn emit(&mut self) -> Result<Emit<WriterMutation, WriterConfigMutation>, &'static str> {
        let command = self.command.take().ok_or("writer command job lost its command owner")?;
        let config = self.config.as_ref().ok_or("writer command job lost its config owner")?;
        let snapshot = self.snapshot.as_ref().ok_or("writer command job lost its snapshot owner")?;
        let text = self.text.as_ref().ok_or("writer command job lost its text owner")?;
        Ok(match command {
            WriterCommand::TextEdit(payload) => Emit::amend(vec![WriterMutation::EditText(crate::op::EditText { text: payload.text })], "writer-text-edit"),
            WriterCommand::SetText(payload) => Emit::mutations(vec![WriterMutation::EditText(crate::op::EditText { text: payload.text })]),
            WriterCommand::SetCamera(payload) => Emit::config(vec![WriterConfigMutation::SetCamera(crate::editor::writer::config::SetCamera { camera: payload.camera })]),
            WriterCommand::RequestCompletions(_) => Emit::config(vec![WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })]),
            WriterCommand::LintDocument(_) => Emit::config(vec![WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { value: config.lint_signal + 1 }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })]),
            WriterCommand::SetEditorSelection(payload) => Emit::config(vec![
                WriterConfigMutation::SetEditorSelection(crate::editor::writer::config::SetEditorSelection { selection: Some(crate::editor::writer::config::WriterEditorSelection { start: payload.start, end: payload.end }) }),
                WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 }),
            ]),
            WriterCommand::ToggleLineNumbers(_) => {
                let mut settings = config.editor_settings.clone();
                settings.show_line_numbers = !settings.show_line_numbers;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })])
            }
            WriterCommand::SetFontPx(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.font_px = payload.value;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })])
            }
            WriterCommand::SetLineHeight(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.line_height = payload.value;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })])
            }
            WriterCommand::SetTabSize(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.tab_size = payload.value.max(1);
                Emit::config(vec![WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })])
            }
            WriterCommand::EngagementInput(payload) if payload.value != config.engagement_input => {
                Emit::config(vec![WriterConfigMutation::SetEngagementInput(crate::editor::writer::config::SetEngagementInput { value: payload.value }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })])
            }
            WriterCommand::EngagementInput(_) => Emit::default(),
            WriterCommand::SetActiveExample(payload) => {
                let document = match payload.example_id.as_str() {
                    "jack" => crate::dsl::jack_example_document(),
                    "dag.jack" => crate::dsl::dag_jack_example_document(),
                    _ => crate::schema::empty_writer_snapshot(),
                };
                Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }
            }
            WriterCommand::SetSnapshot(payload) => dsl::os_pack::json::from_json_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::OpenDocument(payload) => open_document::emit(&payload),
            WriterCommand::SetSnapshotJson(payload) => dsl::os_pack::json::from_json_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::SetFixtureJson(payload) => dsl::os_pack::json::from_json_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::FormatDocument(_) => {
                let formatted = crate::schema::format_writer_text(text, &snapshot.language_id);
                let mut emit = Emit::config(vec![WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { value: config.format_signal + 1 })]);
                if formatted != text.as_ref() {
                    emit.artifact_mutations = vec![WriterMutation::EditText(crate::op::EditText { text: formatted })];
                }
                emit
            }
            WriterCommand::CommitRename(payload) => {
                use crate::schema::{apply_jack_rename, jack_symbol_at_offset, JackSymbolKind};
                let selection = config.editor_selection.clone().unwrap_or(crate::editor::writer::config::WriterEditorSelection { start: 0, end: 0 });
                if selection.start == selection.end {
                    if let Some(symbol) = jack_symbol_at_offset(text, selection.start) {
                        if symbol.kind == JackSymbolKind::Variable {
                            let renamed = apply_jack_rename(text, &symbol.occurrences, &payload.text);
                            return Ok(Emit::mutations(vec![WriterMutation::EditText(crate::op::EditText { text: renamed })]));
                        }
                    }
                }
                if selection.start <= selection.end && selection.end <= text.len() {
                    let mut updated = text.to_string();
                    updated.replace_range(selection.start..selection.end, &payload.text);
                    Emit::mutations(vec![WriterMutation::EditText(crate::op::EditText { text: updated })])
                } else {
                    Emit::default()
                }
            }
            WriterCommand::EngagementSubmit(payload) => {
                let value = payload.value.unwrap_or_else(|| config.engagement_input.clone());
                let trimmed = value.trim();
                let mut config_mutations = vec![WriterConfigMutation::SetEngagementInput(crate::editor::writer::config::SetEngagementInput { value: String::new() }), WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value: config.revision + 1 })];
                let mut artifact_mutations = Vec::new();
                if engagement_token_matches(trimmed, "format") {
                    config_mutations.push(WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { value: config.format_signal + 1 }));
                    let formatted = crate::schema::format_writer_text(text, &snapshot.language_id);
                    if formatted != text.as_ref() {
                        artifact_mutations.push(WriterMutation::EditText(crate::op::EditText { text: formatted }));
                    }
                } else if engagement_token_matches(trimmed, "lint") {
                    config_mutations.push(WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { value: config.lint_signal + 1 }));
                } else if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
                    let mut settings = config.editor_settings.clone();
                    settings.show_line_numbers = !settings.show_line_numbers;
                    config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
                } else if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
                    if let Ok(px) = rest.parse::<u32>() {
                        let mut settings = config.editor_settings.clone();
                        settings.font_px = px;
                        config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
                    }
                } else if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
                    if let Ok(size) = rest.parse::<u32>() {
                        let mut settings = config.editor_settings.clone();
                        settings.tab_size = size.max(1);
                        config_mutations.push(WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }));
                    }
                }
                Emit { artifact_mutations, config_mutations, ..Default::default() }
            }
            WriterCommand::SetLocale(payload) => Emit::config(vec![WriterConfigMutation::SetLocale(crate::editor::writer::config::SetLocale { value: payload.value })]),
        })
    }

    fn fault() -> StepOutcome {
        StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) })
    }
}

impl InteractiveJob for WriterCommandToolJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return StepOutcome::Yield;
        }
        context.set_stage(if self.raw_validated { "writer-command-reduce" } else { "writer-command-retained-wire-decode" });
        if !self.raw_validated {
            let Some(input) = self.raw_input.as_ref() else { return Self::fault() };
            if let Some(page) = input.page(self.raw_page_cursor) {
                if self.raw_bytes.len().checked_add(page.len()).is_none_or(|bytes| bytes > MAX_WRITER_COMMAND_RAW_BYTES) {
                    return Self::fault();
                }
                self.raw_bytes.extend_from_slice(page);
                self.raw_page_cursor += 1;
                context.consume_fuel(1);
                return self.checkpoint(context);
            }
            if self.raw_scan_cursor < self.raw_bytes.len() {
                self.raw_scan_cursor += 1;
                context.consume_fuel(1);
                return self.checkpoint(context);
            }
            let decoded = match <WriterCommand as protocol::OpBinary>::decode_op(&self.raw_bytes) {
                Ok(command) => command,
                Err(_) => return Self::fault(),
            };
            if self.command.as_ref() != Some(&decoded) {
                return Self::fault();
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return self.checkpoint(context);
        }
        if !self.text_admitted {
            if !self.admit_text() {
                return Self::fault();
            }
            context.consume_fuel(1);
            return self.checkpoint(context);
        }
        if self.pending_completion_rejection.is_some() {
            return Self::fault();
        }
        if !self.completed {
            let Some(completion) = self.completion.clone() else { return Self::fault() };
            if !completion.has_mounted_consumer() {
                return Self::fault();
            }
            let emit = match self.emit() {
                Ok(emit) => emit,
                Err(_) => return Self::fault(),
            };
            if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                self.pending_completion_rejection = Some(rejected);
                return Self::fault();
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if !self.raw_bytes.is_empty() {
            if maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.raw_bytes.len().min(maximum_bytes);
            self.raw_bytes.truncate(self.raw_bytes.len() - released_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
        }
        if self.raw_bytes.capacity() != 0 {
            let released_bytes = self.raw_bytes.capacity();
            if maximum_items == 0 || maximum_bytes < released_bytes {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.raw_bytes = Vec::new();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(maximum_items.min(1), maximum_bytes);
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            };
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return match step {
                        semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                        semio_framework_plugin::PluginCloseStep::Blocked { .. } | semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } => InteractiveJobCloseStep::Blocked,
                        semio_framework_plugin::PluginCloseStep::Complete => unreachable!("child close helper consumes completed children"),
                    };
                }
            }
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.pending_completion_rejection = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.command.is_some() {
            if maximum_items == 0 || maximum_bytes < MAX_WRITER_COMMAND_RAW_BYTES {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.command = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: MAX_WRITER_COMMAND_RAW_BYTES };
        }
        if self.text.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.text = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.snapshot.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.snapshot = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.config.is_some() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.config = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(completion) = self.completion.as_ref() {
            if maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if !completion.has_mounted_consumer() {
                return InteractiveJobCloseStep::Blocked;
            }
            self.completion = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.command.is_none() && self.snapshot.is_none() && self.text.is_none() && self.config.is_none() && self.completion.is_none() && self.raw_input.is_none() && self.raw_bytes.is_empty() && self.raw_bytes.capacity() == 0
    }
}

struct WriterCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl WriterCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: WRITER_COMMAND_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for WriterCommandJobFactory {
    type Payload = WriterCommandToolPayload;
    type Job = WriterCommandToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        WRITER_COMMAND_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_WRITER_COMMAND_RAW_BYTES, MAX_WRITER_COMMAND_DECODED_ITEMS, 1, 64, 2_000, 1, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(WriterCommandToolJob {
            command: Some(payload.command),
            snapshot: Some(payload.snapshot),
            text: Some(payload.text),
            config: Some(payload.config),
            completion: payload.completion,
            pending_completion_rejection: None,
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_scan_cursor: 0,
            raw_validated: true,
            text_admitted: false,
            completed: false,
            closing: false,
        })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: Operation,
        payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("writer command retained ingress rejects unvalidated checkpoints"), input, checkpoint));
        }
        let declared_bytes = input.declared_bytes();
        if declared_bytes > MAX_WRITER_COMMAND_RAW_BYTES {
            return Err((ToolJobFactoryError::new("writer command retained ingress exceeds its admitted wire cap"), input, None));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        if job.raw_bytes.try_reserve_exact(declared_bytes).is_err() {
            return Err((ToolJobFactoryError::new("writer command retained decoder capacity was not admitted"), input, None));
        }
        job.raw_input = Some(input);
        job.raw_validated = false;
        Ok(job)
    }
}

impl ArtifactOwnedToolJobFactory for WriterCommandJobFactory {
    type Owner = EditorApp<WriterPlayApp>;
    const TOOL_IDS: &'static [&'static str] = WRITER_COMMAND_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setText", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshotJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "formatDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "commitRename", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "requestCompletions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "lintDocument", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setEditorSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleLineNumbers", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setEditorSetting", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️InteractiveJobs

//#region 📬️ConfigStorePreparation
const WRITER_CONFIG_STORE_MAXIMUM_BYTES: usize = 8_192;

struct WriterConfigStorePreparationFactory;

struct WriterConfigStorePreparation {
    base: Option<store::SnapshotRead<WriterConfig>>,
    mutation: Option<WriterConfigMutation>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<WriterConfig, WriterConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn writer_config_retained_bytes(config: &WriterConfig) -> usize {
    config.engagement_input.len().saturating_add(config.locale.len())
}

fn writer_config_mutation_retained_bytes(mutation: &WriterConfigMutation) -> usize {
    match mutation {
        WriterConfigMutation::ReplaceConfig(crate::editor::writer::config::ReplaceConfig { config }) => writer_config_retained_bytes(config),
        WriterConfigMutation::SetEngagementInput(crate::editor::writer::config::SetEngagementInput { value }) | WriterConfigMutation::SetLocale(crate::editor::writer::config::SetLocale { value }) => value.len(),
        WriterConfigMutation::SetEditorSelection(crate::editor::writer::config::SetEditorSelection { .. })
        | WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { .. })
        | WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { .. })
        | WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { .. })
        | WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { .. })
        | WriterConfigMutation::SetCamera(crate::editor::writer::config::SetCamera { .. }) => 0,
    }
}

fn admit_writer_config_mutation(mutation: &WriterConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = writer_config_mutation_retained_bytes(mutation);
    if retained_bytes > WRITER_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Writer config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_writer_config(base: &WriterConfig, mutation: WriterConfigMutation) -> Result<(WriterConfig, Vec<WriterConfigMutation>, WriterConfigMutation), String> {
    admit_writer_config_mutation(&mutation)?;
    if writer_config_retained_bytes(base) > WRITER_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Writer config base exceeds its fixed retained preparation envelope".into());
    }
    let mut post = base.clone();
    match &mutation {
        WriterConfigMutation::ReplaceConfig(crate::editor::writer::config::ReplaceConfig { config }) => post = config.clone(),
        WriterConfigMutation::SetEditorSelection(crate::editor::writer::config::SetEditorSelection { selection }) => post.editor_selection = selection.clone(),
        WriterConfigMutation::SetFormatSignal(crate::editor::writer::config::SetFormatSignal { value }) => post.format_signal = *value,
        WriterConfigMutation::SetLintSignal(crate::editor::writer::config::SetLintSignal { value }) => post.lint_signal = *value,
        WriterConfigMutation::SetRevision(crate::editor::writer::config::SetRevision { value }) => post.revision = *value,
        WriterConfigMutation::SetEditorSettings(crate::editor::writer::config::SetEditorSettings { settings }) => post.editor_settings = settings.clone(),
        WriterConfigMutation::SetEngagementInput(crate::editor::writer::config::SetEngagementInput { value }) => post.engagement_input = value.clone(),
        WriterConfigMutation::SetCamera(crate::editor::writer::config::SetCamera { camera }) => post.camera = camera.clone(),
        WriterConfigMutation::SetLocale(crate::editor::writer::config::SetLocale { value }) => post.locale = value.clone(),
    }
    Ok((post, vec![WriterConfigMutation::ReplaceConfig(crate::editor::writer::config::ReplaceConfig { config: base.clone() })], mutation))
}

fn writer_store_edit<M>(
    prefix: &str,
    forward: M,
    inverse: Vec<M>,
    description: Option<String>,
    authority: &store::ArtifactStoreOneItemLiveAuthority,
) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
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

impl store::ArtifactStoreOneItemPreparationFactory<WriterConfig, WriterConfigMutation> for WriterConfigStorePreparationFactory {
    fn preflight(&self, mutation: &WriterConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Writer config preparation rejected its lane or description envelope".into());
        }
        admit_writer_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<WriterConfig, WriterConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<WriterConfig, WriterConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<WriterConfig, WriterConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(WriterConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<WriterConfig, WriterConfigMutation> for WriterConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Writer config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Writer config preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_writer_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Writer config preparation lost its Store authority".to_string())?;
        let edit = writer_store_edit("writer-config-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<WriterConfig, WriterConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<WriterConfig, WriterConfigMutation>> {
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
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Writer config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
const WRITER_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 32_768;

struct WriterArtifactStorePreparationFactory;

struct WriterArtifactStorePreparation {
    base: Option<store::SnapshotRead<WriterSnapshot>>,
    mutation: Option<WriterMutation>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<WriterSnapshot, WriterMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn writer_snapshot_retained_bytes(snapshot: &WriterSnapshot) -> usize {
    snapshot
        .schema
        .len()
        .saturating_add(snapshot.id.len())
        .saturating_add(snapshot.language_id.len())
        .saturating_add(snapshot.uri.len())
        .saturating_add(writer_text_owner(snapshot).len())
}

fn admit_writer_artifact_mutation(mutation: &WriterMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let WriterMutation::EditText(payload) = mutation else {
        return Err("Writer retained Artifact preparation only admits the exact EditText cohort".into());
    };
    if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES {
        return Err("Writer EditText exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: payload.text.len() })
}

fn prepare_writer_artifact(base: &WriterSnapshot, mutation: WriterMutation) -> Result<(WriterSnapshot, Vec<WriterMutation>, WriterMutation), String> {
    admit_writer_artifact_mutation(&mutation)?;
    if writer_snapshot_retained_bytes(base) > WRITER_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Writer Artifact base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = crate::op::inverse_writer_mutation(base, &mutation);
    let mut post = base.clone();
    crate::op::apply_writer_mutation(&mut post, &mutation).map_err(|_| "Writer Artifact preparation could not apply its exact sparse diff".to_string())?;
    Ok((post, inverse, mutation))
}

impl store::ArtifactStoreOneItemPreparationFactory<WriterSnapshot, WriterMutation> for WriterArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &WriterMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Writer Artifact preparation rejected its lane or description envelope".into());
        }
        admit_writer_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<WriterSnapshot, WriterMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<WriterSnapshot, WriterMutation>>, store::ArtifactStoreOneItemPreparationRequest<WriterSnapshot, WriterMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(WriterArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<WriterSnapshot, WriterMutation> for WriterArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Writer Artifact preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Writer Artifact preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_writer_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Writer Artifact preparation lost its Store authority".to_string())?;
        let edit = writer_store_edit("writer-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<WriterSnapshot, WriterMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<WriterSnapshot, WriterMutation>> {
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
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Writer Artifact preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation

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

    const DIALECT: Dialect = crate::WRITER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(WriterArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(Arc::new(WriterConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<WriterPlayApp>,
        owner_file: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.writer.writer@1/*#editor",
        document_schema: "writer.document",
        factory: "WriterCommandJobFactory",
        factory_type: WriterCommandJobFactory,
        contract: ToolExecutionContract::resumable(4_096, 4_096, 1, 64, 2_000, 1, 1),
        tools: [
            "textEdit",
            "setText",
            "setActiveExample",
            "setSnapshot",
            "openDocument",
            "setSnapshotJson",
            "setFixtureJson",
            "formatDocument",
            "commitRename",
            "setCamera",
            "requestCompletions",
            "lintDocument",
            "setEditorSelection",
            "toggleLineNumbers",
            "setEditorSetting",
            "engagementInput",
            "engagementSubmit",
            "setLocale"
        ]
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::writer_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::writer_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::writer_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::writer::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> WriterSnapshot {
        crate::schema::empty_writer_snapshot()
    }

    fn io() -> Option<AppIo> {
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
    fn command_id(command: &WriterCommand) -> &'static str {
        command.command_id()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(WriterCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !WRITER_COMMAND_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("writer-command-tool-mismatch"));
        }
        let text = writer_text_owner(&request.snapshot);
        let payload = WriterCommandToolPayload { command: *request.command, snapshot: request.snapshot, text, config: request.config, completion: Some(request.completion) };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn handle(
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
    fn interaction_topology(doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("ast".to_string(), writer_ast_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"text:out"` exports the writer document's current text as one "chapter" payload (see
    /// `writer_chapter_payload`) — `playbook`'s `"chapters:in"` is the intended consumer. Falls through
    /// to the default whole-document-pack export for `"document:out"` (duplicated inline, not delegated
    /// — Rust traits have no `super` call for an overridden default).
    fn export_media(port: &str, doc: &ArtifactView<'_, WriterSnapshot>) -> Result<Media, MediaError> {
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

    fn render(body_key: &str, doc: &ArtifactView<'_, WriterSnapshot>, cfg: &ConfigView<'_, WriterConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = writer_play_labels(config);
        let node = match body_key {
            WRITER_PLAY_BODY_MAIN => main::render(document, config),
            WRITER_PLAY_BODY_ARTIFACT => document_panel::render(document, config, labels),
            WRITER_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            WRITER_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "writer unknown-body text admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
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
pub fn create_writer_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::WRITER_DIALECT)
            .document(["semio", "writer"])
            .artifact_kind(crate::artifact_kind())
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
            .action_with(ActionDefinition::bounded_catalog("formatDocument", LocalizedLabel::native("Format Document", "Dokument formatieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(ActionDefinition::bounded_catalog("lintDocument", LocalizedLabel::native("Lint Document", "Dokument prüfen"), ActionKind::View).with_category("tools"))
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
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setText", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("requestCompletions", InteractiveJobClassification::Migrated)
            .action_interactive_job("lintDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("setEditorSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLineNumbers", InteractiveJobClassification::Migrated)
            .action_interactive_job("setEditorSetting", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("openDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFixtureJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("formatDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("commitRename", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument forms: example choice + the dev JSON setters.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("jack", LocalizedLabel::native("Jack", "Jack")),
                    ActionArgOption::new("dag.jack", LocalizedLabel::native("Dag Jack", "Dag Jack")),
                ]).default_value(&"jack"),
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
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
