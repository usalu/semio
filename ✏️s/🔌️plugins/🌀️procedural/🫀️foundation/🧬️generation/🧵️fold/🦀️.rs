//! 🧬️ Original generation edits compose retained row, text, equality and ordered-map producers.
use super::{GenerationPlayState,FormGeneration};
use super::retained_scratch_rows::{ScratchRowsWork,ScratchRowsKind};
use super::retained_scratch_scan::ScratchRowScan;
use super::retained_scratch_driver::{OriginalScratchWork,begun,drive,local_progress,pay,replied,reported,settled_step};
use super::retained_scratch_lookup::ScratchMapLookup;
use semio_os_flow::{OriginalTextOrdering,FlowTextCopyWork,OriginalOwnedEntry,OrderedMap};
use semio_framework_value::{DslValue,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding};
use std::{cmp::Ordering,mem::{ManuallyDrop,size_of}};
/// 🌱️ Actual owned fields move from the native procedural mutation into this shared domain edit.
#[derive(semio_framework_value::RetireOwned)]
pub enum GenerationFoldInput{
 Add{generation:FormGeneration,index:Option<usize>},Remove{id:String},Rename{id:String,name:String},
 SetValue{id:String,key:String,value:DslValue},Select{id:Option<String>},Preview{text:Option<String>},
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum GenerationFoldOutcome{Applied,Unchanged,Duplicate,Index,Missing}
impl GenerationFoldOutcome{
 pub fn code(self)->&'static str{match self{Self::Applied=>"applied",Self::Unchanged=>"unchanged",Self::Duplicate=>"duplicate",Self::Index=>"index",Self::Missing=>"missing"}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Find,Dispatch,Compare,Reset,Fallback,Copy,TakeCopy,StartRows,Rows,Selection,LookupBirth,Lookup,Equality,LookupCloseBirth,LookupClose,EntryBirth,Entry,Map,Commit,Ready}
struct Fields{input:Option<GenerationFoldInput>,order:Option<OriginalTextOrdering>,copy:Option<FlowTextCopyWork>,rows:Option<ScratchRowsWork<FormGeneration>>,lookup:Option<ScratchMapLookup<DslValue>>,entry:Option<OriginalOwnedEntry<DslValue>>,entry_close:Option<ControlledRetirement<OriginalOwnedEntry<DslValue>>>,map:Option<OrderedMap<DslValue>>,selection:Option<String>}
pub struct GenerationFoldWork{fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,source_binding:OriginalSourceBinding,origin:(u64,u64),binding:Option<usize>,scan:ScratchRowScan,target:Option<usize>,fallback:usize,replace_selection:bool,value_equal:bool,phase:Phase,after_reset:Phase,outcome:Option<GenerationFoldOutcome>,closing:bool,recover:bool,progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original generation fold changed its input, source or retained phase")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn generation_id(value:&FormGeneration)->&str{&value.id}
fn comparison_pair<'a>(phase:Phase,target:Option<usize>,fallback:usize,input:&'a GenerationFoldInput,state:&'a GenerationPlayState)->Result<Option<(&'a str,&'a str)>,ValueError>{
 if phase==Phase::Fallback{let GenerationFoldInput::Remove{id}=input else{return Err(invalid())};return Ok(state.generations.get(fallback).map(|generation|(generation.id.as_str(),id.as_str())))}
 Ok(match input{GenerationFoldInput::Rename{name,..}=>Some((state.generations.get(target.ok_or_else(invalid)?).ok_or_else(invalid)?.name.as_str(),name)),GenerationFoldInput::Remove{id}=>state.selected_generation_id.as_deref().map(|selected|(selected,id.as_str())),GenerationFoldInput::Select{id}=>state.selected_generation_id.as_deref().zip(id.as_deref()),GenerationFoldInput::Preview{text}=>state.preview_text.as_deref().zip(text.as_deref()),_=>return Err(invalid())})
}
fn copied_selection_source<'a>(input:&'a GenerationFoldInput,state:&'a GenerationPlayState,fallback:usize)->Result<&'a str,ValueError>{match input{GenerationFoldInput::Add{generation,..}=>Ok(&generation.id),GenerationFoldInput::Remove{..}=>Ok(&state.generations.get(fallback).ok_or_else(invalid)?.id),_=>Err(invalid())}}
fn receive_step(progress:&mut RetainedCloneProgress,effect:&mut RetainedAllocationEffect<'_>,turn:RetainedWorkOutcome)->Result<RetainedCloneStep,ValueError>{reported(effect,turn.work)?;*progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress());turn.result}
fn scalar_equal(left:&DslValue,right:&DslValue)->bool{match(left,right){(DslValue::Null,DslValue::Null)=>true,_=>match((left.as_str(),right.as_str()),(left.as_f64(),right.as_f64())){((Some(a),Some(b)),_)=>a.len()<=256&&a==b,(_,(Some(a),Some(b)))=>a==b,_=>false}}}
impl GenerationFoldWork{
 pub fn new(input:GenerationFoldInput,cx:&StepContext<'_>)->Self{Self::new_original(input,OriginalSourceBinding::borrow(cx))}
 pub fn new_original(input:GenerationFoldInput,source_binding:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{input:Some(input),order:None,copy:None,rows:None,lookup:None,entry:None,entry_close:None,map:None,selection:None}),retirement:Default::default(),source_binding,origin:(source_binding.operation().0,source_binding.generation().0),binding:None,scan:ScratchRowScan::new(),target:None,fallback:0,replace_selection:false,value_equal:false,phase:Phase::Bind,after_reset:Phase::Fallback,outcome:None,closing:false,recover:false,progress:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub fn outcome(&self)->Option<GenerationFoldOutcome>{self.outcome}
 fn finish(&mut self,outcome:GenerationFoldOutcome){self.outcome=Some(outcome);self.phase=Phase::Ready}
 fn input(&self)->Result<&GenerationFoldInput,ValueError>{self.fields.input.as_ref().ok_or_else(invalid)}
 fn id(&self)->Result<&str,ValueError>{match self.input()?{GenerationFoldInput::Add{generation,..}=>Ok(&generation.id),GenerationFoldInput::Remove{id}|GenerationFoldInput::Rename{id,..}|GenerationFoldInput::SetValue{id,..}=>Ok(id),GenerationFoldInput::Select{id:Some(id)}=>Ok(id),_=>Err(invalid())}}
 fn key(&self)->Result<&str,ValueError>{match self.input()?{GenerationFoldInput::SetValue{key,..}=>Ok(key),_=>Err(invalid())}}
 fn validate(&self,state:&GenerationPlayState)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding!=state as*const GenerationPlayState as usize){Err(invalid())}else{Ok(())}}
 fn pair<'a>(&'a self,state:&'a GenerationPlayState)->Result<Option<(&'a str,&'a str)>,ValueError>{comparison_pair(self.phase,self.target,self.fallback,self.input()?,state)}
 fn optional_equal(&self,state:&GenerationPlayState)->Result<bool,ValueError>{Ok(match self.input()?{GenerationFoldInput::Remove{..}=>false,GenerationFoldInput::Select{id}=>state.selected_generation_id.is_none()&&id.is_none(),GenerationFoldInput::Preview{text}=>state.preview_text.is_none()&&text.is_none(),_=>return Err(invalid())})}
 fn copy_source<'a>(&'a self,state:&'a GenerationPlayState)->Result<&'a str,ValueError>{copied_selection_source(self.input()?,state,self.fallback)}
 fn metadata(&self)->RetirementDemand{fixed(match self.phase{Phase::StartRows=>2*size_of::<GenerationFoldInput>(),Phase::Selection|Phase::Commit=>2*size_of::<Option<String>>(),Phase::LookupCloseBirth=>2*size_of::<usize>(),Phase::EntryBirth=>2*(size_of::<OrderedMap<DslValue>>()+size_of::<String>()+size_of::<DslValue>()),Phase::Map=>2*size_of::<OrderedMap<DslValue>>(),_=>4*size_of::<usize>()})}
 pub fn next_demands(&self,state:&GenerationPlayState,body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(state)?;if self.ready(){return Ok(Default::default())}
  match self.phase{
   Phase::Find=>self.scan.next_demands(&state.generations,self.id()?,generation_id),
   Phase::Compare|Phase::Fallback=>if let Some((left,right))=self.pair(state)?{self.fields.order.as_ref().ok_or_else(invalid)?.next_demands(left,right,body)}else{Ok(self.metadata())},
   Phase::Reset=>Ok(self.fields.order.as_ref().ok_or_else(invalid)?.reset_demand()),
   Phase::Copy=>self.fields.copy.as_ref().ok_or_else(invalid)?.next_demands(self.copy_source(state)?,body),
   Phase::TakeCopy=>self.fields.copy.as_ref().ok_or_else(invalid)?.take_demands(),
   Phase::Rows=>self.fields.rows.as_ref().ok_or_else(invalid)?.next_demands(&state.generations,body),
   Phase::Lookup=>self.fields.lookup.as_ref().ok_or_else(invalid)?.next_demands(&state.generations.get(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values,self.key()?,body),
   Phase::LookupClose=>self.fields.lookup.as_ref().ok_or_else(invalid)?.close_demands(body),
   Phase::Entry=>self.fields.entry.as_ref().ok_or_else(invalid)?.next_demands(body),
   _=>Ok(self.metadata()),
  }
 }
 pub fn advance(&mut self,state:&mut GenerationPlayState,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.progress=Default::default();let result=self.advance_original(state,effect).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:effect.performed_work()}}
 fn compare_done(&mut self,equal:bool)->Result<(),ValueError>{
  match self.input()?{
   GenerationFoldInput::Rename{..}|GenerationFoldInput::Select{..}|GenerationFoldInput::Preview{..}=>if equal{self.finish(GenerationFoldOutcome::Unchanged)}else{self.phase=Phase::Commit},
   GenerationFoldInput::Remove{..}=>if equal{self.replace_selection=true;self.fallback=0;self.after_reset=Phase::Fallback;self.phase=Phase::Reset}else{self.phase=Phase::StartRows},
   _=>return Err(invalid()),
  }Ok(())
 }
 fn advance_original(&mut self,state:&mut GenerationPlayState,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let g=effect.grant();let d=self.next_demands(state,g.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !retained_clone_grant_admits(g,d){return Ok(false)}
  if matches!(self.phase,Phase::Bind|Phase::Dispatch|Phase::StartRows|Phase::Selection|Phase::LookupBirth|Phase::Equality|Phase::LookupCloseBirth|Phase::EntryBirth|Phase::Map|Phase::Commit){if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(d)}
  match self.phase{
   Phase::Bind=>{self.binding=Some(state as*const GenerationPlayState as usize);self.phase=match self.input()?{GenerationFoldInput::Preview{..}|GenerationFoldInput::Select{id:None}=>{self.fields.order=Some(OriginalTextOrdering::new());Phase::Compare},_=>Phase::Find}},
   Phase::Find=>{let fields=&self.fields;let key=match fields.input.as_ref().ok_or_else(invalid)?{GenerationFoldInput::Add{generation,..}=>generation.id.as_str(),GenerationFoldInput::Remove{id}|GenerationFoldInput::Rename{id,..}|GenerationFoldInput::SetValue{id,..}|GenerationFoldInput::Select{id:Some(id)}=>id,_=>return Err(invalid())};let turn=self.scan.advance(&state.generations,key,generation_id,effect);self.progress=self.scan.step_progress();if turn.result?{self.target=self.scan.result();self.phase=Phase::Dispatch}},
   Phase::Dispatch=>{match self.input()?{
    GenerationFoldInput::Add{index,..}=>if self.target.is_some(){self.finish(GenerationFoldOutcome::Duplicate)}else if index.is_some_and(|index|index>state.generations.len()){self.finish(GenerationFoldOutcome::Index)}else{self.replace_selection=true;self.fields.copy=Some(FlowTextCopyWork::new());self.phase=Phase::Copy},
    _ if self.target.is_none()=>self.finish(GenerationFoldOutcome::Missing),
    GenerationFoldInput::SetValue{..}=>self.phase=Phase::LookupBirth,
    _=>{self.fields.order=Some(OriginalTextOrdering::new());self.phase=Phase::Compare},
   }},
   Phase::Compare|Phase::Fallback=>{
    let fields=&mut*self.fields;let pair=comparison_pair(self.phase,self.target,self.fallback,fields.input.as_ref().ok_or_else(invalid)?,state)?;
    if let Some((left,right))=pair{
     if effect.remaining_work_units()?<1{return Ok(false)}
     let turn=fields.order.as_mut().ok_or_else(invalid)?.advance(left,right,g);reported(effect,turn.work)?;self.progress=fields.order.as_ref().unwrap().step_progress();if let Some(order)=turn.result?{
      if self.phase==Phase::Fallback{if order==Ordering::Equal{self.fallback+=1;self.after_reset=Phase::Fallback;self.phase=Phase::Reset}else{self.fields.copy=Some(FlowTextCopyWork::new());self.phase=Phase::Copy}}else{self.compare_done(order==Ordering::Equal)?}
     }
    }else{if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(d);if self.phase==Phase::Fallback{self.phase=Phase::StartRows}else{self.compare_done(self.optional_equal(state)?)?}}
   },
   Phase::Reset=>{if effect.remaining_work_units()?<1{return Ok(false)}let order=self.fields.order.as_mut().ok_or_else(invalid)?;let turn=order.reset(g);receive_step(&mut self.progress,effect,turn)?;self.phase=self.after_reset},
   Phase::Copy=>{
    let fields=&mut*self.fields;let source=copied_selection_source(fields.input.as_ref().ok_or_else(invalid)?,state,self.fallback)?;let copy=fields.copy.as_mut().ok_or_else(invalid)?;let turn=copy.advance_original(source,effect);let ready=copy.ready();self.progress=turn.result?.progress();if ready{self.phase=Phase::TakeCopy}
   },
   Phase::TakeCopy=>{let turn=self.fields.copy.as_mut().ok_or_else(invalid)?.take_original(effect);self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|output|output.as_ref().map_or(Default::default(),|(_,p)|*p));if let Some((text,p))=turn.result?{self.fields.selection=Some(text);self.progress=p;self.phase=Phase::StartRows}},
   Phase::StartRows=>{let input=self.fields.input.take().ok_or_else(invalid)?;match input{
    GenerationFoldInput::Add{generation,index}=>{let at=index.unwrap_or(state.generations.len());match ScratchRowsWork::new(ScratchRowsKind::Insert,at,Some(generation)){Ok(rows)=>self.fields.rows=Some(rows),Err((error,generation))=>{self.fields.input=Some(GenerationFoldInput::Add{generation:generation.ok_or_else(invalid)?,index});return Err(error)}}},
    GenerationFoldInput::Remove{id}=>{self.fields.input=Some(GenerationFoldInput::Remove{id});self.fields.rows=Some(ScratchRowsWork::new(ScratchRowsKind::Remove,self.target.ok_or_else(invalid)?,None).map_err(|(error,_)|error)?)},
    input=>{self.fields.input=Some(input);return Err(invalid())},
   }self.phase=Phase::Rows},
   Phase::Rows=>{let rows=self.fields.rows.as_mut().ok_or_else(invalid)?;let turn=rows.advance(&mut state.generations,effect);self.progress=rows.step_progress();if turn.result?{if self.replace_selection{self.phase=Phase::Selection}else{self.finish(GenerationFoldOutcome::Applied)}}},
   Phase::Selection=>{std::mem::swap(&mut state.selected_generation_id,&mut self.fields.selection);self.finish(GenerationFoldOutcome::Applied)},
   Phase::LookupBirth=>{self.fields.lookup=Some(ScratchMapLookup::new());self.phase=Phase::Lookup},
   Phase::Lookup=>{
    let fields=&mut*self.fields;let GenerationFoldInput::SetValue{key,..}=fields.input.as_ref().ok_or_else(invalid)?else{return Err(invalid())};let values=&state.generations.get(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values;let lookup=fields.lookup.as_mut().ok_or_else(invalid)?;
    let complete=lookup.advance(values,key,effect)?;self.progress=lookup.step_progress();if complete{if lookup.result().is_some(){self.phase=Phase::Equality}else{self.value_equal=false;self.phase=Phase::LookupCloseBirth}}
   },
   Phase::Equality=>{let fields=&*self.fields;let GenerationFoldInput::SetValue{value,..}=fields.input.as_ref().ok_or_else(invalid)?else{return Err(invalid())};let left=fields.lookup.as_ref().and_then(ScratchMapLookup::result).ok_or_else(invalid)?;self.value_equal=scalar_equal(left,value);self.phase=Phase::LookupCloseBirth},
   Phase::LookupCloseBirth=>{self.fields.lookup.as_mut().ok_or_else(invalid)?.begin_close();self.phase=Phase::LookupClose},
   Phase::LookupClose=>{let lookup=self.fields.lookup.as_mut().ok_or_else(invalid)?;self.progress=settled_step(lookup.close_step(g.maximum_copy_bytes,effect))?.progress();if lookup.terminal_is_empty(){self.fields.lookup=None;if self.value_equal{self.finish(GenerationFoldOutcome::Unchanged)}else{self.phase=Phase::EntryBirth}}},
   Phase::EntryBirth=>{let GenerationFoldInput::SetValue{key,value,..}=self.fields.input.as_mut().ok_or_else(invalid)?else{return Err(invalid())};let values=&mut state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values;self.fields.entry=Some(OriginalOwnedEntry::new(std::mem::take(values),std::mem::take(key),std::mem::replace(value,DslValue::Null)));self.phase=Phase::Entry},
   Phase::Entry=>{
    let entry=self.fields.entry.as_mut().ok_or_else(invalid)?;let body=g.maximum_copy_bytes;let mut output=None;
    let reply=if self.recover{replied(effect,|effect,reply|entry.recover_advance_into(body,&mut output,effect,reply))}else{replied(effect,|effect,reply|entry.advance_into(body,&mut output,effect,reply))}?;
    match reply{Some(result)=>{self.recover=false;self.progress=result.map_err(OriginalRetirementFailure::into_value_error)?.progress();if let Some(map)=output{self.fields.map=Some(map);let entry=self.fields.entry.take().unwrap();self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("ordered entry declares controlled retirement")));self.phase=Phase::Map}},None=>self.recover=true}
   },
   Phase::Map=>{state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values=self.fields.map.take().ok_or_else(invalid)?;self.finish(GenerationFoldOutcome::Applied)},
   Phase::Commit=>{match self.fields.input.as_mut().ok_or_else(invalid)?{GenerationFoldInput::Rename{name,..}=>std::mem::swap(&mut state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.name,name),GenerationFoldInput::Select{id}=>std::mem::swap(&mut state.selected_generation_id,id),GenerationFoldInput::Preview{text}=>std::mem::swap(&mut state.preview_text,text),_=>return Err(invalid())}self.finish(GenerationFoldOutcome::Applied)},
   Phase::Ready=>return Ok(true),
  }Ok(self.ready())
 }
 pub fn step(&mut self,state:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.source_binding.matches(cx){return Err(invalid().into())}drive(self,state,cx)}
 pub fn advancing_retirement(&self)->bool{matches!(self.phase,Phase::LookupClose)||self.fields.rows.as_ref().is_some_and(ScratchRowsWork::advancing_retirement)||self.fields.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn begin_close(&mut self){self.closing=true;if let Some(order)=self.fields.order.as_mut(){order.begin_close()}if let Some(copy)=self.fields.copy.as_mut(){copy.begin_close()}if let Some(lookup)=self.fields.lookup.as_mut(){lookup.begin_close()}if let Some(rows)=self.fields.rows.as_mut(){rows.begin_close()}}
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(body)}
  if let Some(order)=self.fields.order.as_ref(){return if order.terminal_is_empty(){Ok(fixed(0))}else{Ok(order.close_demands())}}
  if let Some(copy)=self.fields.copy.as_ref(){return if copy.terminal_is_empty(){Ok(fixed(0))}else{copy.close_demands(body)}}
  if let Some(lookup)=self.fields.lookup.as_ref(){return if lookup.terminal_is_empty(){Ok(fixed(0))}else{lookup.close_demands(body)}}
  if self.fields.entry.is_some(){return Ok(fixed(0))}
  if let Some(close)=self.fields.entry_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(body)}}
  if let Some(rows)=self.fields.rows.as_ref(){return if rows.terminal_is_empty(){Ok(fixed(0))}else{rows.next_close_demands(body)}}
  if self.fields.map.is_some(){return self.retirement.next_owner_demands::<OrderedMap<DslValue>>(true,&None,body)}
  if self.fields.selection.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}
  if self.fields.input.is_some(){return self.retirement.next_owner_demands::<GenerationFoldInput>(true,&None,body)}Ok(if self.binding.is_some(){fixed(size_of::<usize>())}else{Default::default()})
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)}
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.retirement.is_empty(){self.progress=RetirementPollExit::resume(self.retirement.step_granted(body,effect))?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(order)=self.fields.order.as_mut(){if order.terminal_is_empty(){self.fields.order=None;self.progress.copied_items=1}else{let turn=order.close_step(effect.grant());reported(effect,turn.work)?;self.progress=turn.result?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(copy)=self.fields.copy.as_mut(){if copy.terminal_is_empty(){self.fields.copy=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(copy.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(lookup)=self.fields.lookup.as_mut(){if lookup.terminal_is_empty(){self.fields.lookup=None;self.progress.copied_items=1}else{lookup.begin_close();self.progress=RetirementPollExit::resume(lookup.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(entry)=self.fields.entry.take(){self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("ordered entry declares controlled retirement")));self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(close)=self.fields.entry_close.as_mut(){if close.terminal_is_empty(){self.fields.entry_close=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(close.step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(rows)=self.fields.rows.as_mut(){if rows.terminal_is_empty(){self.fields.rows=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(rows.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if self.fields.map.is_some(){self.progress=begun(&mut self.retirement,&mut self.fields.map,effect)?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if self.fields.selection.is_some(){self.progress=begun(&mut self.retirement,&mut self.fields.selection,effect)?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if self.fields.input.is_some(){self.progress=begun(&mut self.retirement,&mut self.fields.input,effect)?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  self.binding=None;self.progress.copied_items=1;Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.retirement.is_empty()&&self.fields.input.is_none()&&self.fields.order.is_none()&&self.fields.copy.is_none()&&self.fields.rows.is_none()&&self.fields.lookup.is_none()&&self.fields.entry.is_none()&&self.fields.entry_close.is_none()&&self.fields.map.is_none()&&self.fields.selection.is_none()}
}
impl OriginalScratchWork<GenerationPlayState> for GenerationFoldWork{
 fn source_binding(&self)->OriginalSourceBinding{self.source_binding}
 fn origin(&self)->(u64,u64){self.origin}
 fn ready(&self)->bool{self.ready()}
 fn advancing_retirement(&self)->bool{self.advancing_retirement()}
 fn next_demands(&self,state:&GenerationPlayState,body:usize,_work:u64)->Result<RetirementDemand,ValueError>{self.next_demands(state,body)}
 fn advance(&mut self,state:&mut GenerationPlayState,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.advance(state,effect)}
 fn step_progress(&self)->RetainedCloneProgress{self.step_progress()}
}
impl Drop for GenerationFoldWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original generation fold retains input, row, comparison, lookup or map custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
