//! 🏗️ Complete dimensional finite-element entities and exact binary64 words.
use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use crate::{FemAnalysisSettings,FemAxis,FemCombination,FemDof,FemElement,FemLoad,FemLoadCase,FemMaterial,FemNode,FemSection,FemSolid,FemSupport};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Cell,FloatColumn}}};
#[path="../../../../../../../../../🧩️sqlite/🦀️.rs"]
mod rows;
use rows::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

const DOCUMENT:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const NODE:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const FRAME:&[FloatColumn]=&[FloatColumn::Binary64(1)];
const MATERIAL:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const SECTION:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const SOLID:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(8)];
const POINT:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const NODAL:&[FloatColumn]=&[FloatColumn::Binary64(3)];
const UDL:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const AREA:&[FloatColumn]=&[FloatColumn::Binary64(2)];
const TERM:&[FloatColumn]=&[FloatColumn::Binary64(4)];
fn dof(v:FemDof)->&'static str{match v{FemDof::Tx=>"Tx",FemDof::Ty=>"Ty",FemDof::Tz=>"Tz",FemDof::Rx=>"Rx",FemDof::Ry=>"Ry",FemDof::Rz=>"Rz"}}
fn read_dof(v:&str)->Result<FemDof,ValueError>{match v{"Tx"=>Ok(FemDof::Tx),"Ty"=>Ok(FemDof::Ty),"Tz"=>Ok(FemDof::Tz),"Rx"=>Ok(FemDof::Rx),"Ry"=>Ok(FemDof::Ry),"Rz"=>Ok(FemDof::Rz),_=>Err(invalid("FEM3d DOF differs"))}}
fn axis(v:FemAxis)->&'static str{match v{FemAxis::X=>"x",FemAxis::Y=>"y",FemAxis::Z=>"z"}}
fn read_axis(v:&str)->Result<FemAxis,ValueError>{match v{"x"=>Ok(FemAxis::X),"y"=>Ok(FemAxis::Y),"z"=>Ok(FemAxis::Z),_=>Err(invalid("FEM3d extrusion axis differs"))}}
fn forecast(s:&Fem3dSnapshot,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 let mut total=1;c.check_rows(total)?;for n in[s.nodes.len(),s.elements.len(),s.materials.len(),s.sections.len(),s.solids.len(),s.supports.len(),s.load_cases.len(),s.combinations.len()]{add_rows(&mut total,n,c)?;}
 for(i,v)in s.elements.iter().enumerate(){if matches!(v,FemElement::Frame{..}){add_rows(&mut total,1,c)?;}if i%256==0{c.checkpoint(phase,i,s.elements.len())?;}}
 for(i,v)in s.solids.iter().enumerate(){add_rows(&mut total,v.outline.len(),c)?;add_rows(&mut total,v.holes.len(),c)?;for(j,h)in v.holes.iter().enumerate(){add_rows(&mut total,h.len(),c)?;if j%256==0{c.checkpoint(phase,j,v.holes.len())?;}}if i%256==0{c.checkpoint(phase,i,s.solids.len())?;}}
 for(i,v)in s.supports.iter().enumerate(){add_rows(&mut total,v.fixed.len(),c)?;if i%256==0{c.checkpoint(phase,i,s.supports.len())?;}}
 for(i,v)in s.load_cases.iter().enumerate(){add_rows(&mut total,v.loads.len().checked_mul(2).ok_or_else(||invalid("FEM3d load row count overflow"))?,c)?;if i%256==0{c.checkpoint(phase,i,s.load_cases.len())?;}}
 for(i,v)in s.combinations.iter().enumerate(){add_rows(&mut total,v.terms.len(),c)?;if i%256==0{c.checkpoint(phase,i,s.combinations.len())?;}}Ok(total)
}
impl ArtifactSqliteSnapshot for Fem3dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let value=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },c,native_control)?;let total=forecast(&value,c,SqliteSnapshotPhase::DecodeNative)?;let mut out=RowWriter::forecast(c,SqliteSnapshotPhase::DecodeNative)?;value.write_sqlite_rows(&mut out,total)?;out.finish_forecast(total)?;Ok(value)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{let total=forecast(self,c,SqliteSnapshotPhase::EncodeNative)?;let mut out=RowWriter::forecast(c,SqliteSnapshotPhase::EncodeNative)?;self.write_sqlite_rows(&mut out,total)?;out.finish_forecast(total)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c,native_owner)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let total=forecast(self,c,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,c)?;self.write_sqlite_rows(&mut out,total)?;out.checkpoint_total(total)?;out.finish()
 }
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  store::sqlite_snapshot::validate_sqlite_database_schema(d,Self::SQLITE_SCHEMA,c.limits()).map_err(|e|invalid(e.to_string()))?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let mut documents=entities(d,"fem3d_document",4,DOCUMENT,c)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("FEM3d requires one identity-1 document"));}let document=documents.remove(&1).expect("validated document");
  let element_rows=entities(d,"fem3d_element",9,&[],c)?;let mut frames=variants(entities(d,"fem3d_frame",2,FRAME,c)?,&element_rows,"frame",3,c)?;let solid_rows=entities(d,"fem3d_solid",11,SOLID,c)?;let support_rows=entities(d,"fem3d_support",5,&[],c)?;let case_rows=entities(d,"fem3d_load_case",6,&[],c)?;let combination_rows=entities(d,"fem3d_combination",5,&[],c)?;
  let mut outlines=groups(entities(d,"fem3d_outline_vertex",5,POINT,c)?,&solid_rows,c)?;let hole_rows=entities(d,"fem3d_hole",3,&[],c)?;let mut vertices=groups(entities(d,"fem3d_hole_vertex",5,POINT,c)?,&hole_rows,c)?;let mut holes=groups(hole_rows,&solid_rows,c)?;let mut fixed=groups(entities(d,"fem3d_support_dof",4,&[],c)?,&support_rows,c)?;let load_rows=entities(d,"fem3d_load",5,&[],c)?;
  let mut nodal=variants(entities(d,"fem3d_nodal_load",4,NODAL,c)?,&load_rows,"nodal",4,c)?;let mut udl=variants(entities(d,"fem3d_member_udl",5,UDL,c)?,&load_rows,"memberUdl",4,c)?;let mut area=variants(entities(d,"fem3d_area_load",3,AREA,c)?,&load_rows,"area",4,c)?;let mut loads=groups(load_rows,&case_rows,c)?;let mut terms=groups(entities(d,"fem3d_combination_term",5,TERM,c)?,&combination_rows,c)?;
  let(mut nodes,mut elements,mut materials,mut sections,mut solids,mut supports,mut load_cases,mut combinations)=(Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new());
  for r in ordered(entities(d,"fem3d_node",7,NODE,c)?.into_values(),1,c)?{nodes.push(FemNode{id:text(r,3,c)?,x:real(r,4,c)?,y:real(r,5,c)?,z:real(r,6,c)?});}
  for r in ordered(element_rows.into_values(),1,c)?{let(id,start,end,material_id,section_id)=(text(r,4,c)?,text(r,5,c)?,text(r,6,c)?,text(r,7,c)?,text(r,8,c)?);elements.push(match r.text(3)?{"bar"=>FemElement::Bar{id,start,end,material_id,section_id},"frame"=>{let frame=frames.remove(&r.rowid).ok_or_else(||invalid("FEM3d missing frame roll"))?;FemElement::Frame{id,start,end,material_id,section_id,roll:real(frame,1,c)?}},_=>return Err(invalid("FEM3d member variant differs"))});}
  for r in ordered(entities(d,"fem3d_material",9,MATERIAL,c)?.into_values(),1,c)?{materials.push(FemMaterial{id:text(r,3,c)?,name:text(r,4,c)?,e:real(r,5,c)?,nu:real(r,6,c)?,rho:real(r,7,c)?,g:real(r,8,c)?});}
  for r in ordered(entities(d,"fem3d_section",9,SECTION,c)?.into_values(),1,c)?{sections.push(FemSection{id:text(r,3,c)?,name:text(r,4,c)?,area:real(r,5,c)?,iy:real(r,6,c)?,iz:real(r,7,c)?,j:real(r,8,c)?});}
  for r in ordered(solid_rows.into_values(),1,c)?{let outline=points(outlines.remove(&r.rowid).unwrap_or_default(),r.rowid,c)?;let mut owned_holes=Vec::new();for hole in ordered(holes.remove(&r.rowid).unwrap_or_default(),r.rowid,c)?{owned_holes.push(points(vertices.remove(&hole.rowid).unwrap_or_default(),hole.rowid,c)?);}solids.push(FemSolid{id:text(r,3,c)?,name:text(r,4,c)?,outline,holes:owned_holes,base_z:real(r,5,c)?,height:real(r,6,c)?,layers:count(r,7,c)?,mesh_size:real(r,8,c)?,material_id:text(r,9,c)?,axis:read_axis(r.text(10)?)?});}
  for r in ordered(support_rows.into_values(),1,c)?{let mut owned_fixed=Vec::new();for f in ordered(fixed.remove(&r.rowid).unwrap_or_default(),r.rowid,c)?{owned_fixed.push(read_dof(f.text(3)?)?);}supports.push(FemSupport{id:text(r,3,c)?,node_id:text(r,4,c)?,fixed:owned_fixed});}
  for r in ordered(case_rows.into_values(),1,c)?{let mut case_loads=Vec::new();for l in ordered(loads.remove(&r.rowid).unwrap_or_default(),r.rowid,c)?{let id=text(l,3,c)?;case_loads.push(match l.text(4)?{"nodal"=>{let x=nodal.remove(&l.rowid).ok_or_else(||invalid("FEM3d missing nodal load"))?;FemLoad::Nodal{id,node_id:text(x,1,c)?,dof:read_dof(x.text(2)?)?,value:real(x,3,c)?}},"memberUdl"=>{let x=udl.remove(&l.rowid).ok_or_else(||invalid("FEM3d missing member load"))?;FemLoad::MemberUdl{id,element_id:text(x,1,c)?,wx:real(x,2,c)?,wy:real(x,3,c)?,wz:real(x,4,c)?}},"area"=>{let x=area.remove(&l.rowid).ok_or_else(||invalid("FEM3d missing area load"))?;FemLoad::Area{id,solid_id:text(x,1,c)?,pressure:real(x,2,c)?}},_=>return Err(invalid("FEM3d load variant differs"))});}let self_weight=match r.integer(5)?{0=>false,1=>true,_=>return Err(invalid("FEM3d self-weight boolean differs"))};load_cases.push(FemLoadCase{id:text(r,3,c)?,name:text(r,4,c)?,loads:case_loads,self_weight});}
  for r in ordered(combination_rows.into_values(),1,c)?{let mut owned_terms=BTreeMap::new();let mut previous=None;for t in ordered(terms.remove(&r.rowid).unwrap_or_default(),r.rowid,c)?{let key=t.text(3)?;if previous.is_some_and(|p:&str|p>=key){return Err(invalid("FEM3d map keys are not uniquely UTF-8 ordered"));}owned_terms.insert(text(t,3,c)?,real(t,4,c)?);previous=Some(key);}combinations.push(FemCombination{id:text(r,3,c)?,name:text(r,4,c)?,terms:owned_terms});}
  let analysis=FemAnalysisSettings{modal_count:count(document,1,c)?,buckling_count:count(document,2,c)?,deformation_scale:real(document,3,c)?};c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{nodes,elements,materials,sections,solids,supports,load_cases,combinations,analysis})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.fem.fem3d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("FEM3d owned dialect differs")));}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(store::io_schema::IoError::from_value_error)?;Ok(store::io_schema::IoOutcome::clean(()))}
}

impl Fem3dSnapshot{
 fn write_sqlite_rows(&self,out:&mut RowWriter<'_,'_>,total:usize)->Result<(),ValueError>{
  let(mut modal,mut buckling)=([0u8;20],[0u8;20]);
  out.insert_key_float("fem3d_document",1,&[Cell::Text(decimal(self.analysis.modal_count,&mut modal)),Cell::Text(decimal(self.analysis.buckling_count,&mut buckling)),Cell::Real(self.analysis.deformation_scale)],DOCUMENT)?;
  for(i,v)in self.nodes.iter().enumerate(){out.insert_float("fem3d_node",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Real(v.x),Cell::Real(v.y),Cell::Real(v.z)],NODE)?;if i%256==0{out.checkpoint_total(total)?;}}
  for(i,v)in self.elements.iter().enumerate(){match v{FemElement::Bar{id,start,end,material_id,section_id}=>{out.insert("fem3d_element",&[Cell::Integer(1),ordinal(i)?,Cell::Text("bar"),Cell::Text(id),Cell::Text(start),Cell::Text(end),Cell::Text(material_id),Cell::Text(section_id)])?;},FemElement::Frame{id,start,end,material_id,section_id,roll}=>{let owner=out.insert("fem3d_element",&[Cell::Integer(1),ordinal(i)?,Cell::Text("frame"),Cell::Text(id),Cell::Text(start),Cell::Text(end),Cell::Text(material_id),Cell::Text(section_id)])?;out.insert_key_float("fem3d_frame",owner,&[Cell::Real(*roll)],FRAME)?;}}}
  for(i,v)in self.materials.iter().enumerate(){out.insert_float("fem3d_material",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.name),Cell::Real(v.e),Cell::Real(v.nu),Cell::Real(v.rho),Cell::Real(v.g)],MATERIAL)?;}
  for(i,v)in self.sections.iter().enumerate(){out.insert_float("fem3d_section",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.name),Cell::Real(v.area),Cell::Real(v.iy),Cell::Real(v.iz),Cell::Real(v.j)],SECTION)?;}
  for(i,v)in self.solids.iter().enumerate(){let mut layers=[0u8;20];let owner=out.insert_float("fem3d_solid",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.name),Cell::Real(v.base_z),Cell::Real(v.height),Cell::Text(decimal(v.layers,&mut layers)),Cell::Real(v.mesh_size),Cell::Text(&v.material_id),Cell::Text(axis(v.axis))],SOLID)?;for(j,p)in v.outline.iter().enumerate(){out.insert_float("fem3d_outline_vertex",&[Cell::Integer(owner),ordinal(j)?,Cell::Real(p[0]),Cell::Real(p[1])],POINT)?;}for(j,h)in v.holes.iter().enumerate(){let parent=out.insert("fem3d_hole",&[Cell::Integer(owner),ordinal(j)?])?;for(k,p)in h.iter().enumerate(){out.insert_float("fem3d_hole_vertex",&[Cell::Integer(parent),ordinal(k)?,Cell::Real(p[0]),Cell::Real(p[1])],POINT)?;}}}
  for(i,v)in self.supports.iter().enumerate(){let owner=out.insert("fem3d_support",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.node_id)])?;for(j,d)in v.fixed.iter().enumerate(){out.insert("fem3d_support_dof",&[Cell::Integer(owner),ordinal(j)?,Cell::Text(dof(*d))])?;}}
  for(i,v)in self.load_cases.iter().enumerate(){let owner=out.insert("fem3d_load_case",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.name),Cell::Integer(i64::from(v.self_weight))])?;for(j,x)in v.loads.iter().enumerate(){let(id,kind)=match x{FemLoad::Nodal{id,..}=>(id,"nodal"),FemLoad::MemberUdl{id,..}=>(id,"memberUdl"),FemLoad::Area{id,..}=>(id,"area")};let parent=out.insert("fem3d_load",&[Cell::Integer(owner),ordinal(j)?,Cell::Text(id),Cell::Text(kind)])?;match x{FemLoad::Nodal{node_id,dof:d,value,..}=>{out.insert_key_float("fem3d_nodal_load",parent,&[Cell::Text(node_id),Cell::Text(dof(*d)),Cell::Real(*value)],NODAL)?;},FemLoad::MemberUdl{element_id,wx,wy,wz,..}=>{out.insert_key_float("fem3d_member_udl",parent,&[Cell::Text(element_id),Cell::Real(*wx),Cell::Real(*wy),Cell::Real(*wz)],UDL)?;},FemLoad::Area{solid_id,pressure,..}=>{out.insert_key_float("fem3d_area_load",parent,&[Cell::Text(solid_id),Cell::Real(*pressure)],AREA)?;}}}}
  for(i,v)in self.combinations.iter().enumerate(){let owner=out.insert("fem3d_combination",&[Cell::Integer(1),ordinal(i)?,Cell::Text(&v.id),Cell::Text(&v.name)])?;for(j,(case,factor))in v.terms.iter().enumerate(){out.insert_float("fem3d_combination_term",&[Cell::Integer(owner),ordinal(j)?,Cell::Text(case),Cell::Real(*factor)],TERM)?;}}
  Ok(())
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
