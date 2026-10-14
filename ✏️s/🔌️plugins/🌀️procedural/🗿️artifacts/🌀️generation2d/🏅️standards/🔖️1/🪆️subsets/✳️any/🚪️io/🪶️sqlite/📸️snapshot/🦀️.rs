//! 🌀️ Owned Generation2d retained field contract and exact declared SQLite capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
use semio_s_procedural_core_generation_sqlite as typed;
impl ArtifactSqliteSnapshotHost for Generation2dSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{c.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit"))}semio_s_artifact_procedural_2d_io_text::snapshot::decode_sqlite_native(payload,c,native_owner)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit"))}semio_s_artifact_procedural_2d_io_text::snapshot::encode_sqlite_native(self,encoding,c,native_owner)}
 fn retire_sqlite_snapshot(self){self.retire_cold();}
 
 
 
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{typed::preflight(&self.host_snapshot,&self.generation,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;if dialect.artifact_kind!="s.procedural.generation2d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation2d dialect differs from its owned wildcard")))}if d.table("generation_host").map_err(|error|store::io_schema::IoError::from_value_error(error))?.single_row().map_err(|error|store::io_schema::IoError::from_value_error(error))?.text(1).map_err(|error|store::io_schema::IoError::from_value_error(error))?!=self.host_snapshot.schema{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation2d projected host schema differs")))}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;Ok(store::io_schema::IoOutcome::clean(()))}
}
impl pack::snapshot::ArtifactSqliteSnapshot for Generation2dSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{typed::project(&self.host_snapshot,&self.generation,Self::SQLITE_SCHEMA,c)}
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let(host_snapshot,generation)=typed::reconstruct(d,Self::SQLITE_SCHEMA,c)?;Ok(Self{host_snapshot,generation})}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

