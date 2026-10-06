//! 🛂️ Process3d opt-in and genuine controlled DSL/Pack endpoint producers.
use super::{Process3dSnapshot,SQL};
use semio_framework_value::ValueError;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};

impl store::ArtifactSqliteSnapshot for Process3dSnapshot{
 const SQLITE_SCHEMA:&'static str=SQL;
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{super::project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::reconstruct(database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let maximum=control.limits().max_value_bytes;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{super::semantic::borrowed(record,maximum,native)?;Self::__dsl_from_record_controlled(record,native)},control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{super::semantic::owned(self,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::forecast(self,control,SqliteSnapshotPhase::EncodeNative)?;super::semantic::owned(self,control)}
 fn retire_sqlite_snapshot(self){super::ownership::retire(self);}
}
