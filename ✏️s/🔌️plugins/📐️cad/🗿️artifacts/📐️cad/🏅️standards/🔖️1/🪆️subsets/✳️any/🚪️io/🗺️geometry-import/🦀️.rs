//! 📥️ Native mesh and fixture projection into owned CAD geometry.
use crate::standards::v1::subsets::any::schema::geometry::*;
use semio_framework_plugin::{ArtifactSerializer, MeshData};
use semio_framework_3d::brep::engine::{Brep,BrepKernel};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::encode_obj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
fn mesh_to_obj_text(mesh: &MeshData) -> Option<String> {
    let semio_mesh = semio_mesh_snapshot_from_mesh_data(mesh)?;
    let obj_snapshot = ::semio_framework_async::poll::resolve_ready(SemioMeshToObj::serialize(&semio_mesh)).ok()?;
    Some(encode_obj(&obj_snapshot))
}
pub(crate) fn cad_object_from_mesh(kernel: &mut Brep, id: impl Into<String>, label: impl Into<String>, typology: impl Into<String>, mesh: &MeshData) -> CadObject {
    let extent = mesh_extent(mesh);
    let solid_handle = mesh_to_obj_text(mesh).and_then(|text| kernel.import_obj(&text, 0.01).ok()).map(|handle| handle.0);
    let primitives = solid_handle.clone().map(|primitive_id| vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() }]).unwrap_or_default();
    CadObject { id: id.into(), label: label.into(), typology: typology.into(), visible: true, locked: false, origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, extent, solid_handle, primitives }
}

use crate::{CadPaneId,CadWorkingScene};
use crate::standards::v1::subsets::any::schema::inferences::cad_brep_kernel;
use std::sync::{Arc,OnceLock};
    const FOREST_LEFT_MODEL_JSON: &str = include_str!("../../📚️examples/🖼️assets/🎮️play/🔣️.json");
    const CAD_MODEL_INDEX_STRUCTURE_CLASSIC: usize = 3;
    const CAD_MODEL_INDEX_ENERGY: usize = 2;
    const CAD_MODEL_INDEX_BUILDING: usize = 1;
    const CAD_MODEL_INDEX_SHAPE: usize = 0;
    pub(crate) fn cad_document_pane_bundle(source_json: &str, model_index: usize) -> (Vec<CadObject>, CadGeometry) {
        let Ok(root) = semio_framework_pack_json::parse(source_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return (Vec::new(), CadGeometry::default());
        };
        let geometry_value = root.pointer(&format!("/models/{model_index}/model/geometry")).map(semio_framework_pack_json::to_dsl_value);
        let geometry = parse_geometry(geometry_value.as_ref());
        let Some(objects_value) = root.pointer(&format!("/models/{model_index}/model/objects")).and_then(|value| value.as_array()) else {
            return (Vec::new(), geometry);
        };
        let objects_value: Vec<semio_framework_value::DslValue> = objects_value.iter().map(semio_framework_pack_json::to_dsl_value).collect();
        let mut kernel = cad_brep_kernel();
        let objects = objects_from_host_snapshot_model(&mut kernel, &objects_value, &geometry);
        (objects, geometry)
    }
    pub(crate) fn forest_pane_bundle(pane: CadPaneId) -> (Vec<CadObject>, CadGeometry) {
        let model_index = match pane {
            CadPaneId::Shape => CAD_MODEL_INDEX_SHAPE,
            CadPaneId::Building => CAD_MODEL_INDEX_BUILDING,
            CadPaneId::Energy => CAD_MODEL_INDEX_ENERGY,
            CadPaneId::StructureClassic => CAD_MODEL_INDEX_STRUCTURE_CLASSIC,
        };
        cad_document_pane_bundle(FOREST_LEFT_MODEL_JSON, model_index)
    }
    pub(crate) fn forest_pane_scene(pane: CadPaneId) -> Arc<CadWorkingScene> {
        static FOREST_PANE_SCENES: OnceLock<[Arc<CadWorkingScene>; 4]> = OnceLock::new();
        FOREST_PANE_SCENES
            .get_or_init(|| {
                CadPaneId::all().map(|pane| {
                    let (objects, geometry) = forest_pane_bundle(pane);
                    let geometry = Some(geometry);
                    Arc::new(match pane {
                        CadPaneId::Shape => CadWorkingScene { objects, geometry, ..Default::default() },
                        CadPaneId::Building => CadWorkingScene { building_objects: objects, building_geometry: geometry, ..Default::default() },
                        CadPaneId::Energy => CadWorkingScene { energy_objects: objects, energy_geometry: geometry, ..Default::default() },
                        CadPaneId::StructureClassic => CadWorkingScene { structure_classic_objects: objects, structure_classic_geometry: geometry, ..Default::default() },
                    })
                })
            })[pane.index()]
        .clone()
    }


const CONCRETE_FOREST_LEFT_SHAPE_MODEL_JSON: &str = include_str!("../../📚️examples/🖼️assets/🎮️play/🔣️.json");

/// 🌲️ Face loops for the shape-pane solid in the Hexagonal Cut Concrete Forest Left play fixture.
pub fn concrete_forest_left_shape_solid_face_loops() -> Result<semio_framework_3d::brep::engine::SolidFaceLoops, String> {
    use semio_framework_3d::brep::engine::{Brep, BrepError, GeometryHandle};
    let root = semio_framework_pack_json::parse(CONCRETE_FOREST_LEFT_SHAPE_MODEL_JSON, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let geometry_value = root.pointer("/models/0/model/geometry").map(semio_framework_pack_json::to_dsl_value);
    let geometry = parse_geometry(geometry_value.as_ref());
    let objects_value = root
        .pointer("/models/0/model/objects")
        .and_then(|value| value.as_array())
        .map(|entries| entries.iter().map(semio_framework_pack_json::to_dsl_value).collect::<Vec<_>>())
        .unwrap_or_default();
    let mut kernel = Brep::new();
    let imported = objects_from_host_snapshot_model(&mut kernel, &objects_value, &geometry);
    let handle = GeometryHandle(imported.first().and_then(|object| object.solid_handle.clone()).ok_or_else(|| "concrete forest shape object has no solid handle".to_string())?);
    kernel.solid_face_loops_sync(&handle).map_err(|error: BrepError| error.to_string())
}
