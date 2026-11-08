//! 🛂️ Process3d opt-in and genuine controlled DSL/Pack endpoint producers.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{Process3dSnapshot,SQL};
use semio_framework_value::ValueError;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for Process3dSnapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome {let maximum=control.limits().max_value_bytes;store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(), &mut |record,snapshot_output,native| { let constructed: Result<_, ValueError> = (|| {super::semantic::borrowed(record,maximum,native)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control,output)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {if let Err(error)=(||->Result<(),semio_framework_value::ValueError>{super::semantic::owned(self,control)?;Ok(())})(){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(), &mut |native|self.__dsl_to_record_controlled(native),control,native_owner,output)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::forecast(self,control,SqliteSnapshotPhase::EncodeNative)?;super::semantic::owned(self,control)}
 fn retire_sqlite_snapshot(self){super::ownership::retire(self);}
}
impl pack::snapshot::ArtifactSqliteSnapshot for Process3dSnapshot {
const SQLITE_SCHEMA:&'static str=SQL;
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{super::project(self,control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::reconstruct(database,control)}
}
