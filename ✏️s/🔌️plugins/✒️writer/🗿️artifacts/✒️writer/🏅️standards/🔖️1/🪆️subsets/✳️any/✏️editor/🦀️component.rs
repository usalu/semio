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

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::{writer_text, writer_text_owner, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
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
use semio_framework::{kernel::Effect, InteractiveJobClassification, RetainedToolWireInput, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, EditorApp, EphemeralEmit, InteractionView};
use semio_framework_plugin::{
    engagement_token_matches, strip_engagement_prefix, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, AppActionRegistry, AppIo, ArtifactEditor, ArtifactView, ConfigView, ContextMenuItemSpec,
    ContextMenuRequest, ContextMenuTextContext, Dialect, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode, WindowMeasure,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
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
pub fn writer_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(WRITER_PLAY_APP_ID).action(action, args)
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
    let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&scene.schema, &scene.id, scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("writer document spr encode is infallible for a fresh, edit-free envelope");
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
    use crate::artifacts::writer::schema::{parse_jack_ast, JackAstNode};

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
mod record_tutorial {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "record-tutorial")]
    pub struct RecordTutorial {}

    pub fn handle(_payload: &RecordTutorial, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}

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
        "recordTutorial" as "record-tutorial" => record_tutorial::RecordTutorial,
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
    "setCamera",
    "requestCompletions",
    "lintDocument",
    "setEditorSelection",
    "toggleLineNumbers",
    "setEditorSetting",
    "engagementInput",
    "setActiveExample",
    "setSnapshot",
    "openDocument",
    "setSnapshotJson",
    "setFixtureJson",
    "formatDocument",
    "commitRename",
    "engagementSubmit",
    "setLocale",
    "recordTutorial",
];
const WRITER_COMMAND_PAYLOAD_SCHEMA: &str = "writer.writer.tool-command.v1";
const MAX_WRITER_COMMAND_RAW_BYTES: usize = 4_096;
const MAX_WRITER_COMMAND_DECODED_ITEMS: usize = 4_096;
const MAX_WRITER_COMMAND_TEXT_BYTES: usize = 4_096;
const MAX_WRITER_COMMAND_URI_BYTES: usize = 1_024;
const MAX_WRITER_LOCALE_BYTES: usize = 64;

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
        if matches!(command, WriterCommand::OpenDocument(payload) if payload.text.len() > MAX_WRITER_COMMAND_TEXT_BYTES || payload.uri.len() > MAX_WRITER_COMMAND_URI_BYTES) {
            return false;
        }
        let requires_text = match command {
            WriterCommand::FormatDocument(_) | WriterCommand::CommitRename(_) => true,
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
            WriterCommand::TextEdit(payload) => Emit::amend(vec![WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: payload.text })], "writer-text-edit"),
            WriterCommand::SetText(payload) => Emit::mutations(vec![WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: payload.text })]),
            WriterCommand::SetCamera(payload) => Emit::config(vec![WriterConfigMutation::SetCamera { camera: payload.camera }]),
            WriterCommand::RequestCompletions(_) => Emit::config(vec![WriterConfigMutation::SetRevision { value: config.revision + 1 }]),
            WriterCommand::LintDocument(_) => Emit::config(vec![WriterConfigMutation::SetLintSignal { value: config.lint_signal + 1 }, WriterConfigMutation::SetRevision { value: config.revision + 1 }]),
            WriterCommand::SetEditorSelection(payload) => Emit::config(vec![
                WriterConfigMutation::SetEditorSelection { selection: Some(crate::editor::writer::config::WriterEditorSelection { start: payload.start, end: payload.end }) },
                WriterConfigMutation::SetRevision { value: config.revision + 1 },
            ]),
            WriterCommand::ToggleLineNumbers(_) => {
                let mut settings = config.editor_settings.clone();
                settings.show_line_numbers = !settings.show_line_numbers;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetFontPx(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.font_px = payload.value;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetLineHeight(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.line_height = payload.value;
                Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::SetTabSize(payload) => {
                let mut settings = config.editor_settings.clone();
                settings.tab_size = payload.value.max(1);
                Emit::config(vec![WriterConfigMutation::SetEditorSettings { settings }, WriterConfigMutation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::EngagementInput(payload) if payload.value != config.engagement_input => {
                Emit::config(vec![WriterConfigMutation::SetEngagementInput { value: payload.value }, WriterConfigMutation::SetRevision { value: config.revision + 1 }])
            }
            WriterCommand::EngagementInput(_) => Emit::default(),
            WriterCommand::SetActiveExample(payload) => {
                let document = match payload.example_id.as_str() {
                    "jack" => crate::artifacts::writer::dsl::jack_example_document(),
                    "dag.jack" => crate::artifacts::writer::dsl::dag_jack_example_document(),
                    _ => crate::artifacts::writer::schema::empty_writer_snapshot(),
                };
                Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }
            }
            WriterCommand::SetSnapshot(payload) => serde_json::from_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::OpenDocument(payload) => open_document::emit(&payload),
            WriterCommand::SetSnapshotJson(payload) => serde_json::from_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::SetFixtureJson(payload) => serde_json::from_str::<WriterSnapshot>(&payload.json).map(|document| Emit { effects: vec![reset_document_effect_now(&document)], ..Default::default() }).unwrap_or_default(),
            WriterCommand::FormatDocument(_) => {
                let formatted = crate::artifacts::writer::schema::format_writer_text(text, &snapshot.language_id);
                let mut emit = Emit::config(vec![WriterConfigMutation::SetFormatSignal { value: config.format_signal + 1 }]);
                if formatted != text.as_ref() {
                    emit.artifact_mutations = vec![WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: formatted })];
                }
                emit
            }
            WriterCommand::CommitRename(payload) => {
                use crate::artifacts::writer::schema::{apply_jack_rename, jack_symbol_at_offset, JackSymbolKind};
                let selection = config.editor_selection.clone().unwrap_or(crate::editor::writer::config::WriterEditorSelection { start: 0, end: 0 });
                if selection.start == selection.end {
                    if let Some(symbol) = jack_symbol_at_offset(text, selection.start) {
                        if symbol.kind == JackSymbolKind::Variable {
                            let renamed = apply_jack_rename(text, &symbol.occurrences, &payload.text);
                            return Ok(Emit::mutations(vec![WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: renamed })]));
                        }
                    }
                }
                if selection.start <= selection.end && selection.end <= text.len() {
                    let mut updated = text.to_string();
                    updated.replace_range(selection.start..selection.end, &payload.text);
                    Emit::mutations(vec![WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: updated })])
                } else {
                    Emit::default()
                }
            }
            WriterCommand::EngagementSubmit(payload) => {
                let value = payload.value.unwrap_or_else(|| config.engagement_input.clone());
                let trimmed = value.trim();
                let mut config_mutations = vec![WriterConfigMutation::SetEngagementInput { value: String::new() }, WriterConfigMutation::SetRevision { value: config.revision + 1 }];
                let mut artifact_mutations = Vec::new();
                if engagement_token_matches(trimmed, "format") {
                    config_mutations.push(WriterConfigMutation::SetFormatSignal { value: config.format_signal + 1 });
                    let formatted = crate::artifacts::writer::schema::format_writer_text(text, &snapshot.language_id);
                    if formatted != text.as_ref() {
                        artifact_mutations.push(WriterMutation::EditText(crate::artifacts::writer::op::EditText { text: formatted }));
                    }
                } else if engagement_token_matches(trimmed, "lint") {
                    config_mutations.push(WriterConfigMutation::SetLintSignal { value: config.lint_signal + 1 });
                } else if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
                    let mut settings = config.editor_settings.clone();
                    settings.show_line_numbers = !settings.show_line_numbers;
                    config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
                } else if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
                    if let Ok(px) = rest.parse::<u32>() {
                        let mut settings = config.editor_settings.clone();
                        settings.font_px = px;
                        config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
                    }
                } else if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
                    if let Ok(size) = rest.parse::<u32>() {
                        let mut settings = config.editor_settings.clone();
                        settings.tab_size = size.max(1);
                        config_mutations.push(WriterConfigMutation::SetEditorSettings { settings });
                    }
                }
                Emit { artifact_mutations, config_mutations, ..Default::default() }
            }
            WriterCommand::SetLocale(payload) => Emit::config(vec![WriterConfigMutation::SetLocale { value: payload.value }]),
            WriterCommand::RecordTutorial(_) => Emit::default(),
            _ => return Err("writer command job received an unregistered command"),
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
        if !self.completed {
            let Some(completion) = self.completion.clone() else { return Self::fault() };
            if !completion.has_mounted_consumer() {
                return Self::fault();
            }
            let emit = match self.emit() {
                Ok(emit) => emit,
                Err(_) => return Self::fault(),
            };
            if completion.complete(Ok(emit), EphemeralEmit::default()).is_err() {
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
            return step;
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
        self.closing && self.command.is_none() && self.snapshot.is_none() && self.text.is_none() && self.config.is_none() && self.completion.is_none() && self.raw_input.is_none() && self.raw_bytes.is_empty() && self.raw_bytes.capacity() == 0
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
}
//#endregion 🧵️InteractiveJobs

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

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<WriterPlayApp>,
        owner_file: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.writer.writer@1/*#editor",
        document_schema: "writer.document",
        factory: "BoundedFirstStepCommandJobFactory",
        contract: semio_framework::ToolExecutionContract::bounded_first_step(4_096, 4_096, 1, 64, 2_000),
        tools: [
            "textEdit",
            "setText",
            "setCamera",
            "requestCompletions",
            "lintDocument",
            "setEditorSelection",
            "toggleLineNumbers",
            "setEditorSetting",
            "engagementInput",
            "setActiveExample",
            "setSnapshot",
            "openDocument",
            "setSnapshotJson",
            "setFixtureJson",
            "formatDocument",
            "commitRename",
            "engagementSubmit",
            "setLocale",
            "recordTutorial"
        ]
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::writer::spr::writer_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::writer::spr::writer_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::writer::spr::writer_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::writer::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::schema::empty_writer_snapshot()
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
            .action_interactive_job("recordTutorial", InteractiveJobClassification::Migrated)
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
    pub fn new_app() -> WriterApp {
        framework_new_app::<EditorApp<WriterPlayApp>>()
    }

    /// Adapts create_writer_app's AppDefinition (contract 2.4) into the App { definition, examples }
    /// shape testkit::new_app_with_registry/assert_declared_actions_bridge_to_commands still expect --
    /// framework testkit gap (framework crate outside this packet's lease), not modifiable here.
    fn writer_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_writer_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry() -> WriterApp {
        framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_testkit)
    }

    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    /// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `reset_document_effect`'s doc comment), so `setActiveExample` no longer lands via
    /// `dispatch_typed` alone; this loads the same document pack a real host would apply from that
    /// command's `Effect::LoadDocument`, via `PluginApp::load_document_pack` directly — the same
    /// technique `📐️cad`'s own `two_instances_converge_disjoint_edits_via_backbone` test uses.
    pub fn app_with_jack() -> WriterApp {
        let mut app = new_app();
        let document = crate::artifacts::writer::dsl::jack_example_document();
        let (schema, id) = (document.schema.clone(), document.id.clone());
        let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&schema, &id, document, None);
        let files = store::print_document_pack(&envelope).expect("print jack document pack");
        app.load_document_pack(&files).expect("load jack");
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
    use crate::editor::writer::testkit::{new_app_with_registry, WriterApp};
    use semio_framework_plugin::PluginApp;

    fn context_menu_items(app: &mut WriterApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "writer.play".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    fn writer_envelope_wire() -> Vec<u8> {
        let envelope = store::create_document_envelope(WRITER_DOCUMENT_SCHEMA, "writer-live-load", crate::artifacts::writer::schema::empty_writer_snapshot(), None);
        let wire = serde_json::to_vec(&envelope).expect("outgoing Writer fixture envelope");
        let mut retirement = crate::artifacts::writer::spr::writer_envelope_decode_owner_bundle().retire_envelope(envelope);
        for _ in 0..10_000 {
            match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Writer fixture envelope retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return wire;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Writer fixture envelope retirement blocked"),
            }
        }
        panic!("Writer fixture envelope retirement did not reach terminal")
    }

    #[test]
    fn interactive_job_fixture_matches_the_exact_factory_join() {
        let fixture: Value = serde_json::from_str(include_str!("📚️examples/🎬️demo-session/🧵️interactive-job-migration.json")).expect("language-neutral Writer migration fixture");
        assert_eq!(fixture["payloadSchema"], WRITER_COMMAND_PAYLOAD_SCHEMA);
        assert_eq!(fixture["maxRawWireBytes"], MAX_WRITER_COMMAND_RAW_BYTES);
        assert_eq!(fixture["maxCurrentTextBytes"], MAX_WRITER_COMMAND_TEXT_BYTES);
        assert_eq!(fixture["maxOpenDocumentUriBytes"], MAX_WRITER_COMMAND_URI_BYTES);
        assert_eq!(fixture["maxWorkUnitsPerStep"], 1);
        let actions = fixture["migrated"].as_array().expect("migrated action rows").iter().map(|row| row["action"].as_str().expect("action id")).collect::<Vec<_>>();
        assert_eq!(actions, WRITER_COMMAND_TOOL_IDS);
        assert_eq!(fixture["textAdmissionCases"][0]["textBytes"], MAX_WRITER_COMMAND_TEXT_BYTES);
        assert_eq!(fixture["textAdmissionCases"][1]["textBytes"], MAX_WRITER_COMMAND_TEXT_BYTES + 1);
        assert_eq!(fixture["textAdmissionCases"][1]["fuelDelta"], 0);
        assert_eq!(fixture["textAdmissionCases"][1]["cursorDelta"], 0);
        assert_eq!(fixture["textAdmissionCases"][1]["ownersPreserved"], true);
        assert_eq!(fixture["localeAdmissionCases"][0]["localeBytes"], MAX_WRITER_LOCALE_BYTES);
        assert_eq!(fixture["localeAdmissionCases"][1]["localeBytes"], MAX_WRITER_LOCALE_BYTES + 1);
        assert_eq!(fixture["localeAdmissionCases"][1]["fuelDelta"], 0);
        assert_eq!(fixture["localeAdmissionCases"][1]["cursorDelta"], 0);
        assert_eq!(fixture["localeAdmissionCases"][1]["ownersPreserved"], true);
        assert_eq!(fixture["openDocumentAdmissionCases"][0]["textBytes"], MAX_WRITER_COMMAND_TEXT_BYTES);
        assert_eq!(fixture["openDocumentAdmissionCases"][0]["uriBytes"], MAX_WRITER_COMMAND_URI_BYTES);
        assert_eq!(fixture["openDocumentAdmissionCases"][1]["textBytes"], MAX_WRITER_COMMAND_TEXT_BYTES + 1);
        assert_eq!(fixture["openDocumentAdmissionCases"][2]["uriBytes"], MAX_WRITER_COMMAND_URI_BYTES + 1);
        assert_eq!(fixture["openDocumentAdmissionCases"][1]["ownersPreserved"], true);
        assert_eq!(fixture["openDocumentAdmissionCases"][2]["ownersPreserved"], true);
        assert_eq!(fixture["requiredLifecycle"].as_array().map(Vec::len), Some(10));
    }

    #[test]
    fn retained_wire_decoder_and_third_party_serde_have_command_parity() {
        let snapshot_json = serde_json::to_string(&crate::artifacts::writer::schema::empty_writer_snapshot()).expect("bounded snapshot fixture");
        let commands = vec![
            WriterCommand::TextEdit(text_edit::TextEdit { text: "ä".into() }),
            WriterCommand::SetText(set_text::SetText { text: "bounded".into() }),
            WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::artifacts::writer::WriterCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
            WriterCommand::RequestCompletions(request_completions::RequestCompletions {}),
            WriterCommand::LintDocument(lint_document::LintDocument {}),
            WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 1, end: 2 }),
            WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {}),
            WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 14 }),
            WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 20 }),
            WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 }),
            WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "Format".into() }),
            WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }),
            WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: snapshot_json.clone() }),
            WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://brief.md".into(), text: "# Brief".into() }),
            WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: snapshot_json.clone() }),
            WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: snapshot_json }),
            WriterCommand::FormatDocument(format_document::FormatDocument {}),
            WriterCommand::CommitRename(commit_rename::CommitRename { text: "renamed".into() }),
            WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("format".into()) }),
            WriterCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            WriterCommand::RecordTutorial(record_tutorial::RecordTutorial {}),
        ];
        for command in commands {
            let wire = <WriterCommand as protocol::OpBinary>::encode_op(&command).expect("owned protocol wire");
            assert!(wire.len() <= MAX_WRITER_COMMAND_RAW_BYTES);
            assert_eq!(<WriterCommand as protocol::OpBinary>::decode_op(&wire).expect("owned retained decoder"), command);
            let serde_wire = serde_json::to_vec(&command).expect("third-party serde wire");
            assert_eq!(serde_json::from_slice::<WriterCommand>(&serde_wire).expect("third-party serde decoder"), command);
        }
    }

    fn writer_command_job(command: WriterCommand, text: Arc<str>) -> WriterCommandToolJob {
        WriterCommandToolJob {
            command: Some(command),
            snapshot: Some(Arc::new(crate::artifacts::writer::schema::empty_writer_snapshot())),
            text: Some(text),
            config: Some(Arc::new(WriterConfig::default())),
            completion: None,
            raw_input: None,
            raw_bytes: vec![1, 2, 3],
            raw_page_cursor: 2,
            raw_scan_cursor: 1,
            raw_validated: true,
            text_admitted: false,
            completed: false,
            closing: false,
        }
    }

    #[test]
    fn bounded_text_admission_preserves_rejected_job_state_and_owners() {
        for command in [
            WriterCommand::FormatDocument(format_document::FormatDocument {}),
            WriterCommand::CommitRename(commit_rename::CommitRename { text: "renamed".into() }),
            WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("format".into()) }),
        ] {
            let maximum: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES));
            let mut accepted = writer_command_job(command, maximum.clone());
            let accepted_cursor = (accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone());
            assert!(accepted.admit_text());
            assert!(accepted.text_admitted);
            assert_eq!((accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone()), accepted_cursor);
            assert_eq!(Arc::strong_count(&maximum), 2);

            let over: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1));
            let mut rejected = writer_command_job(accepted.command.take().expect("accepted command owner"), over.clone());
            let rejected_cursor = (rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone());
            let rejected_command = rejected.command.clone();
            let rejected_snapshot = rejected.snapshot.clone();
            let rejected_config = rejected.config.clone();
            assert!(!rejected.admit_text());
            assert!(!rejected.text_admitted);
            assert_eq!((rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone()), rejected_cursor);
            assert_eq!(rejected.command, rejected_command);
            assert!(Arc::ptr_eq(rejected.snapshot.as_ref().expect("snapshot owner"), rejected_snapshot.as_ref().expect("saved snapshot owner")));
            assert!(Arc::ptr_eq(rejected.config.as_ref().expect("config owner"), rejected_config.as_ref().expect("saved config owner")));
            assert_eq!(Arc::strong_count(&over), 2);
        }

        let over: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1));
        let mut lint = writer_command_job(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("lint".into()) }), over);
        assert!(lint.admit_text());
    }

    #[test]
    fn bounded_locale_admission_preserves_maximum_plus_one_job_state_and_owners() {
        let current: Arc<str> = Arc::from("");
        let mut accepted = writer_command_job(WriterCommand::SetLocale(set_locale::SetLocale { value: "x".repeat(MAX_WRITER_LOCALE_BYTES) }), current.clone());
        let accepted_cursor = (accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone());
        assert!(accepted.admit_text());
        assert!(accepted.text_admitted);
        assert_eq!((accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone()), accepted_cursor);
        assert_eq!(Arc::strong_count(&current), 2);
        let accepted_emit = accepted.emit().expect("bounded locale emission");
        assert_eq!(accepted_emit.config_mutations, vec![WriterConfigMutation::SetLocale { value: "x".repeat(MAX_WRITER_LOCALE_BYTES) }]);

        let mut rejected = writer_command_job(WriterCommand::SetLocale(set_locale::SetLocale { value: "x".repeat(MAX_WRITER_LOCALE_BYTES + 1) }), current.clone());
        let rejected_cursor = (rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone());
        let rejected_command = rejected.command.clone();
        let rejected_snapshot = rejected.snapshot.clone();
        let rejected_config = rejected.config.clone();
        assert!(!rejected.admit_text());
        assert!(!rejected.text_admitted);
        assert_eq!((rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone()), rejected_cursor);
        assert_eq!(rejected.command, rejected_command);
        assert!(Arc::ptr_eq(rejected.snapshot.as_ref().expect("snapshot owner"), rejected_snapshot.as_ref().expect("saved snapshot owner")));
        assert!(Arc::ptr_eq(rejected.config.as_ref().expect("config owner"), rejected_config.as_ref().expect("saved config owner")));
        assert_eq!(Arc::strong_count(&current), 3);
    }

    #[test]
    fn bounded_open_document_admission_preserves_maximum_plus_one_job_state_and_owners() {
        let current: Arc<str> = Arc::from("");
        let accepted_command = WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) });
        let mut accepted = writer_command_job(accepted_command, current.clone());
        assert!(accepted.admit_text());
        assert_eq!(accepted.emit().expect("bounded open document emission").effects.len(), 1);

        for rejected_command in [
            WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
            WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES + 1), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        ] {
            let mut rejected = writer_command_job(rejected_command, current.clone());
            let rejected_cursor = (rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone());
            let rejected_command = rejected.command.clone();
            let rejected_snapshot = rejected.snapshot.clone();
            let rejected_config = rejected.config.clone();
            assert!(!rejected.admit_text());
            assert_eq!((rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone()), rejected_cursor);
            assert_eq!(rejected.command, rejected_command);
            assert!(Arc::ptr_eq(rejected.snapshot.as_ref().expect("snapshot owner"), rejected_snapshot.as_ref().expect("saved snapshot owner")));
            assert!(Arc::ptr_eq(rejected.config.as_ref().expect("config owner"), rejected_config.as_ref().expect("saved config owner")));
        }
    }

    fn admit_writer_envelope(app: &mut WriterApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Writer live envelope ingress credits");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Writer live envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Writer live envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("Writer live envelope seal/submit"));
        handle
    }

    fn drive_writer_live_load(app: &mut WriterApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..100_000 {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Writer live maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("Writer live load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("Writer live envelope load did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
        let mut app = crate::editor::writer::testkit::new_app();
        let base_generation = app.artifact_generation_now();
        let handle = admit_writer_envelope(&mut app, &writer_envelope_wire());
        assert_eq!(handle.generation, base_generation);
        assert_eq!(drive_writer_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Writer load acknowledgement"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Writer load acknowledgement is a no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_live_envelope_cancel_closes_retained_pages_without_publication() {
        let mut app = crate::editor::writer::testkit::new_app();
        let base_generation = app.artifact_generation_now();
        let wire = writer_envelope_wire();
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Writer ingress credits");
        let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..first.len()].copy_from_slice(first);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Writer first page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Writer page admission failed: {fault}"));
        app.cancel_artifact_envelope_load(handle).expect("cancel exact Writer ingress");
        assert_eq!(drive_writer_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), base_generation);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_completions_use_example_fixture() {
        let json = crate::artifacts::writer::standards::v1::subsets::any::schema::jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row (`setEditorSetting`
    /// legitimately covers three rows — see the `app_commands!` doc comment above), and every row's wire
    /// keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 21, "every WriterCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
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
            ("record-tutorial", WriterCommand::RecordTutorial(record_tutorial::RecordTutorial {})),
        ];
        for (expected_keyword, command) in expectations {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
        }
    }

    /// ✍️ Hand-built representative document — used across the app's own command-surface tests.
    fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<WriterCommand> {
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
            WriterCommand::RecordTutorial(record_tutorial::RecordTutorial {}),
        ]
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
    /// bytes captured from the pre-merge `writer_protocol` crate (this ticket's
    /// `🧪️wire-baseline-before.txt`, row 22 — rows 15/16 (`ast-hover`/`text-hover`) dissolved into the
    /// framework's own `ast` interaction domain and no longer exist as writer commands). A regression
    /// here is a real format break, not a test-fixture mismatch.
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
    async fn writer_io_declares_the_extra_text_out_port() {
        let io = writer_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let text_out = ports.iter().find(|port| port.id == "text:out").expect("text:out port declared");
        assert_eq!(text_out.kind_id.as_deref(), Some("text.document"));
        assert_eq!(text_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }

    #[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
    async fn context_menu_via_the_registry_still_starts_with_select_token() {
        let mut app = new_app_with_registry();
        let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "writer.play".into(), kind: "textEditor".into(), hits: vec![], selection: vec![], text: None }));
        assert!(menu.to_string().contains("writer-select-token"), "menu should be {menu}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::writer::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "writer.play.nope").contains("Unknown body"));
    }

    /// 🌱️ `SetSnapshot` is banned outright (see `whole_document_operation`'s doc comment) — the
    /// trait default correctly returns `None`; whole-document replace goes through
    /// `reset_document_effect` instead, exercised by `📚️examples/🎬️demo-session`'s own command
    /// tests and by `commands::text`'s `set_active_example`/`open_document` tests.
    #[semio_framework_async_macros::async_test]
    async fn whole_document_operation_stays_the_trait_default_none() {
        let replacement = jack_snapshot();
        assert_eq!(WriterPlayApp::whole_document_operation(replacement), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn window_engagements_expose_format_lint_placeholder() {
        let mut app = testkit::new_app();
        let engagements = app.window_engagements();
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
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
