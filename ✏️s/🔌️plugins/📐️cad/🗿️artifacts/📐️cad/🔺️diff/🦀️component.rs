//! 🔺️ CAD artifact — the `OperationDiff` half of the op/diff pair: the materialized `CadDiff` shape
//! `CadOperation::diff` produces and the `apply`/`absorb` laws the store folds it onto a `CadProjection`
//! with.

use crate::artifacts::cad::op::{CadNodePatch, CadObjectPatch, CadReferencePatch};
use crate::artifacts::cad::{CadNode, CadObject, CadReference, CadProjection};
use protocol::{CollectionDiff, OperationDiff};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
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
    pub scene: Option<Box<CadProjection>>,
}

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

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

pub fn apply_object_patch(object: &mut CadObject, patch: &CadObjectPatch) {
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

pub fn apply_reference_patch(reference: &mut CadReference, patch: &CadReferencePatch) {
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

impl OperationDiff<CadProjection> for CadDiff {
    fn apply(&self, projection: &CadProjection) -> CadProjection {
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
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::op::CadOperation;
    use crate::artifacts::cad::testkit::{sample_object, sample_scene};
    use crate::artifacts::cad::CadPaneId;
    use protocol::Operation;

    #[test]
    fn whole_scene_diff_replaces_the_projection_and_absorbs_every_earlier_edit() {
        let base = sample_scene();
        let mut diff = CadOperation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() }.diff(&base);
        let replacement = CadOperation::SetScene { scene: Box::new(base.clone()) }.diff(&base);
        diff.absorb(replacement);
        assert_eq!(diff.apply(&base), base, "a whole-scene diff wins over anything absorbed before it");
    }

    #[test]
    fn object_collection_diffs_absorb_into_one_apply() {
        let base = sample_scene();
        let mut diff = CadOperation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-9") }.diff(&base);
        diff.absorb(CadOperation::PatchObject { pane: CadPaneId::Shape, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), ..Default::default() } }.diff(&base));
        let next = diff.apply(&base);
        assert!(next.objects.iter().any(|object| object.id == "object-9"));
        assert_eq!(next.objects.iter().find(|object| object.id == "object-1").expect("object-1").label, "Renamed");
    }
}
//#endregion 🧪️Tests
