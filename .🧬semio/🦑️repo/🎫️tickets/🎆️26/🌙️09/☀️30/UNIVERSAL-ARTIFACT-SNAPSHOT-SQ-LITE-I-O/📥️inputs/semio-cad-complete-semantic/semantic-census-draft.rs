//! 📐️ Borrowed CAD primitives count all layer, block, geometry and exact numeric cells.
use super::{SemioCadSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio CAD native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio CAD complete cell extent overflow"))}
/// 🏗️ Admits the authored CAD tables and exact numeric columns before any native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioCadSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_cad_document","semio_cad_layer","semio_cad_block","semio_cad_entity","semio_cad_line","semio_cad_arc","semio_cad_circle","semio_cad_ellipse","semio_cad_polyline","semio_cad_polyline_vertex","semio_cad_text","semio_cad_insert","semio_cad_solid","semio_cad_dimension"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioCadSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio CAD schema exceeds caller bytes"))}
 if limits.max_tables<14||limits.max_columns<25||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD authored layout exceeds copied limits"))}Ok(())
}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio CAD complete cells exceed caller bytes"))}Ok(())}
 fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn kind(tag:u8)->Result<usize>{match tag{0|5=>Ok(4),1=>Ok(3),2|6=>Ok(6),3=>Ok(7),4=>Ok(8),7=>Ok(5),8=>Ok(9),_=>Err(invalid())}}
fn boolean(value:u8)->Result<()>{if value>1{return Err(invalid())}Ok(())}
fn binary_floats(reader:&mut store::ByteReader<'_>,count:usize)->Result<usize>{let mut bytes=0;for _ in 0..count{bytes=add(bytes,scalar(reader.read_f64_le().map_err(|_|invalid())?))?;}Ok(bytes)}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_record(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,block:bool)->Result<()>{
 let handle=text(reader,control)?;text(reader,control)?;let tag=byte(reader)?;census.row(add(add(if block{40}else{32},handle)?,kind(tag)?)?)?;
 let bytes=match tag{
  0=>add(8,binary_floats(reader,4)?)?,1=>add(8,binary_floats(reader,5)?)?,2=>add(8,binary_floats(reader,3)?)?,3=>add(8,binary_floats(reader,7)?)?,
  4=>{binary_list(reader,census,control,|reader,census,_|census.row(add(24,binary_floats(reader,2)?)?))?;boolean(byte(reader)?)?;16},
  5=>add(add(8,binary_floats(reader,4)?)?,text(reader,control)?)?,
  6=>{text(reader,control)?;add(16,binary_floats(reader,5)?)?},
  7=>add(8,binary_floats(reader,8)?)?,
  8=>add(add(8,binary_floats(reader,5)?)?,text(reader,control)?)?,_=>return Err(invalid())
 };census.row(bytes)
}
/// 📦️ Reads the actual CAD primitive order without constructing layer or block collections.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   let name=text(reader,control)?;let bits=reader.read_varint_u64().map_err(|_|invalid())?;if (bits as i32)as u64!=bits{return Err(invalid())}let line=text(reader,control)?;boolean(byte(reader)?)?;census.row(add(add(40,name)?,line)?)
  })?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{
   census.row(add(add(24,text(reader,control)?)?,binary_floats(reader,2)?)?)?;binary_list(reader,census,control,|reader,census,control|binary_record(reader,census,control,true))
  })?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|binary_record(reader,census,control,false))?;
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(scalar(native::float(value,control)?))}
fn document_point(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{let[x,y]=native::record(value,control)?;add(document_float(x,control)?,document_float(y,control)?)}
fn document_color(value:&str,control:&mut NativeDecodeControl<'_>)->Result<()>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<i32>().map(|_|()).map_err(|_|invalid())})}
fn document_boolean(value:&str)->Result<()>{if !matches!(value,"0"|"1"){return Err(invalid())}Ok(())}
fn document_record(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,block:bool)->Result<()>{
 let[handle,layer,value]=native::record(value,control)?;let handle=native::hex_text_extent(handle,control)?;native::hex_text_extent(layer,control)?;
 let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let tag=match tag{"L"=>0,"A"=>1,"C"=>2,"E"=>3,"P"=>4,"T"=>5,"I"=>6,"S"=>7,"D"=>8,_=>return Err(invalid())};census.row(add(add(if block{40}else{32},handle)?,kind(tag)?)?)?;
 let bytes=match tag{
  0=>{let[a,b]=native::record(value,control)?;add(add(8,document_point(a,control)?)?,document_point(b,control)?)?},
  1=>{let[center,radius,start,end]=native::record(value,control)?;let mut bytes=add(8,document_point(center,control)?)?;for value in[radius,start,end]{bytes=add(bytes,document_float(value,control)?)?;}bytes},
  2=>{let[center,radius]=native::record(value,control)?;add(add(8,document_point(center,control)?)?,document_float(radius,control)?)?},
  3=>{let[center,major,ratio,start,end]=native::record(value,control)?;let mut bytes=add(add(8,document_point(center,control)?)?,document_point(major,control)?)?;for value in[ratio,start,end]{bytes=add(bytes,document_float(value,control)?)?;}bytes},
  4=>{let[vertices,closed]=native::record(value,control)?;document_list(vertices,census,control,|value,census,control|census.row(add(24,document_point(value,control)?)?))?;document_boolean(closed)?;16},
  5=>{let[position,height,rotation,content]=native::record(value,control)?;add(add(add(add(8,document_point(position,control)?)?,document_float(height,control)?)?,document_float(rotation,control)?)?,native::hex_text_extent(content,control)?)?},
  6=>{let[block,position,scale,rotation]=native::record(value,control)?;native::hex_text_extent(block,control)?;add(add(add(16,document_point(position,control)?)?,document_point(scale,control)?)?,document_float(rotation,control)?)?},
  7=>{let[p1,p2,p3,p4]=native::record(value,control)?;let mut bytes=8;for value in[p1,p2,p3,p4]{bytes=add(bytes,document_point(value,control)?)?;}bytes},
  8=>{let[point,position,measurement,text]=native::record(value,control)?;add(add(add(add(8,document_point(point,control)?)?,document_point(position,control)?)?,document_float(measurement,control)?)?,native::hex_text_extent(text,control)?)?},_=>return Err(invalid())
 };census.row(bytes)
}
/// 📝️ Counts every original CAD spelling, signed layer color and exact geometric class.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","layers","blocks","entities"],control)?;let mut census=Census{limits,rows:0,bytes:0};census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[name,color,line,visible]=native::record(value,control)?;let name=native::hex_text_extent(name,control)?;document_color(color,control)?;let line=native::hex_text_extent(line,control)?;document_boolean(visible)?;census.row(add(add(40,name)?,line)?)
  })?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{
   let[name,point,records]=native::record(value,control)?;census.row(add(add(24,native::hex_text_extent(name,control)?)?,document_point(point,control)?)?)?;document_list(records,census,control,|value,census,control|document_record(value,census,control,true))
  })?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|document_record(value,census,control,false))?;control.checkpoint()
 })
}
