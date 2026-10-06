//! 🎧️ Borrowed WAV roles admit exact relational sample, octet and chunk-order cells before ownership.
use super::{invalid,work,ownership,numeric_class,UnsignedWord,WavSnapshot};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_dsl_record::{DslField,RecordValue as R,FieldValue as F};
use semio_framework_value::{ValueError,NativeDecodeControl as N};
type Result<T>=std::result::Result<T,ValueError>;
const WIDTHS:&[(&str,usize)]=&[("wav_chunk_order",8),("wav_data",3),("wav_document",2),("wav_float32_sample",6),("wav_format",8),("wav_format_extension",1),("wav_format_extension_byte",4),("wav_other_chunk",5),("wav_other_chunk_byte",4),("wav_pcm16_sample",4),("wav_pcm8_sample",4),("wav_raw_data_byte",4)];
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{if WavSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes{return Err(ownership("WAV authored SQLite schema exceeds schema byte limit"))}if limits.max_tables<WIDTHS.len()||limits.max_columns<8||limits.max_rows<3{return Err(work("WAV authored relational extent exceeds caller limits"))}Ok(())}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ownership("WAV semantic cell extent overflow"))}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{n.step()?;if !WIDTHS.iter().any(|(name,_)|*name==table){return Err(invalid("WAV census table is not authored"))}let rows=self.rows.checked_add(1).ok_or_else(||work("WAV semantic row count overflow"))?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(work("WAV native snapshot exceeds semantic row limit"))}if bytes>self.limits.max_value_bytes{return Err(ownership("WAV native snapshot exceeds semantic cell byte limit"))}self.rows=rows;self.bytes=bytes;Ok(())}
}
fn shape(record:&R,count:u16)->Result<()>{if record.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("WAV native record fields differ from the authored role map"))}}
fn field(record:&R,id:u16)->Result<&F>{record.get(id).ok_or_else(||invalid("WAV required native field is absent"))}
fn record(value:&F)->Result<&R>{match value{F::Record(value)=>Ok(value),_=>Err(invalid("WAV native role requires its direct record"))}}
fn items(value:&F)->Result<&[F]>{match value{F::List(value)=>Ok(value),_=>Err(invalid("WAV native sample role requires its ordered list"))}}
fn text(value:&F,n:&mut N<'_>)->Result<usize>{n.step()?;match value{F::Text(value)=>Ok(value.len()),_=>Err(invalid("WAV native role requires text"))}}
fn scalar<T:DslField>(value:&F,n:&mut N<'_>)->Result<T>{n.scoped_stage(|n|{n.begin_stage(0)?;T::from_value_controlled(value,n)})}
fn integer<T:DslField>(value:&F,n:&mut N<'_>)->Result<usize>{let _=scalar::<T>(value,n)?;Ok(8)}
fn octets(value:&F,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{let _=scalar::<u8>(value,n)?;c.row(table,32,n)?;}Ok(())}
pub(super)fn admit_record(source:&R,limits:SqliteDatabaseLimits,n:&mut N<'_>)->Result<()>{
 extent(limits)?;n.scoped_stage(|n|{
  n.begin_stage(0)?;shape(source,7)?;let mut c=Census{limits,rows:0,bytes:0};
  c.row("wav_document",add(8,text(field(source,0)?,n)?)?,n)?;
  let format=record(field(source,1)?)?;shape(format,7)?;
  let mut bytes=8;for id in[0,1,4,5]{bytes=add(bytes,integer::<u16>(field(format,id)?,n)?)?;}for id in[2,3]{bytes=add(bytes,integer::<u32>(field(format,id)?,n)?)?;}bytes=add(bytes,integer::<u8>(field(source,3)?,n)?)?;c.row("wav_format",bytes,n)?;
  match field(format,6)?{F::Absent=>n.step()?,value=>{c.row("wav_format_extension",8,n)?;octets(value,"wav_format_extension_byte",&mut c,n)?;}}
  let data=record(field(source,2)?)?;if data.fields.len()>5||data.fields.keys().any(|id|!(1..=5).contains(id)){return Err(invalid("WAV sample variant has undeclared native fields"))}
  let F::Enum(kind)=field(data,1)?else{return Err(invalid("WAV samples require a declared kind"))};let id=kind.checked_add(2).filter(|id|*id<=5).ok_or_else(||invalid("WAV sample kind is undeclared"))?as u16;
  for(other,value)in &data.fields{n.step()?;if *other!=1&&*other!=id&&!matches!(value,F::Absent){return Err(invalid("WAV sample kind selects exactly one typed array"))}}
  let (label,table)=match kind{0=>("pcm16","wav_pcm16_sample"),1=>("pcm8","wav_pcm8_sample"),2=>("float32","wav_float32_sample"),3=>("raw","wav_raw_data_byte"),_=>return Err(invalid("WAV sample kind is undeclared"))};
  let _=scalar::<u8>(field(source,4)?,n)?;c.row("wav_data",add(16,label.len())?,n)?;
  for value in items(field(data,id)?)?{let bytes=match kind{0=>{let _=scalar::<i16>(value,n)?;32},1|3=>{let _=scalar::<u8>(value,n)?;32},2=>{let sample=scalar::<f32>(value,n)?;add(add(32,numeric_class(sample).len())?,if sample.is_finite(){8}else{0})?},_=>unreachable!()};c.row(table,bytes,n)?;}
  let chunks=items(field(source,5)?)?;
  for value in chunks{
   let chunk=record(value)?;shape(chunk,3)?;let _=scalar::<u8>(field(chunk,2)?,n)?;c.row("wav_other_chunk",add(32,text(field(chunk,0)?,n)?)?,n)?;
   let F::Bytes64(bytes)=field(chunk,1)?else{return Err(invalid("WAV chunk requires literal octets"))};for _ in bytes{c.row("wav_other_chunk_byte",32,n)?;}
  }
  for value in items(field(source,6)?)?{
   let value=record(value)?;if value.fields.len()>2||value.fields.keys().any(|id|![1,2].contains(id)){return Err(invalid("WAV chunk reference contains an unknown field"))}
   let index=value.get(2).filter(|value|!matches!(value,F::Absent));
   let bytes=match(value.get(1),index){
    (Some(F::Enum(0)),None)=>32+"format".len(),
    (Some(F::Enum(1)),None)=>32+"samples".len(),
    (Some(F::Enum(2)),Some(F::UInt(index)))=>{let present=usize::try_from(*index).ok().filter(|index|*index<chunks.len()).and_then(|index|i64::try_from(index).ok()?.checked_add(1)).is_some();add(add(24+"other".len(),UnsignedWord::new(*index).text().len())?,if present{8}else{0})?},
    _=>return Err(invalid("WAV chunk reference requires its exact declared kind and unsigned64 index"))
   };c.row("wav_chunk_order",bytes,n)?;
  }
  n.checkpoint()
 })
}
