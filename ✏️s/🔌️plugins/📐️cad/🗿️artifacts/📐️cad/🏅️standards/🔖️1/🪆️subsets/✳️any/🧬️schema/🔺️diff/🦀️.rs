//! 🧬️ Cad diff schema — sparse field delta over the artifact.

use crate::mutations::CadNodePatch;
use crate::{CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the cad artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::CadArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub shape_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub building_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub energy_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub structure_classic_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub drawings: Option<CadDrawingChildList>,
    #[state(artifact)]
    pub references_by_model_definition_id: Option<BTreeMap<String, CadReferenceList>>,
    #[state(artifact)]
    pub nodes: Option<CadNodesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadStringList {
    pub values: Vec<String>,
}

/// 🧩️ Whole-list wrapper for the `drawings` composed CHILD COLLECTION diff field — same `RunList`
/// shape `✳️text`/`✳️kit` use for their own Vec-of-child diff fields (kit's own
/// `SemioKitModelChildList` is the direct precedent for a `Vec<ArtifactChild<S>>` diff wrapper).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadDrawingChildList {
    pub values: Vec<CadDrawingChild>,
}

/// 🧩 Identified-collection delta for nodes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadNodesDelta {
    pub added: Vec<CadNode>,
    pub removed: Vec<String>,
    pub patched: Vec<CadNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched node entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadNodePatchEntry {
    pub id: String,
    pub patch: CadNodePatch,
}
//#endregion 🔖️DeltaHelpers

use crate::mutations::CadReferencePatch;
use crate::schema::CadArtifact;
use crate::CadReference;
use crate::CadSnapshot;
use protocol::MutationDiff;

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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
