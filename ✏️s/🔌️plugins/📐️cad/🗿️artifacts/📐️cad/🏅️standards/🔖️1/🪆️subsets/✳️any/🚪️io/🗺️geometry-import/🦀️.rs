//! 📥️ Native mesh and fixture projection into owned CAD geometry.


use semio_s_artifact_cad_cad_model::{snapshot::CadSnapshot, snapshot::CAD_DRAWINGS_SLOT, snapshot::CadPaneId, snapshot::CadWorkingScene, snapshot::cad_empty_pane_child, snapshot::cad_pane_model, snapshot::cad_pane_model_slot, snapshot::cad_scene_pane_geometry, snapshot::cad_scene_pane_objects};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_cad_cad_model::geometry::*;
use semio_framework_plugin::{ArtifactSerializer, MeshData};
use semio_framework_3d_brep_engine::{Brep, BrepKernel};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::encode_obj;
use semio_s_artifact_stdio_semio_mesh_export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
fn mesh_to_obj_text(mesh: &MeshData) -> Option<String> {
    let semio_mesh = semio_mesh_snapshot_from_mesh_data(mesh)?;
    let obj_snapshot = ::semio_framework_async::poll::resolve_ready(SemioMeshToObj::serialize(&semio_mesh)).ok()?;
    Some(encode_obj(&obj_snapshot))
}
pub fn cad_object_from_mesh(kernel: &mut Brep, id: impl Into<String>, label: impl Into<String>, typology: impl Into<String>, mesh: &MeshData) -> CadObject {
    let extent = mesh_extent(mesh);
    let solid_handle = mesh_to_obj_text(mesh).and_then(|text| kernel.import_obj(&text, 0.01).ok()).map(|handle| handle.0);
    let primitives = solid_handle.clone().map(|primitive_id| vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id, kind: "solid".into() }]).unwrap_or_default();
    CadObject { id: id.into(), label: label.into(), typology: typology.into(), visible: true, locked: false, origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, mesh_url: None, extent, solid_handle, primitives }
}

use semio_s_artifact_cad_cad_model::{snapshot::CadPaneId, snapshot::CadWorkingScene};
use semio_s_artifact_cad_cad_inferences::cad_brep_kernel;
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
    pub fn forest_pane_bundle(pane: CadPaneId) -> (Vec<CadObject>, CadGeometry) {
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
pub fn concrete_forest_left_shape_solid_face_loops() -> Result<semio_framework_3d_brep_engine::SolidFaceLoops, String> {
    use semio_framework_3d_brep_engine::{Brep, BrepError, GeometryHandle};
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

//#region 🔖️Domain
//#region 🔖️WorkingScene
/// 📚️ The immutable catalogue of every pane genesis scene the plugin ships, keyed by the pane child's stable `child_id`
/// ([`cad_named_pane_child`]): the bundled examples' panes and the four empty panes ([`cad_empty_pane_child`]). A hit is
/// the scene the document was born with: its objects are the child's genesis content and its raw geometry is what
/// `SemioModelSnapshot` has no field for. This resolves shipped assets; it never shares one live document's state with
/// another.
pub fn cad_bundled_pane_scene(child_id: &str) -> Option<std::sync::Arc<CadWorkingScene>> {
    static CATALOGUE: std::sync::OnceLock<Vec<(String, std::sync::Arc<CadWorkingScene>)>> = std::sync::OnceLock::new();
    CATALOGUE
        .get_or_init(|| {
            let forest = semio_s_artifact_cad_cad_inferences::forest_play_scene();
            let empty = std::sync::Arc::new(CadWorkingScene::default());
            let bundled = CadPaneId::all().into_iter().filter_map(|pane| Some((cad_pane_model(&forest, pane)?.child_id.clone(), crate::forest_pane_scene(pane))));
            bundled.chain(CadPaneId::all().into_iter().map(|pane| (cad_empty_pane_child(pane).child_id, empty.clone()))).collect()
        })
        .iter()
        .find(|(id, _)| id == child_id)
        .map(|(_, scene)| scene.clone())
}
/// 🌱️ `ArtifactEditor`/`ArtifactViewer::genesis_child_pack`: the pack an owned child the snapshot declares is born with
/// when no store carries it (design §20.15, §22.36). A pane model child whose stable id names a catalogue scene
/// ([`cad_bundled_pane_scene`]) is born with that scene's objects; every other declared child — the one a
/// `create-*-model` or `create-drawing` leaf hands out — is born EMPTY, because a handle is two strings: content reaches
/// a child only as an archive member or as child-lane leaves, never through its name. So a parent decoded from pack or
/// text with no archive members (the React shell's `loadDocumentPair` sends `members: []`) composes every declared
/// child, and a fresh instance plus a creation leaf is a loadable document. A child the snapshot does not declare has
/// no genesis.
pub fn cad_genesis_child_pack(snapshot: &CadSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot == "breps" {
        return snapshot.breps.iter().any(|child| child.child_id == child_id).then(|| <semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot as ArtifactPack>::encode_pack(&Default::default()));
    }
    if slot == CAD_DRAWINGS_SLOT {
        return snapshot.drawings.iter().any(|child| child.child_id == child_id).then(|| <SemioDrawingSnapshot as ArtifactPack>::encode_pack(&SemioDrawingSnapshot::default()));
    }
    let pane = CadPaneId::all().into_iter().find(|pane| cad_pane_model_slot(*pane) == slot)?;
    cad_pane_model(snapshot, pane).filter(|child| child.child_id == child_id)?;
    let genesis = cad_bundled_pane_scene(child_id);
    let objects = genesis.as_deref().map_or(&[][..], |scene| cad_scene_pane_objects(scene, pane));
    Some(<SemioModelSnapshot as ArtifactPack>::encode_pack(&semio_s_artifact_cad_cad_model::geometry::semio_model_snapshot_from_objects(objects)))
}
/// 🪆️ One pane as every reader sees it: its objects read out of the pane's composed `s.stdio.semio@v1/model` child, over
/// the immutable genesis scene its stable child id names ([`cad_bundled_pane_scene`]) — the raw geometry the model has no
/// field for, and the `Arc` whose identity keys the pane's tessellation cache.
#[derive(Clone, Debug, Default)]
pub(crate) struct CadComposedPane {
    pub(crate) objects: Vec<semio_s_artifact_cad_cad_model::geometry::CadObject>,
    pub(crate) owned_geometry: Option<semio_s_artifact_cad_cad_model::geometry::CadGeometry>,
    pub(crate) genesis: Option<std::sync::Arc<CadWorkingScene>>,
}
impl CadComposedPane {
    /// 📐️ The pane's raw genesis geometry, when its child id names a bundled scene.
    pub(crate) fn geometry(&self, pane: CadPaneId) -> Option<&semio_s_artifact_cad_cad_model::geometry::CadGeometry> {
        self.owned_geometry.as_ref().or_else(|| self.genesis.as_deref().and_then(|scene| cad_scene_pane_geometry(scene, pane)))
    }
}
/// 🪆️ The four panes every reader works on (design §20.15): each pane's composed child read through the child view only.
/// The child is the pane's one source of truth, so an edit, an undo, a remote change or a reload is seen the moment its
/// child store has it; a pane whose child is not composed yet reads the genesis content its stable id names, and a pane
/// without a handle has no objects.
#[derive(Clone, Debug, Default)]
pub struct CadComposedPanes([CadComposedPane; 4]);
impl CadComposedPanes {
    /// 🪆️ Composes `snapshot`'s pane handles with their live child content.
    pub fn compose(snapshot: &CadSnapshot, children: &semio_framework_plugin::ChildContentView) -> Self {
        use semio_s_artifact_cad_cad_model::geometry::objects_from_model_snapshot;
        Self(CadPaneId::all().map(|pane| {
            let Some(handle) = cad_pane_model(snapshot, pane) else { return CadComposedPane::default() };
            let slot = cad_pane_model_slot(pane);
            let genesis = cad_bundled_pane_scene(&handle.child_id);
            let modelled = children.dialect(slot, &handle.child_id).is_some_and(|dialect| dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "model");
            let objects = match modelled.then(|| children.typed_read::<SemioModelSnapshot>(slot, &handle.child_id).ok()).flatten() {
                Some(content) => objects_from_model_snapshot(&content),
                None => genesis.as_deref().map(|scene| cad_scene_pane_objects(scene, pane).to_vec()).unwrap_or_default(),
            };
            let mut geometry = genesis.as_deref().and_then(|scene| cad_scene_pane_geometry(scene, pane)).cloned().unwrap_or_default();
            for child in snapshot.breps.iter().filter(|child| objects.iter().any(|object| object.solid_handle.as_ref() == Some(&child.child_id))) {
                geometry.owned_meshes.insert(child.child_id.clone(), std::sync::Arc::new(semio_framework_plugin::MeshData::default()));
                if let Ok(content) = children.typed_read::<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>("breps", &child.child_id) {
                    geometry.owned_breps.insert(child.child_id.clone(), std::sync::Arc::new((*content).clone()));
                    if let Ok(mesh) = semio_s_artifact_cad_cad_model::geometry::mesh_from_owned_brep(&content) {
                        geometry.owned_meshes.insert(child.child_id.clone(), std::sync::Arc::new(mesh));
                    }
                }
            }
            let owned_geometry = (!geometry.owned_meshes.is_empty()).then_some(geometry);
            CadComposedPane { objects, genesis, owned_geometry }
        }))
    }

    /// 🔎️ `pane` as composed.
    pub(crate) fn pane(&self, pane: CadPaneId) -> &CadComposedPane {
        &self.0[pane.index()]
    }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Domain
