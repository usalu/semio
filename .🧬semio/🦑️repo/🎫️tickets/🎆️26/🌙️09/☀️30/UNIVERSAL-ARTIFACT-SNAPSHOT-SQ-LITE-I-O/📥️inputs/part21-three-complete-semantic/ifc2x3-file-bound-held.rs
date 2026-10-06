//! 📏️ Literal IFC2x3 field bounds pay actual borrowed traversal backing.
use super::*;
fn text(value:&str,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{bound.repeated(value.len(),16)?;bound.add(96)}
fn arguments(roots:&[Part21Value],bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
 let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,Part21Value>>(1)?;pending.push(roots.iter());let mut nodes=0usize;
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};nodes=nodes.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC2x3 bound node count overflow"))?;bound.check_rows(nodes)?;bound.add(384)?;
 let children=match value{Part21Value::Str(value)|Part21Value::Enum(value)=>{text(value,bound)?;&[][..]},Part21Value::Real(decimal)=>{text(&decimal.coefficient,bound)?;&[][..]},Part21Value::List(values)=>values.as_slice(),Part21Value::Typed{name,items}=>{text(name,bound)?;items.as_slice()},_=>&[]};if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&Ifc2x3Snapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 value.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;bound.add(4096)?;text(&value.schema,&mut bound)?;
 if let Some(edm)=&value.edm_preamble{for value in [&edm.producer,&edm.module,&edm.creation_date,&edm.host,&edm.database,&edm.database_version,&edm.database_creation_date,&edm.schema,&edm.model,&edm.model_creation_date,&edm.header_model,&edm.header_model_creation_date,&edm.user,&edm.group,&edm.license,&edm.options]{text(value,&mut bound)?;}}
 for values in [&value.document.header.file_description,&value.document.header.file_name,&value.document.header.file_schema]{arguments(values,&mut bound)?;}
 bound.check_rows(value.document.instances.len())?;for instance in &value.document.instances{bound.add(192)?;bound.check_rows(instance.entities.len())?;for(name,values)in &instance.entities{bound.add(128)?;text(name,&mut bound)?;arguments(values,&mut bound)?;}bound.checkpoint()?;}bound.finish()
}
