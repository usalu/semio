//! ⚖️ Provisional input comparison reads original neural dictionaries through their actual retained lookup owners.
use super::retained_typed_literal::InputIdentity;
use super::retained_scratch_driver::{OriginalScratchWork,drive};
use semio_s_artifact_procedural_3d_mutations::WidgetInputValue;
use semio_framework_artifact_flow_flow::neural::{Value,Atom,Dictionary};
use semio_os_flow::LookupCursor;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum ProvisionalInputMatch{Equal,Different,Refused}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Kind{Number,Text,Boolean,Point,Vector,Plane,List}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Node{Root,Item,Part}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Start,ProbeBirth,ProbeRun,SchemaRead,NodeReady,FieldStart,FieldRead,Text,FieldNext,ListStart,ItemBirth,ItemRun,ItemReady,ListNext,PartStart,PartBirth,PartRun,PartReady,PartNext,CloseProbe,CloseItem,ClosePart}
struct Fields{probe:Option<LookupCursor<Value>>,item:Option<LookupCursor<Value>>,part:Option<LookupCursor<Value>>,close:Option<ControlledRetirement<LookupCursor<Value>>>}
pub(crate) struct ProvisionalInputComparison{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,binding:Option<(CurrentIdentity,InputIdentity)>,phase:Phase,after:Phase,node:Node,kind:Kind,schema:bool,field:usize,index:usize,count:usize,part:usize,key:[u8;4],key_length:usize,offset:usize,characters:usize,different:bool,result:Option<ProvisionalInputMatch>,closing:bool,progress:RetainedCloneProgress,work:u64}
#[derive(Clone,Copy,PartialEq,Eq)]
enum CurrentHeader{Null,Boolean(bool),Integer(i64),Decimal(u64),Text(usize,usize),Dictionary(Option<usize>,usize)}
#[derive(Clone,Copy,PartialEq,Eq)]
struct CurrentIdentity{record:usize,header:CurrentHeader}
impl CurrentIdentity{
 fn capture(current:&Value)->Self{let header=match current{Value::Atom(Atom::Null)=>CurrentHeader::Null,Value::Atom(Atom::Boolean(value))=>CurrentHeader::Boolean(*value),Value::Atom(Atom::Integer(value))=>CurrentHeader::Integer(*value),Value::Atom(Atom::Decimal(value))=>CurrentHeader::Decimal(value.to_bits()),Value::Atom(Atom::String(value))=>CurrentHeader::Text(value.as_ptr()as usize,value.len()),Value::Dictionary(value)=>CurrentHeader::Dictionary(value.original_root_identity(),value.len())};Self{record:current as*const Value as usize,header}}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"provisional input comparison changed its original fields or receiving phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn list(input:&WidgetInputValue)->bool{matches!(input,WidgetInputValue::NumberList(_)|WidgetInputValue::TextList(_)|WidgetInputValue::BooleanList(_)|WidgetInputValue::PointList(_)|WidgetInputValue::VectorList(_))}
fn count(input:&WidgetInputValue)->usize{match input{WidgetInputValue::NumberList(v)=>v.len(),WidgetInputValue::TextList(v)=>v.len(),WidgetInputValue::BooleanList(v)=>v.len(),WidgetInputValue::PointList(v)|WidgetInputValue::VectorList(v)=>v.len(),_=>1}}
fn base(input:&WidgetInputValue)->Kind{match input{WidgetInputValue::Number(_)|WidgetInputValue::NumberList(_)=>Kind::Number,WidgetInputValue::Text(_)|WidgetInputValue::TextList(_)=>Kind::Text,WidgetInputValue::Boolean(_)|WidgetInputValue::BooleanList(_)=>Kind::Boolean,WidgetInputValue::Point(_)|WidgetInputValue::PointList(_)=>Kind::Point,WidgetInputValue::Vector(_)|WidgetInputValue::VectorList(_)=>Kind::Vector,WidgetInputValue::Plane(_)=>Kind::Plane}}
fn schema(kind:Kind)->&'static str{match kind{Kind::Number=>"number",Kind::Text=>"text",Kind::Boolean=>"boolean",Kind::Point=>"point",Kind::Vector=>"vector",Kind::Plane=>"plane",Kind::List=>"list"}}
fn field_key(kind:Kind,field:usize)->&'static str{match kind{Kind::Point|Kind::Vector=>match field{0=>"x",1=>"y",_=>"z"},_=>"value"}}
fn field_count(kind:Kind)->usize{if matches!(kind,Kind::Point|Kind::Vector){3}else{1}}
fn probe_text(probe:&Option<LookupCursor<Value>>)->Option<&str>{probe.as_ref()?.result()?.as_atom()?.as_str()}
fn numeric(value:Option<&Value>)->Option<f64>{match value?.as_atom()?{Atom::Integer(value)=>Some(*value as f64),Atom::Decimal(value)=>Some(*value),_=>None}}
fn numeric_carrier(value:Option<&Value>)->bool{matches!(value.and_then(Value::as_atom),Some(Atom::Integer(_)|Atom::Decimal(_)))}
fn wanted_number(input:&WidgetInputValue,node:Node,index:usize,part:usize,field:usize)->Option<&f64>{match input{
 WidgetInputValue::Number(v)=>Some(v),WidgetInputValue::NumberList(v)=>v.get(index),
 WidgetInputValue::Point(v)|WidgetInputValue::Vector(v)=>v.get(field),WidgetInputValue::PointList(v)|WidgetInputValue::VectorList(v)=>v.get(index)?.get(field),
 WidgetInputValue::Plane(v)if node==Node::Part=>if part==0{v.origin.get(field)}else{v.normal.get(field)},_=>None,
}}
fn wanted_text(input:&WidgetInputValue,index:usize)->Option<&str>{match input{WidgetInputValue::Text(v)=>Some(v),WidgetInputValue::TextList(v)=>v.get(index).map(String::as_str),_=>None}}
fn wanted_boolean(input:&WidgetInputValue,index:usize)->Option<&bool>{match input{WidgetInputValue::Boolean(v)=>Some(v),WidgetInputValue::BooleanList(v)=>v.get(index),_=>None}}
fn digits(mut value:usize)->usize{let mut count=1;while value>=10{value/=10;count+=1}count}
impl ProvisionalInputComparison{
 pub(crate) fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{probe:None,item:None,part:None,close:None}),original,binding:None,phase:Phase::Start,after:Phase::Start,node:Node::Root,kind:Kind::Number,schema:true,field:0,index:0,count:0,part:0,key:[0;4],key_length:0,offset:0,characters:0,different:false,result:None,closing:false,progress:Default::default(),work:0}}
 pub(crate) fn result(&self)->Option<ProvisionalInputMatch>{self.result}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.result.is_some()}
 fn finish(&mut self,result:ProvisionalInputMatch){self.result=Some(result)}
 fn complete(&mut self){self.finish(if self.different{ProvisionalInputMatch::Different}else{ProvisionalInputMatch::Equal})}
 fn validate(&self,current:&Value,input:&WidgetInputValue)->Result<(),ValueError>{if self.closing||self.binding.is_some_and(|binding|binding.0!=CurrentIdentity::capture(current)||!binding.1.matches(input)){Err(invalid())}else{Ok(())}}
 fn node<'a>(&'a self,current:&'a Value)->Option<&'a Value>{match self.node{Node::Root=>Some(current),Node::Item=>self.fields.item.as_ref()?.result(),Node::Part=>self.fields.part.as_ref()?.result()}}
 fn dictionary<'a>(&'a self,current:&'a Value)->Option<&'a Dictionary>{self.node(current)?.as_dictionary()}
 fn query(&self)->&'static str{if self.schema{"$schema"}else{field_key(self.kind,self.field)}}
 pub(crate) fn next_demands(&self,current:&Value,input:&WidgetInputValue,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(current,input)?;if self.ready(){return Ok(Default::default())}
  if let Some(close)=self.fields.close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  Ok(match self.phase{
   Phase::ProbeBirth=>self.dictionary(current).map_or(fixed(0),|dictionary|dictionary.pairs().lookup_borrowed_constructor_demands()),
   Phase::ProbeRun=>return self.fields.probe.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(self.query()),
   Phase::SchemaRead=>fixed(probe_text(&self.fields.probe).map_or(0,|text|if text.len()==schema(self.kind).len(){2*text.len()}else{0})),
   Phase::FieldRead=>{
    let value=self.fields.probe.as_ref().and_then(LookupCursor::result);
    fixed(match self.kind{
     Kind::Number|Kind::Point|Kind::Vector=>if numeric_carrier(value){size_of::<f64>()*(1+usize::from(wanted_number(input,self.node,self.index,self.part,self.field).is_some()))}else{0},
     Kind::Boolean=>if matches!(value.and_then(Value::as_atom),Some(Atom::Boolean(_))){1+usize::from(wanted_boolean(input,self.index).is_some())}else{0},
     _=>0,
    })
   },
   Phase::Text=>{
    let text=probe_text(&self.fields.probe).ok_or_else(invalid)?;let wanted=wanted_text(input,self.index);let pairs=wanted.is_some_and(|wanted|wanted.len()==text.len());let bytes=text.len().saturating_sub(self.offset).min(64).min((copy/(1+usize::from(pairs))).max(1));fixed(bytes*(1+usize::from(pairs)))
   },
   Phase::ListStart=>fixed(if self.index<self.count{digits(self.index)}else{0}),
   Phase::ItemBirth|Phase::PartBirth=>current.as_dictionary().map_or(fixed(0),|dictionary|dictionary.pairs().lookup_borrowed_constructor_demands()),
   Phase::ItemRun=>return self.fields.item.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(std::str::from_utf8(&self.key[..self.key_length]).map_err(|_|invalid())?),
   Phase::PartRun=>return self.fields.part.as_ref().ok_or_else(invalid)?.next_borrowed_original_demands(if self.part==0{"origin"}else{"normal"}),
   _=>fixed(0),
  })
 }
 pub(crate) fn advance(&mut self,current:&Value,input:&WidgetInputValue,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(current,input,grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_original(&mut self,current:&Value,input:&WidgetInputValue,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(current,input,grant.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !retained_clone_grant_admits(grant,d){return Ok(false)}
  self.binding=Some((CurrentIdentity::capture(current),InputIdentity::capture(input)));
  if let Some(close)=self.fields.close.as_mut(){
   if close.terminal_is_empty(){self.fields.close=None;self.phase=self.after;self.work=1;self.progress.copied_items=1}
   else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?;}return Ok(false)
  }
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  match self.phase{
   Phase::Start=>{self.kind=if list(input){Kind::List}else{base(input)};self.node=Node::Root;self.schema=true;self.phase=Phase::ProbeBirth},
   Phase::ProbeBirth=>{
    let Some(dictionary)=self.dictionary(current)else{self.finish(ProvisionalInputMatch::Refused);return Ok(true)};
    let turn=dictionary.pairs().admit_lookup_borrowed_original(self.query(),grant);self.work=turn.work;self.progress=Default::default();let(cursor,p)=turn.result?;self.fields.probe=Some(cursor);self.progress=p;self.phase=Phase::ProbeRun
   },
   Phase::ProbeRun=>{
    let query=self.query();let probe=self.fields.probe.as_mut().ok_or_else(invalid)?;let turn=probe.advance_borrowed_original(query,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if probe.is_complete(){self.phase=if self.schema{Phase::SchemaRead}else{Phase::FieldRead}}
   },
   Phase::SchemaRead=>{
    let wanted=schema(self.kind);let matches=probe_text(&self.fields.probe).is_some_and(|value|value.len()==wanted.len()&&value.as_bytes().iter().zip(wanted.as_bytes()).fold(true,|equal,(a,b)|equal&(*a==*b)));
    if matches{self.phase=Phase::CloseProbe;self.after=Phase::NodeReady}else{self.finish(ProvisionalInputMatch::Refused)}
   },
   Phase::NodeReady=>{
    match self.kind{
     Kind::List=>{self.count=self.dictionary(current).ok_or_else(invalid)?.len().saturating_sub(1);if self.count>1024{self.finish(ProvisionalInputMatch::Refused)}else{self.different|=self.count!=count(input);self.index=0;self.phase=Phase::ListStart}},
     Kind::Plane=>{self.part=0;self.phase=Phase::PartStart},
     _=>{self.field=0;self.phase=Phase::FieldStart},
    }
   },
   Phase::FieldStart=>{
    if self.field==field_count(self.kind){match self.node{Node::Root=>self.complete(),Node::Item=>{self.phase=Phase::CloseItem;self.after=Phase::ListNext},Node::Part=>{self.phase=Phase::ClosePart;self.after=Phase::PartNext}}}
    else{self.schema=false;self.phase=Phase::ProbeBirth}
   },
   Phase::FieldRead=>{
    let value=self.fields.probe.as_ref().and_then(LookupCursor::result);
    match self.kind{
     Kind::Number|Kind::Point|Kind::Vector=>{
      let Some(value)=numeric(value)else{self.finish(ProvisionalInputMatch::Refused);return Ok(true)};
      if self.node==Node::Item&&!value.is_finite(){self.finish(ProvisionalInputMatch::Refused);return Ok(true)}
      if let Some(wanted)=wanted_number(input,self.node,self.index,self.part,self.field){self.different|=value!=*wanted}
      self.phase=Phase::CloseProbe;self.after=Phase::FieldNext
     },
     Kind::Boolean=>{
      let Some(Atom::Boolean(value))=value.and_then(Value::as_atom)else{self.finish(ProvisionalInputMatch::Refused);return Ok(true)};
      if let Some(wanted)=wanted_boolean(input,self.index){self.different|=*value!=*wanted}self.phase=Phase::CloseProbe;self.after=Phase::FieldNext
     },
     Kind::Text=>{
      let Some(text)=probe_text(&self.fields.probe)else{self.finish(ProvisionalInputMatch::Refused);return Ok(true)};
      let wanted=wanted_text(input,self.index);let same_length=wanted.is_some_and(|wanted|wanted.len()==text.len());self.different|=!same_length;self.offset=0;self.characters=0;
      if self.node!=Node::Item&&!same_length{self.phase=Phase::CloseProbe;self.after=Phase::FieldNext}else{self.phase=Phase::Text}
     },
     _=>return Err(invalid()),
    }
   },
   Phase::Text=>{
    let text=probe_text(&self.fields.probe).ok_or_else(invalid)?;let wanted=wanted_text(input,self.index);let pairs=wanted.is_some_and(|wanted|wanted.len()==text.len());let bytes=d.copy_bytes/(1+usize::from(pairs));let end=self.offset+bytes;let source=&text.as_bytes()[self.offset..end];
    self.characters+=source.iter().filter(|byte|**byte&0xc0!=0x80).count();if pairs{let wanted=wanted.unwrap();self.different|=source.iter().zip(&wanted.as_bytes()[self.offset..end]).fold(false,|different,(a,b)|different|(*a!=*b))}self.offset=end;
    if self.node==Node::Item&&self.characters>16_777_216{self.finish(ProvisionalInputMatch::Refused)}
    else if self.offset==text.len(){self.phase=Phase::CloseProbe;self.after=Phase::FieldNext}
   },
   Phase::FieldNext=>{self.field+=1;self.phase=Phase::FieldStart},
   Phase::ListStart=>{
    if self.index==self.count{self.complete()}
    else{self.key_length=digits(self.index);let mut value=self.index;for at in (0..self.key_length).rev(){self.key[at]=b'0'+(value%10)as u8;value/=10}self.phase=Phase::ItemBirth}
   },
   Phase::ItemBirth=>{
    let dictionary=current.as_dictionary().ok_or_else(invalid)?;let query=std::str::from_utf8(&self.key[..self.key_length]).map_err(|_|invalid())?;let turn=dictionary.pairs().admit_lookup_borrowed_original(query,grant);self.work=turn.work;self.progress=Default::default();let(cursor,p)=turn.result?;self.fields.item=Some(cursor);self.progress=p;self.phase=Phase::ItemRun
   },
   Phase::ItemRun=>{
    let query=std::str::from_utf8(&self.key[..self.key_length]).map_err(|_|invalid())?;let item=self.fields.item.as_mut().ok_or_else(invalid)?;let turn=item.advance_borrowed_original(query,grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if item.is_complete(){self.phase=Phase::ItemReady}
   },
   Phase::ItemReady=>{if self.fields.item.as_ref().and_then(LookupCursor::result).is_none(){self.finish(ProvisionalInputMatch::Refused)}else{self.node=Node::Item;self.kind=base(input);self.schema=true;self.phase=Phase::ProbeBirth}},
   Phase::ListNext=>{self.index+=1;self.node=Node::Root;self.kind=Kind::List;self.phase=Phase::ListStart},
   Phase::PartStart=>{if self.part==2{self.complete()}else{self.phase=Phase::PartBirth}},
   Phase::PartBirth=>{
    let dictionary=current.as_dictionary().ok_or_else(invalid)?;let turn=dictionary.pairs().admit_lookup_borrowed_original(if self.part==0{"origin"}else{"normal"},grant);self.work=turn.work;self.progress=Default::default();let(cursor,p)=turn.result?;self.fields.part=Some(cursor);self.progress=p;self.phase=Phase::PartRun
   },
   Phase::PartRun=>{
    let part=self.fields.part.as_mut().ok_or_else(invalid)?;let turn=part.advance_borrowed_original(if self.part==0{"origin"}else{"normal"},grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if part.is_complete(){self.phase=Phase::PartReady}
   },
   Phase::PartReady=>{if self.fields.part.as_ref().and_then(LookupCursor::result).is_none(){self.finish(ProvisionalInputMatch::Refused)}else{self.node=Node::Part;self.kind=if self.part==0{Kind::Point}else{Kind::Vector};self.schema=true;self.phase=Phase::ProbeBirth}},
   Phase::PartNext=>{self.part+=1;self.node=Node::Root;self.kind=Kind::Plane;self.phase=Phase::PartStart},
   Phase::CloseProbe|Phase::CloseItem|Phase::ClosePart=>{
    let cursor=match self.phase{Phase::CloseProbe=>self.fields.probe.take(),Phase::CloseItem=>self.fields.item.take(),_=>self.fields.part.take()}.ok_or_else(invalid)?;
    self.fields.close=Some(ControlledRetirement::new(cursor).unwrap_or_else(|_|unreachable!("original neural lookup declares controlled retirement")))
   },
  }Ok(self.ready())
 }
 pub(crate) fn step(&mut self,current:&Value,input:&WidgetInputValue,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut source=(current,input);drive(self,&mut source,cx)}
 pub(crate) fn begin_close(&mut self){self.closing=true}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(close)=self.fields.close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}Ok(if self.fields.probe.is_some()||self.fields.item.is_some()||self.fields.part.is_some(){fixed(0)}else{Default::default()})}
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(close)=self.fields.close.as_mut(){if close.terminal_is_empty(){self.fields.close=None;self.work=1;self.progress.copied_items=1}else{let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();result?;}}
  else{let cursor=self.fields.probe.take().or_else(||self.fields.part.take()).or_else(||self.fields.item.take()).ok_or_else(invalid)?;self.fields.close=Some(ControlledRetirement::new(cursor).unwrap_or_else(|_|unreachable!("original neural lookup declares controlled retirement")));self.work=1;self.progress.copied_items=1}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.probe.is_none()&&self.fields.item.is_none()&&self.fields.part.is_none()&&self.fields.close.is_none()}
 pub(crate) fn advancing_retirement(&self)->bool{self.fields.close.is_some()}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
}
impl<'a,'b> OriginalScratchWork<(&'a Value,&'b WidgetInputValue)> for ProvisionalInputComparison{
 fn origin(&self)->(u64,u64){(self.original.operation().0,self.original.generation().0)}
 fn source_binding(&self)->OriginalSourceBinding{self.original}
 fn ready(&self)->bool{ProvisionalInputComparison::ready(self)}
 fn advancing_retirement(&self)->bool{ProvisionalInputComparison::advancing_retirement(self)}
 fn next_demands(&self,source:&(&'a Value,&'b WidgetInputValue),copy:usize)->Result<RetirementDemand,ValueError>{ProvisionalInputComparison::next_demands(self,source.0,source.1,copy)}
 fn advance(&mut self,source:&mut (&'a Value,&'b WidgetInputValue),grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{ProvisionalInputComparison::advance(self,source.0,source.1,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for ProvisionalInputComparison{fn drop(&mut self){assert!(self.terminal_is_empty(),"provisional neural comparison retains actual lookup owners");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
