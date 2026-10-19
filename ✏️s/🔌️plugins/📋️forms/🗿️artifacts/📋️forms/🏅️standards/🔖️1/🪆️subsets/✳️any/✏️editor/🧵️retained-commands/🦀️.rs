//! 🧵️ Forms retained commands: the retained tool ids and limits, the window command work cursor and the bounded command job factory.

use super::*;

//#region 🧵️RetainedCommands
/// 🧾️ Every Forms tool id in `FormsCommand` declaration order, kept exact against the generated
/// command catalog, proof rows, and publication contracts by the exhaustiveness law below.
pub const FORMS_RETAINED_TOOL_IDS: &[&str] = &[
    "setTryValue",
    "setTryValues",
    "resetTry",
    "previousStep",
    "nextStep",
    "submit",
    "setContributions",
    "addStep",
    "patchStep",
    "removeStep",
    "moveStep",
    "updateForm",
    "addBlock",
    "removeBlock",
    "patchQuestions",
    "patchQuestionOptions",
    "addQuestionOption",
    "removeQuestionOption",
    "patchVectorField",
    "addVectorField",
    "removeVectorField",
    "moveBlock",
    "dropQuestionKind",
    "setSpecJson",
    "setActiveExample",
    "exportSnapshot",
    "setTryValueStep",
    "exportResponses",
    "discardResponse",
];

const FORMS_RETAINED_PAYLOAD_SCHEMA: &str = "forms.tool-command.v1";

pub const FORMS_RETAINED_RAW_BYTES: usize = 16_384;

pub const FORMS_STORE_MUTATION_MAXIMUM_BYTES: usize = store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;

pub fn forms_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FORMS_RETAINED_RAW_BYTES, 64, 64, 4_096, 7_500)
}

pub struct FormsWindowCommandWork {
    pub tool_id: &'static str,
    completed: bool,
    response_export: Option<crate::standards::v1::subsets::any::io::text::snapshot::response::export::ResponseExport>,
}

impl FormsWindowCommandWork {
    pub fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false, response_export: None } }
}

pub fn forms_try_view<'a>(view: &'a semio_framework_plugin::ViewModel, window_id: &str, window_kind_id: &str) -> Result<&'a semio_framework_plugin::ViewModel, Fault> {
    if view.window_id.as_deref() != Some(window_id)
        || window_kind_id != support::FORMS_PLAY_WINDOW_TRY
        || !view.window_instances.iter().any(|window| window.id == window_id && window.window_kind_id == window_kind_id)
    {
        return Err(Fault::from("forms-try-window-route-stale"));
    }
    Ok(view)
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<FormsPlayApp>> for FormsWindowCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(
        &self,
        command: &FormsCommand,
        snapshot: &FormsSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<FormsPlayApp>>>,
    ) -> Option<usize> {
        if self.completed || command.command_id() != self.tool_id { return None; }
        match command {
            FormsCommand::ExportResponses(_) => Some(crate::standards::v1::subsets::any::io::text::snapshot::response::export::ResponseExport::work_items(&snapshot.responses)),
            FormsCommand::SetTryValue(_)
            | FormsCommand::SetTryValues(_)
            | FormsCommand::SetTryValueStep(_)
            | FormsCommand::ResetTry(_)
            | FormsCommand::PreviousStep(_)
            | FormsCommand::NextStep(_)
            | FormsCommand::Submit(_) => context?.view_state.as_ref().map(|_| 1),
            _ => Some(1),
        }
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<FormsPlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<FormsPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id { return Err(Fault::from("forms-window-work-terminal")); }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let mut emit = Emit::default();
        let mut transient = None;
        match input.command {
            FormsCommand::ExportResponses(payload) => {
                if self.response_export.is_none() {
                    self.response_export = Some(crate::standards::v1::subsets::any::io::text::snapshot::response::export::ResponseExport::new(&payload.format).map_err(Fault::from)?);
                }
                let Some(data) = self.response_export.as_mut().expect("admitted response exporter").advance(&input.snapshot.responses) else {
                    return Ok(ArtifactCommandWorkStep::Progress { stage: "forms-export-responses", preview: br#"{"en":"Exporting answers","de":"Antworten werden exportiert"}"# });
                };
                emit = export_responses::download(payload, input.snapshot, data)?;
            }
            FormsCommand::SetTryValue(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let lease = try_window::transient::FormsTryWindowLease::capture(context.window_transient.as_ref())?;
                let output = set_try_value::start_window(payload, input.operation, &try_window::transient::from_snapshot(context.window_transient.as_ref()), &lease)?;
                emit = output.emit;
                transient = output.transient.map(|state| try_window::transient::addressed(view, state)).transpose()?;
            }
            FormsCommand::SetTryValues(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let lease = try_window::transient::FormsTryWindowLease::capture(context.window_transient.as_ref())?;
                let output = set_try_values::start_window(payload, input.operation, &try_window::transient::from_snapshot(context.window_transient.as_ref()), &lease)?;
                emit = output.emit;
                transient = output.transient.map(|state| try_window::transient::addressed(view, state)).transpose()?;
            }
            FormsCommand::SetTryValueStep(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let lease = try_window::transient::FormsTryWindowLease::capture(context.window_transient.as_ref())?;
                let output = set_try_value::advance_window(payload, input.operation, &try_window::transient::from_snapshot(context.window_transient.as_ref()), &lease)?;
                emit = output.emit;
                transient = output.transient.map(|state| try_window::transient::addressed(view, state)).transpose()?;
            }
            FormsCommand::ResetTry(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let lease = try_window::transient::FormsTryWindowLease::capture(context.window_transient.as_ref())?;
                let (output, config) = reset_try::handle_window(payload, input.operation, &lease)?;
                emit = output.emit;
                emit.window_config_mutations.extend(try_window::config::addressed(view, &try_window::config::from_snapshot(context.window_config.as_ref()), config)?);
                transient = output.transient.map(|state| try_window::transient::addressed(view, state)).transpose()?;
            }
            FormsCommand::PreviousStep(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let next = previous_step::handle_window(payload, &try_window::config::from_snapshot(context.window_config.as_ref()))?;
                emit.window_config_mutations.extend(try_window::config::addressed(view, &try_window::config::from_snapshot(context.window_config.as_ref()), next)?);
            }
            FormsCommand::NextStep(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let next = next_step::handle_window(payload, input.snapshot, &try_window::config::from_snapshot(context.window_config.as_ref()), &try_window::transient::from_snapshot(context.window_transient.as_ref()))?;
                emit.window_config_mutations.extend(try_window::config::addressed(view, &try_window::config::from_snapshot(context.window_config.as_ref()), next)?);
            }
            FormsCommand::Submit(payload) => {
                let context = input.context.ok_or_else(|| Fault::from("forms-try-window-context-required"))?;
                let view = forms_try_view(context.view_state.as_ref().ok_or_else(|| Fault::from("forms-try-window-view-required"))?, &payload.window_id, &payload.window_kind_id)?;
                let (output, config) = submit::handle_window(input.snapshot, &try_window::config::from_snapshot(context.window_config.as_ref()), &try_window::transient::from_snapshot(context.window_transient.as_ref()), input.operation.canonical_base_revision_hex())?;
                emit = output;
                emit.window_config_mutations.extend(try_window::config::addressed(view, &try_window::config::from_snapshot(context.window_config.as_ref()), config)?);
            }
            _ => emit = input.command.dispatch(&doc, &cfg)?,
        }
        self.completed = true;
        match transient {
            Some(mutation) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] },
            }),
            None => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        if let Some(work) = self.response_export.as_mut() { work.cancel(); }
    }

    fn terminal_is_empty(&self) -> bool {
        self.response_export.as_ref().is_none_or(|work| work.is_empty())
    }
}

pub struct FormsBoundedCommandJobFactory {
    pub keys: Vec<ToolFactoryKey>,
}

impl FormsBoundedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: FORMS_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FormsBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<FormsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<FormsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FORMS_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        forms_bounded_contract()
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
        if input.declared_bytes() > FORMS_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Forms retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FormsBoundedCommandJobFactory {
    type Owner = EditorApp<FormsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FORMS_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setTryValue", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "setTryValues", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "resetTry", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "previousStep", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "nextStep", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "submit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "updateForm", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchQuestions", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchQuestionOptions", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addQuestionOption", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeQuestionOption", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveBlock", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "dropQuestionKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSpecJson", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "exportSnapshot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setTryValueStep", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "exportResponses", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "discardResponse", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}
//#endregion 🧵️RetainedCommands
