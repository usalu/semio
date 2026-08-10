//! 🔺️ Shooting artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::shooting::schema::ShootingArtifact;
use crate::artifacts::shooting::{
    ShootingAsset, ShootingAssetPatch, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingShot,
    ShootingShotPatch, ShootingSnapshot,
};
use protocol::{CollectionMutation, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


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
/// 🧩 Builds a collection delta from a [`CollectionMutation`].
pub fn assets_delta_from_collection_mutation(
    base: &[ShootingAsset],
    op: &CollectionMutation<String, ShootingAsset, ShootingAssetPatch>,
) -> ShootingAssetsDelta {
    match op {
        CollectionMutation::Add { item, .. } => ShootingAssetsDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => ShootingAssetsDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry {
                id: id.clone(),
                patch: patch.clone(),
            }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            ShootingAssetsDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

/// 🧩 Builds a shots collection delta from a [`CollectionMutation`].
pub fn shots_delta_from_collection_mutation(
    base: &[ShootingShot],
    op: &CollectionMutation<String, ShootingShot, ShootingShotPatch>,
) -> ShootingShotsDelta {
    match op {
        CollectionMutation::Add { item, .. } => ShootingShotsDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => ShootingShotsDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => ShootingShotsDelta {
            patched: vec![ShootingShotPatchEntry {
                id: id.clone(),
                patch: patch.clone(),
            }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            ShootingShotsDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

/// 🧩 Builds a saved-cameras collection delta from a [`CollectionMutation`].
pub fn saved_cameras_delta_from_collection_mutation(
    base: &[ShootingSavedCamera],
    op: &CollectionMutation<String, ShootingSavedCamera, ShootingSavedCameraPatch>,
) -> ShootingSavedCamerasDelta {
    match op {
        CollectionMutation::Add { item, .. } => ShootingSavedCamerasDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => ShootingSavedCamerasDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => ShootingSavedCamerasDelta {
            patched: vec![ShootingSavedCameraPatchEntry {
                id: id.clone(),
                patch: patch.clone(),
            }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            ShootingSavedCamerasDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        artifact: Some(Box::new(ShootingArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
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
