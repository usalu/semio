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
pub fn apply_assets_delta(items: &[ShootingAsset], delta: &ShootingAssetsDelta) -> Vec<ShootingAsset> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingAssetPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

/// 🧩 Applies an identified-collection delta to a shot list.
pub fn apply_shots_delta(items: &[ShootingShot], delta: &ShootingShotsDelta) -> Vec<ShootingShot> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &ShootingShotPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

/// 🧩 Applies an identified-collection delta to a saved-camera list.
pub fn apply_saved_cameras_delta(
    items: &[ShootingSavedCamera],
    delta: &ShootingSavedCamerasDelta,
) -> Vec<ShootingSavedCamera> {
    apply_identified_delta(
        items,
        &delta.removed,
        &delta.added,
        &delta.patched,
        delta.reordered.as_ref(),
        |entry: &ShootingSavedCameraPatchEntry| (&entry.id, &entry.patch),
    )
}

fn apply_identified_delta<T, P, E, F>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[E],
    reordered: Option<&Vec<String>>,
    entry_parts: F,
) -> Vec<T>
where
    T: Clone + protocol::Identified<String> + Patchable<P>,
    P: Clone,
    F: Fn(&E) -> (&String, &P),
{
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| item.id() != id);
    }
    for item in added {
        next.push(item.clone());
    }
    for entry in patched {
        let (id, patch) = entry_parts(entry);
        if let Some(item) = next.iter_mut().find(|item| item.id() == id) {
            item.apply_patch(patch);
        }
    }
    if let Some(order) = reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id().clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
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

fn absorb_saved_cameras_delta(
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
    pub fn apply_to_artifact(&self, artifact: &ShootingArtifact) -> ShootingArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.assets {
            next.assets = apply_assets_delta(&next.assets, delta);
        }
        if let Some(delta) = &self.saved_cameras {
            next.saved_cameras = apply_saved_cameras_delta(&next.saved_cameras, delta);
        }
        if let Some(scene) = &self.scene {
            next.scene = scene.clone();
        }
        if let Some(delta) = &self.shots {
            next.shots = apply_shots_delta(&next.shots, delta);
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
        if let Some(list) = &self.selected_asset_ids {
            next.selected_asset_ids = list.values.clone();
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
        if let Some(value) = &self.selection_method {
            next.selection_method = value.clone();
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
        if let Some(value) = &self.hovered_asset_id {
            next.hovered_asset_id = value.clone();
        }
        next
    }
}

impl MutationDiff<ShootingSnapshot> for ShootingDiff {
    fn apply(&self, snapshot: &ShootingSnapshot) -> ShootingSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.assets {
            next.assets = apply_assets_delta(&next.assets, delta);
        }
        if let Some(delta) = &self.saved_cameras {
            next.saved_cameras = apply_saved_cameras_delta(&next.saved_cameras, delta);
        }
        if let Some(scene) = &self.scene {
            next.scene = scene.clone();
        }
        if let Some(delta) = &self.shots {
            next.shots = apply_shots_delta(&next.shots, delta);
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
        take!(selected_shot_ids);
        take!(selected_asset_ids);
        take!(active_utility_id);
        take!(default_shot_format);
        take!(default_shot_shape);
        take!(default_asset_format);
        take!(selection_method);
        take!(center_model);
        take!(fit_revision);
        take!(camera_draft_label);
        take!(camera);
        take!(locale);
        take!(hovered_asset_id);
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
    #[test]
    fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::shooting::empty_shooting_snapshot();
        let diff = ShootingDiff::default();
        assert_eq!(diff.apply(&base), base);
    }
}
//#endregion 🧪️Tests
