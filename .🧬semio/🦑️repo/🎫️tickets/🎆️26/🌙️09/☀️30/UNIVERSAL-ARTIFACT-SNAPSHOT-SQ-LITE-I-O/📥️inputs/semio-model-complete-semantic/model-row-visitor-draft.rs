/// 🔍️ Resolves a paid literal Model identity with bounded text comparisons.
fn entity_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Model entity identity"))
}
/// 🏛️ Resolves only the declared spatial owner partition for hierarchy and containment.
fn spatial_reference(ids:&[(&str,i64)],id:&str,count:usize,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let row=entity_reference(ids,id,out)?;if row>number(count)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio Model parent is outside its spatial partition"))}Ok(row)
}
/// ♻️ Detects spatial cycles using paid fixed-width parent marks without growing path ownership.
fn spatial_cycles(parents:&mut[(Option<usize>,u8)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for root in 0..parents.len(){
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;match parents[index].1{2=>break,1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic Semio Model spatial hierarchy")),_=>{parents[index].1=1;current=parents[index].0;}}}
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;if parents[index].1!=1{break}parents[index].1=2;current=parents[index].0;}
 }Ok(())
}
/// 🪪️ Collects the exact entity partition using a paid borrowed frontier and cancellable comparisons.
fn entity_frontier<'a>(snapshot:&'a SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let count=snapshot.spatial.len().checked_add(snapshot.elements.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity extent overflow"))?;let phase=out.phase();let mut ids=out.allocate_frontier(count)?;
 for(ordinal,id)in snapshot.spatial.iter().map(|node|node.id.as_str()).chain(snapshot.elements.iter().map(|element|element.id.as_str())).enumerate(){out.checkpoint()?;ids.push((id,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity identity overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model entity identity"))}}Ok(ids)
}
/// 📐️ Projects all ten exact placement words through the actual float column declaration.
fn placement(id:i64,t:SemioTransform,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("semio_model_placement",id,&[Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_model_placement"))
}
/// 🫳️ Visits the real Model entities, hierarchy, typed property variants and relation endpoints.
fn visit_rows(snapshot:&SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let entities=entity_frontier(snapshot,out)?;let mut parents=out.allocate_frontier(snapshot.spatial.len())?;
 for node in &snapshot.spatial{out.checkpoint()?;let parent=match &node.parent_id{None=>None,Some(id)=>Some(usize::try_from(spatial_reference(&entities,id,snapshot.spatial.len(),out)?-1).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)};parents.push((parent,0u8));}
 spatial_cycles(&mut parents,out)?;let phase=out.phase();let mut relations=out.allocate_frontier(snapshot.relations.len())?;for edge in &snapshot.relations{out.checkpoint()?;relations.push(edge.id.as_str());}
 out.sort_frontier(&mut relations,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,control))?;
 for pair in relations.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model relation identity"))}}
 out.insert_key("semio_model_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.spatial.iter().enumerate(){
  let id=entity_reference(&entities,&node.id,out)?;let parent=parents[ordinal].0.map(|index|number(index+1).map(Cell::Integer)).transpose()?.unwrap_or(Cell::Null);
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("spatial"),Cell::Integer(number(ordinal)?),Cell::Text(&node.id)])?;
  out.insert_key("semio_model_spatial",id,&[Cell::Text(match node.kind{SpatialKind::Site=>"site",SpatialKind::Building=>"building",SpatialKind::Storey=>"storey",SpatialKind::Space=>"space"}),Cell::Text(&node.name),parent])?;
  placement(id,node.placement,out)?;
 }
 for(ordinal,element)in snapshot.elements.iter().enumerate(){
  let id=entity_reference(&entities,&element.id,out)?;let(class,name)=class(&element.class);let spatial=match &element.spatial_id{None=>Cell::Null,Some(id)=>Cell::Integer(spatial_reference(&entities,id,snapshot.spatial.len(),out)?)};
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("element"),Cell::Integer(number(ordinal)?),Cell::Text(&element.id)])?;
  out.insert_key("semio_model_element",id,&[Cell::Text(class),name.map(Cell::Text).unwrap_or(Cell::Null),spatial])?;placement(id,element.placement,out)?;
  let(kind,target)=match &element.geometry{GeometryRef::None=>("none",Cell::Null),GeometryRef::Brep{brep_id}=>("brep",Cell::Text(brep_id)),GeometryRef::Mesh{mesh_id}=>("mesh",Cell::Text(mesh_id))};
  out.insert_key("semio_model_geometry_reference",id,&[Cell::Text(kind),target])?;
  for(ordinal,pset)in element.psets.iter().enumerate(){
   let owner=out.insert("semio_model_property_set",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&pset.name)])?;
   for(ordinal,property)in pset.properties.iter().enumerate(){
    let(kind,text,number_value,boolean)=match &property.value{PsetValue::Text{value}=>("text",Cell::Text(value),Cell::Null,Cell::Null),PsetValue::Number{value}=>("number",Cell::Null,Cell::Real(*value),Cell::Null),PsetValue::Boolean{value}=>("boolean",Cell::Null,Cell::Null,Cell::Integer(i64::from(*value)))};
    out.insert_float("semio_model_property",&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Text(kind),text,number_value,boolean],float_columns("semio_model_property"))?;
   }
  }
 }
 for(ordinal,edge)in snapshot.relations.iter().enumerate(){
  let(kind,label)=relation(&edge.kind);let source=entity_reference(&entities,&edge.from,out)?;let target=entity_reference(&entities,&edge.to,out)?;
  out.insert("semio_model_relation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Text(kind),label.map(Cell::Text).unwrap_or(Cell::Null),Cell::Integer(source),Cell::Integer(target)])?;
 }Ok(())
}
