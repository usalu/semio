//! 🏠️ Flow instance operation owner: the retained evaluation session, operator catalogue and scene cache of one app instance. See [`super`].

use crate::editor::flow::host::{FlowComposedRead, FlowComposedScene};
use crate::FlowSnapshot;
use flow::registry::FlowOperatorCatalogueOwner;
use flow::FlowEvalSession;
use semio_plugin::{ArtifactView, Fault};

#[path = "🖼️scene-cache/🦀️.rs"]
mod scene_cache;
use scene_cache::FlowSceneCache;

//#region 🏠️FlowInstanceOperationOwner
pub struct FlowInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    catalogue: FlowOperatorCatalogueOwner,
    scene_cache: FlowSceneCache,
    pub closing: bool,
}

impl FlowInstanceOperationOwner {
    pub fn new() -> Self {
        let(registry,generation)=flow::registry::bootstrap_flow_operator_registry_source();
        Self { eval_session: Some(FlowEvalSession::new()), catalogue:FlowOperatorCatalogueOwner::new(registry,generation),scene_cache:FlowSceneCache::new(), closing: false }
    }

    pub fn publish_scene_original<A:crate::FlowApp>(&mut self,context:&semio_plugin::app::ArtifactOwnedToolJobContext<semio_plugin::EditorApp<A>>,binding:semio_framework_job::OriginalSourceBinding,scene:&mut Option<FlowComposedScene>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.scene_cache.publish_original(context,binding,scene,cx)}
    pub fn with_scene_session<R>(&mut self,doc:&ArtifactView<'_,FlowSnapshot>,body:impl FnOnce(&FlowComposedRead<'_>,&mut FlowEvalSession)->R)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original Flow render session is closing").into())}
        let source=if let Some(source)=self.scene_cache.read(doc){FlowComposedRead::Projected(source)}else if doc.children.root.is_none(){FlowComposedRead::Local(doc.snapshot.read_source()?)}else{return Ok(None)};
        let session=self.eval_session.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow render session is missing"))?;Ok(Some(body(&source,session)))
    }
    pub fn catalogue_ready(&self)->bool{!self.closing&&self.catalogue.ready()}
    pub fn retain_catalogue_infos(&self,registry:&flow::neural::SharedRegistry,generation:u64,output:&mut Option<flow::registry::FlowOperatorInfoMap>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.catalogue.retain_infos(registry,generation,output,cx)}
    pub fn advance_catalogue(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.catalogue.step(cx)}
    /// 🎟️ Preserves the original typed outcome while borrowing the retained instance session.
    pub fn with_original_session<R>(&mut self,body:impl FnOnce(&mut FlowEvalSession)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original Flow instance session is closing").into())}
        let session=self.eval_session.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Flow instance session is missing"))?;
        body(session)
    }
    pub fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
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
            cx.with_retained_loan(child,|same|{if self.closing{self.catalogue.close(same)?;}else{self.catalogue.step(same)?;}Ok::<_,semio_framework_job::OriginalTurnFailure>(())})?;
            let after=cx.retained_progress();let sub=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original instance catalogue receipt moved backwards"));let progress=RetainedCloneProgress{copied_items:sub(after.copied_items,before.copied_items)?,copied_bytes:sub(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:sub(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:sub(after.released_bytes,before.released_bytes)?};return Ok(Step::Progress(progress))
        }
        if self.scene_cache.pending()||(self.closing&&!self.scene_cache.terminal_is_empty()){
            let before=cx.retained_progress();let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            cx.with_retained_loan(child,|same|self.scene_cache.advance(same))?;
            let after=cx.retained_progress();let sub=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original scene cache receipt moved backwards"));
            return Ok(Step::Progress(RetainedCloneProgress{copied_items:sub(after.copied_items,before.copied_items)?,copied_bytes:sub(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:sub(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:sub(after.released_bytes,before.released_bytes)?}))
        }
        let Some(session)=self.eval_session.as_mut()else{return Ok(Step::Complete(Default::default()))};
        let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if !self.closing{
            let copy=session.next_retirement_copy_byte_demand()?;let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:session.next_retirement_capacity_byte_demand(copy)?,release_bytes:session.next_retirement_release_byte_demand()?,depth:session.next_retirement_depth_demand()?};
            return semio_plugin::app::original_lifecycle_effect_step(cx,child,demand,|_,effect|{let body=effect.grant().maximum_copy_bytes;Ok(match semio_plugin::app::plugin_settle_poll(session.retirement_step(body,effect))?{RetainedCloneStep::Progress(progress)=>(Step::Progress(progress),false),RetainedCloneStep::Complete(progress)=>(Step::Complete(progress),true)})})
        }
        if session.terminal_is_empty(){self.eval_session=None;return Ok(Step::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        let copy=session.next_close_copy_byte_demand()?;let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:session.next_close_capacity_byte_demand(copy)?,release_bytes:session.next_close_release_byte_demand()?,depth:session.next_close_depth_demand()?};
        let step=semio_plugin::app::original_lifecycle_effect_step(cx,child,demand,|_,effect|{
            let progress=match session.close_step(effect)?{semio_framework_job::InteractiveJobCloseStep::Blocked=>Default::default(),semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>progress};
            let terminal=session.terminal_is_empty();Ok((if terminal{Step::Complete(progress)}else{Step::Progress(progress)},terminal))
        })?;
        if matches!(step,Step::Complete(_)){self.eval_session=None;}
        Ok(step)
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_plugin::PluginLifecycleStep,semio_plugin::PluginLifecycleFailure>{
        self.closing=true;self.catalogue.begin_close();self.scene_cache.begin_close();if let Some(session)=self.eval_session.as_mut(){session.begin_close();}self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self)->bool{self.closing&&self.eval_session.is_none()&&self.catalogue.terminal_is_empty()&&self.scene_cache.terminal_is_empty()}
}
//#endregion 🏠️FlowInstanceOperationOwner
