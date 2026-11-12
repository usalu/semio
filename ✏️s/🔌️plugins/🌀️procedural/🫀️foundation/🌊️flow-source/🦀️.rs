//! 🌊️ Procedural jobs retain their genuine Store read while a native Flow field becomes their stable source.
use semio_os_kernel::os_store::snapshot_read::{SnapshotRead,SnapshotReadFieldClone};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,ValueError,ValueRefusalKind,RetirementDemand,RetainedWorkOutcome,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedNativeFieldProjection,RetainedOwnedProjectionAdmission,retained_clone_grant_admits};
use super::retained_scratch_driver::{funded_turn,local_progress,pay};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::ManuallyDrop;
pub struct ProceduralFlowSourceWork<T:RetireOwned+Sync>{field:SnapshotReadFieldClone<T,FlowHostSnapshot>,snapshot:ManuallyDrop<Option<FlowHostSnapshot>>,close:Option<ControlledRetirement<FlowHostSnapshot>>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,copy:bool,detached:bool,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),closing:bool,progress:RetainedCloneProgress}
impl<T:RetireOwned+Sync> ProceduralFlowSourceWork<T>{
 /// 🪪️ Binds one genuine read to its stable Flow field.
 ///
 /// # Safety
 /// `project` must return only the Flow snapshot field of its input and be bounded, callback-free, nonallocating and nonunwinding.
 pub unsafe fn new(read:SnapshotRead<T>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,revision:[u8;32],generation:u64,copy:bool,cx:&StepContext<'_>)->Self{unsafe{Self::new_original(read,project,revision,generation,copy,semio_framework_job::OriginalSourceBinding::borrow(cx))}}
 /// 🪪️ Binds one genuine read to its stable Flow field under an existing original binding.
 ///
 /// # Safety
 /// See [`Self::new`].
 pub unsafe fn new_original(read:SnapshotRead<T>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,revision:[u8;32],generation:u64,copy:bool,original:semio_framework_job::OriginalSourceBinding)->Self{Self{field:SnapshotReadFieldClone::new(read,unsafe{RetainedNativeFieldProjection::new(project,1)},revision,generation),snapshot:ManuallyDrop::new(None),close:None,project,copy,detached:false,original_caller:original,operation:(original.operation().0,original.generation().0),closing:false,progress:Default::default()}}
 pub fn original(&self)->Option<&SnapshotRead<T>>{self.field.original()}
 pub fn source(&self)->Result<Option<&FlowHostSnapshot>,ValueError>{self.field.validate_current()?;Ok(if self.copy{self.snapshot.as_ref()}else{self.field.original().map(|read|(self.project)(read.get()))})}

 pub fn original_source_demands(&self,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.field.next_source_demands(maximum_work_units)}
 pub fn prepare_original_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.operation!=(cx.operation().0,cx.generation().0)||!self.original_caller.matches(cx){return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  self.progress=Default::default();
  for _ in 0..512{
   if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"procedural source is closing").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let mut ready=false;
   let step=funded_turn(cx,|effect|{let turn=self.field.prepare_source(effect);ready=turn.result.as_ref().copied().unwrap_or(false);let p=self.field.step_progress();RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(p)),work:turn.work}})?;
   let Some(step)=step else{return Ok(false)};let p=step.progress();
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   if ready||p==Default::default(){return Ok(ready)}
  }Ok(false)
 }
 pub fn project_original<'reply,V:?Sized+Sync>(&self,witness:&RetainedNativeFieldProjection<T,V>,output:&mut RetainedOwnedProjectionAdmission<V>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{self.field.project_original(witness,output,effect,reply)}


 /// 📦️ Quotes a move of the actual owned native scratch field into or out of its retained source.
 pub fn scratch_transfer_demands(&self)->Result<RetirementDemand,ValueError>{
  self.field.validate_current()?;if self.closing||!self.copy{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch transfer requires its live copied source"))}
  Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<FlowHostSnapshot>>()+std::mem::size_of::<bool>(),depth:1,..Default::default()})
 }
 /// 📤️ Moves the original scratch allocation into a vacant continuation slot without cloning its fields.
 pub fn take_scratch_original(&mut self,output:&mut Option<FlowHostSnapshot>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.transfer_scratch_original(output,false,effect)}
 /// 📥️ Restores that actual owned scratch field after its temporary immutable source releases authority.
 pub fn restore_scratch_original(&mut self,input:&mut Option<FlowHostSnapshot>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.transfer_scratch_original(input,true,effect)}

 pub fn take_scratch_into(&mut self,output:&mut Option<FlowHostSnapshot>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.transfer_scratch(output,false,cx)}
 pub fn restore_scratch_from(&mut self,input:&mut Option<FlowHostSnapshot>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.transfer_scratch(input,true,cx)}
 fn transfer_scratch(&mut self,slot:&mut Option<FlowHostSnapshot>,restore:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.original_caller.matches(cx){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch handoff requires its initiating caller").into())}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let mut ready=false;
  let step=funded_turn(cx,|effect|{let turn=self.transfer_scratch_original(slot,restore,effect);ready=turn.result.as_ref().copied().unwrap_or(false);RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(self.progress)),work:turn.work}})?;
  Ok(step.is_some()&&ready)
 }
 fn transfer_scratch_original(&mut self,slot:&mut Option<FlowHostSnapshot>,restore:bool,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();
  let result=(||{
   let demand=self.scratch_transfer_demands()?;
   if restore{if !self.detached||self.snapshot.is_some()||slot.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch restore changed original ownership"))}}
   else if self.detached||self.snapshot.is_none()||slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch handoff requires its vacant receiver"))}
   if !retained_clone_grant_admits(effect.grant(),demand)||!pay(effect,1)?{return Ok(false)}
   if restore{*self.snapshot=slot.take()}else{*slot=self.snapshot.take()}self.detached=!restore;
   self.progress=local_progress(demand);Ok(true)
  })();RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 pub fn source_mut(&mut self)->Result<Option<&mut FlowHostSnapshot>,ValueError>{self.field.validate_current()?;Ok(self.snapshot.as_mut())}
 pub fn next_demands(&self,copy:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return self.next_close_demands(copy)}self.field.validate_current()?;if self.detached{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch is held by its original handoff"))}if !self.copy||self.snapshot.is_some(){Ok(Default::default())}else{self.field.next_demands(copy,maximum_work_units)}}
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let operation=(cx.operation().0,cx.generation().0);if self.operation!=operation||!self.original_caller.matches(cx){return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  self.progress=Default::default();
  for _ in 0..512{
   if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"procedural source is closing").into())}
   if cx.is_cancelled()||cx.should_yield(){return Ok(false)}self.field.validate_current()?;if self.detached{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch is held by its original handoff").into())}if !self.copy||self.snapshot.is_some(){return Ok(true)}
   let step=funded_turn(cx,|effect|{let turn=self.field.advance(effect);let p=self.field.step_progress();let result=turn.result.map(|snapshot|{if let Some(snapshot)=snapshot{*self.snapshot=Some(snapshot)}RetainedCloneStep::Progress(p)});RetainedWorkOutcome{result,work:turn.work}})?;
   let Some(step)=step else{return Ok(false)};let p=step.progress();
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   if self.snapshot.is_some()||p==Default::default(){return Ok(self.snapshot.is_some())}
  }Ok(false)
 }
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close()}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}if self.snapshot.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}self.field.next_close_demands(copy)}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source close was not begun").into())}
  if let Some(close)=self.close.as_mut(){let step=RetirementPollExit::resume(close.step(body,effect))?;self.progress=close.step_progress();if close.terminal_is_empty(){self.close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.snapshot.is_some(){let grant=effect.grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let snapshot=self.snapshot.take().unwrap();self.close=Some(ControlledRetirement::new(snapshot).unwrap_or_else(|_|unreachable!("native Flow snapshot supports original retirement")));self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  let step=RetirementPollExit::resume(self.field.close_step(effect))?;self.progress=self.field.step_progress();Ok(step)
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.snapshot.is_none()&&self.close.is_none()&&self.field.terminal_is_empty()}
}
impl<T:RetireOwned+Sync> Drop for ProceduralFlowSourceWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"procedural source retains its captured native read or snapshot");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot)}}}}
