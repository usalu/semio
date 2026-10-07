//! 🖼️ Image dimensions, frame pixel buffers, nullable ICC profiles and ordered metadata.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioImageSnapshot,SemioImageFrame,SemioImageMetadataEntry,SemioColorspace};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits each Image sample and nullable ICC cell through the same owned or borrowed writer.
pub(crate)fn visit_rows(snapshot:&SemioImageSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_image_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(i64::from(snapshot.width)),Cell::Integer(i64::from(snapshot.height)),Cell::Text(colorspace(snapshot.colorspace)),Cell::Integer(i64::from(snapshot.bit_depth)),match &snapshot.icc{None=>Cell::Null,Some(value)=>Cell::Blob(value)}])?;
 for(ordinal,frame)in snapshot.frames.iter().enumerate(){
  let key=out.insert("semio_image_frame",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Integer(i64::from(frame.delay_ms))])?;
  for(index,sample)in frame.rgba8.iter().enumerate(){out.insert("semio_image_sample",&[Cell::Integer(key),Cell::Integer(number(index)?),Cell::Integer(number(index%4)?),Cell::Integer(i64::from(*sample))])?;}
 }
 for(ordinal,entry)in snapshot.metadata.iter().enumerate(){out.insert("semio_image_metadata",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&entry.key),Cell::Text(&entry.value)])?;}Ok(())
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioImageSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image row identity or columns"))}else{Ok(())}}
fn colorspace(value:SemioColorspace)->&'static str{match value{SemioColorspace::Rgb=>"rgb",SemioColorspace::Rgba=>"rgba",SemioColorspace::Grayscale=>"grayscale",SemioColorspace::GrayscaleAlpha=>"grayscale_alpha",SemioColorspace::Indexed=>"indexed"}}
impl ArtifactSqliteSnapshot for SemioImageSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::image::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::image::io::sqlite::snapshot::native_decoding::decode(payload,control)}

fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="image"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_image_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioImageSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=database.table("semio_image_document")?.single_row()?;identity(document,7)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image document identifier"));}let width=u32::try_from(document.integer(2)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let height=u32::try_from(document.integer(3)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let bit_depth=u8::try_from(document.integer(5)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let colorspace=match document.text(4)?{"rgb"=>SemioColorspace::Rgb,"rgba"=>SemioColorspace::Rgba,"grayscale"=>SemioColorspace::Grayscale,"grayscale_alpha"=>SemioColorspace::GrayscaleAlpha,"indexed"=>SemioColorspace::Indexed,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio image colorspace"))};let icc=match &document.values[6]{SqliteValue::Null=>None,SqliteValue::Blob(value)=>Some(reconstruct_blob(control,value)?),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image ICC storage class"))};
let frame_rows=ordered(&database.table("semio_image_frame")?.rows,2,control)?;
let metadata_rows=ordered(&database.table("semio_image_metadata")?.rows,2,control)?;
let sample_rows=&database.table("semio_image_sample")?.rows;
let frame_ids=identities(&database.table("semio_image_frame")?.rows,4,control)?;
identities(&database.table("semio_image_metadata")?.rows,5,control)?;
identities(sample_rows,5,control)?;
let mut samples=transfer::reserve(sample_rows.len(),control)?;
for (at,row) in sample_rows.iter().enumerate(){if frame_ids.binary_search(&row.integer(1)?).is_err(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan Semio image sample"));}samples.push(row);if (at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,sample_rows.len())?;}}
transfer::heap_sort(&mut samples,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
let mut frames=transfer::reserve(frame_rows.len(),control)?;let mut completed=0usize;
for row in frame_rows{
if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image frame ownership"));}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,samples.len())?;
let start=samples.partition_point(|sample|sample.integer(1).is_ok_and(|id|id<row.rowid));let end=samples.partition_point(|sample|sample.integer(1).is_ok_and(|id|id<=row.rowid));
let mut rgba8=transfer::reserve(end-start,control)?;
for (ordinal,sample) in samples[start..end].iter().enumerate(){if sample.integer(2)?!=number(ordinal)?||sample.integer(3)?!=number(ordinal%4)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image sample ordinal or channel"));}rgba8.push(u8::try_from(sample.integer(4)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,samples.len())?;}}
frames.push(SemioImageFrame{delay_ms:u32::try_from(row.integer(3)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,rgba8});
}
if completed!=samples.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unconsumed Semio image sample"));}
let mut metadata=transfer::reserve(metadata_rows.len(),control)?;
for row in metadata_rows{if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio image metadata ownership"));}metadata.push(SemioImageMetadataEntry{key:reconstruct_text(control,row.text(3)?)?,value:reconstruct_text(control,row.text(4)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,width,height,colorspace,bit_depth,frames,icc,metadata})
    }
}

impl SemioImageSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(4)?;b.entities(self.frames.len())?;for frame in &self.frames{b.scalars(1)?;b.bytes(&frame.rgba8)?;}if let Some(icc)=&self.icc{b.bytes(icc)?;}b.entities(self.metadata.len())?;for entry in &self.metadata{b.text(&entry.key)?;b.text(&entry.value)?;}Ok(())}
}

impl SemioImageSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}
}


fn ordered<'a>(rows:&'a[SqliteRow],column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let mut ordered=transfer::reserve(rows.len(),control)?;for(at,row)in rows.iter().enumerate(){ordered.push(row);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut ordered,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.integer(column)?.cmp(&b.integer(column)?)))?;
 for(at,row)in ordered.iter().enumerate(){if row.integer(column)?!=number(at)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio image ordinals require contiguous occurrences"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(ordered)
}
fn identities(rows:&[SqliteRow],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<i64>,ValueError>{
 let mut ids=transfer::reserve(rows.len(),control)?;for(at,row)in rows.iter().enumerate(){identity(row,columns)?;ids.push(row.rowid);if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}
 transfer::heap_sort(&mut ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;
 for(at,id)in ids.iter().enumerate(){if at>0&&ids[at-1]==*id{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio image row identity"));}if(at+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,at+1,rows.len())?;}}Ok(ids)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
