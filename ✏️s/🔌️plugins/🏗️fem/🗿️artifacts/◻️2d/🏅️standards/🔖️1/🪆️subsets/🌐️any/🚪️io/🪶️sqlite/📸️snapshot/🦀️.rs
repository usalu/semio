//! 🏗️ Literal finite-element entities, ordered ownership and exact binary64 words.
use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use crate::{FemAnalysisSettings,FemCombination,FemCombinationTerm,FemDof,FemElement,FemLoad,FemLoadCase,FemMaterial,FemNode,FemRegion,FemSection,FemSupport};
#[path="../../../../../../../../../🧩️sqlite/🦀️.rs"]
mod rows;
use rows::*;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Cell,FloatColumn}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
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
fn forecast(source:&Fem2dSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 let mut total=1;control.check_rows(total)?;
 for count in[source.nodes.len(),source.elements.len(),source.regions.len(),source.materials.len(),source.sections.len(),source.supports.len(),source.load_cases.len(),source.combinations.len()]{add_rows(&mut total,count,control)?;}
 control.checkpoint(phase,0,source.regions.len())?;
 for(n,region)in source.regions.iter().enumerate(){add_rows(&mut total,region.outline.len(),control)?;add_rows(&mut total,region.holes.len(),control)?;for(i,hole)in region.holes.iter().enumerate(){add_rows(&mut total,hole.len(),control)?;if i%256==0{control.checkpoint(phase,i,region.holes.len())?;}}if n%256==0{control.checkpoint(phase,n,source.regions.len())?;}}
 control.checkpoint(phase,source.regions.len(),source.regions.len())?;
 for(n,support)in source.supports.iter().enumerate(){add_rows(&mut total,support.fixed.len(),control)?;if n%256==0{control.checkpoint(phase,n,source.supports.len())?;}}
 for(n,case)in source.load_cases.iter().enumerate(){add_rows(&mut total,case.loads.len().checked_mul(2).ok_or_else(||invalid("FEM2d load count overflow"))?,control)?;if n%256==0{control.checkpoint(phase,n,source.load_cases.len())?;}}
 for(n,combination)in source.combinations.iter().enumerate(){add_rows(&mut total,combination.terms.len(),control)?;if n%256==0{control.checkpoint(phase,n,source.combinations.len())?;}}Ok(total)
}
impl ArtifactSqliteSnapshot for Fem2dSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let value=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),c)?;let total=forecast(&value,c,SqliteSnapshotPhase::DecodeNative)?;let mut out=RowWriter::forecast(c,SqliteSnapshotPhase::DecodeNative)?;value.write_sqlite_rows(&mut out,total)?;out.finish_forecast(total)?;Ok(value)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{let total=forecast(self,c,SqliteSnapshotPhase::EncodeNative)?;let mut out=RowWriter::forecast(c,SqliteSnapshotPhase::EncodeNative)?;self.write_sqlite_rows(&mut out,total)?;out.finish_forecast(total)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let total=forecast(self,c,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,c)?;self.write_sqlite_rows(&mut out,total)?;out.checkpoint_total(total)?;out.finish()
 }
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  store::sqlite_snapshot::validate_sqlite_database_schema(d,Self::SQLITE_SCHEMA,c.limits()).map_err(|e|invalid(e.to_string()))?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let documents=entities(d,"fem2d_document",4,DOCUMENT,c)?;if documents.len()!=1{return Err(invalid("FEM2d needs exactly one document"));}let document=*documents.get(&1).ok_or_else(||invalid("FEM2d document identity differs"))?;
  let region_rows=entities(d,"fem2d_region",8,REGION,c)?;let support_rows=entities(d,"fem2d_support",5,&[],c)?;let case_rows=entities(d,"fem2d_load_case",6,&[],c)?;let combination_rows=entities(d,"fem2d_combination",5,&[],c)?;
  let mut outlines=groups(entities(d,"fem2d_outline_vertex",5,POINT,c)?,&region_rows,c)?;let hole_rows=entities(d,"fem2d_hole",3,&[],c)?;let mut hole_vertices=groups(entities(d,"fem2d_hole_vertex",5,POINT,c)?,&hole_rows,c)?;let mut holes=groups(hole_rows,&region_rows,c)?;
  let mut fixed=groups(entities(d,"fem2d_support_dof",4,&[],c)?,&support_rows,c)?;let load_rows=entities(d,"fem2d_load",5,&[],c)?;
  let mut nodal=variants(entities(d,"fem2d_nodal_load",4,NODAL,c)?,&load_rows,"nodal",4,c)?;let mut udl=variants(entities(d,"fem2d_member_udl",4,UDL,c)?,&load_rows,"memberUdl",4,c)?;let mut area=variants(entities(d,"fem2d_area_load",3,AREA,c)?,&load_rows,"area",4,c)?;let mut loads=groups(load_rows,&case_rows,c)?;
  let mut terms=groups(entities(d,"fem2d_combination_term",5,TERM,c)?,&combination_rows,c)?;
  let(mut nodes,mut elements,mut regions,mut materials,mut sections,mut supports,mut load_cases,mut combinations)=(Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new());
  for row in ordered(entities(d,"fem2d_node",6,NODE,c)?.into_values(),1,c)?{nodes.push(FemNode{id:text(row,3,c)?,x:real(row,4,c)?,y:real(row,5,c)?});}
  for row in ordered(entities(d,"fem2d_element",9,&[],c)?.into_values(),1,c)?{let(id,start,end,material_id,section_id)=(text(row,4,c)?,text(row,5,c)?,text(row,6,c)?,text(row,7,c)?,text(row,8,c)?);elements.push(match row.text(3)?{"bar"=>FemElement::Bar{id,start,end,material_id,section_id},"beam"=>FemElement::Beam{id,start,end,material_id,section_id},_=>return Err(invalid("FEM2d element variant differs"))});}
  for row in ordered(region_rows.into_values(),1,c)?{let outline=points(outlines.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?;let mut region_holes=Vec::new();for hole in ordered(holes.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{region_holes.push(points(hole_vertices.remove(&hole.rowid).unwrap_or_default(),hole.rowid,c)?);}regions.push(FemRegion{id:text(row,3,c)?,name:text(row,4,c)?,outline,holes:region_holes,thickness:real(row,5,c)?,material_id:text(row,6,c)?,mesh_size:real(row,7,c)?});}
  for row in ordered(entities(d,"fem2d_material",8,MATERIAL,c)?.into_values(),1,c)?{materials.push(FemMaterial{id:text(row,3,c)?,name:text(row,4,c)?,e:real(row,5,c)?,nu:real(row,6,c)?,rho:real(row,7,c)?});}
  for row in ordered(entities(d,"fem2d_section",7,SECTION,c)?.into_values(),1,c)?{sections.push(FemSection{id:text(row,3,c)?,name:text(row,4,c)?,area:real(row,5,c)?,iy:real(row,6,c)?});}
  for row in ordered(support_rows.into_values(),1,c)?{let mut support_fixed=Vec::new();for value in ordered(fixed.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{support_fixed.push(read_dof(value.text(3)?)?);}supports.push(FemSupport{id:text(row,3,c)?,node_id:text(row,4,c)?,fixed:support_fixed});}
  for row in ordered(case_rows.into_values(),1,c)?{let mut case_loads=Vec::new();for load in ordered(loads.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{let id=text(load,3,c)?;case_loads.push(match load.text(4)?{"nodal"=>{let value=nodal.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing nodal load"))?;FemLoad::Nodal{id,node_id:text(value,1,c)?,dof:read_dof(value.text(2)?)?,value:real(value,3,c)?}},"memberUdl"=>{let value=udl.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing member load"))?;FemLoad::MemberUdl{id,element_id:text(value,1,c)?,wx:real(value,2,c)?,wy:real(value,3,c)?}},"area"=>{let value=area.remove(&load.rowid).ok_or_else(||invalid("FEM2d missing area load"))?;FemLoad::Area{id,region_id:text(value,1,c)?,pressure:real(value,2,c)?}},_=>return Err(invalid("FEM2d load variant differs"))});}let self_weight=match row.integer(5)?{0=>false,1=>true,_=>return Err(invalid("FEM2d load case boolean differs"))};load_cases.push(FemLoadCase{id:text(row,3,c)?,name:text(row,4,c)?,loads:case_loads,self_weight});}
  for row in ordered(combination_rows.into_values(),1,c)?{let mut combination_terms=Vec::new();for term in ordered(terms.remove(&row.rowid).unwrap_or_default(),row.rowid,c)?{combination_terms.push(FemCombinationTerm{case_id:text(term,3,c)?,factor:real(term,4,c)?});}combinations.push(FemCombination{id:text(row,3,c)?,name:text(row,4,c)?,terms:combination_terms});}
  let analysis=FemAnalysisSettings{modal_count:count(document,1,c)?,buckling_count:count(document,2,c)?,deformation_scale:real(document,3,c)?};c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{nodes,elements,regions,materials,sections,supports,load_cases,combinations,analysis})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,_database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.fem.fem2d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("FEM2d owned dialect differs")));}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(store::io_schema::IoError::from_value_error)?;Ok(store::io_schema::IoOutcome::clean(()))}
}

impl Fem2dSnapshot{
 fn write_sqlite_rows(&self,out:&mut RowWriter<'_,'_>,total:usize)->Result<(),ValueError>{
  let(mut modal,mut buckling)=([0u8;20],[0u8;20]);
  out.insert_key_float("fem2d_document",1,&[Cell::Text(decimal(self.analysis.modal_count,&mut modal)),Cell::Text(decimal(self.analysis.buckling_count,&mut buckling)),Cell::Real(self.analysis.deformation_scale)],DOCUMENT)?;
  for(n,node)in self.nodes.iter().enumerate(){out.insert_float("fem2d_node",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&node.id),Cell::Real(node.x),Cell::Real(node.y)],NODE)?;}
  for(n,element)in self.elements.iter().enumerate(){let(kind,id,start,end,material,section)=match element{FemElement::Bar{id,start,end,material_id,section_id}=>("bar",id,start,end,material_id,section_id),FemElement::Beam{id,start,end,material_id,section_id}=>("beam",id,start,end,material_id,section_id)};out.insert("fem2d_element",&[Cell::Integer(1),ordinal(n)?,Cell::Text(kind),Cell::Text(id),Cell::Text(start),Cell::Text(end),Cell::Text(material),Cell::Text(section)])?;}
  for(n,region)in self.regions.iter().enumerate(){let parent=out.insert_float("fem2d_region",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&region.id),Cell::Text(&region.name),Cell::Real(region.thickness),Cell::Text(&region.material_id),Cell::Real(region.mesh_size)],REGION)?;for(i,point)in region.outline.iter().enumerate(){out.insert_float("fem2d_outline_vertex",&[Cell::Integer(parent),ordinal(i)?,Cell::Real(point[0]),Cell::Real(point[1])],POINT)?;}for(i,hole)in region.holes.iter().enumerate(){let owner=out.insert("fem2d_hole",&[Cell::Integer(parent),ordinal(i)?])?;for(j,point)in hole.iter().enumerate(){out.insert_float("fem2d_hole_vertex",&[Cell::Integer(owner),ordinal(j)?,Cell::Real(point[0]),Cell::Real(point[1])],POINT)?;}}}
  for(n,material)in self.materials.iter().enumerate(){out.insert_float("fem2d_material",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&material.id),Cell::Text(&material.name),Cell::Real(material.e),Cell::Real(material.nu),Cell::Real(material.rho)],MATERIAL)?;}
  for(n,section)in self.sections.iter().enumerate(){out.insert_float("fem2d_section",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&section.id),Cell::Text(&section.name),Cell::Real(section.area),Cell::Real(section.iy)],SECTION)?;}
  for(n,support)in self.supports.iter().enumerate(){let parent=out.insert("fem2d_support",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&support.id),Cell::Text(&support.node_id)])?;for(i,value)in support.fixed.iter().enumerate(){out.insert("fem2d_support_dof",&[Cell::Integer(parent),ordinal(i)?,Cell::Text(dof(*value))])?;}}
  for(n,case)in self.load_cases.iter().enumerate(){let owner=out.insert("fem2d_load_case",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&case.id),Cell::Text(&case.name),Cell::Integer(i64::from(case.self_weight))])?;for(i,load)in case.loads.iter().enumerate(){let(id,kind)=match load{FemLoad::Nodal{id,..}=>(id,"nodal"),FemLoad::MemberUdl{id,..}=>(id,"memberUdl"),FemLoad::Area{id,..}=>(id,"area")};let parent=out.insert("fem2d_load",&[Cell::Integer(owner),ordinal(i)?,Cell::Text(id),Cell::Text(kind)])?;match load{FemLoad::Nodal{node_id,dof:value_dof,value,..}=>out.insert_key_float("fem2d_nodal_load",parent,&[Cell::Text(node_id),Cell::Text(dof(*value_dof)),Cell::Real(*value)],NODAL)?,FemLoad::MemberUdl{element_id,wx,wy,..}=>out.insert_key_float("fem2d_member_udl",parent,&[Cell::Text(element_id),Cell::Real(*wx),Cell::Real(*wy)],UDL)?,FemLoad::Area{region_id,pressure,..}=>out.insert_key_float("fem2d_area_load",parent,&[Cell::Text(region_id),Cell::Real(*pressure)],AREA)?,}}}
  for(n,combination)in self.combinations.iter().enumerate(){let parent=out.insert("fem2d_combination",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&combination.id),Cell::Text(&combination.name)])?;for(i,term)in combination.terms.iter().enumerate(){out.insert_float("fem2d_combination_term",&[Cell::Integer(parent),ordinal(i)?,Cell::Text(&term.case_id),Cell::Real(term.factor)],TERM)?;}}
  Ok(())
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

