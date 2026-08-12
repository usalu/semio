//! 🔺 Diff constructors for the `scene` facet's per-field mutation kinds. `ShootingDiff::scene` is
//! whole-struct-when-present (not itself sparse), so every constructor here clones `base.scene`,
//! writes its one field, and wraps the clone.

use super::mutation::{
    ChangeSceneAmbientIntensity, ChangeSceneMaterialRoughness, ChangeSceneShadowEnabled, ChangeSceneSunAzimuth, ChangeSceneSunElevation, ChangeSceneSunEnabled, ChangeSceneSunIntensity,
};
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

//#region ☀️ChangeSceneSunEnabled
pub fn diff_change_scene_sun_enabled(payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.enabled = payload.new_enabled;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion ☀️ChangeSceneSunEnabled

//#region 🧭️ChangeSceneSunAzimuth
pub fn diff_change_scene_sun_azimuth(payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.azimuth = payload.new_azimuth;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 🧭️ChangeSceneSunAzimuth

//#region 🌅️ChangeSceneSunElevation
pub fn diff_change_scene_sun_elevation(payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.elevation = payload.new_elevation;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 🌅️ChangeSceneSunElevation

//#region 💡️ChangeSceneSunIntensity
pub fn diff_change_scene_sun_intensity(payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.intensity = payload.new_intensity;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 💡️ChangeSceneSunIntensity

//#region 🔅️ChangeSceneAmbientIntensity
pub fn diff_change_scene_ambient_intensity(payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.ambient.intensity = payload.new_intensity;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 🔅️ChangeSceneAmbientIntensity

//#region 🌑️ChangeSceneShadowEnabled
pub fn diff_change_scene_shadow_enabled(payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.shadow.enabled = payload.new_enabled;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 🌑️ChangeSceneShadowEnabled

//#region 🪨️ChangeSceneMaterialRoughness
pub fn diff_change_scene_material_roughness(payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.material.roughness = payload.new_roughness;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
//#endregion 🪨️ChangeSceneMaterialRoughness
