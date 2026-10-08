//! ✒️ Writer native boundary with one controlled producer and literal two-row admission.
use super::WriterSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase}};
fn admit(control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if WriterSnapshot::SQLITE_SCHEMA.len()>control.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Writer authored schema byte limit exceeded"))}
 control.check_rows(2)
}

fn admit_values(snapshot:&WriterSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 let document=super::document_cells(snapshot);let child=super::child_cells(snapshot);let mut total=0usize;
 for(index,cells)in[document.as_slice(),child.as_slice()].into_iter().enumerate(){
  total=total.checked_add(8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Writer semantic identity bytes overflow"))?;
  for cell in cells{let size=match cell{store::sqlite_snapshot::artifact::Cell::Null=>0,store::sqlite_snapshot::artifact::Cell::Integer(_)|store::sqlite_snapshot::artifact::Cell::Real(_)|store::sqlite_snapshot::artifact::Cell::Float32(_)=>8,store::sqlite_snapshot::artifact::Cell::Text(value)=>value.len(),store::sqlite_snapshot::artifact::Cell::PagedText(value)=>value.text_bytes(),store::sqlite_snapshot::artifact::Cell::Blob(value)=>value.len()};total=total.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Writer semantic value bytes overflow"))?;control.check_value_bytes(total)?;}
  control.checkpoint(phase,index+1,2)?;
 }Ok(())
}

pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<WriterSnapshot,ValueError>{
 control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,2)?;
 admit(control)?;
 let value=store::decode_sqlite_snapshot_record_native(payload,<WriterSnapshot as store::ArtifactDsl>::envelope_id(),WriterSnapshot::__dsl_spec_producer(),|record,native|WriterSnapshot::__dsl_from_record_controlled(record,native),control)?;admit_values(&value,control,SqliteSnapshotPhase::DecodeNative)?;Ok(value)
}
pub(super) fn encode(snapshot:&WriterSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{
 control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,2)?;
 admit(control)?;admit_values(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;
 store::encode_sqlite_snapshot_record_native(encoding,<WriterSnapshot as store::ArtifactDsl>::envelope_id(),WriterSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)
}
