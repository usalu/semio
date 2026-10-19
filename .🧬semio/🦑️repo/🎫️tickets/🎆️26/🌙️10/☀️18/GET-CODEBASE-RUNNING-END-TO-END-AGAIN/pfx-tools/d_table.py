import dom
B='''/// 🧩️ Fills the retained table root from borrowed binary octets, inserting every column, row and cell before its fields.
pub fn binary_into(output:&mut SemioTableSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio table native format"))}native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.columns,&mut reader,control,limits,&mut entities,Default::default,|column,reader,control,_|{native::reader_text_into(&mut column.name,reader,control)?;column.kind=kind(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;Ok(())})?;
 native::binary_list_into(&mut output.rows,&mut reader,control,limits,&mut entities,Default::default,|row,reader,control,entities|{
  let count=native::length(reader)?;native::entities(entities,count,limits)?;if count>limits.max_rows.saturating_sub(*entities){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio table native cells exceed remaining row limit"))}row.cells=control.allocate_vec::<SemioValue>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{row.cells.push(SemioValue::Null);values::value_binary_into(row.cells.last_mut().unwrap(),reader,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})
 })?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio table native trailing bytes"))}Ok(())
}
'''
D='''/// 📝️ Fills the retained table root from the borrowed document body, inserting every column, row and cell before its fields.
pub fn document_into(output:&mut SemioTableSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","columns","rows"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio table native schema missing"))?,control)?;
 native::text_list_into(&mut output.columns,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|column,item,control,_|{let[name,tag]=native::record(item,control)?;let tag=match tag{"n"=>0,"b"=>1,"i"=>2,"f"=>3,"s"=>4,"y"=>5,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native column kind"))};native::hex_text_into(&mut column.name,name,control)?;column.kind=kind(tag)?;Ok(())})?;
 native::text_list_into(&mut output.rows,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|row,item,control,entities|{
  let mut cell_items=native::Items::new(item)?;let count=cell_items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;if count>limits.max_rows.saturating_sub(*entities){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio table native cells exceed remaining row limit"))}row.cells=control.allocate_vec::<SemioValue>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=cell_items.next(control)?{row.cells.push(SemioValue::Null);values::value_text_into(row.cells.last_mut().unwrap(),item,control,limits,entities)?;control.step()?;}Ok::<_,ValueError>(())})
 })
}
'''
print(dom.apply("📊️table",B,D))
print(dom.fmt("📊️table"))
