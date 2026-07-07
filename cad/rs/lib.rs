//! 📐 CAD scene document + typed VCS on `vcs`.

use std::collections::HashMap;
use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";
pub const CAD_PLAY_DOCUMENT_SCHEMA: &str = "cad.document";

//#region 🔖Domain
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadPrimitiveSlot {
    pub slot: String,
    pub primitive_id: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(default)]
    pub primitives: Vec<CadPrimitiveSlot>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadScene {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    pub camera: CadCamera,
    #[serde(default)]
    pub objects: Vec<CadObject>,
    #[serde(default)]
    pub building_objects: Vec<CadObject>,
    #[serde(default)]
    pub energy_objects: Vec<CadObject>,
    #[serde(default)]
    pub structure_classic_objects: Vec<CadObject>,
    #[serde(default)]
    pub references_by_model_definition_id: HashMap<String, Vec<CadReference>>,
    #[serde(default)]
    pub nodes: Vec<CadNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tool: Option<String>,
    #[serde(default = "default_model_definition_id")]
    pub active_model_definition_id: String,
}

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

pub type CadEnvelope = DocumentVcsEnvelope<CadScene, CadOp>;
pub type CadStore = DocumentVcsStore<CadScene, CadOp>;

pub fn empty_cad_projection() -> CadScene {
    CadScene {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        camera: CadCamera::default(),
        objects: Vec::new(),
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        references_by_model_definition_id: HashMap::new(),
        nodes: Vec::new(),
        active_tool: Some("selectDirect".into()),
        active_model_definition_id: default_model_definition_id(),
    }
}

pub fn cad_pane_objects<'a>(scene: &'a CadScene, pane: CadPaneId) -> &'a [CadObject] {
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
    for pane in CadPaneId::all() {
        if cad_pane_objects(scene, pane).iter().any(|object| object.id == object_id) {
            return Some(pane);
        }
    }
    None
}

pub fn cad_all_objects(scene: &CadScene) -> impl Iterator<Item = (&CadObject, CadPaneId)> {
    CadPaneId::all()
        .into_iter()
        .flat_map(|pane| cad_pane_objects(scene, pane).iter().map(move |object| (object, pane)))
}

pub fn cad_pane_from_model_definition_id(model_definition_id: &str) -> Option<CadPaneId> {
    CadPaneId::all()
        .into_iter()
        .find(|pane| pane.model_definition_id() == model_definition_id)
}
//#endregion 🔖Domain

//#region 🔖Ops
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum CadOp {
    AddObject {
        pane: CadPaneId,
        object: CadObject,
    },
    RemoveObject {
        pane: CadPaneId,
        object_id: String,
    },
    PatchObject {
        pane: CadPaneId,
        object_id: String,
        patch: CadObjectPatch,
    },
    TranslateObjects {
        object_ids: Vec<String>,
        dx: f64,
        dy: f64,
        dz: f64,
    },
    RotateObjects {
        object_ids: Vec<String>,
        ax: f64,
        ay: f64,
        az: f64,
        angle: f64,
    },
    ScaleObjects {
        object_ids: Vec<String>,
        sx: f64,
        sy: f64,
        sz: f64,
    },
    SetPaneObjects {
        pane: CadPaneId,
        objects: Vec<CadObject>,
    },
    AddNode {
        node: CadNode,
    },
    RemoveNode {
        node_id: String,
    },
    RenameNode {
        node_id: String,
        label: String,
    },
    PatchReference {
        model_definition_id: String,
        reference_id: String,
        patch: CadReferencePatch,
    },
    SetReferences {
        model_definition_id: String,
        references: Vec<CadReference>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDiff {
    pub objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub building_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub energy_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub structure_classic_objects: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    pub references_by_model_definition_id: Option<HashMap<String, Vec<CadReference>>>,
    pub nodes: Option<CollectionDiff<String, CadNodePatch, CadNode>>,
    pub active_model_definition_id: Option<String>,
}

fn apply_object_collection_diff(
    objects: &mut Vec<CadObject>,
    diff: &CollectionDiff<String, CadObjectPatch, CadObject>,
) {
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
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
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
                next.references_by_model_definition_id
                    .insert(model_definition_id.clone(), rows.clone());
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
        next
    }

    fn absorb(&mut self, other: Self) {
        absorb_object_diff(&mut self.objects, other.objects);
        absorb_object_diff(&mut self.building_objects, other.building_objects);
        absorb_object_diff(&mut self.energy_objects, other.energy_objects);
        absorb_object_diff(&mut self.structure_classic_objects, other.structure_classic_objects);
        if let Some(references) = other.references_by_model_definition_id {
            let target = self.references_by_model_definition_id.get_or_insert_with(HashMap::new);
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
    }
}

fn absorb_object_diff(
    target: &mut Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
    incoming: Option<CollectionDiff<String, CadObjectPatch, CadObject>>,
) {
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

impl Operation<CadScene> for CadOp {
    type Diff = CadDiff;

    fn diff(&self, projection: &CadScene) -> CadDiff {
        match self {
            CadOp::AddObject { pane, object } => CadDiff {
                objects: pane_collection_diff_for_add(*pane, object),
                building_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Building, object),
                energy_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Energy, object),
                structure_classic_objects: pane_collection_diff_for_add_if(
                    *pane,
                    CadPaneId::StructureClassic,
                    object,
                ),
                ..Default::default()
            },
            CadOp::RemoveObject { pane, object_id } => CadDiff {
                objects: pane_collection_diff_for_remove(*pane, CadPaneId::Shape, object_id),
                building_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Building, object_id),
                energy_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Energy, object_id),
                structure_classic_objects: pane_collection_diff_for_remove(
                    *pane,
                    CadPaneId::StructureClassic,
                    object_id,
                ),
                ..Default::default()
            },
            CadOp::PatchObject {
                pane,
                object_id,
                patch,
            } => CadDiff {
                objects: pane_collection_diff_for_patch(*pane, CadPaneId::Shape, object_id, patch),
                building_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Building, object_id, patch),
                energy_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Energy, object_id, patch),
                structure_classic_objects: pane_collection_diff_for_patch(
                    *pane,
                    CadPaneId::StructureClassic,
                    object_id,
                    patch,
                ),
                ..Default::default()
            },
            CadOp::TranslateObjects {
                object_ids,
                dx,
                dy,
                dz,
            } => transform_objects_diff(projection, object_ids, |object| CadObjectPatch {
                origin: Some([
                    object.origin[0] + dx,
                    object.origin[1] + dy,
                    object.origin[2] + dz,
                ]),
                ..Default::default()
            }),
            CadOp::RotateObjects {
                object_ids,
                ax,
                ay,
                az,
                angle,
            } => {
                let delta = quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_objects_diff(projection, object_ids, |object| {
                    let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    CadObjectPatch {
                        orientation: Some(quat_mul(delta, current)),
                        ..Default::default()
                    }
                })
            }
            CadOp::ScaleObjects {
                object_ids,
                sx,
                sy,
                sz,
            } => transform_objects_diff(projection, object_ids, |object| {
                let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
                CadObjectPatch {
                    scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]),
                    ..Default::default()
                }
            }),
            CadOp::SetPaneObjects { pane, objects } => {
                let mut diff = CadDiff::default();
                let removed: Vec<String> = cad_pane_objects(projection, *pane)
                    .iter()
                    .map(|object| object.id.clone())
                    .collect();
                let collection = CollectionDiff {
                    removed,
                    modified: Vec::new(),
                    added: objects.clone(),
                };
                set_pane_collection_diff(&mut diff, *pane, collection);
                diff
            }
            CadOp::AddNode { node } => CadDiff {
                nodes: Some(CollectionDiff {
                    added: vec![node.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            CadOp::RemoveNode { node_id } => CadDiff {
                nodes: Some(CollectionDiff {
                    removed: vec![node_id.clone()],
                    ..Default::default()
                }),
                ..Default::default()
            },
            CadOp::RenameNode { node_id, label } => CadDiff {
                nodes: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: node_id.clone(),
                        patch: CadNodePatch {
                            label: Some(label.clone()),
                        },
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            CadOp::PatchReference {
                model_definition_id,
                reference_id,
                patch,
            } => {
                let references = projection
                    .references_by_model_definition_id
                    .get(model_definition_id)
                    .cloned()
                    .unwrap_or_default();
                let next = references
                    .into_iter()
                    .map(|mut reference| {
                        if reference.id == *reference_id {
                            apply_reference_patch(&mut reference, patch);
                        }
                        reference
                    })
                    .collect();
                CadDiff {
                    references_by_model_definition_id: Some(HashMap::from([(
                        model_definition_id.clone(),
                        next,
                    )])),
                    ..Default::default()
                }
            }
            CadOp::SetReferences {
                model_definition_id,
                references,
            } => CadDiff {
                references_by_model_definition_id: Some(HashMap::from([(
                    model_definition_id.clone(),
                    references.clone(),
                )])),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &CadScene) -> Vec<Self> {
        match self {
            CadOp::AddObject { pane, object } => vec![CadOp::RemoveObject {
                pane: *pane,
                object_id: object.id.clone(),
            }],
            CadOp::RemoveObject { pane, object_id } => cad_pane_objects(projection, *pane)
                .iter()
                .find(|object| object.id == *object_id)
                .map(|object| {
                    vec![CadOp::AddObject {
                        pane: *pane,
                        object: object.clone(),
                    }]
                })
                .unwrap_or_default(),
            CadOp::PatchObject {
                pane,
                object_id,
                patch,
            } => cad_pane_objects(projection, *pane)
                .iter()
                .find(|object| object.id == *object_id)
                .map(|before| {
                    vec![CadOp::PatchObject {
                        pane: *pane,
                        object_id: object_id.clone(),
                        patch: reverse_object_patch(before, patch),
                    }]
                })
                .unwrap_or_default(),
            CadOp::TranslateObjects {
                object_ids,
                dx,
                dy,
                dz,
            } => vec![CadOp::TranslateObjects {
                object_ids: object_ids.clone(),
                dx: -dx,
                dy: -dy,
                dz: -dz,
            }],
            CadOp::RotateObjects {
                object_ids,
                ax,
                ay,
                az,
                angle,
            } => vec![CadOp::RotateObjects {
                object_ids: object_ids.clone(),
                ax: *ax,
                ay: *ay,
                az: *az,
                angle: -angle,
            }],
            CadOp::ScaleObjects {
                object_ids,
                sx,
                sy,
                sz,
            } => {
                let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
                vec![CadOp::ScaleObjects {
                    object_ids: object_ids.clone(),
                    sx: inv(*sx),
                    sy: inv(*sy),
                    sz: inv(*sz),
                }]
            }
            CadOp::SetPaneObjects { pane, objects } => {
                let before = cad_pane_objects(projection, *pane)
                    .iter()
                    .cloned()
                    .collect();
                vec![CadOp::SetPaneObjects {
                    pane: *pane,
                    objects: before,
                }]
            }
            CadOp::AddNode { node } => vec![CadOp::RemoveNode {
                node_id: node.id.clone(),
            }],
            CadOp::RemoveNode { node_id } => projection
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| vec![CadOp::AddNode { node: node.clone() }])
                .unwrap_or_default(),
            CadOp::RenameNode { node_id, .. } => projection
                .nodes
                .iter()
                .find(|node| node.id == *node_id)
                .map(|node| {
                    vec![CadOp::RenameNode {
                        node_id: node_id.clone(),
                        label: node.label.clone(),
                    }]
                })
                .unwrap_or_default(),
            CadOp::PatchReference {
                model_definition_id,
                reference_id,
                patch,
            } => projection
                .references_by_model_definition_id
                .get(model_definition_id)
                .and_then(|references| {
                    references
                        .iter()
                        .find(|reference| reference.id == *reference_id)
                        .map(|before| {
                            vec![CadOp::PatchReference {
                                model_definition_id: model_definition_id.clone(),
                                reference_id: reference_id.clone(),
                                patch: reverse_reference_patch(before, patch),
                            }]
                        })
                })
                .unwrap_or_default(),
            CadOp::SetReferences {
                model_definition_id,
                ..
            } => {
                let before = projection
                    .references_by_model_definition_id
                    .get(model_definition_id)
                    .cloned()
                    .unwrap_or_default();
                vec![CadOp::SetReferences {
                    model_definition_id: model_definition_id.clone(),
                    references: before,
                }]
            }
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
        extent: patch.extent.map(|_| before.extent).flatten(),
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
        opacity: patch.opacity.map(|_| before.opacity).flatten(),
    }
}

fn pane_collection_diff_for_add(
    pane: CadPaneId,
    object: &CadObject,
) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    pane_collection_diff_for_add_if(pane, CadPaneId::Shape, object)
}

fn pane_collection_diff_for_add_if(
    pane: CadPaneId,
    target: CadPaneId,
    object: &CadObject,
) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff {
            added: vec![object.clone()],
            ..Default::default()
        })
    } else {
        None
    }
}

fn pane_collection_diff_for_remove(
    pane: CadPaneId,
    target: CadPaneId,
    object_id: &str,
) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff {
            removed: vec![object_id.into()],
            ..Default::default()
        })
    } else {
        None
    }
}

fn pane_collection_diff_for_patch(
    pane: CadPaneId,
    target: CadPaneId,
    object_id: &str,
    patch: &CadObjectPatch,
) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff {
            modified: vec![ItemPatch {
                id: object_id.into(),
                patch: patch.clone(),
            }],
            ..Default::default()
        })
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

fn transform_objects_diff(
    projection: &CadScene,
    object_ids: &[String],
    patch_for: impl Fn(&CadObject) -> CadObjectPatch,
) -> CadDiff {
    let mut diff = CadDiff::default();
    for pane in CadPaneId::all() {
        let mut modified = Vec::new();
        for object in cad_pane_objects(projection, pane) {
            if !object_ids.contains(&object.id) {
                continue;
            }
            modified.push(ItemPatch {
                id: object.id.clone(),
                patch: patch_for(object),
            });
        }
        if !modified.is_empty() {
            set_pane_collection_diff(
                &mut diff,
                pane,
                CollectionDiff {
                    modified,
                    ..Default::default()
                },
            );
        }
    }
    diff
}
//#endregion 🔖Ops

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
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
                    let envelope: CadEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    CadStore::new(envelope)
                }
                None => CadStore::new(create_document_vcs_envelope(
                    CAD_DOCUMENT_SCHEMA,
                    "cad",
                    empty_cad_projection(),
                    None,
                )),
            };
            Ok(Self {
                store: RefCell::new(store),
            })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_vcs_envelope(
            CAD_DOCUMENT_SCHEMA,
            "cad",
            empty_cad_projection(),
            None,
        ));
        assert_eq!(store.projection().expect("projection").id, "cad");
    }

    #[test]
    fn add_object_round_trips_through_store() {
        let mut store = CadStore::new(create_document_vcs_envelope(
            CAD_DOCUMENT_SCHEMA,
            "cad",
            empty_cad_projection(),
            None,
        ));
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
            primitives: vec![CadPrimitiveSlot {
                slot: "solid".into(),
                primitive_id: "solid-1".into(),
                kind: "solid".into(),
            }],
        };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![CadOp::AddObject {
                    pane: CadPaneId::Shape,
                    object,
                }],
                description: None,
            })
            .expect("apply");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].primitives[0].kind, "solid");
    }

    #[test]
    fn translate_objects_updates_origin() {
        let mut store = CadStore::new(create_document_vcs_envelope(
            CAD_DOCUMENT_SCHEMA,
            "cad",
            empty_cad_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![CadOp::AddObject {
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
                        primitives: Vec::new(),
                    },
                }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![CadOp::TranslateObjects {
                    object_ids: vec!["object-1".into()],
                    dx: 1.0,
                    dy: -1.0,
                    dz: 0.5,
                }],
                description: None,
            })
            .expect("translate");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects[0].origin, [2.0, 1.0, 3.5]);
    }
}
//#endregion 🧪Tests
