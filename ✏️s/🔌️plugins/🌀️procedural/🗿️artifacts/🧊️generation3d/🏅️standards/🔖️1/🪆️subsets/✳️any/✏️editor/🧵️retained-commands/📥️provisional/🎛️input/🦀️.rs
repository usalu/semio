//! 🎛️ Provisional typed inputs retain original payloads while native fields land on their actual scratch widget.
use super::retained_typed_input_admission::TypedInputAdmission;
use super::retained_typed_current::{ProvisionalInputComparison,ProvisionalInputMatch};
use super::retained_typed_literal::TypedLiteralWork;
use super::retained_native_param_patch::NativeParamPatchWork;
use super::retained_scratch_scan::ScratchRowScan;
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use semio_s_artifact_procedural_3d_mutations::{ChangeWidgetInput,WidgetInputValue};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowSnapshotSource,FlowSourceIdentity,Widget,widget_id_for,neural::Value};
use semio_os_flow::LookupCursor;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Admit,Scan,Wired,Kind,NoteChannel,NoteCompare,NoteSwap,LookupBirth,LookupRun,CompareBirth,Compare,CompareClose,LookupClose,LiteralBirth,Literal,LiteralTake,LiteralClose,PatchBirth,Patch}
struct Fields{payload:Option<ChangeWidgetInput>,lookup:Option<LookupCursor<Value>>,lookup_close:Option<ControlledRetirement<LookupCursor<Value>>>,comparison:Option<ProvisionalInputComparison>,literal:Option<TypedLiteralWork>,value:Option<Value>,patch:Option<NativeParamPatchWork>}
pub struct ProvisionalTypedInputWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,binding:Option<FlowSourceIdentity>,admission:TypedInputAdmission,scan:ScratchRowScan,target:Option<usize>,wire:usize,field:usize,offset:usize,phase:Phase,ready:bool,applied:bool,closing:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional typed input changed its original scratch source or owner phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn pair_window(a:&str,b:&str,offset:usize)->usize{if a.len()==b.len(){a.len().saturating_sub(offset).min(64)}else{0}}
fn equal_window(a:&str,b:&str,offset:usize,bytes:usize)->bool{a.as_bytes()[offset..offset+bytes].iter().zip(&b.as_bytes()[offset..offset+bytes]).fold(true,|equal,(a,b)|equal&(*a==*b))}
impl ProvisionalTypedInputWork{
 pub(crate) fn new(payload:ChangeWidgetInput,cx:&StepContext<'_>)->Self{Self::new_original(payload,OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(payload:ChangeWidgetInput,original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{payload:Some(payload),lookup:None,lookup_close:None,comparison:None,literal:None,value:None,patch:None}),original,binding:None,admission:TypedInputAdmission::new(),scan:ScratchRowScan::new(),target:None,wire:0,field:0,offset:0,phase:Phase::Bind,ready:false,applied:false,closing:false,retirement:Default::default(),progress:Default::default(),work:0}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.ready}
 pub(crate) fn applied(&self)->bool{self.ready&&self.applied}
 fn finish(&mut self,applied:bool){self.ready=true;self.applied=applied}
 fn validate(&self,source:&FlowHostSnapshot)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding!=source.source_identity()){Err(invalid())}else{Ok(())}}
 pub(crate) fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.ready{return Ok(Default::default())}
  if let Some(close)=self.fields.lookup_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;
  match self.phase{
   Phase::Admit=>self.admission.next_demands(payload,copy),
   Phase::Scan=>self.scan.next_demands(&source.widgets,&payload.id,widget_id_for),
   Phase::Wired=>Ok(fixed(source.synapses.get(self.wire).map_or(0,|wire|{let(a,b)=if self.field==0{(wire.to.as_str(),payload.id.as_str())}else{(wire.to_port.as_str(),payload.channel.as_str())};2*pair_window(a,b,self.offset)}))),
   Phase::NoteChannel=>Ok(fixed(if payload.channel.len()==4{8}else{0})),
   Phase::NoteCompare=>match(source.widgets.get(self.target.ok_or_else(invalid)?),&payload.input){(Some(Widget::InputNote{text,..}),WidgetInputValue::Text(value))=>Ok(fixed(2*pair_window(text,value,self.offset))),_=>Err(invalid())},
   Phase::NoteSwap=>Ok(fixed(2*size_of::<String>())),
   Phase::LookupBirth=>match source.widgets.get(self.target.ok_or_else(invalid)?){Some(Widget::Neuron{params,..})=>Ok(params.pairs().lookup_borrowed_constructor_demands()),_=>Err(invalid())},
   Phase::LookupRun=>self.fields.lookup.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(&payload.channel),
   Phase::Compare=>self.fields.comparison.as_ref().ok_or_else(invalid)?.next_demands(self.fields.lookup.as_ref().and_then(LookupCursor::result).ok_or_else(invalid)?,&payload.input,copy),
   Phase::CompareClose=>match self.fields.comparison.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Literal=>self.fields.literal.as_ref().ok_or_else(invalid)?.next_demands(&payload.input,copy),
   Phase::LiteralTake=>self.fields.literal.as_ref().ok_or_else(invalid)?.take_demands(),
   Phase::LiteralClose=>match self.fields.literal.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Patch=>match source.widgets.get(self.target.ok_or_else(invalid)?){Some(Widget::Neuron{params,..})=>self.fields.patch.as_ref().ok_or_else(invalid)?.next_demands(params,&payload.channel,copy),_=>Err(invalid())},
   _=>Ok(fixed(0)),
  }
 }
 pub(crate) fn advance(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(source,grant);if self.work>0||self.progress!=Default::default(){self.binding=Some(source.source_identity())}let result=result.map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,d){return Ok(false)}
  if let Some(close)=self.fields.lookup_close.as_mut(){if close.terminal_is_empty(){self.fields.lookup_close=None;self.phase=Phase::LiteralBirth;self.work=1;self.progress.copied_items=1}else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?;}return Ok(false)}
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   Phase::Bind=>{self.phase=Phase::Admit},
   Phase::Admit=>{let turn=self.admission.advance(self.fields.payload.as_ref().ok_or_else(invalid)?,grant);self.work=turn.work;self.progress=self.admission.step_progress();if turn.result?{if self.admission.result()==Some(true){self.phase=Phase::Scan}else{self.finish(false)}}},
   Phase::Scan=>{let fields=&*self.fields;let turn=self.scan.advance(&source.widgets,&fields.payload.as_ref().ok_or_else(invalid)?.id,widget_id_for,grant);self.work=turn.work;self.progress=self.scan.step_progress();if turn.result?{self.target=self.scan.result();if self.target.is_none(){self.finish(false)}else{self.phase=Phase::Wired}}},
   Phase::Wired=>{
    let Some(wire)=source.synapses.get(self.wire)else{self.phase=Phase::Kind;return Ok(false)};
    let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;let(a,b)=if self.field==0{(wire.to.as_str(),payload.id.as_str())}else{(wire.to_port.as_str(),payload.channel.as_str())};
    if a.len()!=b.len()||!equal_window(a,b,self.offset,d.copy_bytes/2){self.wire+=1;self.field=0;self.offset=0}
    else{self.offset+=d.copy_bytes/2;if self.offset==a.len(){self.offset=0;self.field+=1;if self.field==2{self.finish(false)}}}
   },
   Phase::Kind=>{match source.widgets.get(self.target.ok_or_else(invalid)?){Some(Widget::InputNote{..})=>self.phase=Phase::NoteChannel,Some(Widget::Neuron{..})=>self.phase=Phase::LookupBirth,_=>self.finish(false)}},
   Phase::NoteChannel=>{let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;if payload.channel.len()!=4||!equal_window(&payload.channel,"text",0,4)||!matches!(&payload.input,WidgetInputValue::Text(_)){self.finish(false)}else{self.phase=Phase::NoteCompare;self.offset=0}},
   Phase::NoteCompare=>{
    let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;let(Some(Widget::InputNote{text,..}),WidgetInputValue::Text(value))=(source.widgets.get(self.target.ok_or_else(invalid)?),&payload.input)else{return Err(invalid())};
    if text.len()!=value.len()||!equal_window(text,value,self.offset,d.copy_bytes/2){self.phase=Phase::NoteSwap}
    else{self.offset+=d.copy_bytes/2;if self.offset==text.len(){self.finish(false)}}
   },
   Phase::NoteSwap=>{
    let index=self.target.ok_or_else(invalid)?;let payload=self.fields.payload.as_mut().ok_or_else(invalid)?;let(Some(Widget::InputNote{text,..}),WidgetInputValue::Text(value))=(source.widgets.get_mut(index),&mut payload.input)else{return Err(invalid())};std::mem::swap(text,value);self.finish(true)
   },
   Phase::LookupBirth=>{
    let Some(Widget::Neuron{params,..})=source.widgets.get(self.target.ok_or_else(invalid)?)else{return Err(invalid())};let turn=params.pairs().admit_lookup_borrowed_original(&self.fields.payload.as_ref().ok_or_else(invalid)?.channel,grant);self.work=turn.work;self.progress=Default::default();let(lookup,p)=turn.result?;self.fields.lookup=Some(lookup);self.progress=p;self.phase=Phase::LookupRun
   },
   Phase::LookupRun=>{
    let fields=&mut *self.fields;let query=&fields.payload.as_ref().ok_or_else(invalid)?.channel;let lookup=fields.lookup.as_mut().ok_or_else(invalid)?;let turn=lookup.advance_borrowed_original(query,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if lookup.is_complete(){if lookup.result().is_none(){self.finish(false)}else{self.phase=Phase::CompareBirth}}
   },
   Phase::CompareBirth=>{self.fields.comparison=Some(ProvisionalInputComparison::new_original(self.original));self.phase=Phase::Compare},
   Phase::Compare=>{
    let fields=&mut *self.fields;let current=fields.lookup.as_ref().and_then(LookupCursor::result).ok_or_else(invalid)?;let input=&fields.payload.as_ref().ok_or_else(invalid)?.input;let comparison=fields.comparison.as_mut().ok_or_else(invalid)?;let turn=comparison.advance(current,input,grant);self.work=turn.work;self.progress=comparison.step_progress();if turn.result?{if comparison.result()==Some(ProvisionalInputMatch::Different){comparison.begin_close();self.phase=Phase::CompareClose}else{self.finish(false)}}
   },
   Phase::CompareClose=>{
    let comparison=self.fields.comparison.as_mut().ok_or_else(invalid)?;if comparison.terminal_is_empty(){self.fields.comparison=None;self.phase=Phase::LookupClose}else{let turn=comparison.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
   },
   Phase::LookupClose=>{self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().ok_or_else(invalid)?).unwrap_or_else(|_|unreachable!("original parameter lookup declares controlled retirement")))},
   Phase::LiteralBirth=>{self.fields.literal=Some(TypedLiteralWork::new_original(self.original));self.phase=Phase::Literal},
   Phase::Literal=>{
    let fields=&mut *self.fields;let input=&fields.payload.as_ref().ok_or_else(invalid)?.input;let literal=fields.literal.as_mut().ok_or_else(invalid)?;let turn=literal.advance(input,grant);self.work=turn.work;self.progress=literal.step_progress();if turn.result?{self.phase=Phase::LiteralTake}
   },
   Phase::LiteralTake=>{
    let literal=self.fields.literal.as_mut().ok_or_else(invalid)?;let turn=literal.take_original(grant);self.work=turn.work;self.progress=Default::default();if let Some((value,p))=turn.result?{self.fields.value=Some(value);self.progress=p;self.fields.literal.as_mut().unwrap().begin_close();self.phase=Phase::LiteralClose}
   },
   Phase::LiteralClose=>{
    let literal=self.fields.literal.as_mut().ok_or_else(invalid)?;if literal.terminal_is_empty(){self.fields.literal=None;self.phase=Phase::PatchBirth}else{let turn=literal.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
   },
   Phase::PatchBirth=>{self.fields.patch=Some(NativeParamPatchWork::new_original(self.fields.value.take().ok_or_else(invalid)?,self.original));self.phase=Phase::Patch},
   Phase::Patch=>{
    let Some(Widget::Neuron{params,..})=source.widgets.get_mut(self.target.ok_or_else(invalid)?)else{return Err(invalid())};
    let fields=&mut *self.fields;let key=&fields.payload.as_ref().ok_or_else(invalid)?.channel;let patch=fields.patch.as_mut().ok_or_else(invalid)?;let turn=patch.advance(params,key,grant);self.work=turn.work;self.progress=patch.step_progress();if turn.result?{self.finish(true)}
   },
  }Ok(self.ready())
 }
 pub(crate) fn step(&mut self,source:&mut FlowHostSnapshot,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{drive(self,source,cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(comparison)=self.fields.comparison.as_mut(){comparison.begin_close()}if let Some(literal)=self.fields.literal.as_mut(){literal.begin_close()}if let Some(patch)=self.fields.patch.as_mut(){patch.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(comparison)=self.fields.comparison.as_ref(){return if comparison.terminal_is_empty(){Ok(fixed(0))}else{comparison.next_close_demands(copy)}}
  if let Some(close)=self.fields.lookup_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  if self.fields.lookup.is_some(){return Ok(fixed(0))}
  if let Some(literal)=self.fields.literal.as_ref(){return if literal.terminal_is_empty(){Ok(fixed(0))}else{literal.next_close_demands(copy)}}
  if let Some(patch)=self.fields.patch.as_ref(){return if patch.terminal_is_empty(){Ok(fixed(0))}else{patch.next_close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.value.is_some(){return self.retirement.next_owner_demands::<Value>(true,&None,copy)}
  if self.fields.payload.is_some(){return self.retirement.next_owner_demands::<ChangeWidgetInput>(true,&None,copy)}Ok(Default::default())
 }
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.work=1;self.progress.copied_items=1;
  if let Some(comparison)=self.fields.comparison.as_mut(){if comparison.terminal_is_empty(){self.fields.comparison=None}else{let turn=comparison.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if let Some(close)=self.fields.lookup_close.as_mut(){if close.terminal_is_empty(){self.fields.lookup_close=None}else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?;}}
  else if self.fields.lookup.is_some(){self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().unwrap()).unwrap_or_else(|_|unreachable!("original parameter lookup declares controlled retirement")))}
  else if let Some(literal)=self.fields.literal.as_mut(){if literal.terminal_is_empty(){self.fields.literal=None}else{let turn=literal.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if let Some(patch)=self.fields.patch.as_mut(){if patch.terminal_is_empty(){self.fields.patch=None}else{let turn=patch.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
  else{
   let turn=if self.fields.value.is_some(){self.retirement.begin_granted(&mut self.fields.value,grant)}else{self.retirement.begin_granted(&mut self.fields.payload,grant)};
   self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.map_or(Default::default(),|step|step.progress());
  }Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.payload.is_none()&&self.fields.lookup.is_none()&&self.fields.lookup_close.is_none()&&self.fields.comparison.is_none()&&self.fields.literal.is_none()&&self.fields.value.is_none()&&self.fields.patch.is_none()&&self.retirement.is_empty()}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
}
impl OriginalScratchWork<FlowHostSnapshot> for ProvisionalTypedInputWork{
 fn origin(&self)->(u64,u64){(self.original.operation().0,self.original.generation().0)}
 fn source_binding(&self)->OriginalSourceBinding{self.original}
 fn ready(&self)->bool{ProvisionalTypedInputWork::ready(self)}
 fn advancing_retirement(&self)->bool{self.fields.lookup_close.is_some()||matches!(self.phase,Phase::CompareClose|Phase::LiteralClose)||self.fields.comparison.as_ref().is_some_and(ProvisionalInputComparison::advancing_retirement)||self.fields.literal.as_ref().is_some_and(TypedLiteralWork::advancing_retirement)||self.fields.patch.as_ref().is_some_and(NativeParamPatchWork::advancing_retirement)}
 fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{ProvisionalTypedInputWork::next_demands(self,source,copy)}
 fn advance(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{ProvisionalTypedInputWork::advance(self,source,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for ProvisionalTypedInputWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional typed input retains its original payload, native fields or map owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

pub enum P3ProvisionalFold{Input(ProvisionalTypedInputWork),Transform(super::retained_provisional_transform::ProvisionalTransformWork)}
impl super::retained_native_provisional_fold::NativeAdditionalFold for P3ProvisionalFold{
 fn step(&mut self,host:&mut FlowHostSnapshot,_generation:&mut super::GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Input(work)=>work.step(host,cx),Self::Transform(work)=>work.step(host,cx)}}
 fn next_demands(&self,host:&FlowHostSnapshot,_generation:&super::GenerationPlayState,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Input(work)=>work.next_demands(host,copy),Self::Transform(work)=>work.next_demands(host,copy)}}
 fn begin_close(&mut self){match self{Self::Input(work)=>work.begin_close(),Self::Transform(work)=>work.begin_close()}}
 fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Input(work)=>work.next_close_demands(copy),Self::Transform(work)=>work.next_close_demands(copy)}}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{match self{Self::Input(work)=>work.close_step(grant),Self::Transform(work)=>work.close_step(grant)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Input(work)=>work.terminal_is_empty(),Self::Transform(work)=>work.terminal_is_empty()}}
 fn step_progress(&self)->RetainedCloneProgress{match self{Self::Input(work)=>work.step_progress(),Self::Transform(work)=>work.step_progress()}}
}
