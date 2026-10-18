//! 🧩️ Contributions route and instance owner: the contributions tool ids, work and job factory and the flow instance operation owner. See [`super`].

use super::*;

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route — the flow twin of generation2d's: the shell's `flow.extension`
/// closure is what makes this app's extension operators exist at all (`install_builtin_flow_extensions`
/// installs none).
pub const FLOW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];

const FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "flow.contributions-command.v1";

/// 📐️ The real wire ceiling of one contributions push: the paged command ingress's assembled-command bound,
/// exactly as generation2d/generation3d declare it — the unscoped nine-extension closure (293 642 characters,
/// `flow-extension-brep` alone 190 656) crosses whole.
pub(super) const FLOW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

fn flow_contributions_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(FLOW_CONTRIBUTIONS_RAW_BYTES, 256, 1, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page against the app instance's retained evaluation session.
pub struct FlowContributionsWork {
    pub instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub instance_retirement: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement>,
    pub completed: bool,
    pub closing: bool,
}

/// 🧾️ The original registry turn reaches the external receipt before yielding or refusing.
pub fn flow_contributions_registry_receipt(cx:&mut semio_framework_job::StepContext<'_>,result:Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>)->Result<bool,Fault>{
    let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
    cx.consume_retained(progress).map_err(value_fault)?;
    match result{Ok(semio_framework_value::RetainedCloneStep::Complete(progress))=>Ok(progress==Default::default()),Ok(_)=>Ok(false),Err(error)=>Err(value_fault(error))}
}

impl FlowContributionsWork {
    pub fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner: Some(instance_owner), instance_retirement: None, completed: false, closing: false }
    }

    pub(super) fn close_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.instance_retirement.as_ref(){let mut demand=owner.demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original contributions alias depth overflow"))?;return Ok(demand)}
        Ok(if self.instance_owner.is_some(){semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands()}else{Default::default()})
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &FlowCommand,
        _snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        (!self.closing && !self.completed && matches!(command, FlowCommand::SetContributions(payload) if payload.json.len() <= FLOW_CONTRIBUTIONS_RAW_BYTES)).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.completed {
            return Err(Fault::from("flow-contributions-work-terminal"));
        }
        let FlowCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("flow-contributions-route-rejected"));
        };
        let maintenance=semio_framework_artifact_flow_flow::retire_flow_extension_registries_step(cx.retained_grant());
        if !flow_contributions_registry_receipt(cx,maintenance)?{
            return Ok(ArtifactCommandWorkStep::Progress{stage:"flow-contributions-retirement",preview:br#"{"en":"Preparing operators","de":"Operatoren werden vorbereitet"}"#});
        }
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-contributions-instance-owner"))?;
        let install=instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(|session| set_contributions::install(payload, session))?);
        match install{Ok(_)=>{},Err(fault)if fault.message=="flow.registry-retirement-full"=>return Ok(ArtifactCommandWorkStep::Progress{stage:"flow-contributions-retirement",preview:br#"{"en":"Preparing operators","de":"Operatoren werden vorbereitet"}"#}),Err(fault)=>return Err(fault)}
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.depth)}

    fn close_step(&mut self, grant:semio_framework_value::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueRefusalKind};
        if !self.closing||grant.maximum_items==0{return Step::Blocked}
        let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Step::Refused{kind:error.kind,progress:error.retained_progress()}};
        if grant.maximum_depth<demand.depth{return Step::Refused{kind:ValueRefusalKind::DepthLimit,progress:Default::default()}}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Step::Pending{progress:Default::default()}}
        if let Some(original)=self.instance_owner.take(){self.instance_retirement=Some(semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(original));return Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}}}
        if let Some(owner)=self.instance_retirement.as_mut(){
            let grant=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let result=owner.step(grant);
            let terminal=owner.terminal_is_empty();
            if terminal{self.instance_retirement=None;}
            return match result{Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Err(fault)=>Step::Refused{kind:fault.value_refusal_kind().unwrap_or(ValueRefusalKind::InvariantViolated),progress:fault.retained_progress}};
        }
        Step::Complete{progress:Default::default()}
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none() && self.instance_retirement.is_none()
    }
}

pub struct FlowContributionsJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowContributionsJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_contributions_contract()
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
        if input.declared_bytes() > FLOW_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("Flow contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowContributionsJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] =
        &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct FlowContributionsJobFactoryProofs;

impl FlowContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowContributionsJobFactory",
        factory_type: FlowContributionsJobFactory,
        contract: flow_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 🔖️FlowPlayApp
pub struct FlowInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    pub(super) closing: bool,
}

impl FlowInstanceOperationOwner {
    pub fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), closing: false }
    }

    pub(super) fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("flow-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("flow-eval-session-owner-missing"))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for FlowInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}

    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let Some(session)=self.eval_session.as_ref()else{return Ok(Default::default())};
        let mut demand=if self.closing{semio_framework_value::RetirementDemand{copy_bytes:session.next_close_copy_byte_demand()?,capacity_bytes:session.next_close_capacity_byte_demand(body)?,release_bytes:session.next_close_release_byte_demand()?,depth:session.next_close_depth_demand()?}}else{semio_framework_value::RetirementDemand{copy_bytes:session.next_retirement_copy_byte_demand()?,capacity_bytes:session.next_retirement_capacity_byte_demand(body)?,release_bytes:session.next_retirement_release_byte_demand()?,depth:session.next_retirement_depth_demand()?}};
        demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Flow instance session depth overflow"))?;Ok(demand)
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{
        use semio_framework_plugin::PluginLifecycleStep as Step;
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0{return Ok(Step::Progress(Default::default()))}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Flow instance session exceeds parent depth").into())}
        let Some(session)=self.eval_session.as_mut()else{return Ok(Step::Complete(Default::default()))};
        let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if !self.closing{return session.retirement_step(child).map(|step|match step{RetainedCloneStep::Progress(progress)=>Step::Progress(progress),RetainedCloneStep::Complete(progress)=>Step::Complete(progress)}).map_err(semio_framework_plugin::PluginLifecycleFailure::from)}
        if session.terminal_is_empty(){self.eval_session=None;return Ok(Step::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        let result=session.close_step(child)?;
        let terminal=session.terminal_is_empty();
        if terminal{self.eval_session=None;}
        match result{semio_framework_job::InteractiveJobCloseStep::Blocked=>Ok(Step::Progress(Default::default())),semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>Ok(if self.eval_session.is_none(){Step::Complete(progress)}else{Step::Progress(progress)})}
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_plugin::PluginLifecycleFailure>{
        self.closing=true;if let Some(session)=self.eval_session.as_mut(){session.begin_close();}self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self)->bool{self.closing&&self.eval_session.is_none()}
}
//#endregion 🔖️FlowPlayApp
