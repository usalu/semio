//! 🌊️ Named flow nodes, ordered parameters and port-addressed edge entities.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::flow::schema::snapshot::{SemioFlowSnapshot,FlowNode,FlowEdge,FlowParam,PortRef};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits the exact Flow entities through the caller's owned or borrowed row writer.
fn visit_rows(snapshot:&SemioFlowSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let phase=out.phase();let mut nodes=out.allocate_frontier(snapshot.nodes.len())?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){out.checkpoint()?;nodes.push((node.id.as_str(),number(ordinal+1)?));}
 out.sort_frontier(&mut nodes,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,c))?;
 for pair in nodes.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow node identifier"))}}
 let mut ids=out.allocate_frontier(snapshot.edges.len())?;for edge in &snapshot.edges{out.checkpoint()?;ids.push(edge.id.as_str());}
 out.sort_frontier(&mut ids,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,c))?;
 for pair in ids.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow edge identifier"))}}
 out.insert_key("semio_flow_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.nodes.iter().enumerate(){
  let id=out.insert_float("semio_flow_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)],float_columns("semio_flow_node"))?;
  for(ordinal,param)in node.params.iter().enumerate(){out.insert("semio_flow_parameter",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&param.key),Cell::Text(&param.value)])?;}
 }
 for(ordinal,edge)in snapshot.edges.iter().enumerate(){
  let source=node_reference(&nodes,&edge.from.node,out)?;let target=node_reference(&nodes,&edge.to.node,out)?;
  out.insert("semio_flow_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Integer(source),Cell::Text(&edge.from.port),Cell::Integer(target),Cell::Text(&edge.to.port),Cell::Text(&edge.kind)])?;
 }Ok(())
}
/// 🔎️ Resolves the actual sorted borrowed node frontier with bounded literal comparisons.
fn node_reference(nodes:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=nodes.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(nodes[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(nodes[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow edge node"))
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioFlowSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioFlowSnapshot{
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="flow"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_flow_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioFlowSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=single_float_row(database,"semio_flow_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow document identifier"));}
let ordered=ordered_float_rows(database,"semio_flow_node",2,control)?;let mut names=BTreeMap::new();let mut native_ids=BTreeSet::new();for row in &ordered{identity(row,8)?;if row.integer(1)?!=1||!native_ids.insert(row.text(3)?)||names.insert(row.rowid,row.text(3)?).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow node ownership or identity"));}}
let mut parameters=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut ids=BTreeSet::new();let mut completed=0usize;for row in float_rows(database,"semio_flow_parameter",control)?{identity(row,5)?;if !names.contains_key(&row.integer(1)?)||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow parameter owner or identity"));}row.integer(2)?;parameters.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut nodes=Vec::new();for row in ordered{let mut ordered_params=parameters.remove(&row.rowid).unwrap_or_default();ordered_params.sort_by_key(|row|row.integer(2).unwrap_or(-1));let mut params=Vec::new();for(ordinal,param)in ordered_params.into_iter().enumerate(){if param.integer(2)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow parameter ordinals must be contiguous"));}params.push(FlowParam{key:reconstruct_text(control,param.text(3)?)?,value:reconstruct_text(control,param.text(4)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}nodes.push(FlowNode{id:reconstruct_text(control,row.text(3)?)?,kind:reconstruct_text(control,row.text(4)?)?,label:reconstruct_text(control,row.text(5)?)?,position:SemioPoint2{x:row.real(6)?,y:row.real(7)?},params});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut edges=Vec::new();let mut ids=BTreeSet::new();let mut native_ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_flow_edge",2,control)?{identity(row,9)?;if row.integer(1)?!=1||!ids.insert(row.rowid)||!native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow edge ownership or identity"));}edges.push(FlowEdge{id:reconstruct_text(control,row.text(3)?)?,from:PortRef{node:reconstruct_text(control,names.get(&row.integer(4)?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow source node"))?)?,port:reconstruct_text(control,row.text(5)?)?},to:PortRef{node:reconstruct_text(control,names.get(&row.integer(6)?).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow target node"))?)?,port:reconstruct_text(control,row.text(7)?)?},kind:reconstruct_text(control,row.text(8)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,nodes,edges})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_flow_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioFlowSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(2)?;b.entities(node.params.len())?;for param in &node.params{b.text(&param.key)?;b.text(&param.value)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id)?;b.text(&edge.from.node)?;b.text(&edge.from.port)?;b.text(&edge.to.node)?;b.text(&edge.to.port)?;b.text(&edge.kind)?;}Ok(())}
}

impl SemioFlowSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
