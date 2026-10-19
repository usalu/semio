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

use crate::editor::writer::editor_vocabulary::{WRITER_INTERACTION_AST, WRITER_INTERACTION_GRANULARITY, writer_action, writer_chapter_payload, writer_io};
use crate::editor::writer::commands::set_camera;
use crate::editor::writer::commands::set_editor_selection;
use crate::editor::writer::commands::{commit_rename, format_document, open_document, set_active_example, load_document_json, set_text, text_edit, text_splice};
use crate::editor::writer::commands::{engagement_input, engagement_submit};
use crate::editor::writer::commands::{lint_document, request_completions};
use crate::editor::writer::commands::{set_font_px, set_line_height, set_tab_size, toggle_line_numbers};
use crate::editor::writer::modes::edit;
use crate::editor::writer::modes::edit::windows::main;
use crate::editor::writer::modes::edit::windows::main::config::{WriterMainWindowConfig, WriterMainWindowConfigMutation, WriterMainWindowConfigOwner};
use crate::editor::writer::modes::edit::windows::main::transient::{WriterMainWindowTransient, WriterMainWindowTransientMutation, WriterMainWindowTransientOwner};
use crate::editor::writer::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::writer::terminology::writer_play_labels;
use crate::op::WriterMutation;
use crate::{writer_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework::{kernel::Effect, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::Operation;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::app::{ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, EditorApp, EphemeralEmit, InteractionView};
use semio_framework_plugin::plugin_app_close_prelude::ArtifactDisposal;
use semio_framework_plugin::engagement_token_matches;
use semio_framework_plugin::strip_engagement_prefix;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::ContextMenuTextContext;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::WindowMeasure;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
pub use catalogue_panel::WRITER_PLAY_BODY_CATALOGUE;
pub use document_panel::WRITER_PLAY_BODY_ARTIFACT;
pub use inspection_panel::WRITER_PLAY_BODY_INSPECTION;
pub use main::{WRITER_PLAY_BODY_MAIN, WRITER_PLAY_WINDOW_KIND};

#[path = "🧵️command-admission-emission-and-retained-work/🦀️.rs"]
mod command_admission_emission_and_retained_work;
use command_admission_emission_and_retained_work::{
    writer_ast_topology, writer_context_menu_items, writer_hidden_operation, writer_hidden_view,
    WriterArtifactPreparationFactory, WriterCommandJobFactory, WriterRetainedCommandWork,
    MAX_WRITER_COMMAND_RAW_BYTES, MAX_WRITER_COMMAND_WORK_ITEMS, WRITER_COMMAND_TOOL_IDS,
};

#[path = "🎛️writer-play-app-editor-and-constructor/🦀️.rs"]
mod writer_play_app_editor_and_constructor;
pub use writer_play_app_editor_and_constructor::{create_writer_app, WriterPlayApp};

//#endregion 🔖️Constants

//#region 🔖️Interaction
//#endregion 🔖️Interaction

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `WriterPlayApp::Command` — the SOLE dispatch surface for writer's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies.
    /// (font/line-height/tab-size) sharing one manifest action id but three distinct wire keys and
    /// payload types — mirrors the pre-migration `WriterCommand::command_id()` match arm that mapped all
    /// three variants to the same `"setEditorSetting"` string. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WriterCommand for WriterSnapshot, WriterMutation, NoConfig, NoConfigMutation {
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "setText" as "set-text" => set_text::SetText,
        "openDocument" as "open-document" => open_document::OpenDocument,
        "loadDocumentJson" as "load-document-json" => load_document_json::LoadDocumentJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "formatDocument" as "format-document" => format_document::FormatDocument,
        "commitRename" as "commit-rename" => commit_rename::CommitRename,
        "setCamera" as "camera" => set_camera::SetCamera,
        "requestCompletions" as "request-completions" => request_completions::RequestCompletions,
        "lintDocument" as "lint-document" => lint_document::LintDocument,
        "textSelect" as "editor-selection" => set_editor_selection::SetEditorSelection,
        "toggleLineNumbers" as "toggle-line-numbers" => toggle_line_numbers::ToggleLineNumbers,
        "setEditorSetting" as "font-px" => set_font_px::SetFontPx,
        "setEditorSetting" as "line-height" => set_line_height::SetLineHeight,
        "setEditorSetting" as "tab-size" => set_tab_size::SetTabSize,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "textSplice" as "text-splice" => text_splice::TextSplice,
    }
}
//#endregion 🔖️Commands

//#region 🧪️UnitTests

#[cfg(test)]
#[path = "🧪️tests/🤖️agent-lane/🦀️.rs"]
mod agent_lane_tests;

//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{WRITER_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.document".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Text Document", "Textdokument"),
        source_format: WRITER_DOCUMENT_SCHEMA.into(),
        component_kind: "writer".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems5
