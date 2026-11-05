//! 🎟️ Procedural scratch producers share the original caller's work and retirement driver.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub(crate) trait OriginalScratchWork<S>{
 fn origin(&self)->(u64,u64);
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding;
 fn ready(&self)->bool;
 fn advancing_retirement(&self)->bool;
 fn next_demands(&self,source:&S,copy:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,source:&mut S,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>;
 fn step_progress(&self)->RetainedCloneProgress;
}
pub(crate) fn drive<S,W:OriginalScratchWork<S>>(work:&mut W,source:&mut S,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if work.origin()!=(cx.operation().0,cx.generation().0)||!work.source_binding().matches(cx){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"scratch work requires its initiating caller").into())}
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 for _ in 0..512{
  if work.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  if work.advancing_retirement(){
   let demand=work.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;
   let step=cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|{
    let turn=work.advance(source,grant);let progress=work.step_progress();
    RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(progress)),work:turn.work}
   }))?;
   if step.progress()==Default::default(){return Ok(work.ready())}
  }else{
   let turn=work.advance(source,cx.retained_grant());let progress=work.step_progress();
   let step=semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(progress)),work:turn.work})?;
   if work.ready()||step.progress()==Default::default(){return Ok(work.ready())}
  }
 }
 Ok(work.ready())
}
