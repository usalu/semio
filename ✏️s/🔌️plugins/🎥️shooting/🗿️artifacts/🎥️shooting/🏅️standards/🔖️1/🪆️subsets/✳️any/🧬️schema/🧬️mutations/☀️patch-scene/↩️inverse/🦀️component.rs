//! ↩ Inverse constructors for the `scene` facet's per-field mutation kinds — always applicable
//! (the document has exactly one `scene`, no missing-target case).

use super::mutation::{
    ChangeSceneAmbientIntensity, ChangeSceneMaterialRoughness, ChangeSceneShadowEnabled, ChangeSceneSunAzimuth, ChangeSceneSunElevation, ChangeSceneSunEnabled, ChangeSceneSunIntensity,
};
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region ☀️ChangeSceneSunEnabled
pub fn inverse_change_scene_sun_enabled(_payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunEnabled(ChangeSceneSunEnabled { new_enabled: base.scene.sun.enabled })]
}
//#endregion ☀️ChangeSceneSunEnabled

//#region 🧭️ChangeSceneSunAzimuth
pub fn inverse_change_scene_sun_azimuth(_payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunAzimuth(ChangeSceneSunAzimuth { new_azimuth: base.scene.sun.azimuth })]
}
//#endregion 🧭️ChangeSceneSunAzimuth

//#region 🌅️ChangeSceneSunElevation
pub fn inverse_change_scene_sun_elevation(_payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunElevation(ChangeSceneSunElevation { new_elevation: base.scene.sun.elevation })]
}
//#endregion 🌅️ChangeSceneSunElevation

//#region 💡️ChangeSceneSunIntensity
pub fn inverse_change_scene_sun_intensity(_payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunIntensity(ChangeSceneSunIntensity { new_intensity: base.scene.sun.intensity })]
}
//#endregion 💡️ChangeSceneSunIntensity

//#region 🔅️ChangeSceneAmbientIntensity
pub fn inverse_change_scene_ambient_intensity(_payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneAmbientIntensity(ChangeSceneAmbientIntensity { new_intensity: base.scene.ambient.intensity })]
}
//#endregion 🔅️ChangeSceneAmbientIntensity

//#region 🌑️ChangeSceneShadowEnabled
pub fn inverse_change_scene_shadow_enabled(_payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneShadowEnabled(ChangeSceneShadowEnabled { new_enabled: base.scene.shadow.enabled })]
}
//#endregion 🌑️ChangeSceneShadowEnabled

//#region 🪨️ChangeSceneMaterialRoughness
pub fn inverse_change_scene_material_roughness(_payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneMaterialRoughness(ChangeSceneMaterialRoughness { new_roughness: base.scene.material.roughness })]
}
//#endregion 🪨️ChangeSceneMaterialRoughness
