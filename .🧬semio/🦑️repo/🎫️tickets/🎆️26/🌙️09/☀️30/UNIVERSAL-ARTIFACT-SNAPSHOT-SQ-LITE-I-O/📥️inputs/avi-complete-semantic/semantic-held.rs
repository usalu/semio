//! 📼️ Borrowed AVI header, format and ordered chunk cells admit the literal snapshot before typed ownership.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue};
use store::sqlite_snapshot::SqliteDatabaseLimits;
use semio_framework_value::NativeDecodeControl;
const TABLES:[(&str,usize);12]=[("avi_document",3),("avi_main_header",11),("avi_main_reserved",4),("avi_stream",4),("avi_stream_header",20),("avi_bitmap_info",12),("avi_wave_format",8),("avi_raw_format",2),("avi_stream_chunk",6),("avi_stream_extra",5),("avi_unknown_chunk",5),("avi_hdrl_extra",5)];
pub(super) fn extent(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 let mut bytes=0usize;for statement in AviSnapshot::SQLITE_SCHEMA.split(';'){bytes=bytes.checked_add(statement.trim().len()).ok_or_else(||ownership("AVI authored schema byte count overflow"))?;}for(name,_)in TABLES{bytes=bytes.checked_add(name.len()).ok_or_else(||ownership("AVI authored schema name count overflow"))?;}
 if bytes.max(AviSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ownership("AVI authored schema exceeds caller bytes"))}if limits.max_tables<TABLES.len()||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<2{return Err(work("AVI authored relational extent exceeds caller limits"))}Ok(())
}
pub(super) fn typed(snapshot:&AviSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;super::visit_rows(snapshot,&mut writer)?;writer.finish_borrowed()}
fn field(record:&RecordValue,id:u16)->Result<&F,ValueError>{record.get(id).ok_or_else(||invalid("AVI native authored field is missing"))}
fn record(value:&F,count:usize)->Result<&RecordValue,ValueError>{let F::Record(record)=value else{return Err(invalid("AVI literal Record required"))};exact(record,count)?;Ok(record)}
fn exact(record:&RecordValue,count:usize)->Result<(),ValueError>{if record.fields.len()!=count{return Err(invalid("AVI native Record has undeclared fields"))}Ok(())}
fn list(value:&F)->Result<&[F],ValueError>{match value{F::List(values)=>Ok(values),_=>Err(invalid("AVI literal List required"))}}
fn uint(value:&F,maximum:u64)->Result<(),ValueError>{if matches!(value,F::UInt(number)if *number<=maximum){Ok(())}else{Err(invalid("AVI unsigned field exceeds its authored width"))}}
fn signed(value:&F)->Result<(),ValueError>{if matches!(value,F::Int(number)if i32::try_from(*number).is_ok()){Ok(())}else{Err(invalid("AVI signed field requires integer32"))}}
fn boolean(value:&F)->Result<(),ValueError>{if matches!(value,F::Bool(_)){Ok(())}else{Err(invalid("AVI flag requires Bool"))}}
fn add(bytes:&mut usize,amount:usize,maximum:usize)->Result<(),ValueError>{*bytes=bytes.checked_add(amount).ok_or_else(||ownership("AVI semantic cell byte count overflow"))?;if *bytes>maximum{return Err(ownership("AVI semantic cells exceed caller bytes"))}Ok(())}
fn text(value:&F,bytes:&mut usize,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let F::Text(value)=value else{return Err(invalid("AVI lexical field requires Text"))};add(bytes,value.len(),maximum)?;native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok::<_,ValueError>(())})}
fn blob(value:&F,bytes:&mut usize,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let values=list(value)?;add(bytes,values.len(),maximum)?;native.scoped_stage(|native|{native.begin_stage(values.len())?;for value in values{uint(value,u64::from(u8::MAX))?;native.step()?;}Ok::<_,ValueError>(())})}
fn rows(count:&mut usize,amount:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{*count=count.checked_add(amount).ok_or_else(||work("AVI semantic row count overflow"))?;if *count>limits.max_rows{return Err(work("AVI semantic rows exceed caller limit"))}Ok(())}
fn format(value:&F)->Result<(&str,&RecordValue),ValueError>{let F::Block(value)=value else{return Err(invalid("AVI format requires its declared Block"))};let F::Statements(values)=value.as_ref() else{return Err(invalid("AVI format requires tagged Statements"))};let[(keyword,record)]=values.as_slice()else{return Err(invalid("AVI format requires exactly one tagged value"))};Ok((keyword,record))}
fn riff(values:&[F],bytes:&mut usize,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for value in values{let value=record(value,2)?;add(bytes,24,maximum)?;text(field(value,0)?,bytes,maximum,native)?;blob(field(value,1)?,bytes,maximum,native)?;native.step()?;}Ok(())}
pub(super) fn borrowed(root:&RecordValue,limits:SqliteDatabaseLimits,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 extent(limits)?;native.scoped_stage(|native|{exact(root,6)?;let main=record(field(root,1)?,11)?;let reserved=list(field(main,10)?)?;let streams=list(field(root,2)?)?;let unknown=list(field(root,4)?)?;let hdrl=list(field(root,5)?)?;
 let total=native.scoped_stage(|native|{native.begin_stage(streams.len())?;let mut count=2;for amount in[reserved.len(),unknown.len(),hdrl.len(),streams.len().checked_mul(3).ok_or_else(||work("AVI stream row count overflow"))?]{rows(&mut count,amount,limits)?;}for stream in streams{let stream=record(stream,4)?;rows(&mut count,list(field(stream,2)?)?.len(),limits)?;rows(&mut count,list(field(stream,3)?)?.len(),limits)?;native.step()?;}Ok::<_,ValueError>(count)})?;
 native.begin_stage(total)?;let mut bytes=0;boolean(field(root,3)?)?;add(&mut bytes,16,limits.max_value_bytes)?;text(field(root,0)?,&mut bytes,limits.max_value_bytes,native)?;native.step()?;
 for id in 0..10{uint(field(main,id)?,u64::from(u32::MAX))?;}add(&mut bytes,88,limits.max_value_bytes)?;native.step()?;for value in reserved{uint(value,u64::from(u32::MAX))?;add(&mut bytes,32,limits.max_value_bytes)?;native.step()?;}
 for stream in streams{let stream=record(stream,4)?;let header=record(field(stream,0)?,19)?;let(kind,parameters)=format(field(stream,1)?)?;let(kind_width,base)=match kind{"bitmapInfo"=>(11,88),"waveFormat"=>(7,56),"raw"=>(1,8),_=>return Err(invalid("AVI format keyword is not authored"))};exact(parameters,kind_width)?;add(&mut bytes,24usize.checked_add(kind.len()).ok_or_else(||ownership("AVI format keyword extent overflow"))?,limits.max_value_bytes)?;native.step()?;
  add(&mut bytes,136,limits.max_value_bytes)?;text(field(header,0)?,&mut bytes,limits.max_value_bytes,native)?;text(field(header,1)?,&mut bytes,limits.max_value_bytes,native)?;for id in[2,5,6,7,8,9,10,12]{uint(field(header,id)?,u64::from(u32::MAX))?;}for id in[3,4]{uint(field(header,id)?,u64::from(u16::MAX))?;}for id in[11,13,14,15,16]{signed(field(header,id)?)?;}uint(field(header,17)?,u64::from(u8::MAX))?;blob(field(header,18)?,&mut bytes,limits.max_value_bytes,native)?;native.step()?;
  add(&mut bytes,base,limits.max_value_bytes)?;match kind{
   "bitmapInfo"=>{for id in[0,6,9,10]{uint(field(parameters,id)?,u64::from(u32::MAX))?;}for id in[1,2,7,8]{signed(field(parameters,id)?)?;}for id in[3,4]{uint(field(parameters,id)?,u64::from(u16::MAX))?;}text(field(parameters,5)?,&mut bytes,limits.max_value_bytes,native)?;}
   "waveFormat"=>{for id in[0,1,4,5]{uint(field(parameters,id)?,u64::from(u16::MAX))?;}for id in[2,3]{uint(field(parameters,id)?,u64::from(u32::MAX))?;}blob(field(parameters,6)?,&mut bytes,limits.max_value_bytes,native)?;}
   "raw"=>blob(field(parameters,0)?,&mut bytes,limits.max_value_bytes,native)?,
   _=>unreachable!()
  }native.step()?;for chunk in list(field(stream,2)?)?{let chunk=record(chunk,3)?;boolean(field(chunk,2)?)?;add(&mut bytes,32,limits.max_value_bytes)?;text(field(chunk,0)?,&mut bytes,limits.max_value_bytes,native)?;blob(field(chunk,1)?,&mut bytes,limits.max_value_bytes,native)?;native.step()?;}riff(list(field(stream,3)?)?,&mut bytes,limits.max_value_bytes,native)?;
 }riff(unknown,&mut bytes,limits.max_value_bytes,native)?;riff(hdrl,&mut bytes,limits.max_value_bytes,native)?;native.checkpoint()
 })
}
