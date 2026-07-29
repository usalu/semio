//! 📐 CAD scene document + typed VCS on `vcs`.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use protocol::{CollectionDiff, ItemPatch, Operation, OperationDiff};
use store::{DocumentEnvelope, DocumentStore};

pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";
pub const CAD_PLAY_DOCUMENT_SCHEMA: &str = "cad.document";

//#region 🔖Domain
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "kebab-case")]
pub enum CadPaneId {
    Shape,
    Building,
    Energy,
    StructureClassic,
}

impl CadPaneId {
    pub fn model_definition_id(self) -> &'static str {
        match self {
            Self::Shape => "spatial.shape",
            Self::Building => "aec.building",
            Self::Energy => "aec.building.energy",
            Self::StructureClassic => "aec.building.structure.classic",
        }
    }

    pub fn all() -> [Self; 4] {
        [Self::Shape, Self::Building, Self::Energy, Self::StructureClassic]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadPrimitiveSlot {
    pub slot: String,
    pub primitive_id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadVertex {
    pub id: String,
    pub position: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadEdgeCurve {
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadEdge {
    pub id: String,
    pub vertex_ids: Vec<String>,
    #[dsl(block)]
    pub curve: CadEdgeCurve,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadWire {
    pub id: String,
    pub edge_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadPlaneSurface {
    pub kind: String,
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadFace {
    pub id: String,
    pub wire_ids: Vec<String>,
    #[dsl(block)]
    pub surface: CadPlaneSurface,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadShell {
    pub id: String,
    pub face_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSolid {
    pub id: String,
    pub shell_ids: Vec<String>,
}

/// @emoji 🧱 Authored brep topology carried alongside spatial model objects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadGeometry {
    #[serde(default)]
    pub anchors: Vec<Value>,
    #[serde(default)]
    pub vertices: Vec<CadVertex>,
    #[serde(default)]
    pub edges: Vec<CadEdge>,
    #[serde(default)]
    pub wires: Vec<CadWire>,
    #[serde(default)]
    pub faces: Vec<CadFace>,
    #[serde(default)]
    pub shells: Vec<CadShell>,
    #[serde(default)]
    pub solids: Vec<CadSolid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadObject {
    pub id: String,
    pub label: String,
    pub typology: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<[f64; 3]>,
    #[serde(default, rename = "meshUrl")]
    pub mesh_url: Option<String>,
    #[serde(default)]
    pub extent: Option<[f64; 3]>,
    #[serde(default, rename = "solidHandle")]
    pub solid_handle: Option<String>,
    #[serde(default)]
    pub primitives: Vec<CadPrimitiveSlot>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReference {
    pub id: String,
    pub source_url: String,
    #[serde(default = "default_image_media_kind")]
    pub media_kind: String,
    #[serde(default)]
    pub origin: [f64; 3],
    #[serde(default)]
    pub orientation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<Value>,
    #[serde(default = "default_width_world")]
    pub width_world: f64,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub opacity: Option<f64>,
}

fn default_image_media_kind() -> String {
    "image".into()
}

fn default_width_world() -> f64 {
    10.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadCamera {
    #[serde(default = "default_camera_position")]
    pub position: [f64; 3],
    #[serde(default = "default_camera_target")]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default = "default_fov")]
    pub fov: f64,
    /// 📐 Serialized `semio_framework_program::WorldProjectionConfig` — kept as raw json here (cad/rs has no
    /// dependency on the program layer); `cad/program/rs` parses/writes it around the shared projection helpers.
    #[serde(default)]
    pub projection: Value,
}

impl Default for CadCamera {
    fn default() -> Self {
        Self { position: default_camera_position(), target: default_camera_target(), zoom: one_f64(), fov: default_fov(), projection: Value::Null }
    }
}

fn default_camera_position() -> [f64; 3] {
    [12.0, -12.0, 8.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "cad", layout = "lines")]
pub struct CadScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    #[dsl(block)]
    pub camera: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_building: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_energy: CadCamera,
    #[serde(default)]
    #[dsl(block)]
    pub camera_structure_classic: CadCamera,
    #[serde(default)]
    #[dsl(table)]
    pub objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub building_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub energy_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    pub structure_classic_objects: Vec<CadObject>,
    #[serde(default)]
    pub references_by_model_definition_id: BTreeMap<String, Vec<CadReference>>,
    #[serde(default)]
    #[dsl(table)]
    pub nodes: Vec<CadNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub shape_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub building_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub energy_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub structure_classic_geometry: Option<CadGeometry>,
    #[serde(default = "default_model_definition_id")]
    pub active_model_definition_id: String,
}

pub fn cad_pane_geometry(scene: &CadScene, pane: CadPaneId) -> Option<&CadGeometry> {
    match pane {
        CadPaneId::Shape => scene.shape_geometry.as_ref(),
        CadPaneId::Building => scene.building_geometry.as_ref(),
        CadPaneId::Energy => scene.energy_geometry.as_ref(),
        CadPaneId::StructureClassic => scene.structure_classic_geometry.as_ref(),
    }
}

pub fn cad_pane_geometry_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut Option<CadGeometry> {
    match pane {
        CadPaneId::Shape => &mut scene.shape_geometry,
        CadPaneId::Building => &mut scene.building_geometry,
        CadPaneId::Energy => &mut scene.energy_geometry,
        CadPaneId::StructureClassic => &mut scene.structure_classic_geometry,
    }
}

pub fn cad_pane_camera(scene: &CadScene, pane: CadPaneId) -> &CadCamera {
    match pane {
        CadPaneId::Shape => &scene.camera,
        CadPaneId::Building => &scene.camera_building,
        CadPaneId::Energy => &scene.camera_energy,
        CadPaneId::StructureClassic => &scene.camera_structure_classic,
    }
}

pub fn cad_pane_camera_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut CadCamera {
    match pane {
        CadPaneId::Shape => &mut scene.camera,
        CadPaneId::Building => &mut scene.camera_building,
        CadPaneId::Energy => &mut scene.camera_energy,
        CadPaneId::StructureClassic => &mut scene.camera_structure_classic,
    }
}

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

pub type CadEnvelope = DocumentEnvelope<CadScene, CadOperation>;
pub type CadStore = DocumentStore<CadScene, CadOperation>;

pub fn empty_cad_projection() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        camera: CadCamera::default(),
        camera_building: CadCamera::default(),
        camera_energy: CadCamera::default(),
        camera_structure_classic: CadCamera::default(),
        objects: Vec::new(),
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        references_by_model_definition_id: BTreeMap::new(),
        nodes: Vec::new(),
        shape_geometry: None,
        building_geometry: None,
        energy_geometry: None,
        structure_classic_geometry: None,
        active_model_definition_id: default_model_definition_id(),
    }
}

pub fn cad_pane_objects(scene: &CadScene, pane: CadPaneId) -> &[CadObject] {
    match pane {
        CadPaneId::Shape => &scene.objects,
        CadPaneId::Building => &scene.building_objects,
        CadPaneId::Energy => &scene.energy_objects,
        CadPaneId::StructureClassic => &scene.structure_classic_objects,
    }
}

pub fn cad_pane_objects_mut(scene: &mut CadScene, pane: CadPaneId) -> &mut Vec<CadObject> {
    match pane {
        CadPaneId::Shape => &mut scene.objects,
        CadPaneId::Building => &mut scene.building_objects,
        CadPaneId::Energy => &mut scene.energy_objects,
        CadPaneId::StructureClassic => &mut scene.structure_classic_objects,
    }
}

pub fn cad_find_object_pane(scene: &CadScene, object_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|&pane| cad_pane_objects(scene, pane).iter().any(|object| object.id == object_id))
}

pub fn cad_all_objects(scene: &CadScene) -> impl Iterator<Item = (&CadObject, CadPaneId)> {
    CadPaneId::all().into_iter().flat_map(|pane| cad_pane_objects(scene, pane).iter().map(move |object| (object, pane)))
}

pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all().into_iter().find(|pane| pane.model_definition_id() == model_definition_id)
}
//#endregion 🔖Domain

//#region 🔖Operations
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadObjectPatch {
    pub label: Option<String>,
    pub typology: Option<String>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
    #[serde(rename = "meshUrl")]
    pub mesh_url: Option<String>,
    pub extent: Option<[f64; 3]>,
    #[serde(rename = "solidHandle")]
    pub solid_handle: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

/// 🎥 A per-pane camera assignment carried by `CadOperation::SetCamera`/`CadDiff` — pairs the target pane
/// with its new camera so viewpoint moves flow through the store like any other document operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadCameraSet {
    pub pane: CadPaneId,
    pub camera: CadCamera,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReferencePatch {
    pub source_url: Option<String>,
    pub media_kind: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<Value>,
    pub width_world: Option<f64>,
    pub hidden: Option<bool>,
    pub locked: Option<bool>,
    pub opacity: Option<f64>,
}

/// 🪆 `Box<CadScene>` needs its own `dsl::DslField` binding for `CadOperation::SetScene` — `Box` is
/// `#[fundamental]` in `std`, so implementing a foreign trait (`dsl::DslField`) for `Box<CadScene>`
/// (a local type inside the foreign, fundamental `Box` wrapper) is permitted by the orphan rules;
/// this delegates entirely to `CadScene`'s own derive-generated `DslField` impl (from `DslDocument`).
impl dsl::DslField for Box<CadScene> {
    fn shape() -> dsl::Shape {
        <CadScene as dsl::DslField>::shape()
    }
    fn to_value(&self) -> dsl::FieldValue {
        <CadScene as dsl::DslField>::to_value(self)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        <CadScene as dsl::DslField>::from_value(value).map(Box::new)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum CadOperation {
    AddObject { pane: CadPaneId, #[dsl(block)] object: CadObject },
    RemoveObject { pane: CadPaneId, object_id: String },
    PatchObject { pane: CadPaneId, object_id: String, #[dsl(block)] patch: CadObjectPatch },
    TranslateObjects { object_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    RotateObjects { object_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    ScaleObjects { object_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    SetPaneObjects { pane: CadPaneId, objects: Vec<CadObject> },
    AddNode { #[dsl(block)] node: CadNode },
    RemoveNode { node_id: String },
    RenameNode { node_id: String, label: String },
    PatchReference { model_definition_id: String, reference_id: String, #[dsl(block)] patch: CadReferencePatch },
    SetReferences { model_definition_id: String, references: Vec<CadReference> },
    SetActiveModelDefinition { model_definition_id: String },
    SetCamera { pane: CadPaneId, #[dsl(block)] camera: CadCamera },
    SetScene { #[dsl(block)] scene: Box<CadScene> },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDiff {
    pub objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub building_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub energy_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub structure_classic_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub references_by_model_definition_id: Option<BTreeMap<String, Vec<CadReference>>>,
    pub nodes: Option<CollectionDiff<String, CadNodePatch, CadNode>>,
    pub active_model_definition_id: Option<String>,
    pub camera: Option<CadCameraSet>,
    pub scene: Option<Box<CadScene>>,
}

fn apply_object_collection_diff(objects: &mut Vec<CadObject>, diff: &CollectionDiff<String, CadObjectPatch, CadObject>) {
    for id in &diff.removed {
        objects.retain(|object| object.id != *id);
    }
    for patch in &diff.modified {
        for object in objects.iter_mut() {
            if object.id != patch.id {
                continue;
            }
            apply_object_patch(object, &patch.patch);
        }
    }
    for added in &diff.added {
        objects.push(added.clone());
    }
}

fn apply_object_patch(object: &mut CadObject, patch: &CadObjectPatch) {
    if let Some(label) = &patch.label {
        object.label = label.clone();
    }
    if let Some(typology) = &patch.typology {
        object.typology = typology.clone();
    }
    if let Some(visible) = patch.visible {
        object.visible = visible;
    }
    if let Some(locked) = patch.locked {
        object.locked = locked;
    }
    if let Some(origin) = patch.origin {
        object.origin = origin;
    }
    if let Some(orientation) = patch.orientation {
        object.orientation = Some(orientation);
    }
    if let Some(scale) = patch.scale {
        object.scale = Some(scale);
    }
    if let Some(mesh_url) = &patch.mesh_url {
        object.mesh_url = Some(mesh_url.clone());
    }
    if let Some(extent) = patch.extent {
        object.extent = Some(extent);
    }
    if let Some(solid_handle) = &patch.solid_handle {
        object.solid_handle = Some(solid_handle.clone());
    }
}

fn apply_reference_patch(reference: &mut CadReference, patch: &CadReferencePatch) {
    if let Some(source_url) = &patch.source_url {
        reference.source_url = source_url.clone();
    }
    if let Some(media_kind) = &patch.media_kind {
        reference.media_kind = media_kind.clone();
    }
    if let Some(origin) = patch.origin {
        reference.origin = origin;
    }
    if let Some(orientation) = patch.orientation {
        reference.orientation = Some(orientation);
    }
    if let Some(scale) = &patch.scale {
        reference.scale = Some(scale.clone());
    }
    if let Some(width_world) = patch.width_world {
        reference.width_world = width_world;
    }
    if let Some(hidden) = patch.hidden {
        reference.hidden = hidden;
    }
    if let Some(locked) = patch.locked {
        reference.locked = locked;
    }
    if let Some(opacity) = patch.opacity {
        reference.opacity = Some(opacity);
    }
}

fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

impl OperationDiff<CadScene> for CadDiff {
    fn apply(&self, projection: &CadScene) -> CadScene {
        if let Some(scene) = &self.scene {
            return (**scene).clone();
        }
        let mut next = projection.clone();
        if let Some(objects) = &self.objects {
            apply_object_collection_diff(&mut next.objects, objects);
        }
        if let Some(objects) = &self.building_objects {
            apply_object_collection_diff(&mut next.building_objects, objects);
        }
        if let Some(objects) = &self.energy_objects {
            apply_object_collection_diff(&mut next.energy_objects, objects);
        }
        if let Some(objects) = &self.structure_classic_objects {
            apply_object_collection_diff(&mut next.structure_classic_objects, objects);
        }
        if let Some(references) = &self.references_by_model_definition_id {
            for (model_definition_id, rows) in references {
                next.references_by_model_definition_id.insert(model_definition_id.clone(), rows.clone());
            }
        }
        if let Some(nodes) = &self.nodes {
            for id in &nodes.removed {
                next.nodes.retain(|node| node.id != *id);
            }
            for patch in &nodes.modified {
                for node in &mut next.nodes {
                    if node.id == patch.id {
                        if let Some(label) = &patch.patch.label {
                            node.label = label.clone();
                        }
                    }
                }
            }
            for added in &nodes.added {
                next.nodes.push(added.clone());
            }
        }
        if let Some(active_model_definition_id) = &self.active_model_definition_id {
            next.active_model_definition_id = active_model_definition_id.clone();
        }
        if let Some(set) = &self.camera {
            *cad_pane_camera_mut(&mut next, set.pane) = set.camera.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.scene.is_some() {
            self.scene = other.scene;
            return;
        }
        absorb_object_diff(&mut self.objects, other.objects);
        absorb_object_diff(&mut self.building_objects, other.building_objects);
        absorb_object_diff(&mut self.energy_objects, other.energy_objects);
        absorb_object_diff(&mut self.structure_classic_objects, other.structure_classic_objects);
        if let Some(references) = other.references_by_model_definition_id {
            let target = self.references_by_model_definition_id.get_or_insert_with(BTreeMap::new);
            target.extend(references);
        }
        match (&mut self.nodes, other.nodes) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.nodes = Some(b),
            _ => {}
        }
        if other.active_model_definition_id.is_some() {
            self.active_model_definition_id = other.active_model_definition_id;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

fn absorb_object_diff(target: &mut Option<CollectionDiff<String, CadObjectPatch, CadObject>>, incoming: Option<CollectionDiff<String, CadObjectPatch, CadObject>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}

impl Operation<CadScene> for CadOperation {
    type Diff = CadDiff;

    fn diff(&self, projection: &CadScene) -> CadDiff {
        match self {
            CadOperation::AddObject { pane, object } => CadDiff {
                objects: pane_collection_diff_for_add(*pane, object),
                building_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Building, object),
                energy_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Energy, object),
                structure_classic_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::StructureClassic, object),
                ..Default::default()
            },
            CadOperation::RemoveObject { pane, object_id } => CadDiff {
                objects: pane_collection_diff_for_remove(*pane, CadPaneId::Shape, object_id),
                building_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Building, object_id),
                energy_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Energy, object_id),
                structure_classic_objects: pane_collection_diff_for_remove(*pane, CadPaneId::StructureClassic, object_id),
                ..Default::default()
            },
            CadOperation::PatchObject { pane, object_id, patch } => CadDiff {
                objects: pane_collection_diff_for_patch(*pane, CadPaneId::Shape, object_id, patch),
                building_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Building, object_id, patch),
                energy_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Energy, object_id, patch),
                structure_classic_objects: pane_collection_diff_for_patch(*pane, CadPaneId::StructureClassic, object_id, patch),
                ..Default::default()
            },
            CadOperation::TranslateObjects { object_ids, dx, dy, dz } => transform_objects_diff(projection, object_ids, |object| CadObjectPatch { origin: Some([object.origin[0] + dx, object.origin[1] + dy, object.origin[2] + dz]), ..Default::default() }),
            CadOperation::RotateObjects { object_ids, ax, ay, az, angle } => {
                let delta = quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_objects_diff(projection, object_ids, |object| {
                    let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    CadObjectPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() }
                })
            }
            CadOperation::ScaleObjects { object_ids, sx, sy, sz } => transform_objects_diff(projection, object_ids, |object| {
                let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
                CadObjectPatch { scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]), ..Default::default() }
            }),
            CadOperation::SetPaneObjects { pane, objects } => {
                let mut diff = CadDiff::default();
                let removed: Vec<String> = cad_pane_objects(projection, *pane).iter().map(|object| object.id.clone()).collect();
                let collection = CollectionDiff { removed, modified: Vec::new(), added: objects.clone() };
                set_pane_collection_diff(&mut diff, *pane, collection);
                diff
            }
            CadOperation::AddNode { node } => CadDiff { nodes: Some(CollectionDiff { added: vec![node.clone()], ..Default::default() }), ..Default::default() },
            CadOperation::RemoveNode { node_id } => CadDiff { nodes: Some(CollectionDiff { removed: vec![node_id.clone()], ..Default::default() }), ..Default::default() },
            CadOperation::RenameNode { node_id, label } => CadDiff { nodes: Some(CollectionDiff { modified: vec![ItemPatch { id: node_id.clone(), patch: CadNodePatch { label: Some(label.clone()) } }], ..Default::default() }), ..Default::default() },
            CadOperation::PatchReference { model_definition_id, reference_id, patch } => {
                let references = projection.references_by_model_definition_id.get(model_definition_id).cloned().unwrap_or_default();
                let next = references
                    .into_iter()
                    .map(|mut reference| {
                        if reference.id == *reference_id {
                            apply_reference_patch(&mut reference, patch);
                        }
                        reference
                    })
                    .collect();
                CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(model_definition_id.clone(), next)])), ..Default::default() }
            }
            CadOperation::SetReferences { model_definition_id, references } => CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(model_definition_id.clone(), references.clone())])), ..Default::default() },
            CadOperation::SetActiveModelDefinition { model_definition_id } => CadDiff { active_model_definition_id: Some(model_definition_id.clone()), ..Default::default() },
            CadOperation::SetCamera { pane, camera } => CadDiff { camera: Some(CadCameraSet { pane: *pane, camera: camera.clone() }), ..Default::default() },
            CadOperation::SetScene { scene } => CadDiff { scene: Some(scene.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &CadScene) -> Vec<Self> {
        match self {
            CadOperation::AddObject { pane, object } => vec![CadOperation::RemoveObject { pane: *pane, object_id: object.id.clone() }],
            CadOperation::RemoveObject { pane, object_id } => cad_pane_objects(projection, *pane).iter().find(|object| object.id == *object_id).map(|object| vec![CadOperation::AddObject { pane: *pane, object: object.clone() }]).unwrap_or_default(),
            CadOperation::PatchObject { pane, object_id, patch } => {
                cad_pane_objects(projection, *pane).iter().find(|object| object.id == *object_id).map(|before| vec![CadOperation::PatchObject { pane: *pane, object_id: object_id.clone(), patch: reverse_object_patch(before, patch) }]).unwrap_or_default()
            }
            CadOperation::TranslateObjects { object_ids, dx, dy, dz } => vec![CadOperation::TranslateObjects { object_ids: object_ids.clone(), dx: -dx, dy: -dy, dz: -dz }],
            CadOperation::RotateObjects { object_ids, ax, ay, az, angle } => vec![CadOperation::RotateObjects { object_ids: object_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: -angle }],
            CadOperation::ScaleObjects { object_ids, sx, sy, sz } => {
                let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
                vec![CadOperation::ScaleObjects { object_ids: object_ids.clone(), sx: inv(*sx), sy: inv(*sy), sz: inv(*sz) }]
            }
            CadOperation::SetPaneObjects { pane, objects: _ } => {
                let before = cad_pane_objects(projection, *pane).to_vec();
                vec![CadOperation::SetPaneObjects { pane: *pane, objects: before }]
            }
            CadOperation::AddNode { node } => vec![CadOperation::RemoveNode { node_id: node.id.clone() }],
            CadOperation::RemoveNode { node_id } => projection.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadOperation::AddNode { node: node.clone() }]).unwrap_or_default(),
            CadOperation::RenameNode { node_id, .. } => projection.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadOperation::RenameNode { node_id: node_id.clone(), label: node.label.clone() }]).unwrap_or_default(),
            CadOperation::PatchReference { model_definition_id, reference_id, patch } => projection
                .references_by_model_definition_id
                .get(model_definition_id)
                .and_then(|references| {
                    references
                        .iter()
                        .find(|reference| reference.id == *reference_id)
                        .map(|before| vec![CadOperation::PatchReference { model_definition_id: model_definition_id.clone(), reference_id: reference_id.clone(), patch: reverse_reference_patch(before, patch) }])
                })
                .unwrap_or_default(),
            CadOperation::SetReferences { model_definition_id, .. } => {
                let before = projection.references_by_model_definition_id.get(model_definition_id).cloned().unwrap_or_default();
                vec![CadOperation::SetReferences { model_definition_id: model_definition_id.clone(), references: before }]
            }
            CadOperation::SetActiveModelDefinition { .. } => vec![CadOperation::SetActiveModelDefinition { model_definition_id: projection.active_model_definition_id.clone() }],
            CadOperation::SetCamera { pane, .. } => vec![CadOperation::SetCamera { pane: *pane, camera: cad_pane_camera(projection, *pane).clone() }],
            CadOperation::SetScene { .. } => vec![CadOperation::SetScene { scene: Box::new(projection.clone()) }],
        }
    }
}

fn reverse_object_patch(before: &CadObject, patch: &CadObjectPatch) -> CadObjectPatch {
    CadObjectPatch {
        label: patch.label.as_ref().map(|_| before.label.clone()),
        typology: patch.typology.as_ref().map(|_| before.typology.clone()),
        visible: patch.visible.map(|_| before.visible),
        locked: patch.locked.map(|_| before.locked),
        origin: patch.origin.map(|_| before.origin),
        orientation: patch.orientation.map(|_| before.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
        scale: patch.scale.map(|_| before.scale.unwrap_or([1.0, 1.0, 1.0])),
        mesh_url: patch.mesh_url.as_ref().map(|_| before.mesh_url.clone().unwrap_or_default()),
        extent: patch.extent.and(before.extent),
        solid_handle: patch.solid_handle.as_ref().and_then(|_| before.solid_handle.clone()),
    }
}

fn reverse_reference_patch(before: &CadReference, patch: &CadReferencePatch) -> CadReferencePatch {
    CadReferencePatch {
        source_url: patch.source_url.as_ref().map(|_| before.source_url.clone()),
        media_kind: patch.media_kind.as_ref().map(|_| before.media_kind.clone()),
        origin: patch.origin.map(|_| before.origin),
        orientation: patch.orientation.map(|_| before.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
        scale: patch.scale.as_ref().map(|_| before.scale.clone().unwrap_or(Value::Null)),
        width_world: patch.width_world.map(|_| before.width_world),
        hidden: patch.hidden.map(|_| before.hidden),
        locked: patch.locked.map(|_| before.locked),
        opacity: patch.opacity.and(before.opacity),
    }
}

fn pane_collection_diff_for_add(pane: CadPaneId, object: &CadObject) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    pane_collection_diff_for_add_if(pane, CadPaneId::Shape, object)
}

fn pane_collection_diff_for_add_if(pane: CadPaneId, target: CadPaneId, object: &CadObject) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { added: vec![object.clone()], ..Default::default() })
    } else {
        None
    }
}

fn pane_collection_diff_for_remove(pane: CadPaneId, target: CadPaneId, object_id: &str) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { removed: vec![object_id.into()], ..Default::default() })
    } else {
        None
    }
}

fn pane_collection_diff_for_patch(pane: CadPaneId, target: CadPaneId, object_id: &str, patch: &CadObjectPatch) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { modified: vec![ItemPatch { id: object_id.into(), patch: patch.clone() }], ..Default::default() })
    } else {
        None
    }
}

fn set_pane_collection_diff(diff: &mut CadDiff, pane: CadPaneId, collection: CollectionDiff<String, CadObjectPatch, CadObject>) {
    match pane {
        CadPaneId::Shape => diff.objects = Some(collection),
        CadPaneId::Building => diff.building_objects = Some(collection),
        CadPaneId::Energy => diff.energy_objects = Some(collection),
        CadPaneId::StructureClassic => diff.structure_classic_objects = Some(collection),
    }
}

fn transform_objects_diff(projection: &CadScene, object_ids: &[String], patch_for: impl Fn(&CadObject) -> CadObjectPatch) -> CadDiff {
    let mut diff = CadDiff::default();
    for pane in CadPaneId::all() {
        let mut modified = Vec::new();
        for object in cad_pane_objects(projection, pane) {
            if !object_ids.contains(&object.id) {
                continue;
            }
            modified.push(ItemPatch { id: object.id.clone(), patch: patch_for(object) });
        }
        if !modified.is_empty() {
            set_pane_collection_diff(&mut diff, pane, CollectionDiff { modified, ..Default::default() });
        }
    }
    diff
}
//#endregion 🔖Operations

//#region 🔖InteractionSpec
// 📐 Declarative interaction machine types mirroring `cad/schema/json/interaction.json` and
// `cad/schema/json/expression.json` — parsed from the JSON interaction assets embedded by the
// cad program and interpreted generically at runtime (states/transitions/guards/effects/display),
// replacing hand-written per-interaction Rust statecharts.

/// A path root within an expression/effect target — `context` (session context), `event` (the
/// event payload being handled), or `params` (an enclosing action's parameters; unused by the
/// interaction machine interpreter itself, only by `spatial.action` step specs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExprPathRoot {
    Context,
    Event,
    Params,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ExprPathSegment {
    Field { name: String },
    Index { index: usize },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExprPathTarget {
    pub root: ExprPathRoot,
    #[serde(default)]
    pub segments: Vec<ExprPathSegment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExprBinding {
    pub name: String,
    pub value: Box<Expr>,
}

/// `spatial://schema/json/expression` — a small declarative expression AST. Only the kinds
/// actually used by the interaction machine specs' guards/effects/display are interpreted here
/// (`kernel.call`/`distance`/`fold` appear only in `spatial.action` step specs, which are not
/// executed generically — see the commit-action runner in `cad/program/rs/interaction.rs`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Expr {
    Path {
        root: ExprPathRoot,
        #[serde(default)]
        segments: Vec<ExprPathSegment>,
    },
    Const {
        value: Value,
    },
    Var {
        name: String,
    },
    Let {
        bindings: Vec<ExprBinding>,
        #[serde(rename = "in")]
        body: Box<Expr>,
    },
    Exists {
        target: ExprPathTarget,
    },
    NotEmpty {
        target: ExprPathTarget,
    },
    All {
        args: Vec<Expr>,
    },
    Any {
        args: Vec<Expr>,
    },
    Not {
        arg: Box<Expr>,
    },
    Abs {
        arg: Box<Expr>,
    },
    Distance {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    #[serde(rename = "kernel.call")]
    KernelCall {
        function: String,
        #[serde(default)]
        args: std::collections::HashMap<String, Expr>,
    },
    Binop {
        operation: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fold {
        operation: String,
        args: Vec<Expr>,
    },
}

/// Evaluation environment for {@link Expr}: `context` is the engagement session's persistent
/// state, `event` is the payload of the event currently being handled (if any).
pub struct ExprEnv<'a> {
    pub context: &'a std::collections::HashMap<String, Value>,
    pub event: Option<&'a Value>,
}

fn expr_path_get(root_value: Option<&Value>, segments: &[ExprPathSegment]) -> Option<Value> {
    let mut current = root_value?.clone();
    for segment in segments {
        current = match segment {
            ExprPathSegment::Field { name } => current.get(name)?.clone(),
            ExprPathSegment::Index { index } => current.get(index)?.clone(),
        };
    }
    Some(current)
}

fn expr_value_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|v| v != 0.0).unwrap_or(false),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

fn expr_value_not_empty(value: Option<&Value>) -> bool {
    match value {
        None => false,
        Some(Value::Null) => false,
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

fn expr_as_f64(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

/// Evaluates an {@link Expr} against `env` and an outer `let`-binding scope (`vars`).
pub fn evaluate_expr(expr: &Expr, env: &ExprEnv, vars: &std::collections::HashMap<String, Value>) -> Value {
    match expr {
        Expr::Path { root, segments } => {
            let root_value = match root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            expr_path_get(root_value.as_ref(), segments).unwrap_or(Value::Null)
        }
        Expr::Const { value } => value.clone(),
        Expr::Var { name } => vars.get(name).cloned().unwrap_or(Value::Null),
        Expr::Let { bindings, body } => {
            let mut scope = vars.clone();
            for binding in bindings {
                let value = evaluate_expr(&binding.value, env, &scope);
                scope.insert(binding.name.clone(), value);
            }
            evaluate_expr(body, env, &scope)
        }
        Expr::Exists { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            Value::Bool(expr_path_get(root_value.as_ref(), &target.segments).is_some())
        }
        Expr::NotEmpty { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(serde_json::to_value(env.context).unwrap_or(Value::Null)),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            Value::Bool(expr_value_not_empty(expr_path_get(root_value.as_ref(), &target.segments).as_ref()))
        }
        Expr::All { args } => Value::Bool(args.iter().all(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Any { args } => Value::Bool(args.iter().any(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Not { arg } => Value::Bool(!expr_value_truthy(&evaluate_expr(arg, env, vars))),
        Expr::Abs { arg } => json!(expr_as_f64(&evaluate_expr(arg, env, vars)).abs()),
        Expr::Distance { a, b } => {
            let av = evaluate_expr(a, env, vars);
            let bv = evaluate_expr(b, env, vars);
            let da: Option<[f64; 3]> = serde_json::from_value(av).ok();
            let db: Option<[f64; 3]> = serde_json::from_value(bv).ok();
            match (da, db) {
                (Some(a), Some(b)) => {
                    json!(((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt())
                }
                _ => Value::Null,
            }
        }
        // `kernel.call` expressions are only used inside `spatial.action` step specs (not executed
        // generically by this interpreter); evaluating one directly yields null.
        Expr::KernelCall { .. } => Value::Null,
        Expr::Binop { operation, left, right } => {
            let lv = evaluate_expr(left, env, vars);
            let rv = evaluate_expr(right, env, vars);
            match operation.as_str() {
                "==" => Value::Bool(lv == rv),
                "!=" => Value::Bool(lv != rv),
                ">" => Value::Bool(expr_as_f64(&lv) > expr_as_f64(&rv)),
                "<" => Value::Bool(expr_as_f64(&lv) < expr_as_f64(&rv)),
                ">=" => Value::Bool(expr_as_f64(&lv) >= expr_as_f64(&rv)),
                "<=" => Value::Bool(expr_as_f64(&lv) <= expr_as_f64(&rv)),
                "+" => json!(expr_as_f64(&lv) + expr_as_f64(&rv)),
                "-" => json!(expr_as_f64(&lv) - expr_as_f64(&rv)),
                "*" => json!(expr_as_f64(&lv) * expr_as_f64(&rv)),
                "/" => json!(expr_as_f64(&lv) / expr_as_f64(&rv)),
                _ => Value::Null,
            }
        }
        Expr::Fold { operation, args } => {
            let values: Vec<f64> = args.iter().map(|arg| expr_as_f64(&evaluate_expr(arg, env, vars))).collect();
            match operation.as_str() {
                "min" => values.into_iter().fold(f64::INFINITY, f64::min).into(),
                "max" => values.into_iter().fold(f64::NEG_INFINITY, f64::max).into(),
                _ => Value::Null,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Effect {
    Assign {
        target: ExprPathTarget,
        value: Expr,
    },
    Clear {
        target: ExprPathTarget,
    },
    Append {
        target: ExprPathTarget,
        value: Expr,
    },
    Emit {
        event: Value,
    },
    Raise {
        event: String,
    },
    OpenTransaction,
    CommitTransaction,
    RollbackTransaction,
    RequestPreview,
    #[serde(rename = "kernel.query")]
    KernelQuery {
        #[serde(default)]
        query: Option<String>,
        #[serde(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    ResolveEditable,
    SetDiagnostic {
        severity: String,
        code: String,
        message: String,
    },
    ClearDiagnostic {
        code: String,
    },
    Action {
        action: String,
        #[serde(default)]
        params: std::collections::HashMap<String, Expr>,
        #[serde(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    /// Asset-only extension (not in the formal schema): delegates to a nested sub-interaction
    /// (`interaction`), then maps each of its `outputs[].value` expressions (evaluated against
    /// the sub-interaction's context) onto `outputs[].target` in the parent context. Used only by
    /// the curve-drawing sub-flow (`mode.curve` in the wall/slab/column specs) — not yet
    /// interpreted (sub-interaction composition is a follow-up; the primary `mode.2points` flow
    /// does not depend on it).
    #[serde(rename = "interaction.call")]
    InteractionCall {
        interaction: String,
        #[serde(default)]
        outputs: Vec<InteractionCallOutput>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionCallOutput {
    pub target: ExprPathTarget,
    pub value: Expr,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionSpec {
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub guard: Option<String>,
    #[serde(default)]
    pub transient: bool,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventHandlerSpec {
    pub event: String,
    #[serde(default)]
    pub transitions: Vec<TransitionSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSpec {
    #[serde(default)]
    pub accept: Vec<String>,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub prompt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDefSpec {
    pub name: String,
    #[serde(default)]
    pub r#final: bool,
    #[serde(default)]
    pub selection: Option<SelectionSpec>,
    #[serde(default)]
    pub on: Vec<EventHandlerSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineSpec {
    pub initial: String,
    pub states: Vec<StateDefSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardSpec {
    pub name: String,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LengthEntrySpec {
    pub state: String,
    pub anchor: String,
    pub field: String,
    #[serde(default)]
    pub commit: Option<String>,
    #[serde(default)]
    pub control: Option<String>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarEntrySpec {
    pub state: String,
    pub event: String,
    pub field: String,
    #[serde(default)]
    pub commit: Option<String>,
    #[serde(default)]
    pub axis_anchor: Option<String>,
    #[serde(default)]
    pub axis_floor: Option<String>,
    #[serde(default)]
    pub axis: Option<[f64; 3]>,
    #[serde(default)]
    pub control: Option<String>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialInteractionConfig {
    #[serde(default)]
    pub spatial_ground_pick: bool,
    #[serde(default)]
    pub pick_disabled_states: Vec<String>,
    #[serde(default)]
    pub ground_pointer_move_states: Vec<String>,
    #[serde(default)]
    pub height_drag_states: Vec<String>,
    #[serde(default)]
    pub vertical_rod_states: Vec<String>,
    #[serde(default)]
    pub height_confirm_state: Option<String>,
    #[serde(default)]
    pub length_entry: Vec<LengthEntrySpec>,
    #[serde(default)]
    pub scalar_entry: Vec<ScalarEntrySpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DisplayItemSpec {
    Point {
        id: String,
        #[serde(default)]
        role: Option<String>,
        position: Expr,
    },
    Label {
        id: String,
        #[serde(default)]
        role: Option<String>,
        text: String,
        position: Expr,
    },
    Segment {
        id: String,
        #[serde(default)]
        role: Option<String>,
        from: Expr,
        to: Expr,
    },
    #[serde(rename = "linear-handle")]
    LinearHandle {
        id: String,
        #[serde(default)]
        role: Option<String>,
        axis: [f64; 3],
        origin: Expr,
    },
    #[serde(rename = "box-preview")]
    BoxPreview {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(rename = "cornerA")]
        corner_a: Expr,
        #[serde(rename = "cornerB")]
        corner_b: Expr,
        height: Expr,
    },
    #[serde(rename = "entity-highlight")]
    EntityHighlight {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(rename = "geometryEntityKind")]
        geometry_entity_kind: String,
        #[serde(rename = "entityId")]
        entity_id: Expr,
    },
    Curve {
        id: String,
        #[serde(default)]
        role: Option<String>,
    },
    Mesh {
        id: String,
        #[serde(default)]
        role: Option<String>,
    },
    /// Asset-only extension kind (`"preview"`) not in the formal schema: a generic wireframe
    /// preview keyed by `previewKind`, evaluated params passed through verbatim to the renderer.
    Preview {
        id: String,
        #[serde(default)]
        role: Option<String>,
        #[serde(default, rename = "previewKind")]
        preview_kind: Option<String>,
        #[serde(default)]
        params: std::collections::HashMap<String, Expr>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayStateSpec {
    pub state: String,
    pub items: Vec<DisplayItemSpec>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplaySpec {
    #[serde(default)]
    pub states: Vec<DisplayStateSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitSpec {
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub from_states: Vec<String>,
    pub operation: CommitOperationSpec,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitOperationSpec {
    pub action: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, Expr>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionProducesSpec {
    #[serde(default)]
    pub typology: Option<String>,
}

/// `spatial://schema/json/interaction` — the full declarative construction-interaction spec, as
/// authored in `cad/asset/modelDefinition/*/interaction/*.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionSpec {
    pub id: String,
    pub version: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub produces: InteractionProducesSpec,
    #[serde(default)]
    pub guards: Vec<GuardSpec>,
    pub machine: MachineSpec,
    #[serde(default)]
    pub display: DisplaySpec,
    #[serde(default)]
    pub interaction: SpatialInteractionConfig,
    pub commit: CommitSpec,
}

impl InteractionSpec {
    pub fn state<'a>(&'a self, name: &str) -> Option<&'a StateDefSpec> {
        self.machine.states.iter().find(|state| state.name == name)
    }

    pub fn guard(&self, name: &str, env: &ExprEnv) -> bool {
        self.guards.iter().find(|guard| guard.name == name).map(|guard| expr_value_truthy(&evaluate_expr(&guard.expr, env, &std::collections::HashMap::new()))).unwrap_or(false)
    }
}
//#endregion 🔖InteractionSpec

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct CadDocumentVcs {
        store: RefCell<CadStore>,
    }

    #[wasm_bindgen]
    impl CadDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<CadDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: CadEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    CadStore::new(envelope)
                }
                None => CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::{create_document_envelope, DocumentCommand};

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        assert_eq!(store.projection().expect("projection").id, "cad");
    }

    #[test]
    fn add_object_round_trips_through_store() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let object = CadObject {
            id: "object-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        };
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::AddObject { pane: CadPaneId::Shape, object }], description: None }).expect("apply");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].primitives[0].kind, "solid");
    }

    #[test]
    fn translate_objects_updates_origin() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![CadOperation::AddObject {
                    pane: CadPaneId::Shape,
                    object: CadObject {
                        id: "object-1".into(),
                        label: "Box".into(),
                        typology: "spatial.shape.box".into(),
                        visible: true,
                        locked: false,
                        origin: [1.0, 2.0, 3.0],
                        orientation: None,
                        scale: None,
                        mesh_url: None,
                        extent: None,
                        solid_handle: None,
                        primitives: Vec::new(),
                    },
                }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -1.0, dz: 0.5 }], description: None }).expect("translate");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects[0].origin, [2.0, 1.0, 3.5]);
    }

    #[test]
    fn set_scene_replaces_projection_and_inverts() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let mut replacement = empty_cad_projection();
        replacement.id = "replaced".into();
        replacement.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::SetScene { scene: Box::new(replacement) }], description: None }).expect("set scene");
        assert_eq!(store.projection().expect("projection").id, "replaced");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").id, "cad");
        assert!(store.projection().expect("projection").nodes.is_empty());
    }

    #[test]
    fn set_camera_flows_through_operations() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let camera = CadCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], zoom: 2.0, fov: 60.0, projection: Value::Null };
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::SetCamera { pane: CadPaneId::Building, camera: camera.clone() }], description: None }).expect("apply");
        let scene = store.projection().expect("projection");
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).zoom, 2.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).zoom, 1.0);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        let scene = store.projection().expect("projection");
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).zoom, 1.0);
    }

    #[test]
    fn pane_cameras_isolate_states() {
        let mut scene = empty_cad_projection();

        // Assert initial defaults
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 50.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 50.0);

        // Update Shape camera
        cad_pane_camera_mut(&mut scene, CadPaneId::Shape).fov = 40.0;

        // Verify isolation
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 40.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 50.0);

        // Update Building camera
        cad_pane_camera_mut(&mut scene, CadPaneId::Building).fov = 60.0;

        // Verify isolation
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Shape).fov, 40.0);
        assert_eq!(cad_pane_camera(&scene, CadPaneId::Building).fov, 60.0);
    }

    // --- 🧬 dsl:: engine adoption: `CadScene` (`store::DocumentDsl`, extension `cad`) and
    // `CadOperation` (`store::OpText`) text round trips ---

    fn sample_object(id: &str) -> CadObject {
        CadObject {
            id: id.into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [1.0, 2.0, 3.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: Some([1.0, 1.0, 1.0]),
            mesh_url: Some("https://example.test/mesh.glb".into()),
            extent: Some([2.0, 2.0, 2.0]),
            solid_handle: Some("solid-1".into()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        }
    }

    fn sample_geometry() -> CadGeometry {
        CadGeometry {
            anchors: vec![json!({ "id": "anchor-1", "position": [0.0, 0.0, 0.0] })],
            vertices: vec![CadVertex { id: "v1".into(), position: [0.0, 0.0, 0.0] }, CadVertex { id: "v2".into(), position: [1.0, 0.0, 0.0] }],
            edges: vec![CadEdge { id: "e1".into(), vertex_ids: vec!["v1".into(), "v2".into()], curve: CadEdgeCurve { kind: "line".into() } }],
            wires: vec![CadWire { id: "w1".into(), edge_ids: vec!["e1".into()] }],
            faces: vec![CadFace { id: "f1".into(), wire_ids: vec!["w1".into()], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }],
            shells: vec![CadShell { id: "s1".into(), face_ids: vec!["f1".into()] }],
            solids: vec![CadSolid { id: "sol1".into(), shell_ids: vec!["s1".into()] }],
        }
    }

    fn sample_reference() -> CadReference {
        CadReference {
            id: "ref-1".into(),
            source_url: "https://example.test/plan.png".into(),
            media_kind: "image".into(),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: Some(json!(1.5)),
            width_world: 8.0,
            hidden: false,
            locked: true,
            opacity: Some(0.8),
        }
    }

    fn sample_scene() -> CadScene {
        let mut scene = empty_cad_projection();
        scene.camera.zoom = 2.0;
        scene.objects.push(sample_object("object-1"));
        scene.building_objects.push(sample_object("object-2"));
        scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        scene.shape_geometry = Some(sample_geometry());
        scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
        scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
        scene
    }

    #[test]
    fn cad_scene_round_trips_through_dsl_document() {
        store::test_support::assert_dsl_round_trip(&sample_scene());
        store::test_support::assert_dsl_pack_equivalence(&sample_scene());
    }

    #[test]
    fn cad_scene_with_all_geometry_panes_round_trips_through_dsl_document() {
        let mut scene = sample_scene();
        scene.building_geometry = Some(sample_geometry());
        scene.energy_geometry = Some(sample_geometry());
        scene.structure_classic_geometry = Some(sample_geometry());
        store::test_support::assert_dsl_round_trip(&scene);
        store::test_support::assert_dsl_pack_equivalence(&scene);
    }

    #[test]
    fn cad_operation_print_op_round_trips_every_variant_as_one_line() {
        let ops = vec![
            CadOperation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-1") },
            CadOperation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() },
            CadOperation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), visible: Some(false), ..Default::default() } },
            CadOperation::TranslateObjects { object_ids: vec!["object-1".into(), "object-2".into()], dx: 1.0, dy: -1.0, dz: 0.5 },
            CadOperation::RotateObjects { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 },
            CadOperation::ScaleObjects { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 },
            CadOperation::SetPaneObjects { pane: CadPaneId::Energy, objects: vec![sample_object("object-1"), sample_object("object-2")] },
            CadOperation::AddNode { node: CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() } },
            CadOperation::RemoveNode { node_id: "node-1".into() },
            CadOperation::RenameNode { node_id: "node-1".into(), label: "Renamed".into() },
            CadOperation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch { hidden: Some(true), ..Default::default() } },
            CadOperation::SetReferences { model_definition_id: "spatial.shape".into(), references: vec![sample_reference()] },
            CadOperation::SetActiveModelDefinition { model_definition_id: "aec.building".into() },
            CadOperation::SetCamera { pane: CadPaneId::StructureClassic, camera: CadCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], zoom: 2.0, fov: 60.0, projection: Value::Null } },
            CadOperation::SetScene { scene: Box::new(sample_scene()) },
        ];
        for op in ops {
            store::test_support::assert_op_line_round_trip(&op);
        }
    }

    #[test]
    fn interaction_spec_parses_box_asset() {
        let raw = include_str!("../asset/modelDefinition/spatial.shape/interaction/box.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("box.json parses as InteractionSpec");
        assert_eq!(spec.id, "primitive.box");
        assert_eq!(spec.machine.initial, "idle");
        assert!(spec.state("first_corner").is_some());
        assert!(spec.state("ready").is_some());
        assert_eq!(spec.commit.operation.action, "primitive.createBoxFromCorners");
        assert!(spec.commit.operation.params.contains_key("cornerA"));
        assert!(spec.commit.operation.params.contains_key("cornerB"));
        assert!(spec.commit.operation.params.contains_key("height"));
        assert_eq!(spec.commit.from_states, vec!["ready".to_string()]);
    }

    #[test]
    fn interaction_spec_parses_sphere_asset_with_command_finish() {
        let raw = include_str!("../asset/modelDefinition/spatial.shape/interaction/sphere.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("sphere.json parses as InteractionSpec");
        assert_eq!(spec.id, "solid.sphere");
        assert_eq!(spec.commit.operation.action, "command.finish");
        assert!(spec.display.states.iter().any(|s| s.state == "radius"));
    }

    #[test]
    fn interaction_spec_parses_all_energy_and_structure_classic_assets() {
        let sources = [
            include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructBasePlate.json"),
            include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructExternalWall.json"),
            include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructHull.json"),
            include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructRoof.json"),
            include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructWindows.json"),
            include_str!("../asset/modelDefinition/aec.building.structure.classic/interaction/constructOneWayReinforcedConcreteSlab.json"),
            include_str!("../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteColumn.json"),
            include_str!("../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteExternalWall.json"),
            include_str!("../asset/modelDefinition/aec.building.structure.classic/interaction/constructReinforcedConcreteInternalWall.json"),
        ];
        for raw in sources {
            let spec: InteractionSpec = serde_json::from_str(raw).expect("asset parses as InteractionSpec");
            assert!(spec.commit.operation.action.ends_with("From2PointsAndHeight") || spec.commit.operation.action.ends_with("FromSurface"));
            assert!(spec.commit.operation.params.contains_key("pointA"));
            assert!(spec.commit.operation.params.contains_key("pointB"));
            assert!(spec.commit.operation.params.contains_key("height"));
            assert!(spec.commit.operation.params.contains_key("typology"));
        }
    }

    /// Regression guard: every `interaction/*.json` asset in the tree must parse as
    /// `InteractionSpec` — catches schema drift between the JSON assets and these Rust types.
    #[test]
    fn every_interaction_asset_on_disk_parses_as_interaction_spec() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../asset/modelDefinition");
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.file_name().and_then(|n| n.to_str()).map(|n| n.ends_with(".json")).unwrap_or(false) && path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()) == Some("interaction") {
                    out.push(path);
                }
            }
        }
        let mut files = Vec::new();
        walk(&root, &mut files);
        assert!(files.len() >= 40, "expected at least 40 interaction assets, found {}", files.len());
        let mut failures = Vec::new();
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("read asset");
            if let Err(err) = serde_json::from_str::<InteractionSpec>(&raw) {
                failures.push(format!("{}: {}", file.display(), err));
            }
        }
        assert!(failures.is_empty(), "{} interaction assets failed to parse:\n{}", failures.len(), failures.join("\n"));
    }

    #[test]
    fn evaluate_expr_supports_path_const_var_and_boolean_combinators() {
        let mut context = std::collections::HashMap::new();
        context.insert("height".to_string(), json!(2.5));
        context.insert("origin".to_string(), json!([0.0, 0.0, 0.0]));
        let env = ExprEnv { context: &context, event: None };
        let vars = std::collections::HashMap::new();

        let path_expr = Expr::Path { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "height".into() }] };
        assert_eq!(evaluate_expr(&path_expr, &env, &vars), json!(2.5));

        let exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "origin".into() }] } };
        assert_eq!(evaluate_expr(&exists_expr, &env, &vars), json!(true));

        let missing_exists_expr = Expr::Exists { target: ExprPathTarget { root: ExprPathRoot::Context, segments: vec![ExprPathSegment::Field { name: "missing".into() }] } };
        assert_eq!(evaluate_expr(&missing_exists_expr, &env, &vars), json!(false));

        let binop_expr = Expr::Binop { operation: ">".into(), left: Box::new(path_expr.clone()), right: Box::new(Expr::Const { value: json!(1.0) }) };
        assert_eq!(evaluate_expr(&binop_expr, &env, &vars), json!(true));

        let all_expr = Expr::All { args: vec![exists_expr, binop_expr] };
        assert_eq!(evaluate_expr(&all_expr, &env, &vars), json!(true));

        let let_expr = Expr::Let {
            bindings: vec![ExprBinding { name: "h".into(), value: Box::new(path_expr) }],
            body: Box::new(Expr::Binop { operation: "*".into(), left: Box::new(Expr::Var { name: "h".into() }), right: Box::new(Expr::Const { value: json!(2.0) }) }),
        };
        assert_eq!(evaluate_expr(&let_expr, &env, &vars), json!(5.0));
    }

    #[test]
    fn interaction_spec_guard_evaluates_against_context() {
        let raw = include_str!("../asset/modelDefinition/aec.building.energy/interaction/constructExternalWall.json");
        let spec: InteractionSpec = serde_json::from_str(raw).expect("parses");
        let mut context = std::collections::HashMap::new();
        let env_without = ExprEnv { context: &context, event: None };
        assert!(!spec.guard("hasConstructMode", &env_without));
        context.insert("constructMode".to_string(), json!("2PointsAndHeight"));
        let env_with = ExprEnv { context: &context, event: None };
        assert!(spec.guard("hasConstructMode", &env_with));
    }
}
//#endregion 🧪Tests
