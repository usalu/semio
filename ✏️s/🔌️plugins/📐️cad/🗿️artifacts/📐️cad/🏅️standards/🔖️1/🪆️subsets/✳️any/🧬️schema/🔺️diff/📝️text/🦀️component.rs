//! 🔺️ CAD artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::cad::diff::schema::{
    CadDiff, CadNodePatchEntry, CadNodesDelta, CadObjectPatchEntry, CadObjectsDelta, CadStringList,
};
use crate::artifacts::cad::mutations::{CadNodePatch, CadObjectPatch, CadReferencePatch};
use crate::artifacts::cad::schema::CadArtifact;
use crate::artifacts::cad::{CadNode, CadObject, CadReference, CadSnapshot};
use protocol::MutationDiff;
use std::collections::BTreeMap;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl CadDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &CadArtifact) -> CadArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(id) = &self.id { next.id = id.clone(); }
        if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta); }
        if let Some(delta) = &self.building_objects { next.building_objects = apply_objects_delta(&next.building_objects, delta); }
        if let Some(delta) = &self.energy_objects { next.energy_objects = apply_objects_delta(&next.energy_objects, delta); }
        if let Some(delta) = &self.structure_classic_objects { next.structure_classic_objects = apply_objects_delta(&next.structure_classic_objects, delta); }
        if let Some(references) = &self.references_by_model_definition_id {
            for (key, rows) in references {
                next.references_by_model_definition_id.insert(key.clone(), rows.clone());
            }
        }
        if let Some(delta) = &self.nodes { next.nodes = apply_nodes_delta(&next.nodes, delta); }
        if let Some(value) = &self.shape_geometry { next.shape_geometry = value.clone(); }
        if let Some(value) = &self.building_geometry { next.building_geometry = value.clone(); }
        if let Some(value) = &self.energy_geometry { next.energy_geometry = value.clone(); }
        if let Some(value) = &self.structure_classic_geometry { next.structure_classic_geometry = value.clone(); }
        if let Some(value) = &self.active_model_definition_id { next.active_model_definition_id = value.clone(); }
        if let Some(list) = &self.selected_object_ids { next.selected_object_ids = list.values.clone(); }
        if let Some(list) = &self.selected_node_ids { next.selected_node_ids = list.values.clone(); }
        if let Some(value) = &self.active_object_id { next.active_object_id = value.clone(); }
        if let Some(value) = &self.component_selection { next.component_selection = value.clone(); }
        if let Some(value) = &self.selected_reference_model_definition_id { next.selected_reference_model_definition_id = value.clone(); }
        if let Some(value) = &self.selected_reference_id { next.selected_reference_id = value.clone(); }
        if let Some(value) = &self.selected_primitive_id { next.selected_primitive_id = value.clone(); }
        if let Some(value) = &self.selected_primitive_kind { next.selected_primitive_kind = value.clone(); }
        if let Some(value) = &self.active_utility_id { next.active_utility_id = value.clone(); }
        if let Some(value) = &self.active_example_id { next.active_example_id = value.clone(); }
        if let Some(value) = &self.selection_method { next.selection_method = value.clone(); }
        if let Some(value) = &self.engagement_input { next.engagement_input = value.clone(); }
        if let Some(value) = &self.engagement_step { next.engagement_step = value.clone(); }
        if let Some(value) = &self.engagement_pane { next.engagement_pane = value.clone(); }
        if let Some(value) = &self.engagement_session_json { next.engagement_session_json = value.clone(); }
        if let Some(value) = &self.last_finalized_interaction_id { next.last_finalized_interaction_id = value.clone(); }
        if let Some(value) = self.sun_enabled { next.sun_enabled = value; }
        if let Some(value) = self.sun_azimuth { next.sun_azimuth = value; }
        if let Some(value) = self.sun_elevation { next.sun_elevation = value; }
        if let Some(value) = self.sun_intensity { next.sun_intensity = value; }
        if let Some(value) = &self.sun_color { next.sun_color = value.clone(); }
        if let Some(value) = &self.camera { next.camera = value.clone(); }
        if let Some(value) = &self.camera_building { next.camera_building = value.clone(); }
        if let Some(value) = &self.camera_energy { next.camera_energy = value.clone(); }
        if let Some(value) = &self.camera_structure_classic { next.camera_structure_classic = value.clone(); }
        if let Some(value) = self.dislocate_shape { next.dislocate_shape = value; }
        if let Some(value) = self.dislocate_building { next.dislocate_building = value; }
        if let Some(value) = self.dislocate_energy { next.dislocate_energy = value; }
        if let Some(value) = self.dislocate_structure_classic { next.dislocate_structure_classic = value; }
        if let Some(value) = &self.locale { next.locale = value.clone(); }
        if let Some(value) = &self.terminology { next.terminology = value.clone(); }
        if let Some(value) = &self.contributions_json { next.contributions_json = value.clone(); }
        if let Some(value) = &self.hovered_object_id { next.hovered_object_id = value.clone(); }
        if let Some(value) = &self.hovered_target_object_id { next.hovered_target_object_id = value.clone(); }
        if let Some(value) = &self.hovered_target_mode { next.hovered_target_mode = value.clone(); }
        if let Some(value) = &self.hovered_target_id { next.hovered_target_id = *value; }
        next
    }
}

/// 🧩 Applies an identified-collection delta to an object list.
pub fn apply_objects_delta(objects: &[CadObject], delta: &CadObjectsDelta) -> Vec<CadObject> {
    let mut next = objects.to_vec();
    for id in &delta.removed {
        next.retain(|object| &object.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(object) = next.iter_mut().find(|object| object.id == entry.id) {
            apply_object_patch(object, &entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|object| (object.id.clone(), object)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(object) = by_id.remove(id) {
                ordered.push(object);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn apply_nodes_delta(nodes: &[CadNode], delta: &CadNodesDelta) -> Vec<CadNode> {
    let mut next = nodes.to_vec();
    for id in &delta.removed {
        next.retain(|node| &node.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(node) = next.iter_mut().find(|node| node.id == entry.id) {
            if let Some(label) = &entry.patch.label {
                node.label = label.clone();
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|node| (node.id.clone(), node)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(node) = by_id.remove(id) {
                ordered.push(node);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

pub fn apply_object_patch(object: &mut CadObject, patch: &CadObjectPatch) {
    if let Some(label) = &patch.label { object.label = label.clone(); }
    if let Some(typology) = &patch.typology { object.typology = typology.clone(); }
    if let Some(visible) = patch.visible { object.visible = visible; }
    if let Some(locked) = patch.locked { object.locked = locked; }
    if let Some(origin) = patch.origin { object.origin = origin; }
    if let Some(orientation) = patch.orientation { object.orientation = Some(orientation); }
    if let Some(scale) = patch.scale { object.scale = Some(scale); }
    if let Some(mesh_url) = &patch.mesh_url { object.mesh_url = Some(mesh_url.clone()); }
    if let Some(extent) = patch.extent { object.extent = Some(extent); }
    if let Some(solid_handle) = &patch.solid_handle { object.solid_handle = Some(solid_handle.clone()); }
}

pub fn apply_reference_patch(reference: &mut CadReference, patch: &CadReferencePatch) {
    if let Some(source_url) = &patch.source_url { reference.source_url = source_url.clone(); }
    if let Some(media_kind) = &patch.media_kind { reference.media_kind = media_kind.clone(); }
    if let Some(origin) = patch.origin { reference.origin = origin; }
    if let Some(orientation) = patch.orientation { reference.orientation = Some(orientation); }
    if let Some(scale) = patch.scale { reference.scale = Some(scale); }
    if let Some(width_world) = patch.width_world { reference.width_world = width_world; }
    if let Some(hidden) = patch.hidden { reference.hidden = hidden; }
    if let Some(locked) = patch.locked { reference.locked = locked; }
    if let Some(opacity) = patch.opacity { reference.opacity = Some(opacity); }
}

impl MutationDiff<CadSnapshot> for CadDiff {
    fn apply(&self, snapshot: &CadSnapshot) -> CadSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(id) = &self.id { next.id = id.clone(); }
        if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta); }
        if let Some(delta) = &self.building_objects { next.building_objects = apply_objects_delta(&next.building_objects, delta); }
        if let Some(delta) = &self.energy_objects { next.energy_objects = apply_objects_delta(&next.energy_objects, delta); }
        if let Some(delta) = &self.structure_classic_objects { next.structure_classic_objects = apply_objects_delta(&next.structure_classic_objects, delta); }
        if let Some(references) = &self.references_by_model_definition_id {
            for (key, rows) in references {
                next.references_by_model_definition_id.insert(key.clone(), rows.clone());
            }
        }
        if let Some(delta) = &self.nodes { next.nodes = apply_nodes_delta(&next.nodes, delta); }
        if let Some(value) = &self.shape_geometry { next.shape_geometry = value.clone(); }
        if let Some(value) = &self.building_geometry { next.building_geometry = value.clone(); }
        if let Some(value) = &self.energy_geometry { next.energy_geometry = value.clone(); }
        if let Some(value) = &self.structure_classic_geometry { next.structure_classic_geometry = value.clone(); }
        if let Some(value) = &self.active_model_definition_id { next.active_model_definition_id = value.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(references_by_model_definition_id);
        take!(shape_geometry);
        take!(building_geometry);
        take!(energy_geometry);
        take!(structure_classic_geometry);
        take!(active_model_definition_id);
        take!(selected_object_ids);
        take!(selected_node_ids);
        take!(active_object_id);
        take!(component_selection);
        take!(selected_reference_model_definition_id);
        take!(selected_reference_id);
        take!(selected_primitive_id);
        take!(selected_primitive_kind);
        take!(active_utility_id);
        take!(active_example_id);
        take!(selection_method);
        take!(engagement_input);
        take!(engagement_step);
        take!(engagement_pane);
        take!(engagement_session_json);
        take!(last_finalized_interaction_id);
        take!(sun_enabled);
        take!(sun_azimuth);
        take!(sun_elevation);
        take!(sun_intensity);
        take!(sun_color);
        take!(camera);
        take!(camera_building);
        take!(camera_energy);
        take!(camera_structure_classic);
        take!(dislocate_shape);
        take!(dislocate_building);
        take!(dislocate_energy);
        take!(dislocate_structure_classic);
        take!(locale);
        take!(terminology);
        take!(contributions_json);
        take!(hovered_object_id);
        take!(hovered_target_object_id);
        take!(hovered_target_mode);
        take!(hovered_target_id);
        absorb_objects_delta(&mut self.objects, other.objects);
        absorb_objects_delta(&mut self.building_objects, other.building_objects);
        absorb_objects_delta(&mut self.energy_objects, other.energy_objects);
        absorb_objects_delta(&mut self.structure_classic_objects, other.structure_classic_objects);
        match (&mut self.nodes, other.nodes) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() { dst.reordered = src.reordered; }
            }
            (None, Some(src)) => self.nodes = Some(src),
            _ => {}
        }
    }
}

fn absorb_objects_delta(target: &mut Option<CadObjectsDelta>, incoming: Option<CadObjectsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() { dst.reordered = src.reordered; }
            }
            None => *target = Some(src),
        }
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::op::CadMutation;
    use crate::artifacts::cad::testkit::{sample_object, sample_scene};
    use crate::artifacts::cad::CadPaneId;
    use protocol::Mutation;

    #[test]
    fn whole_artifact_diff_replaces_the_snapshot_and_absorbs_every_earlier_edit() {
        let base = sample_scene();
        let mut diff = CadMutation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() }.diff(&base);
        let replacement = CadMutation::SetSnapshot { snapshot: Box::new(base.clone()) }.diff(&base);
        diff.absorb(replacement);
        assert_eq!(diff.apply(&base), base, "a whole-artifact diff wins over anything absorbed before it");
    }

    #[test]
    fn object_collection_diffs_absorb_into_one_apply() {
        let base = sample_scene();
        let mut diff = CadMutation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-9") }.diff(&base);
        diff.absorb(CadMutation::PatchObject { pane: CadPaneId::Shape, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), ..Default::default() } }.diff(&base));
        let next = diff.apply(&base);
        assert!(next.objects.iter().any(|object| object.id == "object-9"));
        assert_eq!(next.objects.iter().find(|object| object.id == "object-1").expect("object-1").label, "Renamed");
    }
}
//#endregion 🧪️Tests
