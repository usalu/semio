//! ❓️ Unknown child kinds preserve their original bounded debug note without formatting an unfunded string.
use super::*;
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand};
use semio_framework_value::RetainedWorkOutcome;
const PREFIX:&str="[unknown widget kind ";
pub(super) struct UnknownTextWork{output:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,binding:Option<(usize,usize)>,offset:usize,length:usize,phase:u8,closing:bool}
impl UnknownTextWork{
 pub(super) fn new()->Self{Self{output:ManuallyDrop::new(None),retirement:Default::default(),binding:None,offset:0,length:PREFIX.len()+3,phase:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes:bytes,..fixed()};if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_bytes:bytes,..unit()})?;cx.consume_fuel(1);Ok(true)}
 fn end(&self,text:&str)->usize{let mut end=(self.offset+64).min(text.len());while !text.is_char_boundary(end){end-=1}end}
 fn width(ch:char)->usize{if ch=='\''{1}else{ch.escape_debug().map(char::len_utf8).sum()}}
 pub(super) fn step(&mut self,text:&str,cx:&mut StepContext<'_>)->Result<Option<String>,OriginalTurnFailure>{
  let binding=(text.as_ptr()as usize,text.len());if self.closing||self.binding.is_some_and(|b|b!=binding){return Err(invalid().into())}match self.phase{
   0=>if Self::paid(cx,size_of::<Self>())?{self.binding=Some(binding);self.phase=1},
   1=>{let end=self.end(text);let width=text[self.offset..end].chars().map(Self::width).sum::<usize>();if Self::paid(cx,end-self.offset+2*size_of::<usize>())?{self.length=self.length.checked_add(width).ok_or_else(invalid)?;self.offset=end;if end==text.len(){self.phase=2}}},
   2=>{let mut accepted=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:size_of::<Self>(),capacity_bytes:self.length,release_bytes:0,depth:1};admit_original_construction(cx,&mut accepted,demand,|_|{let result=(||{let mut output=String::new();output.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original unknown-kind note allocation refused"))?;let actual=output.capacity();*self.output=Some(output);self.offset=0;self.phase=3;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:actual,..Default::default()}))})();RetainedWorkOutcome{result,work:1}})?;},
   3=>if Self::paid(cx,PREFIX.len()+1)?{let output=self.output.as_mut().ok_or_else(invalid)?;output.push_str(PREFIX);output.push('"');self.phase=4},
   4=>{let end=self.end(text);let width=text[self.offset..end].chars().map(Self::width).sum::<usize>();if Self::paid(cx,end-self.offset+width+size_of::<usize>())?{let output=self.output.as_mut().ok_or_else(invalid)?;for ch in text[self.offset..end].chars(){if ch=='\''{output.push(ch)}else{for escaped in ch.escape_debug(){output.push(escaped)}}}self.offset=end;if end==text.len(){self.phase=5}}},
   5=>if Self::paid(cx,2)?{self.output.as_mut().ok_or_else(invalid)?.push_str("\"]");self.phase=6},
   6=>if Self::paid(cx,size_of::<Option<String>>())?{self.phase=7;return Ok(self.output.take())},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn begin_close(&mut self){self.closing=true}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.output.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}Ok(if self.binding.is_some(){RetirementDemand{copy_bytes:size_of::<Option<(usize,usize)>>(),..fixed()}}else{Default::default()})}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}
  if self.binding.is_some(){let d=RetirementDemand{copy_bytes:size_of::<Option<(usize,usize)>>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for UnknownTextWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"unknown-kind source text requires original output closure")}}
