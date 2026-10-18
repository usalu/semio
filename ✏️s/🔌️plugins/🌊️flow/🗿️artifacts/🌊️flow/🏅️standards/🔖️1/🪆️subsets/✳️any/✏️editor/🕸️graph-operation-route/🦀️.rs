//! 🕸️ Graph operation route: the graph operation tool ids, references, work and job factory and the job factory proofs of the retained routes. See [`super`].

use super::*;

//#region 🧵️GraphOperationRetainedRoute
/// 🕸️ The whole-graph routes: each runs one `FlowHost` operation (or the rename's pure snapshot
/// rewrite) against the composed scene and publishes ONE snapshot group on its content child. Retained rather
/// than batch-dispatched, because a `BatchOnlyPendingRewrite` classification on these six is what
/// faulted the entire app at construction with `interactive-job.catalog-authority` on every host
/// that instantiates the flow editor (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const FLOW_GRAPH_OPERATION_TOOL_IDS: &[&str] = &["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"];

/// 🧮️ The ONE capacity this route declares — its survey walks at most every widget of an admitted
/// scene, plus the single apply step (`📓️work-capacity-2026-09-10.md`).
pub const FLOW_GRAPH_OPERATION_CAPACITY: semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity =
    semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(FLOW_STORE_MAX_SCENE_ITEMS + 1);

/// 🕸️ The widget ids one graph operation references, resolved by the survey walk before the host
/// runs — `[source, target]` for `connectMediaPorts`, `[old, trimmed new]` for `renameFlowWidget`,
/// none for the whole-graph routes.
fn flow_graph_operation_references(command: &FlowCommand) -> [Option<&str>; 2] {
    match command {
        FlowCommand::ConnectMediaPorts(payload) => [Some(payload.source_node_id.as_str()), Some(payload.target_node_id.as_str())],
        FlowCommand::RenameFlowWidget(payload) => [Some(payload.old_id.as_str()), Some(payload.value.trim())],
        _ => [None, None],
    }
}

/// 🧾️ Every graph-operation payload bound this route admits, in one place — a payload over any of
/// them refuses at `extent` (`None`) instead of reaching the host.
fn flow_graph_operation_payload_admitted(command: &FlowCommand) -> bool {
    let text_bytes = flow_graph_operation_references(command).iter().flatten().map(|reference| reference.len()).fold(0, usize::saturating_add);
    if text_bytes > FLOW_STORE_MAX_TEXT_BYTES {
        return false;
    }
    match command {
        FlowCommand::RunExtensionAction(payload) => payload.action_id.len() <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::ConnectMediaPorts(payload) => payload.source_port_id.len().saturating_add(payload.target_port_id.len()) <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::SetActiveExample(payload) => payload.example_id.len() <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::Reorganize(_) | FlowCommand::RenameFlowWidget(_) => true,
        _ => false,
    }
}

pub(super) struct FlowGraphOperationWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub(super) cursor: usize,
    pub(super) replay_target: usize,
    resolved: [bool; 2],
    pub(super) completed: bool,
    pub(super) closing: bool,
}

impl FlowGraphOperationWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner: Some(instance_owner), cursor: 0, replay_target: 0, resolved: [false, false], completed: false, closing: false }
    }

    pub(super) fn replaying(&self) -> bool {
        self.cursor < self.replay_target
    }

    pub(super) fn apply(&self, command: &FlowCommand, composed: &FlowSnapshot, config: &FlowMainWindowConfig, _interaction: &protocol::InteractionState) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-graph-instance-owner"))?;
        let resolved = self.resolved;
        instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match command {
                FlowCommand::ConnectMediaPorts(payload) if resolved == [true, true] => connect_media_ports::connect_edit(payload, composed, config, session),
                FlowCommand::ConnectMediaPorts(payload) => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("connectMediaPorts found no widget \"{}\"", if resolved[0] { &payload.target_node_id } else { &payload.source_node_id }))),
                FlowCommand::Reorganize(_) => reorganize::reorganize_edit(composed, config, session),
                FlowCommand::RenameFlowWidget(payload) if resolved[0] && !resolved[1] && !payload.value.trim().is_empty() => rename_flow_widget::rename_edit(payload, composed),
                FlowCommand::RenameFlowWidget(payload) if !resolved[0] => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("renameFlowWidget found no widget \"{}\"", payload.old_id))),
                FlowCommand::RenameFlowWidget(payload) => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.widget-id-unavailable"), format!("renameFlowWidget cannot rename \"{}\" to \"{}\": the id is empty or taken", payload.old_id, payload.value.trim()))),
                FlowCommand::RunExtensionAction(payload) => run_extension_action::extension_action_result(payload, composed, config, session),
                FlowCommand::SetActiveExample(payload) => set_active_example::set_active_example_edit(payload),
                _ => Err(Fault::from("flow-retained-graph-route-mismatch")),
            })?
        })
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowGraphOperationWork {
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
        if self.closing || self.completed || command.command_id() != self.tool_id || !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&self.tool_id) || !flow_graph_operation_payload_admitted(command) {
            return None;
        }
        let child = flow_content_child(snapshot, &context?.children)?;
        if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return None;
        }
        FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(child.nodes.len().saturating_add(1))
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-graph-work-terminal"));
        }
        if command.command_id() != self.tool_id || !flow_graph_operation_payload_admitted(command) {
            return Err(Fault::from("flow-retained-graph-payload-capacity"));
        }
        let context = context.ok_or_else(|| Fault::from("flow-retained-graph-context-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
        if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return Err(Fault::from("flow-retained-scene-capacity"));
        }
        if let Some(node) = child.nodes.get(self.cursor) {
            for (slot, reference) in flow_graph_operation_references(command).into_iter().enumerate() {
                if reference.is_some_and(|reference| reference == node.id) {
                    self.resolved[slot] = true;
                }
            }
            self.cursor += 1;
            return Ok(if self.replaying() {
                ArtifactCommandWorkStep::Replay { stage: "flow-graph-survey-replay", preview: br#"{"en":"Restoring graph survey","de":"Graph-Erhebung wird wiederhergestellt"}"# }
            } else {
                ArtifactCommandWorkStep::Progress { stage: "flow-graph-survey", preview: br#"{"en":"Surveying the graph","de":"Graph wird erhoben"}"# }
            });
        }
        self.completed = true;
        let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
        self.apply(command, &composed, &config, interaction).map(ArtifactCommandWorkStep::Complete)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0=>Some(u8::from(self.completed)),1..=8=>Some(((self.cursor as u64)>>((index-1)*8))as u8),_=>None}}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 9 || checkpoint[0] > 1 {
            return Err(Fault::from("flow-retained-graph-checkpoint-invalid"));
        }
        self.completed = checkpoint[0] == 1;
        self.replay_target = usize::try_from(u64::from_le_bytes(checkpoint[1..9].try_into().map_err(|_| Fault::from("flow-retained-graph-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-graph-checkpoint-invalid"))?;
        self.cursor = 0;
        self.resolved = [false, false];
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || grant.maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.instance_owner.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(usize::from(self.instance_owner.is_some())) }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none()
    }
}

pub struct FlowGraphOperationJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowGraphOperationJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_GRAPH_OPERATION_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

pub fn flow_graph_operation_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(FLOW_GRAPH_OPERATION_RAW_BYTES, 256, FLOW_GRAPH_OPERATION_CAPACITY.work_items() as u64, 16_384, 7_500, 1, 1)
}

impl semio_framework::ToolJobFactory for FlowGraphOperationJobFactory {
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
        flow_graph_operation_contract()
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
        if input.declared_bytes() > FLOW_GRAPH_OPERATION_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Flow graph operation job rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowGraphOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_GRAPH_OPERATION_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameFlowWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "runExtensionAction", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}

pub(super) struct FlowGraphOperationJobFactoryProofs;

impl FlowGraphOperationJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowGraphOperationJobFactory",
        factory_type: FlowGraphOperationJobFactory,
        contract: flow_graph_operation_contract(),
        tools: ["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"]
    }
}
//#endregion 🧵️GraphOperationRetainedRoute

pub(super) struct FlowDirectStoreJobFactoryProofs;

impl FlowDirectStoreJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowDirectStoreJobFactory",
        factory_type: FlowDirectStoreJobFactory,
        contract: flow_direct_store_contract(),
        tools: [
            "removeWidget",
            "deleteSelection",
            "disconnect",
            "patchFlowWidgets",
            "duplicateWidget",
            "focusSelection",
            "nodeGraphViewport",
            "setLodMode",
            "setProximityDistance",
            "setGridVisible",
            "setGridSnapEnabled",
            "setGridFactor",
            "setPreviewOff",
            "setCatalogueSections",
            "toggleExtension",
            "addGeneration",
            "removeGeneration",
            "selectGeneration",
            "renameGeneration",
            "updateGenerationValues",
        ]
    }
}

pub(super) struct FlowHostEffectJobFactoryProofs;

impl FlowHostEffectJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowHostEffectJobFactory",
        factory_type: FlowHostEffectJobFactory,
        tools: {
            "evaluate" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "contextMenuAt" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "openSpotlight" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "replaceImage" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "flowEvalTick" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "flowEvalResolve" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
        }
    }
}

pub(super) struct FlowChildGroupJobFactoryProofs;

impl FlowChildGroupJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowChildGroupJobFactory",
        factory_type: FlowChildGroupJobFactory,
        tools: {
            "addWidget" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "moveMediaNode" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "spotlightCommit" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
        }
    }
}
