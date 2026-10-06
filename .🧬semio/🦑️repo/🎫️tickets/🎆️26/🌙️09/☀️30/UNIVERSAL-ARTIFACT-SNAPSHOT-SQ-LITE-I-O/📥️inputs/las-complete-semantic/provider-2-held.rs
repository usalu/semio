//! ☁️ Controlled LAS persistence uses the owner's existing literal Header, Point and VLR records.
use super::Snapshot;
#[path="🪆️owner/🦀️.rs"]
mod owner;
use super::super::LasSnapshot;
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::io_schema::IoPayload;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};

fn fault(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

fn add(rows:&mut usize,additional:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(additional).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"LAS native entity count overflow"))?;if *rows>maximum{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"LAS native entities exceed row limit"))}Ok(())}
fn list(record:&RecordValue,id:u16)->Result<&[FieldValue],ValueError>{match record.get(id){Some(FieldValue::List(values))=>Ok(values),_=>Err(fault("LAS native list is missing or differs from its declared type"))}}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{FieldValue::Record(record)=>Ok(record),_=>Err(fault("LAS native entity differs from its declared record"))}}

fn forecast_record(value:&RecordValue,maximum:usize,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 let vlrs=list(value,2)?;let points=list(value,3)?;let mut rows=0;add(&mut rows,7,maximum)?;add(&mut rows,vlrs.len(),maximum)?;add(&mut rows,points.len(),maximum)?;
 control.scoped_stage(|control|{control.begin_stage(vlrs.len().checked_add(points.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"LAS native collection work overflow"))?)?;
 for value in vlrs{add(&mut rows,list(record(value)?,3)?.len(),maximum)?;control.step()?;}
 for value in points{let point=record(value)?;for id in[12,13]{if point.get(id).is_some_and(|field|!matches!(field,FieldValue::Absent)){add(&mut rows,1,maximum)?;}}control.step()?;}Ok(())})
}
fn forecast_snapshot(value:&LasSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 let mut rows=0;add(&mut rows,7,maximum)?;add(&mut rows,value.vlrs.len(),maximum)?;add(&mut rows,value.points.len(),maximum)?;
 control.scoped_stage(|control|{control.begin_stage(value.vlrs.len().checked_add(value.points.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"LAS native collection work overflow"))?)?;
 for vlr in &value.vlrs{add(&mut rows,vlr.data.len(),maximum)?;control.step()?;}
 for point in &value.points{add(&mut rows,usize::from(point.gps_time.is_some())+usize::from(point.rgb.is_some()),maximum)?;control.step()?;}Ok(())})
}
pub fn retire(value:LasSnapshot){owner::retire(value)}
/// 📥️ Uses one cumulative controller from borrowed envelope through direct actual owned fields.
pub fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<LasSnapshot,ValueError>{let limits=control.limits();let maximum=limits.max_rows;store::decode_sqlite_snapshot_record_native(payload,"stdio.las",Snapshot::__dsl_spec_producer(),|record,native|(||->Result<_,ValueError>{LasSnapshot::admit_sqlite_record(record,limits,native)?;forecast_record(record,maximum,native)?;owner::from_record(record,native)})(),control)}
/// 📤️ Emits the declared LAS record protocol with exact raw IEEE words and no external-file quantization.
pub fn encode(value:&LasSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{value.admit_sqlite_values(control,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative)?;let maximum=control.limits().max_rows;store::encode_sqlite_snapshot_record_native(encoding,"stdio.las",Snapshot::__dsl_spec_producer(),|native|(|| -> Result<_,ValueError>{forecast_snapshot(value,maximum,native)?;owner::to_record(value,native)})(),control)}
