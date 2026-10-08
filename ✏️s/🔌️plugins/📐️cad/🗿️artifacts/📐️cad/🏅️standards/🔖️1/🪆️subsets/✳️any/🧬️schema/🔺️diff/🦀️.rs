//! 🧬️ Cad diff schema — sparse field delta over the artifact.

use crate::mutations::{CadNodePatch, CadOpacitySet, CadOrientationSet, CadReferencePatch, CadScaleSet};
use crate::{CadBrepChild, CadDrawingChild, CadModelChild, CadNode, CadReference};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the cad artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub shape_model: Option<CadModelSlot>,
    #[state(artifact)]
    pub building_model: Option<CadModelSlot>,
    #[state(artifact)]
    pub energy_model: Option<CadModelSlot>,
    #[state(artifact)]
    pub structure_classic_model: Option<CadModelSlot>,
    #[state(artifact)]
    pub drawings: Option<CadDrawingsDelta>,
    #[state(artifact)]
    pub breps: Option<CadBrepsDelta>,
    #[state(artifact)]
    pub references_by_model_definition_id: Option<BTreeMap<String, CadReferencesDelta>>,
    #[state(artifact)]
    pub nodes: Option<CadNodesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers

/// 🧱️ Explicit replacement of one fixed model slot: preserves an untouched slot (absent) apart from a cleared one (`child: None`),
/// which a bare `Option<Option<_>>` would collapse on the wire.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadModelSlot {
    pub child: Option<CadModelChild>,
}

protocol::list_delta! {
    /// 🧩 Positional delta for nodes (`protocol::list_delta`).
    pub CadNodesDelta { removal: CadNodeRemoval, insertion: CadNodeInsertion, relocation: CadNodeRelocation, modification: CadNodeModification, row: CadNode, patch: CadNodePatch, key: id }
}

protocol::list_delta! {
    /// 📎 Positional delta for one model's reference list (`protocol::list_delta`).
    pub CadReferencesDelta { removal: CadReferenceRemoval, insertion: CadReferenceInsertion, relocation: CadReferenceRelocation, modification: CadReferenceModification, row: CadReference, patch: CadReferencePatch, key: id }
}

protocol::plain_list_delta! {
    /// 🧩️ Positional delta for the `drawings` composed CHILD COLLECTION (`protocol::list_delta`).
    pub CadDrawingsDelta { removal: CadDrawingRemoval, insertion: CadDrawingInsertion, relocation: CadDrawingRelocation, row: CadDrawingChild, list: Vec<CadDrawingChild>, key: String = |row| row.child_id.clone() }
}

protocol::plain_list_delta! {
    /// 🧊️ Positional delta for the ordered `breps` topology sibling collection (`protocol::list_delta`).
    pub CadBrepsDelta { removal: CadBrepRemoval, insertion: CadBrepInsertion, relocation: CadBrepRelocation, row: CadBrepChild, list: Vec<CadBrepChild>, key: String = |row| row.child_id.clone() }
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Rows
use crate::CadSnapshot;
use protocol::list_delta::RowPatch;
use protocol::MutationDiff;

impl RowPatch<CadNode> for CadNodePatch {
    fn commit_into(&self, row: &mut CadNode, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        patch_node(row, self);
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        merge_node_patch(self, later);
    }

    fn inverse(&self, row: &CadNode) -> Self {
        invert_node_patch(self, row)
    }

    fn is_empty(&self) -> bool {
        *self == CadNodePatch::default()
    }
}

impl RowPatch<CadReference> for CadReferencePatch {
    fn commit_into(&self, row: &mut CadReference, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        patch_reference(row, self);
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        merge_reference_patch(self, later);
    }

    fn inverse(&self, row: &CadReference) -> Self {
        invert_reference_patch(self, row)
    }

    fn is_empty(&self) -> bool {
        *self == CadReferencePatch::default()
    }
}

fn patch_node(node: &mut CadNode, patch: &CadNodePatch) {
    if let Some(label) = &patch.label {
        node.label = label.clone();
    }
}

fn merge_node_patch(existing: &mut CadNodePatch, incoming: CadNodePatch) {
    if incoming.label.is_some() {
        existing.label = incoming.label;
    }
}

fn invert_node_patch(patch: &CadNodePatch, base: &CadNode) -> CadNodePatch {
    CadNodePatch { label: patch.label.as_ref().map(|_| base.label.clone()) }
}


fn patch_reference(reference: &mut CadReference, patch: &CadReferencePatch) {
    if let Some(source_url) = &patch.source_url {
        reference.source_url = source_url.clone();
    }
    if let Some(media_kind) = &patch.media_kind {
        reference.media_kind = media_kind.clone();
    }
    if let Some(origin) = patch.origin {
        reference.origin = origin;
    }
    if let Some(orientation) = &patch.orientation {
        reference.orientation = orientation.value;
    }
    if let Some(scale) = &patch.scale {
        reference.scale = scale.value;
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
    if let Some(opacity) = &patch.opacity {
        reference.opacity = opacity.value;
    }
}

fn merge_reference_patch(existing: &mut CadReferencePatch, incoming: CadReferencePatch) {
    macro_rules! take {
        ($field:ident) => {
            if incoming.$field.is_some() {
                existing.$field = incoming.$field;
            }
        };
    }
    take!(source_url);
    take!(media_kind);
    take!(origin);
    take!(orientation);
    take!(scale);
    take!(width_world);
    take!(hidden);
    take!(locked);
    take!(opacity);
}

fn invert_reference_patch(patch: &CadReferencePatch, base: &CadReference) -> CadReferencePatch {
    CadReferencePatch {
        source_url: patch.source_url.as_ref().map(|_| base.source_url.clone()),
        media_kind: patch.media_kind.as_ref().map(|_| base.media_kind.clone()),
        origin: patch.origin.map(|_| base.origin),
        orientation: patch.orientation.as_ref().map(|_| CadOrientationSet { value: base.orientation }),
        scale: patch.scale.as_ref().map(|_| CadScaleSet { value: base.scale }),
        width_world: patch.width_world.map(|_| base.width_world),
        hidden: patch.hidden.map(|_| base.hidden),
        locked: patch.locked.map(|_| base.locked),
        opacity: patch.opacity.as_ref().map(|_| CadOpacitySet { value: base.opacity }),
    }
}


//#endregion 🔖️Rows

//#region 🔖️Apply
impl MutationDiff<CadSnapshot> for CadDiff {
    fn apply(&self, snapshot: &CadSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CadSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(slot) = &self.shape_model {
            next.shape_model = slot.child.clone();
        }
        if let Some(slot) = &self.building_model {
            next.building_model = slot.child.clone();
        }
        if let Some(slot) = &self.energy_model {
            next.energy_model = slot.child.clone();
        }
        if let Some(slot) = &self.structure_classic_model {
            next.structure_classic_model = slot.child.clone();
        }
        if let Some(delta) = &self.drawings {
            next.drawings = delta.commit_onto(&next.drawings, capability).map_err(|error| error.under(["drawings"]))?;
        }
        if let Some(delta) = &self.breps {
            next.breps = delta.commit_onto(&next.breps, capability).map_err(|error| error.under(["breps"]))?;
        }
        if let Some(models) = &self.references_by_model_definition_id {
            for (model, delta) in models {
                let existing = next.references_by_model_definition_id.get(model).cloned().unwrap_or_default();
                let rows = delta.commit_onto(&existing, capability).map_err(|error| error.under(["referencesByModelDefinitionId".to_string(), model.clone()]))?;
                if rows.is_empty() {
                    next.references_by_model_definition_id.remove(model);
                } else {
                    next.references_by_model_definition_id.insert(model.clone(), rows);
                }
            }
        }
        if let Some(delta) = &self.nodes {
            next.nodes = delta.commit_onto(&next.nodes, capability).map_err(|error| error.under(["nodes"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
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
        match (&mut self.drawings, other.drawings) {
            (Some(dst), Some(src)) => dst.absorb(src),
            (None, Some(src)) => self.drawings = Some(src),
            _ => {}
        }
        match (&mut self.breps, other.breps) {
            (Some(dst), Some(src)) => dst.absorb(src),
            (None, Some(src)) => self.breps = Some(src),
            _ => {}
        }
        if let Some(incoming) = other.references_by_model_definition_id {
            let models = self.references_by_model_definition_id.get_or_insert_with(BTreeMap::new);
            for (model, src) in incoming {
                match models.get_mut(&model) {
                    Some(dst) => dst.absorb(src),
                    None => {
                        models.insert(model, src);
                    }
                }
            }
        }
        match (&mut self.nodes, other.nodes) {
            (Some(dst), Some(src)) => dst.absorb(src),
            (None, Some(src)) => self.nodes = Some(src),
            _ => {}
        }
        if self.drawings.as_ref().is_some_and(CadDrawingsDelta::is_empty) {
            self.drawings = None;
        }
        if self.breps.as_ref().is_some_and(CadBrepsDelta::is_empty) {
            self.breps = None;
        }
        if self.nodes.as_ref().is_some_and(CadNodesDelta::is_empty) {
            self.nodes = None;
        }
        if let Some(models) = self.references_by_model_definition_id.as_mut() {
            models.retain(|_, delta| !delta.is_empty());
            if models.is_empty() {
                self.references_by_model_definition_id = None;
            }
        }
    }
}

impl protocol::DiffAlgebra<CadSnapshot> for CadDiff {
    fn inverse(&self, base: &CadSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            shape_model: self.shape_model.as_ref().map(|_| CadModelSlot { child: base.shape_model.clone() }),
            building_model: self.building_model.as_ref().map(|_| CadModelSlot { child: base.building_model.clone() }),
            energy_model: self.energy_model.as_ref().map(|_| CadModelSlot { child: base.energy_model.clone() }),
            structure_classic_model: self.structure_classic_model.as_ref().map(|_| CadModelSlot { child: base.structure_classic_model.clone() }),
            drawings: self.drawings.as_ref().map(|delta| delta.inverse(&base.drawings)),
            breps: self.breps.as_ref().map(|delta| delta.inverse(&base.breps)),
            references_by_model_definition_id: self.references_by_model_definition_id.as_ref().map(|models| {
                let none = Vec::new();
                models
                    .iter()
                    .map(|(model, delta)| {
                        let existing = base.references_by_model_definition_id.get(model).unwrap_or(&none);
                        (model.clone(), delta.inverse(existing))
                    })
                    .collect()
            }),
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.nodes)),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.id.is_none()
            && self.shape_model.is_none()
            && self.building_model.is_none()
            && self.energy_model.is_none()
            && self.structure_classic_model.is_none()
            && self.drawings.as_ref().is_none_or(CadDrawingsDelta::is_empty)
            && self.breps.as_ref().is_none_or(CadBrepsDelta::is_empty)
            && self.references_by_model_definition_id.as_ref().is_none_or(|models| models.values().all(CadReferencesDelta::is_empty))
            && self.nodes.as_ref().is_none_or(CadNodesDelta::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
