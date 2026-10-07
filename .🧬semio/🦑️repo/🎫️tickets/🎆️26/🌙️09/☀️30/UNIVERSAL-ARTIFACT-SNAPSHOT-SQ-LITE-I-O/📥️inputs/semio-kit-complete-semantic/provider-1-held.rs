//! 🧰️ Kit types, designs, placed pieces, connections, typed child handles and history pins.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot,SemioKitType,SemioKitDesign,SemioKitPiece,SemioKitConnection};
use crate::standards::v1::subsets::base::schema::geometry::{SemioTransform};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::base::schema::{geometry::{SemioPoint3,SemioQuaternion},child::validate_semio_child_identity};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🔍️ Resolves a retained, paid identity frontier with bounded text comparisons.
fn identifier(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Kit relational identity"))
}
/// 🪪️ Admits literal identities using paid borrowed entries and cancellable comparisons.
fn frontier<'a,T>(values:&'a[T],start:usize,id:impl Fn(&'a T)->&'a str,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(values.len())?;for(ordinal,value)in values.iter().enumerate(){out.checkpoint()?;let index=start.checked_add(ordinal).and_then(|value|value.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit relational identity overflow"))?;ids.push((id(value),number(index)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Kit literal identity"))}}Ok(ids)
}
/// 🧾️ Projects the literal four-text target identity without an opaque carrier.
fn reference(target:&store::os_io::ArtifactRef,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 out.insert("semio_kit_reference",&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])
}
/// 🧒️ Projects ordered child aliases independently from their persisted target identities.
fn project_children<S>(children:&[store::ArtifactChild<S>],table:&str,subset:&str,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _ids=frontier(children,0,|child|child.child_id.as_str(),out)?;
 for(ordinal,child)in children.iter().enumerate(){let target=reference(&child.target,out)?;out.insert(table,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}Ok(())
}
/// 🔢️ Borrows the canonical unsigned decimal size from fixed stack storage.
fn decimal(mut value:u64,scratch:&mut[u8;20])->&str{
 let mut at=scratch.len();loop{at-=1;scratch[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}std::str::from_utf8(&scratch[at..]).expect("ASCII unsigned decimal")
}
/// 🫳️ Visits the actual Kit catalog, placements, child references and history pins through one row writer.
pub(crate)fn visit_rows(snapshot:&SemioKitSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let types=frontier(&snapshot.types,0,|kind|kind.id.as_str(),out)?;
 let _designs=frontier(&snapshot.designs,0,|design|design.id.as_str(),out)?;
 out.insert_key("semio_kit_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,kind)in snapshot.types.iter().enumerate(){out.insert("semio_kit_type",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&kind.id),Cell::Text(&kind.name),Cell::Text(&kind.category)])?;}
 let mut piece_start=0usize;
 for(ordinal,design)in snapshot.designs.iter().enumerate(){
  let design_id=out.insert("semio_kit_design",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&design.id),Cell::Text(&design.name)])?;
  let pieces=frontier(&design.pieces,piece_start,|piece|piece.id.as_str(),out)?;
  piece_start=piece_start.checked_add(design.pieces.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Kit piece extent overflow"))?;
  let _connections=frontier(&design.connections,0,|connection|connection.id.as_str(),out)?;
  for(ordinal,piece)in design.pieces.iter().enumerate(){
   let t=piece.transform;let kind=identifier(&types,&piece.type_id,out)?;
   out.insert_float("semio_kit_piece",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&piece.id),Cell::Integer(kind),Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_kit_piece"))?;
  }
  for(ordinal,connection)in design.connections.iter().enumerate(){
   let source=identifier(&pieces,&connection.connecting_piece_id,out)?;let target=identifier(&pieces,&connection.connected_piece_id,out)?;
   out.insert("semio_kit_connection",&[Cell::Integer(design_id),Cell::Integer(number(ordinal)?),Cell::Text(&connection.id),Cell::Integer(source),Cell::Text(&connection.connecting_port),Cell::Integer(target),Cell::Text(&connection.connected_port)])?;
  }
 }
 project_children(&snapshot.objects,"semio_kit_object_child","object",out)?;
 project_children(&snapshot.models,"semio_kit_model_child","model",out)?;
 if let Some(child)=&snapshot.properties{let target=reference(&child.target,out)?;out.insert("semio_kit_value_child",&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Integer(target)])?;validate_semio_child_identity(&child.child_id,&child.target,"value").map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;}
 for(ordinal,link)in snapshot.representations.iter().enumerate(){
  let target=reference(&link.target,out)?;let kind=identifier(&types,&link.role,out)?;
  let(pin,checkpoint,blob)=match &link.pin{
   store::LinkPin::Head=>("head",Cell::Null,Cell::Null),
   store::LinkPin::Checkpoint{id}=>("checkpoint",Cell::Text(id),Cell::Null),
   store::LinkPin::Snapshot{blob}=>{let mut scratch=[0u8;20];let size=decimal(blob.size,&mut scratch);let id=out.insert("semio_kit_blob",&[Cell::Text(&blob.hash),Cell::Text(size),Cell::Text(&blob.media_type)])?;("snapshot",Cell::Null,Cell::Integer(id))}
  };
  out.insert("semio_kit_representation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Integer(kind),Cell::Integer(target),Cell::Text(pin),checkpoint,blob])?;
 }Ok(())
}
/// 🎟️ Admits all typed Kit cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioKitSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit row identity or columns"))}else{Ok(())}}
fn ordered<'a>(mut rows:Vec<SqliteRow<'a>>)->Result<Vec<SqliteRow<'a>>,ValueError>{rows.sort_by_key(|row|row.integer(2).unwrap_or(-1));for(ordinal,row)in rows.iter().enumerate(){if row.integer(2)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio kit ordinals must be contiguous"));}}Ok(rows)}
fn children<S>(database:&SqliteDatabase,table:&str,subset:&str,references:&mut BTreeMap<i64,store::os_io::ArtifactRef>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<store::ArtifactChild<S>>,ValueError>{let mut result=Vec::new();let mut ids=BTreeSet::new();let mut child_ids=BTreeSet::new();for(count,row)in ordered_float_rows(database,table,2,control)?.into_iter().enumerate(){identity(row,5)?;if row.integer(1)?!=1||!ids.insert(row.rowid)||!child_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit child ownership or identity"));}if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}let target=references.remove(&row.integer(4)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling or multiply owned Semio kit child reference"))?;validate_semio_child_identity(row.text(3)?,&target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;result.push(store::ArtifactChild::new(reconstruct_text(control,row.text(3)?)?,target));}Ok(result)}
impl ArtifactSqliteSnapshot for SemioKitSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::kit::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if !SemioKitSnapshot::admits_dialect_parts(&dialect.artifact_kind,&dialect.standard,&dialect.subset){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_kit_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioKitSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=single_float_row(database,"semio_kit_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit document identifier"));}
let mut types=Vec::new();let mut type_names=BTreeMap::new();let mut native_ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_kit_type",2,control)?{identity(row,6)?;if row.integer(1)?!=1||type_names.insert(row.rowid,row.text(3)?).is_some()||!native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit type ownership or identity"));}types.push(SemioKitType{id:reconstruct_text(control,row.text(3)?)?,name:reconstruct_text(control,row.text(4)?)?,category:reconstruct_text(control,row.text(5)?)?});}
let design_rows=ordered_float_rows(database,"semio_kit_design",2,control)?;let mut design_ids=BTreeSet::new();let mut native_ids=BTreeSet::new();for row in &design_rows{identity(row,5)?;if row.integer(1)?!=1||!design_ids.insert(row.rowid)||!native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit design ownership or identity"));}}
let mut pieces=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut piece_names=BTreeMap::new();let mut ids=BTreeSet::new();let mut completed=0usize;for row in float_rows(database,"semio_kit_piece",control)?{identity(row,15)?;if !design_ids.contains(&row.integer(1)?)||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit piece ownership or identity"));}row.integer(2)?;piece_names.insert(row.rowid,(row.integer(1)?,row.text(3)?));pieces.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut connections=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut ids=BTreeSet::new();for row in float_rows(database,"semio_kit_connection",control)?{identity(row,8)?;if !design_ids.contains(&row.integer(1)?)||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit connection ownership or identity"));}row.integer(2)?;connections.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut designs=Vec::new();for design in design_rows{let mut native_pieces=Vec::new();let mut native_ids=BTreeSet::new();for row in ordered(pieces.remove(&design.rowid).unwrap_or_default())?{if !native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio kit piece native identifier"));}let transform=SemioTransform{translation:SemioPoint3{x:row.real(5)?,y:row.real(6)?,z:row.real(7)?},rotation:SemioQuaternion{x:row.real(8)?,y:row.real(9)?,z:row.real(10)?,w:row.real(11)?},scale:SemioPoint3{x:row.real(12)?,y:row.real(13)?,z:row.real(14)?}};native_pieces.push(SemioKitPiece{id:reconstruct_text(control,row.text(3)?)?,type_id:reconstruct_text(control,type_names.get(&row.integer(4)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio kit piece type"))?)?,transform});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut native_connections=Vec::new();let mut native_ids=BTreeSet::new();for row in ordered(connections.remove(&design.rowid).unwrap_or_default())?{if !native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio kit connection native identifier"));}let connecting=piece_names.get(&row.integer(4)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio kit connecting piece"))?;let connected=piece_names.get(&row.integer(6)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio kit connected piece"))?;if connecting.0!=design.rowid||connected.0!=design.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio kit connection crosses its owning design"));}native_connections.push(SemioKitConnection{id:reconstruct_text(control,row.text(3)?)?,connecting_piece_id:reconstruct_text(control,connecting.1)?,connecting_port:reconstruct_text(control,row.text(5)?)?,connected_piece_id:reconstruct_text(control,connected.1)?,connected_port:reconstruct_text(control,row.text(7)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}designs.push(SemioKitDesign{id:reconstruct_text(control,design.text(3)?)?,name:reconstruct_text(control,design.text(4)?)?,pieces:native_pieces,connections:native_connections});}
let mut references=BTreeMap::new();for row in float_rows(database,"semio_kit_reference",control)?{identity(row,5)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;let target=store::os_io::ArtifactRef{artifact_id:reconstruct_text(control,row.text(1)?)?,dialect:store::os_io::ArtifactDialect{artifact_kind:reconstruct_text(control,row.text(2)?)?,standard:reconstruct_text(control,row.text(3)?)?,subset:reconstruct_text(control,row.text(4)?)?}};if references.insert(row.rowid,target).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid or duplicate Semio kit reference"));}completed+=1;}
let objects=children::<SemioObjectSnapshot>(database,"semio_kit_object_child","object",&mut references,control)?;let models=children::<SemioModelSnapshot>(database,"semio_kit_model_child","model",&mut references,control)?;let table=database.table("semio_kit_value_child")?;let properties=if table.rows.is_empty(){None}else{let row=SqliteRow::new(table.single_row()?,float_columns("semio_kit_value_child"))?;identity(row,4)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit value child owner"));}let target=references.remove(&row.integer(3)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio kit value child reference"))?;validate_semio_child_identity(row.text(2)?,&target,"value").map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;Some(store::ArtifactChild::<SemioValueSnapshot>::new(reconstruct_text(control,row.text(2)?)?,target))};
let mut blobs=BTreeMap::new();for row in float_rows(database,"semio_kit_blob",control)?{identity(row,4)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;let size=row.text(2)?.parse::<u64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;if size.to_string()!=row.text(2)?||blobs.insert(row.rowid,store::BlobRef{hash:reconstruct_text(control,row.text(1)?)?,size,media_type:reconstruct_text(control,row.text(3)?)?}).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit blob identity or size"));}completed+=1;}
let mut representations=Vec::new();let mut ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_kit_representation",2,control)?{identity(row,8)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit representation ownership or identity"));}let checkpoint=row.optional_text(6)?;let blob=if row.is_null(7)?{None}else{Some(row.integer(7)?)};let pin=match row.text(5)?{"head" if checkpoint.is_none()&&blob.is_none()=>store::LinkPin::Head,"checkpoint" if blob.is_none()=>store::LinkPin::Checkpoint{id:reconstruct_text(control,checkpoint.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio kit checkpoint pin"))?)?},"snapshot" if checkpoint.is_none()=>store::LinkPin::Snapshot{blob:blobs.remove(&blob.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio kit snapshot pin"))?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling or multiply owned Semio kit blob pin"))?},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio kit link pin shape"))};representations.push(store::ArtifactLink{target:references.remove(&row.integer(4)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling or multiply owned Semio kit representation reference"))?,role:reconstruct_text(control,type_names.get(&row.integer(3)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio kit representation type role"))?)?,pin});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
if !references.is_empty()||!blobs.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan Semio kit reference or blob pin descriptor"));}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,types,designs,objects,models,properties,representations})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_kit_piece"=>&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14)],_=>&[]}}
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioKitSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.types.len())?;for ty in &self.types{b.text(&ty.id)?;b.text(&ty.name)?;b.text(&ty.category)?;}b.entities(self.designs.len())?;for design in &self.designs{b.text(&design.id)?;b.text(&design.name)?;b.entities(design.pieces.len())?;for piece in &design.pieces{b.text(&piece.id)?;b.text(&piece.type_id)?;b.scalars(10)?;}b.entities(design.connections.len())?;for connection in &design.connections{b.text(&connection.id)?;b.text(&connection.connecting_piece_id)?;b.text(&connection.connecting_port)?;b.text(&connection.connected_piece_id)?;b.text(&connection.connected_port)?;}}for child in &self.objects{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}for child in &self.models{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}if let Some(child)=&self.properties{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_child(child,b)?;}b.entities(self.representations.len())?;for link in &self.representations{b.text(&link.role)?;crate::standards::v1::subsets::object::io::sqlite::snapshot::native_reference(&link.target,b)?;match &link.pin{store::LinkPin::Head=>{},store::LinkPin::Checkpoint{id}=>b.text(id)?,store::LinkPin::Snapshot{blob}=>{b.text(&blob.hash)?;b.text(&blob.media_type)?;b.scalars(1)?;}}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
