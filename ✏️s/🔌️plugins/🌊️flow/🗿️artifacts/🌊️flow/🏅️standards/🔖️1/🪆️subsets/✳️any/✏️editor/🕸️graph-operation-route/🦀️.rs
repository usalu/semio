//! 🕸️ Graph operation route: the graph operation tool ids, references, work and job factory and the job factory proofs of the retained routes. See [`super`].

use super::*;
use super::retained::{FlowOriginalIntake,FlowAuthoringCommandWork,FlowAddDescriptorWork};

//#region 🧵️GraphOperationRetainedRoute
/// 🕸️ The whole-graph routes: each runs one `FlowHost` operation (or the rename's pure snapshot
/// rewrite) against the composed scene and publishes ONE snapshot group on its content child. Retained rather
/// than batch-dispatched, because a `BatchOnlyPendingRewrite` classification on these six is what
/// faulted the entire app at construction with `interactive-job.catalog-authority` on every host
/// that instantiates the flow editor (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const FLOW_GRAPH_OPERATION_TOOL_IDS: &[&str] = &["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"];

/// 🧮️ The ONE capacity this route declares — its survey walks at most every widget of an admitted
/// scene, plus the single apply step (`📓️work-capacity-2026-09-10.md`).
pub const FLOW_GRAPH_OPERATION_CAPACITY: semio_plugin::retained_command::ArtifactRetainedWorkCapacity =
    semio_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(FLOW_STORE_MAX_SCENE_ITEMS + 1);

/// 🕸️ The widget ids one graph operation references, resolved by the survey walk before the host
/// runs — `[source, target]` for `connectMediaPorts`, `[old, trimmed new]` for `renameFlowWidget`,
/// none for the whole-graph routes.
fn flow_graph_operation_references(command: &FlowCommand) -> [Option<&str>; 2] {
    match command {
        FlowCommand::ConnectMediaPorts(payload) => [Some(payload.source_node_id.as_str()), Some(payload.target_node_id.as_str())],
        FlowCommand::RenameFlowWidget(payload) => [Some(payload.old_id.as_str()), Some(payload.value.trim())],
        FlowCommand::RemoveWidget(payload)=>[Some(&payload.widget_id),None],
        FlowCommand::Disconnect(payload)=>[Some(&payload.synapse_id),None],
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
        FlowCommand::Reorganize(_) | FlowCommand::RenameFlowWidget(_) | FlowCommand::RemoveWidget(_) | FlowCommand::Disconnect(_) | FlowCommand::AddWidget(_) | FlowCommand::NodeGraphEdit(_) => true,
        _ => false,
    }
}

pub(super) struct FlowGraphOperationWork {
    pub(super) tool_id: &'static str,
    pub(super) instance_owner: Option<semio_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub(super) cursor: usize,
    pub(super) replay_target: usize,
    resolved: [bool; 2],
    original:FlowOriginalIntake,
    pipeline:Option<FlowAuthoringCommandWork>,
    descriptor_work:Option<FlowAddDescriptorWork>,
    descriptor:Option<semio_framework_artifact_flow_flow::WidgetDescriptor>,
    descriptor_retirement:semio_framework_value::retained_clone::RetainedCloneClose,
    layout_options:infinite_canvas::board::schema::layout::DagLayoutOptions,
    instance_retirement:Option<semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement>,
    pub(super) completed: bool,
    pub(super) closing: bool,
}

impl FlowGraphOperationWork {
    pub(super) fn new(tool_id: &'static str, instance_owner: semio_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner: Some(instance_owner), cursor: 0, replay_target: 0, resolved: [false, false], original:FlowOriginalIntake::new(tool_id),pipeline:None,descriptor_work:None,descriptor:None,descriptor_retirement:Default::default(),layout_options:Default::default(),instance_retirement:None, completed: false, closing: false }
    }

    fn original_route(&self)->bool{matches!(self.tool_id,"connectMediaPorts"|"reorganize"|"removeWidget"|"disconnect"|"addWidget"|"nodeGraphEdit")}
    fn original_request<'a>(command:&'a FlowCommand,descriptor:Option<&'a semio_framework_artifact_flow_flow::WidgetDescriptor>,options:&'a infinite_canvas::board::schema::layout::DagLayoutOptions)->Result<(flow::FlowAuthoringRequest<'a>,super::child_publication::FlowLeafMode),semio_framework_value::ValueError>{
        use flow::FlowAuthoringRequest as R;use super::child_publication::FlowLeafMode as M;let invalid=||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original graph authoring request has an unsupported native operation");
        Ok(match command{FlowCommand::ConnectMediaPorts(p)=>(R::ConnectPorts{from:&p.source_node_id,from_port:&p.source_port_id,to:&p.target_node_id,to_port:&p.target_port_id},M::Connect),FlowCommand::Disconnect(p)=>(R::Disconnect{id:&p.synapse_id},M::Disconnect),FlowCommand::RemoveWidget(p)=>(R::RemoveWidget{id:&p.widget_id},M::Remove),FlowCommand::Reorganize(_)=>(R::Reorganize{options},M::Reorganize),FlowCommand::AddWidget(p)=>{let x=p.x.unwrap_or(120.0);let y=p.y.unwrap_or(120.0);(R::AddWidget{descriptor:descriptor.ok_or_else(invalid)?,x,y},M::Add{x,y})},FlowCommand::NodeGraphEdit(p)=>{let[super::node_graph_edit::FlowNodeGraphEditOp::InsertPort{node_id,side,index}]=p.operations.as_slice()else{return Err(invalid())};let input=match side.as_str(){"input"=>true,"output"=>false,_=>return Err(invalid())};(R::InsertPort{id:node_id,input,index:usize::try_from(*index).map_err(|_|invalid())?},M::Insert{input})},_=>return Err(invalid())})
    }
    fn original_close_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(pipeline)=self.pipeline.as_ref(){return if pipeline.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<usize>(),depth:1,..Default::default()})}else{pipeline.close_demands(copy)}}
        if let Some(work)=self.descriptor_work.as_ref(){return if work.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<FlowAddDescriptorWork>(),depth:1,..Default::default()})}else{work.close_demands(copy)}}
        if !self.descriptor_retirement.is_empty(){return self.descriptor_retirement.demands(copy)}
        if self.descriptor.is_some(){return self.descriptor_retirement.next_owner_demands::<semio_framework_artifact_flow_flow::WidgetDescriptor>(true,&None,copy)}
        if !self.original.terminal_is_empty(){return self.original.close_demands(copy)}
        if let Some(owner)=self.instance_retirement.as_ref(){return owner.demands(copy)}
        Ok(if self.instance_owner.is_some(){semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands()}else{Default::default()})
    }
    fn original_step(&mut self,input:&semio_plugin::retained_command::ArtifactCommandInputs<'_,semio_plugin::EditorApp<FlowPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<semio_plugin::EditorApp<FlowPlayApp>>,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind};let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original graph command changed its retained intake");
        if self.completed||self.closing{return Err(invalid().into())}let context=input.context.ok_or_else(invalid)?;
        if !self.original.receive_parent(input,cx)?{return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-graph-original",preview:br#"{"en":"Preparing graph changes","de":"Graph\u00e4nderungen werden vorbereitet"}"#})}
        if self.tool_id=="addWidget"&&self.descriptor.is_none(){
            if self.descriptor_work.is_none(){let d=RetirementDemand{copy_bytes:size_of::<FlowAddDescriptorWork>(),depth:1,..Default::default()};if cx.retained_work_can_enter(d){self.descriptor_work=Some(FlowAddDescriptorWork::new());cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;}return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-descriptor",preview:br#"{"en":"Preparing widget","de":"Widget wird vorbereitet"}"#})}
            let(_,command,_)=self.original.references()?;let FlowCommand::AddWidget(payload)=command.get()else{return Err(invalid().into())};let work=self.descriptor_work.as_mut().ok_or_else(invalid)?;if let Some(descriptor)=work.step(payload,cx)?{self.descriptor=Some(descriptor);work.begin_close()}return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-descriptor",preview:br#"{"en":"Preparing widget","de":"Widget wird vorbereitet"}"#})
        }
        if let Some(work)=self.descriptor_work.as_mut(){if work.terminal_is_empty(){let d=RetirementDemand{copy_bytes:size_of::<FlowAddDescriptorWork>(),depth:1,..Default::default()};if cx.retained_work_can_enter(d){self.descriptor_work=None;cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;}}else{work.close_step(cx)?;}return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-descriptor",preview:br#"{"en":"Preparing widget","de":"Widget wird vorbereitet"}"#})}
        if self.pipeline.is_none(){let d=RetirementDemand{copy_bytes:size_of::<FlowAuthoringCommandWork>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(d){return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-graph-original",preview:br#"{"en":"Preparing graph changes","de":"Graph\u00e4nderungen werden vorbereitet"}"#})}let(_,command,_)=self.original.references()?;let(_,mode)=Self::original_request(command.get(),self.descriptor.as_ref(),&self.layout_options)?;self.pipeline=Some(FlowAuthoringCommandWork::new(flow::FlowAuthoringSerials::default(),mode));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;return Ok(ArtifactCommandWorkStep::Pending{stage:"flow-graph-original",preview:br#"{"en":"Preparing graph changes","de":"Graph\u00e4nderungen werden vorbereitet"}"#})}
        let(parent,command,infos)=self.original.references()?;let(request,_)=Self::original_request(command.get(),self.descriptor.as_ref(),&self.layout_options)?;
        match self.pipeline.as_mut().ok_or_else(invalid)?.step(parent,command,&context.children,input.operation,infos,request,cx)?{Some(emit)=>{self.completed=true;Ok(ArtifactCommandWorkStep::Complete(emit))},None=>Ok(ArtifactCommandWorkStep::Pending{stage:"flow-graph-original",preview:br#"{"en":"Preparing graph changes","de":"Graph\u00e4nderungen werden vorbereitet"}"#})}
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
                FlowCommand::ConnectMediaPorts(payload) => Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("mutation.target-missing"), format!("connectMediaPorts found no widget \"{}\"", if resolved[0] { &payload.target_node_id } else { &payload.source_node_id }))),
                FlowCommand::Reorganize(_) => reorganize::reorganize_edit(composed, config, session),
                FlowCommand::RenameFlowWidget(payload) if resolved[0] && !resolved[1] && !payload.value.trim().is_empty() => rename_flow_widget::rename_edit(payload, composed),
                FlowCommand::RenameFlowWidget(payload) if !resolved[0] => Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("mutation.target-missing"), format!("renameFlowWidget found no widget \"{}\"", payload.old_id))),
                FlowCommand::RenameFlowWidget(payload) => Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("flow.widget-id-unavailable"), format!("renameFlowWidget cannot rename \"{}\" to \"{}\": the id is empty or taken", payload.old_id, payload.value.trim()))),
                FlowCommand::RunExtensionAction(payload) => run_extension_action::extension_action_result(payload, composed, config, session),
                FlowCommand::SetActiveExample(payload) => set_active_example::set_active_example_edit(payload),
                _ => Err(Fault::from("flow-retained-graph-route-mismatch")),
            })?
        })
    }
}

impl ArtifactCommandWork<semio_plugin::EditorApp<FlowPlayApp>> for FlowGraphOperationWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn receive_original_command(&mut self,command:&mut FlowCommand,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.original_route(){self.original.receive_command(command,cx)}else{Ok(true)}}
    fn receive_original_context(&mut self,_context:&semio_plugin::app::ArtifactOwnedContextHandle<semio_plugin::EditorApp<FlowPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.original_route(){let owner=self.instance_owner.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original graph instance owner is absent"))?;self.original.receive_catalogue(owner,cx)}else{Ok(true)}}
    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<semio_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != self.tool_id || (!FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&self.tool_id)&&!self.original_route()) || !flow_graph_operation_payload_admitted(command) {
            return None;
        }
        if self.original_route(){return Some(1)}
        let child = flow_content_child(snapshot, &context?.children)?;
        if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return None;
        }
        FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(child.nodes.len().saturating_add(1))
    }

    fn step(&mut self, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, semio_plugin::EditorApp<FlowPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        if self.original_route(){return self.original_step(input,cx)}
        let original: Result<_, Fault> = (|| {
        let semio_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation, snapshot_read: _ } = *input;
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
        self.closing = true;self.original.begin_close();if let Some(work)=self.descriptor_work.as_mut(){work.begin_close()}if let Some(pipeline)=self.pipeline.as_mut(){pipeline.begin_close()}
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Close,close_original_retained_step};use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
        if !self.closing||!self.original.source_matches(cx){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"graph close requires original retained closure").into())}
        if let Some(pipeline)=self.pipeline.as_mut(){if !pipeline.terminal_is_empty(){return pipeline.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}let d=self.original_close_demands(cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(Close::Blocked)}self.pipeline=None;let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(Close::Pending{progress:p})}
        if let Some(work)=self.descriptor_work.as_mut(){if !work.terminal_is_empty(){return work.close_step(cx).map(|step|match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}let d=self.original_close_demands(cx.retained_grant().maximum_copy_bytes)?;if cx.fuel_remaining()==0||!cx.retained_work_can_enter(d){return Ok(Close::Blocked)}self.descriptor_work=None;let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(Close::Pending{progress:p})}
        if !self.descriptor_retirement.is_empty(){let d=self.descriptor_retirement.demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|self.descriptor_retirement.step_granted(g))?;return Ok(Close::Pending{progress:step.progress()})}
        if self.descriptor.is_some(){let d=self.descriptor_retirement.next_owner_demands::<semio_framework_artifact_flow_flow::WidgetDescriptor>(true,&None,cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let turn=self.descriptor_retirement.begin_granted(&mut self.descriptor,g);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;return Ok(Close::Pending{progress:step.progress()})}
        if !self.original.terminal_is_empty(){let d=self.original.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|self.original.close_step(g))?;return Ok(Close::Pending{progress:step.progress()})}
        let d=self.original_close_demands(cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(Close::Blocked)}
        if let Some(owner)=self.instance_owner.take(){self.instance_retirement=Some(semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(owner));let p=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(p)?;return Ok(Close::Pending{progress:p})}
        if let Some(owner)=self.instance_retirement.as_mut(){let grant=cx.retained_grant();let result=owner.step(cx,grant).map_err(semio_framework_job::OriginalTurnFailure::from)?;if owner.terminal_is_empty(){self.instance_retirement=None}return Ok(if self.terminal_is_empty(){Close::Complete{progress:result.progress()}}else{Close::Pending{progress:result.progress()}})}
        Ok(Close::Complete{progress:Default::default()})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.original_close_demands(0)?.depth)}

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none()&&self.instance_retirement.is_none()&&self.pipeline.is_none()&&self.descriptor_work.is_none()&&self.descriptor.is_none()&&self.descriptor_retirement.is_empty()&&self.original.terminal_is_empty()
    }
}

pub struct FlowGraphOperationJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowGraphOperationJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_GRAPH_OPERATION_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

pub fn flow_graph_operation_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(FLOW_GRAPH_OPERATION_RAW_BYTES, 256, FLOW_GRAPH_OPERATION_CAPACITY.work_items() as u64, 16_384, 7_500, 1, 1)
}

impl semio_framework::ToolJobFactory for FlowGraphOperationJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_plugin::EditorApp<FlowPlayApp>>;

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

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: FLOW_GRAPH_OPERATION_RAW_BYTES, checkpoint_resume: true);

}

impl semio_plugin::ArtifactOwnedToolJobFactory for FlowGraphOperationJobFactory {
    type Owner = semio_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_GRAPH_OPERATION_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = &[
        semio_plugin::ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[semio_plugin::ArtifactToolPublicationLane::Child] },
        semio_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_plugin::ArtifactToolPublicationLane::Child] },
        semio_plugin::ArtifactToolPublicationContract { tool_id: "renameFlowWidget", lanes: &[semio_plugin::ArtifactToolPublicationLane::Child] },
        semio_plugin::ArtifactToolPublicationContract { tool_id: "runExtensionAction", lanes: &[semio_plugin::ArtifactToolPublicationLane::Child] },
        semio_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_plugin::ArtifactToolPublicationLane::Child] },
    ];
}

pub(super) struct FlowGraphOperationJobFactoryProofs;

impl FlowGraphOperationJobFactoryProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowGraphOperationJobFactory",
        factory_type: FlowGraphOperationJobFactory,
        contract: flow_graph_operation_contract(),
        tools: ["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"]
    }

    pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }
}
//#endregion 🧵️GraphOperationRetainedRoute

pub(super) struct FlowDirectStoreJobFactoryProofs;

impl FlowDirectStoreJobFactoryProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<FlowPlayApp>,
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

    pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }
}

pub(super) struct FlowHostEffectJobFactoryProofs;

impl FlowHostEffectJobFactoryProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowHostEffectJobFactory",
        factory_type: FlowHostEffectJobFactory,
        tools: {
            "contextMenuAt" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "openSpotlight" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "replaceImage" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "flowEvalResolve" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
        }
    }

    pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }
}

pub(super) struct FlowChildGroupJobFactoryProofs;

impl FlowChildGroupJobFactoryProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<FlowPlayApp>,
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

    pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }
}

semio_plugin::retained_command::terminal_frame_bound!(FlowGraphOperationWork);
