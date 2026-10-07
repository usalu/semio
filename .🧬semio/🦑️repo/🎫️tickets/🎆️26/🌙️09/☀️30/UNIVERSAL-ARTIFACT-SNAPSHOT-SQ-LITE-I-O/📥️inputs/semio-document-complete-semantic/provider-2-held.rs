//! 📑️ Borrowed Document primitives count every authored collection, block and optional exact cell.
use super::{SemioDocumentSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Document native field differs from its authored shape")}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Document complete cell extent overflow"))}
/// 📑️ Admits the complete declared relational document layout before native ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioDocumentSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_document_document","semio_document_style","semio_document_image","semio_document_collection","semio_document_member","semio_document_block","semio_document_paragraph","semio_document_heading","semio_document_run","semio_document_list","semio_document_list_item","semio_document_table","semio_document_table_row","semio_document_table_cell","semio_document_code","semio_document_quote","semio_document_image_block","semio_document_page_break"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioDocumentSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Document schema exceeds caller bytes"))}
 if limits.max_tables<18||limits.max_columns<13||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Document authored layout exceeds copied limits"))}Ok(())
}
/// 🧮️ Shared cell accounting for the actual Document blocks embedded in Presentation.
pub(crate)struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 pub(crate)fn new(limits:SqliteDatabaseLimits)->Self{Self{limits,rows:0,bytes:0}}
 /// 🎛️ Copies the authored row ceiling into a sibling borrowed list preflight.
 pub(crate)fn max_rows(&self)->usize{self.limits.max_rows}
 pub(crate)fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Document complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Document complete cells exceed caller bytes"))}Ok(())}
 pub(crate)fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Document declared rows exceed caller limit"))}Ok(())}
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn byte(reader:&mut store::ByteReader<'_>)->Result<u8>{reader.read_u8().map_err(|_|invalid())}
fn boolean(value:u8)->Result<()>{if value>1{return Err(invalid())}Ok(())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_float(reader:&mut store::ByteReader<'_>)->Result<usize>{Ok(scalar(reader.read_f64_le().map_err(|_|invalid())?))}
fn binary_optional(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,read:impl FnOnce(&mut store::ByteReader<'_>,&mut NativeDecodeControl<'_>)->Result<usize>)->Result<usize>{match byte(reader)?{0=>Ok(0),1=>read(reader,control),_=>Err(invalid())}}
fn binary_reference(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,references:bool)->Result<usize>{let bytes=text(reader,control)?;Ok(if references{8}else{bytes})}
fn binary_list(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&mut store::ByteReader<'_>,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let count=native::length(reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{visit(reader,census,control)?;control.step()?;}control.checkpoint()})
}
fn binary_run(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let mut bytes=add(48,text(reader,control)?)?;for _ in 0..3{boolean(byte(reader)?)?;}bytes=add(bytes,binary_optional(reader,control,|reader,_|binary_float(reader))?)?;for _ in 0..3{bytes=add(bytes,binary_optional(reader,control,text)?)?;}census.row(bytes)
}
/// 📦️ Counts actual shared binary DocBlock collections before constructing any owned block.
pub(crate)fn binary_blocks(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,references:bool)->Result<()>{
 census.row(8)?;binary_list(reader,census,control,|reader,census,control|control.scoped_depth(64,|control|{
  let tag=byte(reader)?;let kind=match tag{0=>9,1=>7,2|4=>4,3|5|6=>5,7=>9,_=>return Err(invalid())};census.row(add(8,kind)?)?;census.row(32)?;
  match tag{
   0|1=>{let mut bytes=8;if tag==1{byte(reader)?;bytes=add(bytes,8)?;}bytes=add(bytes,binary_optional(reader,control,|reader,control|binary_reference(reader,control,references))?)?;census.row(bytes)?;binary_list(reader,census,control,binary_run)?;},
   2=>{boolean(byte(reader)?)?;census.row(16)?;binary_list(reader,census,control,|reader,census,control|{census.row(32)?;binary_blocks(reader,census,control,references)})?;},
   3=>{census.row(8)?;binary_list(reader,census,control,|reader,census,control|{census.row(24)?;binary_list(reader,census,control,|reader,census,control|{census.row(32)?;binary_blocks(reader,census,control,references)})})?;},
   4=>{let language=binary_optional(reader,control,text)?;census.row(add(add(8,language)?,text(reader,control)?)?)?;},
   5=>{census.row(16)?;binary_blocks(reader,census,control,references)?;},
   6=>{let mut bytes=add(add(8,binary_reference(reader,control,references)?)?,text(reader,control)?)?;for _ in 0..2{bytes=add(bytes,binary_optional(reader,control,|reader,_|binary_float(reader))?)?;}census.row(bytes)?;},
   7=>census.row(8)?,_=>return Err(invalid())
  }Ok(())
 }))
}
pub(crate)fn binary_blob(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{let value=native::bytes(reader)?;control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value{control.step()?;}control.checkpoint()?;Ok(value.len())})}
/// 🗃️ Counts document identity, style inheritance, image bytes and the exact root collection.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if byte(&mut reader)?!=1{return Err(invalid())}let mut census=Census::new(limits);census.row(add(16,text(&mut reader,control)?)?)?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|{let mut bytes=add(add(24,text(reader,control)?)?,text(reader,control)?)?;bytes=add(bytes,binary_optional(reader,control,|reader,control|binary_reference(reader,control,true))?)?;census.row(bytes)})?;
  binary_list(&mut reader,&mut census,control,|reader,census,control|census.row(add(add(add(24,text(reader,control)?)?,text(reader,control)?)?,binary_blob(reader,control)?)?))?;
  binary_blocks(&mut reader,&mut census,control,true)?;if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_list(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,mut visit:impl FnMut(&str,&mut Census,&mut NativeDecodeControl<'_>)->Result<()>)->Result<()>{
 let mut items=native::Items::new(value)?;let count=items.count(control,census.limits.max_rows)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{visit(value,census,control)?;control.step()?;}control.checkpoint()})
}
fn document_optional(value:&str,control:&mut NativeDecodeControl<'_>,read:impl FnOnce(&str,&mut NativeDecodeControl<'_>)->Result<usize>)->Result<usize>{if value=="[0]"{return Ok(0)}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(invalid())}read(value,control)}
fn document_reference(value:&str,control:&mut NativeDecodeControl<'_>,references:bool)->Result<usize>{let bytes=native::hex_text_extent(value,control)?;Ok(if references{8}else{bytes})}
pub(crate)fn document_float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}let bits=value.parse::<u64>().map_err(|_|invalid())?;Ok(scalar(f64::from_bits(bits)))})}
fn document_boolean(value:&str)->Result<()>{if !matches!(value,"0"|"1"){return Err(invalid())}Ok(())}
fn document_run(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>)->Result<()>{
 let[text,style]=native::record(value,control)?;let[bold,italic,underline,size,font,color,link]=native::record(style,control)?;let mut bytes=add(48,native::hex_text_extent(text,control)?)?;for value in[bold,italic,underline]{document_boolean(value)?;}
 bytes=add(bytes,document_optional(size,control,document_float)?)?;for value in[font,color,link]{bytes=add(bytes,document_optional(value,control,native::hex_text_extent)?)?;}census.row(bytes)
}
/// 📝️ Counts original shared text blocks, preserving every optional decimal unsigned64 bit word.
pub(crate)fn document_blocks(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,references:bool)->Result<()>{
 census.row(8)?;document_list(value,census,control,|value,census,control|document_block(value,census,control,references))
}
/// 🧩️ Counts one actual text DocBlock stored as a borrowed binary Presentation record.
pub(crate)fn document_block(value:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,references:bool)->Result<()>{
 control.scoped_depth(64,|control|{
  let(tag,value)=value.split_at_checked(1).ok_or_else(invalid)?;let kind=match tag{"P"=>9,"H"=>7,"L"|"C"=>4,"T"|"Q"|"I"=>5,"B"=>9,_=>return Err(invalid())};census.row(add(8,kind)?)?;census.row(32)?;
  match tag{
   "P"|"H"=>{let(style,runs,base)=if tag=="P"{let[style,runs]=native::record(value,control)?;(style,runs,8)}else{let[level,style,runs]=native::record(value,control)?;control.scoped_stage(|control|{control.begin_stage(level.len())?;for _ in level.bytes(){control.step()?;}level.parse::<u8>().map_err(|_|invalid())})?;(style,runs,16)};
    census.row(add(base,document_optional(style,control,|value,control|document_reference(value,control,references))?)?)?;document_list(runs,census,control,document_run)?;},
   "L"=>{let[ordered,items]=native::record(value,control)?;document_boolean(ordered)?;census.row(16)?;document_list(items,census,control,|value,census,control|{let[blocks]=native::record(value,control)?;census.row(32)?;document_blocks(blocks,census,control,references)})?;},
   "T"=>{let[rows]=native::record(value,control)?;census.row(8)?;document_list(rows,census,control,|value,census,control|{let[cells]=native::record(value,control)?;census.row(24)?;document_list(cells,census,control,|value,census,control|{let[blocks]=native::record(value,control)?;census.row(32)?;document_blocks(blocks,census,control,references)})})?;},
   "C"=>{let[language,text]=native::record(value,control)?;census.row(add(add(8,document_optional(language,control,native::hex_text_extent)?)?,native::hex_text_extent(text,control)?)?)?;},
   "Q"=>{let[blocks]=native::record(value,control)?;census.row(16)?;document_blocks(blocks,census,control,references)?;},
   "I"=>{let[image,alt,width,height]=native::record(value,control)?;census.row(add(add(add(add(8,document_reference(image,control,references)?)?,native::hex_text_extent(alt,control)?)?,document_optional(width,control,document_float)?)?,document_optional(height,control,document_float)?)?)?;},
   "B"=>{let[]=native::record::<0>(value,control)?;census.row(8)?;},_=>return Err(invalid())
  }Ok(())
 })
}
pub(crate)fn document_blob(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 if !value.len().is_multiple_of(2){return Err(invalid())}control.scoped_stage(|control|{control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){for byte in pair{if !byte.is_ascii_hexdigit(){return Err(invalid())}}control.step()?;}control.checkpoint()?;Ok(value.len()/2)})
}
/// 🏷️ Counts all original top-level document fields without forecasting typed owned values.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","styles","images","blocks"],control)?;let mut census=Census::new(limits);census.row(add(16,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;
  document_list(fields[1].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,name,parent]=native::record(value,control)?;census.row(add(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(name,control)?)?,document_optional(parent,control,|value,control|document_reference(value,control,true))?)?)?;})?;
  document_list(fields[2].unwrap_or("[]"),&mut census,control,|value,census,control|{let[id,mime,blob]=native::record(value,control)?;census.row(add(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(mime,control)?)?,document_blob(blob,control)?)?)?;})?;
  document_blocks(fields[3].unwrap_or("[]"),&mut census,control,true)?;control.checkpoint()
 })
}
