//! ⚖️ Every verb the architect editor declares honours its declaration — the framework's declared-verb
//! law (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, on the
//! sample program the editor boots with.

use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use crate::{editor::architect::catalog::adjacency_kind_from_id, editor::architect::catalog::analysis_kind_picker_options, editor::architect::catalog::parse_entity_id, editor::architect::catalog::parse_entity_id_from_args, editor::architect::catalog::parse_register_id, editor::architect::catalog::report_kind_picker_options, editor::architect::catalog::REGISTER_IDS};
#[allow(unused_imports)]
use crate::{editor::architect::commands::adjacency::set_adjacency_field, editor::architect::commands::adjacency::set_adjacency_filter, editor::architect::commands::adjacency::set_adjacency_kind};
#[allow(unused_imports)]
use crate::{editor::architect::commands::analysis::run_analysis, editor::architect::commands::analysis::run_report, editor::architect::commands::analysis::run_validation};
#[allow(unused_imports)]
use crate::{editor::architect::commands::element::add_element, editor::architect::commands::element::remove_element};
#[allow(unused_imports)]
use crate::editor::architect::commands::example::set_active_example;
#[allow(unused_imports)]
use crate::{editor::architect::commands::exchange::export_program, editor::architect::commands::exchange::export_registers_csv, editor::architect::commands::exchange::import_program, editor::architect::commands::exchange::import_program_request, editor::architect::commands::exchange::import_registers_csv, editor::architect::commands::exchange::import_registers_csv_request};
#[allow(unused_imports)]
use crate::{editor::architect::commands::graph::node_graph_edit, editor::architect::commands::graph::node_graph_viewport};
#[allow(unused_imports)]
use crate::{editor::architect::commands::register::add_register_item, editor::architect::commands::register::patch_register_item, editor::architect::commands::register::remove_register_item, editor::architect::commands::register::select_register};
#[allow(unused_imports)]
use crate::editor::architect::commands::search::query;
#[allow(unused_imports)]
use crate::editor::architect::commands::template::apply;
#[allow(unused_imports)]
use crate::{editor::architect::config::ArchitectConfig, editor::architect::config::ArchitectConfigMutation};
#[allow(unused_imports)]
use crate::editor::architect::modes::edit as edit_mode;
#[allow(unused_imports)]
use crate::{editor::architect::modes::edit::windows::adjacency as adjacency_window, editor::architect::modes::edit::windows::graph as graph_window, editor::architect::modes::edit::windows::register as register_window, editor::architect::modes::edit::windows::report as report_window, editor::architect::modes::edit::windows::trace as trace_window};
#[allow(unused_imports)]
use crate::{editor::architect::modes::report as report_mode, editor::architect::modes::review as review_mode};
#[allow(unused_imports)]
use crate::{editor::architect::panels::catalogue as catalogue_panel, editor::architect::panels::document as document_panel, editor::architect::panels::inspection as inspection_panel};
#[allow(unused_imports)]
use crate::{editor::architect::presence::ArchitectPresence, editor::architect::presence::ArchitectPresenceMutation};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::registers::catalog::RegisterRow;
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::snapshot::sample_plugin, standards::v1::subsets::any::schema::snapshot::ProgramSnapshot, standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA};
#[allow(unused_imports)]
use semio_framework_value::DslValue as Value;
#[allow(unused_imports)]
use semio_framework_plugin::{app::ArtifactOwnedToolJobContext, app::ArtifactOwnedToolJobRequest, app::Dialect, app::InteractionView};
#[allow(unused_imports)]
use semio_framework_plugin::{retained_command::ArtifactCommandWork, retained_command::ArtifactRetainedCommandInputs, retained_command::ArtifactRetainedCommandJob, retained_command::ArtifactRetainedCommandPayload, retained_command::BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
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
use semio_framework_plugin::HistoryView;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::editor::architect::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_probe};

/// ⚖️ LAW: all 23 declared verbs honour their declarations. The agent lane runs the same retained job the shell runs
/// and refuses by name (`interactive-job.agent-lane-uncarried`) the five whose result an agent transaction cannot carry
/// yet: the example switch (a host `LoadDocument`), the two exports (a file download) and the two import pickers (a
/// host file request) — routed to the MCP gateway (ticket 26/09/23 `wp-p8.md` § routed). The pin turns red when a
/// carrier lands.
#[semio_framework_async_macros::async_test]
async fn every_declared_architect_verb_honours_its_declaration() {
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_architect_app, None, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    assert_eq!(probes.len(), 23, "the architect editor declares twenty-three verbs");
    for verb in ["runAnalysis", "runReport"] {
        assert!(declared_verb_ever_wrote_document(declared_verb_probe(&probes, verb)), "{verb} stores its record in the program");
    }
    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importRegistersCsvRequest"], "agent-lane divergences");
}
