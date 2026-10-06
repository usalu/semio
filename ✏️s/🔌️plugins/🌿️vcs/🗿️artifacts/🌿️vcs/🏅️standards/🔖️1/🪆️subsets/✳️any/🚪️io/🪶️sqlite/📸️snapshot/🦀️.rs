//! 🌿️ Handwritten VCS document and ordered-tag relational ownership.
use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Cell,NativeEncodingBound,RowWriter},SqliteDatabaseLimits}};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}


const TABLES:[(&str,usize);2]=[("vcs_document",6),("vcs_tag",4)];
fn extent(limits:SqliteDatabaseLimits)->Result<(),ValueError>{let bytes=VcsSnapshot::SQLITE_SCHEMA.split(';').map(str::trim).filter(|sql|!sql.is_empty()).try_fold(0usize,|sum,sql|sum.checked_add(sql.len()).ok_or_else(||invalid("VCS schema byte overflow")))?;let bytes=TABLES.iter().try_fold(bytes,|sum,(name,_)|sum.checked_add(name.len()).ok_or_else(||invalid("VCS table-name byte overflow")))?;if bytes>limits.max_schema_bytes||TABLES.len()>limits.max_tables||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<1{return Err(invalid("VCS authored schema extent exceeds caller limits"))}Ok(())}

fn visit_rows(snapshot:&VcsSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.check_rows(snapshot.tags.len().checked_add(1).ok_or_else(||invalid("VCS semantic row count overflow"))?)?;
 out.insert_key("vcs_document",1,&[Cell::Text(&snapshot.schema),Cell::Text(&snapshot.title),Cell::Integer(snapshot.counter),Cell::Text(&snapshot.notes),Cell::Text(&snapshot.status)])?;
 for(ordinal,tag)in snapshot.tags.iter().enumerate(){out.insert("vcs_tag",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Text(tag)])?;}Ok(())
}
fn admit_typed(snapshot:&VcsSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{extent(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
fn admit_record(record:&semio_framework_dsl_record::RecordValue,limits:SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 use semio_framework_dsl_record::FieldValue as F;
 extent(limits)?;native.scoped_stage(|native|{
  if record.fields.len()!=6||record.fields.keys().any(|id|*id>=6){return Err(invalid("VCS native record requires six authored fields"))}
  let Some(F::List(tags))=record.get(5)else{return Err(invalid("VCS tags require literal list"))};
  let rows=tags.len().checked_add(1).ok_or_else(||invalid("VCS semantic row count overflow"))?;if rows>limits.max_rows{return Err(invalid("VCS semantic row limit exceeded"))}
  if !matches!(record.get(2),Some(F::Int(_))){return Err(invalid("VCS counter requires signed64"))}
  native.begin_stage(rows)?;let mut bytes=16usize;
  let text=|value:&str,native:&mut semio_framework_value::NativeDecodeControl<'_>|->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){native.advance(part.len())?;}Ok(())})};
  for id in[0,1,3,4]{let Some(F::Text(value))=record.get(id)else{return Err(invalid("VCS authored document field requires literal text"))};bytes=bytes.checked_add(value.len()).ok_or_else(||invalid("VCS semantic cell byte count overflow"))?;text(value,native)?;}
  if bytes>limits.max_value_bytes{return Err(invalid("VCS document semantic value limit exceeded"))}native.step()?;
  for tag in tags{let F::Text(value)=tag else{return Err(invalid("VCS tag requires literal text"))};bytes=bytes.checked_add(24).and_then(|bytes|bytes.checked_add(value.len())).ok_or_else(||invalid("VCS tag semantic cell byte count overflow"))?;if bytes>limits.max_value_bytes{return Err(invalid("VCS tag semantic value limit exceeded"))}text(value,native)?;native.step()?;}native.checkpoint()
 })
}

fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid{return Err(invalid("VCS requires exact fields and positive aliased identities"))}Ok(())}
impl ArtifactSqliteSnapshot for VcsSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let limits=control.limits();extent(limits)?;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{
   admit_record(record,limits,native)?;
   Self::__dsl_from_record_controlled(record,native)
  },control)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  admit_typed(self,SqliteSnapshotPhase::EncodeNative,control)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  admit_typed(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut bound=NativeEncodingBound::file_only(control)?;bound.add(8192)?;
  for text in [&self.schema,&self.title,&self.notes,&self.status]{bound.repeated(text.len(),24)?;}
  for tag in &self.tags{bound.add(1024)?;bound.repeated(tag.len(),24)?;}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  extent(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  use store::sqlite_snapshot::{artifact::{ordered_row_refs,reconstruct_text},transfer::{reserve,heap_sort}};
  let phase=SqliteSnapshotPhase::ReconstructSnapshot;extent(control.limits())?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,phase)?;
  let documents=&database.table("vcs_document")?.rows;if documents.len()!=1||documents[0].rowid!=1{return Err(invalid("VCS requires one document identity"))}
  let document=&documents[0];identity(document,6)?;let table=database.table("vcs_tag")?;
  for(index,row)in table.rows.iter().enumerate(){identity(row,4)?;if row.integer(1)?!=1{return Err(invalid("VCS tag requires document parent"))}row.text(3)?;if(index+1)%256==0{control.checkpoint(phase,index+1,table.rows.len())?;}}
  let mut ordered=ordered_row_refs(table,2,control)?;
  heap_sort(&mut ordered,phase,control,|left,right,_|Ok(left.rowid.cmp(&right.rowid)))?;
  for(index,pair)in ordered.windows(2).enumerate(){if pair[0].rowid==pair[1].rowid{return Err(invalid("VCS tags require unique identities"))}if(index+1)%256==0{control.checkpoint(phase,index+1,ordered.len())?;}}
  heap_sort(&mut ordered,phase,control,|left,right,_|Ok(left.integer(2)?.cmp(&right.integer(2)?)))?;
  let schema=reconstruct_text(control,document.text(1)?)?;let title=reconstruct_text(control,document.text(2)?)?;let counter=document.integer(3)?;let notes=reconstruct_text(control,document.text(4)?)?;let status=reconstruct_text(control,document.text(5)?)?;
  let mut tags=reserve::<String>(ordered.len(),control)?;let total=ordered.len().checked_add(1).ok_or_else(||invalid("VCS reconstruction row count overflow"))?;control.checkpoint(phase,1,total)?;
  for(index,row)in ordered.into_iter().enumerate(){tags.push(reconstruct_text(control,row.text(3)?)?);if(index+2)%256==0{control.checkpoint(phase,index+2,total)?;}}
  control.checkpoint(phase,total,total)?;Ok(Self{schema,title,counter,notes,status,tags})
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

