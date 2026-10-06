//! 🕸️ Borrowed Graph primitives count exact coordinates, property forests and literal external value references.
use super::{SemioGraphSnapshot,ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_value::native_decoding::NativeDecodeControl;
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use crate::standards::v1::subsets::value::io::sqlite::snapshot::semantic as values;
use values::{Census,add};
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio Graph native field differs from its authored shape")}
/// 🏛️ Admits all ten exact Graph and nested Value tables before ownership.
pub(super)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioGraphSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_graph_document","semio_graph_node","semio_graph_port","semio_graph_property","semio_graph_port_property","semio_graph_edge","semio_graph_edge_property","semio_graph_value","semio_graph_list_element","semio_graph_map_entry"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioGraphSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio Graph schema exceeds caller bytes"))}
 if limits.max_tables<10||limits.max_columns<18||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio Graph authored layout exceeds copied limits"))}Ok(())
}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{Ok(control.borrow_text(native::bytes(reader)?)?.len())}
fn scalar(value:f64)->usize{if value.is_nan(){11}else if value.is_infinite(){32}else{22}}
fn binary_scalar(reader:&mut store::ByteReader<'_>)->Result<usize>{Ok(scalar(reader.read_f64_le().map_err(|_|invalid())?))}
fn binary_optional(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<usize>{match reader.read_u8().map_err(|_|invalid())?{0=>Ok(0),1=>text(reader,control),_=>Err(invalid())}}
fn binary_properties(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 let count=native::length(reader)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{census.row(add(32,text(reader,control)?)?)?;values::value_binary(reader,census,control,limits)?;control.step()?;}control.checkpoint()})
}
struct Hex<'a>{bytes:&'a[u8],at:usize}
impl Hex<'_>{
 fn peek(&self)->Option<u8>{if self.at==self.bytes.len(){None}else{let digit=|v:u8|match v{b'0'..=b'9'=>v-b'0',b'a'..=b'f'=>v-b'a'+10,b'A'..=b'F'=>v-b'A'+10,_=>unreachable!()};Some((digit(self.bytes[self.at])<<4)|digit(self.bytes[self.at+1]))}}
 fn take(&mut self,control:&mut NativeDecodeControl<'_>)->Result<u8>{let value=self.peek().ok_or_else(invalid)?;self.at+=2;control.step()?;Ok(value)}
}
fn decimal_digit(value:u8,count:&mut usize,first:&mut Option<usize>,kept:&mut[u8;309]){
 if value!=b'0'&&first.is_none(){*first=Some(*count)}if let Some(first)=*first{let at=*count-first;if at<kept.len(){kept[at]=value}}*count+=1;
}
/// 🔣️ Classifies the entire original decimal grammar with fixed scratch and exact overflow midpoint.
fn hex_scalar(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 let length=native::hex_text_extent(value,control)?;
 control.scoped_stage(|control|{
  control.begin_stage(length)?;let mut input=Hex{bytes:value.as_bytes(),at:0};
  if length<=32{let mut word=[0u8;32];for byte in &mut word[..length]{*byte=input.take(control)?;}let word=std::str::from_utf8(&word[..length]).map_err(|_|invalid())?;return Ok(scalar(native::float(word,control)?))}
  if matches!(input.peek(),Some(b'+'|b'-')){input.take(control)?;}
  let mut count=0usize;let mut first=None;let mut kept=[b'0';309];
  while input.peek().is_some_and(|byte|byte.is_ascii_digit()){decimal_digit(input.take(control)?,&mut count,&mut first,&mut kept);}
  let whole=count;
  if input.peek()==Some(b'.'){input.take(control)?;while input.peek().is_some_and(|byte|byte.is_ascii_digit()){decimal_digit(input.take(control)?,&mut count,&mut first,&mut kept);}}
  if count==0{return Err(invalid())}
  let mut exponent=0i128;let mut negative=false;
  if matches!(input.peek(),Some(b'e'|b'E')){
   input.take(control)?;if matches!(input.peek(),Some(b'+'|b'-')){negative=input.take(control)?==b'-';}
   let mut digits=0usize;let cap=length as i128+400;
   while input.peek().is_some_and(|byte|byte.is_ascii_digit()){exponent=(exponent*10+i128::from(input.take(control)?-b'0')).min(cap);digits+=1;}
   if digits==0{return Err(invalid())}
  }
  if input.peek().is_some(){return Err(invalid())}control.checkpoint()?;
  let Some(first)=first else{return Ok(22)};let order=whole as i128-first as i128-1+if negative{-exponent}else{exponent};
  if order<308{return Ok(22)}if order>308{return Ok(32)}
  const MIDPOINT:&[u8;309]=b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497792";
  Ok(if kept.as_slice()<MIDPOINT.as_slice(){22}else{32})
 })
}
/// 📦️ Counts exact four-coordinate words and all nested property entities in actual binary order.
pub(super)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}
  let mut census=Census::new(limits);census.row(add(8,text(&mut reader,control)?)?)?;let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{
   let mut bytes=add(add(add(24,text(&mut reader,control)?)?,text(&mut reader,control)?)?,text(&mut reader,control)?)?;for _ in 0..4{bytes=add(bytes,binary_scalar(&mut reader)?)?;}census.row(bytes)?;
   let count=native::length(&mut reader)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let name=text(&mut reader,control)?;let kind=match reader.read_u8().map_err(|_|invalid())?{0=>2,1=>3,2=>6,_=>return Err(invalid())};census.row(add(add(add(24,name)?,kind)?,text(&mut reader,control)?)?)?;binary_properties(&mut reader,&mut census,control,limits)?;control.step()?;}control.checkpoint()})?;
   binary_properties(&mut reader,&mut census,control,limits)?;control.step()?;
  }
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;
  for _ in 0..count{
   let id=text(&mut reader,control)?;text(&mut reader,control)?;text(&mut reader,control)?;let kind=text(&mut reader,control)?;let label=text(&mut reader,control)?;let source_port=binary_optional(&mut reader,control)?;let target_port=binary_optional(&mut reader,control)?;
   census.row(add(add(add(add(add(40,id)?,kind)?,label)?,source_port)?,target_port)?)?;binary_properties(&mut reader,&mut census,control,limits)?;control.step()?;
  }if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
fn document_optional(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{if value=="-"{Ok(0)}else{let[value]=native::record(value,control)?;native::hex_text_extent(value,control)}}
fn document_properties(body:&str,census:&mut Census,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 let mut items=native::Items::new(body)?;let count=items.count(control,limits.max_rows)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=items.next(control)?{let(key,value)=value.split_once(':').ok_or_else(invalid)?;census.row(add(32,native::hex_text_extent(key,control)?)?)?;values::value_text(value,None,census,control,limits)?;control.step()?;}control.checkpoint()})
}
/// 📝️ Preserves actual hex-encoded numbers, duplicate property keys and external reference text.
pub(super)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","nodes","edges"],control)?;let mut census=Census::new(limits);census.row(add(8,native::hex_text_extent(fields[0].ok_or_else(invalid)?,control)?)?)?;let mut nodes=native::Items::new(fields[1].unwrap_or("[]"))?;let count=nodes.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=nodes.next(control)?{
   let[id,kind,label,x,y,width,height,ports,properties]=native::record(value,control)?;let mut bytes=add(add(add(24,native::hex_text_extent(id,control)?)?,native::hex_text_extent(kind,control)?)?,native::hex_text_extent(label,control)?)?;for value in[x,y,width,height]{bytes=add(bytes,hex_scalar(value,control)?)?;}census.row(bytes)?;
   let mut ports=native::Items::new(ports)?;let count=ports.count(control,limits.max_rows)?;census.future(count)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=ports.next(control)?{let[name,kind,category,properties]=native::record(value,control)?;let kind=match kind{"i"=>2,"o"=>3,"x"=>6,_=>return Err(invalid())};census.row(add(add(add(24,native::hex_text_extent(name,control)?)?,kind)?,native::hex_text_extent(category,control)?)?)?;document_properties(properties,&mut census,control,limits)?;control.step()?;}control.checkpoint()})?;
   document_properties(properties,&mut census,control,limits)?;control.step()?;
  }
  let mut edges=native::Items::new(fields[2].unwrap_or("[]"))?;let count=edges.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(value)=edges.next(control)?{
   let[id,source,target,kind,label,source_port,target_port,properties]=native::record(value,control)?;native::hex_text_extent(source,control)?;native::hex_text_extent(target,control)?;
   census.row(add(add(add(add(add(40,native::hex_text_extent(id,control)?)?,native::hex_text_extent(kind,control)?)?,native::hex_text_extent(label,control)?)?,document_optional(source_port,control)?)?,document_optional(target_port,control)?)?)?;
   document_properties(properties,&mut census,control,limits)?;control.step()?;
  }control.checkpoint()
 })
}
