//! 🗽️ Borrowed native OBJ fields admit all handcrafted SQL cells before typed construction.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::NativeDecodeControl;
use semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn sum(left:usize,right:usize)->Result<usize>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"OBJ semantic cell extent overflow"))}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(||invalid("OBJ required native role is missing"))}
fn record(value:&F,count:usize)->Result<&R>{let F::Record(value)=value else{return Err(invalid("OBJ native role requires record"))};exact(value,count)?;Ok(value)}
fn exact(value:&R,count:usize)->Result<()>{if value.fields.len()!=count||value.fields.keys().any(|id|usize::from(*id)>=count){return Err(invalid("OBJ native record differs from its authored fields"))}Ok(())}
fn list(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid("OBJ native role requires literal list"))};Ok(value)}
fn uint(value:&F,maximum:u64)->Result<u64>{match value{F::UInt(value)if *value<=maximum=>Ok(*value),_=>Err(invalid("OBJ literal unsigned word exceeds its authored width"))}}
fn optional_uint(value:&F,maximum:u64)->Result<Option<u64>>{match value{F::Absent=>Ok(None),value=>uint(value,maximum).map(Some)}}
fn real(value:&F)->Result<usize>{let F::Float(value)=value else{return Err(invalid("OBJ coordinate requires exact float word"))};Ok(if value.is_nan(){11}else if value.is_infinite(){32}else{22})}
fn optional_real(value:&F)->Result<usize>{match value{F::Absent=>Ok(0),value=>real(value)}}
fn text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Text(value)=value else{return Err(invalid("OBJ lexical role requires text"))};native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
fn optional_text(value:&F,native:&mut NativeDecodeControl<'_>)->Result<usize>{match value{F::Absent=>Ok(0),value=>text(value,native)}}
fn resolved(value:u64,count:usize)->usize{if value<count as u64{8}else{0}}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize,native:&mut NativeDecodeControl<'_>)->Result<()>{native.step()?;self.rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ semantic rows overflow"))?;self.bytes=sum(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"OBJ semantic row limit exceeded"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"OBJ semantic value limit exceeded"))}Ok(())}
}
/// 🫳️ Traverses the original literal roles without a Snapshot, SQL database or copied native record.
pub(super)fn admit_record(value:&R,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<()>{
 super::extent(limits)?;native.scoped_stage(|native|{
  native.begin_stage(0)?;exact(value,11)?;let vertices=list(field(value,1)?)?;let coordinates=list(field(value,2)?)?;let normals=list(field(value,3)?)?;let faces=list(field(value,4)?)?;let boundaries=faces.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"OBJ boundary count overflow"))?;
  let mut census=Census{limits,rows:0,bytes:0};let schema=text(field(value,0)?,native)?;let library=optional_text(field(value,7)?,native)?;census.row(sum(sum(8,schema)?,library)?,native)?;
  for value in vertices{let value=record(value,4)?;let bytes=sum(sum(sum(sum(24,real(field(value,0)?)?)?,real(field(value,1)?)?)?,real(field(value,2)?)?)?,optional_real(field(value,3)?)?)?;census.row(bytes,native)?;}
  for value in coordinates{let value=record(value,3)?;census.row(sum(sum(sum(24,real(field(value,0)?)?)?,real(field(value,1)?)?)?,optional_real(field(value,2)?)?)?,native)?;}
  for value in normals{let value=record(value,3)?;census.row(sum(sum(sum(24,real(field(value,0)?)?)?,real(field(value,1)?)?)?,real(field(value,2)?)?)?,native)?;}
  for value in faces{let value=record(value,1)?;census.row(24,native)?;for value in list(field(value,0)?)?{let value=record(value,3)?;let vertex=uint(field(value,0)?,u64::from(u32::MAX))?;let coordinate=optional_uint(field(value,1)?,u64::from(u32::MAX))?;let normal=optional_uint(field(value,2)?,u64::from(u32::MAX))?;let mut bytes=sum(32,resolved(vertex,vertices.len()))?;if let Some(value)=coordinate{bytes=sum(bytes,sum(8,resolved(value,coordinates.len()))?)?;}if let Some(value)=normal{bytes=sum(bytes,sum(8,resolved(value,normals.len()))?)?;}census.row(bytes,native)?;}}
  for id in[5,6]{for value in list(field(value,id)?)?{let value=record(value,2)?;census.row(sum(24,text(field(value,0)?,native)?)?,native)?;for value in list(field(value,1)?)?{let index=uint(value,u64::MAX)?;census.row(sum(40,resolved(index,faces.len()))?,native)?;}}}
  for index in 0..boundaries{census.row(if index<faces.len(){32}else{24},native)?;}
  for value in list(field(value,8)?)?{let value=record(value,2)?;let index=uint(field(value,0)?,u64::MAX)?;census.row(sum(sum(40,resolved(index,boundaries))?,text(field(value,1)?,native)?)?,native)?;}
  for value in list(field(value,9)?)?{let value=record(value,2)?;let index=uint(field(value,0)?,u64::MAX)?;let group=optional_uint(field(value,1)?,u64::from(u32::MAX))?;census.row(sum(sum(40,resolved(index,boundaries))?,if group.is_some(){8}else{0})?,native)?;}
  for value in list(field(value,10)?)?{let value=record(value,2)?;uint(field(value,0)?,u64::MAX)?;census.row(sum(40,text(field(value,1)?,native)?)?,native)?;}
  native.checkpoint()
 })
}
