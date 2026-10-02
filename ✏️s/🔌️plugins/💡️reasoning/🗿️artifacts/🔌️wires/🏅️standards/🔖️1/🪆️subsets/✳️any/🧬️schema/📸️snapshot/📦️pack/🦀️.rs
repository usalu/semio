//! 🔌️ Flat native Wires records own typed intrinsic nodes and the independent child handle.
use super::*;
use dsl::{DslField,schema::Number};
use std::collections::VecDeque;
#[derive(dsl::DslScalar)]enum Kind{Null,Boolean,Unsigned,Signed,Float,Text,Bytes,Array,Object}
struct Octets(Vec<u8>);
impl DslField for Octets{
 fn shape()->dsl::Shape{dsl::Shape::Bytes64}
 fn shape_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::Shape,String>{control.checkpoint()?;Ok(dsl::Shape::Bytes64)}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Bytes64(self.0.clone())}
 fn to_value_controlled(&self,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{control.copy_bytes(&self.0).map(dsl::FieldValue::Bytes64)}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{match value{dsl::FieldValue::Bytes64(bytes)=>Ok(Self(bytes.clone())),_=>Err("Wires native octets differ".into())}}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match value{dsl::FieldValue::Bytes64(bytes)=>control.copy_bytes(bytes).map(Self),_=>Err("Wires native octets differ".into())}}
}
#[derive(dsl::DslRecord)]struct Member{name:String,value:u64}
#[derive(dsl::DslRecord)]struct Node{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Option<i64>,float:Option<f64>,text:Option<String>,bytes:Option<Octets>,items:Vec<u64>,members:Vec<Member>}
#[derive(dsl::DslRecord)]#[dsl(extension="wires")]struct Document{fixture_root:u64,meta_root:u64,content:crate::WiresContentChild,nodes:Vec<Node>}
fn from_snapshot(snapshot:&WiresSnapshot)->Document{
 let mut pending=VecDeque::from([&snapshot.wires_fixture,&snapshot.meta]);let mut next=2u64;let mut nodes=Vec::new();while let Some(value)=pending.pop_front(){let mut node=Node{kind:Kind::Null,boolean:None,unsigned:None,signed:None,float:None,text:None,bytes:None,items:Vec::new(),members:Vec::new()};match value{dsl::DslValue::Null=>{},dsl::DslValue::Bool(value)=>{node.kind=Kind::Boolean;node.boolean=Some(*value)},dsl::DslValue::Number(Number::UInt(value))=>{node.kind=Kind::Unsigned;node.unsigned=Some(*value)},dsl::DslValue::Number(Number::Int(value))=>{node.kind=Kind::Signed;node.signed=Some(*value)},dsl::DslValue::Number(Number::Float(value))=>{node.kind=Kind::Float;node.float=Some(*value)},dsl::DslValue::String(text)=>{node.kind=Kind::Text;node.text=Some(text.clone())},dsl::DslValue::Bytes(bytes)=>{node.kind=Kind::Bytes;node.bytes=Some(Octets(bytes.clone()))},dsl::DslValue::Array(items)=>{node.kind=Kind::Array;for value in items{node.items.push(next);next=next.checked_add(1).expect("Wires native node index fits u64");pending.push_back(value)}},dsl::DslValue::Object(members)=>{node.kind=Kind::Object;for(name,value)in members{node.members.push(Member{name:name.clone(),value:next});next=next.checked_add(1).expect("Wires native node index fits u64");pending.push_back(value)}}}nodes.push(node);}
 Document{fixture_root:0,meta_root:1,content:snapshot.content.clone(),nodes}
}
struct Forest(Vec<Option<dsl::DslValue>>);impl Drop for Forest{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){<dsl::DslValue as dsl::FromValue>::retire_decoded(value)}}}
struct Items(Vec<dsl::DslValue>);impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){<dsl::DslValue as dsl::FromValue>::retire_decoded(value)}}}
struct Members(Vec<(String,dsl::DslValue)>);impl Drop for Members{fn drop(&mut self){for(_,value)in self.0.drain(..){<dsl::DslValue as dsl::FromValue>::retire_decoded(value)}}}
fn take(forest:&mut Forest,key:u64,parent:Option<usize>)->Result<dsl::DslValue,String>{let index=usize::try_from(key).map_err(|_|"Wires native node index exceeds platform domain")?;if index>=forest.0.len()||parent.is_some_and(|parent|index<=parent){return Err("Wires native topology differs".into())}forest.0[index].take().ok_or_else(||"Wires native node has repeated ownership".into())}
fn reconstruct(document:Document,control:&mut dsl::NativeDecodeControl<'_>)->Result<WiresSnapshot,String>{
 let Document{fixture_root,meta_root,content,nodes}=document;let mut units=nodes.len();control.begin_stage(nodes.len())?;for node in&nodes{control.step()?;units=units.checked_add(node.items.len()).and_then(|units|units.checked_add(node.members.len())).ok_or("Wires native work count overflow")?;}control.begin_stage(units)?;let mut forest=Forest(control.allocate_vec::<Option<dsl::DslValue>>(nodes.len())?);forest.0.resize_with(nodes.len(),||None);
 for(index,node)in nodes.into_iter().enumerate().rev(){control.step()?;let Node{kind,boolean,unsigned,signed,float,text,bytes,items,members}=node;let value=match(kind,boolean,unsigned,signed,float,text,bytes,items.is_empty(),members.is_empty()){
 (Kind::Null,None,None,None,None,None,None,true,true)=>dsl::DslValue::Null,
 (Kind::Boolean,Some(value),None,None,None,None,None,true,true)=>dsl::DslValue::Bool(value),
 (Kind::Unsigned,None,Some(value),None,None,None,None,true,true)=>dsl::DslValue::Number(Number::UInt(value)),
 (Kind::Signed,None,None,Some(value),None,None,None,true,true)=>dsl::DslValue::Number(Number::Int(value)),
 (Kind::Float,None,None,None,Some(value),None,None,true,true)=>dsl::DslValue::Number(Number::Float(value)),
 (Kind::Text,None,None,None,None,Some(value),None,true,true)=>dsl::DslValue::String(value),
 (Kind::Bytes,None,None,None,None,None,Some(bytes),true,true)=>dsl::DslValue::Bytes(bytes.0),
 (Kind::Array,None,None,None,None,None,None,_,true)=>{let mut result=Items(control.allocate_vec::<dsl::DslValue>(items.len())?);for key in items{control.step()?;result.0.push(take(&mut forest,key,Some(index))?);}dsl::DslValue::Array(std::mem::take(&mut result.0))},
 (Kind::Object,None,None,None,None,None,None,true,_)=>{let mut result=Members(control.allocate_vec::<(String,dsl::DslValue)>(members.len())?);for member in members{control.step()?;result.0.push((member.name,take(&mut forest,member.value,Some(index))?));}dsl::DslValue::Object(std::mem::take(&mut result.0))},
 _=>return Err("Wires native node contains unrelated variant fields".into())};forest.0[index]=Some(value);
 }
 let mut roots=Items(control.allocate_vec::<dsl::DslValue>(2)?);roots.0.push(take(&mut forest,fixture_root,None)?);roots.0.push(take(&mut forest,meta_root,None)?);if forest.0.iter().any(Option::is_some){return Err("Wires native node is unowned".into())}control.checkpoint()?;
 let meta=roots.0.pop().ok_or("Wires meta root is missing")?;let wires_fixture=roots.0.pop().ok_or("Wires fixture root is missing")?;Ok(WiresSnapshot{wires_fixture,meta,content})
}
fn ordinary(document:Document)->Result<WiresSnapshot,String>{let mut callback=|_:protocol::native_decoding::NativeDecodeProgress|true;let mut control=dsl::NativeDecodeControl::new(usize::MAX,&mut callback);reconstruct(document,&mut control)}
pub(crate) fn record_spec()->dsl::RecordSpec{Document::__dsl_spec()}
pub(crate) fn record_spec_producer()->dsl::RecordSpecProducer{Document::__dsl_spec_producer()}
pub(crate) fn record(snapshot:&WiresSnapshot)->dsl::RecordValue{from_snapshot(snapshot).__dsl_to_record()}
pub(crate) fn record_controlled(snapshot:&WiresSnapshot,control:&mut dsl::NativeEncodeControl<'_>,maximum_rows:usize)->Result<dsl::RecordValue,dsl::TextError>{
 fn build(snapshot:&WiresSnapshot,control:&mut dsl::NativeEncodeControl<'_>,maximum_rows:usize)->Result<dsl::RecordValue,String>{
  use dsl::{FieldValue as V,native_encoding::{EncodedRecord,retire_field}};
  fn reserve(frontier:&mut Vec<&dsl::DslValue>,count:usize,control:&mut dsl::NativeEncodeControl<'_>)->Result<(),String>{control.charge(count.checked_mul(std::mem::size_of::<&dsl::DslValue>()).ok_or("Wires native frontier overflow")?)?;frontier.try_reserve_exact(count).map_err(|_|"Wires native frontier allocation failed".into())}
  fn fields(count:usize,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::__rt::DecodedFieldOwner<Vec<V>>,String>{Ok(dsl::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?,|values:Vec<V>|{for value in values{retire_field(value)}}))}
  control.begin_stage(0)?;if maximum_rows<8{return Err("Wires native row limit exceeded".into())}let mut frontier=control.allocate_vec::<&dsl::DslValue>(2)?;frontier.extend([&snapshot.wires_fixture,&snapshot.meta]);let mut position=0;let mut rows=8usize;
  while position<frontier.len(){control.step()?;let value=frontier[position];position+=1;
   match value{
    dsl::DslValue::Array(items)=>{rows=items.len().checked_mul(3).and_then(|count|rows.checked_add(count)).ok_or("Wires native row count overflow")?;if rows>maximum_rows{return Err("Wires native row limit exceeded".into())}reserve(&mut frontier,items.len(),control)?;for item in items{control.step()?;frontier.push(item)}},
    dsl::DslValue::Object(members)=>{rows=members.len().checked_mul(3).and_then(|count|rows.checked_add(count)).ok_or("Wires native row count overflow")?;if rows>maximum_rows{return Err("Wires native row limit exceeded".into())}reserve(&mut frontier,members.len(),control)?;for(_,value)in members{control.step()?;frontier.push(value)}},
    dsl::DslValue::Bytes(bytes)=>{rows=rows.checked_add(bytes.len()).ok_or("Wires native row count overflow")?;},
    dsl::DslValue::Null|dsl::DslValue::Bool(_)|dsl::DslValue::Number(_)|dsl::DslValue::String(_)=>{},
   }if rows>maximum_rows{return Err("Wires native row limit exceeded".into())}
  }
  control.begin_stage(frontier.len())?;let mut nodes=fields(frontier.len(),control)?;let mut next=2u64;
  for value in frontier{let mut node=EncodedRecord::new(9,control)?;for key in[1,2,3,4,5,6]{node.insert(key,V::Absent)}let mut items=fields(0,control)?;let mut members=fields(0,control)?;
   let kind=match value{
    dsl::DslValue::Null=>Kind::Null,
    dsl::DslValue::Bool(value)=>{node.insert(1,V::Bool(*value));Kind::Boolean},
    dsl::DslValue::Number(Number::UInt(value))=>{node.insert(2,V::UInt(*value));Kind::Unsigned},
    dsl::DslValue::Number(Number::Int(value))=>{node.insert(3,V::Int(*value));Kind::Signed},
    dsl::DslValue::Number(Number::Float(value))=>{node.insert(4,V::Float(*value));Kind::Float},
    dsl::DslValue::String(value)=>{node.insert(5,V::Text(control.copy_text(value)?));Kind::Text},
    dsl::DslValue::Bytes(value)=>{node.insert(6,V::Bytes64(control.copy_bytes(value)?));Kind::Bytes},
    dsl::DslValue::Array(values)=>{items=fields(values.len(),control)?;control.scoped_stage(|control|{control.begin_stage(values.len())?;for _ in values{items.as_mut().push(V::UInt(next));next=next.checked_add(1).ok_or("Wires native node index overflow")?;control.step()?;}Ok::<(),String>(())})?;Kind::Array},
    dsl::DslValue::Object(values)=>{members=fields(values.len(),control)?;control.scoped_stage(|control|{control.begin_stage(values.len())?;for(name,_)in values{let mut member=EncodedRecord::new(2,control)?;member.insert(0,V::Text(control.copy_text(name)?));member.insert(1,V::UInt(next));next=next.checked_add(1).ok_or("Wires native node index overflow")?;members.as_mut().push(V::Record(member.take()));control.step()?;}Ok::<(),String>(())})?;Kind::Object},
   };
   node.insert(0,control.scoped_stage(|control|{control.begin_stage(0)?;kind.to_value_controlled(control)})?);node.insert(7,V::List(items.take()));node.insert(8,V::List(members.take()));nodes.as_mut().push(V::Record(node.take()));control.step()?;
  }
  control.begin_stage(4)?;let mut document=EncodedRecord::new(4,control)?;document.insert(0,V::UInt(0));control.step()?;document.insert(1,V::UInt(1));control.step()?;document.insert(2,control.scoped_stage(|control|{control.begin_stage(0)?;snapshot.content.to_value_controlled(control)})?);control.step()?;document.insert(3,V::List(nodes.take()));control.step()?;Ok(document.take())
 }
 build(snapshot,control,maximum_rows).map_err(dsl::__rt::field_error)
}
pub(crate) fn reconstruct_record(source:&dsl::RecordValue)->Result<WiresSnapshot,store::TextError>{ordinary(Document::__dsl_from_record(source)?).map_err(dsl::__rt::field_error)}
pub(super) fn reconstruct_record_controlled(source:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>,maximum_rows:usize)->Result<WiresSnapshot,String>{
 fn list(source:&dsl::RecordValue,key:u16)->Result<&[dsl::FieldValue],String>{match source.fields.get(&key){Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("Wires native relationship list differs".into())}}
 let nodes=list(source,3)?;let mut rows=nodes.len().checked_mul(2).and_then(|count|count.checked_add(4)).ok_or("Wires native row count overflow")?;if rows>maximum_rows{return Err("Wires native row limit exceeded".into())}
 control.scoped_stage(|control|{control.begin_stage(nodes.len())?;for node in nodes{control.step()?;let node=match node{dsl::FieldValue::Record(node)=>node,_=>return Err("Wires native node record differs".into())};let bytes=match node.fields.get(&6){Some(dsl::FieldValue::Absent)|None=>0,Some(dsl::FieldValue::Bytes64(bytes))=>bytes.len(),_=>return Err("Wires native octet field differs".into())};rows=rows.checked_add(bytes).and_then(|count|list(node,7).ok().and_then(|items|count.checked_add(items.len()))).and_then(|count|list(node,8).ok().and_then(|members|count.checked_add(members.len()))).ok_or("Wires native relationship count overflow or field differs")?;if rows>maximum_rows{return Err("Wires native row limit exceeded".into())}}Ok::<(),String>(())})?;
 reconstruct(Document::__dsl_from_record_controlled(source,control).map_err(|error|error.message)?,control)
}
