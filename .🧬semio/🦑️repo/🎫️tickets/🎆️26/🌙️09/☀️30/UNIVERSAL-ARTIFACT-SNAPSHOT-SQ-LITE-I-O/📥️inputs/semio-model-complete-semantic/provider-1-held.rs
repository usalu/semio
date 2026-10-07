//! 🏛️ Spatial hierarchy, physical elements, geometry identities and typed property relationships.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::model::schema::snapshot::{SemioModelSnapshot,SpatialNode,SpatialKind,SemioModelElement,ElementClass,GeometryRef,PropertySet,Property,PsetValue,ModelRelation,RelationKind};
use crate::standards::v1::subsets::base::schema::geometry::{SemioTransform,SemioPoint3,SemioQuaternion};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🔍️ Resolves a paid literal Model identity with bounded text comparisons.
fn entity_reference(ids:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(ids[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(ids[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio Model entity identity"))
}
/// 🏛️ Resolves only the declared spatial owner partition for hierarchy and containment.
fn spatial_reference(ids:&[(&str,i64)],id:&str,count:usize,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let row=entity_reference(ids,id,out)?;if row>number(count)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio Model parent is outside its spatial partition"))}Ok(row)
}
/// ♻️ Detects spatial cycles using paid fixed-width parent marks without growing path ownership.
fn spatial_cycles(parents:&mut[(Option<usize>,u8)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for root in 0..parents.len(){
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;match parents[index].1{2=>break,1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic Semio Model spatial hierarchy")),_=>{parents[index].1=1;current=parents[index].0;}}}
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;if parents[index].1!=1{break}parents[index].1=2;current=parents[index].0;}
 }Ok(())
}
/// 🪪️ Collects the exact entity partition using a paid borrowed frontier and cancellable comparisons.
fn entity_frontier<'a>(snapshot:&'a SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let count=snapshot.spatial.len().checked_add(snapshot.elements.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity extent overflow"))?;let phase=out.phase();let mut ids=out.allocate_frontier(count)?;
 for(ordinal,id)in snapshot.spatial.iter().map(|node|node.id.as_str()).chain(snapshot.elements.iter().map(|element|element.id.as_str())).enumerate(){out.checkpoint()?;ids.push((id,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio Model entity identity overflow"))?)?));}
 out.sort_frontier(&mut ids,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in ids.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model entity identity"))}}Ok(ids)
}
/// 📐️ Projects all ten exact placement words through the actual float column declaration.
fn placement(id:i64,t:SemioTransform,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("semio_model_placement",id,&[Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_model_placement"))
}
/// 🫳️ Visits the real Model entities, hierarchy, typed property variants and relation endpoints.
pub(crate)fn visit_rows(snapshot:&SemioModelSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let entities=entity_frontier(snapshot,out)?;let mut parents=out.allocate_frontier(snapshot.spatial.len())?;
 for node in &snapshot.spatial{out.checkpoint()?;let parent=match &node.parent_id{None=>None,Some(id)=>Some(usize::try_from(spatial_reference(&entities,id,snapshot.spatial.len(),out)?-1).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)};parents.push((parent,0u8));}
 spatial_cycles(&mut parents,out)?;let phase=out.phase();let mut relations=out.allocate_frontier(snapshot.relations.len())?;for edge in &snapshot.relations{out.checkpoint()?;relations.push(edge.id.as_str());}
 out.sort_frontier(&mut relations,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,control))?;
 for pair in relations.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio Model relation identity"))}}
 out.insert_key("semio_model_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,node)in snapshot.spatial.iter().enumerate(){
  let id=entity_reference(&entities,&node.id,out)?;let parent=parents[ordinal].0.map(|index|number(index+1).map(Cell::Integer)).transpose()?.unwrap_or(Cell::Null);
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("spatial"),Cell::Integer(number(ordinal)?),Cell::Text(&node.id)])?;
  out.insert_key("semio_model_spatial",id,&[Cell::Text(match node.kind{SpatialKind::Site=>"site",SpatialKind::Building=>"building",SpatialKind::Storey=>"storey",SpatialKind::Space=>"space"}),Cell::Text(&node.name),parent])?;
  placement(id,node.placement,out)?;
 }
 for(ordinal,element)in snapshot.elements.iter().enumerate(){
  let id=entity_reference(&entities,&element.id,out)?;let(class,name)=class(&element.class);let spatial=match &element.spatial_id{None=>Cell::Null,Some(id)=>Cell::Integer(spatial_reference(&entities,id,snapshot.spatial.len(),out)?)};
  out.insert_key("semio_model_entity",id,&[Cell::Integer(1),Cell::Text("element"),Cell::Integer(number(ordinal)?),Cell::Text(&element.id)])?;
  out.insert_key("semio_model_element",id,&[Cell::Text(class),name.map(Cell::Text).unwrap_or(Cell::Null),spatial])?;placement(id,element.placement,out)?;
  let(kind,target)=match &element.geometry{GeometryRef::None=>("none",Cell::Null),GeometryRef::Brep{brep_id}=>("brep",Cell::Text(brep_id)),GeometryRef::Mesh{mesh_id}=>("mesh",Cell::Text(mesh_id))};
  out.insert_key("semio_model_geometry_reference",id,&[Cell::Text(kind),target])?;
  for(ordinal,pset)in element.psets.iter().enumerate(){
   let owner=out.insert("semio_model_property_set",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&pset.name)])?;
   for(ordinal,property)in pset.properties.iter().enumerate(){
    let(kind,text,number_value,boolean)=match &property.value{PsetValue::Text{value}=>("text",Cell::Text(value),Cell::Null,Cell::Null),PsetValue::Number{value}=>("number",Cell::Null,Cell::Real(*value),Cell::Null),PsetValue::Boolean{value}=>("boolean",Cell::Null,Cell::Null,Cell::Integer(i64::from(*value)))};
    out.insert_float("semio_model_property",&[Cell::Integer(owner),Cell::Integer(number(ordinal)?),Cell::Text(&property.key),Cell::Text(kind),text,number_value,boolean],float_columns("semio_model_property"))?;
   }
  }
 }
 for(ordinal,edge)in snapshot.relations.iter().enumerate(){
  let(kind,label)=relation(&edge.kind);let source=entity_reference(&entities,&edge.from,out)?;let target=entity_reference(&entities,&edge.to,out)?;
  out.insert("semio_model_relation",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Text(kind),label.map(Cell::Text).unwrap_or(Cell::Null),Cell::Integer(source),Cell::Integer(target)])?;
 }Ok(())
}
/// 🎟️ Admits all typed Model cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioModelSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model row identity or columns"))}else{Ok(())}}
fn optional_integer(row:SqliteRow<'_>,index:usize)->Result<Option<i64>,ValueError>{if row.is_null(index)?{Ok(None)}else{row.integer(index).map(Some)}}
fn ordered<'a>(mut rows:Vec<SqliteRow<'a>>,column:usize)->Result<Vec<SqliteRow<'a>>,ValueError>{rows.sort_by_key(|row|row.integer(column).unwrap_or(-1));for(ordinal,row)in rows.iter().enumerate(){if row.integer(column)?!=number(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio model ordinals must be contiguous"));}}Ok(rows)}
fn rows<'a>(db:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,SqliteRow<'a>>,ValueError>{let mut result=BTreeMap::new();for(count,row)in float_rows(db,table,control)?.into_iter().enumerate(){identity(row,columns)?;if result.insert(row.rowid,row).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio model row identity"));}if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn cycles(parents:&BTreeMap<i64,Option<i64>>,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{let mut marks=BTreeMap::new();let mut count=0;for &root in parents.keys(){let mut path=Vec::new();let mut current=Some(root);while let Some(id)=current{match marks.get(&id){Some(2)=>break,Some(1)=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic Semio model spatial hierarchy")),_=>{}}marks.insert(id,1);path.push(id);current=*parents.get(&id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio model spatial parent"))?;count+=1;if count%256==0{control.checkpoint(phase,0,0)?;}}for id in path{marks.insert(id,2);}}Ok(())}
fn class(value:&ElementClass)->(&'static str,Option<&str>){match value{ElementClass::Wall=>("wall",None),ElementClass::Slab=>("slab",None),ElementClass::Column=>("column",None),ElementClass::Beam=>("beam",None),ElementClass::Door=>("door",None),ElementClass::Window=>("window",None),ElementClass::Roof=>("roof",None),ElementClass::Stair=>("stair",None),ElementClass::Furniture=>("furniture",None),ElementClass::Other{name}=>("other",Some(name))}}
fn relation(value:&RelationKind)->(&'static str,Option<&str>){match value{RelationKind::Aggregates=>("aggregates",None),RelationKind::ContainedIn=>("contained_in",None),RelationKind::ConnectsTo=>("connects_to",None),RelationKind::FillsVoid=>("fills_void",None),RelationKind::VoidsElement=>("voids_element",None),RelationKind::Other{label}=>("other",Some(label))}}
fn restore_placement(row:SqliteRow<'_>)->Result<SemioTransform,ValueError>{Ok(SemioTransform{translation:SemioPoint3{x:row.real(1)?,y:row.real(2)?,z:row.real(3)?},rotation:SemioQuaternion{x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,w:row.real(7)?},scale:SemioPoint3{x:row.real(8)?,y:row.real(9)?,z:row.real(10)?}})}
impl ArtifactSqliteSnapshot for SemioModelSnapshot{
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::model::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="model"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_model_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioModelSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;let document=single_float_row(database,"semio_model_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model document identifier"));}
let entities=rows(database,"semio_model_entity",5,control)?;let mut spatial_rows=rows(database,"semio_model_spatial",4,control)?;let mut element_rows=rows(database,"semio_model_element",4,control)?;let mut placements=rows(database,"semio_model_placement",11,control)?;let mut geometries=rows(database,"semio_model_geometry_reference",3,control)?;let mut names=BTreeMap::new();let mut native_ids=BTreeSet::new();let mut spatial_order=Vec::new();let mut element_order=Vec::new();for(&id,row)in &entities{if row.integer(1)?!=1||!native_ids.insert(row.text(4)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model entity ownership or identity"));}names.insert(id,row.text(4)?);match row.text(2)?{"spatial"=>spatial_order.push(*row),"element"=>element_order.push(*row),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio model entity kind"))}}
let mut parents=BTreeMap::new();for(&id,row)in &spatial_rows{parents.insert(id,optional_integer(*row,3)?);}cycles(&parents,control,SqliteSnapshotPhase::ReconstructSnapshot)?;
let mut psets=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut properties=BTreeMap::<i64,Vec<SqliteRow<'_>>>::new();let mut pset_ids=BTreeSet::new();let mut completed=0usize;for row in float_rows(database,"semio_model_property_set",control)?{identity(row,4)?;if !element_rows.contains_key(&row.integer(1)?)||!pset_ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model property-set owner or identity"));}row.integer(2)?;psets.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut property_ids=BTreeSet::new();for row in float_rows(database,"semio_model_property",control)?{identity(row,8)?;if !pset_ids.contains(&row.integer(1)?)||!property_ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model property owner or identity"));}row.integer(2)?;properties.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut spatial=Vec::new();for entity in ordered(spatial_order,3)?{let row=spatial_rows.remove(&entity.rowid).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio model spatial detail"))?;let parent_id=parents[&entity.rowid].map(|id|reconstruct_text(control,names.get(&id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio model spatial parent"))?)).transpose()?;let kind=match row.text(1)?{"site"=>SpatialKind::Site,"building"=>SpatialKind::Building,"storey"=>SpatialKind::Storey,"space"=>SpatialKind::Space,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio spatial kind"))};spatial.push(SpatialNode{id:reconstruct_text(control,entity.text(4)?)?,kind,name:reconstruct_text(control,row.text(2)?)?,parent_id,placement:restore_placement(placements.remove(&entity.rowid).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing spatial placement"))?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut elements=Vec::new();for entity in ordered(element_order,3)?{let row=element_rows.remove(&entity.rowid).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio model element detail"))?;let other=row.optional_text(2)?;let class=match row.text(1)?{"wall"=>ElementClass::Wall,"slab"=>ElementClass::Slab,"column"=>ElementClass::Column,"beam"=>ElementClass::Beam,"door"=>ElementClass::Door,"window"=>ElementClass::Window,"roof"=>ElementClass::Roof,"stair"=>ElementClass::Stair,"furniture"=>ElementClass::Furniture,"other"=>ElementClass::Other{name:reconstruct_text(control,other.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing custom Semio element class name"))?)?},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio model element class"))};if row.text(1)?!="other"&&other.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected custom Semio element class name"));}let spatial_id=optional_integer(row,3)?.map(|id|if parents.contains_key(&id){reconstruct_text(control,names.get(&id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio element spatial reference"))?)}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio element spatial reference"))}).transpose()?;let reference=geometries.remove(&entity.rowid).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio model geometry reference"))?;let target=reference.optional_text(2)?;let geometry=match reference.text(1)?{"none" if target.is_none()=>GeometryRef::None,"brep"=>GeometryRef::Brep{brep_id:reconstruct_text(control,target.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio BRep geometry identity"))?)?},"mesh"=>GeometryRef::Mesh{mesh_id:reconstruct_text(control,target.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio mesh geometry identity"))?)?},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model geometry reference shape"))};let mut native_psets=Vec::new();
for pset in ordered(psets.remove(&entity.rowid).unwrap_or_default(),2)?{let mut native_properties=Vec::new();for property in ordered(properties.remove(&pset.rowid).unwrap_or_default(),2)?{let value=match property.text(4)?{"text" if property.is_null(6)?&&property.is_null(7)?=>PsetValue::Text{value:reconstruct_text(control,property.text(5)?)?},"number" if property.is_null(5)?&&property.is_null(7)?=>PsetValue::Number{value:property.real(6)?},"boolean" if property.is_null(5)?&&property.is_null(6)?=>PsetValue::Boolean{value:match property.integer(7)?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model boolean property"))}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model property variant shape"))};native_properties.push(Property{key:reconstruct_text(control,property.text(3)?)?,value});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}native_psets.push(PropertySet{name:reconstruct_text(control,pset.text(3)?)?,properties:native_properties});}
elements.push(SemioModelElement{id:reconstruct_text(control,entity.text(4)?)?,class,placement:restore_placement(placements.remove(&entity.rowid).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing element placement"))?)?,geometry,spatial_id,psets:native_psets});}
if !spatial_rows.is_empty()||!element_rows.is_empty()||!placements.is_empty()||!geometries.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan or contradictory Semio model entity detail"));}let mut relations=Vec::new();let mut ids=BTreeSet::new();let mut native_ids=BTreeSet::new();for row in ordered_float_rows(database,"semio_model_relation",2,control)?{identity(row,8)?;if row.integer(1)?!=1||!ids.insert(row.rowid)||!native_ids.insert(row.text(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio model relation owner or identity"));}let other=row.optional_text(5)?;let kind=match row.text(4)?{"aggregates"=>RelationKind::Aggregates,"contained_in"=>RelationKind::ContainedIn,"connects_to"=>RelationKind::ConnectsTo,"fills_void"=>RelationKind::FillsVoid,"voids_element"=>RelationKind::VoidsElement,"other"=>RelationKind::Other{label:reconstruct_text(control,other.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"missing custom Semio relation label"))?)?},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio model relation kind"))};if row.text(4)?!="other"&&other.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected custom Semio model relation label"));}relations.push(ModelRelation{id:reconstruct_text(control,row.text(3)?)?,kind,from:reconstruct_text(control,names.get(&row.integer(6)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio model relation source"))?)?,to:reconstruct_text(control,names.get(&row.integer(7)?).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio model relation target"))?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,spatial,elements,relations})
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_model_property"=>&[FloatColumn::Binary64(6)],"semio_model_placement"=>&[FloatColumn::Binary64(1),FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10)],_=>&[]}}
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,ValueError>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioModelSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.spatial.len())?;for spatial in &self.spatial{b.text(&spatial.id)?;b.text(&spatial.name)?;b.optional_text(spatial.parent_id.as_deref())?;b.scalars(11)?;}b.entities(self.elements.len())?;for element in &self.elements{b.text(&element.id)?;b.optional_text(element.spatial_id.as_deref())?;b.scalars(11)?;if let ElementClass::Other{name}=&element.class{b.text(name)?;}match &element.geometry{GeometryRef::None=>{},GeometryRef::Brep{brep_id}=>b.text(brep_id)?,GeometryRef::Mesh{mesh_id}=>b.text(mesh_id)?}b.entities(element.psets.len())?;for pset in &element.psets{b.text(&pset.name)?;b.entities(pset.properties.len())?;for property in &pset.properties{b.text(&property.key)?;match &property.value{PsetValue::Text{value}=>b.text(value)?,PsetValue::Number{..}|PsetValue::Boolean{..}=>b.scalars(1)?}}}}b.entities(self.relations.len())?;for relation in &self.relations{b.text(&relation.id)?;b.text(&relation.from)?;b.text(&relation.to)?;b.scalars(1)?;if let RelationKind::Other{label}=&relation.kind{b.text(label)?;}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
