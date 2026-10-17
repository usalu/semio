//! 🗃️ Io and retained commands of the BIM editor: the io ports and the bounded command job factory. See [`super`].

use super::*;

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

pub(super) fn bim_bounded_extent(command: &BimCommand) -> Option<usize> {
    BIM_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

/// 🧵️ The single step of a bounded command: dispatches against the retained roots with the addressed window's config and the local presence, and publishes the presence the handler asked for.
pub(super) struct BimCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) completed: bool,
}

impl BimCommandWork {
    pub(super) fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
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
        let mut ctx = BimDispatchCtx::new(selection(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection(crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view, window_config, presence.as_ref());
        ctx.gestures = Some(self.owner.clone());
        ctx.window_transient = crate::editor::bim::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let emit = input.command.dispatch(&doc, &cfg, &mut ctx)?;
        crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::record_mutations(Some(&self.owner), input.snapshot, &emit.artifact_mutations);
        self.completed = true;
        let window_transient = match (ctx.transient_out.take(), view) {
            (Some(transient), Some(view)) => vec![crate::editor::bim::transient::addressed(view, transient)?],
            _ => Vec::new(),
        };
        Ok(if ctx.presence_out.is_empty() && window_transient.is_empty() { ArtifactCommandWorkStep::Complete(emit) } else { ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: ctx.presence_out, transient: Vec::new(), window_transient } } })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct BimCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl BimCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
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
        if input.declared_bytes() > BIM_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Bim command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BimCommandJobFactory {
    type Owner = EditorApp<BimModelApp>;
    const TOOL_IDS: &'static [&'static str] = BIM_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BIM_PUBLICATION_CONTRACTS;
}
//#endregion 🔖️RetainedCommands
