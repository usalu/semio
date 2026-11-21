//! 🗺️ Complete ordered map objects, independent of rendering projections and JSON media.
use semio_framework_value::{DslValue,Number,ToValue,FromValue,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::{
 NativeReceive,
 owned::{NativeMapValidation,NativeReceiveTryMap,NativeTryMappedReceiver,tree::NativeValueSource},
};
/// 🧾️ Literal root property occurrence.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive)]
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
 pub fn validate(&self)->Result<(),String>{
  validate_map([&self.positions,&self.routes,&self.regions], &self.properties).map_err(str::to_owned)
 }
 /// 📐️ Bounds may read coordinates without the pin renderer's separate identifier requirement.
 pub fn coordinates(&self)->impl Iterator<Item=(f64,f64)>+'_ {self.positions.iter().filter_map(|value|{let lon=value.get("lon")?.as_f64()?;let lat=value.get("lat")?.as_f64()?;Some((lon,lat))})}
 /// 🪪️ Exact tree equality includes numeric variants, IEEE payloads and literal member order.
 pub fn same(&self,other:&Self)->bool{if self.properties.len()!=other.properties.len()||self.positions.len()!=other.positions.len()||self.routes.len()!=other.routes.len()||self.regions.len()!=other.regions.len(){return false}let mut pending:Vec<(&DslValue,&DslValue)>=self.positions.iter().zip(&other.positions).chain(self.routes.iter().zip(&other.routes)).chain(self.regions.iter().zip(&other.regions)).collect();for(a,b)in self.properties.iter().zip(&other.properties){if a.name!=b.name{return false}pending.push((&a.value,&b.value))}while let Some((a,b))=pending.pop(){match(a,b){(DslValue::Null,DslValue::Null)=>(),(DslValue::Bool(a),DslValue::Bool(b))if a==b=>(),(DslValue::String(a),DslValue::String(b))if a==b=>(),(DslValue::Bytes(a),DslValue::Bytes(b))if a==b=>(),(DslValue::Number(Number::UInt(a)),DslValue::Number(Number::UInt(b)))if a==b=>(),(DslValue::Number(Number::Int(a)),DslValue::Number(Number::Int(b)))if a==b=>(),(DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))if a.to_bits()==b.to_bits()=>(),(DslValue::Array(a),DslValue::Array(b))if a.len()==b.len()=>pending.extend(a.iter().zip(b)),(DslValue::Object(a),DslValue::Object(b))if a.len()==b.len()=>{for((an,av),(bn,bv))in a.iter().zip(b){if an!=bn{return false}pending.push((av,bv))}},_=>return false}}true}
}

fn validate_feature(record: &DslValue) -> Result<(), &'static str> {
 matches!(record,DslValue::Object(_)).then_some(()).ok_or("imported feature requires intrinsic object")
}

fn validate_property(property: &ImportedProperty) -> Result<(), &'static str> {
 (!matches!(property.name.as_str(),"positions"|"routes"|"regions")).then_some(()).ok_or("reserved imported map property")
}

fn validate_map(records: [&[DslValue]; 3], properties: &[ImportedProperty]) -> Result<(), &'static str> {
 for record in records.into_iter().flatten() { validate_feature(record)?; }
 for property in properties { validate_property(property)?; }
 Ok(())
}

#[doc(hidden)]
#[derive(Default,FromValue,semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMapFields{positions:Vec<DslValue>,routes:Vec<DslValue>,regions:Vec<DslValue>,properties:Vec<ImportedProperty>}
impl ImportedMapFields{
 fn records(&self) -> [&[DslValue]; 3] { [&self.positions, &self.routes, &self.regions] }
 fn into_map(self)->ImportedMap{ImportedMap{positions:self.positions,routes:self.routes,regions:self.regions,properties:self.properties}}
}

impl NativeReceiveTryMap<ImportedMapFields> for ImportedMap {
 fn validate_native(value: &ImportedMapFields) -> Result<(), ValueError> {
  validate_map(value.records(), &value.properties).map_err(|message| ValueError::literal(ValueRefusalKind::InvalidValue, message))
 }

 fn from_valid_native(value: ImportedMapFields) -> Self { value.into_map() }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ImportedMapValidation(usize);

impl NativeMapValidation<ImportedMapFields, ImportedMap> for ImportedMapValidation {
 fn demands(&self, _: &ImportedMapFields, _: usize) -> Result<RetirementDemand, ValueError> {
  Ok(RetirementDemand { depth: 1, ..Default::default() })
 }

 fn advance(&mut self, value: &ImportedMapFields, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
  if !effect.performed(1)? {
   return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "imported map validation needs one original work unit"));
  }
  let mut index = self.0;
  let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
  let invalid = |message| ValueError::literal(ValueRefusalKind::InvalidValue, message).with_retained_progress(progress);
  for records in value.records() {
   if let Some(record) = records.get(index) {
    validate_feature(record).map_err(invalid)?;
    self.0 += 1;
    return Ok(RetainedCloneStep::Progress(progress));
   }
   index -= records.len();
  }
  if let Some(property) = value.properties.get(index) {
   validate_property(property).map_err(invalid)?;
   self.0 += 1;
   return Ok(RetainedCloneStep::Progress(progress));
  }
  Ok(RetainedCloneStep::Complete(progress))
 }
}

impl<I: NativeValueSource + ?Sized> NativeReceive<I> for ImportedMap {
 type Receiver = NativeTryMappedReceiver<I, ImportedMapFields, Self, ImportedMapValidation>;

 fn receiver() -> Self::Receiver { Self::Receiver::new() }
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
use crate::schema::terrain_descriptor::TerrainProjectOrigin;
pub use semio_framework_surface::terrain::{TerrainElevationGrid, TerrainTileMeshJson};
use semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint3, SemioUv};
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

/// 🛰️ Reads the packaged Liège elevation grid shared by the demo document and the default scene.
pub fn bundled_terrain_grid() -> &'static TerrainElevationGrid {
    static LIEGE: std::sync::LazyLock<TerrainElevationGrid> = std::sync::LazyLock::new(|| {
        semio_framework_pack_json::from_json_str(
            include_str!("../../🖼️assets/🎬️demo/🏔️elevation.json"),
            semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).expect("sampled Liège elevation grid")
    });
    &LIEGE
}

/// 🪪️ Preserves the attribution of the bundled grid when it is saved as an authored source.
pub fn is_bundled_terrain_grid(grid: &TerrainElevationGrid) -> bool {
    grid == bundled_terrain_grid()
}

/// 📡️ Resolves the elevation source persisted in this terrain artifact.
pub fn gis_terrain_elevation_grid(document: &GisTerrainSnapshot) -> Result<TerrainElevationGrid, &'static str> {
    let source = document.elevation_source.as_ref().ok_or("This terrain has no elevation source")?;
    source.is_valid().then(|| source.grid()).ok_or("The authored terrain elevation grid is invalid")
}

/// 📍️ Uses the authored elevation extent as the geographic origin of a terrain scene.
pub fn terrain_grid_origin([west, south, east, north]: [f64; 4]) -> TerrainProjectOrigin {
    TerrainProjectOrigin { lon: west.midpoint(east), lat: south.midpoint(north) }
}

/// 📐️ Projects the available elevation samples to the same local meter coordinates as the viewport.
pub fn gis_terrain_mesh_buffers(document: &GisTerrainSnapshot) -> Result<TerrainTileMeshJson, &'static str> {
    let grid = gis_terrain_elevation_grid(document)?;
    let origin = terrain_grid_origin(grid.bounds);
    grid.mesh(origin.lon, origin.lat, document.exaggeration)
        .ok_or("Terrain elevation cannot be projected in the defined world coordinates")
}

/// 🏔️ Assembles the projected terrain buffers into the canonical Semio mesh snapshot.
pub fn gis_terrain_mesh_from_snapshot(document: &GisTerrainSnapshot) -> Result<SemioMeshSnapshot, &'static str> {
    let mesh = gis_terrain_mesh_buffers(document)?;
    let points = |values: &[f32]| values.chunks_exact(3).map(|p| SemioPoint3 {
        x: f64::from(p[0]),
        y: f64::from(p[1]),
        z: f64::from(p[2]),
    }).collect();
    Ok(SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "gisterrain-surface".into(),
            primitives: vec![SemioPrimitive {
                id: "gisterrain-elevation".into(),
                topology: SemioTopology::Triangles,
                positions: points(&mesh.positions),
                normals: points(&mesh.normals),
                uvs: mesh.uvs.chunks_exact(2).map(|uv| SemioUv { u: f64::from(uv[0]), v: f64::from(uv[1]) }).collect(),
                colors: Vec::new(),
                indices: mesh.indices,
                material_id: None,
            }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    })
}
//#endregion 🔖️MeshComposition
}
//#endregion 🧩️ArtifactItems3

pub use artifact_items_3::{bundled_terrain_grid, gis_terrain_elevation_grid, gis_terrain_mesh_buffers, is_bundled_terrain_grid, terrain_grid_origin, TerrainTileMeshJson};

pub use semio_framework_surface::terrain::TerrainGridMeshProjection;
