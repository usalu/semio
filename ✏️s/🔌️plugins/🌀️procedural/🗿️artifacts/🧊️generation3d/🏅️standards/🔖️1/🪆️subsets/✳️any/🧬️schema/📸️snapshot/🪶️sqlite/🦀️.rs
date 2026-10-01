//! 🧊️ Owned Generation3d retained field contract and exact declared SQLite capability.
use super::Generation3dSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
#[path="../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🦀️.rs"]mod typed;
impl ArtifactSqliteSnapshot for Generation3dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn retire_sqlite_snapshot(self){self.retire_cold();}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{typed::project(&self.host_snapshot,&self.generation,Self::SQLITE_SCHEMA,c)}
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{let(host_snapshot,generation)=typed::reconstruct(d,Self::SQLITE_SCHEMA,c)?;Ok(Self{host_snapshot,generation})}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),String>{typed::preflight(&self.host_snapshot,&self.generation,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;if dialect.artifact_kind!="s.procedural.generation3d"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("Generation3d dialect differs from its owned wildcard").into())}if d.table("generation_host")?.single_row()?.text(1)?!=self.host_snapshot.schema{return Err(String::from("Generation3d projected host schema differs").into())}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(store::io_schema::IoOutcome::clean(()))}
}
