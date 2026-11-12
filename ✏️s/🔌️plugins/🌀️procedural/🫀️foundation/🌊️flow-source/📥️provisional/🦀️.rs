//! 📥️ Provisional leaves retain the genuine immutable context and original issuer.
use super::retained_scratch_driver::{funded_turn,local_progress,pay};
use semio_plugin::{ArtifactApp,app::{ArtifactOwnedContextHandle,ArtifactOwnedToolJobContext}};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{DslValue,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjectionAdmission,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[derive(Clone,Copy,PartialEq,Eq)]
struct ProvisionalIdentity{context:usize,values:usize,len:usize,generation:u64,revision:[u8;32]}
type Context<A:ArtifactApp> = ArtifactOwnedToolJobContext<A>;
type Handle<A:ArtifactApp> = ArtifactOwnedContextHandle<A>;
pub struct ProvisionalSourceWork<A:ArtifactApp>{original:ManuallyDrop<Option<Handle<A>>>,admission:Option<RetainedCloneSourceAdmission<Context<A>,Handle<A>>>,source:Option<RetainedCloneSource<Context<A>,Handle<A>>>,close:Option<ControlledRetirement<Handle<A>>>,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),identity:Option<ProvisionalIdentity>,closing:bool,progress:RetainedCloneProgress}
impl<A:ArtifactApp> ProvisionalSourceWork<A>{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:semio_framework_job::OriginalSourceBinding)->Self{Self{original:ManuallyDrop::new(None),admission:None,source:None,close:None,original_caller:original,operation:(original.operation().0,original.generation().0),identity:None,closing:false,progress:Default::default()}}
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.operation==(cx.operation().0,cx.generation().0)&&self.original_caller.matches(cx)&&!self.closing{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional source requires its initiating caller").into())}}
 pub fn receive_context(&mut self,context:&Handle<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;let view=context.get();let values=view.provisional();let identity=ProvisionalIdentity{context:view as *const Context<A> as usize,values:values.as_ptr()as usize,len:values.len(),generation:view.provisional_generation(),revision:view.canonical_base_revision};
  if self.identity.is_some_and(|previous|previous!=identity){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original provisional context identity changed").into())}if self.original.is_some()||self.admission.is_some()||self.source.is_some(){return Ok(true)}
  let demand=context.original_alias_demands();let slot=&mut*self.original;
  funded_turn(cx,|effect|{
   if !retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
   let turn=context.duplicate_original(effect);
   RetainedWorkOutcome{result:turn.result.map(|(handle,progress)|{*slot=Some(handle);RetainedCloneStep::Complete(progress)}),work:turn.work}
  })?;
  let ready=self.original.is_some();if ready{self.identity=Some(identity)}Ok(ready)
 }
 pub fn original(&self)->Option<&Context<A>>{self.original.as_ref().or_else(||self.admission.as_ref().and_then(RetainedCloneSourceAdmission::original_authority)).or_else(||self.source.as_ref().and_then(RetainedCloneSource::original_authority)).map(Handle::get)}
 pub fn len(&self)->usize{self.identity.map_or(0,|identity|identity.len)}
 pub fn generation(&self)->Option<u64>{self.identity.map(|identity|identity.generation)}
 pub fn next_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original provisional source is closing"))}if let Some(admission)=self.admission.as_ref(){if admission.phase()!="ready"{return admission.next_demand()}}Ok(RetirementDemand{depth:1,..Default::default()})}
 pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.progress=Default::default();let result=self.advance_original(effect).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:effect.performed_work()}}
 fn advance_original(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let demand=self.next_demands()?;if self.source.is_some(){return Ok(true)}if !retained_clone_grant_admits(effect.grant(),demand){return Ok(false)}
  if self.original.is_some(){if !pay(effect,1)?{return Ok(false)}let original=self.original.take().unwrap();self.admission=Some(original.provisional_source_admission());self.progress=local_progress(demand);return Ok(false)}
  let admission=self.admission.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional context was not captured"))?;
  let turn=if admission.phase()=="ready"{admission.take_output(&mut self.source,effect)}else{admission.advance(effect)};self.progress=turn.result?.progress();if admission.terminal_is_empty(){self.admission=None}Ok(self.source.is_some())
 }
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;cx.original_source_parent()?.validate_borrowed_context(cx)?;
  for _ in 0..512{
   if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let mut progress=RetainedCloneProgress::default();
   let step=funded_turn(cx,|effect|{let turn=self.advance(effect);progress=self.progress;RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(progress)),work:turn.work}})?;
   if step.is_none()||self.source.is_some()||progress==Default::default(){return Ok(self.source.is_some())}
  }Ok(false)
 }

 /// 📥️ Quotes projecting one captured provisional leaf into the caller's admission.
 pub fn projection_demands(&self,index:usize,output:&RetainedOwnedProjectionAdmission<DslValue>)->Result<(RetirementDemand,u64),ValueError>{
  let source=self.leaf(index)?;Ok((output.demand(source)?,output.work_demand(source)?))
 }
 fn leaf(&self,index:usize)->Result<semio_framework_value::retained_clone::RetainedCloneRef<'_,DslValue>,ValueError>{
  if self.closing||index>=self.len(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original provisional index is outside its captured list"))}
  Ok(self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original provisional source admission is pending"))?.try_borrow()?.project(index,|context|&context.provisional()[index]))
 }
 /// 📥️ Admits one captured provisional leaf as an original projection under the caller's effect.
 pub fn project_into<'reply>(&self,index:usize,output:&mut RetainedOwnedProjectionAdmission<DslValue>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{output.admit(self.leaf(index)?,effect,reply)}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}if let Some(source)=self.source.as_ref(){return source.next_close_demand(copy)}if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.original.is_some()),..Default::default()})}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)}
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original provisional close was not begun").into())}let demand=self.next_close_demands(body)?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(admission)=self.admission.as_mut(){let mut returned=None;let turn=admission.return_inputs(&mut returned,effect);self.progress=turn.result?.progress();if let Some((original,()))=returned{*self.original=Some(original)}if admission.terminal_is_empty(){self.admission=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(source)=self.source.as_mut(){self.progress=RetirementPollExit::resume(source.close_step(body,effect))?.progress();if source.terminal_is_empty(){self.source=None}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(original)=self.original.take(){self.close=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original context supports controlled retirement")));self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(close)=self.close.as_mut(){let step=RetirementPollExit::resume(close.step(body,effect))?;self.progress=close.step_progress();if close.terminal_is_empty(){self.close=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.original.is_none()&&self.admission.is_none()&&self.source.is_none()&&self.close.is_none()}
}
impl<A:ArtifactApp> Drop for ProvisionalSourceWork<A>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original provisional source retains its captured context issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original)}}}}

#[path="🛬️receive/🦀️.rs"]
mod receiving;
pub use receiving::ProvisionalFieldWork;
