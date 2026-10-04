//! 🔌️ Flat native Wires records own typed intrinsic nodes and the independent child handle.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_diagnostic::{TextError,TextSpan};
use super::*;
use semio_framework_dsl_record::DslField;
use semio_framework_value::Number;
use std::collections::VecDeque;
#[path="🧮️census/🦀️.rs"] mod semantic_census;
#[derive(semio_framework_dsl_record_derive::DslScalar)]enum Kind{Null,Boolean,Unsigned,Signed,Float,Text,Bytes,Array,Object}
struct Octets(Vec<u8>);
impl DslField for Octets{
 fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Bytes64}
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Bytes64)}
 fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Bytes64(self.0.clone())}
 fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.copy_bytes(&self.0).map(semio_framework_dsl_record::FieldValue::Bytes64)}
 fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Bytes64(bytes)=>Ok(Self(bytes.clone())),_=>Err("Wires native octets differ".into())}}
 fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{semio_framework_dsl_record::FieldValue::Bytes64(bytes)=>control.copy_bytes(bytes).map(Self),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native octets differ"))}}
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]struct Member{name:String,value:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]struct Node{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Option<i64>,float:Option<f64>,text:Option<String>,bytes:Option<Octets>,items:Vec<u64>,members:Vec<Member>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]#[dsl(extension="wires")]struct Document{fixture_root:u64,meta_root:u64,content:crate::WiresContentChild,nodes:Vec<Node>}
fn from_snapshot(snapshot:&WiresSnapshot)->Document{
 let mut pending=VecDeque::from([&snapshot.wires_fixture,&snapshot.meta]);let mut next=2u64;let mut nodes=Vec::new();while let Some(value)=pending.pop_front(){let mut node=Node{kind:Kind::Null,boolean:None,unsigned:None,signed:None,float:None,text:None,bytes:None,items:Vec::new(),members:Vec::new()};match value{semio_framework_value::DslValue::Null=>{},semio_framework_value::DslValue::Bool(value)=>{node.kind=Kind::Boolean;node.boolean=Some(*value)},semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value))=>{node.kind=Kind::Unsigned;node.unsigned=Some(*value)},semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(value))=>{node.kind=Kind::Signed;node.signed=Some(*value)},semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(value))=>{node.kind=Kind::Float;node.float=Some(*value)},semio_framework_value::DslValue::String(text)=>{node.kind=Kind::Text;node.text=Some(text.clone())},semio_framework_value::DslValue::Bytes(bytes)=>{node.kind=Kind::Bytes;node.bytes=Some(Octets(bytes.clone()))},semio_framework_value::DslValue::Array(items)=>{node.kind=Kind::Array;for value in items{node.items.push(next);next=next.checked_add(1).expect("Wires native node index fits u64");pending.push_back(value)}},semio_framework_value::DslValue::Object(members)=>{node.kind=Kind::Object;for(name,value)in members{node.members.push(Member{name:name.clone(),value:next});next=next.checked_add(1).expect("Wires native node index fits u64");pending.push_back(value)}}}nodes.push(node);}
 Document{fixture_root:0,meta_root:1,content:snapshot.content.clone(),nodes}
}
struct Forest(Vec<Option<semio_framework_value::DslValue>>);impl Drop for Forest{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value)}}}
struct Items(Vec<semio_framework_value::DslValue>);impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value)}}}
struct Members(Vec<(String,semio_framework_value::DslValue)>);impl Drop for Members{fn drop(&mut self){for(_,value)in self.0.drain(..){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value)}}}
fn take(forest:&mut Forest,key:u64,parent:Option<usize>)->Result<semio_framework_value::DslValue,ValueError>{let index=usize::try_from(key).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Wires native node index exceeds platform domain"))?;if index>=forest.0.len()||parent.is_some_and(|parent|index<=parent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native topology differs"))}forest.0[index].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires native node has repeated ownership"))}
fn reconstruct(document:Document,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<WiresSnapshot,ValueError>{
 let Document{fixture_root,meta_root,content,nodes}=document;let mut units=nodes.len();control.begin_stage(nodes.len())?;for node in&nodes{control.step()?;units=units.checked_add(node.items.len()).and_then(|units|units.checked_add(node.members.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native work count overflow"))?;}control.begin_stage(units)?;let mut forest=Forest(control.allocate_vec::<Option<semio_framework_value::DslValue>>(nodes.len())?);forest.0.resize_with(nodes.len(),||None);
 for(index,node)in nodes.into_iter().enumerate().rev(){control.step()?;let Node{kind,boolean,unsigned,signed,float,text,bytes,items,members}=node;let value=match(kind,boolean,unsigned,signed,float,text,bytes,items.is_empty(),members.is_empty()){
 (Kind::Null,None,None,None,None,None,None,true,true)=>semio_framework_value::DslValue::Null,
 (Kind::Boolean,Some(value),None,None,None,None,None,true,true)=>semio_framework_value::DslValue::Bool(value),
 (Kind::Unsigned,None,Some(value),None,None,None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value)),
 (Kind::Signed,None,None,Some(value),None,None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(value)),
 (Kind::Float,None,None,None,Some(value),None,None,true,true)=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(value)),
 (Kind::Text,None,None,None,None,Some(value),None,true,true)=>semio_framework_value::DslValue::String(value),
 (Kind::Bytes,None,None,None,None,None,Some(bytes),true,true)=>semio_framework_value::DslValue::Bytes(bytes.0),
 (Kind::Array,None,None,None,None,None,None,_,true)=>{let mut result=Items(control.allocate_vec::<semio_framework_value::DslValue>(items.len())?);for key in items{control.step()?;result.0.push(take(&mut forest,key,Some(index))?);}semio_framework_value::DslValue::Array(std::mem::take(&mut result.0))},
 (Kind::Object,None,None,None,None,None,None,true,_)=>{let mut result=Members(control.allocate_vec::<(String,semio_framework_value::DslValue)>(members.len())?);for member in members{control.step()?;result.0.push((member.name,take(&mut forest,member.value,Some(index))?));}semio_framework_value::DslValue::Object(std::mem::take(&mut result.0))},
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native node contains unrelated variant fields"))};forest.0[index]=Some(value);
 }
 let mut roots=Items(control.allocate_vec::<semio_framework_value::DslValue>(2)?);roots.0.push(take(&mut forest,fixture_root,None)?);roots.0.push(take(&mut forest,meta_root,None)?);if forest.0.iter().any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native node is unowned"))}control.checkpoint()?;
 let meta=roots.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires meta root is missing"))?;let wires_fixture=roots.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires fixture root is missing"))?;Ok(WiresSnapshot{wires_fixture,meta,content})
}
fn ordinary(document:Document)->Result<WiresSnapshot,ValueError>{let mut callback=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut control=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut callback);reconstruct(document,&mut control)}
pub(crate) fn record_spec()->semio_framework_dsl_record::RecordSpec{Document::__dsl_spec()}
pub(crate) fn record_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{Document::__dsl_spec_producer()}
pub(crate) fn record(snapshot:&WiresSnapshot)->semio_framework_dsl_record::RecordValue{from_snapshot(snapshot).__dsl_to_record()}
pub(crate) fn record_controlled(snapshot:&WiresSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>,maximum_rows:usize)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 record_controlled_semantic(snapshot,control,maximum_rows,usize::MAX)
}
pub(super) fn record_controlled_semantic(snapshot:&WiresSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>,maximum_rows:usize,maximum_bytes:usize)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
 fn build(snapshot:&WiresSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>,maximum_rows:usize,maximum_bytes:usize)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
  use semio_framework_dsl_record::FieldValue as V;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use semio_framework_dsl_record::native_encoding::retire_field;
  fn reserve(frontier:&mut Vec<&semio_framework_value::DslValue>,count:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{control.charge(count.checked_mul(std::mem::size_of::<&semio_framework_value::DslValue>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Wires native frontier overflow"))?)?;frontier.try_reserve_exact(count).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Wires native frontier allocation failed"))}
  fn fields(count:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::__rt::DecodedFieldOwner<Vec<V>>,ValueError>{Ok(semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?,|values:Vec<V>|{for value in values{retire_field(value)}}))}
  let mut census=semantic_census::Census::new(&snapshot.content,maximum_rows,maximum_bytes)?;control.begin_stage(0)?;if maximum_rows<8{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}let mut frontier=control.allocate_vec::<&semio_framework_value::DslValue>(2)?;frontier.extend([&snapshot.wires_fixture,&snapshot.meta]);let mut position=0;let mut rows=8usize;
  while position<frontier.len(){control.step()?;let value=frontier[position];position+=1;census.value(value)?;
   match value{
    semio_framework_value::DslValue::Array(items)=>{rows=items.len().checked_mul(3).and_then(|count|rows.checked_add(count)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row count overflow"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}reserve(&mut frontier,items.len(),control)?;for item in items{control.step()?;frontier.push(item)}},
    semio_framework_value::DslValue::Object(members)=>{rows=members.len().checked_mul(3).and_then(|count|rows.checked_add(count)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row count overflow"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}reserve(&mut frontier,members.len(),control)?;for(_,value)in members{control.step()?;frontier.push(value)}},
    semio_framework_value::DslValue::Bytes(bytes)=>{rows=rows.checked_add(bytes.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row count overflow"))?;},
    semio_framework_value::DslValue::Null|semio_framework_value::DslValue::Bool(_)|semio_framework_value::DslValue::Number(_)|semio_framework_value::DslValue::String(_)=>{},
   }if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}
  }
  control.begin_stage(frontier.len())?;let mut nodes=fields(frontier.len(),control)?;let mut next=2u64;
  for value in frontier{let mut node=semio_framework_dsl_record::native_encoding::EncodedRecord::new(9,control)?;for key in[1,2,3,4,5,6]{node.insert(key,semio_framework_dsl_record::FieldValue::Absent)?}let mut items=fields(0,control)?;let mut members=fields(0,control)?;
   let kind=match value{
    semio_framework_value::DslValue::Null=>Kind::Null,
    semio_framework_value::DslValue::Bool(value)=>{node.insert(1,semio_framework_dsl_record::FieldValue::Bool(*value))?;Kind::Boolean},
    semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value))=>{node.insert(2,semio_framework_dsl_record::FieldValue::UInt(*value))?;Kind::Unsigned},
    semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(value))=>{node.insert(3,semio_framework_dsl_record::FieldValue::Int(*value))?;Kind::Signed},
    semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(value))=>{node.insert(4,semio_framework_dsl_record::FieldValue::Float(*value))?;Kind::Float},
    semio_framework_value::DslValue::String(value)=>{node.insert(5,semio_framework_dsl_record::FieldValue::Text(control.copy_text(value)?))?;Kind::Text},
    semio_framework_value::DslValue::Bytes(value)=>{node.insert(6,semio_framework_dsl_record::FieldValue::Bytes64(control.copy_bytes(value)?))?;Kind::Bytes},
    semio_framework_value::DslValue::Array(values)=>{items=fields(values.len(),control)?;control.scoped_stage(|control|{control.begin_stage(values.len())?;for _ in values{items.as_mut().push(semio_framework_dsl_record::FieldValue::UInt(next));next=next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native node index overflow"))?;control.step()?;}Ok::<(),ValueError>(())})?;Kind::Array},
    semio_framework_value::DslValue::Object(values)=>{members=fields(values.len(),control)?;control.scoped_stage(|control|{control.begin_stage(values.len())?;for(name,_)in values{let mut member=semio_framework_dsl_record::native_encoding::EncodedRecord::new(2,control)?;member.insert(0,semio_framework_dsl_record::FieldValue::Text(control.copy_text(name)?))?;member.insert(1,semio_framework_dsl_record::FieldValue::UInt(next))?;next=next.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native node index overflow"))?;members.as_mut().push(semio_framework_dsl_record::FieldValue::Record(member.take()));control.step()?;}Ok::<(),ValueError>(())})?;Kind::Object},
   };
   node.insert(0,control.scoped_stage(|control|{control.begin_stage(0)?;kind.to_value_controlled(control)})?)?;node.insert(7,semio_framework_dsl_record::FieldValue::List(items.take()))?;node.insert(8,semio_framework_dsl_record::FieldValue::List(members.take()))?;nodes.as_mut().push(semio_framework_dsl_record::FieldValue::Record(node.take()));control.step()?;
  }
  control.begin_stage(4)?;let mut document=semio_framework_dsl_record::native_encoding::EncodedRecord::new(4,control)?;document.insert(0,semio_framework_dsl_record::FieldValue::UInt(0))?;control.step()?;document.insert(1,semio_framework_dsl_record::FieldValue::UInt(1))?;control.step()?;document.insert(2,control.scoped_stage(|control|{control.begin_stage(0)?;snapshot.content.to_value_controlled(control)})?)?;control.step()?;document.insert(3,semio_framework_dsl_record::FieldValue::List(nodes.take()))?;control.step()?;Ok(document.take())
 }
 build(snapshot,control,maximum_rows,maximum_bytes)
}
pub(crate) fn reconstruct_record(source:&semio_framework_dsl_record::RecordValue)->Result<WiresSnapshot,TextError>{ordinary(Document::__dsl_from_record(source)?).map_err(|error|TextError::from_value_error(error,TextSpan::at(1,1)))}
pub(super) fn reconstruct_record_controlled(source:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize,maximum_bytes:usize)->Result<WiresSnapshot,ValueError>{
 fn list(source:&semio_framework_dsl_record::RecordValue,key:u16)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match source.fields.get(&key){Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native relationship list differs"))}}
 let nodes=list(source,3)?;let mut rows=nodes.len().checked_mul(2).and_then(|count|count.checked_add(4)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row count overflow"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}
 control.scoped_stage(|control|{control.begin_stage(nodes.len())?;for node in nodes{control.step()?;let node=match node{semio_framework_dsl_record::FieldValue::Record(node)=>node,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native node record differs"))};let bytes=match node.fields.get(&6){Some(semio_framework_dsl_record::FieldValue::Absent)|None=>0,Some(semio_framework_dsl_record::FieldValue::Bytes64(bytes))=>bytes.len(),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Wires native octet field differs"))};rows=rows.checked_add(bytes).and_then(|count|list(node,7).ok().and_then(|items|count.checked_add(items.len()))).and_then(|count|list(node,8).ok().and_then(|members|count.checked_add(members.len()))).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires native relationship count overflow or field differs"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires native row limit exceeded"))}}Ok::<(),ValueError>(())})?;
 let document=Document::__dsl_from_record_controlled(source,control)?;let mut census=semantic_census::Census::new(&document.content,maximum_rows,maximum_bytes)?;control.scoped_stage(|control|{control.begin_stage(0)?;for node in &document.nodes{control.step()?;census.node(node,control)?}control.checkpoint()})?;reconstruct(document,control)
}
