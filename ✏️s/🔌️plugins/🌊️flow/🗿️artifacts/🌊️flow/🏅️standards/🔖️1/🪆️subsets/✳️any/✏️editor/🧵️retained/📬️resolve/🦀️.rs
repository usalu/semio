//! 📬️ Canonical evaluation envelopes move their actual native output and pending fields under original caller funding.
use flow::{PreviewEvalPhase,PreviewEvalProgress};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,InteractiveJobCloseStep as Close,close_original_retained_step};
use semio_framework_value::{DslValue,Number,RetireOwned,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
const KEYS:[&str;7]=["cancellable","done","output","phase","unitsDone","unitsTotal","pending"];
const PENDING:[&str;5]=["extensionId","inputJson","neuronId","nodeHash","operatorId"];
#[derive(RetireOwned)]
pub enum FlowReplyEnvelope{Complete{output:DslValue,progress:PreviewEvalProgress},Pending{pending:flow::neural::PendingExtensionEval,progress:PreviewEvalProgress},Working(PreviewEvalProgress),Cancelled(PreviewEvalProgress)}
#[derive(RetireOwned)]
struct Fields{source:Option<DslValue>,output:Option<FlowReplyEnvelope>}
pub struct FlowReplyEnvelopeWork{fields:ManuallyDrop<Option<Fields>>,retirement:RetainedCloneClose,binding:Option<OriginalSourceBinding>,slots:[usize;7],pending:[usize;5],scan:usize,phase:u8,kind:u8,progress:PreviewEvalProgress,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original Flow evaluation envelope differs from the native reply contract")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<FlowReplyEnvelope>()+5*size_of::<DslValue>()+64,depth:1,..Default::default()}}
fn rows(value:&DslValue)->Result<&[(String,DslValue)],ValueError>{match value{DslValue::Object(rows)=>Ok(rows),_=>Err(invalid())}}
fn rows_mut(value:&mut DslValue)->Result<&mut[(String,DslValue)],ValueError>{match value{DslValue::Object(rows)=>Ok(rows),_=>Err(invalid())}}
fn text(value:&DslValue)->Result<&str,ValueError>{match value{DslValue::String(value)=>Ok(value),_=>Err(invalid())}}
fn flag(value:&DslValue)->Result<bool,ValueError>{match value{DslValue::Bool(value)=>Ok(*value),_=>Err(invalid())}}
fn unsigned(value:&DslValue)->Result<u64,ValueError>{match value{DslValue::Number(value)=>value.as_u64().ok_or_else(invalid),_=>Err(invalid())}}
fn take_text(value:&mut DslValue)->Result<String,ValueError>{if !matches!(value,DslValue::String(_)){return Err(invalid())}match std::mem::replace(value,DslValue::Null){DslValue::String(value)=>Ok(value),_=>unreachable!()}}
impl FlowReplyEnvelopeWork{
 pub fn new(source:DslValue)->Self{Self{fields:ManuallyDrop::new(Some(Fields{source:Some(source),output:None})),retirement:Default::default(),binding:None,slots:[usize::MAX;7],pending:[usize::MAX;5],scan:0,phase:0,kind:0,progress:Default::default(),closing:false}}
 pub fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<FlowReplyEnvelope>,OriginalTurnFailure>{for _ in 0..512{let before=(cx.retained_progress(),cx.original_total_fuel_remaining());if let Some(output)=self.once(cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.original_total_fuel_remaining()){break}}Ok(None)}
 fn once(&mut self,cx:&mut StepContext<'_>)->Result<Option<FlowReplyEnvelope>,OriginalTurnFailure>{
  if self.closing||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(fixed()){return Ok(None)}
  self.binding=Some(OriginalSourceBinding::borrow(cx));cx.consume_fuel(1);let p=RetainedCloneProgress{copied_items:1,copied_bytes:fixed().copy_bytes,..Default::default()};
  let result=(||->Result<Option<FlowReplyEnvelope>,ValueError>{let fields=self.fields.as_mut().ok_or_else(invalid)?;match self.phase{
   0=>{let source=rows(fields.source.as_ref().ok_or_else(invalid)?)?;if !(6..=7).contains(&source.len()){return Err(invalid())}if let Some((key,_))=source.get(self.scan){if key.len()>11{return Err(invalid())}let slot=KEYS.iter().position(|candidate|*candidate==key).ok_or_else(invalid)?;if self.slots[slot]!=usize::MAX{return Err(invalid())}self.slots[slot]=self.scan;self.scan+=1}else{if self.slots[..6].contains(&usize::MAX){return Err(invalid())}self.phase=1}},
   1=>{let source=rows(fields.source.as_ref().ok_or_else(invalid)?)?;let get=|slot:usize|&source[self.slots[slot]].1;let done=flag(get(1))?;if flag(get(0))?==done{return Err(invalid())}let tag=text(get(3))?;if tag.len()>64{return Err(invalid())}self.progress=PreviewEvalProgress{phase:PreviewEvalPhase::from_job_tag(tag),units_done:unsigned(get(4))?,units_total:unsigned(get(5))?};if self.progress.units_done>self.progress.units_total{return Err(invalid())}
    if self.slots[6]!=usize::MAX{if done||tag!="external-invoke"||!matches!(get(2),DslValue::Null)||rows(get(6))?.len()!=5{return Err(invalid())}self.scan=0;self.kind=1;self.phase=2}
    else if !done{if matches!(self.progress.phase,PreviewEvalPhase::Complete|PreviewEvalPhase::Cancelled)||!matches!(get(2),DslValue::Null){return Err(invalid())}self.kind=2;self.phase=3}
    else if tag=="cancelled"&&matches!(get(2),DslValue::Null){self.kind=3;self.phase=3}
    else if tag=="complete"&&self.progress.units_done==self.progress.units_total&&matches!(get(2),DslValue::Object(_)){self.kind=0;self.phase=3}
    else{return Err(invalid())}},
   2=>{let source=rows(fields.source.as_ref().ok_or_else(invalid)?)?;let pending=rows(&source[self.slots[6]].1)?;if let Some((key,value))=pending.get(self.scan){if key.len()>11{return Err(invalid())}let slot=PENDING.iter().position(|candidate|*candidate==key).ok_or_else(invalid)?;if self.pending[slot]!=usize::MAX{return Err(invalid())}if slot==3{unsigned(value)?;}else{text(value)?;}self.pending[slot]=self.scan;self.scan+=1}else{if self.pending.contains(&usize::MAX){return Err(invalid())}self.phase=3}},
   3=>{let source=rows_mut(fields.source.as_mut().ok_or_else(invalid)?)?;fields.output=Some(match self.kind{
    0=>FlowReplyEnvelope::Complete{output:std::mem::replace(&mut source[self.slots[2]].1,DslValue::Null),progress:self.progress},
    1=>{let pending=rows_mut(&mut source[self.slots[6]].1)?;let hash=unsigned(&pending[self.pending[3]].1)?;FlowReplyEnvelope::Pending{pending:flow::neural::PendingExtensionEval{extension_id:take_text(&mut pending[self.pending[0]].1)?,input_json:take_text(&mut pending[self.pending[1]].1)?,neuron_id:take_text(&mut pending[self.pending[2]].1)?,node_hash:hash,operator_id:take_text(&mut pending[self.pending[4]].1)?},progress:self.progress}},
    2=>FlowReplyEnvelope::Working(self.progress),3=>FlowReplyEnvelope::Cancelled(self.progress),_=>return Err(invalid())});self.phase=4},
   4=>{self.phase=5;return Ok(fields.output.take())},
   _=>return Err(invalid()),
  }Ok(None)})();
  match result{Ok(output)=>{if let Err(error)=cx.consume_retained(p){if let Some(output)=output{self.restore_output(output)}return Err(error.into())}Ok(output)},Err(error)=>Err(OriginalTurnFailure::debit_original(error.with_retained_progress(p),cx))}
 }
 pub fn restore_output(&mut self,output:FlowReplyEnvelope){assert_eq!(self.phase,5);self.fields.as_mut().expect("original reply fields").output=Some(output);self.phase=4}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,body)}Ok(RetirementDemand{copy_bytes:size_of::<Option<OriginalSourceBinding>>(),depth:usize::from(self.binding.is_some()),..Default::default()})}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{if !self.closing||self.binding.is_some_and(|binding|!binding.matches(cx)){return Err(invalid().into())}let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.fields.is_some(){let turn=self.retirement.begin_granted(&mut self.fields,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}if self.binding.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for FlowReplyEnvelopeWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original native reply requires paid closure of source and output");unsafe{ManuallyDrop::drop(&mut self.fields)}}}

#[cfg(test)]
#[path="🧪️envelope.rs"]
mod tests;

#[path="⚠️encoding.rs"]
mod encoding;


#[path="📡️resume.rs"]
mod resume;

#[path="🌱️origin.rs"]
mod origin;

#[path="⏱️effect.rs"]
mod effect;

#[path="🧵️job.rs"]
mod job;
pub use job::FlowResolveJobWork;
