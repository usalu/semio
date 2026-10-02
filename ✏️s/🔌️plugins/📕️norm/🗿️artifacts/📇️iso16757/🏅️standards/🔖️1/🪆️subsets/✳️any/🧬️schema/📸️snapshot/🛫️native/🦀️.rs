//! 🧮️ Borrowed catalogue entity admission before genuine controlled native construction.
use crate::{Iso16757Snapshot,Names,CatalogueValue,part_1,part_2,part_5};
use dsl::{NativeEncodeControl,DslValue};
fn add(rows:&mut usize,count:usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{*rows=rows.checked_add(count).ok_or("ISO entity count overflow")?;if *rows>maximum{return Err("ISO native entity row limit exceeded".into())}c.step()}
fn names(v:&Names,rows:&mut usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{add(rows,1,maximum,c)?;add(rows,v.alternatives.len(),maximum,c)}
fn value(v:&CatalogueValue,rows:&mut usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{c.scoped_depth(64,|c|{add(rows,1,maximum,c)?;match v{CatalogueValue::Quantity{..}|CatalogueValue::Range{unit:Some(_),..}=>add(rows,1,maximum,c)?,CatalogueValue::List{items}=>{add(rows,items.len(),maximum,c)?;for v in items{value(v,rows,maximum,c)?;}},_=>{}}Ok(())})}
fn property(v:&part_1::PropertyValue,rows:&mut usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{add(rows,2,maximum,c)?;value(&v.value,rows,maximum,c)}
fn node(v:&part_2::GeometryNode,rows:&mut usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{c.scoped_depth(64,|c|{add(rows,1,maximum,c)?;match v{part_2::GeometryNode::Primitive{parameters,..}=>add(rows,parameters.len(),maximum,c)?,part_2::GeometryNode::Transform{child,..}=>{add(rows,1,maximum,c)?;node(child,rows,maximum,c)?;},part_2::GeometryNode::Boolean{children,..}=>{add(rows,children.len(),maximum,c)?;for v in children{node(v,rows,maximum,c)?;}},part_2::GeometryNode::Reference{..}=>{}}Ok(())})}
fn extension(v:&DslValue,rows:&mut usize,maximum:usize,c:&mut NativeEncodeControl<'_>)->Result<(),String>{c.scoped_depth(64,|c|{add(rows,1,maximum,c)?;match v{DslValue::Array(items)=>{add(rows,items.len(),maximum,c)?;for v in items{extension(v,rows,maximum,c)?;}},DslValue::Object(items)=>{add(rows,items.len(),maximum,c)?;for(_,v)in items{extension(v,rows,maximum,c)?;}},_=>{}}Ok(())})}
/// 📇️ Counts each authored entity occurrence without materializing a second document.
pub fn admit_rows(s:&Iso16757Snapshot,c:&mut NativeEncodeControl<'_>,maximum:usize)->Result<(),String>{c.scoped_stage(|c|{
 c.begin_stage(0)?;let mut rows=0usize;add(&mut rows,1,maximum,c)?;let cat=&s.catalogue;names(&cat.metadata.names,&mut rows,maximum,c)?;names(&cat.manufacturer.names,&mut rows,maximum,c)?;
 for v in &cat.product_groups{add(&mut rows,1,maximum,c)?;names(&v.names,&mut rows,maximum,c)?;}
 for v in &cat.product_classes{add(&mut rows,1,maximum,c)?;names(&v.names,&mut rows,maximum,c)?;add(&mut rows,v.required_property_ids.len(),maximum,c)?;add(&mut rows,v.optional_property_ids.len(),maximum,c)?;}
 for v in &cat.product_series{add(&mut rows,1,maximum,c)?;names(&v.names,&mut rows,maximum,c)?;for v in v.shared_property_values.values(){add(&mut rows,1,maximum,c)?;value(v,&mut rows,maximum,c)?;}}
 for v in &cat.products{add(&mut rows,1,maximum,c)?;names(&v.names,&mut rows,maximum,c)?;for v in &v.parameter_domains{add(&mut rows,1,maximum,c)?;if let Some(v)=&v.default_value{value(v,&mut rows,maximum,c)?;}for v in &v.allowed_values{add(&mut rows,1,maximum,c)?;value(v,&mut rows,maximum,c)?;}}for v in &v.variants{add(&mut rows,1,maximum,c)?;for v in v.parameter_values.values(){add(&mut rows,1,maximum,c)?;value(v,&mut rows,maximum,c)?;}for v in &v.property_values{property(v,&mut rows,maximum,c)?;}}for v in &v.static_properties{property(v,&mut rows,maximum,c)?;}}
 for v in &cat.product_indexes{add(&mut rows,1,maximum,c)?;add(&mut rows,v.search_tags.len(),maximum,c)?;}
 for v in &cat.property_definitions{add(&mut rows,1+usize::from(v.unit.is_some()),maximum,c)?;names(&v.names,&mut rows,maximum,c)?;}
 for values in cat.accessories.values(){add(&mut rows,1,maximum,c)?;add(&mut rows,values.len(),maximum,c)?;}for values in cat.compositions.values(){add(&mut rows,1,maximum,c)?;add(&mut rows,values.len(),maximum,c)?;}add(&mut rows,cat.descriptive_objects.len(),maximum,c)?;
 for v in cat.extensions.fields.values(){add(&mut rows,1,maximum,c)?;extension(v,&mut rows,maximum,c)?;}
 let dictionary=&s.dictionary;for v in dictionary.subjects.iter().chain(&dictionary.meta_subjects){add(&mut rows,1,maximum,c)?;names(&v.names,&mut rows,maximum,c)?;}add(&mut rows,dictionary.relationships.len(),maximum,c)?;
 for v in &dictionary.properties{add(&mut rows,1+usize::from(v.unit.is_some()),maximum,c)?;names(&v.names,&mut rows,maximum,c)?;add(&mut rows,v.applicable_subject_ids.len(),maximum,c)?;for v in &v.value_constraints{add(&mut rows,1,maximum,c)?;add(&mut rows,v.allowed_values.len(),maximum,c)?;}}
 for v in &dictionary.controlled_lists{add(&mut rows,1,maximum,c)?;add(&mut rows,v.values.len(),maximum,c)?;add(&mut rows,v.context_subject_ids.len(),maximum,c)?;}
 for v in s.geometry.objects.values(){add(&mut rows,1,maximum,c)?;if let Some(v)=&v.shape{node(v,&mut rows,maximum,c)?;}if let Some(v)=&v.symbolic{node(v,&mut rows,maximum,c)?;}for count in[v.spaces.len(),v.surfaces.len(),v.ports.len(),v.parameter_bindings.len()]{add(&mut rows,count,maximum,c)?;}}
 for v in &s.geometry.primitive_registry{add(&mut rows,1,maximum,c)?;add(&mut rows,v.parameters.len(),maximum,c)?;}
 for v in &s.selection.constraints{add(&mut rows,1,maximum,c)?;value(&v.value,&mut rows,maximum,c)?;}
 if let part_5::PartNumberRule::Table{rows:values,..}=&s.part_number_rule{for v in values{add(&mut rows,1,maximum,c)?;add(&mut rows,v.len(),maximum,c)?;}}
 for v in s.part_number_inputs.values(){add(&mut rows,1,maximum,c)?;value(v,&mut rows,maximum,c)?;}c.checkpoint()
})}
