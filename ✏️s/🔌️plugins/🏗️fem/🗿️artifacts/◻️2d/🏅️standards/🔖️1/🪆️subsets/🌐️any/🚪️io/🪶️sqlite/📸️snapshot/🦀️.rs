//! 🏗️ Literal finite-element entities, ordered ownership and exact binary64 words.
use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use crate::standards::v1::subsets::any::schema::structure::{FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoad, FemLoadCase, FemRegion, FemSection, FemSupport};
use semio_s_fem_material::planar::FemMaterialValue;
use semio_s_fem_node::planar::FemNodeValue;
use semio_s_fem_sqlite as rows;
use rows::*;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Cell,FloatColumn}}};
use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

const DOCUMENT:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const NODE:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const REGION:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(7)];
const POINT:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const MATERIAL:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const SECTION:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const NODAL:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const UDL:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3)];
const AREA:&[FloatColumn]=&[FloatColumn::Binary64(2)];
const TERM:&[FloatColumn]=&[FloatColumn::Binary64(4)];
fn dof(value:FemDof)->&'static str{match value{FemDof::Tx=>"Tx",FemDof::Ty=>"Ty",FemDof::Tz=>"Tz",FemDof::Rx=>"Rx",FemDof::Ry=>"Ry",FemDof::Rz=>"Rz"}}
fn read_dof(value:&str)->Result<FemDof,ValueError>{match value{"Tx"=>Ok(FemDof::Tx),"Ty"=>Ok(FemDof::Ty),"Tz"=>Ok(FemDof::Tz),"Rx"=>Ok(FemDof::Rx),"Ry"=>Ok(FemDof::Ry),"Rz"=>Ok(FemDof::Rz),_=>Err(invalid("FEM2d support/load DOF differs"))}}
fn forecast_with(source:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,action:&mut dyn FnMut()->Result<(),ValueError>)->Result<usize,ValueError>{action()?;
 let mut total=1;control.check_rows(total)?;
 action()?;add_rows(&mut total,source.nodes.len(),control)?;action()?;add_rows(&mut total,source.elements.len(),control)?;action()?;add_rows(&mut total,source.regions.len(),control)?;action()?;add_rows(&mut total,source.materials.len(),control)?;action()?;add_rows(&mut total,source.sections.len(),control)?;action()?;add_rows(&mut total,source.supports.len(),control)?;action()?;add_rows(&mut total,source.load_cases.len(),control)?;action()?;add_rows(&mut total,source.combinations.len(),control)?;
 control.checkpoint(phase,0,source.regions.len())?;
 for(n,region)in source.regions.iter().enumerate(){action()?;add_rows(&mut total,region.outline.len(),control)?;add_rows(&mut total,region.holes.len(),control)?;for(i,hole)in region.holes.iter().enumerate(){action()?;add_rows(&mut total,hole.len(),control)?;if i%256==0{control.checkpoint(phase,i,region.holes.len())?;}}if n%256==0{control.checkpoint(phase,n,source.regions.len())?;}}
 control.checkpoint(phase,source.regions.len(),source.regions.len())?;
 for(n,support)in source.supports.iter().enumerate(){action()?;add_rows(&mut total,support.fixed.len(),control)?;if n%256==0{control.checkpoint(phase,n,source.supports.len())?;}}
 for(n,case)in source.load_cases.iter().enumerate(){action()?;add_rows(&mut total,case.loads.len().checked_mul(2).ok_or_else(||invalid("FEM2d load count overflow"))?,control)?;if n%256==0{control.checkpoint(phase,n,source.load_cases.len())?;}}
 for(n,combination)in source.combinations.iter().enumerate(){action()?;add_rows(&mut total,combination.terms.len(),control)?;if n%256==0{control.checkpoint(phase,n,source.combinations.len())?;}}Ok(total)
}
fn forecast(value:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{forecast_with(value,control,phase,&mut ||Ok(()))}
fn forecast_selected(value:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,action:&mut dyn FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
 let total=forecast_with(value,control,phase,action)?;
 let mut writer=RowWriter::forecast_controlled(control,phase,action)?;
 value.write_sqlite_rows(&mut writer,total)?;writer.finish_forecast(total)
}
fn forecast_decoding(value:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
 let(mut work,mut pending)=(0,false);let native=owner.native();
 let result={let mut action=||{
  if pending{pending=false;let posted=native.commit_work(1);work+=1;posted?;}
  native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"FEM forecast requires original decoder work"))}
  pending=true;Ok(())
 };forecast_selected(value,control,SqliteSnapshotPhase::DecodeNative,&mut action)};
 let post=if pending{let result=native.commit_work(1);work+=1;result}else{Ok(())};
 let result=match(result,post){(Ok(()),Ok(()))=>Ok(()),(Err(error),Ok(()))=>Err(error.into()),(Ok(()),Err(error))=>Err(store::NativeSnapshotFailure::work_post(error)),(Err(primary),Err(post))=>Err(store::NativeSnapshotFailure::primary_with_work(primary,post))};
 semio_framework_value::RetainedWorkOutcome{result,work}
}
fn forecast_encoding(value:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
 let(mut work,mut pending)=(0,false);let native=owner.native();
 let result={let mut action=||{
  if pending{pending=false;let posted=native.commit_work(1);work+=1;posted?;}
  native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"FEM forecast requires original encoder work"))}
  pending=true;Ok(())
 };forecast_selected(value,control,SqliteSnapshotPhase::EncodeNative,&mut action)};
 let post=if pending{let result=native.commit_work(1);work+=1;result}else{Ok(())};
 let result=match(result,post){(Ok(()),Ok(()))=>Ok(()),(Err(error),Ok(()))=>Err(error.into()),(Ok(()),Err(error))=>Err(store::NativeSnapshotFailure::work_post(error)),(Err(primary),Err(post))=>Err(store::NativeSnapshotFailure::primary_with_work(primary,post))};
 semio_framework_value::RetainedWorkOutcome{result,work}
}
impl ArtifactSqliteSnapshotHost for Fem2dSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome{
  store::decode_sqlite_snapshot_record_receiving(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),&mut |record,retained,original,sql|store::native_snapshot_turn(|work|{
   work.take(Self::receive_exchange_record(record,retained,original))?;
   work.take(forecast_decoding(retained.as_ref().ok_or_else(||invalid("FEM retained snapshot is absent"))?,sql,original))?;
   Ok(())
  }),c,native_control,output)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome{if let Err(error)=(||->Result<(),ValueError>{let total=forecast(self,c,SqliteSnapshotPhase::EncodeNative)?;let mut out=RowWriter::forecast(c,SqliteSnapshotPhase::EncodeNative)?;self.write_sqlite_rows(&mut out,total)?;out.finish_forecast(total)?;Ok(())})(){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),&mut |native|self.__dsl_to_record_controlled(native),c,native_owner,output)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.fem.fem2d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("FEM2d owned dialect differs")));}self.validate_interaction_identities().map_err(store::io_schema::IoError::from_value_error)?;c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(store::io_schema::IoError::from_value_error)?;Ok(store::io_schema::IoOutcome::clean(()))}
}
impl pack::snapshot::ArtifactSqliteSnapshot for Fem2dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let total=forecast(self,c,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,c)?;self.write_sqlite_rows(&mut out,total)?;out.checkpoint_total(total)?;out.finish()
 }
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Self::from_reference(decode_reference_database(d,c)?)}
}
/// ❄️ Relational authoring values remain explicit until a caller supplies the native persistent receiving owner.
pub fn decode_reference_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Fem2dSnapshotValue,ValueError>{
  store::sqlite_snapshot::validate_sqlite_database_schema(d,<Fem2dSnapshot as pack::snapshot::ArtifactSqliteSnapshot>::SQLITE_SCHEMA,c.limits()).map_err(|e|invalid(e.to_string()))?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=entities(d,"fem2d_document",4,DOCUMENT,c)?;if documents.len()!=1{return Err(invalid("FEM2d needs exactly one document"));}let document=*documents.get(&1).ok_or_else(||invalid("FEM2d document identity differs"))?;
  let region_rows=entities(d,"fem2d_region",8,REGION,c)?;let support_rows=entities(d,"fem2d_support",5,&[],c)?;let case_rows=entities(d,"fem2d_load_case",6,&[],c)?;let combination_rows=entities(d,"fem2d_combination",5,&[],c)?;
  let mut outlines=groups(entities(d,"fem2d_outline_vertex",5,POINT,c)?,&region_rows,c)?;let hole_rows=entities(d,"fem2d_hole",3,&[],c)?;let mut hole_vertices=groups(entities(d,"fem2d_hole_vertex",5,POINT,c)?,&hole_rows,c)?;let mut holes=groups(hole_rows,&region_rows,c)?;
  let mut fixed=groups(entities(d,"fem2d_support_dof",4,&[],c)?,&support_rows,c)?;let load_rows=entities(d,"fem2d_load",5,&[],c)?;
  let mut nodal=variants(entities(d,"fem2d_nodal_load",4,NODAL,c)?,&load_rows,"nodal",4,c)?;let mut udl=variants(entities(d,"fem2d_member_udl",4,UDL,c)?,&load_rows,"memberUdl",4,c)?;let mut area=variants(entities(d,"fem2d_area_load",3,AREA,c)?,&load_rows,"area",4,c)?;let mut loads=groups(load_rows,&case_rows,c)?;
  let mut terms=groups(entities(d,"fem2d_combination_term",5,TERM,c)?,&combination_rows,c)?;
  let(mut nodes,mut elements,mut regions,mut materials,mut sections,mut supports,mut load_cases,mut combinations)=(Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new());
  for row in ordered(entities(d,"fem2d_node",6,NODE,c)?.into_values(),1,c)?{nodes.push(FemNodeValue{id:text(row,3,c)?,x:real(row,4,c)?,y:real(row,5,c)?});}
  for row in ordered(entities(d,"fem2d_element",9,&[],c)?.into_values(),1,c)?{let(id,start,end,material_id,section_id)=(text(row,4,c)?,text(row,5,c)?,text(row,6,c)?,text(row,7,c)?,text(row,8,c)?);elements.push(match row.text(3)?{"bar"=>FemElement::Bar{id,start,end,material_id,section_id},"beam"=>FemElement::Beam{id,start,end,material_id,section_id},_=>return Err(invalid("FEM2d element variant differs"))});}
  for row in ordered(region_rows.into_values(),1,c)?{let outline=points(outlines.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?;let mut region_holes=Vec::new();for hole in ordered(holes.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{region_holes.push(points(hole_vertices.remove(&hole.rowid).unwrap_or_default(),hole.rowid,c)?);}regions.push(FemRegion{id:text(row,3,c)?,name:text(row,4,c)?,outline,holes:region_holes,thickness:real(row,5,c)?,material_id:text(row,6,c)?,mesh_size:real(row,7,c)?});}
  for row in ordered(entities(d,"fem2d_material",8,MATERIAL,c)?.into_values(),1,c)?{materials.push(FemMaterialValue{id:text(row,3,c)?,name:text(row,4,c)?,e:real(row,5,c)?,nu:real(row,6,c)?,rho:real(row,7,c)?});}
  for row in ordered(entities(d,"fem2d_section",7,SECTION,c)?.into_values(),1,c)?{sections.push(FemSection{id:text(row,3,c)?,name:text(row,4,c)?,area:real(row,5,c)?,iy:real(row,6,c)?});}
  for row in ordered(support_rows.into_values(),1,c)?{let mut support_fixed=Vec::new();for value in ordered(fixed.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{support_fixed.push(read_dof(value.text(3)?)?);}supports.push(FemSupport{id:text(row,3,c)?,node_id:text(row,4,c)?,fixed:support_fixed});}
  for row in ordered(case_rows.into_values(),1,c)?{let mut case_loads=Vec::new();for load in ordered(loads.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{let id=text(load,3,c)?;case_loads.push(match load.text(4)?{"nodal"=>{let value=nodal.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing nodal load"))?;FemLoad::Nodal{id,node_id:text(value,1,c)?,dof:read_dof(value.text(2)?)?,value:real(value,3,c)?}},"memberUdl"=>{let value=udl.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing member load"))?;FemLoad::MemberUdl{id,element_id:text(value,1,c)?,wx:real(value,2,c)?,wy:real(value,3,c)?}},"area"=>{let value=area.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing area load"))?;FemLoad::Area{id,region_id:text(value,1,c)?,pressure:real(value,2,c)?}},_=>return Err(invalid("FEM2d load variant differs"))});}let self_weight=match row.integer(5)?{0=>false,1=>true,_=>return Err(invalid("FEM2d load case boolean differs"))};load_cases.push(FemLoadCase{id:text(row,3,c)?,name:text(row,4,c)?,loads:case_loads,self_weight});}
  for row in ordered(combination_rows.into_values(),1,c)?{let mut combination_terms=Vec::new();for term in ordered(terms.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{combination_terms.push(FemCombinationTerm{case_id:text(term,3,c)?,factor:real(term,4,c)?});}combinations.push(FemCombination{id:text(row,3,c)?,name:text(row,4,c)?,terms:combination_terms});}
  let analysis=FemAnalysisSettings{modal_count:count(document,1,c)?,buckling_count:count(document,2,c)?,deformation_scale:real(document,3,c)?};c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;let snapshot=Fem2dSnapshotValue{nodes,elements,regions,materials,sections,supports,load_cases,combinations,analysis};if let Err(error)=snapshot.validate_interaction_identities(){<Fem2dSnapshotValue as semio_framework_value::FromValue>::retire_decoded(snapshot);return Err(error)}Ok(snapshot)
 }

impl Fem2dSnapshot{
 fn write_sqlite_rows<W:RowSink>(&self,out:&mut W,_total:usize)->Result<(),ValueError>{macro_rules! emit{($o:ident,insert,$t:expr,$c:expr)=>{{let cells=$c;$o.insert($t,cells)}};($o:ident,insert_float,$t:expr,$c:expr,$f:expr)=>{{let cells=$c;$o.insert_float($t,cells,$f)}};($o:ident,insert_key_float,$t:expr,$k:expr,$c:expr,$f:expr)=>{{let cells=$c;$o.insert_key_float($t,$k,cells,$f)}};}out.work()?;
  let(mut modal,mut buckling)=([0u8;20],[0u8;20]);
  emit!(out,insert_key_float,"fem2d_document",1,&[{out.work()?;Cell::Text(decimal(self.analysis.modal_count,&mut modal))},{out.work()?;Cell::Text(decimal(self.analysis.buckling_count,&mut buckling))},{out.work()?;Cell::Real(self.analysis.deformation_scale)}],DOCUMENT)?;
  for(n,node)in self.nodes.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_node",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&node.id)},{out.work()?;Cell::Real(node.x)},{out.work()?;Cell::Real(node.y)}],NODE)?;}
  for(n,element)in self.elements.iter().enumerate(){out.work()?;let(kind,id,start,end,material,section)=match element{FemElement::Bar{id,start,end,material_id,section_id}=>("bar",id,start,end,material_id,section_id),FemElement::Beam{id,start,end,material_id,section_id}=>("beam",id,start,end,material_id,section_id)};emit!(out,insert,"fem2d_element",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(kind)},{out.work()?;Cell::Text(id)},{out.work()?;Cell::Text(start)},{out.work()?;Cell::Text(end)},{out.work()?;Cell::Text(material)},{out.work()?;Cell::Text(section)}])?;}
  for(n,region)in self.regions.iter().enumerate(){out.work()?;let parent=emit!(out,insert_float,"fem2d_region",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&region.id)},{out.work()?;Cell::Text(&region.name)},{out.work()?;Cell::Real(region.thickness)},{out.work()?;Cell::Text(&region.material_id)},{out.work()?;Cell::Real(region.mesh_size)}],REGION)?;for(i,point)in region.outline.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_outline_vertex",&[{out.work()?;Cell::Integer(parent)},{out.work()?;ordinal(i)?},{out.work()?;Cell::Real(point[0])},{out.work()?;Cell::Real(point[1])}],POINT)?;}for(i,hole)in region.holes.iter().enumerate(){out.work()?;let owner=emit!(out,insert,"fem2d_hole",&[{out.work()?;Cell::Integer(parent)},{out.work()?;ordinal(i)?}])?;for(j,point)in hole.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_hole_vertex",&[{out.work()?;Cell::Integer(owner)},{out.work()?;ordinal(j)?},{out.work()?;Cell::Real(point[0])},{out.work()?;Cell::Real(point[1])}],POINT)?;}}}
  for(n,material)in self.materials.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_material",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&material.id)},{out.work()?;Cell::Text(&material.name)},{out.work()?;Cell::Real(material.e)},{out.work()?;Cell::Real(material.nu)},{out.work()?;Cell::Real(material.rho)}],MATERIAL)?;}
  for(n,section)in self.sections.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_section",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&section.id)},{out.work()?;Cell::Text(&section.name)},{out.work()?;Cell::Real(section.area)},{out.work()?;Cell::Real(section.iy)}],SECTION)?;}
  for(n,support)in self.supports.iter().enumerate(){out.work()?;let parent=emit!(out,insert,"fem2d_support",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&support.id)},{out.work()?;Cell::Text(&support.node_id)}])?;for(i,value)in support.fixed.iter().enumerate(){out.work()?;emit!(out,insert,"fem2d_support_dof",&[{out.work()?;Cell::Integer(parent)},{out.work()?;ordinal(i)?},{out.work()?;Cell::Text(dof(*value))}])?;}}
  for(n,case)in self.load_cases.iter().enumerate(){out.work()?;let owner=emit!(out,insert,"fem2d_load_case",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&case.id)},{out.work()?;Cell::Text(&case.name)},{out.work()?;Cell::Integer(i64::from(case.self_weight))}])?;for(i,load)in case.loads.iter().enumerate(){out.work()?;let(id,kind)=match load{FemLoad::Nodal{id,..}=>(id,"nodal"),FemLoad::MemberUdl{id,..}=>(id,"memberUdl"),FemLoad::Area{id,..}=>(id,"area")};let parent=emit!(out,insert,"fem2d_load",&[{out.work()?;Cell::Integer(owner)},{out.work()?;ordinal(i)?},{out.work()?;Cell::Text(id)},{out.work()?;Cell::Text(kind)}])?;match load{FemLoad::Nodal{node_id,dof:value_dof,value,..}=>emit!(out,insert_key_float,"fem2d_nodal_load",parent,&[{out.work()?;Cell::Text(node_id)},{out.work()?;Cell::Text(dof(*value_dof))},{out.work()?;Cell::Real(*value)}],NODAL)?,FemLoad::MemberUdl{element_id,wx,wy,..}=>emit!(out,insert_key_float,"fem2d_member_udl",parent,&[{out.work()?;Cell::Text(element_id)},{out.work()?;Cell::Real(*wx)},{out.work()?;Cell::Real(*wy)}],UDL)?,FemLoad::Area{region_id,pressure,..}=>emit!(out,insert_key_float,"fem2d_area_load",parent,&[{out.work()?;Cell::Text(region_id)},{out.work()?;Cell::Real(*pressure)}],AREA)?,}}}
  for(n,combination)in self.combinations.iter().enumerate(){out.work()?;let parent=emit!(out,insert,"fem2d_combination",&[{out.work()?;Cell::Integer(1)},{out.work()?;ordinal(n)?},{out.work()?;Cell::Text(&combination.id)},{out.work()?;Cell::Text(&combination.name)}])?;for(i,term)in combination.terms.iter().enumerate(){out.work()?;emit!(out,insert_float,"fem2d_combination_term",&[{out.work()?;Cell::Integer(parent)},{out.work()?;ordinal(i)?},{out.work()?;Cell::Text(&term.case_id)},{out.work()?;Cell::Real(term.factor)}],TERM)?;}}
  Ok(())
 }
}

