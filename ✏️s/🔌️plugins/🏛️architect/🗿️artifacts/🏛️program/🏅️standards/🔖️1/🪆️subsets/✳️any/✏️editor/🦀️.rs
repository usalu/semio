//! 🏛️ Architect editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the five window
//! surfaces in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, view state in
//! `🦀️config.rs`, presentation factories in `🦀️chrome.rs`, the register bridge in `🦀️catalog.rs`;
//! pure derived reads over the document live in the artifact's own `🧬️schema` / `🧬️schema/💡️inferences`
//! (see `//#region 🔧️Behavior` below for the app-scoped, `&mut`-taking counterpart).

use crate::editor::architect::catalog::{adjacency_kind_from_id, analysis_kind_picker_options, parse_entity_id, parse_entity_id_from_args, parse_register_id, report_kind_picker_options, REGISTER_IDS};
use crate::editor::architect::commands::adjacency::{set_adjacency_field, set_adjacency_filter, set_adjacency_kind};
use crate::editor::architect::commands::analysis::{run_analysis, run_report, run_validation};
use crate::editor::architect::commands::element::{add_element, remove_element};
use crate::editor::architect::commands::example::set_active_example;
use crate::editor::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv, import_registers_csv_request};
use crate::editor::architect::commands::graph::{node_graph_edit, node_graph_viewport};
use crate::editor::architect::commands::register::{add_register_item, patch_register_item, remove_register_item, select_register};
use crate::editor::architect::commands::search::query;
use crate::editor::architect::commands::template::apply;
use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};
use crate::editor::architect::modes::edit as edit_mode;
use crate::editor::architect::modes::edit::windows::{adjacency as adjacency_window, graph as graph_window, register as register_window, report as report_window, trace as trace_window};
use crate::editor::architect::modes::{report as report_mode, review as review_mode};
use crate::editor::architect::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::architect::presence::{ArchitectPresence, ArchitectPresenceMutation};
use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::registers::catalog::RegisterRow;
use crate::standards::v1::subsets::any::schema::{snapshot::{sample_plugin, ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA}};
// 🚧️ `Dialect`/`InteractionView` are only reachable through `app`, not yet in the crate-root
// re-export list (see the identical note in the sibling viewer surface's root `🦀️.rs`).
use semio_framework_value::DslValue as Value;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, ArtifactOwnedToolJobRequest, Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HistoryView;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_2d::compute::EngineHandles;

pub use semio_s_artifact_architect_program_snapshot::editor::architect::base;
pub use base::*;

//#region 🔧️Behavior
pub use semio_s_artifact_architect_program_panels::editor::architect::behavior;
pub use behavior::*;
//#endregion 🔧️Behavior

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ B1: `ArchitectPlayApp::Command` — the sole typed dispatch surface, one row per action declared
    /// on `create_architect_app`'s `AppBuilder`. Row order IS the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break. Each row's first literal is the camelCase manifest action
    /// id (`command_id()`); the second is the kebab `#[dsl(key)]` wire keyword the codec uses — both are
    /// copied verbatim off the pre-migration `ArchitectCommand` enum, never derived from one another.
    ///
    /// JSON blob arguments (patches, CSV, DSL payloads, and node-graph edit lists) stay
    /// `String`-typed and are parsed inside each handler. The shared `Viewport2d` record has its own
    /// `dsl::DslField` binding and therefore crosses this boundary as a nested typed value.
    pub enum ArchitectCommand for ProgramSnapshot, ProgramMutation, ArchitectConfig, ArchitectConfigMutation {
        "selectRegister" as "select-register" => select_register::SelectRegister,
        "addRegisterItem" as "add-register-item" => add_register_item::AddRegisterItem,
        "removeRegisterItem" as "remove-register-item" => remove_register_item::RemoveRegisterItem,
        "patchRegisterItem" as "patch-register-item" => patch_register_item::PatchRegisterItem,
        "setAdjacencyField" as "set-adjacency-field" => set_adjacency_field::SetAdjacencyField,
        "applyTemplate" as "apply-template" => apply::ApplyTemplate,
        "exportRegistersCsv" as "export-registers-csv" => export_registers_csv::ExportRegistersCsv,
        "importRegistersCsv" as "import-registers-csv" => import_registers_csv::ImportRegistersCsv,
        "addElement" as "add-element" => add_element::AddElement,
        "removeElement" as "remove-element" => remove_element::RemoveElement,
        "runValidation" as "run-validation" => run_validation::RunValidation,
        "runAnalysis" as "run-analysis" => run_analysis::RunAnalysis,
        "runReport" as "run-report" => run_report::RunReport,
        "exportProgram" as "export-program" => export_program::ExportProgram,
        "importProgramRequest" as "import-program-request" => import_program_request::ImportProgramRequest,
        "importProgram" as "import-program" => import_program::ImportProgram,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setAdjacencyKind" as "set-adjacency-kind" => set_adjacency_kind::SetAdjacencyKind,
        "search" as "search" => query::Search,
        "setAdjacencyFilter" as "set-adjacency-filter" => set_adjacency_filter::SetAdjacencyFilter,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "importRegistersCsvRequest" as "import-registers-csv-request" => import_registers_csv_request::ImportRegistersCsvRequest,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedWindowCommands
/// 🧵️ The document verbs that reach the shell. `validate_ui_dispatch_classification` admits ONLY
/// `Migrated`, so every id here has to carry the full retained catalogue — a `TOOL_IDS` row, a
/// `PUBLICATION_CONTRACTS` row naming exactly the lane its handler emits (`Artifact`, since each of
/// these returns `Emit::mutations`), an extent arm and a `bounded_first_step_tool_proofs!` `tools:`
/// entry — or the guest fails its own `interactive-job.catalog-incomplete` boot check instead.
pub(crate) const ARCHITECT_DOCUMENT_TOOL_IDS: &[&str] =
    &["addElement", "removeElement", "addRegisterItem", "removeRegisterItem", "patchRegisterItem", "applyTemplate", "setAdjacencyKind", "setAdjacencyField", "nodeGraphEdit", "setActiveExample"];
pub(crate) const ARCHITECT_RETAINED_TOOL_IDS: &[&str] = &[
    "selectRegister",
    "setAdjacencyFilter",
    "nodeGraphViewport",
    "addElement",
    "removeElement",
    "addRegisterItem",
    "removeRegisterItem",
    "patchRegisterItem",
    "applyTemplate",
    "setAdjacencyKind",
    "setAdjacencyField",
    "nodeGraphEdit",
    "setActiveExample",
];
const ARCHITECT_WINDOW_PAYLOAD_SCHEMA: &str = "architect.program.window-command.v1";
const ARCHITECT_WINDOW_RAW_BYTES: usize = 4_096;

fn architect_window_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ARCHITECT_WINDOW_RAW_BYTES, 32, 1, 4_096, 7_500)
}

fn architect_window_extent(command: &ArchitectCommand, _snapshot: &ProgramSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        ArchitectCommand::SelectRegister(payload) if payload.register_id.len() <= 256 => Some(1),
        ArchitectCommand::SetAdjacencyFilter(payload) if payload.kind.as_ref().is_none_or(|kind| kind.len() <= 256) => Some(1),
        ArchitectCommand::NodeGraphViewport(payload) if payload.viewport.validate().is_ok() => Some(1),
        _ => None,
    }
}

/// 🧵️ One-shot extent for the document verbs — each is a single bounded edit, so the retained job
/// is always exactly one step. `nodeGraphEdit`'s operation payload is the only unbounded input, and
/// it is capped at the same raw-wire budget the factory admits.
fn architect_document_extent(command: &ArchitectCommand, _snapshot: &ProgramSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        ArchitectCommand::AddElement(_)
        | ArchitectCommand::RemoveElement(_)
        | ArchitectCommand::AddRegisterItem(_)
        | ArchitectCommand::RemoveRegisterItem(_)
        | ArchitectCommand::PatchRegisterItem(_)
        | ArchitectCommand::ApplyTemplate(_)
        | ArchitectCommand::SetAdjacencyKind(_)
        | ArchitectCommand::SetAdjacencyField(_) => Some(1),
        ArchitectCommand::SetActiveExample(payload) if payload.example_id.len() <= 256 => Some(1),
        ArchitectCommand::NodeGraphEdit(payload) if payload.operations_json.len() <= ARCHITECT_WINDOW_RAW_BYTES => Some(1),
        _ => None,
    }
}

/// 🧵️ One-shot reducer for the document verbs — the retained-tool shape of the very dispatch the
/// app already performs in `handle`, so the bounded job and the batch path stay the same code.
#[expect(clippy::too_many_arguments, reason = "Implements the framework retained command reducer callback signature.")]
fn architect_document_reduce(
    command: &ArchitectCommand,
    snapshot: &ProgramSnapshot,
    config: &ArchitectConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&ArtifactOwnedToolJobContext<EditorApp<ArchitectPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    command.dispatch(&doc, &ConfigView { snapshot: config, window: None })
}

fn architect_window_emit(command: &ArchitectCommand, view: Option<&semio_framework_plugin::ViewModel>) -> Result<Option<Emit<ProgramMutation, ArchitectConfigMutation, NoDraftMutation>>, Fault> {
    let emit = match command {
        ArchitectCommand::SelectRegister(payload) => {
            let view = view.ok_or_else(|| Fault::from("architect-register-window-context-required"))?;
            Emit { window_config_mutations: vec![register_window::config::addressed(view, payload.register_id.clone())?], ..Default::default() }
        }
        ArchitectCommand::SetAdjacencyFilter(payload) => {
            let view = view.ok_or_else(|| Fault::from("architect-adjacency-window-context-required"))?;
            let filter = payload.kind.as_deref().and_then(adjacency_kind_from_id);
            Emit { window_config_mutations: vec![adjacency_window::config::addressed(view, filter)?], ..Default::default() }
        }
        ArchitectCommand::NodeGraphViewport(payload) => {
            let view = view.ok_or_else(|| Fault::from("architect-graph-window-context-required"))?;
            Emit { window_config_mutations: vec![graph_window::config::addressed(view, payload.viewport)?], ..Default::default() }
        }
        _ => return Ok(None),
    };
    Ok(Some(emit))
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework retained command reducer callback signature.")]
fn architect_window_reduce(
    command: &ArchitectCommand,
    _snapshot: &ProgramSnapshot,
    _config: &ArchitectConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&ArtifactOwnedToolJobContext<EditorApp<ArchitectPlayApp>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, NoDraftMutation>, Fault> {
    architect_window_emit(command, context.and_then(|context| context.view_state.as_ref()))?.ok_or_else(|| Fault::from("architect-window-command-route-mismatch"))
}

pub(crate) struct ArchitectWindowCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ArchitectWindowCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ARCHITECT_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for ArchitectWindowCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ArchitectPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ArchitectPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ARCHITECT_WINDOW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        architect_window_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > ARCHITECT_WINDOW_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Architect window command rejects oversized wire or a checkpoint"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for ArchitectWindowCommandJobFactory {
    type Owner = EditorApp<ArchitectPlayApp>;
    const TOOL_IDS: &'static [&'static str] = ARCHITECT_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "selectRegister", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setAdjacencyFilter", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addElement", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeElement", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addRegisterItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeRegisterItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchRegisterItem", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "applyTemplate", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAdjacencyKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAdjacencyField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

struct ArchitectWindowCommandJobFactoryProofs;

impl ArchitectWindowCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ArchitectPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.architect.program@1/*#editor",
        artifact_schema: "architect.program",
        factory: "ArchitectWindowCommandJobFactory",
        factory_type: ArchitectWindowCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(4_096, 32, 1, 4_096, 7_500),
        tools: [
            "selectRegister",
            "setAdjacencyFilter",
            "nodeGraphViewport",
            "addElement",
            "removeElement",
            "addRegisterItem",
            "removeRegisterItem",
            "patchRegisterItem",
            "applyTemplate",
            "setAdjacencyKind",
            "setAdjacencyField",
            "nodeGraphEdit",
            "setActiveExample"
        ]
    }
}
//#endregion 🧵️RetainedWindowCommands

//#region 🧵️RetainedExchangeCommands
/// 🧵️ The analysis and exchange verbs. Imports carry a whole CSV or ProgramSnapshot DSL body and exports,
/// analyses and reports publish whole records, so they run on their own factory with the public
/// invocation wire budget instead of the 4 KiB window budget. Every verb here is one bounded step.
pub(crate) const ARCHITECT_EXCHANGE_TOOL_IDS: &[&str] = &["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsvRequest", "importRegistersCsv"];
const ARCHITECT_EXCHANGE_PAYLOAD_SCHEMA: &str = "architect.program.exchange-command.v1";
const ARCHITECT_EXCHANGE_RAW_BYTES: usize = semio_framework_plugin::PUBLIC_INVOCATION_BODY_BYTES;

fn architect_exchange_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ARCHITECT_EXCHANGE_RAW_BYTES, 32, 1, ARCHITECT_EXCHANGE_RAW_BYTES, 7_500)
}

fn architect_exchange_extent(command: &ArchitectCommand, _snapshot: &ProgramSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    match command {
        ArchitectCommand::RunValidation(_)
        | ArchitectCommand::RunAnalysis(_)
        | ArchitectCommand::RunReport(_)
        | ArchitectCommand::Search(_)
        | ArchitectCommand::ExportProgram(_)
        | ArchitectCommand::ExportRegistersCsv(_)
        | ArchitectCommand::ImportProgramRequest(_)
        | ArchitectCommand::ImportRegistersCsvRequest(_) => Some(1),
        ArchitectCommand::ImportProgram(payload) if payload.payload.len() <= ARCHITECT_EXCHANGE_RAW_BYTES => Some(1),
        ArchitectCommand::ImportRegistersCsv(payload) if payload.payload.len() <= ARCHITECT_EXCHANGE_RAW_BYTES => Some(1),
        _ => None,
    }
}

/// 🧵️ One-shot reducer for the exchange verbs — the same dispatch `handle` performs, with the invoking
/// window handed to `runReport` so it opens the report it authored in that exact window.
#[expect(clippy::too_many_arguments, reason = "Implements the framework retained command reducer callback signature.")]
fn architect_exchange_reduce(
    command: &ArchitectCommand,
    snapshot: &ProgramSnapshot,
    config: &ArchitectConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&ArtifactOwnedToolJobContext<EditorApp<ArchitectPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    match command {
        ArchitectCommand::RunReport(payload) => run_report::handle_with_view(payload, &doc, context.and_then(|context| context.view_state.as_ref())),
        _ => command.dispatch(&doc, &ConfigView { snapshot: config, window: None }),
    }
}

pub(crate) struct ArchitectExchangeCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ArchitectExchangeCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ARCHITECT_EXCHANGE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for ArchitectExchangeCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ArchitectPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ArchitectPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ARCHITECT_EXCHANGE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        architect_exchange_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > ARCHITECT_EXCHANGE_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Architect exchange command rejects oversized wire or a checkpoint"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for ArchitectExchangeCommandJobFactory {
    type Owner = EditorApp<ArchitectPlayApp>;
    const TOOL_IDS: &'static [&'static str] = ARCHITECT_EXCHANGE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;
    /// 📤️ Validation and search publish their result on the Config lane; an analysis stores its record
    /// in the program and its result in the config; a report stores its record and opens it in the
    /// invoking report window; the exports, the file request and the imports hand the host a download,
    /// a file picker or the replacement document.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "runValidation", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "runAnalysis", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "runReport", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "search", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "exportProgram", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportRegistersCsv", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importProgramRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importProgram", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importRegistersCsvRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importRegistersCsv", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

struct ArchitectExchangeCommandJobFactoryProofs;

impl ArchitectExchangeCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ArchitectPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.architect.program@1/*#editor",
        artifact_schema: "architect.program",
        factory: "ArchitectExchangeCommandJobFactory",
        factory_type: ArchitectExchangeCommandJobFactory,
        contract: architect_exchange_contract(),
        tools: ["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsvRequest", "importRegistersCsv"]
    }
}
//#endregion 🧵️RetainedExchangeCommands

//#region 🔖️ArchitectPlayApp
/// 🧪️ B1: unit struct — every former `RefCell<ArchitectPlayRuntime>` field now lives in
/// `crate::editor::architect::config::ArchitectConfig`, written through `ArchitectConfigMutation`s.
#[derive(Default)]
pub struct ArchitectPlayApp;

impl ArtifactEditor for ArchitectPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    /// 🧩️ Composes `s.stdio.semio@v1/table` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🧬️ The loaded-parent child projection, read straight off the snapshot's own `#[child]` fields.
    /// Without it every live envelope load faults with `editor did not declare a loaded-parent child
    /// projection` before the decoded document can replace the store.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("architect.child-projection"), error.to_string()))
    }

    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Config = ArchitectConfig;
    type ConfigMutation = ArchitectConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ArchitectPresence;
    type PresenceMutation = ArchitectPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ArchitectCommand;

    const DIALECT: Dialect = crate::standards::v1::subsets::any::schema::ARCHITECT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::standards::v1::subsets::any::schema::snapshot::genesis_program_child_pack(snapshot, slot, child_id)
    
})())
}

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(
            semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default())
                .expect("default Architect presence is the exact empty terminal"),
        ))
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    /// 🧾️ BOTH factories' proofs, in registration order — `validate_tool_job_rows` matches this
    /// catalogue against the registered factories exactly.
    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = ArchitectWindowCommandJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(ArchitectExchangeCommandJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    /// 🗃️ The `Artifact` publication lane every document verb above declares is only SUPPORTED when
    /// the app owns a one-item artifact-store preparation factory; without it the guest classifies
    /// each of those tools `unsupported-publication-contract` and refuses the dispatch. The generic
    /// bounded factory is the dag/trinity precedent for an app whose mutations are plain bounded
    /// records (`✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️.rs`).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 🎚️ The `Config` lane validation, analysis and search publish their result on — supported only
    /// when the app owns a one-item config-store preparation factory. The config is one whole record
    /// carrying the last result text, so its bound is the exchange wire budget.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(ArchitectWindowCommandJobFactory::new(&controller))?;
        registry.register(ArchitectExchangeCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        let exchange_verb = ARCHITECT_EXCHANGE_TOOL_IDS.contains(&request.tool_id.as_str());
        if !ARCHITECT_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) && !exchange_verb {
            return Ok(None);
        }
        let document_verb = ARCHITECT_DOCUMENT_TOOL_IDS.contains(&request.tool_id.as_str());
        let extent = if exchange_verb {
            architect_exchange_extent
        } else if document_verb {
            architect_document_extent
        } else {
            architect_window_extent
        };
        if Self::command_id(&request.command) != request.tool_id || extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("architect-window-command-mismatch-or-capacity"));
        }
        let reduce = if exchange_verb {
            architect_exchange_reduce
        } else if document_verb {
            architect_document_reduce
        } else {
            architect_window_reduce
        };
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(Self::command_id(&request.command), reduce, extent));
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            Self::command_id,
            if exchange_verb { ARCHITECT_EXCHANGE_RAW_BYTES } else { ARCHITECT_WINDOW_RAW_BYTES },
            1,
            work,
        );
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::architect::config::schema::app_schema_descriptor())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<register_window::config::ArchitectRegisterWindowConfigOwner>()?;
        registry.register::<adjacency_window::config::ArchitectAdjacencyWindowConfigOwner>()?;
        registry.register::<graph_window::config::ArchitectGraphWindowConfigOwner>()?;
        registry.register::<report_window::config::ArchitectReportWindowConfigOwner>()
    }

    fn initial_snapshot() -> ProgramSnapshot {
        sample_plugin()
    }

    fn initial_config() -> ArchitectConfig {
        ArchitectConfig::default()
    }

    fn command_id(command: &ArchitectCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `ArchitectCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly (mirrors `gis2d`'s `command_from_action`).
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<ArchitectCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
        match action {
            "selectRegister" => Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: parse_register_id(args).unwrap_or_default() })),
            "addRegisterItem" => {
                Ok(ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), name: str_field("name").unwrap_or_else(|| "New Item".into()), template_id: str_field("templateId") }))
            }
            "removeRegisterItem" => {
                Ok(ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem { register_id: parse_register_id(args).unwrap_or_default(), entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default() }))
            }
            "patchRegisterItem" => Ok(ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                patch_json: args.and_then(|value| value.get("patch")).map_or_else(|| "null".into(), semio_framework_pack_json::to_json_string),
            })),
            "setAdjacencyField" => Ok(ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField {
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value_json: args.and_then(|value| value.get("value")).map_or_else(|| "null".into(), semio_framework_pack_json::to_json_string),
            })),
            "applyTemplate" => Ok(ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: parse_entity_id_from_args(args, "templateId").map(|id| id.0).unwrap_or_default() })),
            "exportRegistersCsv" => Ok(ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})),
            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { payload: str_field("payload").unwrap_or_default(), strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()) })),
            "importRegistersCsvRequest" => Ok(ArchitectCommand::ImportRegistersCsvRequest(import_registers_csv_request::ImportRegistersCsvRequest {})),
            "addElement" => Ok(ArchitectCommand::AddElement(add_element::AddElement { name: str_field("name").unwrap_or_else(|| "New Room".into()) })),
            "removeElement" => Ok(ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: str_field("elementId").or_else(|| str_field("id")).unwrap_or_default() })),
            "runValidation" => Ok(ArchitectCommand::RunValidation(run_validation::RunValidation {})),
            "runAnalysis" => Ok(ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: str_field("analysisKind").unwrap_or_default() })),
            "runReport" => Ok(ArchitectCommand::RunReport(run_report::RunReport { report_kind: str_field("reportKind").unwrap_or_default() })),
            "exportProgram" => Ok(ArchitectCommand::ExportProgram(export_program::ExportProgram {})),
            "importProgramRequest" => Ok(ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})),
            "importProgram" => Ok(ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "nodeGraphEdit" => {
                let args = args.ok_or_else(|| Fault::from("nodeGraphEdit needs its operation rows"))?;
                semio_framework_tool_machine::node_graph_edit_rows(args).map_err(Fault::from)?;
                Ok(ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: semio_framework_pack_json::to_json_string(args.get("operations").expect("the row decoder requires operations")) }))
            },
            // 🖼️ An ABSENT `viewport` is the Graph window's own default, not a fault: this action carries
            // no `action_args` row, so a palette/keybinding dispatch (and the framework's own
            // `assert_declared_actions_bridge_to_commands` law) hands the bridge no payload at all.
            // Only a PRESENT but malformed viewport is rejected.
            "nodeGraphViewport" => {
                let viewport = match args.and_then(|value| value.get("viewport")).cloned() {
                    Some(value) => <semio_framework_os_kernel::Viewport2d as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))?,
                    None => graph_window::config::ArchitectGraphWindowConfig::default().viewport,
                };
                Ok(ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport }))
            }
            "setAdjacencyKind" => Ok(ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind {
                element_a_id: parse_entity_id(args, "elementAId").map(|id| id.0).unwrap_or_default(),
                element_b_id: parse_entity_id(args, "elementBId").map(|id| id.0).unwrap_or_default(),
                kind: str_field("kind"),
                cycle: bool_field("cycle").unwrap_or(false),
            })),
            "search" => Ok(ArchitectCommand::Search(query::Search { query: str_field("query").unwrap_or_default() })),
            "setAdjacencyFilter" => Ok(ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: str_field("kind") })),
            "setActiveExample" => Ok(ArchitectCommand::SetActiveExample(set_active_example::SetActiveExample {
                example_id: str_field("exampleId").or_else(|| str_field("example_id")).or_else(|| str_field("id")).or_else(|| str_field("value")).unwrap_or_else(|| crate::examples::demo::ID.into()),
            })),
            other => Err(Fault::from(format!("architect: unhandled action id {other}"))),
        }
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no `ArchitectCommand` row reads
    /// the "program" domain's selection — the document panel's own tree click (its sole real pick
    /// surface) is intercepted by the framework's injected `interactionSelect` before it ever
    /// reaches `handle` (see `dispatch_action`'s reserved-verb interception), and every remaining
    /// command derives its ids from explicit args, not the live selection (mirrors `note`'s
    /// `app_commands!`-dispatched leaves that never needed `InteractionView` either).
    fn handle(
        command: &ArchitectCommand,
        doc: &ArtifactView<'_, ProgramSnapshot>,
        cfg: &ConfigView<'_, ArchitectConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(emit) = architect_window_emit(command, view_state)? {
            return Ok(emit);
        }
        match command {
            ArchitectCommand::RunReport(payload) => run_report::handle_with_view(payload, doc, view_state),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ProgramSnapshot>, cfg: &ConfigView<'_, ArchitectConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let program = doc.snapshot;
        match body_key {
            adjacency_window::ARCHITECT_BODY_ADJACENCY => adjacency_window::render(program, &adjacency_window::config::current(cfg), &semio_framework_plugin::TreeWindows::for_body(view_state, adjacency_window::ARCHITECT_BODY_ADJACENCY)),
            graph_window::ARCHITECT_BODY_GRAPH => graph_window::render(program, &graph_window::config::current(cfg)),
            register_window::ARCHITECT_BODY_REGISTER => register_window::render(program, &register_window::config::current(cfg)),
            report_window::ARCHITECT_BODY_REPORT => report_window::render(program, &report_window::config::current(cfg), view_state, &semio_framework_plugin::TreeWindows::for_body(view_state, report_window::ARCHITECT_BODY_REPORT)),
            trace_window::ARCHITECT_BODY_TRACE => trace_window::render(program, &semio_framework_plugin::TreeWindows::for_body(view_state, trace_window::ARCHITECT_BODY_TRACE)),
            document_panel::ARCHITECT_BODY_ARTIFACT => document_panel::render(program, &semio_framework_plugin::TreeWindows::for_body(view_state, document_panel::ARCHITECT_BODY_ARTIFACT)),
            catalogue_panel::ARCHITECT_BODY_CATALOGUE => catalogue_panel::render(&semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::ARCHITECT_BODY_CATALOGUE)),
            inspection_panel::ARCHITECT_BODY_INSPECTION => inspection_panel::render(program),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️ArchitectPlayApp

//#region 🔖️Manifest
pub fn create_architect_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::standards::v1::subsets::any::schema::ARCHITECT_DIALECT)
            .document(["semio", "architect"])
            .artifact_kind(crate::standards::v1::subsets::any::schema::artifact_kind())
            .icon_id("architect")
            .mode_def(edit_mode::definition())
            .mode_def(review_mode::definition())
            .mode_def(report_mode::definition())
            .default_mode_id(edit_mode::ARCHITECT_MODE_EDIT)
            .window_kind_def(adjacency_window::definition())
            .window_kind_def(graph_window::definition())
            .window_kind_def(register_window::definition())
            .window_kind_def(report_window::definition())
            .window_kind_def(trace_window::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("setAdjacencyKind", LocalizedLabel::native("Set Adjacency Kind", "Adjazenzart festlegen"))
            .mutation("addRegisterItem", LocalizedLabel::native("Add Register Item", "Registereintrag hinzufügen"))
            .mutation("removeRegisterItem", LocalizedLabel::native("Remove Register Item", "Registereintrag entfernen"))
            .action_destructive("removeRegisterItem")
            .mutation("patchRegisterItem", LocalizedLabel::native("Patch Register Item", "Registereintrag patchen"))
            .mutation("importProgram", LocalizedLabel::native("Import ProgramSnapshot", "Programm importieren"))
            .mutation("importRegistersCsv", LocalizedLabel::native("Import Registers CSV", "Register CSV importieren"))
            .mutation("applyTemplate", LocalizedLabel::native("Apply Template", "Vorlage anwenden"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View, "camera"))
            .view_action("selectRegister", LocalizedLabel::native("Select Register", "Register wählen"))
            // 🧬️ These three are DOCUMENT tools (`ARCHITECT_DOCUMENT_TOOL_IDS`, publication lane
            // `Artifact`, handlers returning `Emit::mutations`), so they must be declared `Mutation`:
            // the mounted registry enforces kind discipline and refused every one of them with
            // `View-kind command '<id>' must not emit operations`, which is why they only ever worked
            // in the registry-less unit harness.
            .mutation("addElement", LocalizedLabel::native("Add Element", "Element hinzufügen"))
            .mutation("removeElement", LocalizedLabel::native("Remove Element", "Element entfernen"))
            .action_destructive("removeElement")
            .mutation("setAdjacencyField", LocalizedLabel::native("Set Adjacency Field", "Adjazenzfeld setzen"))
            .view_action("runValidation", LocalizedLabel::native("Run Validation", "Validierung ausführen"))
            .mutation("runAnalysis", LocalizedLabel::native("Run Analysis", "Analyse ausführen"))
            .mutation("runReport", LocalizedLabel::native("Run Report", "Bericht erzeugen"))
            .action_with(ActionDefinition::new("search", LocalizedLabel::native("Search", "Suchen"), ActionKind::View, "search"))
            // 📚️ The playground navbar dispatches `setActiveExample` for its fixture combobox on
            // boot. `try_build_definition` copies app-level actions onto every window kind, so an
            // app-level row is what stops the shell dropping it before dispatch (which it logged as
            // this app's only console *error*).
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_with(ActionDefinition::new("exportProgram", LocalizedLabel::native("Export ProgramSnapshot", "Programm exportieren"), ActionKind::Shell, "download"))
            .action_with(ActionDefinition::new("exportRegistersCsv", LocalizedLabel::native("Export Registers CSV", "Register CSV exportieren"), ActionKind::Shell, "download"))
            .action_with(ActionDefinition::new("importProgramRequest", LocalizedLabel::native("Import Program File", "Programmdatei importieren"), ActionKind::Shell, "upload"))
            .action_with(ActionDefinition::new("importRegistersCsvRequest", LocalizedLabel::native("Import Registers CSV File", "Register-CSV-Datei importieren"), ActionKind::Shell, "upload"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setAdjacencyFilter", LocalizedLabel::native("Set Adjacency Filter", "Adjazenzfilter setzen"), ActionKind::View) })
            .action_interactive_job("addElement", InteractiveJobClassification::Migrated)
            .action_interactive_job("addRegisterItem", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyTemplate", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportProgram", InteractiveJobClassification::Migrated)
            .action_destructive("exportProgram")
            .action_interactive_job("exportRegistersCsv", InteractiveJobClassification::Migrated)
            .action_destructive("exportRegistersCsv")
            .action_interactive_job("importProgram", InteractiveJobClassification::Migrated)
            .action_interactive_job("importProgramRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importRegistersCsvRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importRegistersCsv", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRegisterItem", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeElement", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeRegisterItem", InteractiveJobClassification::Migrated)
            .action_interactive_job("runAnalysis", InteractiveJobClassification::Migrated)
            .action_interactive_job("runReport", InteractiveJobClassification::Migrated)
            .action_interactive_job("runValidation", InteractiveJobClassification::Migrated)
            .action_interactive_job("search", InteractiveJobClassification::Migrated)
            .action_interactive_job("selectRegister", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAdjacencyField", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAdjacencyFilter", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAdjacencyKind", InteractiveJobClassification::Migrated)
            .action_args("selectRegister", vec![ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect())])
            .action_args(
                "addRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID")),
                ],
            )
            .action_args(
                "removeRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                ],
            )
            .action_args(
                "patchRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                    ActionArgDef::text("patch", LocalizedLabel::native("Patch JSON", "Patch-JSON")),
                ],
            )
            .action_args("applyTemplate", vec![ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID"))])
            .action_args("removeElement", vec![ActionArgDef::text("elementId", LocalizedLabel::native("Element Id", "Element-ID"))])
            .action_args("addElement", vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name"))])
            .action_args(
                "setAdjacencyField",
                vec![
                    ActionArgDef::text("entityId", LocalizedLabel::native("Adjacency Id", "Adjazenz-ID")),
                    ActionArgDef::text("field", LocalizedLabel::native("Field", "Feld")),
                    ActionArgDef::json_text("value", LocalizedLabel::native("Value JSON", "Wert-JSON")),
                ],
            )
            .action_args(
                "importRegistersCsv",
                vec![
                    ActionArgDef::text("payload", LocalizedLabel::native("CSV Text", "CSV-Text")),
                    ActionArgDef::select(
                        "strategy",
                        LocalizedLabel::native("Strategy", "Strategie"),
                        vec![
                            ActionArgOption::new("upsert", LocalizedLabel::native("Upsert", "Upsert")),
                            ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
                            ActionArgOption::new("skipDuplicates", LocalizedLabel::native("Skip Duplicates", "Duplikate überspringen")),
                        ],
                    ),
                ],
            )
            .action_args(
                "setAdjacencyKind",
                vec![
                    ActionArgDef::text("elementAId", LocalizedLabel::native("First Element Id", "Erste Element-ID")),
                    ActionArgDef::text("elementBId", LocalizedLabel::native("Second Element Id", "Zweite Element-ID")),
                    ActionArgDef::toggle("cycle", LocalizedLabel::native("Cycle Kind", "Art weiterschalten")),
                    ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    vec![
                        ActionArgOption::new("required", LocalizedLabel::native("Required", "Erforderlich")),
                        ActionArgOption::new("preferred", LocalizedLabel::native("Preferred", "Bevorzugt")),
                        ActionArgOption::new("optional", LocalizedLabel::native("Optional", "Optional")),
                        ActionArgOption::new("prohibited", LocalizedLabel::native("Prohibited", "Verboten")),
                    ],
                ),
                ],
            )
            .action_args("runAnalysis", vec![ActionArgDef::select("analysisKind", LocalizedLabel::native("Analysis", "Analyse"), analysis_kind_picker_options()).required()])
            .action_args("runReport", vec![ActionArgDef::select("reportKind", LocalizedLabel::native("Report", "Bericht"), report_kind_picker_options()).required()])
            .action_args("search", vec![ActionArgDef::text("query", LocalizedLabel::native("Query", "Suchanfrage"))])
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label())])])
            .action_args("importProgram", vec![ActionArgDef::text("payload", LocalizedLabel::native("ProgramSnapshot DSL", "Programm-DSL"))])
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "program" interaction
            // domain — one granularity ("entity") over the 68 flat registers, `HierarchyProvider::Flat`
            // (no parent/child nesting), single-select Pick/Replace only (the document panel's element
            // rows are the sole real pick surface today — see its own doc comment). Scoped to the graph
            // window (the node-graph canvas, this app's closest analog to a primary editing surface),
            // mirroring `dag`'s main window's identical declaration.
            .interaction(InteractionDefinition {
                id: ARCHITECT_INTERACTION_PROGRAM.into(),
                label: LocalizedLabel::native("Program", "Programm"),
                granularities: vec![GranularityDefinition { id: ARCHITECT_INTERACTION_GRANULARITY_ENTITY.into(), label: LocalizedLabel::native("Entity", "Entität"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .window_kind_interactions(graph_window::ARCHITECT_WINDOW_GRAPH, vec![InteractionRef::new(ARCHITECT_INTERACTION_PROGRAM)])
            .default_layout(edit_mode::layout())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
            // `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old `"sample"`/`"empty"` example
            // registrations and the no-op `.workflow("architect", "Architect", "data")` call are dropped
            // here (not silently: reported in the packet's migration report). The subset's own
            // `📚️examples/🎬️demo`/`📚️examples/🎬️demo-session` facets are the modern, role-agnostic
            // replacement surface for this.
            .action_describe("setAdjacencyKind", LocalizedLabel::native("Sets how two program elements must relate (required, preferred, optional or prohibited); without a kind it cycles to the next one, and cycling past prohibited removes the adjacency.", "Legt fest, wie zwei Programmelemente zueinander stehen (erforderlich, bevorzugt, optional oder verboten); ohne Art wird zur nächsten gewechselt, nach verboten entfällt die Adjazenz."))
            .action_describe("addRegisterItem", LocalizedLabel::native("Adds a new row with the given name to one register of the program (such as users, activities or elements), or applies the named template instead.", "Fügt einem Register des Programms (etwa Nutzer, Aktivitäten oder Elemente) eine neue Zeile mit dem angegebenen Namen hinzu oder wendet stattdessen die genannte Vorlage an."))
            .action_describe("removeRegisterItem", LocalizedLabel::native("Deletes one row by id from a register of the program; its data is gone unless the edit is undone.", "Löscht eine Zeile anhand ihrer Id aus einem Register des Programms; ihre Daten sind fort, sofern die Änderung nicht rückgängig gemacht wird."))
            .action_describe("patchRegisterItem", LocalizedLabel::native("Merges the fields of a JSON patch into one row of a register, leaving fields the patch does not name unchanged.", "Übernimmt die Felder eines JSON-Patches in eine Registerzeile; nicht genannte Felder bleiben unverändert."))
            .action_describe("importProgram", LocalizedLabel::native("Replaces the whole architectural program with one parsed from the given ProgramSnapshot DSL text; everything not in the text is discarded.", "Ersetzt das gesamte Raumprogramm durch eines, das aus dem angegebenen ProgramSnapshot-DSL-Text gelesen wird; alles, was nicht im Text steht, wird verworfen."))
            .action_describe("importRegistersCsv", LocalizedLabel::native("Reads register rows from CSV text into the program by upsert, skipping duplicates, or replacing the existing rows, and reloads the resulting program.", "Liest Registerzeilen aus CSV-Text in das Programm, per Upsert, unter Überspringen von Duplikaten oder unter Ersetzen der vorhandenen Zeilen, und lädt das Ergebnis neu."))
            .action_describe("applyTemplate", LocalizedLabel::native("Applies one of the program's sector or project templates by id, adding the rows the template defines to the registers.", "Wendet eine Branchen- oder Projektvorlage des Programms anhand ihrer Id an und fügt die von ihr definierten Zeilen den Registern hinzu."))
            .action_describe("selectRegister", LocalizedLabel::native("Shows the given register (for example stakeholders, users or activities) in the register window; only that window's view changes.", "Zeigt das angegebene Register (etwa Beteiligte, Nutzer oder Aktivitäten) im Registerfenster an; nur die Ansicht dieses Fensters ändert sich."))
            .action_describe("addElement", LocalizedLabel::native("Adds a new room-like program element with the given name to the program's elements register.", "Fügt dem Elementregister des Programms ein neues raumartiges Programmelement mit dem angegebenen Namen hinzu."))
            .action_describe("removeElement", LocalizedLabel::native("Deletes one program element by id together with every adjacency that touches it; an unknown id is refused.", "Löscht ein Programmelement anhand seiner Id samt aller Adjazenzen, die es berühren."))
            .action_describe("setAdjacencyField", LocalizedLabel::native("Sets one named field of one adjacency row to a JSON value, for example its rationale or strength.", "Setzt ein benanntes Feld einer Adjazenzzeile auf einen JSON-Wert, etwa ihre Begründung oder Stärke."))
            .action_describe("runValidation", LocalizedLabel::native("Checks the whole program against its validation rules and lists the issues in the review view; the program itself is not changed.", "Prüft das gesamte Programm gegen seine Validierungsregeln und listet die Befunde in der Prüfansicht; das Programm selbst ändert sich nicht."))
            .action_describe("runAnalysis", LocalizedLabel::native("Runs one program analysis (such as gap, conflict, capacity, cost or risk) and stores its result as a new analysis record in the program.", "Führt eine Programmanalyse aus (etwa Lücken, Konflikte, Kapazität, Kosten oder Risiko) und speichert ihr Ergebnis als neuen Analyseeintrag im Programm."))
            .action_describe("runReport", LocalizedLabel::native("Builds one report (such as executive summary, requirements matrix or adjacency matrix) from the current program, stores it as a report record and opens it in the report window.", "Erstellt einen Bericht (etwa Management-Zusammenfassung, Anforderungsmatrix oder Adjazenzmatrix) aus dem aktuellen Programm, speichert ihn als Berichtseintrag und öffnet ihn im Berichtsfenster."))
            .action_describe("search", LocalizedLabel::native("Searches every register of the program for a keyword, selects the best matches and remembers the query in the search history.", "Durchsucht alle Register des Programms nach einem Stichwort, wählt die besten Treffer aus und merkt sich die Anfrage im Suchverlauf."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole program with one of the plugin's bundled example programs, by example id.", "Ersetzt das gesamte Programm durch eines der mitgelieferten Beispielprogramme, anhand der Beispiel-Id."))
            .action_describe("exportProgram", LocalizedLabel::native("Writes the whole program as ProgramSnapshot DSL text to a downloaded .architect.dsl file on the user's machine.", "Schreibt das gesamte Programm als ProgramSnapshot-DSL-Text in eine heruntergeladene .architect.dsl-Datei auf dem Rechner des Nutzers."))
            .action_describe("importProgramRequest", LocalizedLabel::native("Asks the user to pick a ProgramSnapshot DSL file and then imports it with Import Program, replacing the whole program.", "Lässt den Nutzer eine ProgramSnapshot-DSL-Datei wählen und importiert sie dann mit Programm importieren, wobei das gesamte Programm ersetzt wird."))
            .action_describe("importRegistersCsvRequest", LocalizedLabel::native("Asks the user to pick a CSV file of register rows and then imports it with Import Registers CSV by upsert.", "Lässt den Nutzer eine CSV-Datei mit Registerzeilen wählen und importiert sie dann mit Register CSV importieren per Upsert."))
            .action_describe("exportRegistersCsv", LocalizedLabel::native("Writes every register of the program as CSV to a downloaded .registers.csv file on the user's machine.", "Schreibt alle Register des Programms als CSV in eine heruntergeladene .registers.csv-Datei auf dem Rechner des Nutzers."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("selectRegister", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("importProgram")
            .action_destructive("importRegistersCsv")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/⚖️declared-verbs/🦀️.rs"]
mod declared_verb_laws;

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod window_ownership_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts
