//! 🔤️ Original typed parameter fields become exact canonical JSON without an intermediate value tree.
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeWireCursor};
use semio_framework_value::{RetireOwned,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::RetainedAllocationEffect;
use super::{begin_close_owner,poll_outcome};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow parameter JSON frontier changed")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
/// 📨️ The actual typed field and its resulting UTF-8 allocation remain owned until handoff or closure.
pub(super) struct ParamJsonWork<T:ArtifactCanonicalJsonTree+RetireOwned>{cursor:Option<ArtifactCanonicalJsonTreeWireCursor<T>>,output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,origin:Option<(u64,u64)>,closing:bool}
impl<T:ArtifactCanonicalJsonTree+RetireOwned> ParamJsonWork<T>{
 pub(super) fn new(original:T)->Self{Self{cursor:Some(ArtifactCanonicalJsonTreeWireCursor::new(original)),output:ManuallyDrop::new(None),retirement:Default::default(),origin:None,closing:false}}
 pub(super) fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(copy)}if self.origin.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),..fixed()})}if self.output.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<String>(),..fixed()})}self.cursor.as_ref().ok_or_else(invalid)?.next_demand(copy)}
 pub(super) fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<String>,OriginalTurnFailure>{
  for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.once(cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(None)
 }
 fn once(&mut self,cx:&mut StepContext<'_>)->Result<Option<String>,OriginalTurnFailure>{
  let origin=(cx.operation().0,cx.generation().0);if self.closing||self.origin.is_some_and(|o|o!=origin){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}let d=self.demands(cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}
  if self.origin.is_none(){self.origin=Some(origin);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})?;return Ok(None)}
  if self.output.is_some(){cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})?;return Ok(self.output.take())}
  let cursor=self.cursor.as_mut().ok_or_else(invalid)?;let grant=cx.retained_grant();let Some((result,performed))=cx.with_original_effect(grant,|_,effect|cursor.advance(effect))?else{return Ok(None)};cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));match result{
   Ok(step)=>{if let Some(bytes)=step.output{self.output=ManuallyDrop::new(Some(canonical_text(bytes)))}cx.consume_retained(step.progress)?;Ok(None)},
   Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))
  }
 }
 pub(super) fn restore_output(&mut self,output:String){assert!(self.output.is_none());self.output=ManuallyDrop::new(Some(output))}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.cursor.as_mut(){cursor.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(cursor)=self.cursor.as_ref(){return if cursor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{cursor.retirement_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),..fixed()})}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  if let Some(cursor)=self.cursor.as_mut(){if !cursor.terminal_is_empty(){let result=cursor.close_step(effect).map(RetainedCloneStep::Progress);return RetainedWorkOutcome{result,work:effect.performed_work()}}let d=RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.cursor=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}if self.output.is_some(){return begin_close_owner(&mut self.retirement,&mut self.output,effect)}
  let d=RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.origin=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.cursor.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.origin.is_none()}
}
impl<T:ArtifactCanonicalJsonTree+RetireOwned> Drop for ParamJsonWork<T>{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Flow parameter JSON requires terminal typed-field and output custody");unsafe{ManuallyDrop::drop(&mut self.output)}}}

/// 🔡️ The canonical tree cursor emits only JSON punctuation, ASCII scalar words, and escaped original UTF-8 text.
fn canonical_text(bytes:Vec<u8>)->String{unsafe{String::from_utf8_unchecked(bytes)}}
