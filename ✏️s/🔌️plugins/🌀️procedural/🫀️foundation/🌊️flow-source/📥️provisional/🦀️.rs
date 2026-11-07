//! 📥️ Provisional leaves retain the genuine immutable context and original issuer.
use super::retained_scratch_driver::funded_construction;
use semio_plugin::{ArtifactApp,app::{ArtifactOwnedContextHandle,ArtifactOwnedToolJobContext}};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{DslValue,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[derive(Clone,Copy,PartialEq,Eq)]
struct ProvisionalIdentity{context:usize,values:usize,len:usize,generation:u64,revision:[u8;32]}
type Context<A:ArtifactApp> = ArtifactOwnedToolJobContext<A>;
type Handle<A:ArtifactApp> = ArtifactOwnedContextHandle<A>;
pub struct ProvisionalSourceWork<A:ArtifactApp>{original:ManuallyDrop<Option<Handle<A>>>,admission:Option<RetainedCloneSourceAdmission<Context<A>,Handle<A>>>,source:Option<RetainedCloneSource<Context<A>,Handle<A>>>,close:Option<ControlledRetirement<Handle<A>>>,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),identity:Option<ProvisionalIdentity>,closing:bool,progress:RetainedCloneProgress,work:u64}
impl<A:ArtifactApp> ProvisionalSourceWork<A>{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:semio_framework_job::OriginalSourceBinding)->Self{Self{original:ManuallyDrop::new(None),admission:None,source:None,close:None,original_caller:original,operation:(original.operation().0,original.generation().0),identity:None,closing:false,progress:Default::default(),work:0}}
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.operation==(cx.operation().0,cx.generation().0)&&self.original_caller.matches(cx)&&!self.closing{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional source requires its initiating caller").into())}}
 pub fn receive_context(&mut self,context:&Handle<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;let view=context.get();let values=view.provisional();let identity=ProvisionalIdentity{context:view as *const Context<A> as usize,values:values.as_ptr()as usize,len:values.len(),generation:view.provisional_generation(),revision:view.canonical_base_revision};
  if self.identity.is_some_and(|previous|previous!=identity){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original provisional context identity changed").into())}if self.original.is_some()||self.admission.is_some()||self.source.is_some(){return Ok(true)}
  let demand=context.original_alias_demands();let ready=funded_construction(cx,&mut self.original,OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|grant|context.duplicate_original(grant))?;
  if ready{self.identity=Some(identity)}Ok(ready)
 }
 pub fn original(&self)->Option<&Context<A>>{self.original.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(RetainedCloneSource::original_authority)).map(Handle::get)}
 pub fn len(&self)->usize{self.identity.map_or(0,|identity|identity.len)}
 pub fn generation(&self)->Option<u64>{self.identity.map(|identity|identity.generation)}
 pub fn next_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original provisional source is closing"))}if let Some(admission)=self.admission.as_ref(){if admission.phase()!="ready"{return admission.next_demand()}}Ok(RetirementDemand{depth:1,..Default::default()})}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.progress=Default::default();self.work=0;let result=self.advance_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn advance_original(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let demand=self.next_demands()?;if self.source.is_some(){return Ok(true)}if !admits(grant,demand){return Ok(false)}
  if let Some(original)=self.original.take(){self.admission=Some(original.provisional_source_admission());self.work=1;self.progress.copied_items=1;return Ok(false)}
  let admission=self.admission.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional context was not captured"))?;
  let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,grant)}else{admission.advance(grant)};self.work=turn.work;self.progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None}Ok(self.source.is_some())
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;cx.original_source_parent()?.validate_borrowed_context(cx)?;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let turn=self.advance(cx.retained_grant());let p=self.progress;
   semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(p)),work:turn.work})?;
   if self.source.is_some()||p==Default::default(){return Ok(self.source.is_some())}
  }Ok(false)
 }

 pub fn project(&self,index:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<(RetainedOwnedProjection<DslValue>,RetainedCloneProgress)>{
  if self.closing||index>=self.len(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original provisional index is outside its captured list")),work:0}}
  match self.source.as_ref(){Some(source)=>source.project_owned(index,|context|&context.provisional()[index],grant),None=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original provisional source admission is pending")),work:0}}
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}if let Some(source)=self.source.as_ref(){return source.next_close_demand(copy)}if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.original.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional close was not begun"))}let demand=self.next_close_demands(grant.maximum_copy_bytes)?;if !admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);self.work=turn.work;self.progress=turn.result?.progress();if let Some((original,()))=returned{*self.original=Some(original)}if admission.terminal_is_empty(){self.admission=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(source)=self.source.as_mut(){let turn=source.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress();if source.terminal_is_empty(){self.source=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(original)=self.original.take(){self.close=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original context supports controlled retirement")));self.work=1;self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(close)=self.close.as_mut(){let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();let step=result?;if close.terminal_is_empty(){self.close=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.close.is_none()}
}
impl<A:ArtifactApp> Drop for ProvisionalSourceWork<A>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original provisional source retains its captured context issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}

fn admits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}

#[path="🛬️receive/🦀️.rs"]
mod receiving;
pub use receiving::ProvisionalFieldWork;
