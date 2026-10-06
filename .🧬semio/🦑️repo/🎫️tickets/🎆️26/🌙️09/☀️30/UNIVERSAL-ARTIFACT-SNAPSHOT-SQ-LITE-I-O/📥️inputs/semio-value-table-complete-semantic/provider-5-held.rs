//! 📊️ Borrowed native table declarations and every cell admit exact semantic rows before ownership.
use super::{SemioTableSnapshot,SqliteDatabaseLimits,ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use crate::standards::v1::subsets::value::io::sqlite::snapshot::semantic::{Census,add,hex_text,value_text,value_binary};
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::ArtifactSqliteSnapshot;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio table native primitive differs from its authored shape")}
/// 🏛️ Admits the own seven-table layout including all eight value columns.
pub(crate)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioTableSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_table_document","semio_table_column","semio_table_row","semio_table_cell","semio_table_value","semio_table_list_element","semio_table_map_entry"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioTableSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio table schema exceeds caller bytes"))}
 if limits.max_tables<7||limits.max_columns<8||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio table authored layout exceeds copied limits"))}Ok(())
}
fn kind(tag:u8)->Result<usize>{match tag{0=>Ok(4),1=>Ok(4),2=>Ok(3),3=>Ok(5),4=>Ok(3),5=>Ok(5),_=>Err(invalid())}}
/// 📦️ Counts binary columns and recursive typed cells without constraining row width.
pub(crate)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|_|invalid())?!=1{return Err(invalid())}let mut census=Census::new(limits);census.row(add(8,control.borrow_text(native::bytes(&mut reader)?)?.len())?)?;
  let count=native::length(&mut reader)?;census.future(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let bytes=add(24,control.borrow_text(native::bytes(&mut reader)?)?.len())?;census.row(add(bytes,kind(reader.read_u8().map_err(|_|invalid())?)?)?)?;control.step()?;}Ok::<_,ValueError>(())})?;
  let count=native::length(&mut reader)?;census.future(count)?;control.begin_stage(count)?;for _ in 0..count{
   census.row(24)?;let count=native::length(&mut reader)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{census.row(32)?;value_binary(&mut reader,&mut census,control,limits)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;
  }if reader.remaining()!=0{return Err(invalid())}control.checkpoint()
 })
}
/// 📃️ Counts optional declaration/row lists and all nested literal reference values.
pub(crate)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let fields=native::fields(body,["schema","columns","rows"],control)?;let mut census=Census::new(limits);census.row(add(8,hex_text(fields[0].ok_or_else(invalid)?,control)?)?)?;
  let mut columns=native::Items::new(fields[1].unwrap_or("[]"))?;let count=columns.count(control,limits.max_rows)?;census.future(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=columns.next(control)?{let[name,tag]=native::record(item,control)?;let size=kind(match tag{"n"=>0,"b"=>1,"i"=>2,"f"=>3,"s"=>4,"y"=>5,_=>return Err(invalid())})?;census.row(add(add(24,hex_text(name,control)?)?,size)?)?;control.step()?;}Ok::<_,ValueError>(())})?;
  let mut rows=native::Items::new(fields[2].unwrap_or("[]"))?;let count=rows.count(control,limits.max_rows)?;census.future(count)?;control.begin_stage(count)?;
  while let Some(row)=rows.next(control)?{
   census.row(24)?;let mut cells=native::Items::new(row)?;let count=cells.count(control,limits.max_rows)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;
   control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=cells.next(control)?{census.row(32)?;value_text(value,None,&mut census,control,limits)?;control.step()?;}Ok::<_,ValueError>(())})?;control.step()?;
  }control.checkpoint()
 })
}
