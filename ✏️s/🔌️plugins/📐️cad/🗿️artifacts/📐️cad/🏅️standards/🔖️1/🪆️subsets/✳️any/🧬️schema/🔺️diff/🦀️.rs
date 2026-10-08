//! 🧬️ Cad diff schema — sparse field delta over the artifact.

use crate::mutations::{CadNodePatch, CadOpacitySet, CadOrientationSet, CadReferencePatch, CadScaleSet};
use crate::{CadBrepChild, CadDrawingChild, CadModelChild, CadNode, CadReference};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};

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
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadStringList {
    pub values: Vec<String>,
}

/// 🧱️ Explicit replacement of one fixed model slot: preserves an untouched slot (absent) apart from a cleared one (`child: None`),
/// which a bare `Option<Option<_>>` would collapse on the wire.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadModelSlot {
    pub child: Option<CadModelChild>,
}

/// 🧩️ Identified-collection delta for the `drawings` composed CHILD COLLECTION: removed ids, appended children and, only when the
/// final order is not "survivors then appended", the complete final id order.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadDrawingsDelta {
    pub added: Vec<CadDrawingChild>,
    pub removed: Vec<String>,
    pub reordered: Option<Vec<String>>,
}

/// 🧊️ Identified-collection delta for the ordered `breps` topology sibling collection.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadBrepsDelta {
    pub added: Vec<CadBrepChild>,
    pub removed: Vec<String>,
    pub reordered: Option<Vec<String>>,
}

/// 📎 Identified-collection delta for one model's reference list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadReferencesDelta {
    pub added: Vec<CadReference>,
    pub removed: Vec<String>,
    pub patched: Vec<CadReferencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched reference entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadReferencePatchEntry {
    pub id: String,
    pub patch: CadReferencePatch,
}

/// 🧩 Identified-collection delta for nodes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct CadNodesDelta {
    pub added: Vec<CadNode>,
    pub removed: Vec<String>,
    pub patched: Vec<CadNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched node entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadNodePatchEntry {
    pub id: String,
    pub patch: CadNodePatch,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Rows
use crate::CadSnapshot;
use protocol::MutationDiff;

trait RowKey {
    fn row_key(&self) -> &str;
}

impl RowKey for CadNode {
    fn row_key(&self) -> &str {
        &self.id
    }
}

impl RowKey for CadReference {
    fn row_key(&self) -> &str {
        &self.id
    }
}

impl RowKey for CadDrawingChild {
    fn row_key(&self) -> &str {
        &self.child_id
    }
}

impl RowKey for CadBrepChild {
    fn row_key(&self) -> &str {
        &self.child_id
    }
}

/// 🧮️ The one keyed-collection algebra every cad collection delta shares: removed ids, appended rows, keyed field patches and an
/// optional complete final order — apply, absorb (create∘delete cancels, patch∘patch merges, a later order supersedes), the
/// negative delta and the state delta all work on this shape.
#[derive(Clone, Debug, PartialEq)]
struct Rows<T, P> {
    added: Vec<T>,
    removed: Vec<String>,
    patched: Vec<(String, P)>,
    reordered: Option<Vec<String>>,
}

impl<T, P> Default for Rows<T, P> {
    fn default() -> Self {
        Self { added: Vec::new(), removed: Vec::new(), patched: Vec::new(), reordered: None }
    }
}

fn keys_of<T: RowKey>(rows: &[T]) -> Vec<String> {
    rows.iter().map(|row| row.row_key().to_string()).collect()
}

impl<T: Clone + RowKey, P: Clone + Default + PartialEq> Rows<T, P> {
    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.patched.iter().all(|(_, patch)| *patch == P::default()) && self.reordered.is_none()
    }

    fn normalize(&mut self) {
        self.removed.sort();
        self.removed.dedup();
        self.patched.sort_by(|left, right| left.0.cmp(&right.0));
    }

    fn apply(&self, rows: &[T], patch_row: impl Fn(&mut T, &P)) -> protocol::MutationApplyResult<Vec<T>> {
        for (index, id) in self.removed.iter().enumerate() {
            if !rows.iter().any(|row| row.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed row does not exist").at(["removed".to_string(), index.to_string()]));
            }
            if self.removed[..index].contains(id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "row is removed more than once").at(["removed".to_string(), index.to_string()]));
            }
        }
        for (index, item) in self.added.iter().enumerate() {
            let id = item.row_key();
            if rows.iter().any(|row| row.row_key() == id && !self.removed.iter().any(|removed| removed == id)) || self.added[..index].iter().any(|prior| prior.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added row identity already exists").at(["added".to_string(), index.to_string()]));
            }
        }
        for (index, (id, _)) in self.patched.iter().enumerate() {
            if !rows.iter().any(|row| row.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched row does not exist").at(["patched".to_string(), index.to_string()]));
            }
            if self.removed.contains(id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "row cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
            }
            if self.patched[..index].iter().any(|(prior, _)| prior == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "row is patched more than once").at(["patched".to_string(), index.to_string()]));
            }
        }
        let mut next: Vec<T> = rows.iter().filter(|row| !self.removed.iter().any(|id| id == row.row_key())).cloned().collect();
        for (id, patch) in &self.patched {
            if let Some(row) = next.iter_mut().find(|row| row.row_key() == id) {
                patch_row(row, patch);
            }
        }
        next.extend(self.added.iter().cloned());
        if let Some(order) = &self.reordered {
            if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| row.row_key() == id)) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
            }
            let mut by_id: BTreeMap<String, T> = next.into_iter().map(|row| (row.row_key().to_string(), row)).collect();
            let mut ordered = Vec::with_capacity(order.len());
            for id in order {
                ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered row does not exist").at(["reordered".to_string(), id.clone()]))?);
            }
            next = ordered;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self, patch_row: impl Fn(&mut T, &P), merge: impl Fn(&mut P, P)) {
        let Self { added, removed, patched, reordered } = other;
        for id in &removed {
            if let Some(at) = self.added.iter().position(|row| row.row_key() == id) {
                self.added.remove(at);
                continue;
            }
            self.patched.retain(|(key, _)| key != id);
            if !self.removed.contains(id) {
                self.removed.push(id.clone());
            }
        }
        for (id, incoming) in patched {
            if let Some(row) = self.added.iter_mut().find(|row| row.row_key() == id) {
                patch_row(row, &incoming);
            } else if let Some((_, existing)) = self.patched.iter_mut().find(|(key, _)| *key == id) {
                merge(existing, incoming);
            } else {
                self.patched.push((id, incoming));
            }
        }
        match reordered {
            Some(order) => self.reordered = Some(order),
            None => {
                if let Some(order) = self.reordered.as_mut() {
                    order.retain(|id| !removed.contains(id));
                    order.extend(added.iter().map(|row| row.row_key().to_string()));
                }
            }
        }
        self.added.extend(added);
        self.normalize();
    }

    fn inverse(&self, base: &[T], invert: impl Fn(&P, &T) -> P) -> Self {
        let base_keys = keys_of(base);
        let added_keys: BTreeSet<&str> = self.added.iter().map(RowKey::row_key).collect();
        let restored: Vec<T> = base.iter().filter(|row| self.removed.iter().any(|id| id == row.row_key())).cloned().collect();
        let patched = self.patched.iter().filter(|(id, _)| !added_keys.contains(id.as_str())).filter_map(|(id, patch)| base.iter().find(|row| row.row_key() == id).map(|row| (id.clone(), invert(patch, row)))).collect();
        let mut after: Vec<String> = base_keys.iter().filter(|id| !self.removed.contains(id)).cloned().chain(self.added.iter().map(|row| row.row_key().to_string())).collect();
        if let Some(order) = &self.reordered {
            after = order.clone();
        }
        let natural: Vec<String> = after.into_iter().filter(|id| !added_keys.contains(id.as_str())).chain(restored.iter().map(|row| row.row_key().to_string())).collect();
        let reordered = (natural != base_keys).then_some(base_keys);
        let mut inverse = Self { added: restored, removed: self.added.iter().map(|row| row.row_key().to_string()).collect(), patched, reordered };
        inverse.normalize();
        inverse
    }

    fn between(base: &[T], other: &[T], compare: impl Fn(&T, &T) -> Option<Option<P>>) -> Self {
        let base_keys = keys_of(base);
        let other_keys = keys_of(other);
        let mut delta = Self::default();
        let mut replaced = BTreeSet::new();
        for row in other {
            let Some(source) = base.iter().find(|candidate| candidate.row_key() == row.row_key()) else { continue };
            match compare(source, row) {
                Some(Some(patch)) => delta.patched.push((row.row_key().to_string(), patch)),
                Some(None) => {}
                None => {
                    replaced.insert(row.row_key().to_string());
                }
            }
        }
        delta.removed = base.iter().filter(|row| !other_keys.iter().any(|id| id == row.row_key()) || replaced.contains(row.row_key())).map(|row| row.row_key().to_string()).collect();
        delta.added = other.iter().filter(|row| !base_keys.iter().any(|id| id == row.row_key()) || replaced.contains(row.row_key())).cloned().collect();
        let natural: Vec<String> = base_keys.iter().filter(|id| !delta.removed.contains(id)).cloned().chain(delta.added.iter().map(|row| row.row_key().to_string())).collect();
        delta.reordered = (natural != other_keys).then_some(other_keys);
        delta.normalize();
        delta
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

fn compare_nodes(base: &CadNode, other: &CadNode) -> Option<Option<CadNodePatch>> {
    if base.kind != other.kind {
        return None;
    }
    Some((base.label != other.label).then(|| CadNodePatch { label: Some(other.label.clone()) }))
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

fn compare_references(base: &CadReference, other: &CadReference) -> Option<Option<CadReferencePatch>> {
    let patch = CadReferencePatch {
        source_url: (base.source_url != other.source_url).then(|| other.source_url.clone()),
        media_kind: (base.media_kind != other.media_kind).then(|| other.media_kind.clone()),
        origin: (base.origin != other.origin).then_some(other.origin),
        orientation: (base.orientation != other.orientation).then_some(CadOrientationSet { value: other.orientation }),
        scale: (base.scale != other.scale).then_some(CadScaleSet { value: other.scale }),
        width_world: (base.width_world != other.width_world).then_some(other.width_world),
        hidden: (base.hidden != other.hidden).then_some(other.hidden),
        locked: (base.locked != other.locked).then_some(other.locked),
        opacity: (base.opacity != other.opacity).then_some(CadOpacitySet { value: other.opacity }),
    };
    Some((patch != CadReferencePatch::default()).then_some(patch))
}

impl CadNodesDelta {
    fn rows(&self) -> Rows<CadNode, CadNodePatch> {
        Rows { added: self.added.clone(), removed: self.removed.clone(), patched: self.patched.iter().map(|entry| (entry.id.clone(), entry.patch.clone())).collect(), reordered: self.reordered.clone() }
    }
    fn from_rows(rows: Rows<CadNode, CadNodePatch>) -> Self {
        Self { added: rows.added, removed: rows.removed, patched: rows.patched.into_iter().map(|(id, patch)| CadNodePatchEntry { id, patch }).collect(), reordered: rows.reordered }
    }
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}

impl CadReferencesDelta {
    fn rows(&self) -> Rows<CadReference, CadReferencePatch> {
        Rows { added: self.added.clone(), removed: self.removed.clone(), patched: self.patched.iter().map(|entry| (entry.id.clone(), entry.patch.clone())).collect(), reordered: self.reordered.clone() }
    }
    fn from_rows(rows: Rows<CadReference, CadReferencePatch>) -> Self {
        Self { added: rows.added, removed: rows.removed, patched: rows.patched.into_iter().map(|(id, patch)| CadReferencePatchEntry { id, patch }).collect(), reordered: rows.reordered }
    }
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}

impl CadDrawingsDelta {
    fn rows(&self) -> Rows<CadDrawingChild, ()> {
        Rows { added: self.added.clone(), removed: self.removed.clone(), patched: Vec::new(), reordered: self.reordered.clone() }
    }
    fn from_rows(rows: Rows<CadDrawingChild, ()>) -> Self {
        Self { added: rows.added, removed: rows.removed, reordered: rows.reordered }
    }
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}

impl CadBrepsDelta {
    fn rows(&self) -> Rows<CadBrepChild, ()> {
        Rows { added: self.added.clone(), removed: self.removed.clone(), patched: Vec::new(), reordered: self.reordered.clone() }
    }
    fn from_rows(rows: Rows<CadBrepChild, ()>) -> Self {
        Self { added: rows.added, removed: rows.removed, reordered: rows.reordered }
    }
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}
//#endregion 🔖️Rows

//#region 🔖️Apply
impl MutationDiff<CadSnapshot> for CadDiff {
    fn apply(&self, snapshot: &CadSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CadSnapshot> {
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
            next.drawings = delta.rows().apply(&next.drawings, |_, _| {}).map_err(|error| error.under(["drawings"]))?;
        }
        if let Some(delta) = &self.breps {
            next.breps = delta.rows().apply(&next.breps, |_, _| {}).map_err(|error| error.under(["breps"]))?;
        }
        if let Some(models) = &self.references_by_model_definition_id {
            for (model, delta) in models {
                let existing = next.references_by_model_definition_id.get(model).cloned().unwrap_or_default();
                let rows = delta.rows().apply(&existing, patch_reference).map_err(|error| error.under(["referencesByModelDefinitionId".to_string(), model.clone()]))?;
                if rows.is_empty() {
                    next.references_by_model_definition_id.remove(model);
                } else {
                    next.references_by_model_definition_id.insert(model.clone(), rows);
                }
            }
        }
        if let Some(delta) = &self.nodes {
            next.nodes = delta.rows().apply(&next.nodes, patch_node).map_err(|error| error.under(["nodes"]))?;
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
            (Some(dst), Some(src)) => {
                let mut rows = dst.rows();
                rows.absorb(src.rows(), |_, _| {}, |_, _| {});
                *dst = CadDrawingsDelta::from_rows(rows);
            }
            (None, Some(src)) => self.drawings = Some(src),
            _ => {}
        }
        match (&mut self.breps, other.breps) {
            (Some(dst), Some(src)) => {
                let mut rows = dst.rows();
                rows.absorb(src.rows(), |_, _| {}, |_, _| {});
                *dst = CadBrepsDelta::from_rows(rows);
            }
            (None, Some(src)) => self.breps = Some(src),
            _ => {}
        }
        if let Some(incoming) = other.references_by_model_definition_id {
            let models = self.references_by_model_definition_id.get_or_insert_with(BTreeMap::new);
            for (model, src) in incoming {
                match models.get_mut(&model) {
                    Some(dst) => {
                        let mut rows = dst.rows();
                        rows.absorb(src.rows(), patch_reference, merge_reference_patch);
                        *dst = CadReferencesDelta::from_rows(rows);
                    }
                    None => {
                        models.insert(model, src);
                    }
                }
            }
        }
        match (&mut self.nodes, other.nodes) {
            (Some(dst), Some(src)) => {
                let mut rows = dst.rows();
                rows.absorb(src.rows(), patch_node, merge_node_patch);
                *dst = CadNodesDelta::from_rows(rows);
            }
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
            drawings: self.drawings.as_ref().map(|delta| CadDrawingsDelta::from_rows(delta.rows().inverse(&base.drawings, |_, _| ()))),
            breps: self.breps.as_ref().map(|delta| CadBrepsDelta::from_rows(delta.rows().inverse(&base.breps, |_, _| ()))),
            references_by_model_definition_id: self.references_by_model_definition_id.as_ref().map(|models| {
                models
                    .iter()
                    .map(|(model, delta)| {
                        let existing = base.references_by_model_definition_id.get(model).map(Vec::as_slice).unwrap_or_default();
                        (model.clone(), CadReferencesDelta::from_rows(delta.rows().inverse(existing, invert_reference_patch)))
                    })
                    .collect()
            }),
            nodes: self.nodes.as_ref().map(|delta| CadNodesDelta::from_rows(delta.rows().inverse(&base.nodes, invert_node_patch))),
        }
    }

    fn between(base: &CadSnapshot, other: &CadSnapshot) -> Self {
        let drawings = CadDrawingsDelta::from_rows(Rows::between(&base.drawings, &other.drawings, |left, right| (left == right).then_some(None)));
        let breps = CadBrepsDelta::from_rows(Rows::between(&base.breps, &other.breps, |left, right| (left == right).then_some(None)));
        let nodes = CadNodesDelta::from_rows(Rows::between(&base.nodes, &other.nodes, compare_nodes));
        let models: BTreeMap<String, CadReferencesDelta> = base
            .references_by_model_definition_id
            .keys()
            .chain(other.references_by_model_definition_id.keys())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|model| {
                let (left, right) = (base.references_by_model_definition_id.get(model).map(Vec::as_slice).unwrap_or_default(), other.references_by_model_definition_id.get(model).map(Vec::as_slice).unwrap_or_default());
                (model.clone(), CadReferencesDelta::from_rows(Rows::between(left, right, compare_references)))
            })
            .filter(|(_, delta)| !delta.is_empty())
            .collect();
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            shape_model: (base.shape_model != other.shape_model).then(|| CadModelSlot { child: other.shape_model.clone() }),
            building_model: (base.building_model != other.building_model).then(|| CadModelSlot { child: other.building_model.clone() }),
            energy_model: (base.energy_model != other.energy_model).then(|| CadModelSlot { child: other.energy_model.clone() }),
            structure_classic_model: (base.structure_classic_model != other.structure_classic_model).then(|| CadModelSlot { child: other.structure_classic_model.clone() }),
            drawings: (!drawings.is_empty()).then_some(drawings),
            breps: (!breps.is_empty()).then_some(breps),
            references_by_model_definition_id: (!models.is_empty()).then_some(models),
            nodes: (!nodes.is_empty()).then_some(nodes),
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
