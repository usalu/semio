//! 🖼️ Borrowed Image primitives count every INTEGER sample and nullable ICC byte before native ownership.
use super::{SemioImageSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use super::native_decoding::{Items,pair};
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Image native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Image complete cell extent overflow"))}
/// 🏛️ Admits the exact four-table authored sample layout before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioImageSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_image_document","semio_image_frame","semio_image_sample","semio_image_metadata"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioImageSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Image schema exceeds caller bytes"))}
 if limits.max_tables<4||limits.max_columns<7||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Image authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Image complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Image complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Image declared rows exceed caller limit"))}Ok(())}
 fn samples(&mut self,count:usize,control:&mut NativeDecodeControl<'_>)->Result<()>{self.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{self.row(40)?;control.step()?;}control.checkpoint()})}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn octets(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{let bytes=native::bytes(reader)?;control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for _ in bytes{control.step()?;}control.checkpoint()?;Ok(bytes.len())})}
fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if !value.len().is_multiple_of(2){return Err(invalid())}control.scoped_stage(|control|{control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){if !pair.iter().all(u8::is_ascii_hexdigit){return Err(invalid())}control.step()?;}control.checkpoint()?;Ok(value.len()/2)})}
fn colorspace(value:u8)->Result<usize>{match value{0=>Ok(3),1=>Ok(4),2=>Ok(9),3=>Ok(15),4=>Ok(7),_=>Err(invalid())}}
fn document_colorspace(value:Option<&str>)->Result<usize>{match value{None|Some("r")=>Ok(3),Some("a")=>Ok(4),Some("g")=>Ok(9),Some("y")=>Ok(15),Some("i")=>Ok(7),_=>Err(invalid())}}
/// 📦️ Counts every frame byte as its authored five-INTEGER sample row.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}
  let schema=text(&mut reader,control)?;reader.read_u32_le().map_err(|_|invalid())?;reader.read_u32_le().map_err(|_|invalid())?;let colorspace=colorspace(reader.read_u8().map_err(|_|invalid())?)?;reader.read_u8().map_err(|_|invalid())?;
  let icc=match reader.read_u8().map_err(|_|invalid())?{0=>0,1=>octets(&mut reader,control)?,_=>return Err(invalid())};let mut census=Census{limits,rows:0,bytes:0};census.row(add(add(add(32,schema)?,colorspace)?,icc)?)?;
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{reader.read_u32_le().map_err(|_|invalid())?;census.row(32)?;let count=octets(&mut reader,control)?;census.samples(count,control)?;control.step()?;}
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{census.row(add(add(24,text(&mut reader,control)?)?,text(&mut reader,control)?)?)?;control.step()?;}
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
/// 📝️ Preserves the original Image list grammar, u32 lexemes and nullable ICC presence.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","width","height","colorspace","bitDepth","icc","frames","metadata"],control)?;
  let schema=native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?;
  for index in[1,2]{crate::standards::v1::subsets::mesh::io::text::snapshot::parse_u32(fields[index].ok_or_else(invalid)?).map_err(|_|invalid())?;}
  let colorspace=document_colorspace(fields[3])?;if let Some(value)=fields[4]{crate::standards::v1::subsets::image::io::text::snapshot::parse_u8(value).map_err(|_|invalid())?;}
  let icc=match fields[5]{None|Some("[0]")=>0,Some(value)=>{let(tag,value)=pair(value,control)?;if tag!="1"{return Err(invalid())}hex(value,control)?}};let mut census=Census{limits,rows:0,bytes:0};census.row(add(add(add(32,schema)?,colorspace)?,icc)?)?;
  let mut frames=Items::new(fields[6].unwrap_or("[]"))?;let count=frames.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=frames.next(control)?{let(delay,rgba)=pair(value,control)?;crate::standards::v1::subsets::mesh::io::text::snapshot::parse_u32(delay).map_err(|_|invalid())?;census.row(32)?;let count=hex(rgba,control)?;census.samples(count,control)?;control.step()?;}
  let mut metadata=Items::new(fields[7].unwrap_or("[]"))?;let count=metadata.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=metadata.next(control)?{let(key,value)=pair(value,control)?;census.row(add(add(24,native::hex_text_extent(key,control)?)?,native::hex_text_extent(value,control)?)?)?;control.step()?;}control.checkpoint()
 })
}
