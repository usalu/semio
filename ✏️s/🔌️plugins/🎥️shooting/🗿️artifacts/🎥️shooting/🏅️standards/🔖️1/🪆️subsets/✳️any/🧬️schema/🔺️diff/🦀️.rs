//! 🧬️ Shooting diff schema — sparse field delta over the artifact.

use crate::{ShootingAsset, ShootingEmblemChild, ShootingSavedCamera, ShootingSceneLighting, ShootingShot};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::ShootingArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub assets: Option<ShootingAssetsDelta>,
    #[state(artifact)]
    pub saved_cameras: Option<ShootingSavedCamerasDelta>,
    #[state(artifact)]
    pub scene: Option<ShootingSceneLighting>,
    #[state(artifact)]
    pub shots: Option<ShootingShotsDelta>,
    #[state(artifact)]
    pub active_shot_id: Option<String>,
    #[state(artifact)]
    pub active_asset_id: Option<String>,
    /// 🕸️ Composed `s.stdio.semio.image` child slot. Double-`Option` per the migration recipe's
    /// "optional slot" diff convention: outer = did the presence/identity change, inner = is it now
    /// present. No mutation triad currently sets this (see the artifact root's `🔖️Composition`
    /// doc comment) — present for schema completeness and future writers.
    #[state(artifact)]
    pub emblem: Option<Option<ShootingEmblemChild>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `assets`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingAssetsDelta {
    pub added: Vec<ShootingAsset>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingAssetPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `shots`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingShotsDelta {
    pub added: Vec<ShootingShot>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingShotPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `savedCameras`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingSavedCamerasDelta {
    pub added: Vec<ShootingSavedCamera>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingSavedCameraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched asset entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingAssetPatchEntry {
    pub id: String,
    pub patch: ShootingAssetPatch,
}

/// 🩹 One patched shot entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingShotPatchEntry {
    pub id: String,
    pub patch: ShootingShotPatch,
}

/// 🩹 One patched saved-camera entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingSavedCameraPatchEntry {
    pub id: String,
    pub patch: ShootingSavedCameraPatch,
}
//#endregion 🔖️DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::ShootingAssetPatch;
pub use crate::ShootingSavedCameraPatch;
pub use crate::ShootingShotPatch;
//#endregion 🔁️Re-exports

use crate::standards::v1::subsets::any::schema::ShootingArtifact;
use crate::ShootingSnapshot;
use protocol::MutationDiff;
use protocol::Patchable;

/// 🧩 Applies an identified-collection delta to an asset list.
pub fn apply_assets_delta(items: &[ShootingAsset], delta: &ShootingAssetsDelta) -> protocol::MutationApplyResult<Vec<ShootingAsset>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingAssetPatchEntry| (&entry.id, &entry.patch))
}

/// 🧩 Applies an identified-collection delta to a shot list.
pub fn apply_shots_delta(items: &[ShootingShot], delta: &ShootingShotsDelta) -> protocol::MutationApplyResult<Vec<ShootingShot>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingShotPatchEntry| (&entry.id, &entry.patch))
}

/// 🧩 Applies an identified-collection delta to a saved-camera list.
pub fn apply_saved_cameras_delta(items: &[ShootingSavedCamera], delta: &ShootingSavedCamerasDelta) -> protocol::MutationApplyResult<Vec<ShootingSavedCamera>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingSavedCameraPatchEntry| (&entry.id, &entry.patch))
}

fn apply_identified_delta<T, P, E, F>(items: &[T], removed: &[String], added: &[T], patched: &[E], reordered: Option<&Vec<String>>, entry_parts: F) -> protocol::MutationApplyResult<Vec<T>>
where
    T: Clone + protocol::Identified<String> + Patchable<P>,
    P: Clone,
    F: Fn(&E) -> (&String, &P),
{
    let mut next = items.to_vec();
    let mut seen = std::collections::HashSet::new();
    for id in removed {
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed", id.as_str()]));
        }
        let position = next.iter().position(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed", id.as_str()]))?;
        next.remove(position);
    }
    seen.clear();
    for item in added {
        let id = item.id();
        if !seen.insert(id.clone()) || next.iter().any(|entry| entry.id() == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(["added", id.as_str()]));
        }
        next.push(item.clone());
    }
    seen.clear();
    for entry in patched {
        let (id, patch) = entry_parts(entry);
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is patched more than once").at(["patched", id.as_str()]));
        }
        let item = next.iter_mut().find(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", id.as_str()]))?;
        item.apply_patch(patch);
    }
    if let Some(order) = reordered {
        if order.len() != next.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), next.len())).at(["reordered"]));
        }
        seen.clear();
        for id in order {
            if !seen.insert(id.clone()) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item appears more than once in order").at(["reordered", id.as_str()]));
            }
            if !next.iter().any(|item| item.id() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]));
            }
        }
        let mut ordered = Vec::with_capacity(next.len());
        for id in order {
            let position = next.iter().position(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]))?;
            ordered.push(next.remove(position));
        }
        next = ordered;
    }
    Ok(next)
}

fn absorb_assets_delta(target: &mut Option<ShootingAssetsDelta>, incoming: Option<ShootingAssetsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_shots_delta(target: &mut Option<ShootingShotsDelta>, incoming: Option<ShootingShotsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_saved_cameras_delta(target: &mut Option<ShootingSavedCamerasDelta>, incoming: Option<ShootingSavedCamerasDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl ShootingDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &ShootingArtifact) -> protocol::MutationApplyResult<ShootingArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(delta) = &self.assets {
                next.assets = apply_assets_delta(&next.assets, delta).map_err(|error| error.under(["assets"]))?;
            }
            if let Some(delta) = &self.saved_cameras {
                next.saved_cameras = apply_saved_cameras_delta(&next.saved_cameras, delta).map_err(|error| error.under(["savedCameras"]))?;
            }
            if let Some(scene) = &self.scene {
                next.scene = scene.clone();
            }
            if let Some(delta) = &self.shots {
                next.shots = apply_shots_delta(&next.shots, delta).map_err(|error| error.under(["shots"]))?;
            }
            if let Some(id) = &self.active_shot_id {
                next.active_shot_id = id.clone();
            }
            if let Some(id) = &self.active_asset_id {
                next.active_asset_id = id.clone();
            }
            if let Some(value) = &self.emblem {
                next.emblem = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<ShootingSnapshot> for ShootingDiff {
    fn apply(&self, snapshot: &ShootingSnapshot) -> protocol::MutationApplyResult<ShootingSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(delta) = &self.assets {
                next.assets = apply_assets_delta(&next.assets, delta).map_err(|error| error.under(["assets"]))?;
            }
            if let Some(delta) = &self.saved_cameras {
                next.saved_cameras = apply_saved_cameras_delta(&next.saved_cameras, delta).map_err(|error| error.under(["savedCameras"]))?;
            }
            if let Some(scene) = &self.scene {
                next.scene = scene.clone();
            }
            if let Some(delta) = &self.shots {
                next.shots = apply_shots_delta(&next.shots, delta).map_err(|error| error.under(["shots"]))?;
            }
            if let Some(id) = &self.active_shot_id {
                next.active_shot_id = id.clone();
            }
            if let Some(id) = &self.active_asset_id {
                next.active_asset_id = id.clone();
            }
            if let Some(value) = &self.emblem {
                next.emblem = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_assets_delta(&mut self.assets, other.assets);
        absorb_shots_delta(&mut self.shots, other.shots);
        absorb_saved_cameras_delta(&mut self.saved_cameras, other.saved_cameras);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(scene);
        take!(active_shot_id);
        take!(active_asset_id);
        take!(emblem);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
