//! 🔺️ Shooting artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::shooting::schema::ShootingArtifact;
use crate::artifacts::shooting::{ShootingAsset, ShootingSavedCamera, ShootingShot, ShootingSnapshot};
use protocol::{MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::shooting::schema::diff::*;


//#region 🔖️Apply
/// 🧩 Applies an identified-collection delta to an asset list.
pub async fn apply_assets_delta(items: &[ShootingAsset], delta: &ShootingAssetsDelta) -> protocol::MutationApplyResult<Vec<ShootingAsset>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingAssetPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

/// 🧩 Applies an identified-collection delta to a shot list.
pub async fn apply_shots_delta(items: &[ShootingShot], delta: &ShootingShotsDelta) -> protocol::MutationApplyResult<Vec<ShootingShot>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingShotPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

/// 🧩 Applies an identified-collection delta to a saved-camera list.
pub async fn apply_saved_cameras_delta(
    items: &[ShootingSavedCamera],
    delta: &ShootingSavedCamerasDelta,
) -> protocol::MutationApplyResult<Vec<ShootingSavedCamera>> {
    apply_identified_delta(
        items,
        &delta.removed,
        &delta.added,
        &delta.patched,
        delta.reordered.as_ref(),
        |entry: &ShootingSavedCameraPatchEntry| (&entry.id, &entry.patch),
    )
}

async fn apply_identified_delta<T, P, E, F>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[E],
    reordered: Option<&Vec<String>>,
    entry_parts: F,
) -> protocol::MutationApplyResult<Vec<T>>
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
        let position = next.iter().position(|item| item.id() == id).ok_or_else(|| {
            protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed", id.as_str()])
        })?;
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
        let item = next.iter_mut().find(|item| item.id() == id).ok_or_else(|| {
            protocol::MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", id.as_str()])
        })?;
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
            let position = next.iter().position(|item| item.id() == id).ok_or_else(|| {
                protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()])
            })?;
            ordered.push(next.remove(position));
        }
        next = ordered;
    }
    Ok(next)
}

async fn absorb_assets_delta(target: &mut Option<ShootingAssetsDelta>, incoming: Option<ShootingAssetsDelta>) {
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

async fn absorb_shots_delta(target: &mut Option<ShootingShotsDelta>, incoming: Option<ShootingShotsDelta>) {
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

async fn absorb_saved_cameras_delta(
    target: &mut Option<ShootingSavedCamerasDelta>,
    incoming: Option<ShootingSavedCamerasDelta>,
) {
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
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &ShootingArtifact) -> protocol::MutationApplyResult<ShootingArtifact> {
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
            if let Some(list) = &self.selected_shot_ids {
                next.selected_shot_ids = list.values.clone();
            }
            if let Some(value) = &self.active_utility_id {
                next.active_utility_id = value.clone();
            }
            if let Some(value) = &self.default_shot_format {
                next.default_shot_format = value.clone();
            }
            if let Some(value) = &self.default_shot_shape {
                next.default_shot_shape = value.clone();
            }
            if let Some(value) = &self.default_asset_format {
                next.default_asset_format = value.clone();
            }
            if let Some(value) = self.center_model {
                next.center_model = value;
            }
            if let Some(value) = self.fit_revision {
                next.fit_revision = value;
            }
            if let Some(value) = &self.camera_draft_label {
                next.camera_draft_label = value.clone();
            }
            if let Some(value) = &self.camera {
                next.camera = value.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<ShootingSnapshot> for ShootingDiff {
    async fn apply(&self, snapshot: &ShootingSnapshot) -> protocol::MutationApplyResult<ShootingSnapshot> {
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
    async fn absorb(&mut self, other: Self) {
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
        take!(selected_shot_ids);
        take!(active_utility_id);
        take!(default_shot_format);
        take!(default_shot_shape);
        take!(default_asset_format);
        take!(center_model);
        take!(fit_revision);
        take!(camera_draft_label);
        take!(camera);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🗑️ The pre-migration `*_delta_from_collection_mutation` helpers and `diff_set_snapshot` lived
// here to serve the generic option-bag-collection/whole-document-replace dispatch that `../../🧬️mutations`
// deleted outright (banned per `📓️taxonomy.md`'s "Forbidden vocabulary" — whole-document replace
// has no in-history mutation, see `ArtifactStore::reset`). Every triad leaf under `🧬️mutations/`
// now builds its `ShootingAssetsDelta`/`ShootingShotsDelta`/`ShootingSavedCamerasDelta` sparsely
// and directly from its own payload instead.
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// ⚖️ LAW: an empty diff is a no-operation on the snapshot.
    #[semio_framework_async_macros::async_test]
    async fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::shooting::empty_shooting_snapshot();
        let diff = ShootingDiff::default();
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    }
}
//#endregion 🧪️Tests
