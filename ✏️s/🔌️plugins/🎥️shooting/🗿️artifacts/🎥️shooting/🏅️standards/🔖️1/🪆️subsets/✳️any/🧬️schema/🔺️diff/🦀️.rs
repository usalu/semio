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
/// 🧩 One ordered positional edit of an identified list; every index is a position in the list as it stands when the edit runs, so a row is its own inverse recipe: `add` names where the item lands, `remove` where it stood, `move` where it came from and where it goes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "edit", rename_all = "camelCase")]
pub enum ShootingEdit<T> {
    #[value(rename = "add", rename_all = "camelCase")]
    Add { index: usize, item: T },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { id: String, index: usize },
    #[value(rename = "move", rename_all = "camelCase")]
    Move { id: String, from: usize, to: usize },
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

/// ➕️ Field algebra of one row patch over its row type: composition and the slot-wise restore read from the row it patches.
pub trait ShootingPatchAlgebra<T>: Sized {
    /// ➕️ Composes `later` over `self`: the later value of a field wins.
    fn merge(&mut self, later: Self);
    /// 🔁️ The patch that restores `row`'s value for exactly the fields `self` names.
    fn restoring(&self, row: &T) -> Self;
}

macro_rules! take_later {
    ($self:ident, $later:ident, $($field:ident),+) => {
        $(if $later.$field.is_some() {
            $self.$field = $later.$field;
        })+
    };
}

impl ShootingPatchAlgebra<ShootingAsset> for ShootingAssetPatch {
    fn merge(&mut self, later: Self) {
        take_later!(self, later, name, url, format, origin, orientation, scale);
    }

    fn restoring(&self, row: &ShootingAsset) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.name.clone()),
            url: self.url.as_ref().map(|_| row.url.clone()),
            format: self.format.as_ref().map(|_| row.format.clone()),
            origin: self.origin.map(|_| row.origin),
            orientation: self.orientation.as_ref().map(|_| ShootingAssigned::new(row.orientation)),
            scale: self.scale.as_ref().map(|_| ShootingAssigned::new(row.scale)),
        }
    }
}

impl ShootingPatchAlgebra<ShootingShot> for ShootingShotPatch {
    fn merge(&mut self, later: Self) {
        take_later!(self, later, label, width, height, format, shape, background, camera_id);
    }

    fn restoring(&self, row: &ShootingShot) -> Self {
        Self {
            label: self.label.as_ref().map(|_| row.label.clone()),
            width: self.width.map(|_| row.width),
            height: self.height.map(|_| row.height),
            format: self.format.as_ref().map(|_| row.format.clone()),
            shape: self.shape.as_ref().map(|_| row.shape.clone()),
            background: self.background.as_ref().map(|_| ShootingAssigned::new(row.background.clone())),
            camera_id: self.camera_id.as_ref().map(|_| ShootingAssigned::new(row.camera_id.clone())),
        }
    }
}

impl ShootingPatchAlgebra<ShootingSavedCamera> for ShootingSavedCameraPatch {
    fn merge(&mut self, later: Self) {
        take_later!(self, later, label, camera);
    }

    fn restoring(&self, row: &ShootingSavedCamera) -> Self {
        Self { label: self.label.as_ref().map(|_| row.label.clone()), camera: self.camera.as_ref().map(|_| row.camera.clone()) }
    }
}

fn position_of<T: Identified<String>>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

impl<T, P> ShootingListDelta<T, P>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra<T>,
{
    /// 🧬️ Runs the edits then the patches over `items`; private so that only [`MutationDiff::apply`] (the central applier's entry) reaches it.
    fn apply_rows(&self, items: &[T]) -> MutationApplyResult<Vec<T>> {
        let mut next = items.to_vec();
        for (row, edit) in self.edits.iter().enumerate() {
            let at = |field: &str| ["edits".to_string(), row.to_string(), field.to_string()];
            match edit {
                ShootingEdit::Add { index, item } => {
                    if position_of(&next, item.id()).is_some() {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-id", "added item identity already exists").at(at("item")));
                    }
                    if *index > next.len() {
                        return Err(MutationApplyError::new("mutation.apply.invalid-add-index", format!("insertion index {index} exceeds length {}", next.len())).at(at("index")));
                    }
                    next.insert(*index, item.clone());
                }
                ShootingEdit::Remove { id, index } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(at("id")))?;
                    if position != *index {
                        return Err(MutationApplyError::new("mutation.apply.order-mismatch", format!("removed item stands at {position}, not at {index}")).at(at("index")));
                    }
                    next.remove(position);
                }
                ShootingEdit::Move { id, from, to } => {
                    let position = position_of(&next, id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "moved item does not exist").at(at("id")))?;
                    if position != *from {
                        return Err(MutationApplyError::new("mutation.apply.order-mismatch", format!("moved item stands at {position}, not at {from}")).at(at("from")));
                    }
                    let item = next.remove(position);
                    if *to > next.len() {
                        return Err(MutationApplyError::new("mutation.apply.invalid-add-index", format!("move destination {to} exceeds length {}", next.len())).at(at("to")));
                    }
                    next.insert(*to, item);
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        for entry in &self.patched {
            if !seen.insert(entry.id.as_str()) {
                return Err(MutationApplyError::new("mutation.apply.duplicate-id", "item is patched more than once").at(["patched", entry.id.as_str()]));
            }
            let position = position_of(&next, &entry.id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", entry.id.as_str()]))?;
            next[position].apply_patch(&entry.patch);
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self`: adjacent edits of one row coalesce (add∘remove cancels, move∘move is one move from the first source to the last destination and vanishes when they meet, move∘remove removes from the first source) and keyed patches merge per id.
    pub fn absorb(&mut self, later: Self) {
        let removed: Vec<String> = later.edits.iter().filter_map(|edit| if let ShootingEdit::Remove { id, .. } = edit { Some(id.clone()) } else { None }).collect();
        self.patched.retain(|entry| !removed.contains(&entry.id));
        for edit in later.edits {
            loop {
                match (self.edits.last(), &edit) {
                    (Some(ShootingEdit::Add { item, .. }), ShootingEdit::Remove { id, .. }) if item.id() == id => {
                        let id = id.clone();
                        self.edits.pop();
                        self.patched.retain(|entry| entry.id != id);
                        break;
                    }
                    (Some(ShootingEdit::Move { id: prior, from, .. }), ShootingEdit::Move { id, to, .. }) if prior == id => {
                        let (id, from, to) = (id.clone(), *from, *to);
                        self.edits.pop();
                        if from != to {
                            self.edits.push(ShootingEdit::Move { id, from, to });
                        }
                        break;
                    }
                    (Some(ShootingEdit::Move { id: prior, from, .. }), ShootingEdit::Remove { id, .. }) if prior == id => {
                        let (id, index) = (id.clone(), *from);
                        self.edits.pop();
                        self.edits.push(ShootingEdit::Remove { id, index });
                        break;
                    }
                    _ => {
                        self.edits.push(edit);
                        break;
                    }
                }
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
    P: Clone + ShootingPatchAlgebra<T>,
{
    /// 🔁️ The negative delta, read row by row: every positional edit is undone by its mirror (an `add` removes at its index, a `remove` re-adds the base row at its index, a `move` swaps its ends), in reverse order, and every keyed patch is restored from the base row it patches.
    pub fn inverse_rows(&self, base: &[T]) -> Self {
        let mut edits: Vec<ShootingEdit<T>> = self
            .edits
            .iter()
            .filter_map(|edit| match edit {
                ShootingEdit::Add { index, item } => Some(ShootingEdit::Remove { id: item.id().clone(), index: *index }),
                ShootingEdit::Remove { id, index } => base.iter().find(|item| item.id() == id).map(|item| ShootingEdit::Add { index: *index, item: item.clone() }),
                ShootingEdit::Move { id, from, to } => Some(ShootingEdit::Move { id: id.clone(), from: *to, to: *from }),
            })
            .collect();
        edits.reverse();
        let added: Vec<&String> = self.edits.iter().filter_map(|edit| if let ShootingEdit::Add { item, .. } = edit { Some(item.id()) } else { None }).collect();
        let mut patched: Vec<ShootingPatchEntry<P>> = self
            .patched
            .iter()
            .filter(|entry| !added.contains(&&entry.id))
            .filter_map(|entry| base.iter().find(|item| item.id() == &entry.id).map(|row| ShootingPatchEntry { id: entry.id.clone(), patch: entry.patch.restoring(row) }))
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
    P: Clone + ShootingPatchAlgebra<T>,
{
    match (target.as_mut(), later) {
        (Some(prior), Some(later)) => prior.absorb(later),
        (None, Some(later)) => *target = Some(later),
        _ => {}
    }
}

/// 🔁️ Negates an optional list delta against its base list.
fn inverse_list<T, P>(delta: &Option<ShootingListDelta<T, P>>, base: &[T]) -> Option<ShootingListDelta<T, P>>
where
    T: Clone + Identified<String> + Patchable<P>,
    P: Clone + ShootingPatchAlgebra<T>,
{
    delta.as_ref().map(|delta| delta.inverse_rows(base))
}

//#endregion 🔖️ListDelta

//#region 🔖️SceneAlgebra
macro_rules! scene_patch_algebra {
    ($($field:ident => $($path:ident).+),+ $(,)?) => {
        impl ShootingScenePatch {
            fn apply_rows(&self, scene: &mut crate::ShootingSceneLighting) {
                $(if let Some(value) = &self.$field {
                    scene.$($path).+ = value.clone();
                })+
            }

            /// 🔁️ The patch that restores `base` for exactly the fields this patch names.
            pub fn restoring(&self, base: &crate::ShootingSceneLighting) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$($path).+.clone()),)+ }
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
            next.assets = delta.apply_rows(&base.assets).map_err(|error| error.under(["assets"]))?;
        }
        if let Some(delta) = &self.saved_cameras {
            next.saved_cameras = delta.apply_rows(&base.saved_cameras).map_err(|error| error.under(["savedCameras"]))?;
        }
        if let Some(patch) = &self.scene {
            patch.apply_rows(&mut next.scene);
        }
        if let Some(delta) = &self.shots {
            next.shots = delta.apply_rows(&base.shots).map_err(|error| error.under(["shots"]))?;
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
            assets: inverse_list(&self.assets, &base.assets),
            saved_cameras: inverse_list(&self.saved_cameras, &base.saved_cameras),
            scene: self.scene.as_ref().map(|patch| patch.restoring(&base.scene)),
            shots: inverse_list(&self.shots, &base.shots),
            active_shot_id: self.active_shot_id.as_ref().map(|_| base.active_shot_id.clone()),
            active_asset_id: self.active_asset_id.as_ref().map(|_| base.active_asset_id.clone()),
            emblem: self.emblem.as_ref().map(|_| ShootingAssigned::new(base.emblem.clone())),
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
