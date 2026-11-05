//! 🌊️ Procedural jobs retain their genuine Store read while a native Flow field becomes their stable source.
use semio_os_kernel::os_store::snapshot_read::{SnapshotRead,SnapshotReadFieldClone};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::mem::ManuallyDrop;
pub struct ProceduralFlowSourceWork<T:RetireOwned+Sync>{field:SnapshotReadFieldClone<T,FlowHostSnapshot>,snapshot:ManuallyDrop<Option<FlowHostSnapshot>>,close:Option<ControlledRetirement<FlowHostSnapshot>>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,copy:bool,detached:bool,original_caller:semio_framework_job::OriginalSourceBinding,operation:(u64,u64),closing:bool,progress:RetainedCloneProgress,work:u64}
impl<T:RetireOwned+Sync> ProceduralFlowSourceWork<T>{
 pub fn new(read:SnapshotRead<T>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,revision:[u8;32],generation:u64,copy:bool,cx:&StepContext<'_>)->Self{Self::new_original(read,project,revision,generation,copy,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(read:SnapshotRead<T>,project:for<'a>fn(&'a T)->&'a FlowHostSnapshot,revision:[u8;32],generation:u64,copy:bool,original:semio_framework_job::OriginalSourceBinding)->Self{Self{field:SnapshotReadFieldClone::new(read,project,1,revision,generation),snapshot:ManuallyDrop::new(None),close:None,project,copy,detached:false,original_caller:original,operation:(original.operation().0,original.generation().0),closing:false,progress:Default::default(),work:0}}
 pub fn original(&self)->Option<&SnapshotRead<T>>{self.field.original()}
 pub fn source(&self)->Result<Option<&FlowHostSnapshot>,ValueError>{self.field.validate_current()?;Ok(if self.copy{self.snapshot.as_ref()}else{self.field.original().map(|read|(self.project)(read.get()))})}

 pub fn original_source_demands(&self)->Result<RetirementDemand,ValueError>{self.field.next_source_demands()}
 pub fn prepare_original_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.operation!=(cx.operation().0,cx.generation().0)||!self.original_caller.matches(cx){return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  self.progress=Default::default();self.work=0;
  for _ in 0..512{
   if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"procedural source is closing").into())}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
   let turn=self.field.prepare_source(cx.retained_grant());let ready=turn.result.as_ref().copied().unwrap_or(false);let p=self.field.step_progress();self.work+=turn.work;
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(p)),work:turn.work})?;
   if ready||p==Default::default(){return Ok(ready)}
  }Ok(false)
 }
 pub fn project_original<V:?Sized+Sync>(&self,discriminator:usize,project:for<'a>fn(&'a T)->&'a V,grant:RetainedCloneGrant)->RetainedWorkOutcome<(semio_framework_value::retained_clone::RetainedOwnedProjection<V>,RetainedCloneProgress)>{self.field.project_original(discriminator,project,grant)}


 /// 📦️ Quotes a move of the actual owned native scratch field into or out of its retained source.
 pub fn scratch_transfer_demands(&self)->Result<RetirementDemand,ValueError>{
  self.field.validate_current()?;if self.closing||!self.copy{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch transfer requires its live copied source"))}
  Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<FlowHostSnapshot>>()+std::mem::size_of::<bool>(),depth:1,..Default::default()})
 }
 /// 📤️ Moves the original scratch allocation into a vacant continuation slot without cloning its fields.
 pub fn take_scratch_original(&mut self,output:&mut Option<FlowHostSnapshot>,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.transfer_scratch_original(output,false,grant)}
 /// 📥️ Restores that actual owned scratch field after its temporary immutable source releases authority.
 pub fn restore_scratch_original(&mut self,input:&mut Option<FlowHostSnapshot>,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.transfer_scratch_original(input,true,grant)}

 pub fn take_scratch_into(&mut self,output:&mut Option<FlowHostSnapshot>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.transfer_scratch(output,false,cx)}
 pub fn restore_scratch_from(&mut self,input:&mut Option<FlowHostSnapshot>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.transfer_scratch(input,true,cx)}
 fn transfer_scratch(&mut self,slot:&mut Option<FlowHostSnapshot>,restore:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.original_caller.matches(cx){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch handoff requires its initiating caller").into())}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let turn=self.transfer_scratch_original(slot,restore,cx.retained_grant());let ready=turn.result.as_ref().copied().unwrap_or(false);
  semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(self.progress)),work:turn.work})?;
  Ok(ready)
 }
 fn transfer_scratch_original(&mut self,slot:&mut Option<FlowHostSnapshot>,restore:bool,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;
  let result=(||{
   let demand=self.scratch_transfer_demands()?;
   if restore{if !self.detached||self.snapshot.is_some()||slot.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch restore changed original ownership"))}}
   else if self.detached||self.snapshot.is_none()||slot.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch handoff requires its vacant receiver"))}
   if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false)}
   if restore{*self.snapshot=slot.take()}else{*slot=self.snapshot.take()}self.detached=!restore;
   self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};Ok(true)
  })();RetainedWorkOutcome{result,work:self.work}
 }
 pub fn source_mut(&mut self)->Result<Option<&mut FlowHostSnapshot>,ValueError>{self.field.validate_current()?;Ok(self.snapshot.as_mut())}
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.next_close_demands(copy)}self.field.validate_current()?;if self.detached{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch is held by its original handoff"))}if !self.copy||self.snapshot.is_some(){Ok(Default::default())}else{self.field.next_demands(copy)}}
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let operation=(cx.operation().0,cx.generation().0);if self.operation!=operation||!self.original_caller.matches(cx){return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source operation changed"),cx))}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  self.progress=Default::default();self.work=0;
  for _ in 0..512{
   if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"procedural source is closing").into())}
   if cx.is_cancelled()||cx.should_yield(){return Ok(false)}self.field.validate_current()?;if self.detached{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural scratch is held by its original handoff").into())}if !self.copy||self.snapshot.is_some(){return Ok(true)}
   let turn=self.field.advance(cx.retained_grant());let p=self.field.step_progress();self.work+=turn.work;
   let result=turn.result.map(|snapshot|{if let Some(snapshot)=snapshot{*self.snapshot=Some(snapshot)}RetainedCloneStep::Progress(p)});
   self.progress=self.progress.checked_add(p).map_err(|error|OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))?;
   semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;
   if self.snapshot.is_some()||p==Default::default(){return Ok(self.snapshot.is_some())}
  }Ok(false)
 }
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close()}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}if self.snapshot.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}self.field.next_close_demands(copy)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"procedural source close was not begun"))}
  if let Some(close)=self.close.as_mut(){let result=close.step(grant);self.progress=close.step_progress();self.work=close.last_turn_work();let step=result?;if close.terminal_is_empty(){self.close=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if self.snapshot.is_some(){if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let snapshot=self.snapshot.take().unwrap();self.close=Some(ControlledRetirement::new(snapshot).unwrap_or_else(|_|unreachable!("native Flow snapshot supports original retirement")));self.work=1;self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  let turn=self.field.close_step(grant);self.progress=self.field.step_progress();self.work=turn.work;turn.result
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.snapshot.is_none()&&self.close.is_none()&&self.field.terminal_is_empty()}
}
impl<T:RetireOwned+Sync> Drop for ProceduralFlowSourceWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"procedural source retains its captured native read or snapshot");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot)}}}}
