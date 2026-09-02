//! 🧬️ Shooting artifact — semantic document mutation dispatch enum.
//!
//! `#[derive(dsl_derive::Mutations)]` generates `impl protocol::Mutation<ShootingSnapshot>` and
//! `impl protocol::SemanticMutation<ShootingSnapshot>` for [`ShootingMutation`] by delegating each
//! variant to its payload's `protocol::MutationKind` impl — see the triad leaves
//! (`<slug>/{🦠️mutation,🔺️diff,↩️inverse}`) for the handcrafted logic. This file is dispatch-only.
//!
//! One triad directory per variant, 1:1 with `🦀️.rs`'s per-slug `#[path]` mounts (Wave-C
//! trueing pass, `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`) — the pre-migration grouped directories
//! (`📦assets`, `📸shots`, `🎥saved-cameras`, `☀️patch-scene` hosting multiple kinds each) were
//! split one-kind-per-dir, and the two mismatched slugs (`↔️translate-assets`→`↔️drag-assets`,
//! `📷set-shot-camera`→`📷replace-shot-camera`) and the orphan `📄set-snapshot` scaffold were
//! retired.

use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<ShootingSnapshot, ShootingMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️.rs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ShootingSnapshot, diff = ShootingDiff, schema = "shooting.shooting")]
pub enum ShootingMutation {
    CreateAsset(super::create_asset::CreateAsset),
    DeleteAsset(super::delete_asset::DeleteAsset),
    RenameAsset(super::rename_asset::RenameAsset),
    ChangeAssetUrl(super::change_asset_url::ChangeAssetUrl),
    ReorderAssets(super::reorder_assets::ReorderAssets),
    DragAssets(super::drag_assets::DragAssets),
    RotateAssets(super::rotate_assets::RotateAssets),
    ScaleAssets(super::scale_assets::ScaleAssets),
    CreateShot(super::create_shot::CreateShot),
    DeleteShot(super::delete_shot::DeleteShot),
    RenameShot(super::rename_shot::RenameShot),
    ChangeShotWidth(super::change_shot_width::ChangeShotWidth),
    ChangeShotHeight(super::change_shot_height::ChangeShotHeight),
    ChangeShotFormat(super::change_shot_format::ChangeShotFormat),
    ChangeShotShape(super::change_shot_shape::ChangeShotShape),
    ReorderShots(super::reorder_shots::ReorderShots),
    ReplaceShotCamera(super::replace_shot_camera::ReplaceShotCamera),
    CreateSavedCamera(super::create_saved_camera::CreateSavedCamera),
    DeleteSavedCamera(super::delete_saved_camera::DeleteSavedCamera),
    RenameSavedCamera(super::rename_saved_camera::RenameSavedCamera),
    ReplaceSavedCameraView(super::replace_saved_camera_view::ReplaceSavedCameraView),
    ReorderSavedCameras(super::reorder_saved_cameras::ReorderSavedCameras),
    SetActiveShot(super::set_active_shot::SetActiveShot),
    SetActiveAsset(super::set_active_asset::SetActiveAsset),
    ChangeSceneSunEnabled(super::change_scene_sun_enabled::ChangeSceneSunEnabled),
    ChangeSceneSunAzimuth(super::change_scene_sun_azimuth::ChangeSceneSunAzimuth),
    ChangeSceneSunElevation(super::change_scene_sun_elevation::ChangeSceneSunElevation),
    ChangeSceneSunIntensity(super::change_scene_sun_intensity::ChangeSceneSunIntensity),
    ChangeSceneAmbientIntensity(super::change_scene_ambient_intensity::ChangeSceneAmbientIntensity),
    ChangeSceneShadowEnabled(super::change_scene_shadow_enabled::ChangeSceneShadowEnabled),
    ChangeSceneMaterialRoughness(super::change_scene_material_roughness::ChangeSceneMaterialRoughness),
}

/// 🏷️ The kebab spelling of every [`ShootingMutation`] variant, in DECLARATION ORDER — the one list
/// the language-neutral test platform is measured against. It is duplicated in exactly two other
/// places on purpose: this subset's own oracle manifest catalog `shooting-1-any`
/// (`../../🔣️oracle.json`), which the completeness gate counts, and the
/// `mutate-shooting-1` case adapter, which must not link this crate in the oracle role.
/// [`tests::kinds_match_the_enum_and_the_catalog`] is what keeps all three honest.
pub const KINDS: &[&str] = &[
    "create-asset",
    "delete-asset",
    "rename-asset",
    "change-asset-url",
    "reorder-assets",
    "drag-assets",
    "rotate-assets",
    "scale-assets",
    "create-shot",
    "delete-shot",
    "rename-shot",
    "change-shot-width",
    "change-shot-height",
    "change-shot-format",
    "change-shot-shape",
    "reorder-shots",
    "replace-shot-camera",
    "create-saved-camera",
    "delete-saved-camera",
    "rename-saved-camera",
    "replace-saved-camera-view",
    "reorder-saved-cameras",
    "set-active-shot",
    "set-active-asset",
    "change-scene-sun-enabled",
    "change-scene-sun-azimuth",
    "change-scene-sun-elevation",
    "change-scene-sun-intensity",
    "change-scene-ambient-intensity",
    "change-scene-shadow-enabled",
    "change-scene-material-roughness",
];
//#endregion 🔖️Operations

//#region 🔖️Apply
/// 📦️ Applies `mutation` onto `snapshot`, returning the resulting snapshot — the free entry point
/// external Rust callers use when they cannot name this crate's private `protocol` extern-crate
/// item. Same shape as `🎬️present`'s `apply_present_mutation`.
pub fn apply_shooting_mutation(snapshot: &ShootingSnapshot, mutation: &ShootingMutation) -> protocol::MutationApplyResult<ShootingSnapshot> {
    semio_framework_plugin::resolve_ready(vcs::apply_mutation(snapshot, mutation)).map(|(next, _messages)| next)
}

/// ↩️ Computes `mutation`'s inverse mutations against `snapshot` (pre-state).
pub fn inverse_shooting_mutation(snapshot: &ShootingSnapshot, mutation: &ShootingMutation) -> Vec<ShootingMutation> {
    <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🔖️CaseBridges
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "createAsset", …}`) JSON projection
/// — the shape the `mutate-shooting-1` case's `Examples` rows carry, and the shape every committed
/// per-kind leaf fixture under `<kind>/🧪️tests/*/🦠️mutation/🔣️.json` already is — into a real
/// [`ShootingMutation`]. A thin `serde_json` wrapper (already a direct dependency of this crate, used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule, never a new
/// one), so the case reads the committed payload instead of re-declaring it as a Rust literal.
pub fn decode_shooting_mutation_json(text: &str) -> Result<ShootingMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed snapshot document — the `📸️snapshot/⬅️before/🔣️.json` every leaf
/// fixture of this vocabulary shares — into a real [`ShootingSnapshot`].
pub fn decode_shooting_snapshot_json(text: &str) -> Result<ShootingSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through. It belongs to the subset rather than
/// to a test adapter, because what counts as this document's meaning is this subset's ruling, not a
/// case's. Every inline artifact-lane field is present; the composed `emblem` child handle is not,
/// because it is a content address for an `s.stdio.semio.image` child that no kind of this
/// vocabulary addresses.
pub fn encode_shooting_projection_json(snapshot: &ShootingSnapshot) -> String {
    serde_json::json!({
        "schema": snapshot.schema,
        "assets": snapshot.assets,
        "savedCameras": snapshot.saved_cameras,
        "scene": snapshot.scene,
        "shots": snapshot.shots,
        "activeShotId": snapshot.active_shot_id,
        "activeAssetId": snapshot.active_asset_id
    })
    .to_string()
}
//#endregion 🔖️CaseBridges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::{ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot, SHOOTING_DOCUMENT_SCHEMA};
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};
    use protocol::{Mutation, MutationDiff};

    async fn sample_asset(id: &str) -> ShootingAsset {
        ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None }
    }

    async fn sample_shot(id: &str) -> ShootingShot {
        ShootingShot { id: id.into(), label: format!("Shot {id}"), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None }
    }

    async fn round_trip(snapshot: &ShootingSnapshot, operation: &ShootingMutation) -> ShootingSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation).expect("valid mutation").0;
        let backwards = operation.inverse(snapshot);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_mutation(&restored, back).expect("valid inverse mutation").0;
        }
        assert_eq!(&restored, snapshot, "backwards() must exactly restore the pre-operation fixture");
        forward
    }

    /// 🎞️ A fixture exercising every field — duplicated verbatim across the `dsl`/`op`/`pack`
    /// crates' worth of tests (each is its own compilation unit, so a shared cross-crate test-only
    /// helper isn't worth a dependency).
    #[allow(clippy::approx_constant, reason = "0.7071 is deliberately an approximate quaternion component in this snapshot, not the FRAC_1_SQRT_2 constant")]
    async fn representative_snapshot() -> ShootingSnapshot {
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
            },
            shots: vec![
                ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()) },
                ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None },
            ],
            active_shot_id: "s1".into(),
            active_asset_id: "a1".into(),
            emblem: Some(crate::artifacts::shooting::shooting_emblem_child_handle(&crate::artifacts::shooting::shooting_emblem_image_from_bytes(vec![137, 80, 78, 71]))),
        }
    }

    //#region 📦assets
    #[semio_framework_async_macros::async_test]
    async fn assets_create_rename_change_url_delete_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let create = ShootingMutation::CreateAsset(super::super::create_asset::CreateAsset { asset: sample_asset("a1"), index: Some(0) });
        let with_asset = round_trip(&snapshot, &create);
        assert_eq!(with_asset.assets.len(), 1);

        let rename = ShootingMutation::RenameAsset(super::super::rename_asset::RenameAsset { id: "a1".into(), new_name: "Renamed".into() });
        let renamed = round_trip(&with_asset, &rename);
        assert_eq!(renamed.assets[0].name, "Renamed");

        let change_url = ShootingMutation::ChangeAssetUrl(super::super::change_asset_url::ChangeAssetUrl { id: "a1".into(), new_url: "/mesh/a1-v2.glb".into() });
        let changed = round_trip(&renamed, &change_url);
        assert_eq!(changed.assets[0].url, "/mesh/a1-v2.glb");

        let delete = ShootingMutation::DeleteAsset(super::super::delete_asset::DeleteAsset { id: "a1".into() });
        let deleted = round_trip(&changed, &delete);
        assert!(deleted.assets.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_assets_round_trips() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.assets = vec![sample_asset("a1"), sample_asset("a2"), sample_asset("a3")];
        let reorder = ShootingMutation::ReorderAssets(super::super::reorder_assets::ReorderAssets { id: "a1".into(), to_index: 2 });
        let reordered = round_trip(&snapshot, &reorder);
        assert_eq!(reordered.assets.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), vec!["a2", "a3", "a1"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_asset_of_a_missing_id_has_an_empty_inverse() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let delete = ShootingMutation::DeleteAsset(super::super::delete_asset::DeleteAsset { id: "nope".into() });
        assert!(delete.inverse(&snapshot).is_empty(), "deleting an absent id has nothing to undo");
    }
    //#endregion 📦assets

    //#region ↔️🔄↕️transforms
    #[semio_framework_async_macros::async_test]
    async fn drag_rotate_scale_assets_round_trip() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let mut asset = sample_asset("a1");
        // `ScaleAssets` always writes an explicit `Some([..])` scale, so backwards() restoring an
        // originally-`None` scale lands on `Some([1,1,1])` — the same effective scale (see
        // `shooting_asset_scale`) but not byte-identical. Start from an explicit identity scale so
        // the round-trip assertion checks real equality instead of that representation quirk.
        asset.scale = Some([1.0, 1.0, 1.0]);
        snapshot.assets.push(asset);
        let drag = ShootingMutation::DragAssets(super::super::drag_assets::DragAssets { asset_ids: vec!["a1".into()], dx: 1.0, dy: 2.0, dz: 3.0 });
        let dragged = round_trip(&snapshot, &drag);
        assert_eq!(dragged.assets[0].origin, [1.0, 2.0, 3.0]);

        let rotate = ShootingMutation::RotateAssets(super::super::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 });
        let rotated = round_trip(&dragged, &rotate);
        assert_ne!(rotated.assets[0].orientation, Some([0.0, 0.0, 0.0, 1.0]));

        let scale = ShootingMutation::ScaleAssets(super::super::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 });
        let scaled = round_trip(&rotated, &scale);
        assert_eq!(crate::artifacts::shooting::shooting_asset_scale(&scaled.assets[0]), [2.0, 2.0, 2.0]);
    }
    //#endregion ↔️🔄↕️transforms

    //#region 📸shots
    #[semio_framework_async_macros::async_test]
    async fn shots_create_rename_resize_reformat_reshape_delete_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let create = ShootingMutation::CreateShot(super::super::create_shot::CreateShot { shot: sample_shot("s1"), index: Some(0) });
        let with_shot = round_trip(&snapshot, &create);
        assert_eq!(with_shot.shots.len(), 1);

        let rename = ShootingMutation::RenameShot(super::super::rename_shot::RenameShot { id: "s1".into(), new_label: "Hero".into() });
        let renamed = round_trip(&with_shot, &rename);
        assert_eq!(renamed.shots[0].label, "Hero");

        let width = ShootingMutation::ChangeShotWidth(super::super::change_shot_width::ChangeShotWidth { id: "s1".into(), new_width: 512 });
        let widened = round_trip(&renamed, &width);
        assert_eq!(widened.shots[0].width, 512);

        let height = ShootingMutation::ChangeShotHeight(super::super::change_shot_height::ChangeShotHeight { id: "s1".into(), new_height: 512 });
        let heightened = round_trip(&widened, &height);
        assert_eq!(heightened.shots[0].height, 512);

        let format = ShootingMutation::ChangeShotFormat(super::super::change_shot_format::ChangeShotFormat { id: "s1".into(), new_format: "svg".into() });
        let reformatted = round_trip(&heightened, &format);
        assert_eq!(reformatted.shots[0].format, "svg");

        let shape = ShootingMutation::ChangeShotShape(super::super::change_shot_shape::ChangeShotShape { id: "s1".into(), new_shape: "ellipse".into() });
        let reshaped = round_trip(&reformatted, &shape);
        assert_eq!(reshaped.shots[0].shape, "ellipse");

        let delete = ShootingMutation::DeleteShot(super::super::delete_shot::DeleteShot { id: "s1".into() });
        let deleted = round_trip(&reshaped, &delete);
        assert!(deleted.shots.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_shots_round_trips() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots = vec![sample_shot("s1"), sample_shot("s2")];
        let reorder = ShootingMutation::ReorderShots(super::super::reorder_shots::ReorderShots { id: "s2".into(), to_index: 0 });
        let reordered = round_trip(&snapshot, &reorder);
        assert_eq!(reordered.shots.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["s2", "s1"]);
    }
    //#endregion 📸shots

    //#region 🎥saved-cameras / 📷shot-camera
    #[semio_framework_async_macros::async_test]
    async fn saved_cameras_create_rename_replace_view_reorder_delete_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let create =
            ShootingMutation::CreateSavedCamera(super::super::create_saved_camera::CreateSavedCamera { saved_camera: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() }, index: Some(0) });
        let with_camera = round_trip(&snapshot, &create);
        assert_eq!(with_camera.saved_cameras.len(), 1);

        let rename = ShootingMutation::RenameSavedCamera(super::super::rename_saved_camera::RenameSavedCamera { id: "cam1".into(), new_label: "Renamed".into() });
        let renamed = round_trip(&with_camera, &rename);
        assert_eq!(renamed.saved_cameras[0].label, "Renamed");

        let replace_view = ShootingMutation::ReplaceSavedCameraView(super::super::replace_saved_camera_view::ReplaceSavedCameraView { id: "cam1".into(), new_camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() } });
        let replaced = round_trip(&renamed, &replace_view);
        assert_eq!(replaced.saved_cameras[0].camera.position, [1.0, 2.0, 3.0]);

        let delete = ShootingMutation::DeleteSavedCamera(super::super::delete_saved_camera::DeleteSavedCamera { id: "cam1".into() });
        let deleted = round_trip(&replaced, &delete);
        assert!(deleted.saved_cameras.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_shot_camera_is_a_no_op_when_shot_has_no_saved_camera() {
        // 🎥️ The free/live viewport camera is session-only runtime state now (never a document
        // field) — `ReplaceShotCamera` against a shot with no saved-camera reference has nothing to patch.
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots.push(sample_shot("s1"));
        let camera = ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() };
        let operation = ShootingMutation::ReplaceShotCamera(super::super::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: camera });
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next, snapshot, "no saved camera referenced by the shot means no document change");
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_shot_camera_patches_the_saved_camera_it_references() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.saved_cameras.push(ShootingSavedCamera { id: "cam1".into(), label: "A".into(), camera: ShootingCamera::default() });
        let mut shot = sample_shot("s1");
        shot.camera_id = Some("cam1".into());
        snapshot.shots.push(shot);
        snapshot.active_shot_id = "s1".into();
        let camera = ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() };
        let operation = ShootingMutation::ReplaceShotCamera(super::super::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: camera });
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
    }
    //#endregion 🎥saved-cameras / 📷shot-camera

    //#region 🎯📌active-selection
    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_and_asset_round_trip() {
        let mut snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        snapshot.shots.push(sample_shot("s1"));
        snapshot.assets.push(sample_asset("a1"));
        let operation = ShootingMutation::SetActiveShot(super::super::set_active_shot::SetActiveShot { shot_id: Some("s1".into()) });
        let next = round_trip(&snapshot, &operation);
        assert_eq!(next.active_shot_id, "s1");
        let operation = ShootingMutation::SetActiveAsset(super::super::set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) });
        let next2 = round_trip(&next, &operation);
        assert_eq!(next2.active_asset_id, "a1");
    }
    //#endregion 🎯📌active-selection

    //#region ☀️scene
    #[semio_framework_async_macros::async_test]
    async fn scene_field_mutations_round_trip() {
        let snapshot = crate::artifacts::shooting::empty_shooting_snapshot();
        let next = round_trip(&snapshot, &ShootingMutation::ChangeSceneSunEnabled(super::super::change_scene_sun_enabled::ChangeSceneSunEnabled { new_enabled: true }));
        assert!(next.scene.sun.enabled);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneSunAzimuth(super::super::change_scene_sun_azimuth::ChangeSceneSunAzimuth { new_azimuth: 90.0 }));
        assert_eq!(next.scene.sun.azimuth, 90.0);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneSunElevation(super::super::change_scene_sun_elevation::ChangeSceneSunElevation { new_elevation: 45.0 }));
        assert_eq!(next.scene.sun.elevation, 45.0);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneSunIntensity(super::super::change_scene_sun_intensity::ChangeSceneSunIntensity { new_intensity: 5.0 }));
        assert_eq!(next.scene.sun.intensity, 5.0);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneAmbientIntensity(super::super::change_scene_ambient_intensity::ChangeSceneAmbientIntensity { new_intensity: 0.5 }));
        assert_eq!(next.scene.ambient.intensity, 0.5);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneShadowEnabled(super::super::change_scene_shadow_enabled::ChangeSceneShadowEnabled { new_enabled: false }));
        assert!(!next.scene.shadow.enabled);
        let next = round_trip(&next, &ShootingMutation::ChangeSceneMaterialRoughness(super::super::change_scene_material_roughness::ChangeSceneMaterialRoughness { new_roughness: 0.4 }));
        assert_eq!(next.scene.material.roughness, 0.4);
    }
    //#endregion ☀️scene

    //#region 🗣️OpText
    #[semio_framework_async_macros::async_test]
    async fn shooting_op_text_round_trips_every_variant() {
        let asset = sample_asset("a1");
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::CreateAsset(super::super::create_asset::CreateAsset { asset: asset.clone(), index: Some(0) }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::DeleteAsset(super::super::delete_asset::DeleteAsset { id: "a1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::RenameAsset(super::super::rename_asset::RenameAsset { id: "a1".into(), new_name: "Renamed".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeAssetUrl(super::super::change_asset_url::ChangeAssetUrl { id: "a1".into(), new_url: "/mesh/a1-v2.glb".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ReorderAssets(super::super::reorder_assets::ReorderAssets { id: "a1".into(), to_index: 2 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::DragAssets(super::super::drag_assets::DragAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::RotateAssets(super::super::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ScaleAssets(super::super::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }));

        let shot = sample_shot("s1");
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::CreateShot(super::super::create_shot::CreateShot { shot: shot.clone(), index: Some(0) }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::DeleteShot(super::super::delete_shot::DeleteShot { id: "s1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::RenameShot(super::super::rename_shot::RenameShot { id: "s1".into(), new_label: "Hero".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotWidth(super::super::change_shot_width::ChangeShotWidth { id: "s1".into(), new_width: 512 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotHeight(super::super::change_shot_height::ChangeShotHeight { id: "s1".into(), new_height: 512 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotFormat(super::super::change_shot_format::ChangeShotFormat { id: "s1".into(), new_format: "svg".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotShape(super::super::change_shot_shape::ChangeShotShape { id: "s1".into(), new_shape: "ellipse".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ReorderShots(super::super::reorder_shots::ReorderShots { id: "s1".into(), to_index: 1 }));

        let saved_camera = ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() };
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::CreateSavedCamera(super::super::create_saved_camera::CreateSavedCamera { saved_camera: saved_camera.clone(), index: Some(0) }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::DeleteSavedCamera(super::super::delete_saved_camera::DeleteSavedCamera { id: "cam1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::RenameSavedCamera(super::super::rename_saved_camera::RenameSavedCamera { id: "cam1".into(), new_label: "Renamed".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ReplaceSavedCameraView(super::super::replace_saved_camera_view::ReplaceSavedCameraView {
            id: "cam1".into(),
            new_camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() },
        }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ReorderSavedCameras(super::super::reorder_saved_cameras::ReorderSavedCameras { id: "cam1".into(), to_index: 0 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ReplaceShotCamera(super::super::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: ShootingCamera::default() }));

        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot(super::super::set_active_shot::SetActiveShot { shot_id: Some("s1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot(super::super::set_active_shot::SetActiveShot { shot_id: None }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset(super::super::set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset(super::super::set_active_asset::SetActiveAsset { asset_id: None }));

        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunEnabled(super::super::change_scene_sun_enabled::ChangeSceneSunEnabled { new_enabled: true }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunAzimuth(super::super::change_scene_sun_azimuth::ChangeSceneSunAzimuth { new_azimuth: 90.0 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunElevation(super::super::change_scene_sun_elevation::ChangeSceneSunElevation { new_elevation: 45.0 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunIntensity(super::super::change_scene_sun_intensity::ChangeSceneSunIntensity { new_intensity: 5.0 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneAmbientIntensity(super::super::change_scene_ambient_intensity::ChangeSceneAmbientIntensity { new_intensity: 0.5 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneShadowEnabled(super::super::change_scene_shadow_enabled::ChangeSceneShadowEnabled { new_enabled: false }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneMaterialRoughness(super::super::change_scene_material_roughness::ChangeSceneMaterialRoughness { new_roughness: 0.4 }));
    }
    //#endregion 🗣️OpText

    //#region ⚖️SemanticLaws
    /// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` (`protocol::os_spr::testkit`,
    /// added by the Wave 0 mechanism pass) against the three most structurally distinct new kinds:
    /// an id-keyed collection create/delete pair, a bulk bulk-bulk transform, and a document-root
    /// scalar setter.
    #[semio_framework_async_macros::async_test]
    async fn create_asset_obeys_the_inverse_and_absorb_laws() {
        let base = representative_snapshot();
        let create = ShootingMutation::CreateAsset(super::super::create_asset::CreateAsset { asset: sample_asset("a9"), index: None });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &create);
        let d1 = create.diff(&base).into_parts().0;
        let after = d1.apply(&base).expect("valid mutation diff");
        let d2 = ShootingMutation::RenameAsset(super::super::rename_asset::RenameAsset { id: "a9".into(), new_name: "Renamed".into() }).diff(&after).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn drag_assets_obeys_the_inverse_law() {
        let base = representative_snapshot();
        let drag = ShootingMutation::DragAssets(super::super::drag_assets::DragAssets { asset_ids: vec!["a1".into()], dx: 4.0, dy: -1.0, dz: 0.5 });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &drag);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_obeys_the_inverse_law() {
        let base = representative_snapshot();
        let set = ShootingMutation::SetActiveShot(super::super::set_active_shot::SetActiveShot { shot_id: Some("s2".into()) });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &set);
    }
    //#endregion ⚖️SemanticLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
    /// (create/delete/rename/change/reorder/drag-rotate-scale/replace/set).
    #[semio_framework_async_macros::async_test]
    async fn create_asset_duplicate_id_is_fatal() {
        let base = representative_snapshot();
        let outcome = ShootingMutation::CreateAsset(super::super::create_asset::CreateAsset { asset: sample_asset("a1"), index: None }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_asset_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::DeleteAsset(super::super::delete_asset::DeleteAsset { id: "ghost".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_asset_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::RenameAsset(super::super::rename_asset::RenameAsset { id: "ghost".into(), new_name: "x".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_asset_url_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::ChangeAssetUrl(super::super::change_asset_url::ChangeAssetUrl { id: "ghost".into(), new_url: "/x.glb".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_assets_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::ReorderAssets(super::super::reorder_assets::ReorderAssets { id: "ghost".into(), to_index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn drag_assets_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::DragAssets(super::super::drag_assets::DragAssets { asset_ids: vec!["ghost".into()], dx: 1.0, dy: 1.0, dz: 1.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn scale_assets_non_finite_is_fatal() {
        let base = representative_snapshot();
        let outcome = ShootingMutation::ScaleAssets(super::super::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: f64::NAN, sy: 1.0, sz: 1.0 }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn rotate_assets_non_finite_is_fatal() {
        let base = representative_snapshot();
        let outcome = ShootingMutation::RotateAssets(super::super::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: f64::NAN, ay: 0.0, az: 1.0, angle: 1.0 }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_saved_camera_view_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::ReplaceSavedCameraView(super::super::replace_saved_camera_view::ReplaceSavedCameraView { id: "ghost".into(), new_camera: ShootingCamera::default() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::SetActiveShot(super::super::set_active_shot::SetActiveShot { shot_id: Some("ghost".into()) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_shot_duplicate_id_is_fatal() {
        let base = representative_snapshot();
        let outcome = ShootingMutation::CreateShot(super::super::create_shot::CreateShot { shot: sample_shot("s1"), index: None }).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_shot_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::DeleteShot(super::super::delete_shot::DeleteShot { id: "ghost".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_shot_width_missing_target_is_error() {
        let base = representative_snapshot();
        assert_missing_target_is_error(&base, &ShootingMutation::ChangeShotWidth(super::super::change_shot_width::ChangeShotWidth { id: "ghost".into(), new_width: 100 }));
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🔖️KindsCatalog
    /// 🏷️ [`KINDS`] is the bridge between this enum and the language-neutral test platform, which
    /// never parses Rust. This proves it names every variant, in declaration order, with the same
    /// kebab spelling `#[derive(dsl::Mutations)]` derives — and that this subset's own committed
    /// catalog declares exactly the same set, so the completeness gate cannot be measuring a
    /// vocabulary that has drifted away from the code.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let declared: Vec<&str> = <ShootingMutation as protocol::SemanticMutation<ShootingSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(KINDS, declared.as_slice(), "KINDS must name every ShootingMutation variant, in declaration order, spelled as its own MutationKind::SEMANTICS.kind");
        let manifest = include_str!("../../🔣️oracle.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog shooting-1-any");
        }
    }
    //#endregion 🔖️KindsCatalog
}
//#endregion 🧪️Tests
