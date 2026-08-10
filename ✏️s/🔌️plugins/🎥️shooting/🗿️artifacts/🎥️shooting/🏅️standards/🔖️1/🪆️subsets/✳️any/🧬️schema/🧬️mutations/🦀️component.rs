//! 🧬️ Shooting artifact — document mutation dispatch enum.

//! (constitutional: op + protocol's `Command`-adjacent op codec half).
//!
//! `protocol::OpText`/`protocol::OpBinary` for [`ShootingMutation`] can't be derived directly on the
//! enum (`Assets`/`Shots`/`SavedCameras` each wrap a foreign generic `protocol::CollectionMutation<..>`
//! — orphan rule, and not the tagged-enum shape `#[derive(dsl::DslEnum)]` needs anyway), so this file also
//! owns the private `ShootingMutationDsl` mirror that flattens each `CollectionMutation` variant into
//! its own DSL-facing operation variant — the `imperative::ImperativeOperationDsl` idiom. `📡️spr`'s
//! `encode_op`/`decode_op` are thin forwards onto the `OpBinary` impl defined here.


use crate::artifacts::shooting::diff::{
    assets_delta_from_collection_mutation, diff_set_snapshot, saved_cameras_delta_from_collection_mutation,
    shots_delta_from_collection_mutation, ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff,
};
use crate::artifacts::shooting::{
    ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingSavedCamera, ShootingSavedCameraPatch,
    ShootingSceneLighting, ShootingScenePatch, ShootingShot, ShootingShotPatch, ShootingSnapshot,
};
use protocol::{CollectionMutation, Mutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant, reason = "SetSnapshot.snapshot carries the whole document, same shape as the pre-migration enum; boxing is a separate concern from this migration")]
pub enum ShootingMutation {
    Assets(CollectionMutation<String, ShootingAsset, ShootingAssetPatch>),
    Shots(CollectionMutation<String, ShootingShot, ShootingShotPatch>),
    SavedCameras(CollectionMutation<String, ShootingSavedCamera, ShootingSavedCameraPatch>),
    SetActiveShot {
        shot_id: Option<String>,
    },
    SetActiveAsset {
        asset_id: Option<String>,
    },
    /// 🎥️ Patches the saved camera `shot_id` references with `camera` — a no-op (empty diff) when that
    /// shot has no saved camera. The free/live viewport camera is session-only runtime state now (see
    /// `ShootingConfig::camera` in the app's `🦀️config.rs`) and never reaches this op enum at all.
    SetShotCamera {
        shot_id: String,
        camera: ShootingCamera,
    },
    PatchScene {
        patch: ShootingScenePatch,
    },
    TranslateAssets {
        asset_ids: Vec<String>,
        dx: f64,
        dy: f64,
        dz: f64,
    },
    RotateAssets {
        asset_ids: Vec<String>,
        ax: f64,
        ay: f64,
        az: f64,
        angle: f64,
    },
    ScaleAssets {
        asset_ids: Vec<String>,
        sx: f64,
        sy: f64,
        sz: f64,
    },
    SetSnapshot {
        snapshot: ShootingSnapshot,
    },
}

fn reverse_scene_patch(before: &crate::artifacts::shooting::ShootingSceneLighting, patch: &ShootingScenePatch) -> ShootingScenePatch {
    ShootingScenePatch {
        sun_enabled: patch.sun_enabled.map(|_| before.sun.enabled),
        sun_azimuth: patch.sun_azimuth.map(|_| before.sun.azimuth),
        sun_elevation: patch.sun_elevation.map(|_| before.sun.elevation),
        sun_intensity: patch.sun_intensity.map(|_| before.sun.intensity),
        ambient_intensity: patch.ambient_intensity.map(|_| before.ambient.intensity),
        shadow_enabled: patch.shadow_enabled.map(|_| before.shadow.enabled),
        material_roughness: patch.material_roughness.map(|_| before.material.roughness),
    }
}

/// 🎯️ Resolves which `SavedCameras` entry (if any) `shot_id` targets, for `SetShotCamera` diffing: a
/// shot referencing a saved camera patches that entry; a shot with no saved camera has nothing for
/// `SetShotCamera` to touch (the free/live viewport camera is session-only runtime state — see
/// `ShootingConfig::camera` — never a document field).
fn resolve_camera_target(snapshot: &ShootingSnapshot, shot_id: &str) -> Option<String> {
    snapshot.shots.iter().find(|shot| shot.id == shot_id).and_then(|shot| shot.camera_id.clone())
}

fn camera_diff_for_shot(snapshot: &ShootingSnapshot, shot_id: &str, camera: &ShootingCamera) -> ShootingDiff {
    match resolve_camera_target(snapshot, shot_id) {
        Some(camera_id) => ShootingDiff {
            saved_cameras: Some(crate::artifacts::shooting::diff::ShootingSavedCamerasDelta {
                patched: vec![crate::artifacts::shooting::diff::ShootingSavedCameraPatchEntry {
                    id: camera_id,
                    patch: ShootingSavedCameraPatch { label: None, camera: Some(camera.clone()) },
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        None => ShootingDiff::default(),
    }
}

fn camera_for_shot(snapshot: &ShootingSnapshot, shot_id: &str) -> Option<ShootingCamera> {
    let camera_id = resolve_camera_target(snapshot, shot_id)?;
    snapshot.saved_cameras.iter().find(|entry| entry.id == camera_id).map(|entry| entry.camera.clone())
}

fn apply_scene_patch(scene: &ShootingSceneLighting, patch: &ShootingScenePatch) -> ShootingSceneLighting {
    let mut next = scene.clone();
    if let Some(value) = patch.sun_enabled {
        next.sun.enabled = value;
    }
    if let Some(value) = patch.sun_azimuth {
        next.sun.azimuth = value;
    }
    if let Some(value) = patch.sun_elevation {
        next.sun.elevation = value;
    }
    if let Some(value) = patch.sun_intensity {
        next.sun.intensity = value;
    }
    if let Some(value) = patch.ambient_intensity {
        next.ambient.intensity = value;
    }
    if let Some(value) = patch.shadow_enabled {
        next.shadow.enabled = value;
    }
    if let Some(value) = patch.material_roughness {
        next.material.roughness = value;
    }
    next
}

fn transform_assets_diff(snapshot: &ShootingSnapshot, asset_ids: &[String], patch_for: impl Fn(&ShootingAsset) -> ShootingAssetPatch) -> ShootingDiff {
    let patched: Vec<ShootingAssetPatchEntry> = snapshot
        .assets
        .iter()
        .filter(|asset| asset_ids.contains(&asset.id))
        .map(|asset| ShootingAssetPatchEntry { id: asset.id.clone(), patch: patch_for(asset) })
        .collect();
    if patched.is_empty() {
        return ShootingDiff::default();
    }
    ShootingDiff {
        assets: Some(ShootingAssetsDelta { patched, ..Default::default() }),
        ..Default::default()
    }
}


/// @emoji ▶️ Applies one mutation to the fixture in place.
pub fn apply_shooting_mutation(snapshot: &mut ShootingSnapshot, mutation: &ShootingMutation) {
    match mutation {
        ShootingMutation::Assets(m) => super::assets::mutation::apply(snapshot, m),
        ShootingMutation::Shots(m) => super::shots::mutation::apply(snapshot, m),
        ShootingMutation::SavedCameras(m) => super::saved_cameras::mutation::apply(snapshot, m),
        other => {
            let diff = <ShootingMutation as Mutation<ShootingSnapshot>>::diff(other, snapshot);
            *snapshot = MutationDiff::apply(&diff, snapshot);
        }
    }
}

/// @emoji ↩️ Inverse mutations from pre-state.
pub fn inverse_shooting_mutation(base: &ShootingSnapshot, mutation: &ShootingMutation) -> Vec<ShootingMutation> {
    <ShootingMutation as Mutation<ShootingSnapshot>>::inverse(mutation, base)
}

impl Mutation<ShootingSnapshot> for ShootingMutation {
    type Diff = ShootingDiff;

    fn diff(&self, snapshot: &ShootingSnapshot) -> ShootingDiff {
        match self {
            ShootingMutation::Assets(operation) => ShootingDiff {
                assets: Some(assets_delta_from_collection_mutation(&snapshot.assets, operation)),
                ..Default::default()
            },
            ShootingMutation::Shots(operation) => ShootingDiff {
                shots: Some(shots_delta_from_collection_mutation(&snapshot.shots, operation)),
                ..Default::default()
            },
            ShootingMutation::SavedCameras(operation) => ShootingDiff {
                saved_cameras: Some(saved_cameras_delta_from_collection_mutation(&snapshot.saved_cameras, operation)),
                ..Default::default()
            },
            ShootingMutation::SetActiveShot { shot_id } => ShootingDiff {
                active_shot_id: Some(shot_id.clone().unwrap_or_default()),
                ..Default::default()
            },
            ShootingMutation::SetActiveAsset { asset_id } => ShootingDiff {
                active_asset_id: Some(asset_id.clone().unwrap_or_default()),
                ..Default::default()
            },
            ShootingMutation::SetShotCamera { shot_id, camera } => camera_diff_for_shot(snapshot, shot_id, camera),
            ShootingMutation::PatchScene { patch } => ShootingDiff {
                scene: Some(apply_scene_patch(&snapshot.scene, patch)),
                ..Default::default()
            },
            ShootingMutation::TranslateAssets { asset_ids, dx, dy, dz } => {
                transform_assets_diff(snapshot, asset_ids, |asset| ShootingAssetPatch {
                    origin: Some([asset.origin[0] + dx, asset.origin[1] + dy, asset.origin[2] + dz]),
                    ..Default::default()
                })
            }
            ShootingMutation::RotateAssets { asset_ids, ax, ay, az, angle } => {
                let delta = crate::artifacts::shooting::quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_assets_diff(snapshot, asset_ids, |asset| {
                    let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    ShootingAssetPatch {
                        orientation: Some(crate::artifacts::shooting::quat_mul(delta, current)),
                        ..Default::default()
                    }
                })
            }
            ShootingMutation::ScaleAssets { asset_ids, sx, sy, sz } => transform_assets_diff(snapshot, asset_ids, |asset| {
                let current = crate::artifacts::shooting::shooting_asset_scale(asset);
                ShootingAssetPatch {
                    scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]),
                    ..Default::default()
                }
            }),
            ShootingMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &ShootingSnapshot) -> Vec<Self> {
        match self {
            ShootingMutation::Assets(operation) => super::assets::inverse::inverse(snapshot, operation),
            ShootingMutation::Shots(operation) => super::shots::inverse::inverse(snapshot, operation),
            ShootingMutation::SavedCameras(operation) => super::saved_cameras::inverse::inverse(snapshot, operation),
            ShootingMutation::SetActiveShot { .. } => vec![ShootingMutation::SetActiveShot { shot_id: if snapshot.active_shot_id.is_empty() { None } else { Some(snapshot.active_shot_id.clone()) } }],
            ShootingMutation::SetActiveAsset { .. } => vec![ShootingMutation::SetActiveAsset { asset_id: if snapshot.active_asset_id.is_empty() { None } else { Some(snapshot.active_asset_id.clone()) } }],
            ShootingMutation::SetShotCamera { shot_id, .. } => camera_for_shot(snapshot, shot_id).map(|camera| vec![ShootingMutation::SetShotCamera { shot_id: shot_id.clone(), camera }]).unwrap_or_default(),
            ShootingMutation::PatchScene { patch } => vec![ShootingMutation::PatchScene { patch: reverse_scene_patch(&snapshot.scene, patch) }],
            ShootingMutation::TranslateAssets { asset_ids, dx, dy, dz } => vec![ShootingMutation::TranslateAssets { asset_ids: asset_ids.clone(), dx: -dx, dy: -dy, dz: -dz }],
            ShootingMutation::RotateAssets { asset_ids, ax, ay, az, angle } => vec![ShootingMutation::RotateAssets { asset_ids: asset_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: -angle }],
            ShootingMutation::ScaleAssets { asset_ids, sx, sy, sz } => {
                let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
                vec![ShootingMutation::ScaleAssets { asset_ids: asset_ids.clone(), sx: inv(*sx), sy: inv(*sy), sz: inv(*sz) }]
            }
            ShootingMutation::SetSnapshot { .. } => vec![ShootingMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA;

    fn sample_asset(id: &str) -> ShootingAsset {
        ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None }
    }

    fn sample_shot(id: &str) -> ShootingShot {
        ShootingShot { id: id.into(), label: format!("Shot {id}"), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None }
    }

    fn round_trip(snapshot: &ShootingSnapshot, operation: &ShootingMutation) -> ShootingSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let backwards = operation.inverse(snapshot);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_mutation(&restored, back);
        }
        assert_eq!(&restored, snapshot, "backwards() must exactly restore the pre-operation fixture");
        forward
    }

    /// 🎞️ A fixture exercising every field/variant — duplicated verbatim across the `dsl`/`op`/`pack`
    /// crates' worth of tests (each is its own compilation unit, so a shared cross-crate test-only
    /// helper isn't worth a dependency).
    #[allow(clippy::approx_constant, reason = "0.7071 is deliberately an approximate quaternion component in this snapshot, not the FRAC_1_SQRT_2 constant")]
    fn representative_snapshot() -> ShootingSnapshot {
        ShootingSnapshot {
            schema: SHOOTING_DOCUMENT_SCHEMA.into(),
            assets: vec![
                ShootingAsset { id: "a1".into(), name: "Base \"Mesh\"".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [1.0, 2.0, 3.0], orientation: Some([0.0, 0.0, 0.7071, 0.7071]), scale: Some([2.0, 2.0, 2.0]) },
                ShootingAsset { id: "a2".into(), name: "Plain".into(), url: "/mesh/a2.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None },
            ],
            saved_cameras: vec![ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() } }],
            scene: crate::artifacts::shooting::ShootingSceneLighting {
                background: "#111111".into(),
                sun: crate::artifacts::shooting::ShootingSun { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 3.0, color: "#ff00ff".into() },
                ambient: crate::artifacts::shooting::ShootingAmbient { intensity: 0.9, color: "#00ffff".into() },
                shadow: crate::artifacts::shooting::ShootingShadow { enabled: false, opacity: 0.5, softness: 0.2 },
                material: crate::artifacts::shooting::ShootingMaterial { color: "#abcdef".into(), metalness: 0.3, roughness: 0.7, emissive: "#123456".into(), emissive_intensity: 0.1 },
                emblem_base64: Some("data:image/png;base64,abc==".into()),
            },
            shots: vec![
                ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()) },
                ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None },
            ],
            active_shot_id: "s1".into(),
            active_asset_id: "a1".into(),
        }
    }

    #[test]
    fn assets_add_remove_patch_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let add = ShootingMutation::Assets(CollectionMutation::Add { index: 0, item: sample_asset("a1") });
        let with_asset = round_trip(&snapshot, &add);
        assert_eq!(with_asset.assets.len(), 1);

        let patch = ShootingMutation::Assets(CollectionMutation::Patch { id: "a1".into(), patch: ShootingAssetPatch { name: Some("Renamed".into()), ..Default::default() } });
        let patched = round_trip(&with_asset, &patch);
        assert_eq!(patched.assets[0].name, "Renamed");

        let remove = ShootingMutation::Assets(CollectionMutation::Remove { id: "a1".into() });
        let removed = round_trip(&patched, &remove);
        assert!(removed.assets.is_empty());
    }

    #[test]
    fn shots_patch_round_trip() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots.push(sample_shot("s1"));
        let patch = ShootingMutation::Shots(CollectionMutation::Patch { id: "s1".into(), patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), ..Default::default() } });
        let patched = round_trip(&snapshot, &patch);
        assert_eq!(patched.shots[0].label, "Hero");
        assert_eq!(patched.shots[0].width, 512);
    }

    #[test]
    fn saved_cameras_add_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let add = ShootingMutation::SavedCameras(CollectionMutation::Add { index: 0, item: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() } });
        let added = round_trip(&snapshot, &add);
        assert_eq!(added.saved_cameras.len(), 1);
    }

    #[test]
    fn set_active_shot_and_asset_round_trip() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots.push(sample_shot("s1"));
        snapshot.assets.push(sample_asset("a1"));
        let operation = ShootingMutation::SetActiveShot { shot_id: Some("s1".into()) };
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next.active_shot_id, "s1");
        let operation = ShootingMutation::SetActiveAsset { asset_id: Some("a1".into()) };
        let next2 = round_trip(&next, &operation);
        assert_eq!(next2.active_asset_id, "a1");
    }

    #[test]
    fn set_shot_camera_is_a_no_op_when_shot_has_no_saved_camera() {
        // 🎥️ The free/live viewport camera is session-only runtime state now (never a document field) —
        // `SetShotCamera` against a shot with no saved-camera reference has nothing to patch.
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots.push(sample_shot("s1"));
        let camera = ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() };
        let operation = ShootingMutation::SetShotCamera { shot_id: "s1".into(), camera };
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next, snapshot, "no saved camera referenced by the shot means no document change");
    }

    #[test]
    fn set_shot_camera_patches_the_saved_camera_it_references() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.saved_cameras.push(ShootingSavedCamera { id: "cam1".into(), label: "A".into(), camera: ShootingCamera::default() });
        let mut shot = sample_shot("s1");
        shot.camera_id = Some("cam1".into());
        snapshot.shots.push(shot);
        snapshot.active_shot_id = "s1".into();
        let camera = ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() };
        let operation = ShootingMutation::SetShotCamera { shot_id: "s1".into(), camera };
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
    }

    #[test]
    fn patch_scene_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let operation = ShootingMutation::PatchScene { patch: ShootingScenePatch { sun_azimuth: Some(90.0), shadow_enabled: Some(false), ..Default::default() } };
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next.scene.sun.azimuth, 90.0);
        assert!(!next.scene.shadow.enabled);
    }

    #[test]
    fn translate_rotate_scale_assets_round_trip() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let mut asset = sample_asset("a1");
        // ScaleAssets always writes an explicit `Some([..])` scale, so backwards() restoring an
        // originally-`None` scale lands on `Some([1,1,1])` — the same effective scale (see
        // `shooting_asset_scale`) but not byte-identical. Start from an explicit identity scale so
        // the round-trip assertion checks real equality instead of that representation quirk.
        asset.scale = Some([1.0, 1.0, 1.0]);
        snapshot.assets.push(asset);
        let translate = ShootingMutation::TranslateAssets { asset_ids: vec!["a1".into()], dx: 1.0, dy: 2.0, dz: 3.0 };
        let translated = round_trip(&snapshot, &translate);
        assert_eq!(translated.assets[0].origin, [1.0, 2.0, 3.0]);

        let rotate = ShootingMutation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 };
        let rotated = round_trip(&translated, &rotate);
        assert_ne!(rotated.assets[0].orientation, Some([0.0, 0.0, 0.0, 1.0]));

        let scale = ShootingMutation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 };
        let scaled = round_trip(&rotated, &scale);
        assert_eq!(crate::artifacts::shooting::shooting_asset_scale(&scaled.assets[0]), [2.0, 2.0, 2.0]);
    }

    #[test]
    fn set_snapshot_replaces_whole_document_and_restores() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let mut replacement = crate::artifacts::shooting::empty_shooting_snapshot();
        replacement.assets.push(sample_asset("a1"));
        replacement.shots.push(sample_shot("s1"));
        let operation = ShootingMutation::SetSnapshot { snapshot: replacement.clone() };
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next, replacement);
    }

    #[test]
    fn shooting_op_text_round_trips_collection_variants() {
        let asset = sample_asset("a1");
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Assets(CollectionMutation::Add { index: 0, item: asset }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Assets(CollectionMutation::Remove { id: "a1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Assets(CollectionMutation::Move { id: "a1".into(), to_index: 2 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Assets(CollectionMutation::Patch {
            id: "a1".into(),
            patch: ShootingAssetPatch { name: Some("Renamed".into()), url: None, origin: Some([1.0, 2.0, 3.0]), orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: Some([2.5, 2.5, 2.5]) },
        }));

        let shot = sample_shot("s1");
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Shots(CollectionMutation::Add { index: 0, item: shot }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Shots(CollectionMutation::Remove { id: "s1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Shots(CollectionMutation::Move { id: "s1".into(), to_index: 1 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::Shots(CollectionMutation::Patch {
            id: "s1".into(),
            patch: ShootingShotPatch { label: Some("Hero".into()), width: Some(512), height: None, format: None, shape: Some("ellipse".into()) },
        }));

        let saved_camera = ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() };
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SavedCameras(CollectionMutation::Add { index: 0, item: saved_camera }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SavedCameras(CollectionMutation::Remove { id: "cam1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SavedCameras(CollectionMutation::Move { id: "cam1".into(), to_index: 0 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SavedCameras(CollectionMutation::Patch {
            id: "cam1".into(),
            patch: ShootingSavedCameraPatch { label: Some("Renamed".into()), camera: Some(ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() }) },
        }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SavedCameras(CollectionMutation::Patch { id: "cam1".into(), patch: ShootingSavedCameraPatch { label: None, camera: None } }));
    }

    #[test]
    fn shooting_op_text_round_trips_every_other_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot { shot_id: Some("s1".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot { shot_id: None });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset { asset_id: Some("a1".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset { asset_id: None });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetShotCamera { shot_id: "s1".into(), camera: ShootingCamera::default() });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::PatchScene {
            patch: ShootingScenePatch { sun_enabled: Some(true), sun_azimuth: Some(90.0), sun_elevation: None, sun_intensity: Some(1.0), ambient_intensity: None, shadow_enabled: Some(false), material_roughness: Some(0.4) },
        });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::TranslateAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetSnapshot { snapshot: representative_snapshot() });
    }
}
//#endregion 🧪️Tests
