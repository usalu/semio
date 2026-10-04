//! 🧮️ Exact authored SQL cell and row admission for both native Wires directions.
use super::*;
pub(super) struct Census{rows:usize,bytes:usize,maximum_rows:usize,maximum_bytes:usize}
impl Census{
 pub(super) fn new(content:&crate::WiresContentChild,maximum_rows:usize,maximum_bytes:usize)->Result<Self,ValueError>{
  let mut census=Self{rows:4,bytes:48,maximum_rows,maximum_bytes};
  for text in[&content.child_id,&content.target.artifact_id,&content.target.dialect.artifact_kind,&content.target.dialect.standard,&content.target.dialect.subset]{census.bytes(text.len())?}
  census.check()?;Ok(census)
 }
 fn bytes(&mut self,bytes:usize)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Wires semantic cell byte count overflow"))?;self.check()}
 fn rows(&mut self,rows:usize)->Result<(),ValueError>{self.rows=self.rows.checked_add(rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Wires semantic row count overflow"))?;self.check()}
 fn check(&self)->Result<(),ValueError>{if self.rows>self.maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Wires semantic row limit exceeded"))}if self.bytes>self.maximum_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Wires semantic cell byte limit exceeded"))}Ok(())}
 fn scalar(&mut self,kind:&str,bytes:usize)->Result<(),ValueError>{self.rows(2)?;self.bytes(16)?;self.bytes(kind.len())?;self.bytes(bytes)}
 fn edges(&mut self,count:usize)->Result<(),ValueError>{self.rows(count)?;self.bytes(count.checked_mul(32).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Wires semantic relationship bytes overflow"))?)}
 fn float_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
 pub(super) fn value(&mut self,value:&semio_framework_value::DslValue)->Result<(),ValueError>{
  use semio_framework_value::DslValue as V;
  match value{
   V::Null=>self.scalar("null",0),
   V::Bool(_)=>self.scalar("boolean",8),
   V::Number(Number::UInt(_))=>self.scalar("unsigned",16),
   V::Number(Number::Int(_))=>self.scalar("signed",8),
   V::Number(Number::Float(value))=>self.scalar("float",Self::float_bytes(*value)),
   V::String(text)=>self.scalar("text",text.len()),
   V::Bytes(bytes)=>{self.scalar("bytes",0)?;self.edges(bytes.len())},
   V::Array(items)=>{self.scalar("array",0)?;self.edges(items.len())},
   V::Object(members)=>{self.scalar("object",0)?;self.edges(members.len())?;for(name,_)in members{self.bytes(name.len())?}Ok(())}
  }
 }
 pub(super) fn node(&mut self,node:&Node,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
  match &node.kind{
   Kind::Null=>self.scalar("null",0),
   Kind::Boolean=>self.scalar("boolean",8),
   Kind::Unsigned=>self.scalar("unsigned",16),
   Kind::Signed=>self.scalar("signed",8),
   Kind::Float=>self.scalar("float",Self::float_bytes(node.float.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires native float is missing"))?)),
   Kind::Text=>self.scalar("text",node.text.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires native text is missing"))?.len()),
   Kind::Bytes=>{self.scalar("bytes",0)?;self.edges(node.bytes.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Wires native octets are missing"))?.0.len())},
   Kind::Array=>{self.scalar("array",0)?;self.edges(node.items.len())},
   Kind::Object=>{self.scalar("object",0)?;self.edges(node.members.len())?;for member in &node.members{control.step()?;self.bytes(member.name.len())?}Ok(())}
  }
 }
}
