use crate::ShootingSnapshot;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::diff::ShootingDiff;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot, SHOOTING_DOCUMENT_SCHEMA};
use protocol::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error};
use protocol::{ColdMutation, Mutation, MutationDiff};

fn sample_asset(id: &str) -> ShootingAsset {
    ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None }
}

fn sample_shot(id: &str) -> ShootingShot {
    ShootingShot { id: id.into(), label: format!("Shot {id}"), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: None, drawing: None, auto_fit: None }
}

fn round_trip(snapshot: &ShootingSnapshot, operation: &ShootingMutation) -> ShootingSnapshot {
    let forward = store::apply_mutation(snapshot, operation).expect("valid mutation").0;
    let backwards = operation.inverse(snapshot).expect("valid retained mutation inverse fixture");
    let mut restored = forward.clone();
    for back in &backwards {
        restored = store::apply_mutation(&restored, back).expect("valid inverse mutation").0;
    }
    assert_eq!(&restored, snapshot, "backwards() must exactly restore the pre-operation fixture");
    forward
}

/// 🎞️ A fixture exercising every field — duplicated verbatim across the `dsl`/`op`/`pack`
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
        scene: crate::ShootingSceneLighting {
            background: "#111111".into(),
            sun: crate::ShootingSun { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 3.0, color: "#ff00ff".into() },
            ambient: crate::ShootingAmbient { intensity: 0.9, color: "#00ffff".into() },
            shadow: crate::ShootingShadow { enabled: false, opacity: 0.5, softness: 0.2 },
            material: crate::ShootingMaterial { color: "#abcdef".into(), metalness: 0.3, roughness: 0.7, emissive: "#123456".into(), emissive_intensity: 0.1, stroke: "#000000".into() },
        },
        shots: vec![
            ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()), drawing: None, auto_fit: None },
            ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None, drawing: None, auto_fit: None },
        ],
        active_shot_id: "s1".into(),
        active_asset_id: "a1".into(),
        emblem: Some(crate::shooting_emblem_child_handle(&crate::shooting_emblem_image_from_bytes(vec![137, 80, 78, 71]))),
    }
}

//#region 📦assets
#[semio_framework_async_macros::async_test]
async fn assets_create_rename_change_url_delete_round_trip() {
    let snapshot = crate::empty_shooting_snapshot();
    let create = ShootingMutation::CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset: sample_asset("a1"), index: Some(0) });
    let with_asset = round_trip(&snapshot, &create);
    assert_eq!(with_asset.assets.len(), 1);

    let rename = ShootingMutation::RenameAsset(crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset { id: "a1".into(), new_name: "Renamed".into() });
    let renamed = round_trip(&with_asset, &rename);
    assert_eq!(renamed.assets[0].name, "Renamed");

    let change_url = ShootingMutation::ChangeAssetUrl(crate::standards::v1::subsets::any::schema::mutations::change_asset_url::ChangeAssetUrl { id: "a1".into(), new_url: "/mesh/a1-v2.glb".into() });
    let changed = round_trip(&renamed, &change_url);
    assert_eq!(changed.assets[0].url, "/mesh/a1-v2.glb");

    let delete = ShootingMutation::DeleteAsset(crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset { id: "a1".into() });
    let deleted = round_trip(&changed, &delete);
    assert!(deleted.assets.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn reorder_assets_round_trips() {
    let mut snapshot = crate::empty_shooting_snapshot();
    snapshot.assets = vec![sample_asset("a1"), sample_asset("a2"), sample_asset("a3")];
    let reorder = ShootingMutation::ReorderAssets(crate::standards::v1::subsets::any::schema::mutations::reorder_assets::ReorderAssets { id: "a1".into(), to_index: 2 });
    let reordered = round_trip(&snapshot, &reorder);
    assert_eq!(reordered.assets.iter().map(|a| a.id.clone()).collect::<Vec<_>>(), vec!["a2", "a3", "a1"]);
}

#[semio_framework_async_macros::async_test]
async fn delete_asset_of_a_missing_id_has_an_empty_inverse() {
    let snapshot = crate::empty_shooting_snapshot();
    let delete = ShootingMutation::DeleteAsset(crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset { id: "nope".into() });
    assert!(delete.inverse(&snapshot).expect("valid retained mutation inverse fixture").is_empty(), "deleting an absent id has nothing to undo");
}
//#endregion 📦assets

//#region ↔️🔄↕️transforms
#[semio_framework_async_macros::async_test]
async fn drag_rotate_scale_assets_round_trip() {
    let mut snapshot = crate::empty_shooting_snapshot();
    let mut asset = sample_asset("a1");
    asset.scale = Some([1.0, 1.0, 1.0]);
    snapshot.assets.push(asset);
    let drag = ShootingMutation::DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets { asset_ids: vec!["a1".into()], dx: 1.0, dy: 2.0, dz: 3.0 });
    let dragged = round_trip(&snapshot, &drag);
    assert_eq!(dragged.assets[0].origin, [1.0, 2.0, 3.0]);

    let rotate = ShootingMutation::RotateAssets(crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 });
    let rotated = round_trip(&dragged, &rotate);
    assert_ne!(rotated.assets[0].orientation, Some([0.0, 0.0, 0.0, 1.0]));

    let scale = ShootingMutation::ScaleAssets(crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 });
    let scaled = round_trip(&rotated, &scale);
    assert_eq!(crate::shooting_asset_scale(&scaled.assets[0]), [2.0, 2.0, 2.0]);
}
//#endregion ↔️🔄↕️transforms

//#region 📸shots
#[semio_framework_async_macros::async_test]
async fn shots_create_rename_resize_reformat_reshape_delete_round_trip() {
    let snapshot = crate::empty_shooting_snapshot();
    let create = ShootingMutation::CreateShot(crate::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot { shot: sample_shot("s1"), index: Some(0) });
    let with_shot = round_trip(&snapshot, &create);
    assert_eq!(with_shot.shots.len(), 1);

    let rename = ShootingMutation::RenameShot(crate::standards::v1::subsets::any::schema::mutations::rename_shot::RenameShot { id: "s1".into(), new_label: "Hero".into() });
    let renamed = round_trip(&with_shot, &rename);
    assert_eq!(renamed.shots[0].label, "Hero");

    let width = ShootingMutation::ChangeShotWidth(crate::standards::v1::subsets::any::schema::mutations::change_shot_width::ChangeShotWidth { id: "s1".into(), new_width: 512 });
    let widened = round_trip(&renamed, &width);
    assert_eq!(widened.shots[0].width, 512);

    let height = ShootingMutation::ChangeShotHeight(crate::standards::v1::subsets::any::schema::mutations::change_shot_height::ChangeShotHeight { id: "s1".into(), new_height: 512 });
    let heightened = round_trip(&widened, &height);
    assert_eq!(heightened.shots[0].height, 512);

    let format = ShootingMutation::ChangeShotFormat(crate::standards::v1::subsets::any::schema::mutations::change_shot_format::ChangeShotFormat { id: "s1".into(), new_format: "svg".into() });
    let reformatted = round_trip(&heightened, &format);
    assert_eq!(reformatted.shots[0].format, "svg");

    let shape = ShootingMutation::ChangeShotShape(crate::standards::v1::subsets::any::schema::mutations::change_shot_shape::ChangeShotShape { id: "s1".into(), new_shape: "ellipse".into() });
    let reshaped = round_trip(&reformatted, &shape);
    assert_eq!(reshaped.shots[0].shape, "ellipse");

    let delete = ShootingMutation::DeleteShot(crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot { id: "s1".into() });
    let deleted = round_trip(&reshaped, &delete);
    assert!(deleted.shots.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn reorder_shots_round_trips() {
    let mut snapshot = crate::empty_shooting_snapshot();
    snapshot.shots = vec![sample_shot("s1"), sample_shot("s2")];
    let reorder = ShootingMutation::ReorderShots(crate::standards::v1::subsets::any::schema::mutations::reorder_shots::ReorderShots { id: "s2".into(), to_index: 0 });
    let reordered = round_trip(&snapshot, &reorder);
    assert_eq!(reordered.shots.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["s2", "s1"]);
}
//#endregion 📸shots

//#region 🎥saved-cameras / 📷shot-camera
#[semio_framework_async_macros::async_test]
async fn saved_cameras_create_rename_replace_view_reorder_delete_round_trip() {
    let snapshot = crate::empty_shooting_snapshot();
    let create = ShootingMutation::CreateSavedCamera(crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera { saved_camera: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() }, index: Some(0) });
    let with_camera = round_trip(&snapshot, &create);
    assert_eq!(with_camera.saved_cameras.len(), 1);

    let rename = ShootingMutation::RenameSavedCamera(crate::standards::v1::subsets::any::schema::mutations::rename_saved_camera::RenameSavedCamera { id: "cam1".into(), new_label: "Renamed".into() });
    let renamed = round_trip(&with_camera, &rename);
    assert_eq!(renamed.saved_cameras[0].label, "Renamed");

    let replace_view = ShootingMutation::ReplaceSavedCameraView(crate::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::ReplaceSavedCameraView { id: "cam1".into(), new_camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() } });
    let replaced = round_trip(&renamed, &replace_view);
    assert_eq!(replaced.saved_cameras[0].camera.position, [1.0, 2.0, 3.0]);

    let delete = ShootingMutation::DeleteSavedCamera(crate::standards::v1::subsets::any::schema::mutations::delete_saved_camera::DeleteSavedCamera { id: "cam1".into() });
    let deleted = round_trip(&replaced, &delete);
    assert!(deleted.saved_cameras.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn replace_shot_camera_is_a_no_op_when_shot_has_no_saved_camera() {
    let mut snapshot = crate::empty_shooting_snapshot();
    snapshot.shots.push(sample_shot("s1"));
    let camera = ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() };
    let operation = ShootingMutation::ReplaceShotCamera(crate::standards::v1::subsets::any::schema::mutations::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: camera });
    let next = round_trip(&snapshot, &operation);
    assert_eq!(next, snapshot, "no saved camera referenced by the shot means no document change");
}

#[semio_framework_async_macros::async_test]
async fn replace_shot_camera_patches_the_saved_camera_it_references() {
    let mut snapshot = crate::empty_shooting_snapshot();
    snapshot.saved_cameras.push(ShootingSavedCamera { id: "cam1".into(), label: "A".into(), camera: ShootingCamera::default() });
    let mut shot = sample_shot("s1");
    shot.camera_id = Some("cam1".into());
    snapshot.shots.push(shot);
    snapshot.active_shot_id = "s1".into();
    let camera = ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() };
    let operation = ShootingMutation::ReplaceShotCamera(crate::standards::v1::subsets::any::schema::mutations::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: camera });
    let next = round_trip(&snapshot, &operation);
    assert_eq!(next.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
}
//#endregion 🎥saved-cameras / 📷shot-camera

//#region 🎯📌active-selection
#[semio_framework_async_macros::async_test]
async fn set_active_shot_and_asset_round_trip() {
    let mut snapshot = crate::empty_shooting_snapshot();
    snapshot.shots.push(sample_shot("s1"));
    snapshot.assets.push(sample_asset("a1"));
    let operation = ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: Some("s1".into()) });
    let next = round_trip(&snapshot, &operation);
    assert_eq!(next.active_shot_id, "s1");
    let operation = ShootingMutation::SetActiveAsset(crate::standards::v1::subsets::any::schema::mutations::set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) });
    let next2 = round_trip(&next, &operation);
    assert_eq!(next2.active_asset_id, "a1");
}
//#endregion 🎯📌active-selection

//#region ☀️scene
#[semio_framework_async_macros::async_test]
async fn scene_field_mutations_round_trip() {
    let snapshot = crate::empty_shooting_snapshot();
    let next = round_trip(&snapshot, &ShootingMutation::ChangeSceneSunEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::ChangeSceneSunEnabled { new_enabled: true }));
    assert!(next.scene.sun.enabled);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneSunAzimuth(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::ChangeSceneSunAzimuth { new_azimuth: 90.0 }));
    assert_eq!(next.scene.sun.azimuth, 90.0);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneSunElevation(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::ChangeSceneSunElevation { new_elevation: 45.0 }));
    assert_eq!(next.scene.sun.elevation, 45.0);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneSunIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::ChangeSceneSunIntensity { new_intensity: 5.0 }));
    assert_eq!(next.scene.sun.intensity, 5.0);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneAmbientIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::ChangeSceneAmbientIntensity { new_intensity: 0.5 }));
    assert_eq!(next.scene.ambient.intensity, 0.5);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneShadowEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::ChangeSceneShadowEnabled { new_enabled: false }));
    assert!(!next.scene.shadow.enabled);
    let next = round_trip(&next, &ShootingMutation::ChangeSceneMaterialRoughness(crate::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::ChangeSceneMaterialRoughness { new_roughness: 0.4 }));
    assert_eq!(next.scene.material.roughness, 0.4);
}
//#endregion ☀️scene

//#region 🗣️OpText
#[semio_framework_async_macros::async_test]
async fn shooting_op_text_round_trips_every_variant() {
    let asset = sample_asset("a1");
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset: asset.clone(), index: Some(0) }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::DeleteAsset(crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset { id: "a1".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::RenameAsset(crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset { id: "a1".into(), new_name: "Renamed".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeAssetUrl(crate::standards::v1::subsets::any::schema::mutations::change_asset_url::ChangeAssetUrl { id: "a1".into(), new_url: "/mesh/a1-v2.glb".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ReorderAssets(crate::standards::v1::subsets::any::schema::mutations::reorder_assets::ReorderAssets { id: "a1".into(), to_index: 2 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::RotateAssets(crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ScaleAssets(crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }));

    let shot = sample_shot("s1");
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::CreateShot(crate::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot { shot: shot.clone(), index: Some(0) }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::DeleteShot(crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot { id: "s1".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::RenameShot(crate::standards::v1::subsets::any::schema::mutations::rename_shot::RenameShot { id: "s1".into(), new_label: "Hero".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotWidth(crate::standards::v1::subsets::any::schema::mutations::change_shot_width::ChangeShotWidth { id: "s1".into(), new_width: 512 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotHeight(crate::standards::v1::subsets::any::schema::mutations::change_shot_height::ChangeShotHeight { id: "s1".into(), new_height: 512 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotFormat(crate::standards::v1::subsets::any::schema::mutations::change_shot_format::ChangeShotFormat { id: "s1".into(), new_format: "svg".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeShotShape(crate::standards::v1::subsets::any::schema::mutations::change_shot_shape::ChangeShotShape { id: "s1".into(), new_shape: "ellipse".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ReorderShots(crate::standards::v1::subsets::any::schema::mutations::reorder_shots::ReorderShots { id: "s1".into(), to_index: 1 }));

    let saved_camera = ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() };
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::CreateSavedCamera(crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera { saved_camera: saved_camera.clone(), index: Some(0) }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::DeleteSavedCamera(crate::standards::v1::subsets::any::schema::mutations::delete_saved_camera::DeleteSavedCamera { id: "cam1".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::RenameSavedCamera(crate::standards::v1::subsets::any::schema::mutations::rename_saved_camera::RenameSavedCamera { id: "cam1".into(), new_label: "Renamed".into() }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ReplaceSavedCameraView(crate::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::ReplaceSavedCameraView {
        id: "cam1".into(),
        new_camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..Default::default() },
    }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ReorderSavedCameras(crate::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::ReorderSavedCameras { id: "cam1".into(), to_index: 0 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ReplaceShotCamera(crate::standards::v1::subsets::any::schema::mutations::replace_shot_camera::ReplaceShotCamera { shot_id: "s1".into(), new_camera: ShootingCamera::default() }));

    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: Some("s1".into()) }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: None }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset(crate::standards::v1::subsets::any::schema::mutations::set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::SetActiveAsset(crate::standards::v1::subsets::any::schema::mutations::set_active_asset::SetActiveAsset { asset_id: None }));

    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::ChangeSceneSunEnabled { new_enabled: true }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunAzimuth(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::ChangeSceneSunAzimuth { new_azimuth: 90.0 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunElevation(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::ChangeSceneSunElevation { new_elevation: 45.0 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneSunIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::ChangeSceneSunIntensity { new_intensity: 5.0 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneAmbientIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::ChangeSceneAmbientIntensity { new_intensity: 0.5 }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneShadowEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::ChangeSceneShadowEnabled { new_enabled: false }));
    semio_os_store_test_support::assert_op_line_round_trip(&ShootingMutation::ChangeSceneMaterialRoughness(crate::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::ChangeSceneMaterialRoughness { new_roughness: 0.4 }));
}
//#endregion 🗣️OpText

//#region ⚖️SemanticLaws
/// ⚖️ `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` (`protocol::os_spr::protocol_laws`,
/// added by the Wave 0 mechanism pass) against the three most structurally distinct new kinds:
/// an id-keyed collection create/delete pair, a bulk bulk-bulk transform, and a document-root
/// scalar setter.
#[semio_framework_async_macros::async_test]
async fn create_asset_obeys_the_inverse_and_absorb_laws() {
    let base = representative_snapshot();
    let create = ShootingMutation::CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset: sample_asset("a9"), index: None });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &create).await;
    let d1 = create.diff(&base).into_parts().0;
    let after = protocol::apply_diff_cold(&d1, &base).expect("valid mutation diff");
    let d2 = ShootingMutation::RenameAsset(crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset { id: "a9".into(), new_name: "Renamed".into() }).diff(&after).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn drag_assets_obeys_the_inverse_law() {
    let base = representative_snapshot();
    let drag = ShootingMutation::DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets { asset_ids: vec!["a1".into()], dx: 4.0, dy: -1.0, dz: 0.5 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &drag).await;
}

#[semio_framework_async_macros::async_test]
async fn set_active_shot_obeys_the_inverse_law() {
    let base = representative_snapshot();
    let set = ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: Some("s2".into()) });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &set).await;
}
/// ⚖️ A row deleted from the middle of its list is restored in place, and the inverse diffs sum to the negative diff.
#[semio_framework_async_macros::async_test]
async fn deleting_a_middle_row_is_undone_in_place() {
    let base = representative_snapshot();
    let delete = ShootingMutation::DeleteShot(crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot { id: "s1".into() });
    let restored = round_trip(&base, &delete);
    assert_eq!(restored.shots.iter().map(|shot| shot.id.as_str()).collect::<Vec<_>>(), vec!["s2"]);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&delete, &base).await;
}

/// ⚖️ A create with an index inserts at that position (clamped to the length), and undoing it removes exactly that row.
#[semio_framework_async_macros::async_test]
async fn create_at_an_index_inserts_there() {
    let base = representative_snapshot();
    for (index, expected) in [(Some(0), vec!["a9", "a1", "a2"]), (Some(1), vec!["a1", "a9", "a2"]), (Some(99), vec!["a1", "a2", "a9"]), (None, vec!["a1", "a2", "a9"])] {
        let create = ShootingMutation::CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset: sample_asset("a9"), index });
        let created = round_trip(&base, &create);
        assert_eq!(created.assets.iter().map(|asset| asset.id.as_str()).collect::<Vec<_>>(), expected);
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&create, &base).await;
    }
}

/// ⚖️ The relative gestures (drag, scale, half-turn) sum to the negative diff on exactly representable operands.
#[semio_framework_async_macros::async_test]
async fn gestures_obey_the_inverse_sum_law() {
    let mut base = representative_snapshot();
    base.assets[0].orientation = Some([0.0, 0.0, 0.0, 1.0]);
    base.assets[0].scale = Some([1.0, 2.0, 4.0]);
    let drag = ShootingMutation::DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets { asset_ids: vec!["a1".into(), "a2".into()], dx: 0.5, dy: -2.0, dz: 4.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&drag, &base).await;
    let scale = ShootingMutation::ScaleAssets(crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: 2.0, sy: 0.5, sz: 4.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&scale, &base).await;
    let half_turn = ShootingMutation::RotateAssets(crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::PI });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&half_turn, &base).await;
}

//#endregion ⚖️SemanticLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
/// (create/delete/rename/change/reorder/drag-rotate-scale/replace/set).
#[semio_framework_async_macros::async_test]
async fn create_asset_duplicate_id_is_fatal() {
    let base = representative_snapshot();
    let outcome = ShootingMutation::CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset { asset: sample_asset("a1"), index: None }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_asset_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::DeleteAsset(crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset { id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_asset_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::RenameAsset(crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset { id: "ghost".into(), new_name: "x".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_asset_url_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::ChangeAssetUrl(crate::standards::v1::subsets::any::schema::mutations::change_asset_url::ChangeAssetUrl { id: "ghost".into(), new_url: "/x.glb".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_assets_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::ReorderAssets(crate::standards::v1::subsets::any::schema::mutations::reorder_assets::ReorderAssets { id: "ghost".into(), to_index: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn drag_assets_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets { asset_ids: vec!["ghost".into()], dx: 1.0, dy: 1.0, dz: 1.0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn scale_assets_non_finite_is_fatal() {
    let base = representative_snapshot();
    let outcome = ShootingMutation::ScaleAssets(crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets { asset_ids: vec!["a1".into()], sx: f64::NAN, sy: 1.0, sz: 1.0 }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn rotate_assets_non_finite_is_fatal() {
    let base = representative_snapshot();
    let outcome = ShootingMutation::RotateAssets(crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets { asset_ids: vec!["a1".into()], ax: f64::NAN, ay: 0.0, az: 1.0, angle: 1.0 }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn replace_saved_camera_view_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::ReplaceSavedCameraView(crate::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::ReplaceSavedCameraView { id: "ghost".into(), new_camera: ShootingCamera::default() })).await;
}

#[semio_framework_async_macros::async_test]
async fn set_active_shot_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot { shot_id: Some("ghost".into()) })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_shot_duplicate_id_is_fatal() {
    let base = representative_snapshot();
    let outcome = ShootingMutation::CreateShot(crate::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot { shot: sample_shot("s1"), index: None }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_shot_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::DeleteShot(crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot { id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_shot_width_missing_target_is_error() {
    let base = representative_snapshot();
    assert_missing_target_is_error(&base, &ShootingMutation::ChangeShotWidth(crate::standards::v1::subsets::any::schema::mutations::change_shot_width::ChangeShotWidth { id: "ghost".into(), new_width: 100 })).await;
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
    let manifest = fixture_text!("../../../../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog shooting-1-any");
    }
}
//#endregion 🔖️KindsCatalog


#[test]
fn independent_shot_drawing_edits_and_undo_preserve_other_controls() {
    use protocol::{OpBinary, OpText};
    use set_shot_drawing::SetShotDrawing;
    use crate::{ShootingAssigned, ShootingDrawingPatch};

    let base = representative_snapshot();
    let set = |patch| ShootingMutation::SetShotDrawing(SetShotDrawing { id: "s1".into(), patch });
    let first = set(ShootingDrawingPatch { visible_width: Some(2.5), ..Default::default() });
    let other = set(ShootingDrawingPatch {
        fill_color: Some(ShootingAssigned::new(Some("#123".into()))),
        hidden_lines: Some(true),
        ..Default::default()
    });
    assert_eq!(ShootingMutation::parse_op(&first.print_op()).unwrap(), first);
    assert_eq!(ShootingMutation::decode_op(&first.encode_op().unwrap()).unwrap(), first);
    let with_first = round_trip(&base, &first);
    let with_both = round_trip(&with_first, &other);
    let drawing = with_both.shots[0].drawing.as_ref().unwrap();
    assert_eq!(drawing.visible_width, 2.5);
    assert_eq!(drawing.fill_color.as_deref(), Some("#123"));
    assert!(drawing.hidden_lines);
    assert_eq!(with_both.shots[1], base.shots[1]);

    let undo_first = first.inverse(&base).unwrap().pop().unwrap();
    let only_other = store::apply_mutation(&with_both, &undo_first).unwrap().0;
    assert_eq!(only_other, store::apply_mutation(&base, &other).unwrap().0);
    let clear = set(ShootingDrawingPatch { fill_color: Some(ShootingAssigned::new(None)), ..Default::default() });
    assert_eq!(round_trip(&only_other, &clear).shots[0].drawing.as_ref().unwrap().fill_color, None);
    let invalid = set(ShootingDrawingPatch { line_easing: Some(11.0), ..Default::default() });
    assert!(store::apply_mutation(&base, &invalid).is_err());
}

#[test]
fn shot_camera_first_capture_and_sparse_edits_survive_with_independent_undo() {
    use patch_shot_camera::PatchShotCamera;
    use protocol::{OpBinary, OpText};

    let base = representative_snapshot();
    let fallback = ShootingCamera::default();
    let edit = |id: &str, field: &str, camera: ShootingCamera| {
        ShootingMutation::PatchShotCamera(PatchShotCamera {
            shot_id: id.into(), field: field.into(), camera, fallback: fallback.clone(), release: None,
        })
    };
    let initial = edit("s2", "zoom", ShootingCamera { zoom: 2.5, ..fallback.clone() });
    assert_eq!(ShootingMutation::parse_op(&initial.print_op()).unwrap(), initial);
    assert_eq!(ShootingMutation::decode_op(&initial.encode_op().unwrap()).unwrap(), initial);
    let framed = round_trip(&base, &initial);
    assert_eq!(framed.shots[1].camera_id.as_deref(), Some("shot-camera:s2"));
    assert_eq!(framed.saved_cameras.last().unwrap().camera.zoom, 2.5);
    assert_eq!(framed.shots[0], base.shots[0]);

    let up = edit("s2", "upX", ShootingCamera { up: Some([0.5, 0.0, 1.0]), ..fallback.clone() });
    let both = round_trip(&framed, &up);
    let camera = &both.saved_cameras.last().unwrap().camera;
    assert_eq!(camera.zoom, 2.5);
    assert_eq!(camera.up, Some([0.5, 0.0, 1.0]));
    let undo_initial = initial.inverse(&base).unwrap().pop().unwrap();

    let remove_slot = ShootingMutation::DeleteSavedCamera(delete_saved_camera::DeleteSavedCamera { id: "shot-camera:s2".into() });
    let released = store::apply_mutation(&framed, &remove_slot).unwrap().0;
    assert_eq!(store::apply_mutation(&released, &undo_initial).unwrap().0, released);

    let only_up = store::apply_mutation(&both, &undo_initial).unwrap().0;
    assert_eq!(only_up, store::apply_mutation(&base, &up).unwrap().0);

    let mut shared = base.clone();
    shared.shots[1].camera_id = shared.shots[0].camera_id.clone();
    let isolated = round_trip(&shared, &initial);
    assert_eq!(isolated.saved_cameras[0], shared.saved_cameras[0]);
    assert_eq!(isolated.shots[0], shared.shots[0]);
    assert_eq!(isolated.shots[1].camera_id.as_deref(), Some("shot-camera:s2"));

    let invalid = edit("s2", "zoom", ShootingCamera { zoom: 0.0, ..fallback.clone() });
    assert!(store::apply_mutation(&base, &invalid).is_err());
}

#[test]
fn shot_auto_fit_retains_inheritance_and_isolates_other_shots() {
    use protocol::{OpBinary, OpText};
    use set_shot_auto_fit::SetShotAutoFit;

    let base = representative_snapshot();
    let set = |auto_fit| ShootingMutation::SetShotAutoFit(SetShotAutoFit { id: "s1".into(), auto_fit });
    for value in [None, Some(false), Some(true)] {
        let mutation = set(value);
        assert_eq!(ShootingMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(ShootingMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    assert!(ShootingMutation::parse_op(r#"{"mutation":"setShotAutoFit","id":"s1"}"#).is_err());
    let disabled = round_trip(&base, &set(Some(false)));
    assert_eq!(disabled.shots[0].auto_fit, Some(false));
    assert_eq!(disabled.shots[1], base.shots[1]);
    let enabled = round_trip(&disabled, &set(Some(true)));
    assert_eq!(enabled.shots[0].auto_fit, Some(true));
    assert_eq!(round_trip(&enabled, &set(None)), base);
}
