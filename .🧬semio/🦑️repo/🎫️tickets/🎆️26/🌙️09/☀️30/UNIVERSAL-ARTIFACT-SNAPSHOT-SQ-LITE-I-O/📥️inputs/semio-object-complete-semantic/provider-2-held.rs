//! 📦️ Borrowed placement words and independent child targets admit complete relational cells.
use super::{SemioObjectSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Object native field differs from its declared shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Object complete cell extent overflow"))}
/// 🏛️ Admits all five authored tables and the thirty-two-column placement row.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioObjectSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_object_document","semio_object_reference","semio_object_brep_child","semio_object_mesh_child","semio_object_value_child"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioObjectSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Object schema exceeds caller bytes"))}
 if limits.max_tables<5||limits.max_columns<32||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Object authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Object complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Object complete cells exceed caller bytes"))}Ok(())}
}
fn float_bytes(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn text<'a>(reader:&mut store::ByteReader<'a>,control:&mut NativeDecodeControl<'_>)->Result<&'a str>{control.borrow_text(native::bytes(reader)?)}
fn hex(value:&str,expected:Option<&str>,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let size=native::hex_text_extent(value,control)?;
 if let Some(expected)=expected{if size!=expected.len(){return Err(invalid())}for(pair,byte)in value.as_bytes().chunks_exact(2).zip(expected.bytes()){let digit=|v:u8|match v{b'0'..=b'9'=>v-b'0',b'a'..=b'f'=>v-b'a'+10,b'A'..=b'F'=>v-b'A'+10,_=>unreachable!()};if(digit(pair[0])<<4)|digit(pair[1])!=byte{return Err(invalid())}}}Ok(size)
}
fn binary_child(reader:&mut store::ByteReader<'_>,subset:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 match reader.read_u8().map_err(|_|invalid())?{0=>Ok(()),1=>{let id=text(reader,control)?.len();let artifact=text(reader,control)?.len();let kind=text(reader,control)?;let standard=text(reader,control)?;let actual_subset=text(reader,control)?;if kind!="s.stdio.semio"||standard!="v1"||actual_subset!=subset{return Err(invalid())}census.row(add(24,id)?)?;census.row(add(add(add(add(8,artifact)?,kind.len())?,standard.len())?,actual_subset.len())?)},_=>Err(invalid())}
}
fn text_child(value:&str,subset:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 if value=="[]"{return Ok(())}let[id,target]=native::record(value,control)?;let[artifact,kind,standard,actual_subset]=native::record(target,control)?;
 let id=hex(id,None,control)?;let artifact=hex(artifact,None,control)?;let kind=hex(kind,Some("s.stdio.semio"),control)?;let standard=hex(standard,Some("v1"),control)?;let subset=hex(actual_subset,Some(subset),control)?;
 census.row(add(24,id)?)?;census.row(add(add(add(add(8,artifact)?,kind)?,standard)?,subset)?)
}
/// 🧊️ Counts the original ten raw binary64 words and three optional child pairs.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}let schema=text(&mut reader,control)?;if schema!="stdio.semio.object"{return Err(invalid())}let mut bytes=add(8,schema.len())?;control.begin_stage(13)?;
 for _ in 0..10{let word=reader.read_bytes(8).map_err(|_|invalid())?;let word=f64::from_le_bytes(word.try_into().map_err(|_|invalid())?);bytes=add(bytes,float_bytes(word))?;control.step()?;}
 let mut census=Census{limits,rows:0,bytes:0};census.row(bytes)?;for subset in["brep","mesh","value"]{binary_child(&mut reader,subset,&mut census,control)?;control.step()?;}if reader.remaining()!=0{return Err(invalid())}control.checkpoint()})
}
/// 📝️ Preserves exact float words, omitted optional children and independent local identities.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{let fields=native::fields(body,["schema","transform","brep","mesh","properties"],control)?;let schema=hex(fields[0].ok_or_else(invalid)?,Some("stdio.semio.object"),control)?;let words:[&str;10]=native::record(fields[1].ok_or_else(invalid)?,control)?;let mut bytes=add(8,schema)?;control.begin_stage(13)?;
 for word in words{bytes=add(bytes,float_bytes(native::float(word,control)?))?;control.step()?;}
 let mut census=Census{limits,rows:0,bytes:0};census.row(bytes)?;for(field,subset)in fields[2..].iter().zip(["brep","mesh","value"]){text_child(field.unwrap_or("[]"),subset,&mut census,control)?;control.step()?;}control.checkpoint()})
}
