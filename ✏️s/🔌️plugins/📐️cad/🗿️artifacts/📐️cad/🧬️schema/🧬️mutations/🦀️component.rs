//! 🧬️ CAD artifact — document mutation dispatch enum + shared patches/helpers.

use crate::artifacts::cad::diff::{apply_reference_patch, CadDiff, CadNodePatchEntry, CadNodesDelta, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::{cad_pane_objects, CadNode, CadObject, CadPaneId, CadReference, CadSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Mutations
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum CadMutation {
    AddObject {
        pane: CadPaneId,
        #[dsl(block)]
        object: CadObject,
    },
    RemoveObject {
        pane: CadPaneId,
        object_id: String,
    },
    PatchObject {
        pane: CadPaneId,
        object_id: String,
        #[dsl(block)]
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
        #[dsl(block)]
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
        #[dsl(block)]
        patch: CadReferencePatch,
    },
    SetReferences {
        model_definition_id: String,
        references: Vec<CadReference>,
    },
    SetActiveModelDefinition {
        model_definition_id: String,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: Box<CadSnapshot>,
    },
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
impl Mutation<CadSnapshot> for CadMutation {
    type Diff = CadDiff;

    fn diff(&self, projection: &CadSnapshot) -> CadDiff {
        match self {
            CadMutation::AddObject { pane, object } => CadDiff {
                objects: pane_objects_delta_for_add(*pane, object),
                building_objects: pane_objects_delta_for_add_if(*pane, CadPaneId::Building, object),
                energy_objects: pane_objects_delta_for_add_if(*pane, CadPaneId::Energy, object),
                structure_classic_objects: pane_objects_delta_for_add_if(*pane, CadPaneId::StructureClassic, object),
                ..Default::default()
            },
            CadMutation::RemoveObject { pane, object_id } => CadDiff {
                objects: pane_objects_delta_for_remove(*pane, CadPaneId::Shape, object_id),
                building_objects: pane_objects_delta_for_remove(*pane, CadPaneId::Building, object_id),
                energy_objects: pane_objects_delta_for_remove(*pane, CadPaneId::Energy, object_id),
                structure_classic_objects: pane_objects_delta_for_remove(*pane, CadPaneId::StructureClassic, object_id),
                ..Default::default()
            },
            CadMutation::PatchObject { pane, object_id, patch } => CadDiff {
                objects: pane_objects_delta_for_patch(*pane, CadPaneId::Shape, object_id, patch),
                building_objects: pane_objects_delta_for_patch(*pane, CadPaneId::Building, object_id, patch),
                energy_objects: pane_objects_delta_for_patch(*pane, CadPaneId::Energy, object_id, patch),
                structure_classic_objects: pane_objects_delta_for_patch(*pane, CadPaneId::StructureClassic, object_id, patch),
                ..Default::default()
            },
            CadMutation::TranslateObjects { object_ids, dx, dy, dz } => {
                transform_objects_diff(projection, object_ids, |object| CadObjectPatch { origin: Some([object.origin[0] + dx, object.origin[1] + dy, object.origin[2] + dz]), ..Default::default() })
            }
            CadMutation::RotateObjects { object_ids, ax, ay, az, angle } => {
                let delta = quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_objects_diff(projection, object_ids, |object| {
                    let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    CadObjectPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() }
                })
            }
            CadMutation::ScaleObjects { object_ids, sx, sy, sz } => transform_objects_diff(projection, object_ids, |object| {
                let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
                CadObjectPatch { scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]), ..Default::default() }
            }),
            CadMutation::SetPaneObjects { pane, objects } => {
                let mut diff = CadDiff::default();
                let removed: Vec<String> = cad_pane_objects(projection, *pane).iter().map(|object| object.id.clone()).collect();
                let delta = CadObjectsDelta { removed, added: objects.clone(), ..Default::default() };
                set_pane_objects_delta(&mut diff, *pane, delta);
                diff
            }
            CadMutation::AddNode { node } => CadDiff { nodes: Some(CadNodesDelta { added: vec![node.clone()], ..Default::default() }), ..Default::default() },
            CadMutation::RemoveNode { node_id } => CadDiff { nodes: Some(CadNodesDelta { removed: vec![node_id.clone()], ..Default::default() }), ..Default::default() },
            CadMutation::RenameNode { node_id, label } => CadDiff { nodes: Some(CadNodesDelta { patched: vec![CadNodePatchEntry { id: node_id.clone(), patch: CadNodePatch { label: Some(label.clone()) } }], ..Default::default() }), ..Default::default() },
            CadMutation::PatchReference { model_definition_id, reference_id, patch } => {
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
            CadMutation::SetReferences { model_definition_id, references } => CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(model_definition_id.clone(), references.clone())])), ..Default::default() },
            CadMutation::SetActiveModelDefinition { model_definition_id } => CadDiff { active_model_definition_id: Some(model_definition_id.clone()), ..Default::default() },
            CadMutation::SetSnapshot { snapshot } => CadDiff { artifact: Some(Box::new(crate::artifacts::cad::schema::CadArtifact::from_snapshot((**snapshot).clone()))), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &CadSnapshot) -> Vec<Self> {
        match self {
            CadMutation::AddObject { pane, object } => super::add_object::inverse::inverse(projection, *pane, object),
            CadMutation::RemoveObject { pane, object_id } => super::remove_object::inverse::inverse(projection, *pane, object_id),
            CadMutation::PatchObject { pane, object_id, patch } => super::patch_object::inverse::inverse(projection, *pane, object_id, patch),
            CadMutation::TranslateObjects { object_ids, dx, dy, dz } => super::translate_objects::inverse::inverse(projection, object_ids, *dx, *dy, *dz),
            CadMutation::RotateObjects { object_ids, ax, ay, az, angle } => super::rotate_objects::inverse::inverse(projection, object_ids, *ax, *ay, *az, *angle),
            CadMutation::ScaleObjects { object_ids, sx, sy, sz } => super::scale_objects::inverse::inverse(projection, object_ids, *sx, *sy, *sz),
            CadMutation::SetPaneObjects { pane, objects } => super::set_pane_objects::inverse::inverse(projection, *pane, objects),
            CadMutation::AddNode { node } => super::add_node::inverse::inverse(projection, node),
            CadMutation::RemoveNode { node_id } => super::remove_node::inverse::inverse(projection, node_id),
            CadMutation::RenameNode { node_id, label } => super::rename_node::inverse::inverse(projection, node_id, label),
            CadMutation::PatchReference { model_definition_id, reference_id, patch } => super::patch_reference::inverse::inverse(projection, model_definition_id, reference_id, patch),
            CadMutation::SetReferences { model_definition_id, references } => super::set_references::inverse::inverse(projection, model_definition_id, references),
            CadMutation::SetActiveModelDefinition { model_definition_id } => super::set_active_model_definition::inverse::inverse(projection, model_definition_id),
            CadMutation::SetSnapshot { snapshot } => super::set_snapshot::inverse::inverse(projection, snapshot),
        }
    }
}

pub fn reverse_object_patch(before: &CadObject, patch: &CadObjectPatch) -> CadObjectPatch {
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

pub fn reverse_reference_patch(before: &CadReference, patch: &CadReferencePatch) -> CadReferencePatch {
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

fn pane_objects_delta_for_add(pane: CadPaneId, object: &CadObject) -> Option<CadObjectsDelta> {
    pane_objects_delta_for_add_if(pane, CadPaneId::Shape, object)
}

fn pane_objects_delta_for_add_if(pane: CadPaneId, target: CadPaneId, object: &CadObject) -> Option<CadObjectsDelta> {
    if pane == target {
        Some(CadObjectsDelta { added: vec![object.clone()], ..Default::default() })
    } else {
        None
    }
}

fn pane_objects_delta_for_remove(pane: CadPaneId, target: CadPaneId, object_id: &str) -> Option<CadObjectsDelta> {
    if pane == target {
        Some(CadObjectsDelta { removed: vec![object_id.into()], ..Default::default() })
    } else {
        None
    }
}

fn pane_objects_delta_for_patch(pane: CadPaneId, target: CadPaneId, object_id: &str, patch: &CadObjectPatch) -> Option<CadObjectsDelta> {
    if pane == target {
        Some(CadObjectsDelta { patched: vec![CadObjectPatchEntry { id: object_id.into(), patch: patch.clone() }], ..Default::default() })
    } else {
        None
    }
}

fn set_pane_objects_delta(diff: &mut CadDiff, pane: CadPaneId, delta: CadObjectsDelta) {
    match pane {
        CadPaneId::Shape => diff.objects = Some(delta),
        CadPaneId::Building => diff.building_objects = Some(delta),
        CadPaneId::Energy => diff.energy_objects = Some(delta),
        CadPaneId::StructureClassic => diff.structure_classic_objects = Some(delta),
    }
}

fn transform_objects_diff(projection: &CadSnapshot, object_ids: &[String], patch_for: impl Fn(&CadObject) -> CadObjectPatch) -> CadDiff {
    let mut diff = CadDiff::default();
    for pane in CadPaneId::all() {
        let mut patched = Vec::new();
        for object in cad_pane_objects(projection, pane) {
            if !object_ids.contains(&object.id) {
                continue;
            }
            patched.push(CadObjectPatchEntry { id: object.id.clone(), patch: patch_for(object) });
        }
        if !patched.is_empty() {
            set_pane_objects_delta(&mut diff, pane, CadObjectsDelta { patched, ..Default::default() });
        }
    }
    diff
}
pub use super::add_object::mutation::{add_object, AddObject};
pub use super::remove_object::mutation::{remove_object, RemoveObject};
pub use super::patch_object::mutation::{patch_object, PatchObject};
pub use super::translate_objects::mutation::{translate_objects, TranslateObjects};
pub use super::rotate_objects::mutation::{rotate_objects, RotateObjects};
pub use super::scale_objects::mutation::{scale_objects, ScaleObjects};
pub use super::set_pane_objects::mutation::{set_pane_objects, SetPaneObjects};
pub use super::add_node::mutation::{add_node, AddNode};
pub use super::remove_node::mutation::{remove_node, RemoveNode};
pub use super::rename_node::mutation::{rename_node, RenameNode};
pub use super::patch_reference::mutation::{patch_reference, PatchReference};
pub use super::set_references::mutation::{set_references, SetReferences};
pub use super::set_active_model_definition::mutation::{set_active_model_definition, SetActiveModelDefinition};
pub use super::set_snapshot::mutation::{set_snapshot, SetSnapshot};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::artifacts::cad::testkit::{sample_object, sample_reference, sample_scene};

    /// ⚖️ One value per `CadMutation` variant — the closed set every wire law below iterates.
    pub fn every_mutation() -> Vec<CadMutation> {
        vec![
            CadMutation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-1") },
            CadMutation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() },
            CadMutation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), visible: Some(false), ..Default::default() } },
            CadMutation::TranslateObjects { object_ids: vec!["object-1".into(), "object-2".into()], dx: 1.0, dy: -1.0, dz: 0.5 },
            CadMutation::RotateObjects { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 },
            CadMutation::ScaleObjects { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 },
            CadMutation::SetPaneObjects { pane: CadPaneId::Energy, objects: vec![sample_object("object-1"), sample_object("object-2")] },
            CadMutation::AddNode { node: CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() } },
            CadMutation::RemoveNode { node_id: "node-1".into() },
            CadMutation::RenameNode { node_id: "node-1".into(), label: "Renamed".into() },
            CadMutation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch { hidden: Some(true), ..Default::default() } },
            CadMutation::SetReferences { model_definition_id: "spatial.shape".into(), references: vec![sample_reference()] },
            CadMutation::SetActiveModelDefinition { model_definition_id: "aec.building".into() },
            CadMutation::SetSnapshot { snapshot: Box::new(sample_scene()) },
        ]
    }

    #[test]
    fn inverse_inverts_every_variant_against_a_populated_scene() {
        let base = sample_scene();
        // ➕️ `AddObject`/`AddNode` are only invertible for ids the base scene does NOT already carry —
        // `every_mutation`'s rows deliberately reuse the sample ids to pin the wire format, so the
        // additive rows get fresh ids here.
        let operations = every_mutation().into_iter().map(|op| match op {
            CadMutation::AddObject { pane, mut object } => {
                object.id = "object-fresh".into();
                CadMutation::AddObject { pane, object }
            }
            CadMutation::AddNode { mut node } => {
                node.id = "node-fresh".into();
                CadMutation::AddNode { node }
            }
            other => other,
        });
        for op in operations {
            let forward = protocol::MutationDiff::apply(&op.diff(&base), &base);
            let mut restored = forward.clone();
            for inverse in op.inverse(&base) {
                restored = protocol::MutationDiff::apply(&inverse.diff(&restored), &restored);
            }
            assert_eq!(restored, base, "inverse must restore the base scene for {op:?}");
        }
    }
}
//#endregion 🧪️Tests
