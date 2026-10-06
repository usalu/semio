//! 📏️ Borrowed STEP owned-record bounds pay concrete traversal frontiers.
use super::*;
fn text(value:&str,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{bound.repeated(value.len(),16)?;bound.add(96)}
fn arguments(roots:&[StepValue],bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{
 let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,StepValue>>(1)?;pending.push(roots.iter());let mut nodes=0usize;
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};nodes=nodes.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"STEP bound node count overflow"))?;bound.check_rows(nodes)?;bound.add(256)?;
 let children=match value{StepValue::String(value)|StepValue::Enum(value)=>{text(value,bound)?;&[][..]},StepValue::Aggregate(values)=>values.as_slice(),StepValue::TypedValue{type_name,value}=>{text(type_name,bound)?;std::slice::from_ref(value.as_ref())},_=>&[]};if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&StepSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 value.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;bound.add(4096)?;text(&value.schema,&mut bound)?;let header=&value.header;let name=&header.file_name;
 for value in [&header.file_description.implementation_level,&name.name,&name.timestamp,&name.preprocessor_version,&name.originating_system,&name.authorization]{text(value,&mut bound)?;}
 for list in [&header.file_description.description,&name.author,&name.organization,&header.file_schema.schemas]{bound.check_rows(list.len())?;for value in list{text(value,&mut bound)?;bound.checkpoint()?;}}
 bound.check_rows(value.entities.len())?;for entity in &value.entities{bound.add(256)?;text(&entity.name,&mut bound)?;arguments(&entity.args,&mut bound)?;bound.check_rows(entity.complex.len())?;for part in &entity.complex{bound.add(128)?;text(&part.name,&mut bound)?;arguments(&part.args,&mut bound)?;}bound.checkpoint()?;}bound.finish()
}
