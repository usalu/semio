//! 📏️ Borrowed GIS native syntax admits complete authored SQL cells before typed construction.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use store::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:usize,cells:&[usize],native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<()>{
  native.step()?;let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"GIS semantic rows overflow"))?;
  let bytes=cells.iter().try_fold(self.bytes.checked_add(8).ok_or_else(||invalid("GIS semantic cells overflow"))?,|sum,size|sum.checked_add(*size).ok_or_else(||invalid("GIS semantic cells overflow")))?;
  if super::super::sqlite::WIDTHS[table]>self.limits.max_columns||rows>self.limits.max_rows{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"GIS semantic row or column limit exceeded"));}
  if bytes>self.limits.max_value_bytes{return Err(ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"GIS semantic value limit exceeded"));}
  self.rows=rows;self.bytes=bytes;Ok(())
 }
}
fn required(value:Option<&F>)->Result<&F>{value.ok_or_else(||invalid("GIS required native role is absent"))}
fn record(value:Option<&F>)->Result<&R>{match required(value)?{F::Record(value)=>Ok(value),_=>Err(invalid("GIS native role requires record"))}}
fn text(value:Option<&F>)->Result<&str>{match required(value)?{F::Text(value)=>Ok(value),_=>Err(invalid("GIS native role requires text"))}}
fn list(value:Option<&F>)->Result<&[F]>{match required(value)?{F::List(value)=>Ok(value),_=>Err(invalid("GIS native role requires list"))}}
fn scalar<T:DslField>(value:Option<&F>,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T>{native.scoped_stage(|native|{native.begin_stage(0)?;T::from_value_controlled(required(value)?,native)})}
fn child(value:&R,table:usize,census:&mut Census,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<()>{census.row(table,&[text(value.get(0))?.len(),text(value.get(1))?.len(),text(value.get(2))?.len(),text(value.get(3))?.len(),text(value.get(4))?.len()],native)}
/// 🗺️ Visits every flat value, feature and child using its exact declared storage-class role.
pub(super)fn admit_record(source:&R,native:&mut semio_framework_value::NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 native.scoped_stage(|native|{
  native.begin_stage(0)?;if limits.max_tables<20||super::super::sqlite::WIDTHS.iter().any(|width|*width>limits.max_columns){return Err(ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"GIS authored schema extent exceeds caller limits"));}
  let mut c=Census{limits,rows:0,bytes:0};c.row(0,&[],native)?;child(record(source.get(3))?,5,&mut c,native)?;
  match required(source.get(4))?{F::Absent=>{},F::Record(value)=>child(value,6,&mut c,native)?,_=>return Err(invalid("GIS optional image role differs"))}
  child(record(source.get(5))?,7,&mut c,native)?;
  for(id,table)in[(0,2),(1,3),(2,4)]{for value in list(source.get(id))?{let value=record(Some(value))?;let _=scalar::<u64>(value.get(1),native)?;c.row(table,&[24,text(value.get(0))?.len()],native)?;}}
  for value in list(source.get(6))?{
   let value=record(Some(value))?;let kind=scalar::<Kind>(value.get(0),native)?;let items=list(value.get(7))?;let members=list(value.get(8))?;
   let(tag,role,table)=match kind{Kind::Null=>("null",None,8),Kind::Boolean=>("boolean",Some(1),9),Kind::Unsigned=>("unsigned",Some(2),10),Kind::Signed=>("signed",Some(3),11),Kind::Float=>("float",Some(4),12),Kind::Text=>("text",Some(5),13),Kind::Bytes=>("bytes",Some(6),14),Kind::Array=>("array",None,16),Kind::Object=>("object",None,18)};
   for id in 1..=6{let field=required(value.get(id))?;if matches!(field,F::Absent)==(role==Some(id)){return Err(invalid("GIS native Value has unrelated or absent variant fields"));}}
   if(tag!="array"&&!items.is_empty())||(tag!="object"&&!members.is_empty()){return Err(invalid("GIS native Value has unrelated relationships"));}
   c.row(1,&[tag.len()],native)?;
   match tag{
    "null"=>c.row(table,&[],native)?,
    "boolean"=>{let _=scalar::<bool>(value.get(1),native)?;c.row(table,&[8],native)?;},
    "unsigned"=>{let _=scalar::<u64>(value.get(2),native)?;c.row(table,&[16],native)?;},
    "signed"=>{let _=scalar::<i64>(value.get(3),native)?;c.row(table,&[8],native)?;},
    "float"=>{let number=scalar::<f64>(value.get(4),native)?;c.row(table,&[if number.is_nan(){11}else if number.is_infinite(){32}else{22}],native)?;},
    "text"=>c.row(table,&[text(value.get(5))?.len()],native)?,
    "bytes"=>{let F::Bytes64(bytes)=required(value.get(6))? else{return Err(invalid("GIS native octets differ"))};c.row(table,&[],native)?;for _ in bytes{c.row(15,&[24],native)?;}},
    "array"=>{c.row(table,&[],native)?;for item in items{let _=scalar::<u64>(Some(item),native)?;c.row(17,&[24],native)?;}},
    "object"=>{c.row(table,&[],native)?;for member in members{let member=record(Some(member))?;let _=scalar::<u64>(member.get(1),native)?;c.row(19,&[24,text(member.get(0))?.len()],native)?;}},
    _=>unreachable!()
   }
  }
  native.checkpoint()
 })
}
