/// 🪪️ Builds a paid name frontier in authored ordinal order with bounded comparisons.
fn cad_frontier<'a>(names:impl Iterator<Item=&'a str>,count:usize,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(count)?;for(ordinal,name)in names.enumerate(){out.checkpoint()?;ids.push((name,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD identity extent overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio CAD identity within its owner"))}}Ok(ids)
}
/// 🔍️ Resolves actual CAD layer and block identities with cancellable UTF8 comparisons.
fn cad_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio CAD layer or block"))
}
/// 🏷️ Admits owner-local handles before visiting any entity geometry.
fn cad_records(records:&[CadEntityRecord],block:Option<i64>,layers:&[(&str,i64)],blocks:&[(&str,i64)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _handles=cad_frontier(records.iter().map(|record|record.handle.as_str()),records.len(),out)?;for(ordinal,record)in records.iter().enumerate(){project_record(record,block,ordinal,layers,blocks,out)?;}Ok(())
}
/// 🫳️ Visits every authored CAD cell using the same actual RowWriter for admission and projection.
fn visit_rows(snapshot:&SemioCadSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let layers=cad_frontier(snapshot.layers.iter().map(|layer|layer.name.as_str()),snapshot.layers.len(),out)?;let blocks=cad_frontier(snapshot.blocks.iter().map(|block|block.name.as_str()),snapshot.blocks.len(),out)?;
 out.insert_key("semio_cad_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,layer)in snapshot.layers.iter().enumerate(){out.insert("semio_cad_layer",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layer.name),Cell::Integer(i64::from(layer.color_index)),Cell::Text(&layer.line_type),Cell::Integer(i64::from(layer.visible))])?;}
 for(ordinal,block)in snapshot.blocks.iter().enumerate(){out.insert_float("semio_cad_block",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&block.name),Cell::Real(block.base_point.x),Cell::Real(block.base_point.y)],float_columns("semio_cad_block"))?;}
 for block in &snapshot.blocks{let owner=cad_reference(&blocks,&block.name,out)?;cad_records(&block.entities,Some(owner),&layers,&blocks,out)?;}cad_records(&snapshot.entities,None,&layers,&blocks,out)
}
/// 🧱️ Projects all nine detail variants through their actual exact numeric columns.
fn project_record(record:&CadEntityRecord,block:Option<i64>,ordinal:usize,layers:&[(&str,i64)],blocks:&[(&str,i64)],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
let kind=match record.entity{CadEntity::Line{..}=>"line",CadEntity::Arc{..}=>"arc",CadEntity::Circle{..}=>"circle",CadEntity::Ellipse{..}=>"ellipse",CadEntity::Polyline{..}=>"polyline",CadEntity::Text{..}=>"text",CadEntity::Insert{..}=>"insert",CadEntity::Solid{..}=>"solid",CadEntity::Dimension{..}=>"dimension"};let layer_id=cad_reference(layers,&record.layer,p)?;let id=p.insert("semio_cad_entity",&[Cell::Integer(1),block.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(number(ordinal)?),Cell::Text(&record.handle),Cell::Integer(layer_id),Cell::Text(kind)])?;
match &record.entity{
CadEntity::Line{a,b}=>p.insert_key_float("semio_cad_line",id,&[Cell::Real(a.x),Cell::Real(a.y),Cell::Real(b.x),Cell::Real(b.y)],float_columns("semio_cad_line"))?,
CadEntity::Arc{center,radius,start_angle,end_angle}=>p.insert_key_float("semio_cad_arc",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius),Cell::Real(*start_angle),Cell::Real(*end_angle)],float_columns("semio_cad_arc"))?,
CadEntity::Circle{center,radius}=>p.insert_key_float("semio_cad_circle",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius)],float_columns("semio_cad_circle"))?,
CadEntity::Ellipse{center,major_axis_end,ratio,start_param,end_param}=>p.insert_key_float("semio_cad_ellipse",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(major_axis_end.x),Cell::Real(major_axis_end.y),Cell::Real(*ratio),Cell::Real(*start_param),Cell::Real(*end_param)],float_columns("semio_cad_ellipse"))?,
CadEntity::Polyline{vertices,closed}=>{p.insert_key_float("semio_cad_polyline",id,&[Cell::Integer(i64::from(*closed))],float_columns("semio_cad_polyline"))?;for(ordinal,vertex)in vertices.iter().enumerate(){p.insert_float("semio_cad_polyline_vertex",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(vertex.x),Cell::Real(vertex.y)],float_columns("semio_cad_polyline_vertex"))?;}},
CadEntity::Text{position,height,rotation,content}=>p.insert_key_float("semio_cad_text",id,&[Cell::Real(position.x),Cell::Real(position.y),Cell::Real(*height),Cell::Real(*rotation),Cell::Text(content)],float_columns("semio_cad_text"))?,
CadEntity::Insert{block_name,insertion_point,scale,rotation}=>{let target=cad_reference(blocks,block_name,p)?;p.insert_key_float("semio_cad_insert",id,&[Cell::Integer(target),Cell::Real(insertion_point.x),Cell::Real(insertion_point.y),Cell::Real(scale.x),Cell::Real(scale.y),Cell::Real(*rotation)],float_columns("semio_cad_insert"))?;},
CadEntity::Solid{p1,p2,p3,p4}=>p.insert_key_float("semio_cad_solid",id,&[Cell::Real(p1.x),Cell::Real(p1.y),Cell::Real(p2.x),Cell::Real(p2.y),Cell::Real(p3.x),Cell::Real(p3.y),Cell::Real(p4.x),Cell::Real(p4.y)],float_columns("semio_cad_solid"))?,
CadEntity::Dimension{def_point,text_position,measurement,text}=>p.insert_key_float("semio_cad_dimension",id,&[Cell::Real(def_point.x),Cell::Real(def_point.y),Cell::Real(text_position.x),Cell::Real(text_position.y),Cell::Real(*measurement),Cell::Text(text)],float_columns("semio_cad_dimension"))?}
Ok(())}