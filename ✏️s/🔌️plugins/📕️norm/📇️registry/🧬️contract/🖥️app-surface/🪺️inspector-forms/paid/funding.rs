//! ♻️ Shared funded-turn and close-ladder plumbing of the prepared inspector sources.
use semio_framework_value::{NativeDecodeControl,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::RetirementStep;
/// 🧾️ Records work units on the original effect; an unfunded unit is a typed work-limit refusal.
pub(super) fn pay(effect:&mut RetainedAllocationEffect<'_>,units:u64)->Result<(),ValueError>{if effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"norm.inspector work exceeds its original authority"))}}
/// 🏦️ Runs one transition inside its own allocated effect; `None` means the original work quote was not admitted.
pub(super) fn run<R>(control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,units:u64,work:&mut u128,operation:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->Result<R,ValueError>)->Result<Option<R>,ValueError>{
 match control.with_allocated_effect(demand,units,|control,effect|{let before=effect.performed_work();let result=operation(control,effect);(result,effect.performed_work().saturating_sub(before))})?{
  None=>Ok(None),
  Some((result,spent))=>{*work+=spent;result.map(Some)}
 }
}
/// 🪢️ Maps one settled close poll onto the retirement-cursor step that keeps its exact receipt and work.
pub(super) fn step(poll:OriginalRetirementPoll)->RetirementStep{
 match poll{
  RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),
  RetainedWorkPoll::Complete(turn)=>match turn.result{
   Err(error)=>RetirementStep::Failure(error,turn.work),
   Ok(RetainedCloneStep::Complete(progress))if progress==Default::default()=>RetirementStep::Complete(turn.work),
   Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work),
  },
 }
}
/// 📐️ Derives every secondary cursor quote from the one authored next-close demand.
macro_rules! derived_cursor_quotes{()=>{
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(usize::MAX)?.depth)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
}}
