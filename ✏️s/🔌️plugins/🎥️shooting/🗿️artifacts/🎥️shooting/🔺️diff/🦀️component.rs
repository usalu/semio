//! 🔺️ Shooting artifact — the operation diff (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingFixture, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShot, ShootingShotPatch};
use protocol::{CollectionDiff, Identified, MutationDiff, Patchable};

//#region 🔖️CollectionSupport
/// ▶️ Applies a `CollectionDiff` (removed → modified → added, matching `apply_collection_mutation`'s
/// ordering) to an owned `Vec` — `protocol::CollectionDiff` has no generic apply helper of its own since
/// `modified` patches require the item's `Patchable` impl.
fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

/// ➕️ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}

fn apply_scene_patch(scene: &mut crate::artifacts::shooting::ShootingSceneLighting, patch: &ShootingScenePatch) {
    if let Some(value) = patch.sun_enabled {
        scene.sun.enabled = value;
    }
    if let Some(value) = patch.sun_azimuth {
        scene.sun.azimuth = value;
    }
    if let Some(value) = patch.sun_elevation {
        scene.sun.elevation = value;
    }
    if let Some(value) = patch.sun_intensity {
        scene.sun.intensity = value;
    }
    if let Some(value) = patch.ambient_intensity {
        scene.ambient.intensity = value;
    }
    if let Some(value) = patch.shadow_enabled {
        scene.shadow.enabled = value;
    }
    if let Some(value) = patch.material_roughness {
        scene.material.roughness = value;
    }
}

fn absorb_scene_patch(target: &mut Option<ShootingScenePatch>, incoming: Option<ShootingScenePatch>) {
    if let Some(b) = incoming {
        let t = target.get_or_insert_with(ShootingScenePatch::default);
        if b.sun_enabled.is_some() {
            t.sun_enabled = b.sun_enabled;
        }
        if b.sun_azimuth.is_some() {
            t.sun_azimuth = b.sun_azimuth;
        }
        if b.sun_elevation.is_some() {
            t.sun_elevation = b.sun_elevation;
        }
        if b.sun_intensity.is_some() {
            t.sun_intensity = b.sun_intensity;
        }
        if b.ambient_intensity.is_some() {
            t.ambient_intensity = b.ambient_intensity;
        }
        if b.shadow_enabled.is_some() {
            t.shadow_enabled = b.shadow_enabled;
        }
        if b.material_roughness.is_some() {
            t.material_roughness = b.material_roughness;
        }
    }
}
//#endregion 🔖️CollectionSupport

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingDiff {
    pub assets: Option<CollectionDiff<String, ShootingAssetPatch, ShootingAsset>>,
    pub shots: Option<CollectionDiff<String, ShootingShotPatch, ShootingShot>>,
    pub saved_cameras: Option<CollectionDiff<String, ShootingSavedCameraPatch, ShootingSavedCamera>>,
    pub active_shot_id: Option<String>,
    pub active_asset_id: Option<String>,
    pub scene: Option<ShootingScenePatch>,
    pub fixture: Option<ShootingFixture>,
}

impl MutationDiff<ShootingFixture> for ShootingDiff {
    fn apply(&self, projection: &ShootingFixture) -> ShootingFixture {
        if let Some(fixture) = &self.fixture {
            return fixture.clone();
        }
        let mut next = projection.clone();
        if let Some(diff) = &self.assets {
            apply_collection_diff(&mut next.assets, diff);
        }
        if let Some(diff) = &self.shots {
            apply_collection_diff(&mut next.shots, diff);
        }
        if let Some(diff) = &self.saved_cameras {
            apply_collection_diff(&mut next.saved_cameras, diff);
        }
        if let Some(id) = &self.active_shot_id {
            next.active_shot_id = id.clone();
        }
        if let Some(id) = &self.active_asset_id {
            next.active_asset_id = id.clone();
        }
        if let Some(patch) = &self.scene {
            apply_scene_patch(&mut next.scene, patch);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.fixture.is_some() {
            self.fixture = other.fixture;
            return;
        }
        absorb_collection_diff(&mut self.assets, other.assets);
        absorb_collection_diff(&mut self.shots, other.shots);
        absorb_collection_diff(&mut self.saved_cameras, other.saved_cameras);
        if other.active_shot_id.is_some() {
            self.active_shot_id = other.active_shot_id;
        }
        if other.active_asset_id.is_some() {
            self.active_asset_id = other.active_asset_id;
        }
        absorb_scene_patch(&mut self.scene, other.scene);
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// ⚖️ LAW: an empty diff is a no-operation on the projection.
    #[test]
    fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::shooting::empty_shooting_fixture();
        let diff = ShootingDiff::default();
        assert_eq!(diff.apply(&base), base);
    }
}
//#endregion 🧪️Tests
