//! 🧭️ Original gumball leaves compose bounded native reads and paid parameter publication on scratch rows.
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use super::retained_scratch_scan::ScratchRowScan;
use super::retained_native_param_read::{NativeParamReadWork,ParamReadKind};
use super::retained_native_param_patch::NativeParamPatchWork;
use super::retained_typed_literal::TypedLiteralWork;
use semio_s_artifact_procedural_3d_mutations::{WidgetInputValue,GENERATION3D_TRANSLATE_KINDS,GENERATION3D_ROTATE_KINDS,GENERATION3D_SCALE_KINDS};
use semio_s_artifact_procedural_3d_model::transforms::{AxisAngle,compose_scale};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowSnapshotSource,FlowSourceIdentity,Widget,widget_id_for,neural::{Dictionary,Value}};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetireOwned,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,RetireOwned)]
pub(crate) enum TransformKind{Drag,Rotate,Scale}
#[derive(RetireOwned)]
pub(crate) struct ProvisionalTransformInput{pub targets:Vec<String>,pub kind:TransformKind,pub vector:[f64;3],pub angle:f64}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Validate,Unique,Identity,Scan,Kind,ReadBirth,Read,ReadTake,ReadClose,Compose,LiteralBirth,Literal,LiteralTake,LiteralClose,PatchBirth,Patch,PatchClose,InputClose,Next}
struct Fields{payload:Option<ProvisionalTransformInput>,read:Option<NativeParamReadWork>,literal:Option<TypedLiteralWork>,input:Option<WidgetInputValue>,value:Option<Value>,patch:Option<NativeParamPatchWork>}
pub struct ProvisionalTransformWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,binding:Option<FlowSourceIdentity>,phase:Phase,scan:ScratchRowScan,ordinal:usize,target:Option<usize>,kind_index:usize,read_index:usize,lane:usize,first:[f64;3],second:f64,computed:[f64;3],angle:f64,component:usize,ready:bool,applied:bool,closing:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional transform changed its actual scratch source or native owner phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn target_id(value:&String)->&str{value}
impl ProvisionalTransformWork{
 pub(crate) fn new_original(payload:ProvisionalTransformInput,original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{payload:Some(payload),read:None,literal:None,input:None,value:None,patch:None}),original,binding:None,phase:Phase::Validate,scan:ScratchRowScan::new(),ordinal:0,target:None,kind_index:0,read_index:0,lane:0,first:[0.0;3],second:0.0,computed:[0.0;3],angle:0.0,component:0,ready:false,applied:false,closing:false,retirement:Default::default(),progress:Default::default(),work:0}}
 fn payload(&self)->Result<&ProvisionalTransformInput,ValueError>{self.fields.payload.as_ref().ok_or_else(invalid)}
 fn params<'a>(&self,source:&'a FlowHostSnapshot)->Result<&'a Dictionary,ValueError>{match source.widgets.get(self.target.ok_or_else(invalid)?){Some(Widget::Neuron{params,..})=>Ok(params),_=>Err(invalid())}}
 fn kind_names(&self)->Result<&'static [&'static str;3],ValueError>{Ok(match self.payload()?.kind{TransformKind::Drag=>&GENERATION3D_TRANSLATE_KINDS,TransformKind::Rotate=>&GENERATION3D_ROTATE_KINDS,TransformKind::Scale=>&GENERATION3D_SCALE_KINDS})}
 fn key(&self)->Result<&'static str,ValueError>{Ok(match(self.payload()?.kind,self.component){(TransformKind::Drag,_)=>"offset",(TransformKind::Rotate,0)=>"axis",(TransformKind::Rotate,_)=>"angle",(TransformKind::Scale,0)=>"factor",(TransformKind::Scale,_)=>"center"})}
 fn validate(&self,source:&FlowHostSnapshot)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding!=source.source_identity()){Err(invalid())}else{Ok(())}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.ready}
 pub(crate) fn applied(&self)->bool{self.ready&&self.applied}
 pub(crate) fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.ready{return Ok(Default::default())}
  let payload=self.payload()?;
  match self.phase{
   Phase::Validate|Phase::Identity=>Ok(fixed(32)),
   Phase::ReadTake=>Ok(fixed(24)),
   Phase::Unique=>match payload.targets.get(self.ordinal){Some(key)=>self.scan.next_demands(&payload.targets,key,target_id),None=>Ok(fixed(0))},
   Phase::Scan=>match payload.targets.get(self.ordinal){Some(key)=>self.scan.next_demands(&source.widgets,key,widget_id_for),None=>Ok(fixed(0))},
   Phase::Kind=>match source.widgets.get(self.target.ok_or_else(invalid)?){Some(Widget::Neuron{neuron_kind,..})=>Ok(fixed(if neuron_kind.len()==self.kind_names()?[self.kind_index].len(){2*neuron_kind.len()}else{0})),_=>Ok(fixed(0))},
   Phase::Read=>self.fields.read.as_ref().ok_or_else(invalid)?.next_demands(self.params(source)?,copy),
   Phase::ReadClose=>match self.fields.read.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Compose=>Ok(fixed(128)),
   Phase::LiteralBirth=>Ok(fixed(size_of::<WidgetInputValue>())),
   Phase::Literal=>self.fields.literal.as_ref().ok_or_else(invalid)?.next_demands(self.fields.input.as_ref().ok_or_else(invalid)?,copy),
   Phase::LiteralTake=>self.fields.literal.as_ref().ok_or_else(invalid)?.take_demands(),
   Phase::LiteralClose=>match self.fields.literal.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::Patch=>self.fields.patch.as_ref().ok_or_else(invalid)?.next_demands(self.params(source)?,self.key()?,copy),
   Phase::PatchClose=>match self.fields.patch.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   Phase::InputClose=>if !self.retirement.is_empty(){self.retirement.demands(copy)}else if self.fields.input.is_some(){self.retirement.next_owner_demands::<WidgetInputValue>(true,&None,copy)}else{Ok(fixed(0))},
   _=>Ok(fixed(0)),
  }
 }
 pub(crate) fn advance(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(source,grant);if self.work>0||self.progress!=Default::default(){self.binding=Some(source.source_identity())}
  let result=result.map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let demand=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,demand){return Ok(false)}
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   Phase::Validate=>{
    let (kind,vector,angle,empty)={let payload=self.payload()?;(payload.kind,payload.vector,payload.angle,payload.targets.is_empty())};let value=if self.lane<3{vector[self.lane]}else{angle};
    if empty||!value.is_finite()||matches!(kind,TransformKind::Scale)&&self.lane<3&&value<=0.0{self.ready=true}
    else{self.lane+=1;let count=if matches!(kind,TransformKind::Rotate){4}else{3};if self.lane==count{if matches!(kind,TransformKind::Rotate)&&vector==[0.0;3]{self.ready=true}else{self.phase=Phase::Unique}}}
   },
   Phase::Unique=>{
    let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;
    if let Some(key)=payload.targets.get(self.ordinal){
     if key.is_empty(){self.ready=true}else{let turn=self.scan.advance(&payload.targets,key,target_id,grant);self.work=turn.work;self.progress=self.scan.step_progress();if turn.result?{if self.scan.result()!=Some(self.ordinal){self.ready=true}else{self.ordinal+=1;self.scan=ScratchRowScan::new()}}}
    }else{self.phase=Phase::Identity;self.ordinal=0}
   },
   Phase::Identity=>{
    let payload=self.payload()?;let identity=match payload.kind{TransformKind::Drag=>payload.vector==[0.0;3],TransformKind::Rotate=>payload.angle==0.0,TransformKind::Scale=>payload.vector==[1.0;3]};
    if identity{self.ready=true}else{self.phase=Phase::Scan}
   },
   Phase::Scan=>{
    let payload=self.fields.payload.as_ref().ok_or_else(invalid)?;
    if let Some(key)=payload.targets.get(self.ordinal){let turn=self.scan.advance(&source.widgets,key,widget_id_for,grant);self.work=turn.work;self.progress=self.scan.step_progress();if turn.result?{self.target=self.scan.result();self.kind_index=0;self.phase=if self.target.is_some(){Phase::Kind}else{Phase::Next}}}else{self.ready=true}
   },
   Phase::Kind=>{
    if let Some(Widget::Neuron{neuron_kind,..})=source.widgets.get(self.target.ok_or_else(invalid)?){
     let expected=self.kind_names()?[self.kind_index];let equal=neuron_kind.len()==expected.len()&&neuron_kind.as_bytes().iter().zip(expected.as_bytes()).fold(true,|equal,(a,b)|equal&(*a==*b));
     if equal{self.read_index=0;self.phase=Phase::ReadBirth}else{self.kind_index+=1;if self.kind_index==3{self.phase=Phase::Next}}
    }else{self.phase=Phase::Next}
   },
   Phase::ReadBirth=>{
    let kind=match(self.payload()?.kind,self.read_index){(TransformKind::Drag,_)=>ParamReadKind::Vector{key:"offset",fallback:[0.0;3]},(TransformKind::Rotate,0)=>ParamReadKind::Vector{key:"axis",fallback:[0.0,0.0,1.0]},(TransformKind::Rotate,_)=>ParamReadKind::Number{key:"angle",fallback:0.0},(TransformKind::Scale,_)=>ParamReadKind::Vector{key:"factor",fallback:[1.0;3]}};
    self.fields.read=Some(NativeParamReadWork::new_original(kind,self.original));self.phase=Phase::Read
   },
   Phase::Read=>{
    let index=self.target.ok_or_else(invalid)?;let Some(Widget::Neuron{params,..})=source.widgets.get(index)else{return Err(invalid())};
    let read=self.fields.read.as_mut().ok_or_else(invalid)?;let turn=read.advance(params,grant);self.work=turn.work;self.progress=read.step_progress();
    if turn.result?{self.phase=Phase::ReadTake}
   },
   Phase::ReadTake=>{let read=self.fields.read.as_mut().ok_or_else(invalid)?;let values=*read.values().ok_or_else(invalid)?;if self.read_index==0{self.first=values}else{self.second=values[0]}read.begin_close();self.phase=Phase::ReadClose},
   Phase::ReadClose=>{
    let read=self.fields.read.as_mut().ok_or_else(invalid)?;
    if read.terminal_is_empty(){self.fields.read=None;self.read_index+=1;self.phase=if matches!(self.payload()?.kind,TransformKind::Rotate)&&self.read_index==1{Phase::ReadBirth}else{Phase::Compose}}
    else{let turn=read.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
   },
   Phase::Compose=>{
    let payload=self.payload()?;
    let result=match payload.kind{
     TransformKind::Drag=>{let value=std::array::from_fn(|axis|self.first[axis]+payload.vector[axis]);if value.iter().all(|value|value.is_finite()){Some((value,0.0))}else{None}},
     TransformKind::Rotate=>AxisAngle{axis:self.first,angle:self.second}.then(AxisAngle{axis:payload.vector,angle:payload.angle}).ok().map(|value|(value.axis,value.angle)),
     TransformKind::Scale=>compose_scale(self.first,payload.vector).ok().map(|value|(value,0.0)),
    };
    if let Some((values,angle))=result{self.computed=values;self.angle=angle;self.component=0;self.phase=Phase::LiteralBirth}else{self.phase=Phase::Next}
   },
   Phase::LiteralBirth=>{
    self.fields.input=Some(match(self.payload()?.kind,self.component){(TransformKind::Rotate,1)=>WidgetInputValue::Number(self.angle),(TransformKind::Scale,1)=>WidgetInputValue::Point([0.0;3]),_=>WidgetInputValue::Vector(self.computed)});
    self.fields.literal=Some(TypedLiteralWork::new_original(self.original));self.phase=Phase::Literal
   },
   Phase::Literal=>{
    let fields=&mut*self.fields;let work=fields.literal.as_mut().ok_or_else(invalid)?;let turn=work.advance(fields.input.as_ref().ok_or_else(invalid)?,grant);self.work=turn.work;self.progress=work.step_progress();if turn.result?{self.phase=Phase::LiteralTake}
   },
   Phase::LiteralTake=>{
    let work=self.fields.literal.as_mut().ok_or_else(invalid)?;let turn=work.take_original(grant);self.work=turn.work;self.progress=Default::default();if let Some((value,p))=turn.result?{self.fields.value=Some(value);self.progress=p;self.fields.literal.as_mut().unwrap().begin_close();self.phase=Phase::LiteralClose}
   },
   Phase::LiteralClose=>{
    let work=self.fields.literal.as_mut().ok_or_else(invalid)?;if work.terminal_is_empty(){self.fields.literal=None;self.phase=Phase::PatchBirth}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
   },
   Phase::PatchBirth=>{self.fields.patch=Some(NativeParamPatchWork::new_original(self.fields.value.take().ok_or_else(invalid)?,self.original));self.phase=Phase::Patch},
   Phase::Patch=>{
    let key=self.key()?;let Some(Widget::Neuron{params,..})=source.widgets.get_mut(self.target.ok_or_else(invalid)?)else{return Err(invalid())};
    let work=self.fields.patch.as_mut().ok_or_else(invalid)?;let turn=work.advance(params,key,grant);self.work=turn.work;self.progress=work.step_progress();if turn.result?{self.applied=true;work.begin_close();self.phase=Phase::PatchClose}
   },
   Phase::PatchClose=>{
    let work=self.fields.patch.as_mut().ok_or_else(invalid)?;if work.terminal_is_empty(){self.fields.patch=None;self.phase=Phase::InputClose}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
   },
   Phase::InputClose=>{
    if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
    else if self.fields.input.is_some(){let turn=self.retirement.begin_granted(&mut self.fields.input,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
    else{self.component+=1;self.phase=if !matches!(self.payload()?.kind,TransformKind::Drag)&&self.component==1{Phase::LiteralBirth}else{Phase::Next}}
   },
   Phase::Next=>{self.ordinal+=1;self.scan=ScratchRowScan::new();self.target=None;self.phase=Phase::Scan},
  }Ok(self.ready())
 }
 pub(crate) fn step(&mut self,source:&mut FlowHostSnapshot,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{drive(self,source,cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(work)=self.fields.read.as_mut(){work.begin_close()}if let Some(work)=self.fields.literal.as_mut(){work.begin_close()}if let Some(work)=self.fields.patch.as_mut(){work.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(work)=self.fields.read.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if let Some(work)=self.fields.literal.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if let Some(work)=self.fields.patch.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.value.is_some(){return self.retirement.next_owner_demands::<Value>(true,&None,copy)}
  if self.fields.input.is_some(){return self.retirement.next_owner_demands::<WidgetInputValue>(true,&None,copy)}
  if self.fields.payload.is_some(){return self.retirement.next_owner_demands::<ProvisionalTransformInput>(true,&None,copy)}Ok(Default::default())
 }
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.work=0;self.progress=Default::default();let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.work=1;self.progress.copied_items=1;
  if let Some(work)=self.fields.read.as_mut(){if work.terminal_is_empty(){self.fields.read=None}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if let Some(work)=self.fields.literal.as_mut(){if work.terminal_is_empty(){self.fields.literal=None}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if let Some(work)=self.fields.patch.as_mut(){if work.terminal_is_empty(){self.fields.patch=None}else{let turn=work.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}}
  else if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
  else{let turn=if self.fields.value.is_some(){self.retirement.begin_granted(&mut self.fields.value,grant)}else if self.fields.input.is_some(){self.retirement.begin_granted(&mut self.fields.input,grant)}else{self.retirement.begin_granted(&mut self.fields.payload,grant)};self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.payload.is_none()&&self.fields.read.is_none()&&self.fields.literal.is_none()&&self.fields.input.is_none()&&self.fields.value.is_none()&&self.fields.patch.is_none()&&self.retirement.is_empty()}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
}
impl OriginalScratchWork<FlowHostSnapshot> for ProvisionalTransformWork{
 fn origin(&self)->(u64,u64){(self.original.operation().0,self.original.generation().0)}
 fn source_binding(&self)->OriginalSourceBinding{self.original}
 fn ready(&self)->bool{ProvisionalTransformWork::ready(self)}
 fn advancing_retirement(&self)->bool{matches!(self.phase,Phase::ReadClose|Phase::LiteralClose|Phase::PatchClose|Phase::InputClose)||self.fields.read.as_ref().is_some_and(NativeParamReadWork::advancing_retirement)||self.fields.literal.as_ref().is_some_and(TypedLiteralWork::advancing_retirement)||self.fields.patch.as_ref().is_some_and(NativeParamPatchWork::advancing_retirement)}
 fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{ProvisionalTransformWork::next_demands(self,source,copy)}
 fn advance(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{ProvisionalTransformWork::advance(self,source,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for ProvisionalTransformWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional transform retains original targets, native reads or paid parameter owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
