//! 🏗️ Source works birth every step through one resident construction frame and retire every owner through the original effect.
use semio_framework_job::{InteractiveJobCloseStep,OriginalConstructionDemand,OriginalTurnFailure,StepContext,close_original_effect_step};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetirementPollExit,ValueError};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::RetireOwned;
use semio_plugin::MediaConstruction;
/// 🏗️ One caller-resident construction frame and its settled marker: every local source step is born, constructed and settled through it, and it closes under its owner's retirement.
#[derive(Default)]
pub(crate) struct Birth{frame:MediaConstruction,marker:Option<()>}
impl Birth{
 pub(crate) fn settling(&self)->bool{self.marker.is_some()||self.frame.settling()}
 pub(crate) fn terminal_is_empty(&self)->bool{self.marker.is_none()&&self.frame.terminal_is_empty()}
 /// 🪙️ Runs one local step; `true` once its birth is settled, `false` while the next turn must continue with the same frame.
 pub(crate) fn run(&mut self,cx:&mut StepContext<'_>,demand:OriginalConstructionDemand,build:impl FnOnce()->Result<RetainedCloneProgress,ValueError>)->Result<bool,OriginalTurnFailure>{
  let mut marker=self.marker.take();let settled=self.frame.built(cx,&mut marker,demand,||build().map(|progress|((),progress)));self.marker=marker;
  if settled?{self.marker=None;Ok(true)}else{Ok(false)}
 }
 /// 🏗️ Births one owned result into the caller's empty slot.
 pub(crate) fn into_slot<T>(&mut self,cx:&mut StepContext<'_>,slot:&mut Option<T>,demand:OriginalConstructionDemand,build:impl FnOnce()->Result<(T,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{self.frame.built(cx,slot,demand,build)}
 /// ♻️ Retires the marker, any retained failure and the frame; `true` once terminal.
 pub(crate) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.marker=None;if !self.frame.terminal_is_empty(){self.frame.close_step(cx)?;}Ok(self.frame.terminal_is_empty())
 }
 pub(crate) fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.frame.close_demand(body)}
}
pub(crate) fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
pub(crate) fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
pub(crate) fn local_progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}}
/// 🚪️ Starts the retirement of one resident owner through the shared close field.
pub(crate) fn begun<T:RetireOwned>(retirement:&mut RetainedCloneClose,slot:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
 retirement.begin_granted(slot,effect).result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(RetirementPollExit::from)
}
/// ♻️ Continues the started retirement of the shared close field.
pub(crate) fn resumed(retirement:&mut RetainedCloneClose,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{RetirementPollExit::resume(retirement.step_granted(body,effect))}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; `false` means it was not funded and nothing ran.
pub(crate) fn effected(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.should_yield()||!retained_clone_grant_admits(grant,d){return Ok(false)}
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_work(performed);result?;Ok(true)
}
/// 🪢️ Runs one reply-shaped callee and returns its recorded result; `None` means the callee suspended.
pub(crate) fn replied<E>(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->Result<Option<Result<RetainedCloneStep,E>>,ValueError>{
 let mut slot=None;let reply=OriginalEffectReply::borrow(&mut slot)?;action(&mut *effect,reply)?;Ok(slot.take())
}
/// 🪙️ Closes one owner step inside the context's retirement effect and settles its poll once.
pub(crate) fn closed(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 close_original_effect_step(cx,d,|effect|{let result=run(effect);RetirementPollExit::settle(effect,result)})
}
