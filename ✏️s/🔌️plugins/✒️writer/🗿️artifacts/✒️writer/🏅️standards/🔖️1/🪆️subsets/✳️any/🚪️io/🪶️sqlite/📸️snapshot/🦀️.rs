//! ✒️ Writer-owned strings and fixed persisted document handle use two handcrafted entities.
use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,Reconstruction}}};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}
#[path="🛂️native/🦀️.rs"]mod native;
fn document_cells(value:&WriterSnapshot)->[Cell<'_>;5]{[Cell::Text(&value.schema),Cell::Text(&value.id),Cell::Text(&value.language_id),Cell::Text(&value.uri),Cell::Text(&value.text)]}
fn child_cells(value:&WriterSnapshot)->[Cell<'_>;6]{let child=&value.document;[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Text(&child.target.artifact_id),Cell::Text(&child.target.dialect.artifact_kind),Cell::Text(&child.target.dialect.standard),Cell::Text(&child.target.dialect.subset)]}

impl ArtifactSqliteSnapshot for WriterSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{native::decode(payload,control,native_control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{native::encode(self,encoding,control,native_owner)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;
  for text in[&self.schema,&self.id,&self.language_id,&self.uri,&self.text,&self.document.child_id,&self.document.target.artifact_id,&self.document.target.dialect.artifact_kind,&self.document.target.dialect.standard,&self.document.target.dialect.subset]{bound.repeated(text.len(),6)?;}
  bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;p.check_rows(2)?;
  p.insert_key("writer_document",1,&document_cells(self))?;
  p.insert_key("writer_document_child",1,&child_cells(self))?;p.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;
  if database.tables.len()!=2{return Err(invalid("Writer requires exactly its two owned tables"))}
  let document=database.table("writer_document")?.single_row()?;let child=database.table("writer_document_child")?.single_row()?;
  if document.integer(0)?!=document.rowid||document.values.len()!=6||child.integer(0)?!=child.rowid||child.integer(1)?!=document.rowid||child.values.len()!=7{return Err(invalid("Writer fixed ownership differs"))}
  let mut owned=Reconstruction::new(control)?;
  let result=Self{schema:owned.text(document.text(1)?)?,id:owned.text(document.text(2)?)?,language_id:owned.text(document.text(3)?)?,uri:owned.text(document.text(4)?)?,text:owned.text(document.text(5)?)?,document:store::ArtifactChild::new(owned.text(child.text(2)?)?,semio_framework_artifact_reference::ArtifactRef{artifact_id:owned.text(child.text(3)?)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:owned.text(child.text(4)?)?,standard:owned.text(child.text(5)?)?,subset:owned.text(child.text(6)?)?}})};
  owned.checkpoint()?;Ok(result)
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?;
  if dialect.artifact_kind!="s.writer.writer"||dialect.standard!="1"||dialect.subset!="*"{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(invalid("Writer does not own this semantic subset")))}
  if Self::from_sqlite_database(database,control).map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)?!=*self{return Err(semio_framework_os_kernel::io_schema::IoError::from_value_error(invalid("Writer document identity disagrees with its snapshot")))}
  Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

