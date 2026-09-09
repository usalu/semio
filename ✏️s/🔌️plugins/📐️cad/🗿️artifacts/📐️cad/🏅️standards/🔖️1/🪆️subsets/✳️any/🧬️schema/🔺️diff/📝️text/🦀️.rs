//! 🔺️ CAD artifact — sparse field-delta diff codec and apply/absorb.

use crate::diff::schema::{CadDiff, CadNodesDelta};
use crate::mutations::CadReferencePatch;
use crate::schema::CadArtifact;
use crate::{CadNode, CadReference, CadSnapshot};
use protocol::MutationDiff;
use std::collections::BTreeMap;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl CadDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &CadArtifact) -> protocol::MutationApplyResult<CadArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(value) = &self.shape_model {
                next.shape_model = value.clone();
            }
            if let Some(value) = &self.building_model {
                next.building_model = value.clone();
            }
            if let Some(value) = &self.energy_model {
                next.energy_model = value.clone();
            }
            if let Some(value) = &self.structure_classic_model {
                next.structure_classic_model = value.clone();
            }
            if let Some(list) = &self.drawings {
                next.drawings = list.values.clone();
            }
            if let Some(references) = &self.references_by_model_definition_id {
                for (key, rows) in references {
                    next.references_by_model_definition_id.insert(key.clone(), rows.clone());
                }
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(value) = &self.active_model_definition_id {
                next.active_model_definition_id = value.clone();
            }
            next
        })
    }
}

fn apply_nodes_delta(nodes: &[CadNode], delta: &CadNodesDelta) -> protocol::MutationApplyResult<Vec<CadNode>> {
    for (index, id) in delta.removed.iter().enumerate() {
        if !nodes.iter().any(|node| &node.id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed node does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "node is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, item) in delta.added.iter().enumerate() {
        if nodes.iter().any(|node| node.id == item.id) || delta.added[..index].iter().any(|prior| prior.id == item.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added node identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    for (index, entry) in delta.patched.iter().enumerate() {
        if !nodes.iter().any(|node| node.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched node does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if delta.removed.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "node cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
        }
        if delta.patched[..index].iter().any(|prior| prior.id == entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "node is patched more than once").at(["patched".to_string(), index.to_string()]));
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
        let node =
            next.iter_mut().find(|node| node.id == entry.id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched node does not exist after structural edits").at(["patched".to_string(), index.to_string()]))?;
        if let Some(label) = &entry.patch.label {
            node.label = label.clone();
        }
    }
    if let Some(order) = &delta.reordered {
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|node| &node.id == id)) {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "node reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|node| (node.id.clone(), node)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered node does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    Ok(next)
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

impl MutationDiff<CadSnapshot> for CadDiff {
    fn apply(&self, snapshot: &CadSnapshot) -> protocol::MutationApplyResult<CadSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(value) = &self.shape_model {
                next.shape_model = value.clone();
            }
            if let Some(value) = &self.building_model {
                next.building_model = value.clone();
            }
            if let Some(value) = &self.energy_model {
                next.energy_model = value.clone();
            }
            if let Some(value) = &self.structure_classic_model {
                next.structure_classic_model = value.clone();
            }
            if let Some(list) = &self.drawings {
                next.drawings = list.values.clone();
            }
            if let Some(references) = &self.references_by_model_definition_id {
                for (key, rows) in references {
                    next.references_by_model_definition_id.insert(key.clone(), rows.clone());
                }
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(value) = &self.active_model_definition_id {
                next.active_model_definition_id = value.clone();
            }
            next
        })
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
        take!(shape_model);
        take!(building_model);
        take!(energy_model);
        take!(structure_classic_model);
        take!(drawings);
        take!(references_by_model_definition_id);
        take!(active_model_definition_id);
        match (&mut self.nodes, other.nodes) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.nodes = Some(src),
            _ => {}
        }
    }
}

//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type CadDiffText = String;
//#endregion 🚚️Carrier
