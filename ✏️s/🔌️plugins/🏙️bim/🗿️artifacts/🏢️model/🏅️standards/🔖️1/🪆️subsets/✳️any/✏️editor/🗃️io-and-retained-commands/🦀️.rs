//! 🗃️ Io and retained commands of the BIM editor: the io ports and the bounded command job factory. See [`super`].

use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use crate::editor::bim::{BIM_RETAINED_PAYLOAD_SCHEMA, BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS};
use crate::editor::bim::*;
use semio_s_artifact_bim_model_tests::editor::bim::presence;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient;

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface: the implicit document in/out ports only.
pub fn bim_io() -> AppIo {
    AppIo {
        artifact_schema: BIM_MODEL_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.bim-model".into(), name: "BIM Model".into(), dimension: "3d".into(), component_kind: "bim".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️RetainedCommands
fn bim_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

pub(crate) fn bim_bounded_extent(command: &BimCommand) -> Option<usize> {
    BIM_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

/// 🧵️ The single step of a bounded command: dispatches against the retained roots with the addressed window's config and the local presence, and publishes the presence the handler asked for.
pub(crate) struct BimCommandWork {
    pub(crate) tool_id: &'static str,
    pub(crate) owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(crate) completed: bool,
}

impl BimCommandWork {
    pub(crate) fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, completed: false }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for BimCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.completed && command.command_id() == self.tool_id).then(|| bim_bounded_extent(command)).flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("bim-command-work-terminal"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let window_config = input.context.and_then(|context| context.window_config.as_ref());
        let cfg = ConfigView { snapshot: input.config, window: window_config };
        let selection = |domain: &str| input.interaction.selection.get(domain).map(|selection| selection.ids.clone()).unwrap_or_default();
        let view = input.context.and_then(|context| context.view_state.as_ref());
        let presence = input.context.and_then(|context| context.presence_view()).map(|presence| presence.local.clone());
        let mut ctx = BimDispatchCtx::new(selection(semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection(semio_s_artifact_bim_model_interaction::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view, window_config, presence.as_ref());
        ctx.gestures = Some(self.owner.clone());
        ctx.window_transient = semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let emit = input.command.dispatch(&doc, &cfg, &mut ctx)?;
        semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::record_mutations(Some(&self.owner), input.snapshot, &emit.artifact_mutations);
        self.completed = true;
        let window_transient = match (ctx.transient_out.take(), view) {
            (Some(transient), Some(view)) => vec![semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::addressed(view, transient)?],
            _ => Vec::new(),
        };
        Ok(if ctx.presence_out.is_empty() && window_transient.is_empty() { ArtifactCommandWorkStep::Complete(emit) } else { ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: ctx.presence_out, transient: Vec::new(), window_transient } } })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(crate) struct BimCommandJobFactory {
    pub(crate) keys: Vec<ToolFactoryKey>,
}

impl BimCommandJobFactory {
    pub(crate) fn new(controller_id: &str) -> Self {
        Self { keys: BIM_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for BimCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BimModelApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BimModelApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        BIM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        bim_bounded_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: BIM_BOUNDED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BimCommandJobFactory {
    type Owner = EditorApp<BimModelApp>;
    const TOOL_IDS: &'static [&'static str] = BIM_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BIM_PUBLICATION_CONTRACTS;
}
//#endregion 🔖️RetainedCommands
