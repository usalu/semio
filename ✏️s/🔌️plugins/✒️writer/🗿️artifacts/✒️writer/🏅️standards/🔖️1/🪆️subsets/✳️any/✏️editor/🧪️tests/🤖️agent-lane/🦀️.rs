//! 🤖️ The writer editor's own tool-command job (`writer.writer.tool-command.v1`) on the agent lane: what the semio MCP gateway's
//! `action_prepare` previews for the verbs its coverage battery reaches on a writer document.

use semio_framework_plugin::app::EditorApp;
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use crate::editor::writer::editor_vocabulary::{WRITER_INTERACTION_AST, WRITER_INTERACTION_GRANULARITY, writer_action, writer_chapter_payload, writer_io};
#[allow(unused_imports)]
use crate::editor::writer::commands::set_camera;
#[allow(unused_imports)]
use crate::editor::writer::commands::set_editor_selection;
#[allow(unused_imports)]
use crate::editor::writer::commands::{commit_rename, format_document, open_document, set_active_example, load_document_json, set_text, text_edit, text_splice};
#[allow(unused_imports)]
use crate::editor::writer::commands::{engagement_input, engagement_submit};
#[allow(unused_imports)]
use crate::editor::writer::commands::{lint_document, request_completions};
#[allow(unused_imports)]
use crate::editor::writer::commands::{set_font_px, set_line_height, set_tab_size, toggle_line_numbers};
#[allow(unused_imports)]
use crate::editor::writer::modes::edit;
#[allow(unused_imports)]
use crate::editor::writer::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::editor::writer::modes::edit::windows::main::config::{WriterMainWindowConfig, WriterMainWindowConfigMutation, WriterMainWindowConfigOwner};
#[allow(unused_imports)]
use crate::editor::writer::modes::edit::windows::main::transient::{WriterMainWindowTransient, WriterMainWindowTransientMutation, WriterMainWindowTransientOwner};
#[allow(unused_imports)]
use crate::editor::writer::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
#[allow(unused_imports)]
use crate::editor::writer::terminology::writer_play_labels;
#[allow(unused_imports)]
use crate::op::WriterMutation;
#[allow(unused_imports)]
use crate::{writer_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_action_bus::RetainedToolWireInput;
#[allow(unused_imports)]
use semio_framework::{kernel::Effect, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
#[allow(unused_imports)]
use semio_framework_job::Operation;
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::app::{ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, EphemeralEmit, InteractionView};
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::ArtifactDisposal;
#[allow(unused_imports)]
use semio_framework_plugin::engagement_token_matches;
#[allow(unused_imports)]
use semio_framework_plugin::strip_engagement_prefix;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDescriptor;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppActionRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuItemSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuTextContext;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DomainTopology;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTopology;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::Media;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaError;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPayload;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
#[allow(unused_imports)]
use semio_framework_plugin::Menu;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::TopologyNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use pack::snapshot::ArtifactPack;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::writer::*;
use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_probe, declared_verb_wrote_document, probe_agent_lane, DeclaredVerbOutcome};

/// 🤖️ The verbs the semio MCP coverage battery (`plugin-coverage-check`) tries on a fresh writer document, in its order —
/// every one was refused `interactive-job.preview-unsupported` before the agent lane ran the editor's own job.
const BATTERY_VERBS: &[&str] = &["formatDocument", "commitRename", "setText"];

/// ⚖️ LAW: the agent lane runs the editor's own writer command job for every battery verb — none is unreachable there — and
/// `setText` previews the document write the shell publishes; the agent lane diverges from the shell lane on exactly
/// the pinned verbs.
#[semio_framework_async_macros::async_test]
async fn the_agent_lane_runs_the_writer_editors_own_tool_command_job() {
    let probes = probe_agent_lane::<semio_framework_plugin::EditorApp<WriterPlayApp>, <WriterPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(create_writer_app, None, BATTERY_VERBS, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    for probe in &probes {
        assert!(!matches!(&probe.agent, Some(DeclaredVerbOutcome::Unreachable { .. })), "{} is unreachable on the agent lane: {:?}", probe.verb, probe.agent);
    }
    let written = declared_verb_probe(&probes, "setText");
    assert!(declared_verb_wrote_document(written.agent.as_ref().expect("setText is agent-facing")), "setText previews its document write: {:?}", written.agent);
    assert_eq!(declared_verb_agent_divergences(&probes), Vec::<String>::new(), "agent-lane divergences");
}
