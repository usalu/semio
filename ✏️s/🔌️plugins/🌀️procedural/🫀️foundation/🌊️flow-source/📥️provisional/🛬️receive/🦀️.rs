//! 🛬️ Provisional native receiving consumes genuine field projections under the original decoder.
use super::super::retained_scratch_driver::{effected,receive,receiver_close,receiving_fault};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{DslValue,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,retained_clone_grant_admits};
use semio_framework_pack_json::native_decoding::owned::NativeReceive;
use std::mem::ManuallyDrop;
pub struct ProvisionalFieldWork<T:NativeReceive<DslValue>>{receiver:ManuallyDrop<Option<NativeReceivingOriginal<DslValue,T>>>,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),closing:bool}
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver is absent")}
impl<T:NativeReceive<DslValue>> ProvisionalFieldWork<T>{
 pub fn new(source:RetainedOwnedProjection<DslValue>,cx:&StepContext<'_>)->Self{Self::new_original(source,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(source:RetainedOwnedProjection<DslValue>,original_caller:semio_framework_job::OriginalSourceBinding)->Self{Self{receiver:ManuallyDrop::new(Some(NativeReceivingOriginal::from_projection(source))),original_caller,operation:(original_caller.operation().0,original_caller.generation().0),closing:false}}
 fn authenticate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.operation==(cx.operation().0,cx.generation().0)&&self.original_caller.matches(cx)&&!self.closing{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiving requires its initiating caller").into())}}
 pub fn ready(&self)->bool{!self.closing&&self.receiver.as_ref().is_some_and(NativeReceivingOriginal::output_ready)}
 pub fn next_demands(&self,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{self.receiver.as_ref().ok_or_else(absent)?.demands(copy,work)}
 pub fn take_demands(&self)->Result<RetirementDemand,ValueError>{self.receiver.as_ref().ok_or_else(absent)?.take_demand()}
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;
  for _ in 0..512{
   if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let grant=cx.retained_grant();let body=grant.maximum_copy_bytes;let d=self.next_demands(body,cx.fuel_remaining())?;if !retained_clone_grant_admits(grant,d){return Ok(false)}
   let before=cx.retained_progress();let receiver=self.receiver.as_mut().ok_or_else(absent)?;
   receive(cx,d,|native,maximum,grant|{let witness=match native.source_witness(){Ok(witness)=>witness,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};receiver.advance(native,witness,body,maximum,grant)})?;
   if cx.retained_progress()==before{return Ok(self.ready())}
  }Ok(self.ready())
 }
 pub fn take_into(&mut self,output:&mut Option<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.authenticate(cx)?;let receiver=self.receiver.as_mut().ok_or_else(absent)?;let demand=receiver.take_demand()?;
  effected(cx,demand,|effect|match receiver.take(output,effect){RetainedWorkPoll::Complete(turn)=>turn.result.map_err(receiving_fault),RetainedWorkPoll::Pending{..}=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver output handoff suspended"))})?;
  Ok(output.is_some())
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{match self.receiver.as_ref(){Some(receiver)if !receiver.terminal_is_empty()=>if receiver.is_closing(){receiver.close_demands(copy)}else{receiver.begin_close_demand()},Some(_)=>Ok(RetirementDemand{depth:1,..Default::default()}),None=>Ok(Default::default())}}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let done=|effect:&RetainedAllocationEffect<'_>,result|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()});
  if !self.closing{return done(effect,Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional native receiver close was not begun").into()))}
  let Some(receiver)=self.receiver.as_mut()else{return done(effect,Ok(RetainedCloneStep::Complete(Default::default())))};
  if !receiver.terminal_is_empty(){return receiver_close(receiver,body,effect)}
  let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return done(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  match effect.performed(1){Ok(true)=>{self.receiver.take();done(effect,Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})))},Ok(false)=>done(effect,Ok(RetainedCloneStep::Progress(Default::default()))),Err(error)=>done(effect,Err(error.into()))}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.receiver.is_none()}
}
impl<T:NativeReceive<DslValue>> Drop for ProvisionalFieldWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional native receiver retains its original field or output");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.receiver)}}}}
