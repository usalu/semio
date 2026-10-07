//! 📽️ Borrowed Presentation primitives count real owner partitions, frames and embedded DocBlocks.
use super::{SemioPresentationSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use crate::standards::v1::subsets::document::io::sqlite::snapshot::semantic as doc;
type Result<T>=std::result::Result<T,ValueError>;
type Census=doc::Census;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Presentation native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Presentation complete cell extent overflow"))}
/// 🏛️ Admits all independently authored Presentation and embedded document tables before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioPresentationSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_presentation_document","semio_presentation_master","semio_presentation_layout","semio_presentation_slide","semio_presentation_shape","semio_presentation_text_box","semio_presentation_picture","semio_presentation_shape_table","semio_presentation_shape_table_row","semio_presentation_shape_table_cell","semio_presentation_placeholder","semio_presentation_collection","semio_presentation_member","semio_presentation_block","semio_presentation_paragraph","semio_presentation_heading","semio_presentation_run","semio_presentation_list","semio_presentation_list_item","semio_presentation_table","semio_presentation_table_row","semio_presentation_table_cell","semio_presentation_code","semio_presentation_quote","semio_presentation_image_block","semio_presentation_page_break"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioPresentationSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Presentation schema exceeds caller bytes"))}
 if limits.max_tables<26||limits.max_columns<18||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Presentation authored layout exceeds copied limits"))}Ok(())
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn placeholder(tag:u8)->Result<usize>{match tag{0|6=>Ok(5),1|5=>Ok(8),2=>Ok(4),3=>Ok(6),4=>Ok(11),_=>Err(invalid())}}
fn shape(tag:u8)->Result<usize>{match tag{0|1=>Ok(7),2=>Ok(5),3=>Ok(11),_=>Err(invalid())}}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_optional(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{match byte(reader)?{0=>Ok(0),1=>{text(reader,control)?;Ok(8)},_=>Err(invalid())}}
fn binary_blocks(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 census.row(8)?;binary_list(reader,census,control,|reader,census,control|{let body=control.borrow_text(native::bytes(reader)?)?;doc::document_block(body,census,control,false)})
}
fn binary_shapes(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 binary_list(reader,census,control,|reader,census,control|{
  let tag=byte(reader)?;let mut bytes=24;for _ in 0..4{bytes=add(bytes,scalar(reader.read_f64_le().map_err(|_|invalid())?))?;}census.row(add(bytes,shape(tag)?)?)?;
  match tag{
   0=>{census.row(16)?;binary_blocks(reader,census,control)?;},
   1=>census.row(add(add(add(8,text(reader,control)?)?,text(reader,control)?)?,doc::binary_blob(reader,control)?)?)?,
   2=>{census.row(8)?;binary_list(reader,census,control,|reader,census,control|{census.row(24)?;binary_list(reader,census,control,|reader,census,control|{census.row(32)?;binary_blocks(reader,census,control)})})?;},
   3=>{let tag=byte(reader)?;let mut bytes=add(8,placeholder(tag)?)?;if tag==6{bytes=add(bytes,text(reader,control)?)?;}census.row(bytes)?;},_=>return Err(invalid())
  }Ok(())
 })
}
/// 📦️ Counts actual Presentation binary records, including borrowed text-encoded DocBlock leaves.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census::new(limits);census.row(add(8,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{census.row(add(24,text(reader,control)?)?)?;binary_shapes(reader,census,control)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let id=text(reader,control)?;text(reader,control)?;census.row(add(32,id)?)?;binary_shapes(reader,census,control)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let bytes=add(add(32,text(reader,control)?)?,binary_optional(reader,control)?)?;census.row(bytes)?;binary_shapes(reader,census,control)?;binary_blocks(reader,census,control)})?;
  if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.max_rows())?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_optional(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if value=="[0]"{return Ok(0)}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(invalid())}native::hex_text_extent(value,control)?;Ok(8)}
fn document_frame(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let[origin,width,height]=native::record(value,control)?;let[x,y]=native::record(origin,control)?;let mut bytes=0;for value in[x,y,width,height]{bytes=add(bytes,doc::document_float(value,control)?)?;}Ok(bytes)
}
fn document_placeholder(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let tag=match value{"T"=>0,"S"=>1,"B"=>2,"F"=>3,"N"=>4,"D"=>5,_=>6};let mut bytes=add(8,placeholder(tag)?)?;if tag==6{let[value]=native::record(value.strip_prefix('O').ok_or_else(invalid)?,control)?;bytes=add(bytes,native::hex_text_extent(value,control)?)?;}Ok(bytes)
}
fn document_shapes(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 document_list(value,census,control,|value,census,control|{
  let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let[frame,value]=native::record(value,control)?;let tag=match tag{"X"=>0,"P"=>1,"T"=>2,"H"=>3,_=>return Err(invalid())};census.row(add(add(24,shape(tag)?)?,document_frame(frame,control)?)?)?;
  match tag{
   0=>{census.row(16)?;doc::document_blocks(value,census,control,false)?;},
   1=>{let[asset,mime,blob]=native::record(value,control)?;census.row(add(add(add(8,native::hex_text_extent(asset,control)?)?,native::hex_text_extent(mime,control)?)?,doc::document_blob(blob,control)?)?)?;},
   2=>{census.row(8)?;document_list(value,census,control,|value,census,control|{census.row(24)?;document_list(value,census,control,|value,census,control|{census.row(32)?;doc::document_blocks(value,census,control,false)})})?;},
   3=>census.row(document_placeholder(value,control)?)?,_=>return Err(invalid())
  }Ok(())
 })
}
/// 📝️ Counts original Presentation field records and exact decimal unsigned64 frame words.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","masters","layouts","slides"],control)?;let mut census=Census::new(limits);census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,shapes]=native::record(value,control)?;census.row(add(24,native::hex_text_extent(id,control)?)?)?;document_shapes(shapes,census,control)})?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,master,shapes]=native::record(value,control)?;native::hex_text_extent(master,control)?;census.row(add(32,native::hex_text_extent(id,control)?)?)?;document_shapes(shapes,census,control)})?;
  document_list(fields[3].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,layout,shapes,notes]=native::record(value,control)?;census.row(add(add(32,native::hex_text_extent(id,control)?)?,document_optional(layout,control)?)?)?;document_shapes(shapes,census,control)?;doc::document_blocks(notes,census,control,false)})?;control.checkpoint()
 })
}
