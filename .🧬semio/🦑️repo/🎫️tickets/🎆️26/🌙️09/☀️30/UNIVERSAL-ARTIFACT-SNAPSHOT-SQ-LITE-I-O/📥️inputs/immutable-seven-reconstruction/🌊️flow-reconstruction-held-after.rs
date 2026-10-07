fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=single_float_row(database,"semio_flow_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow document identifier"))}
 let nodes=RowIndex::new(database,"semio_flow_node",8,float_columns("semio_flow_node"),control,"invalid Semio flow node ownership or identity")?;
 let parameters=RowIndex::new(database,"semio_flow_parameter",5,float_columns("semio_flow_parameter"),control,"invalid Semio flow parameter owner or identity")?;
 let edges=RowIndex::new(database,"semio_flow_edge",9,float_columns("semio_flow_edge"),control,"invalid Semio flow edge ownership or identity")?;
 let order=nodes.ordered(2,control,"Semio flow node ordinals must be contiguous")?;let edge_order=edges.ordered(2,control,"Semio flow edge ordinals must be contiguous")?;
 nodes.unique_text(nodes.indices(),3,control,"invalid Semio flow node ownership or identity")?;edges.unique_text(edges.indices(),3,control,"invalid Semio flow edge ownership or identity")?;
 for(count,&index)in nodes.indices().iter().enumerate(){if nodes.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow node ownership or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,nodes.len())?;}
 for(count,&index)in parameters.indices().iter().enumerate(){let row=parameters.row(index)?;if nodes.get(row.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow parameter owner or identity"))}row.integer(2)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,parameters.len())?;}
 let grouped=parameters.grouped_by(2,control,"Semio flow parameter ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),nodes:Vec::new(),edges:Vec::new()});snapshot.get_mut().nodes=reserve(order.len(),control)?;snapshot.get_mut().edges=reserve(edge_order.len(),control)?;let mut completed=0;
 for index in order{
  let row=nodes.row(index)?;let range=parameters.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let position=SemioPoint2{x:row.real(6)?,y:row.real(7)?};let mut node=Owned::new(FlowNode{id:String::new(),kind:String::new(),label:String::new(),position,params:Vec::new()});node.get_mut().params=reserve(range.len(),control)?;
  node.get_mut().id=reconstruct_text(control,row.text(3)?)?;node.get_mut().kind=reconstruct_text(control,row.text(4)?)?;node.get_mut().label=reconstruct_text(control,row.text(5)?)?;
  for index in range{
   let row=parameters.row(grouped[index])?;let mut param=Owned::new(FlowParam{key:String::new(),value:String::new()});param.get_mut().key=reconstruct_text(control,row.text(3)?)?;param.get_mut().value=reconstruct_text(control,row.text(4)?)?;node.get_mut().params.push(param.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().nodes.push(node.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 for index in edge_order{
  let row=edges.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow edge ownership or identity"))}
  let from=nodes.get(row.integer(4)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow source node"))?;let to=nodes.get(row.integer(6)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow target node"))?;
  let mut edge=Owned::new(FlowEdge{id:String::new(),from:PortRef{node:String::new(),port:String::new()},to:PortRef{node:String::new(),port:String::new()},kind:String::new()});
  edge.get_mut().id=reconstruct_text(control,row.text(3)?)?;edge.get_mut().from.node=reconstruct_text(control,from.text(3)?)?;edge.get_mut().from.port=reconstruct_text(control,row.text(5)?)?;edge.get_mut().to.node=reconstruct_text(control,to.text(3)?)?;edge.get_mut().to.port=reconstruct_text(control,row.text(7)?)?;edge.get_mut().kind=reconstruct_text(control,row.text(8)?)?;
  snapshot.get_mut().edges.push(edge.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
