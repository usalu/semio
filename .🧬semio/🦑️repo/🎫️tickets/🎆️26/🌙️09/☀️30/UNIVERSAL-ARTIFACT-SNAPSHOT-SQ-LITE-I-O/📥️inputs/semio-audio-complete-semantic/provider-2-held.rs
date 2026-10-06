//! 🔊️ Borrowed Audio primitives admit exact authored sample and tag cells before typed construction.
use super::{SemioAudioSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Audio native field differs from its declared shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Audio complete cell extent overflow"))}
/// 🏛️ Admits the full authored four-table layout before an Audio owner is allocated.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioAudioSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_audio_document","semio_audio_channel","semio_audio_sample","semio_audio_tag"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioAudioSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Audio schema exceeds caller bytes"))}
 if limits.max_tables<4||limits.max_columns<5||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Audio authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Audio complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Audio complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Audio declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn sample(word:u32)->usize{if word&0x7f800000==0x7f800000&&word&0x007fffff!=0{32}else{40}}
fn format(value:u8)->Result<usize>{match value{0=>Ok(4),1..=3=>Ok(5),4|5=>Ok(3),_=>Err(invalid())}}
fn document_format(value:Option<&str>)->Result<usize>{match value{Some("pcm8")=>Ok(4),Some("pcm16"|"pcm24"|"pcm32")|None=>Ok(5),Some("f32"|"f64")=>Ok(3),_=>Err(invalid())}}
/// 🎚️ Counts actual binary32 source words without allocating channels or samples.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}
  let schema=text(&mut reader,control)?;reader.read_u32_le().map_err(|_|invalid())?;let format=format(reader.read_u8().map_err(|_|invalid())?)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(add(16,schema)?,format)?)?;
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{census.row(24)?;let count=native::length(&mut reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{census.row(sample(reader.read_u32_le().map_err(|_|invalid())?))?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;}
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;for _ in 0..count{census.row(add(add(24,text(&mut reader,control)?)?,text(&mut reader,control)?)?)?;control.step()?;}
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
/// 📝️ Preserves original lexical u32 and format defaults while measuring complete document cells.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","sampleRate","format","channels","tags"],control)?;let schema=native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?;crate::standards::v1::subsets::mesh::io::text::snapshot::parse_u32(fields[1].ok_or_else(invalid)?).map_err(|_|invalid())?;let format=document_format(fields[2])?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(add(16,schema)?,format)?)?;
  let mut channels=native::Items::new(fields[3].unwrap_or("[]"))?;let count=channels.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=channels.next(control)?{census.row(24)?;let mut samples=native::Items::new(value)?;let count=samples.count(control,limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=samples.next(control)?{let word=u32::from_str_radix(value,16).map_err(|_|invalid())?;census.row(sample(word))?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;}
  let mut tags=native::Items::new(fields[4].unwrap_or("[]"))?;let count=tags.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=tags.next(control)?{let[key,value]=native::record(value,control)?;census.row(add(add(24,native::hex_text_extent(key,control)?)?,native::hex_text_extent(value,control)?)?)?;control.step()?;}control.checkpoint()
 })
}
