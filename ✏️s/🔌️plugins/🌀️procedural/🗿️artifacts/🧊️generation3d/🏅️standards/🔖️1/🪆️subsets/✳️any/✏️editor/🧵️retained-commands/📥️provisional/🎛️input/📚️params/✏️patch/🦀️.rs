//! ✏️ Native parameter upserts retain actual input owners through bounded text and AVL events.
use semio_framework_artifact_flow_flow::neural::{Dictionary,Value};
use semio_os_flow::{OrderedMap,OriginalOwnedEntry,FlowTextCopyWork};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
const META:usize=2*size_of::<usize>()+size_of::<u8>();
#[derive(Clone,Copy)]
struct Source{dictionary:usize,root:Option<usize>,key:usize,key_length:usize}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Text,TextClose,Entry,Advance,EntryClose,Publish,Ready,Closed}
struct Owners{value:Option<Value>,key:Option<String>,output:Option<OrderedMap<Value>>}
pub(crate) struct NativeParamPatchWork{owners:ManuallyDrop<Owners>,text:Option<FlowTextCopyWork>,entry:Option<OriginalOwnedEntry<Value>>,retirement:RetainedCloneClose,binding:Option<OriginalSourceBinding>,source:Option<Source>,phase:Phase,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original native parameter patch changed its source or custody")}
impl NativeParamPatchWork{
 pub(crate) fn new(value:Value,cx:&StepContext<'_>)->Self{Self::new_original(value,OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(value:Value,binding:OriginalSourceBinding)->Self{Self{owners:ManuallyDrop::new(Owners{value:Some(value),key:None,output:None}),text:None,entry:None,retirement:Default::default(),binding:Some(binding),source:None,phase:Phase::Bind,closing:false,progress:Default::default(),work:0}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub(crate) fn applied(&self)->bool{self.ready()}
 pub(crate) fn source_binding(&self)->OriginalSourceBinding{self.binding.expect("live original parameter patch binding")}
 pub(crate) fn step(&mut self,params:&mut Dictionary,key:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut source=(params,key);super::retained_scratch_driver::drive(self,&mut source,cx)}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
 pub(crate) fn advancing_retirement(&self)->bool{matches!(self.phase,Phase::TextClose|Phase::EntryClose)||self.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
 fn source(params:&Dictionary,key:&str)->Source{Source{dictionary:params as*const _ as usize,root:params.original_root_identity(),key:key.as_ptr()as usize,key_length:key.len()}}
 fn checked(&self,params:&Dictionary,key:&str)->Result<(),ValueError>{
  if self.closing{return Err(invalid())}
  if let Some(original)=self.source{let current=Self::source(params,key);if original.dictionary!=current.dictionary||original.root!=current.root||original.key!=current.key||original.key_length!=current.key_length{return Err(invalid())}}Ok(())
 }
 fn add(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(META).ok_or_else(invalid)?;Ok(demand)}
 pub(crate) fn next_demands(&self,params:&Dictionary,key:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  self.checked(params,key)?;
  Ok(match self.phase{
   Phase::Bind=>Self::fixed(2*size_of::<Source>()+2*size_of::<FlowTextCopyWork>()+META),
   Phase::Text=>{let text=self.text.as_ref().ok_or_else(invalid)?;if text.ready(){Self::add(text.take_demands()?)?}else{text.next_demands(key,copy)?}},
   Phase::TextClose=>{let text=self.text.as_ref().ok_or_else(invalid)?;if text.terminal_is_empty(){Self::fixed(size_of::<Option<FlowTextCopyWork>>()+META)}else{text.close_demands(copy)?}},
   Phase::Entry=>Self::fixed(2*size_of::<OriginalOwnedEntry<Value>>()+2*size_of::<OrderedMap<Value>>()+2*size_of::<String>()+2*size_of::<Value>()+2*size_of::<Option<usize>>()+META),
   Phase::Advance=>Self::add(self.entry.as_ref().ok_or_else(invalid)?.next_demands(copy.saturating_sub(META))?)?,
   Phase::EntryClose=>{let entry=self.entry.as_ref().ok_or_else(invalid)?;if entry.terminal_is_empty(){Self::fixed(size_of::<Option<OriginalOwnedEntry<Value>>>()+META)}else{entry.next_close_demands(copy)?}},
   Phase::Publish=>Self::fixed(2*size_of::<OrderedMap<Value>>()+2*size_of::<Option<usize>>()+META),
   Phase::Ready=>Default::default(),Phase::Closed=>return Err(invalid()),
  })
 }
 fn paid(&mut self,d:RetirementDemand){self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes};self.work=1}
 fn absorb(&mut self,turn:RetainedWorkOutcome)->Result<(),ValueError>{self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress());turn.result.map(|_|())}
 pub(crate) fn advance(&mut self,params:&mut Dictionary,key:&str,g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_inner(params,key,g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_inner(&mut self,params:&mut Dictionary,key:&str,g:RetainedCloneGrant)->Result<bool,ValueError>{
  self.checked(params,key)?;if self.ready(){return Ok(true)}let demand=self.next_demands(params,key,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,demand){return Ok(false)}
  match self.phase{
   Phase::Bind=>{self.paid(demand);self.source=Some(Self::source(params,key));self.text=Some(FlowTextCopyWork::new());self.phase=Phase::Text},
   Phase::Text=>if self.text.as_ref().ok_or_else(invalid)?.ready(){
    let turn=self.text.as_mut().unwrap().take_original(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,..g});self.work=turn.work;
    match turn.result{Ok(Some((key,p)))=>{self.progress=p;self.progress.copied_bytes+=META;self.owners.key=Some(key);self.text.as_mut().unwrap().begin_close();self.phase=Phase::TextClose},Ok(None)=>{},Err(error)=>{self.progress=error.retained_progress();return Err(error)}}
   }else{let turn=self.text.as_mut().unwrap().advance_original(key,g);self.absorb(turn)?},
   Phase::TextClose=>if self.text.as_ref().ok_or_else(invalid)?.terminal_is_empty(){self.paid(demand);self.text=None;self.phase=Phase::Entry}else{let turn=self.text.as_mut().unwrap().close_step(g);self.absorb(turn)?},
   Phase::Entry=>{self.paid(demand);let key=self.owners.key.take().ok_or_else(invalid)?;let value=self.owners.value.take().ok_or_else(invalid)?;let map=std::mem::take(&mut params.pairs);self.source.as_mut().unwrap().root=None;self.entry=Some(OriginalOwnedEntry::new(map,key,value));self.phase=Phase::Advance},
   Phase::Advance=>{
    let turn=self.entry.as_mut().ok_or_else(invalid)?.advance(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,..g});self.work=turn.work;self.progress=self.entry.as_ref().unwrap().step_progress();
    match turn.result{Ok(Some(map))=>{self.owners.output=Some(map);self.entry.as_mut().unwrap().begin_close();self.phase=Phase::EntryClose;self.progress.copied_bytes+=META},Ok(None)=>{},Err(error)=>return Err(error)}
   },
   Phase::EntryClose=>if self.entry.as_ref().ok_or_else(invalid)?.terminal_is_empty(){self.paid(demand);self.entry=None;self.phase=Phase::Publish}else{let turn=self.entry.as_mut().unwrap().close_step(g);self.absorb(turn)?},
   Phase::Publish=>{self.paid(demand);params.pairs=self.owners.output.take().ok_or_else(invalid)?;self.source.as_mut().unwrap().root=params.original_root_identity();self.phase=Phase::Ready},
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(entry)=self.entry.as_mut(){entry.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(Self::fixed(size_of::<Option<FlowTextCopyWork>>()))}else{text.close_demands(copy)}}
  if let Some(entry)=self.entry.as_ref(){return if entry.terminal_is_empty(){Ok(Self::fixed(size_of::<Option<OriginalOwnedEntry<Value>>>()))}else{entry.next_close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(output,OrderedMap<Value>);owner!(key,String);owner!(value,Value);
  Ok(if self.source.is_some()||self.binding.is_some(){Self::fixed(size_of::<Option<Source>>()+size_of::<Option<OriginalSourceBinding>>()+META)}else{Default::default()})
 }
 pub(crate) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_inner(g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 fn close_inner(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}let demand=self.next_close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_mut(){if child.terminal_is_empty(){self.paid(Self::fixed(size_of::<Option<$ty>>()));self.$field=None}else{let turn=child.close_step(g);self.absorb(turn)?}return Ok(RetainedCloneStep::Progress(self.progress))}}}
  child!(text,FlowTextCopyWork);child!(entry,OriginalOwnedEntry<Value>);
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);self.absorb(turn)?;return Ok(RetainedCloneStep::Progress(self.progress))}
  macro_rules! owner{($field:ident)=>{if self.owners.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.owners.$field,g);self.absorb(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work})?;return Ok(RetainedCloneStep::Progress(self.progress))}}}
  owner!(output);owner!(key);owner!(value);
  if self.source.is_some()||self.binding.is_some(){self.paid(demand);self.source=None;self.binding=None;self.phase=Phase::Closed}Ok(RetainedCloneStep::Complete(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.source.is_none()&&self.binding.is_none()&&self.text.is_none()&&self.entry.is_none()&&self.retirement.is_empty()&&self.owners.output.is_none()&&self.owners.key.is_none()&&self.owners.value.is_none()}
}
impl<'a,'b> super::retained_scratch_driver::OriginalScratchWork<(&'a mut Dictionary,&'b str)> for NativeParamPatchWork{
 fn source_binding(&self)->OriginalSourceBinding{NativeParamPatchWork::source_binding(self)}
 fn origin(&self)->(u64,u64){let source=self.source_binding();(source.operation().0,source.generation().0)}
 fn ready(&self)->bool{NativeParamPatchWork::ready(self)}
 fn advancing_retirement(&self)->bool{NativeParamPatchWork::advancing_retirement(self)}
 fn next_demands(&self,source:&(&'a mut Dictionary,&'b str),copy:usize)->Result<RetirementDemand,ValueError>{NativeParamPatchWork::next_demands(self,&*source.0,source.1,copy)}
 fn advance(&mut self,source:&mut(&'a mut Dictionary,&'b str),g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{NativeParamPatchWork::advance(self,&mut*source.0,source.1,g)}
 fn step_progress(&self)->RetainedCloneProgress{NativeParamPatchWork::step_progress(self)}
}
impl Drop for NativeParamPatchWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"native parameter patch must close its actual value, map, key and child cursors");unsafe{ManuallyDrop::drop(&mut self.owners)}}}

#[cfg(test)]
#[path="🧪️.rs"]mod laws;
