//! 🗺️ Complete ordered map objects, independent of rendering projections and JSON media.
use semio_framework_value::{DslValue,Number,ToValue,FromValue};
/// 🧾️ Literal root property occurrence.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedProperty{pub name:String,pub value:DslValue}
/// 🗺️ Imported map ownership retains all records and every intrinsic domain.
#[derive(Clone,Debug,Default,PartialEq,ToValue,semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMap{pub positions:Vec<DslValue>,pub routes:Vec<DslValue>,pub regions:Vec<DslValue>,pub properties:Vec<ImportedProperty>}
impl ImportedMap{
 /// 📥️ Typed map transport admits complete intrinsic objects without JSON projection.
 pub fn from_media_value(value:&DslValue)->Result<Self,String>{let DslValue::Object(members)=value else{return Err("map intrinsic object required".into())};let mut map=Self::default();let mut reserved=std::collections::HashSet::new();for(name,value)in members{match name.as_str(){"positions"|"routes"|"regions"=>{if !reserved.insert(name){return Err("duplicate map collection".into())}let DslValue::Array(records)=value else{return Err("map collection array required".into())};match name.as_str(){"positions"=>map.positions=records.clone(),"routes"=>map.routes=records.clone(),_=>map.regions=records.clone()}},_=>map.properties.push(ImportedProperty{name:name.clone(),value:value.clone()})}}map.validate()?;Ok(map)}
 /// 📤️ The transport owns the first-party tree, including all literal unknown properties.
 pub fn to_media_value(&self)->Result<DslValue,String>{self.validate()?;let mut members=vec![("positions".into(),DslValue::Array(self.positions.clone())),("routes".into(),DslValue::Array(self.routes.clone())),("regions".into(),DslValue::Array(self.regions.clone()))];members.extend(self.properties.iter().map(|m|(m.name.clone(),m.value.clone())));Ok(DslValue::Object(members))}
 /// 🛂️ A renderer's identifier and coordinate requirements do not narrow storage admission.
 pub fn validate(&self)->Result<(),String>{for records in [&self.positions,&self.routes,&self.regions]{for record in records{if !matches!(record,DslValue::Object(_)){return Err("imported feature requires intrinsic object".into())}}}for member in &self.properties{if matches!(member.name.as_str(),"positions"|"routes"|"regions"){return Err("reserved imported map property".into())}}Ok(())}
 /// 📥️ JSON boundary normalizes missing collections to empty and rejects explicit null.
 pub fn from_json(text:&str)->Result<Self,String>{let value=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e|e.to_string())?;let root=semio_framework_pack_json::to_dsl_value(&value);let DslValue::Object(members)=root else{return Err("map JSON object required".into())};let mut map=Self::default();for(name,value)in members{match name.as_str(){"positions"|"routes"|"regions"=>{let DslValue::Array(records)=value else{return Err("map collection array required".into())};match name.as_str(){"positions"=>map.positions=records,"routes"=>map.routes=records,_=>map.regions=records}},_=>map.properties.push(ImportedProperty{name,value})}}map.validate()?;Ok(map)}
 /// 📤️ Media refuses intrinsic values whose variant or exact word would be lost in JSON.
 pub fn to_json(&self)->Result<String,String>{self.validate()?;let mut pending:Vec<&DslValue>=self.positions.iter().chain(&self.routes).chain(&self.regions).chain(self.properties.iter().map(|m|&m.value)).collect();while let Some(value)=pending.pop(){match value{DslValue::Bytes(_)=>return Err("map JSON cannot represent octets".into()),DslValue::Number(Number::Float(v))if !v.is_finite()=>return Err("map JSON cannot represent nonfinite IEEE words".into()),DslValue::Number(Number::Int(v))if *v>=0=>return Err("map JSON cannot represent signed positive integer tagging".into()),DslValue::Array(items)=>pending.extend(items),DslValue::Object(members)=>{let mut names=std::collections::HashSet::new();for(name,value)in members{if !names.insert(name){return Err("map JSON cannot represent duplicate object members".into())}pending.push(value)}},_=>()}}let mut members=vec![("positions".into(),DslValue::Array(self.positions.clone())),("routes".into(),DslValue::Array(self.routes.clone())),("regions".into(),DslValue::Array(self.regions.clone()))];let mut names=std::collections::HashSet::new();for property in &self.properties{if !names.insert(&property.name){return Err("map JSON cannot represent duplicate root properties".into())}members.push((property.name.clone(),property.value.clone()));}Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&DslValue::Object(members))))}
 /// 📐️ Bounds may read coordinates without the pin renderer's separate identifier requirement.
 pub fn coordinates(&self)->impl Iterator<Item=(f64,f64)>+'_ {self.positions.iter().filter_map(|value|{let lon=value.get("lon")?.as_f64()?;let lat=value.get("lat")?.as_f64()?;Some((lon,lat))})}
 /// 🪪️ Exact tree equality includes numeric variants, IEEE payloads and literal member order.
 pub fn same(&self,other:&Self)->bool{if self.properties.len()!=other.properties.len()||self.positions.len()!=other.positions.len()||self.routes.len()!=other.routes.len()||self.regions.len()!=other.regions.len(){return false}let mut pending:Vec<(&DslValue,&DslValue)>=self.positions.iter().zip(&other.positions).chain(self.routes.iter().zip(&other.routes)).chain(self.regions.iter().zip(&other.regions)).collect();for(a,b)in self.properties.iter().zip(&other.properties){if a.name!=b.name{return false}pending.push((&a.value,&b.value))}while let Some((a,b))=pending.pop(){match(a,b){(DslValue::Null,DslValue::Null)=>(),(DslValue::Bool(a),DslValue::Bool(b))if a==b=>(),(DslValue::String(a),DslValue::String(b))if a==b=>(),(DslValue::Bytes(a),DslValue::Bytes(b))if a==b=>(),(DslValue::Number(Number::UInt(a)),DslValue::Number(Number::UInt(b)))if a==b=>(),(DslValue::Number(Number::Int(a)),DslValue::Number(Number::Int(b)))if a==b=>(),(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))if a.to_bits()==b.to_bits()=>(),(DslValue::Array(a),DslValue::Array(b))if a.len()==b.len()=>pending.extend(a.iter().zip(b)),(DslValue::Object(a),DslValue::Object(b))if a.len()==b.len()=>{for((an,av),(bn,bv))in a.iter().zip(b){if an!=bn{return false}pending.push((av,bv))}},_=>return false}}true}
}

#[derive(Default,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct ImportedMapFields{positions:Vec<DslValue>,routes:Vec<DslValue>,regions:Vec<DslValue>,properties:Vec<ImportedProperty>}
impl ImportedMapFields{
 fn into_map(self)->ImportedMap{ImportedMap{positions:self.positions,routes:self.routes,regions:self.regions,properties:self.properties}}
}
impl FromValue for ImportedMap{
 fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{let map=ImportedMapFields::from_value(value)?.into_map();map.validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;Ok(map)}
 fn from_value_controlled(value:&DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{let owner=semio_framework_value::DecodedValue::new(ImportedMapFields::from_value_controlled(value,control)?.into_map(),Self::retire_decoded);owner.get().validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;control.checkpoint()?;Ok(owner.take())}
 fn default_value_controlled(control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{ImportedMapFields::default_value_controlled(control).map(ImportedMapFields::into_map)}
 fn retire_decoded(self){ImportedMapFields::retire_decoded(ImportedMapFields{positions:self.positions,routes:self.routes,regions:self.regions,properties:self.properties});}
}
