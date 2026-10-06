//! 🕸️ Named graph nodes, ports, edges and explicitly typed property values.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned;
use crate::graph::schema::snapshot::{SemioGraphSnapshot,SemioGraphNode,SemioGraphEdge,SemioGraphPort,SemioGraphPortKind,GraphNodeId,GraphEdgeId};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::value::schema::snapshot::SemioValueEntry;
use crate::standards::v1::subsets::value::io::sqlite::snapshot::{project_value_tree,reconstruct_value_forest,ValueSqliteTables};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_os_kernel::sqlite_snapshot::transfer::{reserve,heap_sort,compare_text};
const VALUES:ValueSqliteTables=ValueSqliteTables{value:"semio_graph_value",list_element:"semio_graph_list_element",map_entry:"semio_graph_map_entry"};
fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio graph row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioGraphSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::schema::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::graph::schema::snapshot::native_encoding::encode(self,encoding,control)}

 fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::graph::schema::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="graph"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_graph_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioGraphSnapshot {
    /// 🧩️ Restores every typed child field through paid relational reference frontiers.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
     control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
     let document=single_float_row(database,"semio_graph_document")?;identity(document,2)?;if document.rowid!=1{return Err(invalid("invalid Semio graph document identifier"))}
     let nodes=ordered_float_rows(database,"semio_graph_node",2,control)?;let edges=ordered_float_rows(database,"semio_graph_edge",2,control)?;
     let mut names=reserve(nodes.len(),control)?;let mut native_ids=reserve(nodes.len(),control)?;
     for row in &nodes{identity(*row,10)?;if row.integer(1)?!=1{return Err(invalid("invalid Semio graph node owner"))}names.push((row.rowid,row.text(3)?));native_ids.push(row.text(3)?);}
     heap_sort(&mut names,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.0.cmp(&b.0)))?;if names.windows(2).any(|pair|pair[0].0==pair[1].0){return Err(invalid("duplicate Semio graph node row identity"))}heap_sort(&mut native_ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,c|compare_text(a,b,SqliteSnapshotPhase::ReconstructSnapshot,c))?;if native_ids.windows(2).any(|pair|pair[0]==pair[1]){return Err(invalid("duplicate Semio graph node identity"))}
     let mut ports=float_rows(database,"semio_graph_port",control)?;let mut port_ids=reserve(ports.len(),control)?;
     for row in &ports{identity(*row,6)?;node_name(&names,row.integer(1)?)?;port_ids.push(row.rowid);}heap_sort(&mut port_ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;if port_ids.windows(2).any(|pair|pair[0]==pair[1]){return Err(invalid("duplicate Semio graph port identity"))}order_relationships(&mut ports,control)?;
     let mut edge_ids=reserve(edges.len(),control)?;let mut edge_native_ids=reserve(edges.len(),control)?;
     for row in &edges{identity(*row,10)?;if row.integer(1)?!=1{return Err(invalid("invalid Semio graph edge owner"))}node_name(&names,row.integer(4)?)?;node_name(&names,row.integer(5)?)?;edge_ids.push(row.rowid);edge_native_ids.push(row.text(3)?);}
     heap_sort(&mut edge_ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.cmp(b)))?;if edge_ids.windows(2).any(|pair|pair[0]==pair[1]){return Err(invalid("duplicate Semio graph edge row identity"))}heap_sort(&mut edge_native_ids,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,c|compare_text(a,b,SqliteSnapshotPhase::ReconstructSnapshot,c))?;if edge_native_ids.windows(2).any(|pair|pair[0]==pair[1]){return Err(invalid("duplicate Semio graph edge identity"))}
     let mut node_properties=float_rows(database,"semio_graph_property",control)?;let mut port_properties=float_rows(database,"semio_graph_port_property",control)?;let mut edge_properties=float_rows(database,"semio_graph_edge_property",control)?;
     let count=node_properties.len().checked_add(port_properties.len()).and_then(|count|count.checked_add(edge_properties.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio graph property count overflow"))?;
     let mut roots=reserve(count,control)?;let mut root_indices=reserve(count,control)?;
     for(family,rows)in [&mut node_properties,&mut port_properties,&mut edge_properties].into_iter().enumerate(){
      for row in rows.iter(){identity(*row,5)?;let parent=row.integer(1)?;let admitted=match family{0=>names.binary_search_by_key(&parent,|row|row.0).is_ok(),1=>port_ids.binary_search(&parent).is_ok(),_=>edge_ids.binary_search(&parent).is_ok()};if !admitted{return Err(invalid("dangling Semio graph property owner"))}let root=row.integer(4)?;root_indices.push((root,roots.len()));roots.push(root);}
      order_relationships(rows,control)?;
     }
     heap_sort(&mut root_indices,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.0.cmp(&b.0)))?;if root_indices.windows(2).any(|pair|pair[0].0==pair[1].0){return Err(invalid("multiply owned Semio graph property value"))}
     let mut decoded=Owned::new(reconstruct_value_forest(database,VALUES,&roots,None,control)?);let mut values=Owned::new(reserve(decoded.get_mut().len(),control)?);for value in decoded.get_mut().drain(..){values.get_mut().push(Some(value));}
     let mut restored_nodes=Owned::new(reserve(nodes.len(),control)?);
     for(row_ordinal,row)in nodes.into_iter().enumerate(){
      let mut native_ports=Owned::new(reserve(group(&ports,row.rowid).len(),control)?);
      for port in group(&ports,row.rowid){let kind=match port.text(4)?{"in"=>SemioGraphPortKind::In,"out"=>SemioGraphPortKind::Out,"in_out"=>SemioGraphPortKind::InOut,_=>return Err(invalid("unknown Semio graph port kind"))};let name=reconstruct_text(control,port.text(3)?)?;let category=reconstruct_text(control,port.text(5)?)?;let properties=restore_properties(&port_properties,port.rowid,&root_indices,values.get_mut(),control)?;native_ports.get_mut().push(SemioGraphPort{name,kind,category,properties});control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,row_ordinal,native_ports.get_mut().len())?;}
      let id=GraphNodeId::new(reconstruct_text(control,row.text(3)?)?);let kind=reconstruct_text(control,row.text(4)?)?;let label=reconstruct_text(control,row.text(5)?)?;let position=SemioPoint2{x:row.real(6)?,y:row.real(7)?};let width=row.real(8)?;let height=row.real(9)?;
      let properties=restore_properties(&node_properties,row.rowid,&root_indices,values.get_mut(),control)?;
      restored_nodes.get_mut().push(SemioGraphNode{id,kind,label,position,width,height,ports:native_ports.take(),properties});control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,row_ordinal+1,0)?;
     }
     let mut restored_edges=Owned::new(reserve(edges.len(),control)?);
     for(ordinal,row)in edges.into_iter().enumerate(){
      let id=GraphEdgeId::new(reconstruct_text(control,row.text(3)?)?);let source=GraphNodeId::new(reconstruct_text(control,node_name(&names,row.integer(4)?)?)?);let target=GraphNodeId::new(reconstruct_text(control,node_name(&names,row.integer(5)?)?)?);
      let kind=reconstruct_text(control,row.text(6)?)?;let label=reconstruct_text(control,row.text(7)?)?;let source_port=optional_text(row,8,control)?;let target_port=optional_text(row,9,control)?;let properties=restore_properties(&edge_properties,row.rowid,&root_indices,values.get_mut(),control)?;
      restored_edges.get_mut().push(SemioGraphEdge{id,source,target,kind,label,source_port,target_port,properties});control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal+1,0)?;
     }
     if values.get_mut().iter().any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"unconsumed Semio graph property value"))}
     let schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),roots.len())?;
     Ok(Self{schema,nodes:restored_nodes.take(),edges:restored_edges.take()})
    }
}
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn node_name<'a>(names:&[(i64,&'a str)],id:i64)->Result<&'a str,ValueError>{names.binary_search_by_key(&id,|row|row.0).map(|index|names[index].1).map_err(|_|invalid("dangling Semio graph node"))}
fn optional_text(row:SqliteRow<'_>,index:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Option<String>,ValueError>{if row.is_null(index)?{Ok(None)}else{Ok(Some(reconstruct_text(control,row.text(index)?)?))}}
fn group<'r,'a>(rows:&'r[SqliteRow<'a>],parent:i64)->&'r[SqliteRow<'a>]{let start=rows.partition_point(|row|row.integer(1).unwrap_or(-1)<parent);let end=rows.partition_point(|row|row.integer(1).unwrap_or(-1)<=parent);&rows[start..end]}
fn order_relationships(rows:&mut[SqliteRow<'_>],control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 for row in rows.iter(){row.integer(1)?;row.integer(2)?;}heap_sort(rows,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok((a.integer(1)?,a.integer(2)?).cmp(&(b.integer(1)?,b.integer(2)?))))?;
 let mut parent=None;let mut ordinal=0;for(index,row)in rows.iter().enumerate(){let owner=row.integer(1)?;if parent!=Some(owner){parent=Some(owner);ordinal=0;}if row.integer(2)?!=number(ordinal)?{return Err(invalid("Semio graph relationship ordinals must be contiguous"))}ordinal+=1;if index%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}}Ok(())
}
fn restore_properties(rows:&[SqliteRow<'_>],owner:i64,indices:&[(i64,usize)],values:&mut[Option<crate::standards::v1::subsets::value::schema::snapshot::SemioValue>],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SemioValueEntry>,ValueError>{
 let rows=group(rows,owner);let mut properties=Owned::new(reserve(rows.len(),control)?);
 for(ordinal,row)in rows.iter().enumerate(){let key=reconstruct_text(control,row.text(3)?)?;let id=row.integer(4)?;let index=indices.binary_search_by_key(&id,|row|row.0).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"missing admitted Semio graph property root"))?;let value=values[indices[index].1].take().ok_or_else(||invalid("multiply owned Semio graph property"))?;properties.get_mut().push(SemioValueEntry{key,value});if ordinal%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,ordinal,rows.len())?;}}
 Ok(properties.take())
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_graph_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)],_=>&[]}}
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=reserve(table_rows.rows.len(),control)?;for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}heap_sort(&mut result,SqliteSnapshotPhase::ReconstructSnapshot,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;if result.windows(2).any(|pair|pair[0].rowid==pair[1].rowid){return Err(invalid("duplicate Semio graph relationship identity"))}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let references=semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?;let mut result=reserve(references.len(),control)?;for(count,row)in references.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioGraphSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id.value)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(4)?;b.entities(node.ports.len())?;for port in &node.ports{b.text(&port.name)?;b.scalars(1)?;b.text(&port.category)?;b.entities(port.properties.len())?;for property in &port.properties{b.text(&property.key)?;crate::standards::v1::subsets::value::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}b.entities(node.properties.len())?;for property in &node.properties{b.text(&property.key)?;crate::standards::v1::subsets::value::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id.value)?;b.text(&edge.source.value)?;b.text(&edge.target.value)?;b.text(&edge.kind)?;b.text(&edge.label)?;b.scalars(2)?;if let Some(value)=&edge.source_port{b.text(value)?;}if let Some(value)=&edge.target_port{b.text(value)?;}b.entities(edge.properties.len())?;for property in &edge.properties{b.text(&property.key)?;crate::standards::v1::subsets::value::io::sqlite::snapshot::native_values(std::slice::from_ref(&property.value),b)?;}}Ok(())}
}

impl SemioGraphSnapshot{
/// 🧮️ Projects full node, port and edge state with paid literal reference indexes.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 control.check_rows(self.nodes.len().checked_add(self.edges.len()).and_then(|count|count.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio graph row count overflow"))?)?;
 let mut projection=RowWriter::new(Self::SQLITE_SCHEMA,control)?;
 let mut nodes=projection.allocate_frontier(self.nodes.len())?;
 for(ordinal,node)in self.nodes.iter().enumerate(){projection.checkpoint()?;nodes.push((node.id.value.as_str(),number(ordinal+1)?));}
 projection.sort_frontier(&mut nodes,|a,b,c|compare_text(a.0,b.0,SqliteSnapshotPhase::ProjectSnapshot,c))?;projection.checkpoint()?;
 if nodes.windows(2).any(|pair|pair[0].0==pair[1].0){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio graph node identifier"));}
 let mut edges=projection.allocate_frontier(self.edges.len())?;for edge in &self.edges{projection.checkpoint()?;edges.push(edge.id.value.as_str());}projection.sort_frontier(&mut edges,|a,b,c|compare_text(a,b,SqliteSnapshotPhase::ProjectSnapshot,c))?;projection.checkpoint()?;
 if edges.windows(2).any(|pair|pair[0]==pair[1]){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio graph edge identifier"));}
 projection.insert_key("semio_graph_document",1,&[Cell::Text(&self.schema)])?;
 for(ordinal,node)in self.nodes.iter().enumerate(){
  let id=projection.insert_float("semio_graph_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id.value),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y),Cell::Real(node.width),Cell::Real(node.height)],float_columns("semio_graph_node"))?;
  for(ordinal,port)in node.ports.iter().enumerate(){let port_id=projection.insert("semio_graph_port",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&port.name),Cell::Text(match port.kind{SemioGraphPortKind::In=>"in",SemioGraphPortKind::Out=>"out",SemioGraphPortKind::InOut=>"in_out"}),Cell::Text(&port.category)])?;project_properties(&port.properties,"semio_graph_port_property",port_id,&mut projection)?;}
  project_properties(&node.properties,"semio_graph_property",id,&mut projection)?;
 }
 for(ordinal,edge)in self.edges.iter().enumerate(){
  let source=nodes.binary_search_by(|row|row.0.cmp(edge.source.value.as_str())).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio graph source"))?;
  let target=nodes.binary_search_by(|row|row.0.cmp(edge.target.value.as_str())).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio graph target"))?;
  let id=projection.insert("semio_graph_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id.value),Cell::Integer(nodes[source].1),Cell::Integer(nodes[target].1),Cell::Text(&edge.kind),Cell::Text(&edge.label),edge.source_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null),edge.target_port.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
  project_properties(&edge.properties,"semio_graph_edge_property",id,&mut projection)?;
 }
 projection.finish()
}
}
fn project_properties(properties:&[SemioValueEntry],table:&str,owner:i64,projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for(ordinal,property)in properties.iter().enumerate(){let value_id=project_value_tree(&property.value,VALUES,None,projection)?;projection.insert(table,&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Integer(value_id)])?;}Ok(())
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

