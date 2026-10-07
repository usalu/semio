/// 🔍️ Resolves a retained, paid identity frontier with bounded text comparisons.
fn identifier(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Kit relational identity"))
}
/// 🪪️ Admits literal identities using paid borrowed entries and cancellable comparisons.
fn frontier<'a,T>(values:&'a[T],start:usize,id:impl Fn(&'a T)->&'a str,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(values.len())?;for(ordinal,value)in values.iter().enumerate(){out.checkpoint()?;let index=start.checked_add(ordinal).and_then(|value|value.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit relational identity overflow"))?;ids.push((id(value),number(index)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Kit literal identity"))}}Ok(ids)
}
/// 🧾️ Projects the literal four-text target identity without an opaque carrier.
fn reference(target:&store::os_io::ArtifactRef,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 out.insert("semio_kit_reference",&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])
}
/// 🧒️ Projects ordered child aliases independently from their persisted target identities.
fn project_children<S>(children:&[store::ArtifactChild<S>],table:&str,subset:&str,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _ids=frontier(children,0,|child|child.child_id.as_str(),out)?;
 for(ordinal,child)in children.iter().enumerate(){let target=reference(&child.target,out)?;out.insert(table,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}Ok(())
}
/// 🔢️ Borrows the canonical unsigned decimal size from fixed stack storage.
fn decimal(mut value:u64,scratch:&mut[u8;20])->&str{
 let mut at=scratch.len();loop{at-=1;scratch[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}std::str::from_utf8(&scratch[at..]).expect("ASCII unsigned decimal")
}
/// 🫳️ Visits the actual Kit catalog, placements, child references and history pins through one row writer.
fn visit_rows(snapshot:&SemioKitSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let types=frontier(&snapshot.types,0,|kind|kind.id.as_str(),out)?;
 let _designs=frontier(&snapshot.designs,0,|design|design.id.as_str(),out)?;
 out.insert_key("semio_kit_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,kind)in snapshot.types.iter().enumerate(){out.insert("semio_kit_type",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&kind.id),Cell::Text(&kind.name),Cell::Text(&kind.category)])?;}
 let mut piece_start=0usize;
 for(ordinal,design)in snapshot.designs.iter().enumerate(){
  let design_id=out.insert("semio_kit_design",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&design.id),Cell::Text(&design.name)])?;
  let pieces=frontier(&design.pieces,piece_start,|piece|piece.id.as_str(),out)?;
  piece_start=piece_start.checked_add(design.pieces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit piece extent overflow"))?;
  let _connections=frontier(&design.connections,0,|connection|connection.id.as_str(),out)?;
  for(ordinal,piece)in design.pieces.iter().enumerate(){
   let t=piece.transform;let kind=identifier(&types,&piece.type_id,out)?;
   out.insert_float("semio_kit_piece",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&piece.id),Cell::Integer(kind),Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_kit_piece"))?;
  }
  for(ordinal,connection)in design.connections.iter().enumerate(){
   let source=identifier(&pieces,&connection.connecting_piece_id,out)?;let target=identifier(&pieces,&connection.connected_piece_id,out)?;
   out.insert("semio_kit_connection",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&connection.id),Cell::Integer(source),Cell::Text(&connection.connecting_port),Cell::Integer(target),Cell::Text(&connection.connected_port)])?;
  }
 }
 project_children(&snapshot.objects,"semio_kit_object_child","object",out)?;
 project_children(&snapshot.models,"semio_kit_model_child","model",out)?;
 if let Some(child)=&snapshot.properties{let target=reference(&child.target,out)?;out.insert("semio_kit_value_child",&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,"value").map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}
 for(ordinal,link)in snapshot.representations.iter().enumerate(){
  let target=reference(&link.target,out)?;let kind=identifier(&types,&link.role,out)?;
  let(pin,checkpoint,blob)=match &link.pin{
   store::LinkPin::Head=>("head",Cell::Null,Cell::Null),
   store::LinkPin::Checkpoint{id}=>("checkpoint",Cell::Text(id),Cell::Null),
   store::LinkPin::Snapshot{blob}=>{let mut scratch=[0u8;20];let size=decimal(blob.size,&mut scratch);let id=out.insert("semio_kit_blob",&[Cell::Text(&blob.hash),Cell::Text(size),Cell::Text(&blob.media_type)])?;("snapshot",Cell::Null,Cell::Integer(id))}
  };
  out.insert("semio_kit_representation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Integer(kind),Cell::Integer(target),Cell::Text(pin),checkpoint,blob])?;
 }Ok(())
}
