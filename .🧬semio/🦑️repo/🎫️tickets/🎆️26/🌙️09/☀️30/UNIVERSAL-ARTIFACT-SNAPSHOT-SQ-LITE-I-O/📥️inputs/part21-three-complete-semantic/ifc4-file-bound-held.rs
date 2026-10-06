//! 📏️ IFC4 native bounds pay traversal storage while forecasting owned field encodings.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::artifact;
fn text(value:&str,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{bound.repeated(value.len(),16)?;bound.add(96)}
fn rows(count:&mut usize,size:usize,bound:&mut artifact::NativeEncodingBound<'_,'_>)->Result<(),ValueError>{*count=count.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 native row census overflow"))?;bound.check_rows(*count)}
fn arguments(roots:&[IfcValue],bound:&mut artifact::NativeEncodingBound<'_,'_>,count:&mut usize)->Result<(),ValueError>{
 rows(count,roots.len(),bound)?;let mut pending=bound.allocate_frontier::<std::slice::Iter<'_,IfcValue>>(1)?;pending.push(roots.iter());
 while !pending.is_empty(){let Some(value)=pending.last_mut().unwrap().next()else{pending.pop();continue};rows(count,1,bound)?;bound.add(256)?;
 let children=match value{IfcValue::String(value)|IfcValue::Enum(value)=>{text(value,bound)?;&[][..]},IfcValue::Aggregate(values)=>values.as_slice(),IfcValue::TypedValue{name,items}=>{text(name,bound)?;items.as_slice()},_=>&[]};
 rows(count,children.len(),bound)?;if !children.is_empty(){bound.push_frontier(&mut pending,children.iter())?;}bound.checkpoint()?;
 }Ok(())
}
pub(super) fn preflight(value:&IfcSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 value.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=artifact::NativeEncodingBound::file_only(control)?;let mut count=0;rows(&mut count,2,&mut bound)?;bound.add(4096)?;text(&value.schema,&mut bound)?;
 for values in [&value.header.file_description,&value.header.file_name,&value.header.file_schema]{arguments(values,&mut bound,&mut count)?;}
 rows(&mut count,value.entities.len(),&mut bound)?;for entity in &value.entities{bound.add(256)?;text(&entity.name,&mut bound)?;arguments(&entity.args,&mut bound,&mut count)?;
 rows(&mut count,entity.complex.len(),&mut bound)?;for part in &entity.complex{bound.add(128)?;text(&part.name,&mut bound)?;arguments(&part.args,&mut bound,&mut count)?;}bound.checkpoint()?;}bound.finish()
}
