//! 🔺️ CAD artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::cad::diff::schema::{CadDiff, CadNodesDelta};
use crate::artifacts::cad::mutations::CadReferencePatch;
use crate::artifacts::cad::schema::CadArtifact;
use crate::artifacts::cad::{CadNode, CadReference, CadSnapshot};
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
    pub async fn apply_to_artifact(&self, artifact: &CadArtifact) -> protocol::MutationApplyResult<CadArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(id) = &self.id { next.id = id.clone(); }
            if let Some(value) = &self.shape_model { next.shape_model = value.clone(); }
            if let Some(value) = &self.building_model { next.building_model = value.clone(); }
            if let Some(value) = &self.energy_model { next.energy_model = value.clone(); }
            if let Some(value) = &self.structure_classic_model { next.structure_classic_model = value.clone(); }
            if let Some(list) = &self.drawings { next.drawings = list.values.clone(); }
            if let Some(references) = &self.references_by_model_definition_id {
                for (key, rows) in references {
                    next.references_by_model_definition_id.insert(key.clone(), rows.clone());
                }
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
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
        })
    }
}

async fn apply_nodes_delta(
    nodes: &[CadNode],
    delta: &CadNodesDelta,
) -> protocol::MutationApplyResult<Vec<CadNode>> {
    for (index, id) in delta.removed.iter().enumerate() {
        if !nodes.iter().any(|node| &node.id == id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "removed node does not exist",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "node is removed more than once",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, item) in delta.added.iter().enumerate() {
        if nodes.iter().any(|node| node.id == item.id)
            || delta.added[..index].iter().any(|prior| prior.id == item.id)
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "added node identity already exists",
            )
            .at(["added".to_string(), index.to_string()]));
        }
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        if !nodes.iter().any(|node| node.id == entry.id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "patched node does not exist",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
        if delta.removed.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.conflicting-target",
                "node cannot be removed and patched",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
        if delta.patched[..index].iter().any(|prior| prior.id == entry.id) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "node is patched more than once",
            )
            .at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next = nodes.to_vec();
    for id in &delta.removed {
        next.retain(|node| &node.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        let node = next.iter_mut().find(|node| node.id == entry.id).ok_or_else(|| {
            protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "patched node does not exist after structural edits",
            )
            .at(["patched".to_string(), index.to_string()])
        })?;
        if let Some(label) = &entry.patch.label {
            node.label = label.clone();
        }
    }
    if let Some(order) = &delta.reordered {
        if order.len() != next.len()
            || order.iter().enumerate().any(|(index, id)| {
                order[..index].contains(id) || !next.iter().any(|node| &node.id == id)
            })
        {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.invalid-order",
                "node reorder must be a complete unique permutation",
            )
            .at(["reordered"]));
        }
        let mut by_id: BTreeMap<_, _> =
            next.into_iter().map(|node| (node.id.clone(), node)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| {
                protocol::MutationApplyError::new(
                    "mutation.apply.missing-target",
                    "reordered node does not exist",
                )
                .at(["reordered".to_string(), id.clone()])
            })?);
        }
        next = ordered;
    }
    Ok(next)
}

pub async fn apply_reference_patch(reference: &mut CadReference, patch: &CadReferencePatch) {
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
    async fn apply(&self, snapshot: &CadSnapshot) -> protocol::MutationApplyResult<CadSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(id) = &self.id { next.id = id.clone(); }
            if let Some(value) = &self.shape_model { next.shape_model = value.clone(); }
            if let Some(value) = &self.building_model { next.building_model = value.clone(); }
            if let Some(value) = &self.energy_model { next.energy_model = value.clone(); }
            if let Some(value) = &self.structure_classic_model { next.structure_classic_model = value.clone(); }
            if let Some(list) = &self.drawings { next.drawings = list.values.clone(); }
            if let Some(references) = &self.references_by_model_definition_id {
                for (key, rows) in references {
                    next.references_by_model_definition_id.insert(key.clone(), rows.clone());
                }
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(value) = &self.active_model_definition_id { next.active_model_definition_id = value.clone(); }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(shape_model);
        take!(building_model);
        take!(energy_model);
        take!(structure_classic_model);
        take!(drawings);
        take!(references_by_model_definition_id);
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

//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::mutations::create_node::mutation::CreateNode;
    use crate::artifacts::cad::mutations::delete_node::mutation::DeleteNode;
    use crate::artifacts::cad::mutations::rename_node::mutation::RenameNode;
    use crate::artifacts::cad::op::CadMutation;
    use crate::artifacts::cad::testkit::sample_scene;
    use protocol::Mutation;

    /// ⚖️ `CadDiff.artifact` (a whole-artifact replacement fragment) still exists as a `CadDiff`
    /// FIELD — only its former mutation source (`SetSnapshot`, banned per taxonomy) is gone; this
    /// law still holds for whatever future non-mutation path (`ArtifactStore::reset`) populates it.
    #[semio_framework_async_macros::async_test]
    async fn whole_artifact_diff_replaces_the_snapshot_and_absorbs_every_earlier_edit() {
        let base = sample_scene();
        let mut diff = CadMutation::DeleteNode(DeleteNode { node_id: "node-1".into() }).diff(&base).diff().clone();
        let replacement = CadDiff { artifact: Some(Box::new(crate::artifacts::cad::schema::CadArtifact::from_snapshot(base.clone()))), ..Default::default() };
        diff.absorb(replacement);
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base, "a whole-artifact diff wins over anything absorbed before it");
    }

    #[semio_framework_async_macros::async_test]
    async fn node_collection_diffs_absorb_into_one_apply() {
        let base = sample_scene();
        let mut diff = CadMutation::CreateNode(CreateNode { node: crate::artifacts::cad::CadNode { id: "node-9".into(), label: "Fresh".into(), kind: "group".into() } }).diff(&base).diff().clone();
        diff.absorb(CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }).diff(&base).diff().clone());
        let next = diff.apply(&base).expect("valid mutation diff");
        assert!(next.nodes.iter().any(|node| node.id == "node-9"));
        assert_eq!(next.nodes.iter().find(|node| node.id == "node-1").expect("node-1").label, "Renamed");
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_named_diff_rejects_without_changing_the_base() {
        let base = sample_scene();
        let original = base.clone();
        let diff = CadDiff {
            nodes: Some(CadNodesDelta {
                removed: vec!["missing-node".into()],
                ..Default::default()
            }),
            ..Default::default()
        };
        let error = diff.apply(&base).expect_err("missing named target must reject");
        assert_eq!(error.code, "mutation.apply.missing-target");
        assert_eq!(error.target, ["nodes", "removed", "0"]);
        assert_eq!(base, original);
    }
}
//#endregion 🧪️Tests
