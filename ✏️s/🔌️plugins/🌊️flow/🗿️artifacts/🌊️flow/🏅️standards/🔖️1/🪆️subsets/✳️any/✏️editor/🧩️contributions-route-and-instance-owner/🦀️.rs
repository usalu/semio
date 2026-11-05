//! 🧩️ Contributions route and instance owner: the contributions tool ids, work and job factory and the flow instance operation owner. See [`super`].

use super::*;
use flow::registry::FlowOperatorCatalogueOwner;
#[path="🖼️scene-cache/🦀️.rs"]mod scene_cache;
use scene_cache::FlowSceneCache;
use semio_framework_job::OriginalFailureContext;

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
    pub instance_owner: Option<semio_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub instance_retirement: Option<semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement>,
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
    pub fn new(instance_owner: semio_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner: Some(instance_owner), instance_retirement: None, completed: false, closing: false }
    }

    pub(super) fn close_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.instance_retirement.as_ref(){let mut demand=owner.demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original contributions alias depth overflow"))?;return Ok(demand)}
        Ok(if self.instance_owner.is_some(){semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement::constructor_demands()}else{Default::default()})
    }
}

impl ArtifactCommandWork<semio_plugin::EditorApp<FlowPlayApp>> for FlowContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &FlowCommand,
        _snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<semio_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        (!self.closing && !self.completed && matches!(command, FlowCommand::SetContributions(payload) if payload.json.len() <= FLOW_CONTRIBUTIONS_RAW_BYTES)).then_some(1)
    }

    fn step(&mut self, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, semio_plugin::EditorApp<FlowPlayApp>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.completed {
            return Err(Fault::from("flow-contributions-work-terminal"));
        }
        let FlowCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("flow-contributions-route-rejected"));
        };
        let maintenance=flow::retire_flow_extension_registries_step(cx.retained_grant());
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

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_nested_step(cx,|cx,grant|{
            use semio_framework_job::InteractiveJobCloseStep as Step;
            use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep};
            if !self.closing||grant.maximum_items==0{return Ok(Step::Blocked)}
            let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))};
            if grant.maximum_depth<demand.depth{return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "interactive job close exceeds its admitted depth"),cx))}
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Step::Pending{progress:Default::default()})}
            if let Some(original)=self.instance_owner.take(){self.instance_retirement=Some(semio_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(original));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
            if let Some(owner)=self.instance_retirement.as_mut(){
                let grant=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
                let result=owner.step(cx,grant);
                let terminal=owner.terminal_is_empty();
                if terminal{self.instance_retirement=None;}
                return Ok(match result{Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Err(fault)=>return Err(semio_framework_job::OriginalTurnFailure::from(fault.debit_original(cx)))});
            }
            Ok(Step::Complete{progress:Default::default()})
        })
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
    type Payload = ArtifactRetainedCommandPayload<semio_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_plugin::EditorApp<FlowPlayApp>>;

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

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: FLOW_CONTRIBUTIONS_RAW_BYTES, checkpoint_resume: false);

}

impl semio_plugin::ArtifactOwnedToolJobFactory for FlowContributionsJobFactory {
    type Owner = semio_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] =
        &[semio_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_plugin::ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct FlowContributionsJobFactoryProofs;

impl FlowContributionsJobFactoryProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowContributionsJobFactory",
        factory_type: FlowContributionsJobFactory,
        contract: flow_contributions_contract(),
        tools: ["setContributions"]
    }

    pub(super) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        Self::bounded_first_step_tool_proofs()
    }
}
//#endregion 🧩️ContributionsRoute

//#region 🔖️FlowPlayApp
pub struct FlowInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    catalogue: FlowOperatorCatalogueOwner,
    scene_cache: FlowSceneCache,
    pub(super) closing: bool,
}

impl FlowInstanceOperationOwner {
    pub fn new() -> Self {
        let(registry,generation)=flow::registry::bootstrap_flow_operator_registry_source();
        Self { eval_session: Some(FlowEvalSession::new()), catalogue:FlowOperatorCatalogueOwner::new(registry,generation),scene_cache:FlowSceneCache::new(), closing: false }
    }

    pub(super) fn publish_scene_original(&mut self,context:&semio_plugin::app::ArtifactOwnedToolJobContext<semio_plugin::EditorApp<FlowPlayApp>>,binding:semio_framework_job::OriginalSourceBinding,scene:&mut Option<crate::editor::flow::host::FlowComposedScene>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.scene_cache.publish_original(context,binding,scene,cx)}
    pub(super) fn with_scene_session<R>(&mut self,doc:&ArtifactView<'_,FlowSnapshot>,body:impl FnOnce(&crate::editor::flow::host::FlowComposedRead<'_>,&mut FlowEvalSession)->R)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original Flow render session is closing").into())}
        let source=if let Some(source)=self.scene_cache.read(doc){crate::editor::flow::host::FlowComposedRead::Projected(source)}else if doc.children.root.is_none(){crate::editor::flow::host::FlowComposedRead::Local(doc.snapshot.read_source()?)}else{return Ok(None)};
        let session=self.eval_session.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow render session is missing"))?;Ok(Some(body(&source,session)))
    }
    pub(super) fn catalogue_ready(&self)->bool{!self.closing&&self.catalogue.ready()}
    pub(super) fn retain_catalogue_infos(&self,registry:&flow::neural::SharedRegistry,generation:u64,output:&mut Option<flow::registry::FlowOperatorInfoMap>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.catalogue.retain_infos(registry,generation,output,cx)}
    pub(super) fn advance_catalogue(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.catalogue.step(cx)}
    /// 🎟️ Preserves the original typed outcome while borrowing the retained instance session.
    pub(super) fn with_original_session<R>(&mut self,body:impl FnOnce(&mut FlowEvalSession)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original Flow instance session is closing").into())}
        let session=self.eval_session.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow instance session is missing"))?;
        body(session)
    }
    pub(super) fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("flow-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("flow-eval-session-owner-missing"))
    }
}

impl semio_plugin::ArtifactInstanceOperationOwner for FlowInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}

    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if (self.closing&&!self.catalogue.terminal_is_empty())||(!self.closing&&!self.catalogue.ready()){let mut d=if self.closing{self.catalogue.close_demands(body)?}else{self.catalogue.step_demands(body)?};d.depth=d.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original instance catalogue nesting overflow"))?;return Ok(d)}
        if self.scene_cache.pending()||(self.closing&&!self.scene_cache.terminal_is_empty()){let mut demand=self.scene_cache.demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Flow scene cache nesting overflow"))?;return Ok(demand)}
        let Some(session)=self.eval_session.as_ref()else{return Ok(Default::default())};
        let mut demand=if self.closing{semio_framework_value::RetirementDemand{copy_bytes:session.next_close_copy_byte_demand()?,capacity_bytes:session.next_close_capacity_byte_demand(body)?,release_bytes:session.next_close_release_byte_demand()?,depth:session.next_close_depth_demand()?}}else{semio_framework_value::RetirementDemand{copy_bytes:session.next_retirement_copy_byte_demand()?,capacity_bytes:session.next_retirement_capacity_byte_demand(body)?,release_bytes:session.next_retirement_release_byte_demand()?,depth:session.next_retirement_depth_demand()?}};
        demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Flow instance session depth overflow"))?;Ok(demand)
    }

    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_plugin::PluginLifecycleStep,semio_plugin::PluginLifecycleFailure>{
        use semio_plugin::PluginLifecycleStep as Step;
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0{return Ok(Step::Progress(Default::default()))}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Flow instance session exceeds parent depth").into())}
        if (self.closing&&!self.catalogue.terminal_is_empty())||(!self.closing&&!self.catalogue.ready()){
            let before=cx.retained_progress();let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            cx.with_retained_loan(child,|same|{if self.closing{let turn=self.catalogue.close_step(same.retained_grant());semio_framework_job::receive_original_retained_work(same,turn)?;}else{self.catalogue.step(same)?;}Ok::<_,semio_framework_job::OriginalTurnFailure>(())})?;
            let after=cx.retained_progress();let sub=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original instance catalogue receipt moved backwards"));let progress=RetainedCloneProgress{copied_items:sub(after.copied_items,before.copied_items)?,copied_bytes:sub(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:sub(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:sub(after.released_bytes,before.released_bytes)?};return Ok(Step::Progress(progress))
        }
        if self.scene_cache.pending()||(self.closing&&!self.scene_cache.terminal_is_empty()){
            let before=cx.retained_progress();let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            cx.with_retained_loan(child,|same|{let turn=self.scene_cache.advance(same.retained_grant());semio_framework_job::receive_original_retained_work(same,turn)?;Ok::<_,semio_framework_job::OriginalTurnFailure>(())})?;
            let after=cx.retained_progress();let sub=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original scene cache receipt moved backwards"));
            return Ok(Step::Progress(RetainedCloneProgress{copied_items:sub(after.copied_items,before.copied_items)?,copied_bytes:sub(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:sub(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:sub(after.released_bytes,before.released_bytes)?}))
        }
        let Some(session)=self.eval_session.as_mut()else{return Ok(Step::Complete(Default::default()))};
        let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if !self.closing{return session.retirement_step(child).map(|step|match step{RetainedCloneStep::Progress(progress)=>Step::Progress(progress),RetainedCloneStep::Complete(progress)=>Step::Complete(progress)}).map_err(semio_plugin::PluginLifecycleFailure::from)}
        if session.terminal_is_empty(){self.eval_session=None;return Ok(Step::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        let result=session.close_step(child)?;
        let terminal=session.terminal_is_empty();
        if terminal{self.eval_session=None;}
        match result{semio_framework_job::InteractiveJobCloseStep::Blocked=>Ok(Step::Progress(Default::default())),semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>Ok(if self.eval_session.is_none(){Step::Complete(progress)}else{Step::Progress(progress)})}
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_plugin::PluginLifecycleStep,semio_plugin::PluginLifecycleFailure>{
        self.closing=true;self.catalogue.begin_close();self.scene_cache.begin_close();if let Some(session)=self.eval_session.as_mut(){session.begin_close();}self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self)->bool{self.closing&&self.eval_session.is_none()&&self.catalogue.terminal_is_empty()&&self.scene_cache.terminal_is_empty()}
}
//#endregion 🔖️FlowPlayApp
