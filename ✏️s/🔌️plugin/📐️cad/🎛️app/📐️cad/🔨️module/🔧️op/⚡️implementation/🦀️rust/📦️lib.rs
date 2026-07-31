//! ⚡️ Cad app — operation enum + laws (constitutional: op).

use cad_document::{cad_pane_objects, CadNode, CadObject, CadPaneId, CadReference, CadScene};
use protocol::{CollectionDiff, ItemPatch, Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Operations
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReferencePatch {
    pub source_url: Option<String>,
    pub media_kind: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<f64>,
    pub width_world: Option<f64>,
    pub hidden: Option<bool>,
    pub locked: Option<bool>,
    pub opacity: Option<f64>,
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
    if let Some(scale) = patch.scale {
        reference.scale = Some(scale);
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
        scale: patch.scale.map(|_| before.scale.unwrap_or(1.0)),
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
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cad_document::{empty_cad_projection, CadEdge, CadEdgeCurve, CadFace, CadGeometry, CadPlaneSurface, CadPrimitiveSlot, CadShell, CadSolid, CadVertex, CadWire};
    use serde_json::json;

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
            scale: Some(1.5),
            width_world: 8.0,
            hidden: false,
            locked: true,
            opacity: Some(0.8),
        }
    }

    fn sample_scene() -> CadScene {
        let mut scene = empty_cad_projection();
        scene.objects.push(sample_object("object-1"));
        scene.building_objects.push(sample_object("object-2"));
        scene.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        scene.shape_geometry = Some(sample_geometry());
        scene.references_by_model_definition_id.insert(CadPaneId::Shape.model_definition_id().to_string(), vec![sample_reference()]);
        scene.active_model_definition_id = CadPaneId::Shape.model_definition_id().to_string();
        scene
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
            CadOperation::SetScene { scene: Box::new(sample_scene()) },
        ];
        for op in ops {
            store::test_support::assert_op_line_round_trip(&op);
        }
    }
}
//#endregion 🧪️Tests
