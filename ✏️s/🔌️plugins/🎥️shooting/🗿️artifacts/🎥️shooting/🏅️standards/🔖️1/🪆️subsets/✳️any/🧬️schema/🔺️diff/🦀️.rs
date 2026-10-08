//! 🧬️ Shooting diff schema — sparse typed delta over the artifact: ordered structural edits plus keyed field patches per list,
//! a field patch for the scene, and assignable scalars. The central applier is the only caller of [`MutationDiff::apply`].

use crate::{ShootingAsset, ShootingAssetPatch, ShootingAssigned, ShootingEmblemChild, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShot, ShootingShotPatch, ShootingSnapshot};
use protocol::{ApplyCapability, DiffAlgebra, Identified, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub assets: Option<ShootingAssetsDelta>,
    #[state(artifact)]
    pub saved_cameras: Option<ShootingSavedCamerasDelta>,
    #[state(artifact)]
    pub scene: Option<ShootingScenePatch>,
    #[state(artifact)]
    pub shots: Option<ShootingShotsDelta>,
    #[state(artifact)]
    pub active_shot_id: Option<String>,
    #[state(artifact)]
    pub active_asset_id: Option<String>,
    /// 🕸️ Composed `s.stdio.semio.image` child slot: the outer `Option` says the slot was assigned, the inner whether it is now present.
    #[state(artifact)]
    pub emblem: Option<ShootingAssigned<Option<ShootingEmblemChild>>>,
}
//#endregion 🔖️Diff

//#region 🔖️Builders
/// 🏗️ Sparse single-collection diff constructors: a leaf names its edit or its keyed patches and nothing else.
impl ShootingDiff {
    /// ➕️ A diff carrying one structural edit of `assets`.
    pub fn asset_edit(edit: ShootingEdit<ShootingAsset>) -> Self {
        Self { assets: Some(ShootingAssetsDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `assets` patches, kept in id order.
    pub fn asset_patches(entries: impl IntoIterator<Item = (String, ShootingAssetPatch)>) -> Self {
        Self { assets: Some(ShootingAssetsDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }

    /// ➕️ A diff carrying one structural edit of `shots`.
    pub fn shot_edit(edit: ShootingEdit<ShootingShot>) -> Self {
        Self { shots: Some(ShootingShotsDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `shots` patches, kept in id order.
    pub fn shot_patches(entries: impl IntoIterator<Item = (String, ShootingShotPatch)>) -> Self {
        Self { shots: Some(ShootingShotsDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }

    /// ➕️ A diff carrying one structural edit of `savedCameras`.
    pub fn camera_edit(edit: ShootingEdit<ShootingSavedCamera>) -> Self {
        Self { saved_cameras: Some(ShootingSavedCamerasDelta { edits: vec![edit], patched: Vec::new() }), ..Default::default() }
    }

    /// 🩹 A diff carrying keyed `savedCameras` patches, kept in id order.
    pub fn camera_patches(entries: impl IntoIterator<Item = (String, ShootingSavedCameraPatch)>) -> Self {
        Self { saved_cameras: Some(ShootingSavedCamerasDelta { edits: Vec::new(), patched: sorted_entries(entries) }), ..Default::default() }
    }
}

fn sorted_entries<P>(entries: impl IntoIterator<Item = (String, P)>) -> Vec<ShootingPatchEntry<P>> {
    let mut rows: Vec<ShootingPatchEntry<P>> = entries.into_iter().map(|(id, patch)| ShootingPatchEntry { id, patch }).collect();
    rows.sort_by(|a, b| a.id.cmp(&b.id));
    rows
}
//#endregion 🔖️Builders

//#region 🔖️ListDelta
/// 🧩 One ordered structural edit of an identified list; `index` is the destination position in the list as it stands when the edit runs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "edit", rename_all = "camelCase")]
pub enum ShootingEdit<T> {
    #[value(rename = "add", rename_all = "camelCase")]
    Add { index: usize, item: T },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { id: String },
    #[value(rename = "move", rename_all = "camelCase")]
    Move { id: String, index: usize },
}

/// 🩹 One keyed field patch.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingPatchEntry<P> {
    pub id: String,
    pub patch: P,
}

/// 🧩 Identified-list delta: structural `edits` run in order, then each keyed `patched` entry (kept in id order, at most one per id) patches a surviving row.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingListDelta<T, P> {
    pub edits: Vec<ShootingEdit<T>>,
    pub patched: Vec<ShootingPatchEntry<P>>,
}

impl<T, P> Default for ShootingListDelta<T, P> {
    fn default() -> Self {
        Self { edits: Vec::new(), patched: Vec::new() }
    }
}

/// 🧩 Identified-collection delta for `assets`.
pub type ShootingAssetsDelta = ShootingListDelta<ShootingAsset, ShootingAssetPatch>;
/// 🧩 Identified-collection delta for `shots`.
pub type ShootingShotsDelta = ShootingListDelta<ShootingShot, ShootingShotPatch>;
/// 🧩 Identified-collection delta for `savedCameras`.
pub type ShootingSavedCamerasDelta = ShootingListDelta<ShootingSavedCamera, ShootingSavedCameraPatch>;

/// ➕️ Field-wise composition of two patches of the same row: the later value of a field wins.
pub trait ShootingPatchAlgebra: Sized {
    /// ➕️ Composes `later` over `self`.
    fn merge(&mut self, later: Self);
}

impl ShootingPatchAlgebra for ShootingAssetPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(name);
        take!(url);
        take!(format);
        take!(origin);
        take!(orientation);
        take!(scale);
    }
}

impl ShootingPatchAlgebra for ShootingShotPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(label);
        take!(width);
        take!(height);
        take!(format);
        take!(shape);
        take!(background);
        take!(camera_id);
    }
}

impl ShootingPatchAlgebra for ShootingSavedCameraPatch {
    fn merge(&mut self, later: Self) {
        macro_rules! take {
            ($field:ident) => {
                if later.$field.is_some() {
                    self.$field = later.$field;
                }
            };
        }
        take!(label);
        take!(camera);
    }
}

fn position_of<T: Identified<String>>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

impl<T, P> ShootingListDelta<T, P>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra,
{
    /// 🧬️ Runs the edits then the patches over `items`; private so that only [`MutationDiff::apply`] (the central applier's entry) reaches it.
    fn write_into(&self, items: &[T]) -> MutationApplyResult<Vec<T>> {
        let mut next = items.to_vec();
        for (row, edit) in self.edits.iter().enumerate() {
            let at = |field: &str| ["edits".to_string(), row.to_string(), field.to_string()];
            match edit {
                ShootingEdit::Add { index, item } => {
                    if position_of(&next, item.id()).is_some() {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(at("item")));
                    }
                    if *index > next.len() {
                        return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("insertion index {index} exceeds length {}", next.len())).at(at("index")));
                    }
                    next.insert(*index, item.clone());
                }
                ShootingEdit::Remove { id } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(at("id")))?;
                    next.remove(position);
                }
                ShootingEdit::Move { id, index } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "moved item does not exist").at(at("id")))?;
                    let item = next.remove(position);
                    next.insert((*index).min(next.len()), item);
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        for entry in &self.patched {
            if !seen.insert(entry.id.as_str()) {
                return Err(MutationApplyError::new("mutation.apply.duplicate-target", "item is patched more than once").at(["patched", entry.id.as_str()]));
            }
            let position = position_of(&next, &entry.id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", entry.id.as_str()]))?;
            next[position].apply_patch(&entry.patch);
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self`: adjacent edits of one row coalesce (add∘remove cancels, move∘move keeps the last, move∘remove keeps the remove) and keyed patches merge per id.
    pub fn absorb(&mut self, later: Self) {
        let removed: Vec<String> = later.edits.iter().filter_map(|edit| if let ShootingEdit::Remove { id } = edit { Some(id.clone()) } else { None }).collect();
        self.patched.retain(|entry| !removed.contains(&entry.id));
        for edit in later.edits {
            match (self.edits.last(), &edit) {
                (Some(ShootingEdit::Add { item, .. }), ShootingEdit::Remove { id }) if item.id() == id => {
                    self.edits.pop();
                    self.patched.retain(|entry| &entry.id != id);
                }
                (Some(ShootingEdit::Move { id: prior, .. }), ShootingEdit::Move { id, .. } | ShootingEdit::Remove { id }) if prior == id => {
                    self.edits.pop();
                    self.edits.push(edit);
                }
                _ => self.edits.push(edit),
            }
        }
        for entry in later.patched {
            match self.patched.iter_mut().find(|prior| prior.id == entry.id) {
                Some(prior) => prior.patch.merge(entry.patch),
                None => self.patched.push(entry),
            }
        }
        self.patched.sort_by(|a, b| a.id.cmp(&b.id));
    }
}

impl<T, P> ShootingListDelta<T, P>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    /// 🔁️ The negative delta against `base`: reversed structural edits that put every row back where it was, then patches restoring the pre-patch field values.
    pub fn negative(&self, base: &[T]) -> Self {
        let mut current = base.to_vec();
        let mut added: Vec<String> = Vec::new();
        let mut undo = Vec::new();
        for edit in &self.edits {
            match edit {
                ShootingEdit::Add { index, item } => {
                    undo.push(ShootingEdit::Remove { id: item.id().clone() });
                    added.push(item.id().clone());
                    current.insert((*index).min(current.len()), item.clone());
                }
                ShootingEdit::Remove { id } => {
                    if let Some(position) = position_of(&current, id) {
                        undo.push(ShootingEdit::Add { index: position, item: current.remove(position) });
                    }
                }
                ShootingEdit::Move { id, index } => {
                    if let Some(position) = position_of(&current, id) {
                        undo.push(ShootingEdit::Move { id: id.clone(), index: position });
                        let item = current.remove(position);
                        current.insert((*index).min(current.len()), item);
                    }
                }
            }
        }
        undo.reverse();
        let mut patched: Vec<ShootingPatchEntry<P>> = self
            .patched
            .iter()
            .filter(|entry| !added.contains(&entry.id))
            .filter_map(|entry| {
                let before = current.iter().find(|item| item.id() == &entry.id)?;
                let mut after = before.clone();
                after.apply_patch(&entry.patch);
                after.diff_patch(before).map(|patch| ShootingPatchEntry { id: entry.id.clone(), patch })
            })
            .collect();
        patched.sort_by(|a, b| a.id.cmp(&b.id));
        Self { edits: undo, patched }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import: removals, then the inserts and moves that reach `other`'s order, then keyed patches.
    pub fn between(base: &[T], other: &[T]) -> Self {
        let mut edits = Vec::new();
        for item in base {
            if position_of(other, item.id()).is_none() {
                edits.push(ShootingEdit::Remove { id: item.id().clone() });
            }
        }
        let mut order: Vec<String> = base.iter().filter(|item| position_of(other, item.id()).is_some()).map(|item| item.id().clone()).collect();
        for (index, target) in other.iter().enumerate() {
            if order.get(index) == Some(target.id()) {
                continue;
            }
            match order.iter().position(|id| id == target.id()) {
                Some(position) => {
                    let id = order.remove(position);
                    order.insert(index, id.clone());
                    edits.push(ShootingEdit::Move { id, index });
                }
                None => {
                    order.insert(index, target.id().clone());
                    edits.push(ShootingEdit::Add { index, item: target.clone() });
                }
            }
        }
        let mut patched: Vec<ShootingPatchEntry<P>> = base
            .iter()
            .filter_map(|item| {
                let counterpart = other.iter().find(|candidate| candidate.id() == item.id())?;
                item.diff_patch(counterpart).map(|patch| ShootingPatchEntry { id: item.id().clone(), patch })
            })
            .collect();
        patched.sort_by(|a, b| a.id.cmp(&b.id));
        Self { edits, patched }
    }

    /// 🕳️ Whether the delta names no edit and no patch.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.patched.is_empty()
    }
}

/// ➕️ Composes an optional list delta.
fn absorb_list<T, P>(target: &mut Option<ShootingListDelta<T, P>>, later: Option<ShootingListDelta<T, P>>)
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra,
{
    match (target.as_mut(), later) {
        (Some(prior), Some(later)) => prior.absorb(later),
        (None, Some(later)) => *target = Some(later),
        _ => {}
    }
}

/// 🔁️ Negates an optional list delta against its base list.
fn negative_list<T, P>(delta: &Option<ShootingListDelta<T, P>>, base: &[T]) -> Option<ShootingListDelta<T, P>>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    delta.as_ref().map(|delta| delta.negative(base))
}

/// 🧭️ The optional list delta between two lists, absent when they are equal.
fn between_list<T, P>(base: &[T], other: &[T]) -> Option<ShootingListDelta<T, P>>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + PartialEq,
{
    let delta = ShootingListDelta::between(base, other);
    (!delta.is_empty()).then_some(delta)
}
//#endregion 🔖️ListDelta

//#region 🔖️SceneAlgebra
macro_rules! scene_patch_algebra {
    ($($field:ident => $($path:ident).+),+ $(,)?) => {
        impl ShootingScenePatch {
            fn write_into(&self, scene: &mut crate::ShootingSceneLighting) {
                $(if let Some(value) = &self.$field {
                    scene.$($path).+ = value.clone();
                })+
            }

            /// 🔁️ The patch that restores `base` for exactly the fields this patch names.
            pub fn restoring(&self, base: &crate::ShootingSceneLighting) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$($path).+.clone()),)+ }
            }

            /// 🧭️ The patch from `from` to `to`, naming only differing fields.
            pub fn between(from: &crate::ShootingSceneLighting, to: &crate::ShootingSceneLighting) -> Self {
                Self { $($field: (from.$($path).+ != to.$($path).+).then(|| to.$($path).+.clone()),)+ }
            }

            /// ➕️ Composes `later` over this patch: the later value of a field wins.
            pub fn merge(&mut self, later: Self) {
                $(if later.$field.is_some() {
                    self.$field = later.$field;
                })+
            }

            /// 🕳️ Whether the patch names no field.
            pub fn is_empty(&self) -> bool {
                $(self.$field.is_none())&&+
            }
        }
    };
}

scene_patch_algebra! {
    background => background,
    sun_enabled => sun.enabled,
    sun_azimuth => sun.azimuth,
    sun_elevation => sun.elevation,
    sun_intensity => sun.intensity,
    sun_color => sun.color,
    ambient_intensity => ambient.intensity,
    ambient_color => ambient.color,
    shadow_enabled => shadow.enabled,
    shadow_opacity => shadow.opacity,
    shadow_softness => shadow.softness,
    material_color => material.color,
    material_metalness => material.metalness,
    material_roughness => material.roughness,
    material_emissive => material.emissive,
    material_emissive_intensity => material.emissive_intensity,
    material_stroke => material.stroke,
}
//#endregion 🔖️SceneAlgebra

//#region 🔖️Apply
impl MutationDiff<ShootingSnapshot> for ShootingDiff {
    fn apply(&self, base: &ShootingSnapshot, _capability: ApplyCapability) -> MutationApplyResult<ShootingSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.assets {
            next.assets = delta.write_into(&base.assets).map_err(|error| error.under(["assets"]))?;
        }
        if let Some(delta) = &self.saved_cameras {
            next.saved_cameras = delta.write_into(&base.saved_cameras).map_err(|error| error.under(["savedCameras"]))?;
        }
        if let Some(patch) = &self.scene {
            patch.write_into(&mut next.scene);
        }
        if let Some(delta) = &self.shots {
            next.shots = delta.write_into(&base.shots).map_err(|error| error.under(["shots"]))?;
        }
        if let Some(id) = &self.active_shot_id {
            next.active_shot_id = id.clone();
        }
        if let Some(id) = &self.active_asset_id {
            next.active_asset_id = id.clone();
        }
        if let Some(emblem) = &self.emblem {
            next.emblem = emblem.value.clone();
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
        take!(active_shot_id);
        take!(active_asset_id);
        take!(emblem);
        absorb_list(&mut self.assets, other.assets);
        absorb_list(&mut self.saved_cameras, other.saved_cameras);
        absorb_list(&mut self.shots, other.shots);
        match (self.scene.as_mut(), other.scene) {
            (Some(prior), Some(later)) => prior.merge(later),
            (None, Some(later)) => self.scene = Some(later),
            _ => {}
        }
    }
}

impl DiffAlgebra<ShootingSnapshot> for ShootingDiff {
    fn inverse(&self, base: &ShootingSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            assets: negative_list(&self.assets, &base.assets),
            saved_cameras: negative_list(&self.saved_cameras, &base.saved_cameras),
            scene: self.scene.as_ref().map(|patch| patch.restoring(&base.scene)),
            shots: negative_list(&self.shots, &base.shots),
            active_shot_id: self.active_shot_id.as_ref().map(|_| base.active_shot_id.clone()),
            active_asset_id: self.active_asset_id.as_ref().map(|_| base.active_asset_id.clone()),
            emblem: self.emblem.as_ref().map(|_| ShootingAssigned::new(base.emblem.clone())),
        }
    }

    fn between(base: &ShootingSnapshot, other: &ShootingSnapshot) -> Self {
        let scene = ShootingScenePatch::between(&base.scene, &other.scene);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            assets: between_list(&base.assets, &other.assets),
            saved_cameras: between_list(&base.saved_cameras, &other.saved_cameras),
            scene: (!scene.is_empty()).then_some(scene),
            shots: between_list(&base.shots, &other.shots),
            active_shot_id: (base.active_shot_id != other.active_shot_id).then(|| other.active_shot_id.clone()),
            active_asset_id: (base.active_asset_id != other.active_asset_id).then(|| other.active_asset_id.clone()),
            emblem: (base.emblem != other.emblem).then(|| ShootingAssigned::new(other.emblem.clone())),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
