//! 🧱️ Literal box-grid ownership frontier and cumulative derived construction.
use super::*;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
use semio_framework_diagnostic::TextError;
use semio_framework_value::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}
fn error(error:ValueError)->TextError{TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(count).filter(|n|*n<=maximum).ok_or_else(||invalid("grid3d native rows exceed caller limit"))?;Ok(())}
fn list(record:&RecordValue,id:u16)->Result<&[FieldValue],ValueError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::List(items))=>Ok(items),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),_=>Err(invalid("grid3d native collection shape differs"))}}
fn unwrap(value:&FieldValue)->&FieldValue{match value{semio_framework_dsl_record::FieldValue::Block(value)=>value,_=>value}}
fn admit_record(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<(),ValueError>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let mut rows=1usize;add(&mut rows,0,maximum)?;for id in[5,6,7,11,12,13,14]{add(&mut rows,list(record,id)?.len(),maximum)?;}
 for tile in list(record,11)?{c.step()?;let semio_framework_dsl_record::FieldValue::Record(tile)=tile else{return Err(invalid("grid3d native tile record differs"))};add(&mut rows,1,maximum)?;
  let media=match tile.get(3){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>continue,Some(value)=>unwrap(value)};let semio_framework_dsl_record::FieldValue::Statements(items)=media else{return Err(invalid("grid3d native tile media differs"))};let[(kind,variant)]=items.as_slice()else{return Err(invalid("grid3d native tile media arity differs"))};
  match kind.as_str(){"mesh"=>{let mesh=match variant.get(0){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>continue,Some(value)=>unwrap(value)};let semio_framework_dsl_record::FieldValue::Record(mesh)=mesh else{return Err(invalid("grid3d native inline mesh record differs"))};add(&mut rows,list(mesh,0)?.len(),maximum)?;add(&mut rows,list(mesh,1)?.len(),maximum)?;if mesh.get(2).is_some_and(|v|!matches!(unwrap(v),FieldValue::Absent)){add(&mut rows,1,maximum)?;}},"mesh-child"=>{},_=>return Err(invalid("grid3d native tile media tag differs"))}
 }c.checkpoint()})
}
pub(crate)fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Grid3dSnapshot,ValueError>{let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Grid3dSnapshot as store::ArtifactDsl>::envelope_id(),Grid3dSnapshot::__dsl_spec_producer(),|record,native|{admit_record(record,native,maximum)?;Grid3dSnapshot::__dsl_from_record_controlled(record,native)},c)}
fn rows(s:&Grid3dSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let maximum=c.limits().max_rows;let mut rows=1usize;add(&mut rows,0,maximum)?;for count in[s.cell_sizes_x.len(),s.cell_sizes_y.len(),s.cell_sizes_z.len(),s.tiles.len(),s.rules.len(),s.pinned.len(),s.masked.len()]{add(&mut rows,count,maximum)?;}
 c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,s.tiles.len())?;for(i,tile)in s.tiles.iter().enumerate(){add(&mut rows,1,maximum)?;if let Grid3dTileMedia::Mesh{mesh}=&tile.media{add(&mut rows,mesh.positions.len(),maximum)?;add(&mut rows,mesh.indices.len(),maximum)?;add(&mut rows,usize::from(mesh.color.is_some()),maximum)?;}if i%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,i,s.tiles.len())?;}}c.checkpoint(SqliteSnapshotPhase::EncodeNative,s.tiles.len(),s.tiles.len())
}
pub(crate)fn encode_sqlite_snapshot_native(s:&Grid3dSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{rows(s,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Grid3dSnapshot as store::ArtifactDsl>::envelope_id(),Grid3dSnapshot::__dsl_spec_producer(),|native|s.__dsl_to_record_controlled(native),c)}
