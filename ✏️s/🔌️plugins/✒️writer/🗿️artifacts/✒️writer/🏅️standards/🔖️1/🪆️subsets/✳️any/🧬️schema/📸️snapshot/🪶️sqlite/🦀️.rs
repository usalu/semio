//! ✒️ Writer-owned strings and fixed persisted document handle use two handcrafted entities.
use super::WriterSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,Reconstruction}}};
impl ArtifactSqliteSnapshot for WriterSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;
  for text in[&self.schema,&self.id,&self.language_id,&self.uri,&self.text,&self.document.child_id,&self.document.target.artifact_id,&self.document.target.dialect.artifact_kind,&self.document.target.dialect.standard,&self.document.target.dialect.subset]{bound.repeated(text.len(),6)?;}
  bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;p.check_rows(2)?;
  p.insert_key("writer_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.id),Cell::Text(&self.language_id),Cell::Text(&self.uri),Cell::Text(&self.text)])?;
  let child=&self.document;p.insert_key("writer_document_child",1,&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Text(&child.target.artifact_id),Cell::Text(&child.target.dialect.artifact_kind),Cell::Text(&child.target.dialect.standard),Cell::Text(&child.target.dialect.subset)])?;p.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|error|error.to_string())?;
  if database.tables.len()!=2{return Err("Writer requires exactly its two owned tables".into())}
  let document=database.table("writer_document")?.single_row()?;let child=database.table("writer_document_child")?.single_row()?;
  if document.integer(0)?!=document.rowid||document.values.len()!=6||child.integer(0)?!=child.rowid||child.integer(1)?!=document.rowid||child.values.len()!=7{return Err("Writer fixed ownership differs".into())}
  let mut owned=Reconstruction::new(control)?;
  let result=Self{schema:owned.text(document.text(1)?)?,id:owned.text(document.text(2)?)?,language_id:owned.text(document.text(3)?)?,uri:owned.text(document.text(4)?)?,text:owned.text(document.text(5)?)?,document:store::ArtifactChild::new(owned.text(child.text(2)?)?,store::os_io::ArtifactRef{artifact_id:owned.text(child.text(3)?)?,dialect:store::os_io::ArtifactDialect{artifact_kind:owned.text(child.text(4)?)?,standard:owned.text(child.text(5)?)?,subset:owned.text(child.text(6)?)?}})};
  owned.checkpoint()?;Ok(result)
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
  if dialect.artifact_kind!="s.writer.writer"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("Writer does not own this semantic subset").into())}
  if Self::from_sqlite_database(database,control)?!=*self{return Err(String::from("Writer document identity disagrees with its snapshot").into())}
  Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
 }
}
