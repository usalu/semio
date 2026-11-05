//! 💎️ Native typed literals borrow original inputs and fund the existing text and AVL owners.
use semio_s_artifact_procedural_3d_mutations::WidgetInputValue;
use semio_framework_artifact_flow_flow::neural::{Value,Atom,Dictionary};
use semio_os_flow::{OrderedMap,OriginalOwnedEntry,FlowTextCopyWork};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};
const META:usize=2*size_of::<usize>()+2*size_of::<u8>();
#[derive(Clone,Copy,PartialEq,Eq)]
pub(crate) struct InputIdentity{address:usize,variant:u8,pointer:usize,length:usize,words:[u64;6]}
impl InputIdentity{
 pub(crate) fn matches(self,input:&WidgetInputValue)->bool{self==Self::capture(input)}
 pub(crate) fn capture(input:&WidgetInputValue)->Self{
  let mut identity=Self{address:input as*const _ as usize,variant:0,pointer:0,length:0,words:[0;6]};
  match input{
   WidgetInputValue::Number(value)=>identity.words[0]=value.to_bits(),
   WidgetInputValue::Text(value)=>{identity.variant=1;identity.pointer=value.as_ptr()as usize;identity.length=value.len()},
   WidgetInputValue::Boolean(value)=>{identity.variant=2;identity.words[0]=u64::from(*value)},
   WidgetInputValue::Point(values)|WidgetInputValue::Vector(values)=>{identity.variant=if matches!(input,WidgetInputValue::Point(_)){3}else{4};for(i,value)in values.iter().enumerate(){identity.words[i]=value.to_bits()}},
   WidgetInputValue::Plane(value)=>{identity.variant=5;for(i,value)in value.origin.iter().chain(value.normal.iter()).enumerate(){identity.words[i]=value.to_bits()}},
   WidgetInputValue::NumberList(values)=>{identity.variant=6;identity.pointer=values.as_ptr()as usize;identity.length=values.len()},
   WidgetInputValue::TextList(values)=>{identity.variant=7;identity.pointer=values.as_ptr()as usize;identity.length=values.len()},
   WidgetInputValue::BooleanList(values)=>{identity.variant=8;identity.pointer=values.as_ptr()as usize;identity.length=values.len()},
   WidgetInputValue::PointList(values)|WidgetInputValue::VectorList(values)=>{identity.variant=if matches!(input,WidgetInputValue::PointList(_)){9}else{10};identity.pointer=values.as_ptr()as usize;identity.length=values.len()},
  }identity
 }
}
#[derive(Clone,Copy)]enum View{Root,Item(usize),Origin,Normal}
#[derive(Clone,Copy)]struct Frame{view:View,member:usize}
#[derive(Clone,Copy)]enum Key{Static(&'static str),Index(usize)}
#[derive(Clone,Copy)]enum Payload<'a>{Number(f64),Boolean(bool),Text(&'a str),Child(View)}
#[derive(Clone,Copy)]struct Field<'a>{key:Key,payload:Payload<'a>}
#[derive(Clone,Copy)]enum Literal<'a>{Number(f64),Text(&'a str),Boolean(bool),Axes(&'static str,&'a [f64;3]),Plane,List(usize)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typed literal source or receiving phase changed")}
fn literal(input:&WidgetInputValue,view:View)->Result<Literal<'_>,ValueError>{
 use WidgetInputValue as I;
 Ok(match view{
  View::Root=>match input{I::Number(v)=>Literal::Number(*v),I::Text(v)=>Literal::Text(v),I::Boolean(v)=>Literal::Boolean(*v),I::Point(v)=>Literal::Axes("point",v),I::Vector(v)=>Literal::Axes("vector",v),I::Plane(_)=>Literal::Plane,I::NumberList(v)=>Literal::List(v.len()),I::TextList(v)=>Literal::List(v.len()),I::BooleanList(v)=>Literal::List(v.len()),I::PointList(v)|I::VectorList(v)=>Literal::List(v.len())},
  View::Item(i)=>match input{I::NumberList(v)=>Literal::Number(*v.get(i).ok_or_else(invalid)?),I::TextList(v)=>Literal::Text(v.get(i).ok_or_else(invalid)?),I::BooleanList(v)=>Literal::Boolean(*v.get(i).ok_or_else(invalid)?),I::PointList(v)=>Literal::Axes("point",v.get(i).ok_or_else(invalid)?),I::VectorList(v)=>Literal::Axes("vector",v.get(i).ok_or_else(invalid)?),_=>return Err(invalid())},
  View::Origin=>match input{I::Plane(v)=>Literal::Axes("point",&v.origin),_=>return Err(invalid())},
  View::Normal=>match input{I::Plane(v)=>Literal::Axes("vector",&v.normal),_=>return Err(invalid())},
 })
}
fn field(input:&WidgetInputValue,frame:Frame)->Result<Option<Field<'_>>,ValueError>{
 let value=literal(input,frame.view)?;
 let schema=match value{Literal::Number(_)=>"number",Literal::Text(_)=>"text",Literal::Boolean(_)=>"boolean",Literal::Axes(schema,_)=>schema,Literal::Plane=>"plane",Literal::List(_)=>"list"};
 if frame.member==0{return Ok(Some(Field{key:Key::Static("$schema"),payload:Payload::Text(schema)}))}
 let member=frame.member-1;
 Ok(match value{
  Literal::Number(value)=>(member==0).then_some(Field{key:Key::Static("value"),payload:Payload::Number(value)}),
  Literal::Text(value)=>(member==0).then_some(Field{key:Key::Static("value"),payload:Payload::Text(value)}),
  Literal::Boolean(value)=>(member==0).then_some(Field{key:Key::Static("value"),payload:Payload::Boolean(value)}),
  Literal::Axes(_,axes)=>axes.get(member).map(|value|Field{key:Key::Static(["x","y","z"][member]),payload:Payload::Number(*value)}),
  Literal::Plane=>match member{0=>Some(Field{key:Key::Static("origin"),payload:Payload::Child(View::Origin)}),1=>Some(Field{key:Key::Static("normal"),payload:Payload::Child(View::Normal)}),_=>None},
  Literal::List(count)=>(member<count).then_some(Field{key:Key::Index(member),payload:Payload::Child(View::Item(member))}),
 })
}
#[derive(Clone,Copy,PartialEq,Eq)]enum Phase{Bind,Field,Text,TextClose,Value,String,Entry,EntryStep,EntryClose,Map,Ready,Taken}
#[derive(Clone,Copy,PartialEq,Eq)]enum Target{Key,Value}
struct Owners{map0:Option<OrderedMap<Value>>,map1:Option<OrderedMap<Value>>,key0:Option<String>,key1:Option<String>,string:Option<String>,value:Option<Value>,map_output:Option<OrderedMap<Value>>,output:Option<Value>}
pub(crate) struct TypedLiteralWork{
 owners:ManuallyDrop<Owners>,text:Option<FlowTextCopyWork>,entry:Option<OriginalOwnedEntry<Value>>,retirement:RetainedCloneClose,
 binding:Option<OriginalSourceBinding>,identity:Option<InputIdentity>,frames:[Frame;2],depth:usize,numeric:[u8;20],numeric_length:usize,
 phase:Phase,target:Target,closing:bool,progress:RetainedCloneProgress,work:u64,
}
impl TypedLiteralWork{
 pub(crate) fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(binding:OriginalSourceBinding)->Self{Self{owners:ManuallyDrop::new(Owners{map0:None,map1:None,key0:None,key1:None,string:None,value:None,map_output:None,output:None}),text:None,entry:None,retirement:Default::default(),binding:Some(binding),identity:None,frames:[Frame{view:View::Root,member:0};2],depth:0,numeric:[0;20],numeric_length:0,phase:Phase::Bind,target:Target::Key,closing:false,progress:Default::default(),work:0}}
 pub(crate) fn source_binding(&self)->OriginalSourceBinding{self.binding.expect("live original typed literal binding")}
 pub(crate) fn step(&mut self,input:&WidgetInputValue,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let mut source=input;super::retained_scratch_driver::drive(self,&mut source,cx)}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
 pub(crate) fn advancing_retirement(&self)->bool{matches!(self.phase,Phase::TextClose|Phase::EntryClose)||self.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
 fn checked(&self,input:&WidgetInputValue)->Result<(),ValueError>{if self.closing||self.identity.is_some_and(|identity|identity!=InputIdentity::capture(input)){Err(invalid())}else{Ok(())}}
 fn current(&self)->Result<Frame,ValueError>{self.depth.checked_sub(1).and_then(|i|self.frames.get(i)).copied().ok_or_else(invalid)}
 fn source<'a>(input:&'a WidgetInputValue,frame:Frame,target:Target,numeric:&'a [u8;20],numeric_length:usize)->Result<&'a str,ValueError>{
  let field=field(input,frame)?.ok_or_else(invalid)?;
  match target{Target::Key=>match field.key{Key::Static(value)=>Ok(value),Key::Index(_)=>std::str::from_utf8(&numeric[..numeric_length]).map_err(|_|invalid())},Target::Value=>match field.payload{Payload::Text(value)=>Ok(value),_=>Err(invalid())}}
 }
 fn add(mut demand:RetirementDemand,bytes:usize)->Result<RetirementDemand,ValueError>{demand.copy_bytes=demand.copy_bytes.checked_add(bytes).ok_or_else(invalid)?;Ok(demand)}
 pub(crate) fn next_demands(&self,input:&WidgetInputValue,copy:usize)->Result<RetirementDemand,ValueError>{
  self.checked(input)?;
  Ok(match self.phase{
   Phase::Bind=>Self::fixed(2*size_of::<InputIdentity>()+2*size_of::<Frame>()+2*size_of::<OrderedMap<Value>>()+META),
   Phase::Field=>if field(input,self.current()?)?.is_none(){Self::fixed(2*size_of::<OrderedMap<Value>>()+2*size_of::<Value>()+META)}else{Self::fixed(2*size_of::<FlowTextCopyWork>()+self.numeric.len()+META)},
   Phase::Text=>{let text=self.text.as_ref().ok_or_else(invalid)?;if text.ready(){Self::add(text.take_demands()?,META)?}else{text.next_demands(Self::source(input,self.current()?,self.target,&self.numeric,self.numeric_length)?,copy)?}},
   Phase::TextClose=>{let text=self.text.as_ref().ok_or_else(invalid)?;if text.terminal_is_empty(){Self::fixed(size_of::<Option<FlowTextCopyWork>>()+META)}else{text.close_demands(copy)?}},
   Phase::Value=>match field(input,self.current()?)?.ok_or_else(invalid)?.payload{Payload::Text(_)=>Self::fixed(2*size_of::<FlowTextCopyWork>()+META),Payload::Child(_)=>Self::fixed(2*size_of::<Frame>()+2*size_of::<OrderedMap<Value>>()+META),_=>Self::fixed(2*size_of::<Value>()+META)},
   Phase::String=>Self::fixed(2*size_of::<String>()+2*size_of::<Value>()+META),
   Phase::Entry=>Self::fixed(2*size_of::<OriginalOwnedEntry<Value>>()+2*size_of::<OrderedMap<Value>>()+2*size_of::<String>()+2*size_of::<Value>()+META),
   Phase::EntryStep=>Self::add(self.entry.as_ref().ok_or_else(invalid)?.next_demands(copy.saturating_sub(META))?,META)?,
   Phase::EntryClose=>{let entry=self.entry.as_ref().ok_or_else(invalid)?;if entry.terminal_is_empty(){Self::fixed(size_of::<Option<OriginalOwnedEntry<Value>>>()+META)}else{entry.next_close_demands(copy)?}},
   Phase::Map=>Self::fixed(2*size_of::<OrderedMap<Value>>()+META),
   Phase::Ready=>Default::default(),Phase::Taken=>return Err(invalid()),
  })
 }
 fn paid(&mut self,d:RetirementDemand){self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
 fn absorb(&mut self,turn:RetainedWorkOutcome)->Result<(),ValueError>{self.work=turn.work;self.progress=turn.result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress());turn.result.map(|_|())}
 fn map_mut(&mut self)->Result<&mut Option<OrderedMap<Value>>,ValueError>{match self.depth{1=>Ok(&mut self.owners.map0),2=>Ok(&mut self.owners.map1),_=>Err(invalid())}}
 fn key_mut(&mut self)->Result<&mut Option<String>,ValueError>{match self.depth{1=>Ok(&mut self.owners.key0),2=>Ok(&mut self.owners.key1),_=>Err(invalid())}}
 pub(crate) fn advance(&mut self,input:&WidgetInputValue,g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;
  let result=self.advance_inner(input,g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 fn advance_inner(&mut self,input:&WidgetInputValue,g:RetainedCloneGrant)->Result<bool,ValueError>{
  self.checked(input)?;if self.ready(){return Ok(true)}let demand=self.next_demands(input,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,demand){return Ok(false)}
  match self.phase{
   Phase::Bind=>{self.paid(demand);self.identity=Some(InputIdentity::capture(input));if matches!(literal(input,View::Root)?,Literal::List(count)if count>1024){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"typed input collection exceeds its original schema bound"))}self.depth=1;self.owners.map0=Some(OrderedMap::new());self.phase=Phase::Field},
   Phase::Field=>{
    let Some(field)=field(input,self.current()?)? else{self.paid(demand);let value=Value::Dictionary(Dictionary{pairs:self.map_mut()?.take().ok_or_else(invalid)?});if self.depth==1{self.owners.output=Some(value);self.phase=Phase::Ready}else{self.depth-=1;self.owners.value=Some(value);self.phase=Phase::Entry}return Ok(self.ready())};
    self.paid(demand);self.numeric_length=0;if let Key::Index(mut index)=field.key{let mut position=self.numeric.len();loop{position-=1;self.numeric[position]=b'0'+(index%10)as u8;index/=10;if index==0{break}}let count=self.numeric.len()-position;self.numeric.copy_within(position..,0);self.numeric_length=count}
    self.text=Some(FlowTextCopyWork::new());self.target=Target::Key;self.phase=Phase::Text;
   },
   Phase::Text=>{
    let source=Self::source(input,self.current()?,self.target,&self.numeric,self.numeric_length)?;
    if !self.text.as_ref().ok_or_else(invalid)?.ready(){let turn=self.text.as_mut().unwrap().advance_original(source,g);self.absorb(turn)?;}else{
     let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,..g};let turn=self.text.as_mut().unwrap().take_original(child);self.work=turn.work;
     match turn.result{Ok(Some((value,p)))=>{self.progress=p;self.progress.copied_bytes+=META;if self.target==Target::Key{*self.key_mut()?=Some(value)}else{self.owners.string=Some(value)}self.text.as_mut().unwrap().begin_close();self.phase=Phase::TextClose},Ok(None)=>{},Err(error)=>{self.progress=error.retained_progress();return Err(error)}}
    }
   },
   Phase::TextClose=>if self.text.as_ref().ok_or_else(invalid)?.terminal_is_empty(){self.paid(demand);self.text=None;self.phase=if self.target==Target::Key{Phase::Value}else{Phase::String}}else{let turn=self.text.as_mut().unwrap().close_step(g);self.absorb(turn)?},
   Phase::Value=>{let payload=field(input,self.current()?)?.ok_or_else(invalid)?.payload;self.paid(demand);match payload{
    Payload::Text(_)=>{self.text=Some(FlowTextCopyWork::new());self.target=Target::Value;self.phase=Phase::Text},
    Payload::Number(value)=>{if !value.is_finite(){self.progress.copied_bytes=META;return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"typed input number must be finite"))}self.owners.value=Some(Value::Atom(Atom::Decimal(value)));self.phase=Phase::Entry},
    Payload::Boolean(value)=>{self.owners.value=Some(Value::Atom(Atom::Boolean(value)));self.phase=Phase::Entry},
    Payload::Child(view)=>{if self.depth!=1||self.owners.map1.is_some(){return Err(invalid())}self.frames[1]=Frame{view,member:0};self.depth=2;self.owners.map1=Some(OrderedMap::new());self.phase=Phase::Field},
   }},
   Phase::String=>{self.paid(demand);self.owners.value=Some(Value::Atom(Atom::String(self.owners.string.take().ok_or_else(invalid)?)));self.phase=Phase::Entry},
   Phase::Entry=>{self.paid(demand);let map=self.map_mut()?.take().ok_or_else(invalid)?;let key=self.key_mut()?.take().ok_or_else(invalid)?;let value=self.owners.value.take().ok_or_else(invalid)?;self.entry=Some(OriginalOwnedEntry::new(map,key,value));self.phase=Phase::EntryStep},
   Phase::EntryStep=>{
    let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,..g};let turn=self.entry.as_mut().ok_or_else(invalid)?.advance(child);self.work=turn.work;self.progress=self.entry.as_ref().unwrap().step_progress();
    match turn.result{Ok(Some(map))=>{self.progress.copied_bytes+=META;self.owners.map_output=Some(map);self.entry.as_mut().unwrap().begin_close();self.phase=Phase::EntryClose},Ok(None)=>{},Err(error)=>return Err(error)}
   },
   Phase::EntryClose=>if self.entry.as_ref().ok_or_else(invalid)?.terminal_is_empty(){self.paid(demand);self.entry=None;self.phase=Phase::Map}else{let turn=self.entry.as_mut().unwrap().close_step(g);self.absorb(turn)?},
   Phase::Map=>{self.paid(demand);let map=self.owners.map_output.take().ok_or_else(invalid)?;*self.map_mut()?=Some(map);self.frames[self.depth-1].member+=1;self.phase=Phase::Field},
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub(crate) fn take_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(Self::fixed(2*size_of::<Value>()+META))}
 pub(crate) fn take_original(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Value,RetainedCloneProgress)>>{
  self.progress=Default::default();self.work=0;let d=match self.take_demands(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}self.paid(d);let result=self.owners.output.take().ok_or_else(invalid).map(|value|{self.phase=Phase::Taken;Some((value,self.progress))});RetainedWorkOutcome{result,work:self.work}
 }
 pub(crate) fn take_into(&mut self,slot:&mut Option<Value>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if slot.is_some()||!self.source_binding().matches(cx){return Err(invalid().into())}let d=self.take_demands()?;if !cx.retained_work_can_enter(d){return Ok(false)}let turn=self.take_original(cx.retained_grant());cx.consume_fuel(turn.work);let Some((value,p))=turn.result? else{return Ok(false)};*slot=Some(value);cx.consume_retained(p)?;Ok(true)
 }
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(entry)=self.entry.as_mut(){entry.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(Self::fixed(size_of::<Option<FlowTextCopyWork>>()))}else{text.close_demands(copy)}}
  if let Some(entry)=self.entry.as_ref(){return if entry.terminal_is_empty(){Ok(Self::fixed(size_of::<Option<OriginalOwnedEntry<Value>>>()))}else{entry.next_close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(output,Value);owner!(value,Value);owner!(map_output,OrderedMap<Value>);owner!(string,String);owner!(key1,String);owner!(key0,String);owner!(map1,OrderedMap<Value>);owner!(map0,OrderedMap<Value>);
  Ok(if self.binding.is_some()||self.identity.is_some()||self.depth!=0{Self::fixed(size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<InputIdentity>>()+2*size_of::<Frame>()+META)}else{Default::default()})
 }
 pub(crate) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_inner(g).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}
 }
 fn close_inner(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}let d=self.next_close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(child)=self.$field.as_mut(){if child.terminal_is_empty(){self.paid(Self::fixed(size_of::<Option<$ty>>()));self.$field=None}else{let turn=child.close_step(g);self.absorb(turn)?}return Ok(RetainedCloneStep::Progress(self.progress))}}}
  child!(text,FlowTextCopyWork);child!(entry,OriginalOwnedEntry<Value>);
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(g);self.absorb(turn)?;return Ok(RetainedCloneStep::Progress(self.progress))}
  macro_rules! owner{($field:ident)=>{if self.owners.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.owners.$field,g);let turn=RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work};self.absorb(turn)?;return Ok(RetainedCloneStep::Progress(self.progress))}}}
  owner!(output);owner!(value);owner!(map_output);owner!(string);owner!(key1);owner!(key0);owner!(map1);owner!(map0);
  if self.binding.is_some()||self.identity.is_some()||self.depth!=0{self.paid(d);self.binding=None;self.identity=None;self.depth=0;self.frames=[Frame{view:View::Root,member:0};2];self.numeric_length=0;self.phase=Phase::Taken}
  Ok(RetainedCloneStep::Complete(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.identity.is_none()&&self.depth==0&&self.text.is_none()&&self.entry.is_none()&&self.retirement.is_empty()&&self.owners.output.is_none()&&self.owners.value.is_none()&&self.owners.map_output.is_none()&&self.owners.string.is_none()&&self.owners.key0.is_none()&&self.owners.key1.is_none()&&self.owners.map0.is_none()&&self.owners.map1.is_none()}
}
impl<'a> super::retained_scratch_driver::OriginalScratchWork<&'a WidgetInputValue> for TypedLiteralWork{
 fn source_binding(&self)->OriginalSourceBinding{self.source_binding()}
 fn origin(&self)->(u64,u64){(self.source_binding().operation().0,self.source_binding().generation().0)}
 fn ready(&self)->bool{self.ready()}
 fn advancing_retirement(&self)->bool{self.advancing_retirement()}
 fn next_demands(&self,input:&&'a WidgetInputValue,copy:usize)->Result<RetirementDemand,ValueError>{self.next_demands(input,copy)}
 fn advance(&mut self,input:&mut &'a WidgetInputValue,g:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.advance(input,g)}
 fn step_progress(&self)->RetainedCloneProgress{self.step_progress()}
}
impl Drop for TypedLiteralWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"typed literal must close every original map, text, AVL cursor and pending value");unsafe{ManuallyDrop::drop(&mut self.owners)}}}

#[cfg(test)]
#[path="🧪️.rs"]mod laws;
