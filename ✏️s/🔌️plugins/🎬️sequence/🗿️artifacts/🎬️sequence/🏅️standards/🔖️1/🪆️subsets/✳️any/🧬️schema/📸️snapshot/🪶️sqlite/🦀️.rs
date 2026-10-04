//! 🎬️ Handwritten parent schema and literal composed-child relational ownership.
use super::SequenceSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_diagnostic::{TextError,TextSpan};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound}}};
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Sequence requires exact fields and positive aliased identities"))}Ok(())}
impl ArtifactSqliteSnapshot for SequenceSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn retire_sqlite_snapshot(self){neural_engine::ColdRetire::retire_cold(self)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(2)?;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),control)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(2)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  control.check_rows(2)?;let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;let target=&self.content.target;
  for text in[&self.schema,&self.content.child_id,&target.artifact_id,&target.dialect.artifact_kind,&target.dialect.standard,&target.dialect.subset]{bound.repeated(text.len(),24)?;}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,2)?;control.check_rows(2)?;let target=&self.content.target;let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;
  out.insert_key("sequence_document",1,&[Cell::Text(&self.schema)])?;
  out.insert("sequence_content",&[Cell::Integer(1),Cell::Text(&self.content.child_id),Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])?;out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=&database.table("sequence_document")?.rows;let children=&database.table("sequence_content")?.rows;
  if documents.len()!=1||documents[0].rowid!=1||children.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Sequence requires one document and one content child"))}
  let document=&documents[0];let child=&children[0];identity(document,2)?;identity(child,7)?;if child.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Sequence child requires the document parent"))}
  let maximum=control.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=semio_framework_value::NativeDecodeControl::new(maximum,&mut progress);
  native.begin_stage(6)?;native.charge(std::mem::size_of::<Self>())?;
  let schema=native.copy_text(document.text(1)?)?;native.step()?;let child_id=native.copy_text(child.text(2)?)?;native.step()?;let artifact_id=native.copy_text(child.text(3)?)?;native.step()?;
  let artifact_kind=native.copy_text(child.text(4)?)?;native.step()?;let standard=native.copy_text(child.text(5)?)?;native.step()?;let subset=native.copy_text(child.text(6)?)?;native.step()?;native.checkpoint()?;
  Ok(Self{schema,content:store::ArtifactChild::new(child_id,store::io_schema::ArtifactRef{artifact_id,dialect:store::io_schema::ArtifactDialect{artifact_kind,standard,subset}})})
 }
}
