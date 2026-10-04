//! 🎬️ Individually handwritten figure, tile geometry and composed child relational ownership.
use super::PresentationSnapshot;
use crate::{FigureTileSource,FigureTileDraft,FigureTileFrame};
use std::collections::BTreeSet;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,FloatColumn,insert_key_ieee754,insert_ieee754,read_binary64,ieee754_is_null}}};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

const SOURCE:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const TILE:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("Presentation requires complete positive entities attached to its document"))}Ok(())}
fn frame(row:&SqliteRow,start:usize,columns:&[FloatColumn])->Result<FigureTileFrame,ValueError>{Ok(FigureTileFrame{x:read_binary64(row,start,columns)?,y:read_binary64(row,start+1,columns)?,width:read_binary64(row,start+2,columns)?,height:read_binary64(row,start+3,columns)?})}
fn project_child<S>(out:&mut Projection<'_,'_>,slot:&str,child:&store::ArtifactChild<S>)->Result<(),ValueError>{let t=&child.target;out.insert("presentation_child",&[Cell::Integer(1),Cell::Text(slot),Cell::Text(&child.child_id),Cell::Text(&t.artifact_id),Cell::Text(&t.dialect.artifact_kind),Cell::Text(&t.dialect.standard),Cell::Text(&t.dialect.subset)])?;Ok(())}
fn restore_child<S>(row:&SqliteRow,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<store::ArtifactChild<S>,ValueError>{native.scoped_stage(|native|{native.begin_stage(5)?;let child_id=native.copy_text(row.text(3)?)?;native.step()?;let artifact_id=native.copy_text(row.text(4)?)?;native.step()?;let artifact_kind=native.copy_text(row.text(5)?)?;native.step()?;let standard=native.copy_text(row.text(6)?)?;native.step()?;let subset=native.copy_text(row.text(7)?)?;native.step()?;Ok(store::ArtifactChild::new(child_id,store::io_schema::ArtifactRef{artifact_id,dialect:store::io_schema::ArtifactDialect{artifact_kind,standard,subset}}))})}
fn bound_child<S>(bound:&mut NativeEncodingBound<'_,'_>,child:&store::ArtifactChild<S>)->Result<(),ValueError>{let t=&child.target;for text in[&child.child_id,&t.artifact_id,&t.dialect.artifact_kind,&t.dialect.standard,&t.dialect.subset]{bound.repeated(text.len(),24)?}Ok(())}
impl ArtifactSqliteSnapshot for PresentationSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  control.check_rows(self.tiles.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation row count overflow"))?)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(4)?;let maximum=control.limits().max_rows;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{
   let count=match record.get(2){Some(semio_framework_dsl_record::FieldValue::List(values))=>values.len(),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>0,_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Presentation tiles require a literal list").to_string()))};
   if count.checked_add(4).filter(|count|*count<=maximum).is_none(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Presentation native snapshot exceeds row limit").to_string()))}
   Self::__dsl_from_record_controlled(record,native)
  },control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  control.check_rows(self.tiles.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation row count overflow"))?)?;let mut bound=NativeEncodingBound::new(control)?;bound.add(8192)?;
  for text in[&self.schema,&self.source.src,&self.source.kind]{bound.repeated(text.len(),24)?}
  for tile in &self.tiles{bound.add(1024)?;bound.repeated(tile.id.len(),24)?;bound.repeated(tile.name.len(),24)?}
  bound_child(&mut bound,&self.presentation)?;bound_child(&mut bound,&self.animation)?;bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;control.check_rows(self.tiles.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation row count overflow"))?)?;let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;
  out.insert_key("presentation_document",1,&[Cell::Text(&self.schema)])?;let source=&self.source;let f=&source.frame;
  insert_key_ieee754(&mut out,"presentation_source",1,&[Cell::Integer(1),Cell::Text(&source.src),Cell::Text(&source.kind),Cell::Real(f.x),Cell::Real(f.y),Cell::Real(f.width),Cell::Real(f.height),source.source_aspect.map_or(Cell::Null,Cell::Real),source.pdf_page.map_or(Cell::Null,|v|Cell::Integer(i64::from(v)))],SOURCE)?;
  for(ordinal,tile)in self.tiles.iter().enumerate(){let f=&tile.crop;insert_ieee754(&mut out,"presentation_tile",&[Cell::Integer(1),Cell::Integer(i64::try_from(ordinal).map_err(|e|invalid(e.to_string()))?),Cell::Text(&tile.id),Cell::Text(&tile.name),Cell::Real(f.x),Cell::Real(f.y),Cell::Real(f.width),Cell::Real(f.height)],TILE)?;}
  project_child(&mut out,"presentation",&self.presentation)?;project_child(&mut out,"animation",&self.animation)?;out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=&database.table("presentation_document")?.rows;let sources=&database.table("presentation_source")?.rows;let rows=&database.table("presentation_tile")?.rows;let children=&database.table("presentation_child")?.rows;
  if documents.len()!=1||documents[0].rowid!=1||documents[0].values.len()!=2||documents[0].integer(0)?!=1||sources.len()!=1||sources[0].rowid!=1||children.len()!=2{return Err(invalid("Presentation requires one document, one source and its two child slots"))}
  let source=&sources[0];identity(source,20)?;let mut presentation=None;let mut animation=None;
  for child in children{identity(child,8)?;match child.text(2)?{"presentation" if presentation.is_none()=>presentation=Some(child),"animation" if animation.is_none()=>animation=Some(child),_=>return Err(invalid("Presentation requires its two distinct child slots"))}}
  let maximum=control.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=semio_framework_value::NativeDecodeControl::new(maximum,&mut progress);
  native.begin_stage(rows.len())?;native.charge(rows.len().checked_mul(48).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation identity workspace overflow"))?)?;let mut ids=BTreeSet::new();let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.len())?;ordered.resize(rows.len(),None);
  for row in rows{native.step()?;identity(row,17)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;if !ids.insert(row.rowid)||ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err(invalid("Presentation tiles require unique entities and dense ordinals"))}}
  native.begin_stage(rows.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation workload overflow"))?)?;native.charge(std::mem::size_of::<Self>())?;
  let schema=native.copy_text(documents[0].text(1)?)?;native.step()?;let src=native.copy_text(source.text(2)?)?;let kind=native.copy_text(source.text(3)?)?;
  let source=FigureTileSource{src,kind,frame:frame(source,4,SOURCE)?,source_aspect:if ieee754_is_null(source,8,SOURCE)?{None}else{Some(read_binary64(source,8,SOURCE)?)},pdf_page:if source.values.get(9)==Some(&SqliteValue::Null){None}else{Some(u32::try_from(source.integer(9)?).map_err(|e|invalid(e.to_string()))?)}};native.step()?;
  let presentation=restore_child(presentation.ok_or_else(||invalid("Presentation missing presentation child"))?,&mut native)?;native.step()?;let animation=restore_child(animation.ok_or_else(||invalid("Presentation missing animation child"))?,&mut native)?;native.step()?;
  let mut tiles=native.allocate_vec::<FigureTileDraft>(ordered.len())?;for row in ordered{native.step()?;let row=row.ok_or_else(||invalid("Presentation missing tile ordinal"))?;tiles.push(FigureTileDraft{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,crop:frame(row,5,TILE)?});}native.checkpoint()?;Ok(Self{schema,source,tiles,presentation,animation})
 }
}
