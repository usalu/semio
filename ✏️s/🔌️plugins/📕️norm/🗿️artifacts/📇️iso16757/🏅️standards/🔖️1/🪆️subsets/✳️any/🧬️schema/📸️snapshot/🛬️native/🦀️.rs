//! 🛬️ Explicit borrowed construction of catalogue native syntax under one cumulative control.
use crate::{CatalogueValue as V,CatalogueUnit,DimensionSignature,NullState,part_5::PartNumberRule};
use dsl::{DslValue as D,FieldValue,NativeDecodeControl};
use std::collections::BTreeMap;
fn object(v:&D)->Result<&[(String,D)],String>{match v{D::Object(v)=>Ok(v),_=>Err("expected catalogue syntax object".into())}}
fn optional_field<'a>(v:&'a[(String,D)],name:&str,c:&mut NativeDecodeControl<'_>)->Result<Option<&'a D>,String>{let mut found=None;for(key,value)in v{c.step()?;if key==name{if found.is_some(){return Err("duplicate catalogue field".into())}found=Some(value);}}Ok(found)}
fn field<'a>(v:&'a[(String,D)],name:&str,c:&mut NativeDecodeControl<'_>)->Result<&'a D,String>{optional_field(v,name,c)?.ok_or_else(||"missing catalogue field".into())}
fn keys(v:&[(String,D)],allowed:&[&str],c:&mut NativeDecodeControl<'_>)->Result<(),String>{if v.len()>allowed.len(){return Err("excess catalogue fields".into())}for(key,_)in v{c.step()?;if !allowed.contains(&key.as_str()){return Err("unknown catalogue field".into())}field(v,key,c)?;}Ok(())}
fn text(v:&D)->Result<&str,String>{match v{D::String(v)=>Ok(v),_=>Err("expected catalogue text".into())}}
fn copy(v:&D,c:&mut NativeDecodeControl<'_>)->Result<String,String>{c.copy_text(text(v)?)}
fn numeric(v:&D)->Result<f64,String>{match v{D::Number(v)=>Ok(v.as_f64()),_=>Err("expected catalogue numeric scalar".into())}}
fn integer(v:&D)->Result<i64,String>{match v{D::Number(v)=>v.as_i64().ok_or_else(||"expected exact catalogue integer".into()),_=>Err("expected catalogue integer".into())}}
fn array(v:&D)->Result<&[D],String>{match v{D::Array(v)=>Ok(v),_=>Err("expected catalogue array".into())}}
fn unit(v:&D,c:&mut NativeDecodeControl<'_>)->Result<CatalogueUnit,String>{let v=object(v)?;keys(v,&["symbol","dimension","siFactor"],c)?;let d=object(field(v,"dimension",c)?)?;keys(d,&["length","mass","time","temperature"],c)?;let symbol=copy(field(v,"symbol",c)?,c)?;let si_factor=numeric(field(v,"siFactor",c)?)?;let mut signed=|key|i8::try_from(integer(field(d,key,c)?)?).map_err(|_|"catalogue dimension exceeds signed8".to_string());Ok(CatalogueUnit{symbol,si_factor,dimension:DimensionSignature{length:signed("length")?,mass:signed("mass")?,time:signed("time")?,temperature:signed("temperature")?}})}
fn reserve<T>(v:&mut Vec<T>,c:&mut NativeDecodeControl<'_>)->Result<(),String>{if v.len()==v.capacity(){let extra=v.capacity().max(1);let bytes=v.capacity().checked_add(extra).and_then(|n|n.checked_mul(std::mem::size_of::<T>())).filter(|n|*n<=isize::MAX as usize).ok_or("catalogue frontier allocation overflow")?;c.charge(bytes)?;v.try_reserve_exact(extra).map_err(|_|"catalogue construction allocation failed")?;}Ok(())}
fn push<T>(v:&mut Vec<T>,value:T,c:&mut NativeDecodeControl<'_>)->Result<(),String>{reserve(v,c)?;v.push(value);Ok(())}
pub fn retire_value(value:V){let mut pending=vec![value];while let Some(value)=pending.pop(){if let V::List{items}=value{pending.extend(items);}}}
fn push_value(values:&mut Vec<V>,value:V,c:&mut NativeDecodeControl<'_>)->Result<(),String>{if let Err(error)=reserve(values,c){retire_value(value);return Err(error)}values.push(value);Ok(())}
struct Values(Vec<V>);
impl Drop for Values{fn drop(&mut self){for v in self.0.drain(..){retire_value(v);}}}
enum Task<'a>{Value(&'a D),List(usize)}
pub fn catalogue_value(value:&FieldValue,c:&mut NativeDecodeControl<'_>)->Result<V,String>{
 let FieldValue::Value(root)=value else{return Err("expected catalogue intrinsic syntax value".into())};
 c.begin_stage(0)?;let mut pending=Vec::new();push(&mut pending,Task::Value(root),c)?;let mut values=Values(Vec::new());
 while let Some(task)=pending.pop(){c.step()?;match task{
  Task::List(count)=>{let offset=values.0.len().checked_sub(count).ok_or("catalogue list construction differs")?;let mut items=Values(c.allocate_vec(count)?);for index in offset..values.0.len(){c.step()?;items.0.push(std::mem::replace(&mut values.0[index],V::Null{state:NullState::Unknown}));}values.0.truncate(offset);push_value(&mut values.0,V::List{items:std::mem::take(&mut items.0)},c)?;},
  Task::Value(value)=>{let v=object(value)?;let kind=text(field(v,"kind",c)?)?;
   keys(v,match kind{"boolean"|"integer"|"decimal"|"text"|"identifier"|"enumeration"=>&["kind","value"],"controlled"=>&["kind","value","list_id"],"quantity"=>&["kind","value","unit"],"range"=>&["kind","min","max","unit"],"null"=>&["kind","state"],"reference"=>&["kind","target_id"],"list"=>&["kind","items"],_=>return Err("unknown catalogue value variant".into())},c)?;

   if kind=="list"{let items=array(field(v,"items",c)?)?;push(&mut pending,Task::List(items.len()),c)?;for v in items.iter().rev(){c.step()?;push(&mut pending,Task::Value(v),c)?;}continue;}
   let out=match kind{
    "boolean"=>V::Boolean{value:match field(v,"value",c)?{D::Bool(v)=>*v,_=>return Err("expected catalogue boolean".into())}},
    "integer"=>V::Integer{value:integer(field(v,"value",c)?)?},
    "decimal"=>V::Decimal{value:numeric(field(v,"value",c)?)?},
    "text"=>V::Text{value:copy(field(v,"value",c)?,c)?},
    "identifier"=>V::Identifier{value:copy(field(v,"value",c)?,c)?},
    "enumeration"=>V::Enumeration{value:copy(field(v,"value",c)?,c)?},
    "controlled"=>V::Controlled{value:copy(field(v,"value",c)?,c)?,list_id:copy(field(v,"list_id",c)?,c)?},
    "quantity"=>V::Quantity{value:numeric(field(v,"value",c)?)?,unit:unit(field(v,"unit",c)?,c)?},
    "range"=>V::Range{min:numeric(field(v,"min",c)?)?,max:numeric(field(v,"max",c)?)?,unit:match optional_field(v,"unit",c)?{None|Some(D::Null)=>None,Some(v)=>Some(unit(v,c)?)}},
    "null"=>V::Null{state:match text(field(v,"state",c)?)?{"Unavailable"=>NullState::Unavailable,"Unknown"=>NullState::Unknown,"NotApplicable"=>NullState::NotApplicable,_=>return Err("unknown catalogue null state".into())}},
    "reference"=>V::Reference{target_id:copy(field(v,"target_id",c)?,c)?},
    _=>return Err("unknown catalogue value variant".into())
   };push_value(&mut values.0,out,c)?;
  }
 }}
 if values.0.len()!=1{return Err("catalogue native construction has excess owners".into())}values.0.pop().ok_or_else(||"catalogue native construction missing owner".into())
}
pub fn part_number(value:&FieldValue,c:&mut NativeDecodeControl<'_>)->Result<PartNumberRule,String>{
 c.begin_stage(0)?;
 let FieldValue::Value(value)=value else{return Err("expected part number intrinsic syntax value".into())};let v=object(value)?;let kind=text(field(v,"kind",c)?)?;keys(v,match kind{"literal"=>&["kind","value"],"script"=>&["kind","function_id","source"],"table"=>&["kind","rows","output_column"],_=>return Err("unknown part number variant".into())},c)?;
 match kind{
  "literal"=>Ok(PartNumberRule::Literal{value:copy(field(v,"value",c)?,c)?}),
  "script"=>Ok(PartNumberRule::Script{function_id:copy(field(v,"function_id",c)?,c)?,source:copy(field(v,"source",c)?,c)?}),
  "table"=>{let input=array(field(v,"rows",c)?)?;let mut rows=c.allocate_vec(input.len())?;for row in input{c.step()?;let mut result=BTreeMap::new();for(key,value)in object(row)?{c.step()?;c.charge(std::mem::size_of::<(String,String)>().checked_add(128).ok_or("part number ownership overflow")?)?;let key=c.copy_text(key)?;let value=copy(value,c)?;if result.insert(key,value).is_some(){return Err("duplicate part number column".into())}}rows.push(result);}Ok(PartNumberRule::Table{rows,output_column:copy(field(v,"output_column",c)?,c)?})},
  _=>Err("unknown part number variant".into())
 }
}
pub fn retire_geometry(node:crate::part_2::GeometryNode){use crate::part_2::GeometryNode as N;let mut pending=vec![node];while let Some(node)=pending.pop(){match node{N::Transform{child,..}=>pending.push(*child),N::Boolean{children,..}=>pending.extend(children),_=>{}}}}
