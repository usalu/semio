//! 🌿️ Handwritten VCS document and ordered-tag relational ownership.
use super::VcsSnapshot;
use std::collections::BTreeSet;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound}}};

fn identity(row:&SqliteRow,columns:usize)->Result<(),String>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid{return Err("VCS requires exact fields and positive aliased identities".into())}Ok(())}
impl ArtifactSqliteSnapshot for VcsSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  let maximum=control.limits().max_rows;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{
   let count=match record.get(5){Some(dsl::FieldValue::List(values))=>values.len(),None|Some(dsl::FieldValue::Absent)=>0,_=>return Err(dsl::__rt::field_error("VCS tags require a literal list"))};
   if count.checked_add(1).filter(|count|*count<=maximum).is_none(){return Err(dsl::__rt::field_error("VCS native snapshot exceeds row limit"))}
   Self::__dsl_from_record_controlled(record,native)
  },control)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(self.tags.len().checked_add(1).ok_or("VCS semantic row count overflow")?)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
  control.check_rows(self.tags.len().checked_add(1).ok_or("VCS row count overflow")?)?;let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;
  for text in [&self.schema,&self.title,&self.notes,&self.status]{bound.repeated(text.len(),24)?;}
  for tag in &self.tags{bound.add(1024)?;bound.repeated(tag.len(),24)?;}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;control.check_rows(self.tags.len().checked_add(1).ok_or("VCS row count overflow")?)?;
  let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;out.insert_key("vcs_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.title),Cell::Integer(self.counter),Cell::Text(&self.notes),Cell::Text(&self.status)])?;
  for(ordinal,tag)in self.tags.iter().enumerate(){out.insert("vcs_tag",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|e.to_string())?),Cell::Text(tag)])?;}out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|e|e.to_string())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=&database.table("vcs_document")?.rows;if documents.len()!=1||documents[0].rowid!=1{return Err("VCS requires one document identity".into())}
  let document=&documents[0];identity(document,6)?;let rows=&database.table("vcs_tag")?.rows;let maximum=control.limits().max_value_bytes;
  let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=dsl::NativeDecodeControl::new(maximum,&mut progress);
  native.begin_stage(rows.len())?;native.charge(rows.len().checked_mul(48).ok_or("VCS identity workspace overflow")?)?;let mut identities=BTreeSet::new();
  let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.len())?;ordered.resize(rows.len(),None);
  for row in rows{native.step()?;identity(row,4)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|e.to_string())?;if row.integer(1)?!=1||!identities.insert(row.rowid)||ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err("VCS tags require the document parent, unique identities and dense ordinals".into())}row.text(3)?;}
  native.begin_stage(rows.len().checked_add(1).ok_or("VCS workload overflow")?)?;native.charge(std::mem::size_of::<Self>())?;
  let schema=native.copy_text(document.text(1)?)?;let title=native.copy_text(document.text(2)?)?;let counter=document.integer(3)?;let notes=native.copy_text(document.text(4)?)?;let status=native.copy_text(document.text(5)?)?;native.step()?;
  let mut tags=native.allocate_vec::<String>(ordered.len())?;for row in ordered{native.step()?;tags.push(native.copy_text(row.ok_or("VCS missing ordinal")?.text(3)?)?);}native.checkpoint()?;Ok(Self{schema,title,counter,notes,status,tags})
 }
}
