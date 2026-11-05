//! 🔎️ Transform parameters read genuine native dictionaries with the original per-lane defaults.
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use semio_framework_artifact_flow_flow::neural::{Dictionary,Value,Atom};
use semio_os_flow::LookupCursor;
use semio_framework_job::{OriginalSourceBinding,StepContext,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[derive(Clone,Copy)]
pub(crate) enum ParamReadKind{Vector{key:&'static str,fallback:[f64;3]},Number{key:&'static str,fallback:f64}}
struct Fields{outer:Option<LookupCursor<Value>>,inner:Option<LookupCursor<Value>>,close:Option<ControlledRetirement<LookupCursor<Value>>>}
pub(crate) struct NativeParamReadWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,binding:Option<(usize,Option<usize>,usize)>,kind:ParamReadKind,values:[f64;3],field:usize,phase:u8,ready:bool,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"transform parameter reader changed its actual native source")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn number(value:Option<&Value>)->Option<f64>{match value?.as_atom()?{Atom::Integer(value)=>Some(*value as f64),Atom::Decimal(value)=>Some(*value),_=>None}}
impl NativeParamReadWork{
 pub(crate) fn new_original(kind:ParamReadKind,original:OriginalSourceBinding)->Self{let values=match kind{ParamReadKind::Vector{fallback,..}=>fallback,ParamReadKind::Number{fallback,..}=>[fallback,0.0,0.0]};Self{fields:ManuallyDrop::new(Fields{outer:None,inner:None,close:None}),original,binding:None,kind,values,field:0,phase:0,ready:false,closing:false,progress:Default::default(),work:0}}
 fn key(&self)->&'static str{match self.kind{ParamReadKind::Vector{key,..}|ParamReadKind::Number{key,..}=>key}}
 fn field_key(&self)->&'static str{match self.kind{ParamReadKind::Vector{..}=>match self.field{0=>"x",1=>"y",_=>"z"},ParamReadKind::Number{..}=>"value"}}
 fn count(&self)->usize{if matches!(self.kind,ParamReadKind::Vector{..}){3}else{1}}
 pub(crate) fn values(&self)->Option<&[f64;3]>{self.ready.then_some(&self.values)}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.ready}
 fn validate(&self,source:&Dictionary)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding!=(source as*const Dictionary as usize,source.original_root_identity(),source.len())){Err(invalid())}else{Ok(())}}
 pub(crate) fn next_demands(&self,source:&Dictionary,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.ready{return Ok(Default::default())}if let Some(close)=self.fields.close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  match self.phase{
   0=>Ok(source.pairs().lookup_borrowed_constructor_demands()),
   1=>self.fields.outer.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(self.key()),
   3=>Ok(self.fields.outer.as_ref().and_then(LookupCursor::result).and_then(Value::as_dictionary).ok_or_else(invalid)?.pairs().lookup_borrowed_constructor_demands()),
   4=>self.fields.inner.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(self.field_key()),
   5=>Ok(fixed(if matches!(self.fields.inner.as_ref().and_then(LookupCursor::result).and_then(Value::as_atom),Some(Atom::Integer(_)|Atom::Decimal(_))){8}else{0})),
   _=>Ok(fixed(0)),
  }
 }
 pub(crate) fn advance(&mut self,source:&Dictionary,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(source,grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,source:&Dictionary,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,d){return Ok(false)}
  self.binding=Some((source as*const Dictionary as usize,source.original_root_identity(),source.len()));
  if let Some(close)=self.fields.close.as_mut(){if close.terminal_is_empty(){self.fields.close=None;self.phase=7;self.work=1;self.progress.copied_items=1}else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?}return Ok(false)}
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   0=>{let turn=source.pairs().admit_lookup_borrowed_original(self.key(),grant);self.work=turn.work;self.progress=Default::default();let(lookup,p)=turn.result?;self.fields.outer=Some(lookup);self.progress=p;self.phase=1},
   1=>{let key=self.key();let outer=self.fields.outer.as_mut().ok_or_else(invalid)?;let turn=outer.advance_borrowed_original(key,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if outer.is_complete(){self.phase=2}},
   2=>{if self.fields.outer.as_ref().and_then(LookupCursor::result).and_then(Value::as_dictionary).is_none(){self.ready=true}else{self.phase=3}},
   3=>{let dictionary=self.fields.outer.as_ref().and_then(LookupCursor::result).and_then(Value::as_dictionary).ok_or_else(invalid)?;let turn=dictionary.pairs().admit_lookup_borrowed_original(self.field_key(),grant);self.work=turn.work;self.progress=Default::default();let(lookup,p)=turn.result?;self.fields.inner=Some(lookup);self.progress=p;self.phase=4},
   4=>{let key=self.field_key();let inner=self.fields.inner.as_mut().ok_or_else(invalid)?;let turn=inner.advance_borrowed_original(key,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if inner.is_complete(){self.phase=5}},
   5=>{if let Some(value)=number(self.fields.inner.as_ref().and_then(LookupCursor::result)){self.values[self.field]=value}self.phase=6},
   6=>{self.fields.close=Some(ControlledRetirement::new(self.fields.inner.take().ok_or_else(invalid)?).unwrap_or_else(|_|unreachable!("native parameter lookup supports controlled retirement")))},
   7=>{self.field+=1;if self.field==self.count(){self.ready=true}else{self.phase=3}},
   _=>return Err(invalid()),
  }Ok(self.ready)
 }
 pub(crate) fn step(&mut self,source:&Dictionary,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut source=source;drive(self,&mut source,cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(close)=self.fields.close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}Ok(if self.fields.outer.is_some()||self.fields.inner.is_some(){fixed(0)}else{Default::default()})}
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(close)=self.fields.close.as_mut(){if close.terminal_is_empty(){self.fields.close=None;self.work=1;self.progress.copied_items=1}else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?}}
  else{let lookup=self.fields.inner.take().or_else(||self.fields.outer.take()).ok_or_else(invalid)?;self.fields.close=Some(ControlledRetirement::new(lookup).unwrap_or_else(|_|unreachable!("native parameter lookup supports controlled retirement")));self.work=1;self.progress.copied_items=1}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.outer.is_none()&&self.fields.inner.is_none()&&self.fields.close.is_none()}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
 pub(crate) fn advancing_retirement(&self)->bool{self.fields.close.is_some()}
}
impl<'a> OriginalScratchWork<&'a Dictionary> for NativeParamReadWork{
 fn origin(&self)->(u64,u64){(self.original.operation().0,self.original.generation().0)}
 fn source_binding(&self)->OriginalSourceBinding{self.original}
 fn ready(&self)->bool{NativeParamReadWork::ready(self)}
 fn advancing_retirement(&self)->bool{NativeParamReadWork::advancing_retirement(self)}
 fn next_demands(&self,source:&&'a Dictionary,copy:usize)->Result<RetirementDemand,ValueError>{NativeParamReadWork::next_demands(self,source,copy)}
 fn advance(&mut self,source:&mut &'a Dictionary,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{NativeParamReadWork::advance(self,source,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for NativeParamReadWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"native parameter reader retains original map lookups");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
