//! 🧬️ Original generation edits compose retained row, text, equality and ordered-map producers.
use super::{GenerationPlayState,FormGeneration};
use super::retained_scratch_rows::{ScratchRowsWork,ScratchRowsKind};
use super::retained_scratch_scan::ScratchRowScan;
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use semio_os_flow::{OriginalTextOrdering,FlowTextCopyWork,OriginalOwnedEntry,LookupCursor,OrderedMap};
use semio_framework_value::{DslValue,OriginalValueEquality,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
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
enum Phase{Bind,Find,Dispatch,Compare,Reset,Fallback,Copy,TakeCopy,StartRows,Rows,Selection,LookupBirth,Lookup,Equality,EqualityClose,LookupCloseBirth,LookupClose,EntryBirth,Entry,Map,Commit,Ready}
struct Fields{input:Option<GenerationFoldInput>,order:Option<OriginalTextOrdering>,copy:Option<FlowTextCopyWork>,rows:Option<ScratchRowsWork<FormGeneration>>,lookup:Option<LookupCursor<DslValue>>,lookup_close:Option<ControlledRetirement<LookupCursor<DslValue>>>,equal:Option<OriginalValueEquality>,entry:Option<OriginalOwnedEntry<DslValue>>,map:Option<OrderedMap<DslValue>>,selection:Option<String>}
pub struct GenerationFoldWork{fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,source_binding:OriginalSourceBinding,origin:(u64,u64),binding:Option<usize>,scan:ScratchRowScan,target:Option<usize>,fallback:usize,replace_selection:bool,value_equal:bool,phase:Phase,after_reset:Phase,outcome:Option<GenerationFoldOutcome>,closing:bool,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original generation fold changed its input, source or retained phase")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn generation_id(value:&FormGeneration)->&str{&value.id}
fn comparison_pair<'a>(phase:Phase,target:Option<usize>,fallback:usize,input:&'a GenerationFoldInput,state:&'a GenerationPlayState)->Result<Option<(&'a str,&'a str)>,ValueError>{
 if phase==Phase::Fallback{let GenerationFoldInput::Remove{id}=input else{return Err(invalid())};return Ok(state.generations.get(fallback).map(|generation|(generation.id.as_str(),id.as_str())))}
 Ok(match input{GenerationFoldInput::Rename{name,..}=>Some((state.generations.get(target.ok_or_else(invalid)?).ok_or_else(invalid)?.name.as_str(),name)),GenerationFoldInput::Remove{id}=>state.selected_generation_id.as_deref().map(|selected|(selected,id.as_str())),GenerationFoldInput::Select{id}=>state.selected_generation_id.as_deref().zip(id.as_deref()),GenerationFoldInput::Preview{text}=>state.preview_text.as_deref().zip(text.as_deref()),_=>return Err(invalid())})
}
fn copied_selection_source<'a>(input:&'a GenerationFoldInput,state:&'a GenerationPlayState,fallback:usize)->Result<&'a str,ValueError>{match input{GenerationFoldInput::Add{generation,..}=>Ok(&generation.id),GenerationFoldInput::Remove{..}=>Ok(&state.generations.get(fallback).ok_or_else(invalid)?.id),_=>Err(invalid())}}
fn receive_step(progress:&mut RetainedCloneProgress,work:&mut u64,turn:RetainedWorkOutcome)->Result<RetainedCloneStep,ValueError>{*work=turn.work;*progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress());turn.result}
impl GenerationFoldWork{
 pub fn new(input:GenerationFoldInput,cx:&StepContext<'_>)->Self{Self::new_original(input,OriginalSourceBinding::borrow(cx))}
 pub fn new_original(input:GenerationFoldInput,source_binding:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{input:Some(input),order:None,copy:None,rows:None,lookup:None,lookup_close:None,equal:None,entry:None,map:None,selection:None}),retirement:Default::default(),source_binding,origin:(source_binding.operation().0,source_binding.generation().0),binding:None,scan:ScratchRowScan::new(),target:None,fallback:0,replace_selection:false,value_equal:false,phase:Phase::Bind,after_reset:Phase::Fallback,outcome:None,closing:false,progress:Default::default(),work:0}}
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
 fn metadata(&self)->RetirementDemand{fixed(match self.phase{Phase::StartRows=>2*size_of::<GenerationFoldInput>(),Phase::Selection|Phase::Commit=>2*size_of::<Option<String>>(),Phase::LookupCloseBirth=>2*size_of::<LookupCursor<DslValue>>(),Phase::EntryBirth=>2*(size_of::<OrderedMap<DslValue>>()+size_of::<String>()+size_of::<DslValue>()),Phase::Map=>2*size_of::<OrderedMap<DslValue>>(),_=>4*size_of::<usize>()})}
 pub fn next_demands(&self,state:&GenerationPlayState,body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(state)?;if self.ready(){return Ok(Default::default())}
  match self.phase{
   Phase::Find=>self.scan.next_demands(&state.generations,self.id()?,generation_id),
   Phase::Compare|Phase::Fallback=>if let Some((left,right))=self.pair(state)?{self.fields.order.as_ref().ok_or_else(invalid)?.next_demands(left,right,body)}else{Ok(self.metadata())},
   Phase::Reset=>Ok(self.fields.order.as_ref().ok_or_else(invalid)?.reset_demand()),
   Phase::Copy=>self.fields.copy.as_ref().ok_or_else(invalid)?.next_demands(self.copy_source(state)?,body),
   Phase::TakeCopy=>self.fields.copy.as_ref().ok_or_else(invalid)?.take_demands(),
   Phase::Rows=>self.fields.rows.as_ref().ok_or_else(invalid)?.next_demands(&state.generations,body),
   Phase::LookupBirth=>Ok(state.generations.get(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values.lookup_borrowed_constructor_demands()),
   Phase::Lookup=>self.fields.lookup.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(self.key()?),
   Phase::Equality=>{let left=self.fields.lookup.as_ref().and_then(LookupCursor::result).ok_or_else(invalid)?;let GenerationFoldInput::SetValue{value,..}=self.input()?else{return Err(invalid())};self.fields.equal.as_ref().ok_or_else(invalid)?.next_demands(left,value,body)},
   Phase::EqualityClose=>self.fields.equal.as_ref().ok_or_else(invalid)?.next_close_demands(body),
   Phase::LookupClose=>{let close=self.fields.lookup_close.as_ref().ok_or_else(invalid)?;if close.terminal_is_empty(){Ok(self.metadata())}else{close.selected_demand(body)}},
   Phase::Entry=>self.fields.entry.as_ref().ok_or_else(invalid)?.next_demands(body),
   _=>Ok(self.metadata()),
  }
 }
 pub fn advance(&mut self,state:&mut GenerationPlayState,g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.progress=Default::default();self.work=0;let result=self.advance_original(state,g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}}
 fn compare_done(&mut self,equal:bool)->Result<(),ValueError>{
  match self.input()?{
   GenerationFoldInput::Rename{..}|GenerationFoldInput::Select{..}|GenerationFoldInput::Preview{..}=>if equal{self.finish(GenerationFoldOutcome::Unchanged)}else{self.phase=Phase::Commit},
   GenerationFoldInput::Remove{..}=>if equal{self.replace_selection=true;self.fallback=0;self.after_reset=Phase::Fallback;self.phase=Phase::Reset}else{self.phase=Phase::StartRows},
   _=>return Err(invalid()),
  }Ok(())
 }
 fn advance_original(&mut self,state:&mut GenerationPlayState,g:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(state,g.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !retained_clone_grant_admits(g,d){return Ok(false)}self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   Phase::Bind=>{self.binding=Some(state as*const GenerationPlayState as usize);self.phase=match self.input()?{GenerationFoldInput::Preview{..}|GenerationFoldInput::Select{id:None}=>{self.fields.order=Some(OriginalTextOrdering::new());Phase::Compare},_=>Phase::Find}},
   Phase::Find=>{let fields=&self.fields;let key=match fields.input.as_ref().ok_or_else(invalid)?{GenerationFoldInput::Add{generation,..}=>generation.id.as_str(),GenerationFoldInput::Remove{id}|GenerationFoldInput::Rename{id,..}|GenerationFoldInput::SetValue{id,..}|GenerationFoldInput::Select{id:Some(id)}=>id,_=>return Err(invalid())};let turn=self.scan.advance(&state.generations,key,generation_id,g);self.work=turn.work;self.progress=self.scan.step_progress();if turn.result?{self.target=self.scan.result();self.phase=Phase::Dispatch}},
   Phase::Dispatch=>{match self.input()?{
    GenerationFoldInput::Add{index,..}=>if self.target.is_some(){self.finish(GenerationFoldOutcome::Duplicate)}else if index.is_some_and(|index|index>state.generations.len()){self.finish(GenerationFoldOutcome::Index)}else{self.replace_selection=true;self.fields.copy=Some(FlowTextCopyWork::new());self.phase=Phase::Copy},
    _ if self.target.is_none()=>self.finish(GenerationFoldOutcome::Missing),
    GenerationFoldInput::SetValue{..}=>self.phase=Phase::LookupBirth,
    _=>{self.fields.order=Some(OriginalTextOrdering::new());self.phase=Phase::Compare},
   }},
   Phase::Compare|Phase::Fallback=>{
    let fields=&mut*self.fields;let pair=comparison_pair(self.phase,self.target,self.fallback,fields.input.as_ref().ok_or_else(invalid)?,state)?;
    if let Some((left,right))=pair{
     let turn=fields.order.as_mut().ok_or_else(invalid)?.advance(left,right,g);self.work=turn.work;self.progress=fields.order.as_ref().unwrap().step_progress();if let Some(order)=turn.result?{
      if self.phase==Phase::Fallback{if order==Ordering::Equal{self.fallback+=1;self.after_reset=Phase::Fallback;self.phase=Phase::Reset}else{self.fields.copy=Some(FlowTextCopyWork::new());self.phase=Phase::Copy}}else{self.compare_done(order==Ordering::Equal)?}
     }
    }else if self.phase==Phase::Fallback{self.phase=Phase::StartRows}else{self.compare_done(self.optional_equal(state)?)?}
   },
   Phase::Reset=>{let order=self.fields.order.as_mut().ok_or_else(invalid)?;let turn=order.reset(g);receive_step(&mut self.progress,&mut self.work,turn)?;self.phase=self.after_reset},
   Phase::Copy=>{
    let fields=&mut*self.fields;let source=copied_selection_source(fields.input.as_ref().ok_or_else(invalid)?,state,self.fallback)?;let copy=fields.copy.as_mut().ok_or_else(invalid)?;let turn=copy.advance_original(source,g);let ready=copy.ready();receive_step(&mut self.progress,&mut self.work,turn)?;if ready{self.phase=Phase::TakeCopy}
   },
   Phase::TakeCopy=>{let turn=self.fields.copy.as_mut().ok_or_else(invalid)?.take_original(g);self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|output|output.as_ref().map_or(Default::default(),|(_,p)|*p));if let Some((text,p))=turn.result?{self.fields.selection=Some(text);self.progress=p;self.phase=Phase::StartRows}},
   Phase::StartRows=>{let input=self.fields.input.take().ok_or_else(invalid)?;match input{
    GenerationFoldInput::Add{generation,index}=>{let at=index.unwrap_or(state.generations.len());match ScratchRowsWork::new(ScratchRowsKind::Insert,at,Some(generation)){Ok(rows)=>self.fields.rows=Some(rows),Err((error,generation))=>{self.fields.input=Some(GenerationFoldInput::Add{generation:generation.ok_or_else(invalid)?,index});return Err(error)}}},
    GenerationFoldInput::Remove{id}=>{self.fields.input=Some(GenerationFoldInput::Remove{id});self.fields.rows=Some(ScratchRowsWork::new(ScratchRowsKind::Remove,self.target.ok_or_else(invalid)?,None).map_err(|(error,_)|error)?)},
    input=>{self.fields.input=Some(input);return Err(invalid())},
   }self.phase=Phase::Rows},
   Phase::Rows=>{let rows=self.fields.rows.as_mut().ok_or_else(invalid)?;let turn=rows.advance(&mut state.generations,g);self.work=turn.work;self.progress=rows.step_progress();if turn.result?{if self.replace_selection{self.phase=Phase::Selection}else{self.finish(GenerationFoldOutcome::Applied)}}},
   Phase::Selection=>{std::mem::swap(&mut state.selected_generation_id,&mut self.fields.selection);self.finish(GenerationFoldOutcome::Applied)},
   Phase::LookupBirth=>{let GenerationFoldInput::SetValue{key,..}=self.fields.input.as_ref().ok_or_else(invalid)?else{return Err(invalid())};let values=&state.generations.get(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values;let turn=values.admit_lookup_borrowed_original(key,g);self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|(_,p)|*p);let(lookup,p)=turn.result?;self.fields.lookup=Some(lookup);self.progress=p;self.phase=Phase::Lookup},
   Phase::Lookup=>{let fields=&mut*self.fields;let GenerationFoldInput::SetValue{key,..}=fields.input.as_ref().ok_or_else(invalid)?else{return Err(invalid())};let lookup=fields.lookup.as_mut().ok_or_else(invalid)?;let turn=lookup.advance_borrowed_original(key,g);receive_step(&mut self.progress,&mut self.work,turn)?;if lookup.is_complete(){if lookup.result().is_some(){fields.equal=Some(OriginalValueEquality::new());self.phase=Phase::Equality}else{self.value_equal=false;self.phase=Phase::LookupCloseBirth}}},
   Phase::Equality=>{let fields=&mut*self.fields;let GenerationFoldInput::SetValue{value,..}=fields.input.as_ref().ok_or_else(invalid)?else{return Err(invalid())};let left=fields.lookup.as_ref().and_then(LookupCursor::result).ok_or_else(invalid)?;let equal=fields.equal.as_mut().ok_or_else(invalid)?;let turn=equal.advance(left,value,g);self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress);let step=turn.result?;self.progress=step.progress;if let Some(equal)=step.equal{self.value_equal=equal;fields.equal.as_mut().unwrap().begin_close();self.phase=Phase::EqualityClose}},
   Phase::EqualityClose=>{let equal=self.fields.equal.as_mut().ok_or_else(invalid)?;let turn=equal.close_step(g);receive_step(&mut self.progress,&mut self.work,turn)?;if equal.terminal_is_empty(){self.fields.equal=None;self.phase=Phase::LookupCloseBirth}},
   Phase::LookupCloseBirth=>{self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().ok_or_else(invalid)?).unwrap_or_else(|_|unreachable!("ordered lookup supports controlled retirement")));self.phase=Phase::LookupClose},
   Phase::LookupClose=>{let close=self.fields.lookup_close.as_mut().ok_or_else(invalid)?;if close.terminal_is_empty(){self.fields.lookup_close=None;if self.value_equal{self.finish(GenerationFoldOutcome::Unchanged)}else{self.phase=Phase::EntryBirth}}else{let turn=close.step(g);self.work=close.last_turn_work();self.progress=close.step_progress();turn?;}},
   Phase::EntryBirth=>{let GenerationFoldInput::SetValue{key,value,..}=self.fields.input.as_mut().ok_or_else(invalid)?else{return Err(invalid())};let values=&mut state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values;self.fields.entry=Some(OriginalOwnedEntry::new(std::mem::take(values),std::mem::take(key),std::mem::replace(value,DslValue::Null)));self.phase=Phase::Entry},
   Phase::Entry=>{let entry=self.fields.entry.as_mut().ok_or_else(invalid)?;let turn=entry.advance(g);self.work=turn.work;self.progress=entry.step_progress();if let Some(map)=turn.result?{self.fields.map=Some(map);self.phase=Phase::Map}},
   Phase::Map=>{state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.values=self.fields.map.take().ok_or_else(invalid)?;self.finish(GenerationFoldOutcome::Applied)},
   Phase::Commit=>{match self.fields.input.as_mut().ok_or_else(invalid)?{GenerationFoldInput::Rename{name,..}=>std::mem::swap(&mut state.generations.get_mut(self.target.ok_or_else(invalid)?).ok_or_else(invalid)?.name,name),GenerationFoldInput::Select{id}=>std::mem::swap(&mut state.selected_generation_id,id),GenerationFoldInput::Preview{text}=>std::mem::swap(&mut state.preview_text,text),_=>return Err(invalid())}self.finish(GenerationFoldOutcome::Applied)},
   Phase::Ready=>return Ok(true),
  }Ok(self.ready())
 }
 pub fn step(&mut self,state:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.source_binding.matches(cx){return Err(invalid().into())}drive(self,state,cx)}
 pub fn advancing_retirement(&self)->bool{matches!(self.phase,Phase::EqualityClose|Phase::LookupClose)||self.fields.rows.as_ref().is_some_and(ScratchRowsWork::advancing_retirement)||self.fields.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn begin_close(&mut self){self.closing=true;if let Some(order)=self.fields.order.as_mut(){order.begin_close()}if let Some(equal)=self.fields.equal.as_mut(){equal.begin_close()}if let Some(copy)=self.fields.copy.as_mut(){copy.begin_close()}if let Some(entry)=self.fields.entry.as_mut(){entry.begin_close()}if let Some(rows)=self.fields.rows.as_mut(){rows.begin_close()}}
 pub fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(body)}
  if let Some(order)=self.fields.order.as_ref(){return if order.terminal_is_empty(){Ok(fixed(0))}else{Ok(order.close_demands())}}
  if let Some(equal)=self.fields.equal.as_ref(){return if equal.terminal_is_empty(){Ok(fixed(0))}else{equal.next_close_demands(body)}}
  if let Some(copy)=self.fields.copy.as_ref(){return if copy.terminal_is_empty(){Ok(fixed(0))}else{copy.close_demands(body)}}
  if let Some(close)=self.fields.lookup_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(body)}}
  if self.fields.lookup.is_some(){return Ok(fixed(2*size_of::<LookupCursor<DslValue>>()))}
  if let Some(entry)=self.fields.entry.as_ref(){return if entry.terminal_is_empty(){Ok(fixed(0))}else{entry.next_close_demands(body)}}
  if let Some(rows)=self.fields.rows.as_ref(){return if rows.terminal_is_empty(){Ok(fixed(0))}else{rows.next_close_demands(body)}}
  if self.fields.map.is_some(){return self.retirement.next_owner_demands::<OrderedMap<DslValue>>(true,&None,body)}
  if self.fields.selection.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}
  if self.fields.input.is_some(){return self.retirement.next_owner_demands::<GenerationFoldInput>(true,&None,body)}Ok(if self.binding.is_some(){fixed(size_of::<usize>())}else{Default::default()})
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();self.work=0;let result=self.close_original(g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(g,self.next_close_demands(g.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);receive_step(&mut self.progress,&mut self.work,turn)?;return Ok(RetainedCloneStep::Progress(self.progress))}
  macro_rules! close{($field:ident,$empty:ident,$step:ident)=>{if let Some(owner)=self.fields.$field.as_mut(){if owner.$empty(){self.fields.$field=None;self.work=1;self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}else{let turn=owner.$step(g);receive_step(&mut self.progress,&mut self.work,turn)?;return Ok(RetainedCloneStep::Progress(self.progress))}}};}
  close!(order,terminal_is_empty,close_step);
  close!(equal,terminal_is_empty,close_step);close!(copy,terminal_is_empty,close_step);
  if let Some(close)=self.fields.lookup_close.as_mut(){if close.terminal_is_empty(){self.fields.lookup_close=None}else{let turn=close.step(g);self.work=close.last_turn_work();self.progress=close.step_progress();turn?;return Ok(RetainedCloneStep::Progress(self.progress))}}
  else if self.fields.lookup.is_some(){self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().unwrap()).unwrap_or_else(|_|unreachable!("ordered lookup supports controlled retirement")))}
  else if self.fields.entry.is_some(){close!(entry,terminal_is_empty,close_step)}
  else if self.fields.rows.is_some(){close!(rows,terminal_is_empty,close_step)}
  else if self.fields.map.is_some()||self.fields.selection.is_some()||self.fields.input.is_some(){let turn=if self.fields.map.is_some(){self.retirement.begin_granted(&mut self.fields.map,g)}else if self.fields.selection.is_some(){self.retirement.begin_granted(&mut self.fields.selection,g)}else{self.retirement.begin_granted(&mut self.fields.input,g)};self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.as_ref().map_or(Default::default(),|step|step.progress()));turn.result?;return Ok(RetainedCloneStep::Progress(self.progress))}
  else{self.binding=None}
  self.work=1;self.progress.copied_items=1;Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.retirement.is_empty()&&self.fields.input.is_none()&&self.fields.order.is_none()&&self.fields.copy.is_none()&&self.fields.rows.is_none()&&self.fields.lookup.is_none()&&self.fields.lookup_close.is_none()&&self.fields.equal.is_none()&&self.fields.entry.is_none()&&self.fields.map.is_none()&&self.fields.selection.is_none()}
}
impl OriginalScratchWork<GenerationPlayState> for GenerationFoldWork{
 fn source_binding(&self)->OriginalSourceBinding{self.source_binding}
 fn origin(&self)->(u64,u64){self.origin}
 fn ready(&self)->bool{self.ready()}
 fn advancing_retirement(&self)->bool{self.advancing_retirement()}
 fn next_demands(&self,state:&GenerationPlayState,body:usize)->Result<RetirementDemand,ValueError>{self.next_demands(state,body)}
 fn advance(&mut self,state:&mut GenerationPlayState,g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.advance(state,g)}
 fn step_progress(&self)->RetainedCloneProgress{self.step_progress()}
}
impl Drop for GenerationFoldWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original generation fold retains input, row, comparison, lookup or map custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
