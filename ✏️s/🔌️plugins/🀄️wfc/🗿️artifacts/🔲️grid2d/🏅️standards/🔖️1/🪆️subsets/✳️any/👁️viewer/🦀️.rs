//! 👁️ 2D-grid viewer — the read-only counterpart of `✏️editor` for this subset. `Grid2dViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`, so this surface can never structurally emit
//! an artifact mutation. It must not import anything from the sibling mutation-capable surface
//! (`policyViewerPurityBreaches`).

use crate::viewer::grid2d::modes::view;
use crate::viewer::grid2d::modes::view::windows::preview;
use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer, ConfigView, Dialect,
    Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer, ViewerApp,
};

//#region 🔖️Command
/// 👁️ The viewer authors no verb of its own; every row here is a HOST-dispatched one it must still
/// declare, or the shell drops it (`refused: undeclared-action` on the navbar picker, and once per
/// pointer sample over the pane). Every route is inert: a viewer emits no mutation by construction,
/// and `ArtifactViewer` has no `build_document_store_initialization_job`, so it cannot admit a
/// whole-document replacement either — switching examples is an editor gesture.
#[derive(Clone, Debug, Default, PartialEq, Eq, dsl::DslOps, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid2dViewCommand {
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown,
    #[dsl(key = "canvas-pointer-move")]
    #[default]
    CanvasPointerMove,
    #[dsl(key = "canvas-pointer-up")]
    CanvasPointerUp,
    #[dsl(key = "canvas-double-click")]
    CanvasDoubleClick,
    #[dsl(key = "set-camera")]
    SetCamera,
}

impl protocol::OpBinary for Grid2dViewCommand {
    /// 🎯️ Without this the trait default (`["typed-command"]`) empties the expected proof set and
    /// every host dispatch at this read-only surface is refused `interactive-job.missing-factory`.
    const TOOL_JOB_IDS: &'static [&'static str] = GRID2D_VIEW_TOOL_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🏷️ Every manifest action id this viewer is dispatched with, in command row order.
pub const GRID2D_VIEW_TOOL_IDS: &[&str] = &["setActiveExample", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "setCamera"];

pub fn grid2d_view_command_id(command: &Grid2dViewCommand) -> &'static str {
    match command {
        Grid2dViewCommand::SetActiveExample { .. } => "setActiveExample",
        Grid2dViewCommand::CanvasPointerDown => "canvasPointerDown",
        Grid2dViewCommand::CanvasPointerMove => "canvasPointerMove",
        Grid2dViewCommand::CanvasPointerUp => "canvasPointerUp",
        Grid2dViewCommand::CanvasDoubleClick => "canvasDoubleClick",
        Grid2dViewCommand::SetCamera => "setCamera",
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedCommands
const GRID2D_VIEW_RETAINED_COMMAND_SCHEMA: &str = "s.wfc.grid2d/v1.view-tool-command.v1";
const GRID2D_VIEW_RETAINED_RAW_BYTES: usize = 8_192;

/// 🚦️ Nothing this viewer runs publishes into a store: the example switch rides a host effect and
/// every pointer verb is inert, so every route is `HostOnly`.
const GRID2D_VIEW_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasDoubleClick", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn grid2d_view_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GRID2D_VIEW_RETAINED_RAW_BYTES, 64, 1, 262_144, 7_500)
}

/// 📚️ The bundled example a picker row names. Restated rather than imported: this file must not
/// reach into the sibling mutation-capable surface.
fn view_example_document(example_id: &str) -> Option<Grid2dSnapshot> {
    match example_id {
        crate::examples::grid2d::pipes::ID => Some(crate::examples::grid2d::pipes::document()),
        crate::examples::grid2d::terrain::ID => Some(crate::examples::grid2d::terrain::document()),
        _ => None,
    }
}

/// 🔑️ The example id the navbar picker names, under any of the spellings it sends.
fn view_example_id(args: Option<&dsl::DslValue>) -> String {
    let dsl::DslValue::Object(entries) = args.unwrap_or(&dsl::DslValue::Null) else { return String::new() };
    ["exampleId", "id", "value"]
        .iter()
        .find_map(|key| entries.iter().find(|(name, _)| name == key).and_then(|(_, value)| if let dsl::DslValue::String(text) = value { Some(text.clone()) } else { None }))
        .unwrap_or_default()
}

struct Grid2dViewCommandWork {
    tool_id: &'static str,
    consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Grid2dViewer>> for Grid2dViewCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &Grid2dViewCommand,
        _snapshot: &Grid2dSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Grid2dViewer>>>,
    ) -> Option<usize> {
        GRID2D_VIEW_TOOL_IDS.contains(&grid2d_view_command_id(command)).then_some(1)
    }

    /// 🚧️ A picked example is VALIDATED here but not loaded: `ArtifactViewer` declares no
    /// `build_document_store_initialization_job`, so a viewer cannot admit the replacement
    /// envelope an `Effect::LoadDocument` carries. Declaring and accepting the verb is still what
    /// removes the shell's boot-time `undeclared-action` refusal; switching examples is an editor
    /// gesture.
    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Grid2dViewer>>) -> Result<ArtifactCommandWorkStep<ViewerApp<Grid2dViewer>>, Fault> {
        if self.consumed {
            return Err(Fault::from("wfc-grid2d-view-retained-work-repeated"));
        }
        self.consumed = true;
        if grid2d_view_command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc-grid2d-view-retained-route-mismatch"));
        }
        if let Grid2dViewCommand::SetActiveExample { example_id } = input.command {
            if !example_id.is_empty() && view_example_document(example_id).is_none() {
                return Err(Fault::from(format!("wfc-grid2d-view-unknown-example:{example_id}")));
            }
        }
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    }
}

pub struct Grid2dViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Grid2dViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GRID2D_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    fn register(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Grid2dViewer>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Self::new(&controller))
    }
}

impl semio_framework::ToolJobFactory for Grid2dViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Grid2dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Grid2dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        GRID2D_VIEW_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        grid2d_view_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > GRID2D_VIEW_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded wfc grid2d view command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Grid2dViewCommandJobFactory {
    type Owner = ViewerApp<Grid2dViewer>;
    const TOOL_IDS: &'static [&'static str] = GRID2D_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = GRID2D_VIEW_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Grid2dViewer;

impl ArtifactViewer for Grid2dViewer {
    type Snapshot = Grid2dSnapshot;
    type Mutation = Grid2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Grid2dViewCommand;

    const DIALECT: Dialect = WFC_GRID2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;

    /// 🗃️♻️ The same owned-store declaration the editor carries: the instance close ladder walks one
    /// store lane per stage and faults the whole close with `interactive-job.close-owned-disposer-missing`
    /// (or, one layer down, `artifact store has no owner-supplied bounded disposer`) the moment a lane
    /// answers `None`. A viewer never edits these stores, but it still owns and must release them.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 👁️ The viewer boots the subset's own committed default example, so editor and viewer share
    /// one scene instead of the viewer falling back to a hardcoded empty grid.
    fn initial_snapshot() -> Grid2dSnapshot {
        crate::examples::grid2d::pipes::document()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        Grid2dViewCommandJobFactory::register(registry)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Grid2dViewer>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.wfc.grid2d@1/*#viewer",
        artifact_schema: "s.wfc.grid2d",
        factory: "Grid2dViewCommandJobFactory",
        factory_type: Grid2dViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 64, 1, 262_144, 7_500),
        tools: ["setActiveExample", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "setCamera"]
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GRID2D_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = grid2d_view_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::from("wfc-grid2d-view-command-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            grid2d_view_command_id,
            GRID2D_VIEW_RETAINED_RAW_BYTES,
            1,
            Box::new(Grid2dViewCommandWork { tool_id, consumed: false }),
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        grid2d_view_command_id(command)
    }

    /// 👁️ The canvas host dispatches pointer and camera verbs at any `Canvas2d` surface, read-only
    /// or not, and the shell's navbar picker dispatches `setActiveExample`. Anything else is refused
    /// rather than silently swallowed, so a real typo still surfaces.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        Ok(match action {
            "setActiveExample" => Grid2dViewCommand::SetActiveExample { example_id: view_example_id(args) },
            "canvasPointerDown" => Grid2dViewCommand::CanvasPointerDown,
            "canvasPointerMove" => Grid2dViewCommand::CanvasPointerMove,
            "canvasPointerUp" => Grid2dViewCommand::CanvasPointerUp,
            "canvasDoubleClick" => Grid2dViewCommand::CanvasDoubleClick,
            "setCamera" => Grid2dViewCommand::SetCamera,
            other => return Err(Fault::from(format!("wfc-grid2d-viewer-unknown-action:{other}"))),
        })
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            preview::BODY_KEY => preview::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_grid2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WFC_GRID2D_DIALECT)
        .document(["semio", "wfc", "grid2d"])
        .icon_id("puzzle")
        .mode_def(view::definition())
        .default_mode_id(view::GRID2D_VIEW_MODE_ID)
        .window_kind_def(preview::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
