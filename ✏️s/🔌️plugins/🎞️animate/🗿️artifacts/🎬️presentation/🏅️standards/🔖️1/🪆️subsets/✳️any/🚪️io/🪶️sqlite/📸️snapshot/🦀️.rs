//! 🎬️ Individually handwritten figure, tile geometry and composed child relational ownership.
use crate::standards::v1::subsets::any::schema::snapshot::PresentationSnapshot;
use crate::{FigureTileSource,FigureTileDraft,FigureTileFrame};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound,FloatColumn,insert_key_ieee754,insert_ieee754,read_binary64,ieee754_is_null}}};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

const SOURCE:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const TILE:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("Presentation requires complete positive entities attached to its document"))}Ok(())}
fn frame(row:&SqliteRow,start:usize,columns:&[FloatColumn])->Result<FigureTileFrame,ValueError>{Ok(FigureTileFrame{x:read_binary64(row,start,columns)?,y:read_binary64(row,start+1,columns)?,width:read_binary64(row,start+2,columns)?,height:read_binary64(row,start+3,columns)?})}
fn project_child<S>(out:&mut Projection<'_,'_>,slot:&str,child:&store::ArtifactChild<S>)->Result<(),ValueError>{let t=&child.target;out.insert("presentation_child",&[Cell::Integer(1),Cell::Text(slot),Cell::Text(&child.child_id),Cell::Text(&t.artifact_id),Cell::Text(&t.dialect.artifact_kind),Cell::Text(&t.dialect.standard),Cell::Text(&t.dialect.subset)])?;Ok(())}
fn restore_child<S>(row:&SqliteRow,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<store::ArtifactChild<S>,ValueError>{native.scoped_stage(|native|{native.begin_stage(5)?;let child_id=native.copy_text(row.text(3)?)?;native.step()?;let artifact_id=native.copy_text(row.text(4)?)?;native.step()?;let artifact_kind=native.copy_text(row.text(5)?)?;native.step()?;let standard=native.copy_text(row.text(6)?)?;native.step()?;let subset=native.copy_text(row.text(7)?)?;native.step()?;Ok(store::ArtifactChild::new(child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind,standard,subset}}))})}
fn bound_child<S>(bound:&mut NativeEncodingBound<'_,'_>,child:&store::ArtifactChild<S>)->Result<(),ValueError>{let t=&child.target;for text in[&child.child_id,&t.artifact_id,&t.dialect.artifact_kind,&t.dialect.standard,&t.dialect.subset]{bound.repeated(text.len(),24)?}Ok(())}
impl ArtifactSqliteSnapshot for PresentationSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
  semantic_owned(self,control.limits().max_value_bytes,&mut SemanticCheck(control))?;control.check_rows(self.tiles.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation row count overflow"))?)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  control.check_rows(4)?;let maximum=control.limits().max_rows;let maximum_bytes=control.limits().max_value_bytes;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{
   let count=match record.get(2){Some(semio_framework_dsl_record::FieldValue::List(values))=>values.len(),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>0,_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Presentation tiles require a literal list").to_string()))};
   if count.checked_add(4).filter(|count|*count<=maximum).is_none(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("Presentation native snapshot exceeds row limit").to_string()))}
   semantic_borrowed(record,maximum_bytes,native)?;Self::__dsl_from_record_controlled(record,native)
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
  control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint|{let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining,&mut progress);let result=(||{
  native.begin_stage(rows.len())?;let mut ids=native.allocate_vec::<i64>(rows.len())?;let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.len())?;ordered.resize(rows.len(),None);
  for row in rows{native.step()?;identity(row,17)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;ids.push(row.rowid);if ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err(invalid("Presentation tiles require unique entities and dense ordinals"))}}
  sort_ids(&mut ids,&mut native)?;if ids.windows(2).any(|p|p[0]==p[1]){return Err(invalid("Presentation tiles require unique entities and dense ordinals"))}native.begin_stage(rows.len().checked_add(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"Presentation workload overflow"))?)?;native.charge(std::mem::size_of::<Self>())?;
  let schema=native.copy_text(documents[0].text(1)?)?;native.step()?;let src=native.copy_text(source.text(2)?)?;let kind=native.copy_text(source.text(3)?)?;
  let source=FigureTileSource{src,kind,frame:frame(source,4,SOURCE)?,source_aspect:if ieee754_is_null(source,8,SOURCE)?{None}else{Some(read_binary64(source,8,SOURCE)?)},pdf_page:if source.values.get(9)==Some(&SqliteValue::Null){None}else{Some(u32::try_from(source.integer(9)?).map_err(|e|invalid(e.to_string()))?)}};native.step()?;
  let presentation=restore_child(presentation.ok_or_else(||invalid("Presentation missing presentation child"))?,&mut native)?;native.step()?;let animation=restore_child(animation.ok_or_else(||invalid("Presentation missing animation child"))?,&mut native)?;native.step()?;
  let mut tiles=native.allocate_vec::<FigureTileDraft>(ordered.len())?;for row in ordered{native.step()?;let row=row.ok_or_else(||invalid("Presentation missing tile ordinal"))?;tiles.push(FigureTileDraft{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,crop:frame(row,5,TILE)?});}native.checkpoint()?;Ok(Self{schema,source,tiles,presentation,animation})})();(result,native.owned_bytes())})?
 }
}

struct SemanticCheck<'a,'p>(&'a mut SqliteSnapshotControl<'p>);
impl SemanticCheck<'_,'_>{fn step(&mut self)->Result<(),ValueError>{self.0.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)}}
trait CellCheck{fn step(&mut self)->Result<(),ValueError>;}
impl CellCheck for SemanticCheck<'_,'_>{fn step(&mut self)->Result<(),ValueError>{self.step()}}
impl CellCheck for semio_framework_value::NativeDecodeControl<'_>{fn step(&mut self)->Result<(),ValueError>{semio_framework_dsl_record::NativeSchemaControl::step(self)}}
struct Cells<'a,C>{bytes:usize,maximum:usize,control:&'a mut C}
impl<C:CellCheck> Cells<'_,C>{
 fn add(&mut self,count:usize)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(count).filter(|n|*n<=self.maximum).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Presentation complete SQL semantic value bytes exceeded"))?;self.control.step()}
 fn text(&mut self,v:&str)->Result<(),ValueError>{self.add(v.len())}
 fn float(&mut self,v:f64)->Result<(),ValueError>{self.add(if v.is_nan(){11}else if v.is_infinite(){32}else{22})}
 fn frame(&mut self,v:&FigureTileFrame)->Result<(),ValueError>{for f in [v.x,v.y,v.width,v.height]{self.float(f)?;}Ok(())}
 fn child<S>(&mut self,slot:&str,v:&store::ArtifactChild<S>)->Result<(),ValueError>{self.add(16)?;self.text(slot)?;for text in [&v.child_id,&v.target.artifact_id,&v.target.dialect.artifact_kind,&v.target.dialect.standard,&v.target.dialect.subset]{self.text(text)?;}Ok(())}
}
fn semantic_owned<C:CellCheck>(s:&PresentationSnapshot,maximum:usize,control:&mut C)->Result<(),ValueError>{let mut c=Cells{bytes:0,maximum,control};c.add(24)?;c.text(&s.schema)?;c.text(&s.source.src)?;c.text(&s.source.kind)?;c.frame(&s.source.frame)?;if let Some(v)=s.source.source_aspect{c.float(v)?;}if s.source.pdf_page.is_some(){c.add(8)?;}for t in &s.tiles{c.add(24)?;c.text(&t.id)?;c.text(&t.name)?;c.frame(&t.crop)?;}c.child("presentation",&s.presentation)?;c.child("animation",&s.animation)}
fn field(r:&semio_framework_dsl_record::RecordValue,id:u16)->Result<&semio_framework_dsl_record::FieldValue,ValueError>{r.get(id).ok_or_else(||invalid("Presentation semantic field missing"))}
fn record(v:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match v{semio_framework_dsl_record::FieldValue::Record(r)=>Ok(r),semio_framework_dsl_record::FieldValue::Block(v)=>record(v),_=>Err(invalid("Presentation semantic record differs"))}}
fn text(v:&semio_framework_dsl_record::FieldValue)->Result<&str,ValueError>{match v{semio_framework_dsl_record::FieldValue::Text(v)=>Ok(v),_=>Err(invalid("Presentation semantic text differs"))}}
fn floats<C:CellCheck>(c:&mut Cells<'_,C>,v:&semio_framework_dsl_record::RecordValue)->Result<(),ValueError>{for id in 0..4{let semio_framework_dsl_record::FieldValue::Float(v)=field(v,id)? else{return Err(invalid("Presentation semantic frame differs"))};c.float(*v)?;}Ok(())}
fn semantic_borrowed(r:&semio_framework_dsl_record::RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let mut c=Cells{bytes:0,maximum,control:native};c.add(24)?;c.text(text(field(r,0)?)?)?;let source=record(field(r,1)?)?;c.text(text(field(source,0)?)?)?;c.text(text(field(source,1)?)?)?;floats(&mut c,record(field(source,2)?)?)?;if let Some(v)=source.get(3).filter(|v|!matches!(v,semio_framework_dsl_record::FieldValue::Absent)){let semio_framework_dsl_record::FieldValue::Float(v)=v else{return Err(invalid("Presentation semantic aspect differs"))};c.float(*v)?;}if source.get(4).is_some_and(|v|!matches!(v,semio_framework_dsl_record::FieldValue::Absent)){c.add(8)?;}match r.get(2){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>{},Some(semio_framework_dsl_record::FieldValue::List(tiles))=>for t in tiles{let t=record(t)?;c.add(24)?;c.text(text(field(t,0)?)?)?;c.text(text(field(t,1)?)?)?;floats(&mut c,record(field(t,2)?)?)?;},_=>return Err(invalid("Presentation semantic tiles differ"))}for(id,slot)in[(3,"presentation"),(4,"animation")]{let child=record(field(r,id)?)?;c.add(16)?;c.text(slot)?;c.text(text(field(child,0)?)?)?;let target=record(field(child,1)?)?;for id in 0..4{c.text(text(field(target,id)?)?)?;}}Ok(())})}
fn sort_ids(ids:&mut[i64],control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;fn sift(ids:&mut[i64],mut at:usize,end:usize,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{loop{let left=at.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(||invalid("Presentation identity sort overflow"))?;if left>=end{return Ok(())}let mut child=left;if left+1<end{c.step()?;if ids[left]<ids[left+1]{child=left+1}}c.step()?;if ids[at]>=ids[child]{return Ok(())}ids.swap(at,child);at=child;}}for at in (0..ids.len()/2).rev(){sift(ids,at,ids.len(),control)?;}for end in (1..ids.len()).rev(){ids.swap(0,end);sift(ids,0,end,control)?;}Ok(())})}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

