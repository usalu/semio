//! 📐️ CAD layers, block ownership and nine independently typed entity geometries.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::cad::schema::snapshot::{SemioCadSnapshot,CadLayer,CadBlock,CadEntityRecord,CadEntity};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🪪️ Builds a paid name frontier in authored ordinal order with bounded comparisons.
fn cad_frontier<'a>(names:impl Iterator<Item=&'a str>,count:usize,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut ids=out.allocate_frontier(count)?;for(ordinal,name)in names.enumerate(){out.checkpoint()?;ids.push((name,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio CAD identity extent overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio CAD identity within its owner"))}}Ok(ids)
}
/// 🔍️ Resolves actual CAD layer and block identities with cancellable UTF8 comparisons.
fn cad_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio CAD layer or block"))
}
/// 🏷️ Admits owner-local handles before visiting any entity geometry.
fn cad_records(records:&[CadEntityRecord],block:Option<i64>,layers:&[(&str,i64)],blocks:&[(&str,i64)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let _handles=cad_frontier(records.iter().map(|record|record.handle.as_str()),records.len(),out)?;for(ordinal,record)in records.iter().enumerate(){project_record(record,block,ordinal,layers,blocks,out)?;}Ok(())
}
/// 🫳️ Visits every authored CAD cell using the same actual RowWriter for admission and projection.
pub(crate)fn visit_rows(snapshot:&SemioCadSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let layers=cad_frontier(snapshot.layers.iter().map(|layer|layer.name.as_str()),snapshot.layers.len(),out)?;let blocks=cad_frontier(snapshot.blocks.iter().map(|block|block.name.as_str()),snapshot.blocks.len(),out)?;
 out.insert_key("semio_cad_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,layer)in snapshot.layers.iter().enumerate(){out.insert("semio_cad_layer",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layer.name),Cell::Integer(i64::from(layer.color_index)),Cell::Text(&layer.line_type),Cell::Integer(i64::from(layer.visible))])?;}
 for(ordinal,block)in snapshot.blocks.iter().enumerate(){out.insert_float("semio_cad_block",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&block.name),Cell::Real(block.base_point.x),Cell::Real(block.base_point.y)],float_columns("semio_cad_block"))?;}
 for block in &snapshot.blocks{let owner=cad_reference(&blocks,&block.name,out)?;cad_records(&block.entities,Some(owner),&layers,&blocks,out)?;}cad_records(&snapshot.entities,None,&layers,&blocks,out)
}
/// 🧱️ Projects all nine detail variants through their actual exact numeric columns.
fn project_record(record:&CadEntityRecord,block:Option<i64>,ordinal:usize,layers:&[(&str,i64)],blocks:&[(&str,i64)],p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
let kind=match record.entity{CadEntity::Line{..}=>"line",CadEntity::Arc{..}=>"arc",CadEntity::Circle{..}=>"circle",CadEntity::Ellipse{..}=>"ellipse",CadEntity::Polyline{..}=>"polyline",CadEntity::Text{..}=>"text",CadEntity::Insert{..}=>"insert",CadEntity::Solid{..}=>"solid",CadEntity::Dimension{..}=>"dimension"};let layer_id=cad_reference(layers,&record.layer,p)?;let id=p.insert("semio_cad_entity",&[Cell::Integer(1),block.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(number(ordinal)?),Cell::Text(&record.handle),Cell::Integer(layer_id),Cell::Text(kind)])?;
match &record.entity{
CadEntity::Line{a,b}=>p.insert_key_float("semio_cad_line",id,&[Cell::Real(a.x),Cell::Real(a.y),Cell::Real(b.x),Cell::Real(b.y)],float_columns("semio_cad_line"))?,
CadEntity::Arc{center,radius,start_angle,end_angle}=>p.insert_key_float("semio_cad_arc",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius),Cell::Real(*start_angle),Cell::Real(*end_angle)],float_columns("semio_cad_arc"))?,
CadEntity::Circle{center,radius}=>p.insert_key_float("semio_cad_circle",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(*radius)],float_columns("semio_cad_circle"))?,
CadEntity::Ellipse{center,major_axis_end,ratio,start_param,end_param}=>p.insert_key_float("semio_cad_ellipse",id,&[Cell::Real(center.x),Cell::Real(center.y),Cell::Real(major_axis_end.x),Cell::Real(major_axis_end.y),Cell::Real(*ratio),Cell::Real(*start_param),Cell::Real(*end_param)],float_columns("semio_cad_ellipse"))?,
CadEntity::Polyline{vertices,closed}=>{p.insert_key_float("semio_cad_polyline",id,&[Cell::Integer(i64::from(*closed))],float_columns("semio_cad_polyline"))?;for(ordinal,vertex)in vertices.iter().enumerate(){p.insert_float("semio_cad_polyline_vertex",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Real(vertex.x),Cell::Real(vertex.y)],float_columns("semio_cad_polyline_vertex"))?;}},
CadEntity::Text{position,height,rotation,content}=>p.insert_key_float("semio_cad_text",id,&[Cell::Real(position.x),Cell::Real(position.y),Cell::Real(*height),Cell::Real(*rotation),Cell::Text(content)],float_columns("semio_cad_text"))?,
CadEntity::Insert{block_name,insertion_point,scale,rotation}=>{let target=cad_reference(blocks,block_name,p)?;p.insert_key_float("semio_cad_insert",id,&[Cell::Integer(target),Cell::Real(insertion_point.x),Cell::Real(insertion_point.y),Cell::Real(scale.x),Cell::Real(scale.y),Cell::Real(*rotation)],float_columns("semio_cad_insert"))?;},
CadEntity::Solid{p1,p2,p3,p4}=>p.insert_key_float("semio_cad_solid",id,&[Cell::Real(p1.x),Cell::Real(p1.y),Cell::Real(p2.x),Cell::Real(p2.y),Cell::Real(p3.x),Cell::Real(p3.y),Cell::Real(p4.x),Cell::Real(p4.y)],float_columns("semio_cad_solid"))?,
CadEntity::Dimension{def_point,text_position,measurement,text}=>p.insert_key_float("semio_cad_dimension",id,&[Cell::Real(def_point.x),Cell::Real(def_point.y),Cell::Real(text_position.x),Cell::Real(text_position.y),Cell::Real(*measurement),Cell::Text(text)],float_columns("semio_cad_dimension"))?}
Ok(())}/// 🎟️ Admits all typed CAD cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioCadSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD row identity or columns"))}else{Ok(())}}
fn boolean(value:i64)->Result<bool,ValueError>{match value{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD boolean"))}}
fn optional_integer(row:SqliteRow<'_>,index:usize)->Result<Option<i64>,ValueError>{if row.is_null(index)?{Ok(None)}else{row.integer(index).map(Some)}}
fn ordered<'a>(mut rows:Vec<SqliteRow<'a>>,column:usize)->Result<Vec<SqliteRow<'a>>,ValueError>{rows.sort_by_key(|row|row.integer(column).unwrap_or(-1));for(ordinal,row)in rows.iter().enumerate(){if row.integer(column)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio CAD ordinals must be contiguous"));}}Ok(rows)}
fn details<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,SqliteRow<'a>>,ValueError>{let mut result=BTreeMap::new();for(count,row)in float_rows(db,table,control)?.into_iter().enumerate(){identity(row,columns)?;if result.insert(row.rowid,row).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio CAD row identity"));}if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn point(row:SqliteRow<'_>,x:usize)->Result<SemioPoint2,ValueError>{Ok(SemioPoint2{x:row.real(x)?,y:row.real(x+1)?})}

impl ArtifactSqliteSnapshot for SemioCadSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::cad::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::cad::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="cad"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_cad_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioCadSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=single_float_row(database,"semio_cad_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD document identifier"));}let mut completed=0usize;
let mut layers=Vec::new();let mut layer_names=BTreeMap::new();let mut names=BTreeSet::new();for row in ordered_float_rows(database,"semio_cad_layer",2,control)?{identity(row,7)?;if row.integer(1)?!=1||layer_names.insert(row.rowid,row.text(3)?).is_some()||!names.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD layer ownership or identity"));}layers.push(CadLayer{name:reconstruct_text(control,row.text(3)?)?,color_index:i32::try_from(row.integer(4)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,line_type:reconstruct_text(control,row.text(5)?)?,visible:boolean(row.integer(6)?)?});}
let block_rows=ordered_float_rows(database,"semio_cad_block",2,control)?;let mut block_names=BTreeMap::new();let mut names=BTreeSet::new();for row in &block_rows{identity(row,6)?;if row.integer(1)?!=1||block_names.insert(row.rowid,row.text(3)?).is_some()||!names.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD block ownership or identity"));}}
let mut lines=details(database,"semio_cad_line",5,control)?;let mut arcs=details(database,"semio_cad_arc",6,control)?;let mut circles=details(database,"semio_cad_circle",4,control)?;let mut ellipses=details(database,"semio_cad_ellipse",8,control)?;let mut polylines=details(database,"semio_cad_polyline",2,control)?;let mut texts=details(database,"semio_cad_text",6,control)?;let mut inserts=details(database,"semio_cad_insert",7,control)?;let mut solids=details(database,"semio_cad_solid",9,control)?;let mut dimensions=details(database,"semio_cad_dimension",7,control)?;
let mut vertices=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut ids=BTreeSet::new();for row in float_rows(database,"semio_cad_polyline_vertex",control)?{identity(row,5)?;if !polylines.contains_key(&row.integer(1)?)||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD polyline vertex owner or identity"));}row.integer(2)?;vertices.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut entities=BTreeMap::<Option<i64>,Vec<SqliteRow<'_>>>::new();let mut ids=BTreeSet::new();for row in float_rows(database,"semio_cad_entity",control)?{identity(row,7)?;let owner=optional_integer(row,2)?;if row.integer(1)?!=1||owner.is_some_and(|id|!block_names.contains_key(&id))||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio CAD entity ownership or identity"));}row.integer(3)?;entities.entry(owner).or_default().push(row);}
let mut native_entities=BTreeMap::<Option<i64>,Vec<CadEntityRecord>>::new();for(owner,rows)in entities{let mut records=Vec::new();let mut handles=BTreeSet::new();for row in ordered(rows,3)?{if !handles.insert(row.text(4)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio CAD entity handle within owner"));}let entity=match row.text(6)?{
"line"=>{let r=lines.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD line fields"))?;CadEntity::Line{a:point(r,1)?,b:point(r,3)?}},
"arc"=>{let r=arcs.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD arc fields"))?;CadEntity::Arc{center:point(r,1)?,radius:r.real(3)?,start_angle:r.real(4)?,end_angle:r.real(5)?}},
"circle"=>{let r=circles.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD circle fields"))?;CadEntity::Circle{center:point(r,1)?,radius:r.real(3)?}},
"ellipse"=>{let r=ellipses.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD ellipse fields"))?;CadEntity::Ellipse{center:point(r,1)?,major_axis_end:point(r,3)?,ratio:r.real(5)?,start_param:r.real(6)?,end_param:r.real(7)?}},
"polyline"=>{let r=polylines.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD polyline fields"))?;let mut native_vertices=Vec::new();for vertex in ordered(vertices.remove(&row.rowid).unwrap_or_default(),2)?{native_vertices.push(point(vertex,3)?);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}CadEntity::Polyline{vertices:native_vertices,closed:boolean(r.integer(1)?)?}},
"text"=>{let r=texts.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD text fields"))?;CadEntity::Text{position:point(r,1)?,height:r.real(3)?,rotation:r.real(4)?,content:reconstruct_text(control,r.text(5)?)?}},
"insert"=>{let r=inserts.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD insert fields"))?;CadEntity::Insert{block_name:reconstruct_text(control,block_names.get(&r.integer(1)?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling CAD inserted block"))?)?,insertion_point:point(r,2)?,scale:point(r,4)?,rotation:r.real(6)?}},
"solid"=>{let r=solids.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD solid fields"))?;CadEntity::Solid{p1:point(r,1)?,p2:point(r,3)?,p3:point(r,5)?,p4:point(r,7)?}},
"dimension"=>{let r=dimensions.remove(&row.rowid).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing CAD dimension fields"))?;CadEntity::Dimension{def_point:point(r,1)?,text_position:point(r,3)?,measurement:r.real(5)?,text:reconstruct_text(control,r.text(6)?)?}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio CAD entity kind"))};records.push(CadEntityRecord{handle:reconstruct_text(control,row.text(4)?)?,layer:reconstruct_text(control,layer_names.get(&row.integer(5)?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling CAD entity layer"))?)?,entity});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}native_entities.insert(owner,records);}
if !lines.is_empty()||!arcs.is_empty()||!circles.is_empty()||!ellipses.is_empty()||!polylines.is_empty()||!texts.is_empty()||!inserts.is_empty()||!solids.is_empty()||!dimensions.is_empty()||!vertices.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan or contradictory Semio CAD variant detail"));}let entities=native_entities.remove(&None).unwrap_or_default();let mut blocks=Vec::new();for row in block_rows{blocks.push(CadBlock{name:reconstruct_text(control,row.text(3)?)?,base_point:point(row,4)?,entities:native_entities.remove(&Some(row.rowid)).unwrap_or_default()});}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,layers,blocks,entities})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_cad_block"=>&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_cad_line"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_arc"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],"semio_cad_circle"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3)],"semio_cad_ellipse"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)],"semio_cad_polyline_vertex"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_text"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_cad_insert"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)],"semio_cad_solid"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)],"semio_cad_dimension"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)],_=>&[]}}
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioCadSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.layers.len())?;for layer in &self.layers{b.text(&layer.name)?;b.text(&layer.line_type)?;b.scalars(2)?;}b.entities(self.blocks.len())?;for block in &self.blocks{b.text(&block.name)?;b.scalars(2)?;native_entities(&block.entities,b)?;}native_entities(&self.entities,b)?;Ok(())}
}

fn native_entities(records:&[CadEntityRecord],b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(records.len())?;for record in records{b.text(&record.handle)?;b.text(&record.layer)?;match &record.entity{CadEntity::Line{..}=>b.scalars(4)?,CadEntity::Arc{..}=>b.scalars(5)?,CadEntity::Circle{..}=>b.scalars(3)?,CadEntity::Ellipse{..}=>b.scalars(7)?,CadEntity::Polyline{vertices,..}=>{b.entities(vertices.len())?;b.scalars(vertices.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CAD native point count overflow"))?)?;b.scalars(1)?;},CadEntity::Text{content,..}=>{b.text(content)?;b.scalars(4)?;},CadEntity::Insert{block_name,..}=>{b.text(block_name)?;b.scalars(5)?;},CadEntity::Solid{..}=>b.scalars(8)?,CadEntity::Dimension{text,..}=>{b.text(text)?;b.scalars(5)?;}}}Ok(())}

impl SemioCadSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
