//! 📏️ ISO catalogue native CST admits every authored SQL cell before typed ownership.
use super::*;
use semio_framework_dsl_record::{DslField,FieldValue as F,RecordValue as R};
use semio_framework_value::{DslValue as D,NativeDecodeControl,Number};
type Result<T>=std::result::Result<T,ValueError>;
const WIDTHS:&[(&str,usize)]=&[
("document",26),("names",4),("name_alternative",5),("unit",9),("catalogue_value",18),("value_element",4),
("product_group",6),("product_class",7),("class_required_property",4),("class_optional_property",4),
("product_series",7),("series_property",5),("product",6),("parameter_domain",5),("domain_allowed_value",4),
("product_variant",6),("variant_parameter",5),("property_value",4),("variant_property",4),("product_static_property",4),
("product_index",6),("index_tag",4),("property_definition",11),("accessory_group",4),("accessory",8),
("composition_group",4),("composition",5),("descriptive_object",8),("dictionary_subject",10),("dictionary_relationship",9),
("dictionary_property",8),("property_subject",4),("value_constraint",9),("constraint_allowed",4),("controlled_list",4),
("controlled_value",4),("controlled_context",4),("geometry_object",7),("geometry_node",23),("geometry_child",4),
("primitive_parameter",7),("space",23),("surface",23),("port",24),("geometry_binding",5),("primitive_kind",4),
("primitive_kind_parameter",4),("selection_constraint",7),("part_number_row",3),("part_number_cell",5),
("part_number_input",5),("extension_entry",5),("extension_value",11),("extension_element",4),("extension_member",5)];
fn sum(values:&[usize])->Result<usize>{values.iter().try_fold(0usize,|total,n|total.checked_add(*n).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ISO semantic cell extent overflow")))}
struct Census{limits:store::sqlite_snapshot::SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,cells:&[usize],native:&mut NativeDecodeControl<'_>)->Result<()>{
  native.step()?;let width=WIDTHS.iter().find(|entry|entry.0==table).ok_or_else(||invalid("ISO native census table is not authored"))?.1;
  if width>self.limits.max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ISO authored row exceeds caller column limit"))}
  let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"ISO semantic row count overflow"))?;
  if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ISO semantic rows exceed caller row limit"))}
  let bytes=sum(&[self.bytes,8,sum(cells)?])?;
  if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"ISO semantic cells exceed caller value limit"))}
  self.rows=rows;self.bytes=bytes;Ok(())
 }
}
fn required(v:Option<&F>)->Result<&F>{v.ok_or_else(||invalid("ISO required native role is absent"))}
fn record(v:Option<&F>)->Result<&R>{match required(v)?{F::Record(v)=>Ok(v),_=>Err(invalid("ISO native role requires record"))}}
fn text(v:Option<&F>)->Result<&str>{match required(v)?{F::Text(v)=>Ok(v),_=>Err(invalid("ISO native role requires text"))}}
fn length(r:&R,id:u16)->Result<usize>{text(r.get(id)).map(str::len)}
fn optional(v:Option<&F>)->Result<Option<&F>>{let value=required(v)?;Ok(if matches!(value,F::Absent){None}else{Some(value)})}
fn optional_length(r:&R,id:u16)->Result<usize>{optional(r.get(id))?.map(|v|text(Some(v)).map(str::len)).transpose().map(|v|v.unwrap_or(0))}
fn list(v:Option<&F>)->Result<&[F]>{match required(v)?{F::List(v)=>Ok(v),_=>Err(invalid("ISO native role requires list"))}}
fn mapping(v:Option<&F>)->Result<&[(String,F)]>{match required(v)?{F::Map(v)=>Ok(v),_=>Err(invalid("ISO native role requires map"))}}
fn scalar<T:DslField>(v:Option<&F>,native:&mut NativeDecodeControl<'_>)->Result<T>{native.scoped_stage(|native|{native.begin_stage(0)?;T::from_value_controlled(required(v)?,native)})}
fn integer<T:DslField>(r:&R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<usize>{let _=scalar::<T>(r.get(id),native)?;Ok(8)}
fn optional_u32(r:&R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<usize>{if optional(r.get(id))?.is_none(){Ok(0)}else{integer::<u32>(r,id,native)}}
fn float_size(v:f64)->usize{8+if v.is_nan(){3}else if v.is_infinite(){24}else{14}}
fn float(r:&R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<usize>{scalar::<f64>(r.get(id),native).map(float_size)}
fn optional_float(r:&R,id:u16,native:&mut NativeDecodeControl<'_>)->Result<usize>{if optional(r.get(id))?.is_none(){Ok(0)}else{float(r,id,native)}}
fn array_float(v:Option<&F>,native:&mut NativeDecodeControl<'_>)->Result<usize>{let F::Tuple(values)=required(v)? else{return Err(invalid("ISO native vector requires tuple"))};if values.len()!=3{return Err(invalid("ISO native vector requires three scalars"))}let mut bytes=0;for value in values{bytes=sum(&[bytes,float_size(scalar::<f64>(Some(value),native)?)])?;}Ok(bytes)}
fn names(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 let preferred=record(r.get(0))?;c.row("names",&[length(preferred,0)?,length(preferred,1)?,optional_length(r,1)?],native)?;
 for value in list(r.get(2))?{let value=record(Some(value))?;c.row("name_alternative",&[16,length(value,0)?,length(value,1)?],native)?;}Ok(())
}
fn unit(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 let dimensions=record(r.get(1))?;let mut count=0;for id in 0..4{count=sum(&[count,integer::<i8>(dimensions,id,native)?])?;}
 c.row("unit",&[count,length(r,0)?,float(r,2,native)?],native)
}
fn optional_unit(v:Option<&F>,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<usize>{match optional(v)?{None=>Ok(0),Some(v)=>{unit(record(Some(v))?,c,native)?;Ok(8)}}}
fn intrinsic(v:Option<&F>)->Result<&D>{match required(v)?{F::Value(v)=>Ok(v),_=>Err(invalid("ISO intrinsic SQL role requires value"))}}
fn object(v:&D)->Result<&[(String,D)]>{match v{D::Object(v)=>Ok(v),_=>Err(invalid("ISO intrinsic role requires object"))}}
fn optional_member<'a>(v:&'a D,key:&str,native:&mut NativeDecodeControl<'_>)->Result<Option<&'a D>>{
 let mut found=None;for(name,value)in object(v)?{native.step()?;if name==key{if found.is_some(){return Err(invalid("ISO intrinsic role is duplicated"))}found=Some(value);}}
 Ok(found)
}
fn member<'a>(v:&'a D,key:&str,native:&mut NativeDecodeControl<'_>)->Result<&'a D>{optional_member(v,key,native)?.ok_or_else(||invalid("ISO intrinsic role is absent"))}
fn intrinsic_text(v:&D)->Result<&str>{match v{D::String(v)=>Ok(v),_=>Err(invalid("ISO intrinsic role requires text"))}}
fn intrinsic_array(v:&D)->Result<&[D]>{match v{D::Array(v)=>Ok(v),_=>Err(invalid("ISO intrinsic role requires array"))}}
fn intrinsic_float(v:&D)->Result<f64>{match v{D::Number(v)=>Ok(v.as_f64()),_=>Err(invalid("ISO intrinsic role requires number"))}}
fn intrinsic_unit(v:&D,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 let dimensions=member(v,"dimension",native)?;for key in ["length","mass","time","temperature"]{let D::Number(value)=member(dimensions,key,native)? else{return Err(invalid("ISO intrinsic dimension requires number"))};let value=value.as_i64().ok_or_else(||invalid("ISO intrinsic dimension requires integer"))?;let _=i8::try_from(value).map_err(|_|invalid("ISO intrinsic dimension exceeds signed8"))?;}
 c.row("unit",&[32,intrinsic_text(member(v,"symbol",native)?)?.len(),float_size(intrinsic_float(member(v,"siFactor",native)?)?)],native)
}
fn catalogue_value(v:&D,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 native.scoped_depth(64,|native|{
  let kind=intrinsic_text(member(v,"kind",native)?)?;
  let (tag,bytes)=match kind{
   "boolean"=>{if !matches!(member(v,"value",native)?,D::Bool(_)){return Err(invalid("ISO intrinsic Boolean requires boolean"))}("Boolean",8)},
   "integer"=>{let D::Number(value)=member(v,"value",native)? else{return Err(invalid("ISO intrinsic Integer requires number"))};let _=value.as_i64().ok_or_else(||invalid("ISO intrinsic Integer requires exact signed64"))?;("Integer",8)},
   "decimal"=>("Decimal",float_size(intrinsic_float(member(v,"value",native)?)?)),
   "text"=>("Text",intrinsic_text(member(v,"value",native)?)?.len()),
   "identifier"=>("Identifier",intrinsic_text(member(v,"value",native)?)?.len()),
   "enumeration"=>("Enumeration",intrinsic_text(member(v,"value",native)?)?.len()),
   "controlled"=>("Controlled",sum(&[intrinsic_text(member(v,"value",native)?)?.len(),intrinsic_text(member(v,"list_id",native)?)?.len()])?),
   "quantity"=>{intrinsic_unit(member(v,"unit",native)?,c,native)?;("Quantity",sum(&[8,float_size(intrinsic_float(member(v,"value",native)?)?)])?)},
   "range"=>{let unit=match optional_member(v,"unit",native)?{None|Some(D::Null)=>0,Some(unit)=>{intrinsic_unit(unit,c,native)?;8}};("Range",sum(&[unit,float_size(intrinsic_float(member(v,"min",native)?)?),float_size(intrinsic_float(member(v,"max",native)?)?)])?)},
   "null"=>{let state=intrinsic_text(member(v,"state",native)?)?;if !["Unavailable","Unknown","NotApplicable"].contains(&state){return Err(invalid("ISO intrinsic Null state is unknown"))}("Null",state.len())},
   "reference"=>("Reference",intrinsic_text(member(v,"target_id",native)?)?.len()),
   "list"=>("List",0),
   _=>return Err(invalid("ISO catalogue native variant is unknown"))
  };
  c.row("catalogue_value",&[tag.len(),bytes],native)?;
  if kind=="list"{for value in intrinsic_array(member(v,"items",native)?)?{c.row("value_element",&[24],native)?;catalogue_value(value,c,native)?;}}Ok(())
 })
}
fn property(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 catalogue_value(intrinsic(r.get(1))?,c,native)?;c.row("property_value",&[8,length(r,0)?,optional_length(r,2)?],native)
}
fn extension(v:&D,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 native.scoped_depth(64,|native|{
  let (tag,bytes)=match v{D::Null=>("Null",0),D::Bool(_)=>("Boolean",8),D::Number(Number::Int(_))=>("Integer",8),D::Number(Number::UInt(_))=>("Unsigned",16),D::Number(Number::Float(v))=>("Float",float_size(*v)),D::String(v)=>("Text",v.len()),D::Bytes(v)=>("Bytes",v.len()),D::Array(_)=>("Array",0),D::Object(_)=>("Object",0)};
  c.row("extension_value",&[tag.len(),bytes],native)?;
  match v{D::Array(values)=>for value in values{c.row("extension_element",&[24],native)?;extension(value,c,native)?;},D::Object(values)=>for(key,value)in values{c.row("extension_member",&[24,key.len()],native)?;extension(value,c,native)?;},_=>{}}Ok(())
 })
}
fn statements(v:Option<&F>)->Result<&[(String,R)]>{let F::Block(v)=required(v)? else{return Err(invalid("ISO geometry SQL role requires statement block"))};let F::Statements(v)=v.as_ref() else{return Err(invalid("ISO geometry block requires tagged statements"))};Ok(v)}
fn geometry(tag:&str,r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 native.scoped_depth(64,|native|{
  match tag{
   "primitive"=>{c.row("geometry_node",&[9,length(r,0)?],native)?;for(key,value)in mapping(r.get(1))?{c.row("primitive_parameter",&[16,key.len(),float_size(scalar::<f64>(Some(value),native)?)],native)?;}},
   "transform"=>{c.row("geometry_node",&[9,array_float(r.get(0),native)?,array_float(r.get(1),native)?],native)?;let children=statements(r.get(2))?;if children.len()!=1{return Err(invalid("ISO Transform requires one owned child"))}c.row("geometry_child",&[24],native)?;geometry(&children[0].0,&children[0].1,c,native)?;},
   "boolean"=>{let operator=boolean_operator(scalar::<part_2::BooleanOperator>(r.get(0),native)?);c.row("geometry_node",&[7,operator.len()],native)?;for(tag,row)in statements(r.get(1))?{c.row("geometry_child",&[24],native)?;geometry(tag,row,c,native)?;}},
   "reference"=>c.row("geometry_node",&[9,length(r,0)?],native)?,
   _=>return Err(invalid("ISO native geometry variant is unknown"))
  }Ok(())
 })
}
fn optional_geometry(v:Option<&F>,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<usize>{
 if matches!(required(v)?,F::Absent){return Ok(0)}let values=statements(v)?;match values{[]=>Ok(0),[(tag,row)]=>{geometry(tag,row,c,native)?;Ok(8)},_=>Err(invalid("ISO optional geometry requires at most one owner"))}
}
fn catalogue(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 for value in list(r.get(4))?{let v=record(Some(value))?;names(record(v.get(1))?,c,native)?;c.row("product_group",&[24,length(v,0)?,optional_length(v,2)?],native)?;}
 for value in list(r.get(5))?{let v=record(Some(value))?;names(record(v.get(3))?,c,native)?;c.row("product_class",&[24,length(v,0)?,length(v,1)?,optional_length(v,2)?],native)?;for(id,table)in[(4,"class_required_property"),(5,"class_optional_property")]{for value in list(v.get(id))?{c.row(table,&[16,text(Some(value))?.len()],native)?;}}}
 for value in list(r.get(6))?{let v=record(Some(value))?;names(record(v.get(2))?,c,native)?;c.row("product_series",&[24,length(v,0)?,length(v,1)?,optional_length(v,4)?],native)?;for(key,value)in mapping(v.get(3))?{catalogue_value(intrinsic(Some(value))?,c,native)?;c.row("series_property",&[24,key.len()],native)?;}}
 for value in list(r.get(7))?{
  let v=record(Some(value))?;names(record(v.get(2))?,c,native)?;c.row("product",&[24,length(v,0)?,length(v,1)?],native)?;
  for value in list(v.get(3))?{let d=record(Some(value))?;let default=match optional(d.get(2))?{None=>0,Some(value)=>{catalogue_value(intrinsic(Some(value))?,c,native)?;8}};c.row("parameter_domain",&[16,length(d,0)?,default],native)?;for value in list(d.get(1))?{catalogue_value(intrinsic(Some(value))?,c,native)?;c.row("domain_allowed_value",&[24],native)?;}}
  for value in list(v.get(4))?{let variant=record(Some(value))?;c.row("product_variant",&[16,length(variant,0)?,optional_length(variant,3)?,optional_length(variant,4)?],native)?;for(key,value)in mapping(variant.get(1))?{catalogue_value(intrinsic(Some(value))?,c,native)?;c.row("variant_parameter",&[24,key.len()],native)?;}for value in list(variant.get(2))?{property(record(Some(value))?,c,native)?;c.row("variant_property",&[24],native)?;}}
  for value in list(v.get(5))?{property(record(Some(value))?,c,native)?;c.row("product_static_property",&[24],native)?;}
 }
 for value in list(r.get(8))?{let v=record(Some(value))?;c.row("product_index",&[16,length(v,0)?,length(v,1)?,optional_length(v,2)?],native)?;for value in list(v.get(3))?{c.row("index_tag",&[16,text(Some(value))?.len()],native)?;}}
 for value in list(r.get(9))?{let v=record(Some(value))?;names(record(v.get(1))?,c,native)?;let unit=optional_unit(v.get(3),c,native)?;let cardinality=record(v.get(4))?;c.row("property_definition",&[24,length(v,0)?,length(v,2)?,unit,integer::<u32>(cardinality,0,native)?,optional_u32(cardinality,1,native)?,property_kind(scalar::<part_1::PropertyKind>(v.get(5),native)?).len(),optional_length(v,6)?],native)?;}
 for(key,values)in mapping(r.get(10))?{c.row("accessory_group",&[16,key.len()],native)?;for value in list(Some(values))?{let v=record(Some(value))?;let quantity=record(v.get(2))?;c.row("accessory",&[16,length(v,0)?,integer::<bool>(v,1,native)?,integer::<u32>(quantity,0,native)?,optional_u32(quantity,1,native)?,optional_length(v,3)?],native)?;}}
 for(key,values)in mapping(r.get(11))?{c.row("composition_group",&[16,key.len()],native)?;for value in list(Some(values))?{let v=record(Some(value))?;c.row("composition",&[16,length(v,0)?,integer::<u32>(v,1,native)?],native)?;}}
 for value in list(r.get(12))?{let v=record(Some(value))?;c.row("descriptive_object",&[16,length(v,0)?,length(v,1)?,length(v,2)?,optional_length(v,3)?,optional_length(v,4)?],native)?;}
 let bag=record(r.get(13))?;for(key,value)in mapping(bag.get(0))?{extension(intrinsic(Some(value))?,c,native)?;c.row("extension_entry",&[24,key.len()],native)?;}Ok(())
}
fn dictionary(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 for(id,lane)in[(1,"subjects"),(5,"metaSubjects")]{for value in list(r.get(id))?{let v=record(Some(value))?;names(record(v.get(2))?,c,native)?;let definition=record(v.get(3))?;c.row("dictionary_subject",&[24,lane.len(),length(v,0)?,subject_kind(scalar::<part_4::SubjectKind>(v.get(1),native)?).len(),length(definition,0)?,length(definition,1)?,optional_length(v,4)?],native)?;}}
 for value in list(r.get(2))?{let v=record(Some(value))?;let cardinality=record(v.get(4))?;c.row("dictionary_relationship",&[16,length(v,0)?,relationship_kind(scalar::<part_4::RelationshipKind>(v.get(1),native)?).len(),length(v,2)?,length(v,3)?,integer::<u32>(cardinality,0,native)?,optional_u32(cardinality,1,native)?],native)?;}
 for value in list(r.get(3))?{let v=record(Some(value))?;names(record(v.get(1))?,c,native)?;let unit=optional_unit(v.get(4),c,native)?;c.row("dictionary_property",&[24,length(v,0)?,property_kind(scalar::<part_1::PropertyKind>(v.get(2),native)?).len(),length(v,3)?,unit],native)?;for value in list(v.get(5))?{c.row("property_subject",&[16,text(Some(value))?.len()],native)?;}for value in list(v.get(6))?{let v=record(Some(value))?;c.row("value_constraint",&[16,optional_float(v,0,native)?,optional_float(v,1,native)?],native)?;for value in list(v.get(2))?{c.row("constraint_allowed",&[16,text(Some(value))?.len()],native)?;}}}
 for value in list(r.get(4))?{let v=record(Some(value))?;c.row("controlled_list",&[16,length(v,0)?],native)?;for(id,table)in[(1,"controlled_value"),(2,"controlled_context")]{for value in list(v.get(id))?{c.row(table,&[16,text(Some(value))?.len()],native)?;}}}Ok(())
}
fn geometry_catalogue(r:&R,c:&mut Census,native:&mut NativeDecodeControl<'_>)->Result<()>{
 for(key,value)in mapping(r.get(0))?{
  let v=record(Some(value))?;let shape=optional_geometry(v.get(1),c,native)?;let symbolic=optional_geometry(v.get(2),c,native)?;c.row("geometry_object",&[16,key.len(),length(v,0)?,shape,symbolic],native)?;
  for(id,table)in[(3,"space"),(4,"surface")]{for value in list(v.get(id))?{let value=record(Some(value))?;let bounds=record(value.get(2))?;let kind=if id==3{space_kind(scalar::<part_2::SpaceKind>(value.get(1),native)?).len()}else{length(value,1)?};c.row(table,&[16,length(value,0)?,kind,array_float(bounds.get(0),native)?,array_float(bounds.get(1),native)?],native)?;}}
  for value in list(v.get(5))?{let value=record(Some(value))?;c.row("port",&[16,length(value,0)?,length(value,1)?,array_float(value.get(2),native)?,array_float(value.get(3),native)?,length(value,4)?],native)?;}
  for(key,value)in mapping(v.get(6))?{c.row("geometry_binding",&[16,key.len(),text(Some(value))?.len()],native)?;}
 }
 for value in list(r.get(1))?{let v=record(Some(value))?;c.row("primitive_kind",&[16,length(v,0)?],native)?;for value in list(v.get(1))?{c.row("primitive_kind_parameter",&[16,text(Some(value))?.len()],native)?;}}Ok(())
}
/// 📇️ Admits the exact55-table parent from borrowed native syntax with no secondary document.
pub(super)fn admit_record(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<()>{
 native.scoped_stage(|native|{
  native.begin_stage(0)?;if WIDTHS.len()>limits.max_tables{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ISO authored tables exceed caller table limit"))}for(_,width)in WIDTHS{native.step()?;if *width>limits.max_columns{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ISO authored columns exceed caller column limit"))}}let mut c=Census{limits,rows:0,bytes:0};
  let cat=record(root.get(0))?;let metadata=record(cat.get(1))?;let lifecycle=record(metadata.get(1))?;let manufacturer=record(cat.get(2))?;let cat_dictionary=record(cat.get(3))?;let dict=record(root.get(1))?;let dict_reference=record(dict.get(0))?;let selection=record(root.get(3))?;let script=record(root.get(6))?;
  names(record(metadata.get(0))?,&mut c,native)?;names(record(manufacturer.get(1))?,&mut c,native)?;
  let rule=intrinsic(root.get(4))?;let kind=intrinsic_text(member(rule,"kind",native)?)?;
  let rule_bytes=match kind{
   "literal"=>sum(&[7,intrinsic_text(member(rule,"value",native)?)?.len()])?,
   "table"=>sum(&[5,intrinsic_text(member(rule,"output_column",native)?)?.len()])?,
   "script"=>sum(&[6,intrinsic_text(member(rule,"function_id",native)?)?.len(),intrinsic_text(member(rule,"source",native)?)?.len()])?,
   _=>return Err(invalid("ISO native part-number role is unknown"))
  };
  let _=scalar::<u32>(script.get(0),native)?;let _=scalar::<u32>(script.get(1),native)?;let _=scalar::<u64>(script.get(2),native)?;
  c.row("document",&[48,length(cat,0)?,length(lifecycle,0)?,length(lifecycle,1)?,optional_length(lifecycle,2)?,optional_length(lifecycle,3)?,edition(scalar::<part_1::EditionProfile>(metadata.get(2),native)?).len(),length(manufacturer,0)?,length(cat_dictionary,0)?,length(cat_dictionary,1)?,length(dict_reference,0)?,length(dict_reference,1)?,length(selection,0)?,optional_length(selection,2)?,rule_bytes,exchange(scalar::<part_5::ExchangeProcess>(root.get(7),native)?).len()],native)?;
  catalogue(cat,&mut c,native)?;dictionary(dict,&mut c,native)?;geometry_catalogue(record(root.get(2))?,&mut c,native)?;
  for value in list(selection.get(1))?{let v=record(Some(value))?;catalogue_value(intrinsic(v.get(3))?,&mut c,native)?;c.row("selection_constraint",&[24,length(v,0)?,length(v,1)?,operator(scalar::<part_1::ConstraintOperator>(v.get(2),native)?).len()],native)?;}
  if kind=="table"{for row in intrinsic_array(member(rule,"rows",native)?)?{c.row("part_number_row",&[16],native)?;for(key,value)in object(row)?{c.row("part_number_cell",&[16,key.len(),intrinsic_text(value)?.len()],native)?;}}}
  for(key,value)in mapping(root.get(5))?{catalogue_value(intrinsic(Some(value))?,&mut c,native)?;c.row("part_number_input",&[24,key.len()],native)?;}
  native.checkpoint()
 })
}

