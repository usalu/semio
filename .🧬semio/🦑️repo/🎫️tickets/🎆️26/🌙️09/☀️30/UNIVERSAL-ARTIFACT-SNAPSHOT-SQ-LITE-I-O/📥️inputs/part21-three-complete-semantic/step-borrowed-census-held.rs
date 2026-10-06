//! 🧮️ Domain-owned Part21 cells are admitted before flat typed ownership.
const TABLES:[(&str,usize);12]=[("step_aggregate_element",4),("step_argument",5),("step_author",4),("step_complex_type",4),("step_description",4),("step_document",3),("step_entity",5),("step_header",7),("step_organization",4),("step_schema_identifier",4),("step_typed_value",4),("step_value",10)];
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Part21 native role or exclusive forest differs from its authored fields")}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic extent overflow"))}
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{
 let bytes=StepSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).try_fold(0usize,|total,sql|sum(total,sql.len()))?;
 let bytes=TABLES.iter().try_fold(bytes,|total,(name,_)|sum(total,name.len()))?;
 if bytes.max(StepSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 authored schema exceeds caller bytes"))}
 if TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 authored relational extent exceeds caller limits"))}Ok(())
}
fn closed(value:&R,last:u16)->Result<()>{if value.fields.keys().any(|id|*id>last){return Err(invalid())}Ok(())}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(invalid)}
fn record(value:&F,last:u16)->Result<&R>{let F::Record(value)=value else{return Err(invalid())};closed(value,last)?;Ok(value)}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid())};Ok(value)}
fn uint(value:&F)->Result<u64>{let F::UInt(value)=value else{return Err(invalid())};Ok(*value)}
fn optional(value:&R,id:u16)->Option<&F>{match value.get(id){None|Some(F::Absent)=>None,value=>value}}
fn text<'a>(value:&'a F,native:&mut NativeDecodeControl<'_>)->Result<&'a str>{let F::Text(value)=value else{return Err(invalid())};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value)}
fn optional_text<'a>(value:&'a R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>>{optional(value,id).map(|value|text(value,native)).transpose()}
fn optional_int(value:&R,id:u16)->Result<Option<i64>>{optional(value,id).map(|value|{let F::Int(value)=value else{return Err(invalid())};Ok(*value)}).transpose()}
fn optional_uint(value:&R,id:u16)->Result<Option<u64>>{optional(value,id).map(uint).transpose()}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Part21 semantic row extent overflow"))?;let total=sum(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 semantic rows exceed caller limit"))}if total>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Part21 semantic values exceed caller limit"))}native.step()?;self.rows=rows;self.bytes=total;Ok(())}
 fn edges(&mut self,values:&[F],owners:&mut[u8],parent:Option<usize>,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{for value in values{let index=usize::try_from(uint(value)?).map_err(|_|invalid())?;if parent.is_some_and(|parent|index<=parent){return Err(invalid())}let Some(owner)=owners.get_mut(index)else{return Err(invalid())};if *owner!=0{return Err(invalid())}*owner=1;self.row(bytes,native)?;}Ok(())}
}
fn identities(values:&[F],last:u16,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<Vec<u64>>{
 if values.len()>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Part21 identity count exceeds caller rows"))}
 let mut identities=native.allocate_vec(values.len())?;for value in values{identities.push(uint(field(record(value,last)?,0)?)?);native.step()?;}
 super::sort_paid(&mut identities,|left,right|{native.step()?;Ok(left.cmp(right))})?;for pair in identities.windows(2){if pair[0]==pair[1]{return Err(invalid())}native.step()?;}Ok(identities)
}
fn nodes(values:&[F],identities:&[u64],owners:&mut[u8],census:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 for(index,value)in values.iter().enumerate(){let value=record(value,6)?;let kind=text(field(value,0)?,native)?;let integer=optional_int(value,1)?;let real=optional(value,2).map(|value|{let F::Float(value)=value else{return Err(invalid())};Ok(*value)}).transpose()?;let literal=optional_text(value,3,native)?;let reference=optional_uint(value,4)?;let name=optional_text(value,5,native)?;let children=list(field(value,6)?)?;
  let payloads=usize::from(integer.is_some())+usize::from(real.is_some())+usize::from(literal.is_some())+usize::from(reference.is_some())+usize::from(name.is_some());
  let valid=match kind{"unset"|"derived"=>payloads==0&&children.is_empty(),"integer"=>integer.is_some()&&payloads==1&&children.is_empty(),"real"=>real.is_some()&&payloads==1&&children.is_empty(),"string"|"enum"=>literal.is_some()&&payloads==1&&children.is_empty(),"reference"=>reference.is_some()&&payloads==1&&children.is_empty(),"aggregate"=>payloads==0,"typed"=>name.is_some()&&payloads==1&&children.len()==1,_=>false};if !valid{return Err(invalid())}
  let sql_kind=if kind=="typed"{"typedValue"}else{kind};let mut bytes=sum(8,sql_kind.len())?;
  if integer.is_some(){bytes=sum(bytes,8)?;}if let Some(value)=real{bytes=sum(bytes,if value.is_nan(){11}else if value.is_infinite(){32}else{22})?;}if let Some(value)=literal{bytes=sum(bytes,value.len())?;}
  if let Some(value)=reference{let resolved=identities.binary_search(&value).is_ok();if !resolved{return Err(invalid())}bytes=sum(bytes,UnsignedWord::new(value).text().len())?;if resolved{bytes=sum(bytes,8)?;}}
  
  census.row(bytes,native)?;let edge_bytes=if kind=="typed"{sum(24,name.unwrap().len())?}else{32};census.edges(children,owners,Some(index),edge_bytes,native)?;
 }Ok(())
}
/// 🫳️ Admits literal header strings, paid identity ordering and exclusive value ownership.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 extent(limits)?;native.scoped_stage(|native|{
  native.begin_stage(0)?;closed(value,12)?;let entities=list(field(value,11)?)?;let values=list(field(value,12)?)?;if values.len()>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"STEP node count exceeds caller rows"))}
  let mut census=Census{limits,rows:0,bytes:0};census.row(sum(16,text(field(value,0)?,native)?.len())?,native)?;let mut bytes=8usize;for id in[2,3,4,7,8,9]{bytes=sum(bytes,text(field(value,id)?,native)?.len())?;}census.row(bytes,native)?;for id in[1,5,6,10]{for value in list(field(value,id)?)?{census.row(sum(24,text(value,native)?.len())?,native)?;}}
  let identities=identities(entities,3,limits,native)?;let mut owners=native.allocate_vec::<u8>(values.len())?;owners.resize(values.len(),0);
  for value in entities{let value=record(value,3)?;let word=UnsignedWord::new(uint(field(value,0)?)?);census.row(sum(sum(24,word.text().len())?,text(field(value,1)?,native)?.len())?,native)?;census.edges(list(field(value,2)?)?,&mut owners,None,32,native)?;for value in list(field(value,3)?)?{let value=record(value,1)?;census.row(sum(24,text(field(value,0)?,native)?.len())?,native)?;census.edges(list(field(value,1)?)?,&mut owners,None,32,native)?;}}
  nodes(values,&identities,&mut owners,&mut census,native)?;for owner in owners{if owner!=1{return Err(invalid())}native.step()?;}native.checkpoint()
 })
}
