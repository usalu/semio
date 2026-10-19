//! 🏭️ Direct store factory and child group route: the direct store job factory and the child group retained route with its work, factory and tool job implementations. See [`super`].

use super::*;

//#region 🧵️DirectStoreLaneRoutes
/// 🧾️ The ONE execution contract the direct-Store route declares — returned by the factory and
/// proved by `FlowDirectStoreJobFactoryProofs`, so `validate_tool_job_rows`' `registration.contract
/// == row.contract` join can never drift from a hand-copied literal.
pub(super) fn flow_direct_store_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(FLOW_DIRECT_STORE_RAW_BYTES, 256, FLOW_STORE_MAX_MUTATION_ITEMS as u64, 16_384, 7_500, 1, 1)
}

pub struct FlowDirectStoreJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowDirectStoreJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_DIRECT_STORE_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowDirectStoreJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_direct_store_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_DIRECT_STORE_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Flow direct Store job rejects oversized wire owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowDirectStoreJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_DIRECT_STORE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnect", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setProximityDistance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setPreviewOff", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCatalogueSections", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleExtension", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
    ];
}
//#endregion 🧵️DirectStoreLaneRoutes

//#region 🧵️ChildGroupRetainedRoute
pub const FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget", "moveMediaNode", "nodeGraphEdit", "spotlightCommit"];

pub(super) const FLOW_CHILD_GROUP_RAW_BYTES: usize = 16_384;

pub struct FlowChildGroupWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub output: Option<Emit<FlowMutation, NoConfigMutation, NoDraftMutation>>,
    output_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<Emit<FlowMutation, NoConfigMutation, NoDraftMutation>>>,
    pub completed: bool,
    pub closing: bool,
}

impl FlowChildGroupWork {
    pub fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner: Some(instance_owner), output: None, output_retirement: None, completed: false, closing: false }
    }

    pub fn accept_output(&mut self,emit:Emit<FlowMutation,NoConfigMutation,NoDraftMutation>,child_id:&str)->Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>,Fault>{
        assert!(self.output.is_none(),"one Flow work unit retains at most one original output");
        self.output=Some(emit);
        let emit=self.output.as_ref().expect("original output retained before validation");
        let maximum=FLOW_STORE_MAX_MUTATION_ITEMS+1;
        let ready=emit.child_emits.first().is_some_and(|child|child.slot=="content"&&child.child_id==child_id&&!child.ops.is_empty()&&child.ops.len()<=maximum&&child.labels.len()==child.ops.len());
        let preparing=emit.child_preparations.front().is_some_and(|source|source.matches_source::<SemioFlowMutation>("content",child_id,maximum));
        let exact=(ready&&emit.child_preparations.is_empty())||(preparing&&emit.child_emits.is_empty());
        if emit.child_emits.len()+emit.child_preparations.len()>1
            || ((!emit.child_emits.is_empty()||!emit.child_preparations.is_empty())&&!exact)
            || (emit.transaction.is_some()&&!exact)
            || !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()
            || !emit.effects.is_empty()||!emit.events.is_empty()
        {return Err(Fault::from("flow-retained-child-group-output-contract"));}
        self.completed=true;
        Ok(ArtifactCommandWorkStep::Complete(self.output.take().expect("validated original output transferred once")))
    }

    fn admitted_child<'a>(
        command: &FlowCommand,
        snapshot: &'a FlowSnapshot,
        context: Option<&'a semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<store::os_store::snapshot_read::SnapshotReadRef<'a, SemioFlowSnapshot>> {
        if !matches!(command, FlowCommand::AddWidget(_) | FlowCommand::MoveMediaNode(_) | FlowCommand::NodeGraphEdit(_) | FlowCommand::SpotlightCommit(_)) {
            return None;
        }
        flow_content_child(snapshot, &context?.children)
    }

    fn payload_admitted(command: &FlowCommand) -> bool {
        match command {
            FlowCommand::AddWidget(payload) => {
                let text_bytes = payload.kind.len().checked_add(payload.neuron_kind.as_ref().map_or(0, String::len));
                text_bytes.is_some_and(|bytes| bytes <= FLOW_CHILD_GROUP_RAW_BYTES) && payload.x.unwrap_or(120.0).is_finite() && payload.y.unwrap_or(120.0).is_finite()
            }
            FlowCommand::MoveMediaNode(payload) => !payload.node_id.is_empty() && payload.node_id.len() <= FLOW_STORE_MAX_TEXT_BYTES && payload.x.is_finite() && payload.y.is_finite(),
            FlowCommand::NodeGraphEdit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
            FlowCommand::SpotlightCommit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
            _ => false,
        }
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowChildGroupWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != self.tool_id || !FLOW_CHILD_GROUP_TOOL_IDS.contains(&self.tool_id) || !Self::payload_admitted(command) {
            return None;
        }
        let child = Self::admitted_child(command, snapshot, context)?;
        (child.nodes.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && child.edges.len() <= FLOW_STORE_MAX_MUTATION_ITEMS).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction: _interaction, hover: _hover, context, operation } = *input;
        if self.closing || self.completed || self.output.is_some() {
            return Err(Fault::from("flow-retained-child-group-terminal"));
        }
        if command.command_id() != self.tool_id || !Self::payload_admitted(command) {
            return Err(Fault::from("flow-retained-child-group-payload-capacity"));
        }
        let _child = Self::admitted_child(command, snapshot, context).ok_or_else(|| Fault::from("flow-retained-child-group-authority"))?;
        let context = context.ok_or_else(|| Fault::from("flow-retained-child-group-context"))?;
        let view = ArtifactView::with_children(snapshot, history, (*context.children).clone()).bound_to_operation(operation.clone());
        let window_config = main::config::from_snapshot(context.window_config.as_ref());
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-child-group-instance-owner"))?;
        let emit = instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match command {
                FlowCommand::AddWidget(payload) => add_widget::handle(payload, &view, &ConfigView { snapshot: config, window: context.window_config.as_ref() }, session),
                FlowCommand::MoveMediaNode(payload) => move_media_node::handle(payload, &view, &ConfigView { snapshot: config, window: context.window_config.as_ref() }, session),
                FlowCommand::NodeGraphEdit(payload) => node_graph_edit::node_graph_edit_result(&view, &window_config, session, &payload.operations),
                FlowCommand::SpotlightCommit(payload) => spotlight_commit::node_graph_edit_result(&view, &window_config, session, &payload.operations),
                _ => Err(Fault::from("flow-retained-child-group-route-mismatch")),
            })?
        })?;
        self.accept_output(emit, &snapshot.content.child_id)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            use semio_framework_job::InteractiveJobCloseStep;
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if !self.closing {
                return Ok(InteractiveJobCloseStep::Blocked);
            }
            if let Some(owner) = self.output_retirement.as_mut() {
                let step = owner.step(grant);
                if owner.terminal_is_empty() {
                    self.output_retirement = None;
                }
                return Ok(match step {
                    Ok(RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Complete { progress },
                    Ok(RetainedCloneStep::Progress(progress)) => InteractiveJobCloseStep::Pending { progress },
                    Err(error) => return Err(error),
                });
            }
            if let Some(emit) = self.output.as_mut() {
                match emit.close_child_one(grant) {
                    Ok(Some(step)) => return Ok(InteractiveJobCloseStep::Pending { progress: step.progress().unwrap_or_default() }),
                    Err(_) => return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "interactive job close refused")),
                    Ok(None) => {}
                }
                if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                    return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
                }
                let emit = self.output.take().expect("observed original child group output");
                return Ok(match semio_framework_value::retirement::controlled::ControlledRetirement::new(emit) {
                    Ok(owner) => {
                        self.output_retirement = Some(owner);
                        InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
                    }
                    Err((error, emit)) => {
                        self.output = Some(emit);
                        return Err(error)
                    }
                });
            }
            if grant.maximum_items == 0 {
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            if self.instance_owner.take().is_some() {
                return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } });
            }
            Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(owner) = self.output_retirement.as_ref() {
            return owner.next_copy_byte_demand();
        }
        Ok(self.output.as_ref().map_or(Ok(None), |emit| emit.child_close_demands(0))?.map_or(0, |demand| demand.copy_bytes))
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(owner) = self.output_retirement.as_ref() {
            return owner.next_capacity_byte_demand(maximum_copy_bytes);
        }
        Ok(self.output.as_ref().map_or(Ok(None), |emit| emit.child_close_demands(maximum_copy_bytes))?.map_or(0, |demand| demand.capacity_bytes))
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(owner) = self.output_retirement.as_ref() {
            return owner.next_release_byte_demand();
        }
        Ok(self.output.as_ref().map_or(Ok(None), |emit| emit.child_close_demands(0))?.map_or(0, |demand| demand.release_bytes))
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(owner) = self.output_retirement.as_ref() {
            return owner.next_depth_demand();
        }
        match self.output.as_ref() {
            Some(emit) => Ok(emit.child_close_demands(0)?.map_or(1, |demand| demand.depth)),
            None => Ok(usize::from(self.instance_owner.is_some())),
        }
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none() && self.output.is_none() && self.output_retirement.is_none()
    }
}

pub struct FlowChildGroupJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowChildGroupJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_CHILD_GROUP_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowChildGroupJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(FLOW_CHILD_GROUP_RAW_BYTES, 256, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_CHILD_GROUP_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Flow child-group job rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowChildGroupJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CHILD_GROUP_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "spotlightCommit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}
//#endregion 🧵️ChildGroupRetainedRoute
