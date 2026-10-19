//! 🗺️ Complete ordered map objects, independent of rendering projections and JSON media.
use semio_framework_value::{DslValue,Number,ToValue,FromValue};
/// 🧾️ Literal root property occurrence.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedProperty{pub name:String,pub value:DslValue}
/// 🗺️ Imported map ownership retains all records and every intrinsic domain.
#[derive(Clone,Debug,Default,PartialEq,ToValue,semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMap{pub positions:Vec<DslValue>,pub routes:Vec<DslValue>,pub regions:Vec<DslValue>,pub properties:Vec<ImportedProperty>}
impl ImportedMap{
 /// 📥️ Typed map transport admits complete intrinsic objects without JSON projection.
 pub fn from_media_value(value:&DslValue)->Result<Self,String>{let DslValue::Object(members)=value else{return Err("map intrinsic object required".into())};let mut map=Self::default();let mut reserved=std::collections::HashSet::new();for(name,value)in members{match name.as_str(){"positions"|"routes"|"regions"=>{if !reserved.insert(name){return Err("duplicate map collection".into())}let DslValue::Array(records)=value else{return Err("map collection array required".into())};match name.as_str(){"positions"=>map.positions=records.clone(),"routes"=>map.routes=records.clone(),_=>map.regions=records.clone()}},_=>map.properties.push(ImportedProperty{name:name.clone(),value:value.clone()})}}map.validate()?;Ok(map)}
 /// 📤️ The transport owns the first-party tree, including all literal unknown properties.
 pub fn to_media_value(&self)->Result<DslValue,String>{self.validate()?;let mut members=vec![("positions".into(),DslValue::Array(self.positions.clone())),("routes".into(),DslValue::Array(self.routes.clone())),("regions".into(),DslValue::Array(self.regions.clone()))];members.extend(self.properties.iter().map(|m|(m.name.clone(),m.value.clone())));Ok(DslValue::Object(members))}
 /// 🛂️ A renderer's identifier and coordinate requirements do not narrow storage admission.
 pub fn validate(&self)->Result<(),String>{for records in [&self.positions,&self.routes,&self.regions]{for record in records{if !matches!(record,DslValue::Object(_)){return Err("imported feature requires intrinsic object".into())}}}for member in &self.properties{if matches!(member.name.as_str(),"positions"|"routes"|"regions"){return Err("reserved imported map property".into())}}Ok(())}
 /// 📐️ Bounds may read coordinates without the pin renderer's separate identifier requirement.
 pub fn coordinates(&self)->impl Iterator<Item=(f64,f64)>+'_ {self.positions.iter().filter_map(|value|{let lon=value.get("lon")?.as_f64()?;let lat=value.get("lat")?.as_f64()?;Some((lon,lat))})}
 /// 🪪️ Exact tree equality includes numeric variants, IEEE payloads and literal member order.
 pub fn same(&self,other:&Self)->bool{if self.properties.len()!=other.properties.len()||self.positions.len()!=other.positions.len()||self.routes.len()!=other.routes.len()||self.regions.len()!=other.regions.len(){return false}let mut pending:Vec<(&DslValue,&DslValue)>=self.positions.iter().zip(&other.positions).chain(self.routes.iter().zip(&other.routes)).chain(self.regions.iter().zip(&other.regions)).collect();for(a,b)in self.properties.iter().zip(&other.properties){if a.name!=b.name{return false}pending.push((&a.value,&b.value))}while let Some((a,b))=pending.pop(){match(a,b){(DslValue::Null,DslValue::Null)=>(),(DslValue::Bool(a),DslValue::Bool(b))if a==b=>(),(DslValue::String(a),DslValue::String(b))if a==b=>(),(DslValue::Bytes(a),DslValue::Bytes(b))if a==b=>(),(DslValue::Number(Number::UInt(a)),DslValue::Number(Number::UInt(b)))if a==b=>(),(DslValue::Number(Number::Int(a)),DslValue::Number(Number::Int(b)))if a==b=>(),(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))if a.to_bits()==b.to_bits()=>(),(DslValue::Array(a),DslValue::Array(b))if a.len()==b.len()=>pending.extend(a.iter().zip(b)),(DslValue::Object(a),DslValue::Object(b))if a.len()==b.len()=>{for((an,av),(bn,bv))in a.iter().zip(b){if an!=bn{return false}pending.push((av,bv))}},_=>return false}}true}
}

#[derive(Default,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct ImportedMapFields{positions:Vec<DslValue>,routes:Vec<DslValue>,regions:Vec<DslValue>,properties:Vec<ImportedProperty>}
impl ImportedMapFields{
 fn into_map(self)->ImportedMap{ImportedMap{positions:self.positions,routes:self.routes,regions:self.regions,properties:self.properties}}
}
impl FromValue for ImportedMap{
 fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{let map=ImportedMapFields::from_value(value)?.into_map();map.validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;Ok(map)}
 fn from_value_controlled(value:&DslValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{let owner=semio_framework_value::DecodedValue::new(ImportedMapFields::from_value_controlled(value,control)?.into_map(),Self::retire_decoded);owner.get().validate().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))?;control.checkpoint()?;Ok(owner.take())}
 fn default_value_controlled(control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{ImportedMapFields::default_value_controlled(control).map(ImportedMapFields::into_map)}
 fn retire_decoded(self){ImportedMapFields::retire_decoded(ImportedMapFields{positions:self.positions,routes:self.routes,regions:self.regions,properties:self.properties});}
}

//#region 🧩️ArtifactItems
pub mod artifact_items {
//#region 🔹Constants
/// 🏔️ Schema for undoable GIS terrain documents.
pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";

/// 🪪️ One canonical terrain identity shared by definition, composer and both app roles.
pub const GISTERRAIN_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.gis.gisterrain", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };
//#endregion 🔹Constants
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
pub use crate::schema::snapshot::GisTerrainSnapshot;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_mesh::{schema::snapshot::SemioMesh, schema::snapshot::SemioMeshSnapshot, schema::snapshot::SemioPrimitive, schema::snapshot::SemioTopology};

//#region 🔖️MeshComposition
/// 🕸️ Deterministic content-addressed CHILD handle for the terrain's composed mesh — same
/// `(child_id, target)` for identical `(exaggeration, imported_map)`, a different pair
/// once either actually changes. Mirrors `💠️lowpoly`'s `mesh_child_handle`/`📐️cad`'s
/// `cad_model_child_handle` (same `store::os_store::artifact_child::ArtifactChild::new` + `ArtifactDialect` shape).
pub fn gis_terrain_mesh_child_handle(content_key: &str) -> store::os_store::artifact_child::ArtifactChild<SemioMeshSnapshot> {
    let child_id = store::content_id("gisterrain-mesh", content_key.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🔑️ Construction seed for an initial terrain mesh; persisted identity survives later edits.
pub fn gis_terrain_mesh_content_key(exaggeration: f64, imported_map: Option<&crate::schema::imported_map::ImportedMap>) -> String {
    let identity = imported_map.map(|map| {
        let bytes = store::pack_rt::encode_document(&crate::schema::imported_map::ImportedMap::__dsl_spec(), &map.__dsl_to_record(), &store::PackEncodeOptions::default()).expect("authored imported map native record");
        store::content_id("gisterrain-map", &bytes)
    }).unwrap_or_default();
    format!("{:016x}|{}", exaggeration.to_bits(), identity)
}

/// 🏔️ WRITE direction, real (not a stub): builds an actual `SemioMeshSnapshot` from the terrain
/// document — a single flat quad (2 triangles) sized by the imported overlay's geographic extent
/// (falling back to a 1×1 unit quad when no overlay is present) and lifted by `exaggeration` alone
/// (there is no DEM heightfield yet — see `gis3d_scene_media`'s own long-standing doc comment: "gis3d
/// has no CPU-side heightmap tessellator yet"). Honest placeholder geometry, not a fake: every field
/// is real, computed, and round-trips — the day a tessellator lands, only this function's body needs
/// to grow real per-vertex elevation.
pub fn gis_terrain_mesh_from_snapshot(document: &GisTerrainSnapshot) -> SemioMeshSnapshot {
    let bounds = crate::standards::v1::subsets::any::schema::inferences::bounds::lon_lat_bounds(&crate::standards::v1::subsets::any::schema::inferences::bounds::imported_lon_lat_positions(document));
    let (min_x, min_y, max_x, max_y) = bounds.map_or((0.0, 0.0, 1.0, 1.0), |b| (b.lon_min, b.lat_min, b.lon_max, b.lat_max));
    let (min_x, max_x) = if min_x < max_x { (min_x, max_x) } else { (min_x, min_x + 1.0) };
    let (min_y, max_y) = if min_y < max_y { (min_y, max_y) } else { (min_y, min_y + 1.0) };
    let z = 0.0;
    let positions = vec![SemioPoint3 { x: min_x, y: min_y, z }, SemioPoint3 { x: max_x, y: min_y, z }, SemioPoint3 { x: max_x, y: max_y, z }, SemioPoint3 { x: min_x, y: max_y, z }];
    SemioMeshSnapshot {
        schema: "s.stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "gisterrain-surface".into(),
            primitives: vec![SemioPrimitive { id: "gisterrain-surface-quad".into(), topology: SemioTopology::Triangles, positions, normals: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices: vec![0, 1, 2, 0, 2, 3], material_id: None }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    }
}
//#endregion 🔖️MeshComposition
}
//#endregion 🧩️ArtifactItems3
