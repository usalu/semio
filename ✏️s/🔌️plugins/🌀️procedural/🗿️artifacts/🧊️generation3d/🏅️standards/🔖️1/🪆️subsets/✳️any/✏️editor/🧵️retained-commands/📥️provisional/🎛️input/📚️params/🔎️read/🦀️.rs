//! 🔎️ Transform parameters read genuine native dictionaries with the original per-lane defaults.
use super::retained_scratch_driver::{OriginalScratchWork,drive,local_progress,pay};
use super::retained_scratch_lookup::ScratchMapLookup;
use semio_framework_artifact_flow_flow::neural::{Dictionary,Value,Atom};
use semio_framework_job::{OriginalSourceBinding,StepContext,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::ManuallyDrop;
#[derive(Clone,Copy)]
pub(crate) enum ParamReadKind{Vector{key:&'static str,fallback:[f64;3]},Number{key:&'static str,fallback:f64}}
struct Fields{outer:Option<ScratchMapLookup<Value>>,inner:Option<ScratchMapLookup<Value>>}
pub(crate) struct NativeParamReadWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,binding:Option<(usize,Option<usize>,usize)>,kind:ParamReadKind,values:[f64;3],field:usize,phase:u8,ready:bool,closing:bool,progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"transform parameter reader changed its actual native source")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn number(value:Option<&Value>)->Option<f64>{match value?.as_atom()?{Atom::Integer(value)=>Some(*value as f64),Atom::Decimal(value)=>Some(*value),_=>None}}
impl NativeParamReadWork{
 pub(crate) fn new_original(kind:ParamReadKind,original:OriginalSourceBinding)->Self{let values=match kind{ParamReadKind::Vector{fallback,..}=>fallback,ParamReadKind::Number{fallback,..}=>[fallback,0.0,0.0]};Self{fields:ManuallyDrop::new(Fields{outer:None,inner:None}),original,binding:None,kind,values,field:0,phase:0,ready:false,closing:false,progress:Default::default()}}
 fn key(&self)->&'static str{match self.kind{ParamReadKind::Vector{key,..}|ParamReadKind::Number{key,..}=>key}}
 fn field_key(&self)->&'static str{match self.kind{ParamReadKind::Vector{..}=>match self.field{0=>"x",1=>"y",_=>"z"},ParamReadKind::Number{..}=>"value"}}
 fn count(&self)->usize{if matches!(self.kind,ParamReadKind::Vector{..}){3}else{1}}
 fn dictionary(&self)->Result<&Dictionary,ValueError>{self.fields.outer.as_ref().and_then(ScratchMapLookup::result).and_then(Value::as_dictionary).ok_or_else(invalid)}
 pub(crate) fn values(&self)->Option<&[f64;3]>{self.ready.then_some(&self.values)}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.ready}
 fn validate(&self,source:&Dictionary)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding!=(source as*const Dictionary as usize,source.original_root_identity(),source.len())){Err(invalid())}else{Ok(())}}
 pub(crate) fn next_demands(&self,source:&Dictionary,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.ready{return Ok(Default::default())}
  match self.phase{
   1=>self.fields.outer.as_ref().ok_or_else(invalid)?.next_demands(source.pairs(),self.key(),copy),
   4=>self.fields.inner.as_ref().ok_or_else(invalid)?.next_demands(self.dictionary()?.pairs(),self.field_key(),copy),
   5=>Ok(fixed(if matches!(self.fields.inner.as_ref().and_then(ScratchMapLookup::result).and_then(Value::as_atom),Some(Atom::Integer(_)|Atom::Decimal(_))){8}else{0})),
   7=>match self.fields.inner.as_ref(){Some(inner)if !inner.terminal_is_empty()=>inner.close_demands(copy),_=>Ok(fixed(0))},
   _=>Ok(fixed(0)),
  }
 }
 pub(crate) fn advance(&mut self,source:&Dictionary,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();let result=self.advance_original(source,effect).map_err(|error|{if self.progress==Default::default(){self.progress=error.retained_progress()}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn advance_original(&mut self,source:&Dictionary,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let grant=effect.grant();let d=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,d){return Ok(false)}
  self.binding=Some((source as*const Dictionary as usize,source.original_root_identity(),source.len()));
  let local=matches!(self.phase,0|2|3|5|6);if local&&!pay(effect,1)?{return Ok(false)}if local{self.progress=local_progress(d)}
  match self.phase{
   0=>{self.fields.outer=Some(ScratchMapLookup::new());self.phase=1},
   1=>{let key=self.key();let outer=self.fields.outer.as_mut().ok_or_else(invalid)?;let done=outer.advance(source.pairs(),key,effect)?;self.progress=outer.step_progress();if done{self.phase=2}},
   2=>{if self.dictionary().is_err(){self.ready=true}else{self.phase=3}},
   3=>{self.fields.inner=Some(ScratchMapLookup::new());self.phase=4},
   4=>{let key=self.field_key();let fields=&mut*self.fields;let dictionary=fields.outer.as_ref().and_then(ScratchMapLookup::result).and_then(Value::as_dictionary).ok_or_else(invalid)?;let inner=fields.inner.as_mut().ok_or_else(invalid)?;let done=inner.advance(dictionary.pairs(),key,effect)?;self.progress=inner.step_progress();if done{self.phase=5}},
   5=>{if let Some(value)=number(self.fields.inner.as_ref().and_then(ScratchMapLookup::result)){self.values[self.field]=value}self.phase=6},
   6=>{self.fields.inner.as_mut().ok_or_else(invalid)?.begin_close();self.phase=7},
   7=>{
    let inner=self.fields.inner.as_mut().ok_or_else(invalid)?;
    if inner.terminal_is_empty(){if !pay(effect,1)?{return Ok(false)}self.fields.inner=None;self.progress=local_progress(d);self.field+=1;if self.field==self.count(){self.ready=true}else{self.phase=3}}
    else{self.progress=settled_progress(inner.close_step(grant.maximum_copy_bytes,effect))?}
   },
   _=>return Err(invalid()),
  }Ok(self.ready)
 }
 pub(crate) fn step(&mut self,source:&Dictionary,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut source=source;drive(self,&mut source,cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(inner)=self.fields.inner.as_mut(){inner.begin_close()}if let Some(outer)=self.fields.outer.as_mut(){outer.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  for lookup in [self.fields.inner.as_ref(),self.fields.outer.as_ref()].into_iter().flatten(){if !lookup.terminal_is_empty(){return lookup.close_demands(copy)}return Ok(fixed(0))}
  Ok(Default::default())
 }
 pub(crate) fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect).map_err(|exit|match exit{RetirementPollExit::Failed(failure)=>{if self.progress==Default::default(){self.progress=failure.retained_progress()}RetirementPollExit::Failed(failure.with_retained_progress(self.progress))},pending=>pending});RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let fields=&mut*self.fields;
  for slot in [&mut fields.inner,&mut fields.outer]{if let Some(lookup)=slot.as_mut(){if lookup.terminal_is_empty(){*slot=None;self.progress.copied_items=1}else{lookup.begin_close();self.progress=RetirementPollExit::resume(lookup.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.outer.as_ref().is_none_or(ScratchMapLookup::terminal_is_empty)&&self.fields.inner.as_ref().is_none_or(ScratchMapLookup::terminal_is_empty)}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn advancing_retirement(&self)->bool{self.phase==7&&self.fields.inner.is_some()}
}
fn settled_progress(poll:OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{Ok(super::retained_scratch_driver::settled_step(poll)?.progress())}
impl<'a> OriginalScratchWork<&'a Dictionary> for NativeParamReadWork{
 fn origin(&self)->(u64,u64){(self.original.operation().0,self.original.generation().0)}
 fn source_binding(&self)->OriginalSourceBinding{self.original}
 fn ready(&self)->bool{NativeParamReadWork::ready(self)}
 fn advancing_retirement(&self)->bool{NativeParamReadWork::advancing_retirement(self)}
 fn next_demands(&self,source:&&'a Dictionary,copy:usize,_work:u64)->Result<RetirementDemand,ValueError>{NativeParamReadWork::next_demands(self,source,copy)}
 fn advance(&mut self,source:&mut &'a Dictionary,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{NativeParamReadWork::advance(self,source,effect)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for NativeParamReadWork{fn drop(&mut self){assert!(std::thread::panicking()||(self.terminal_is_empty()),"native parameter reader retains original map lookups");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
