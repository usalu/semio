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
//! `📷set-shot-camera`→`📷replace-shot-camera`) and the orphan `🟤️set-snapshot` scaffold were
//! retired.

use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;

//#region 🔖️Operations
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<ShootingSnapshot, ShootingMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️.rs`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ShootingSnapshot, diff = ShootingDiff, schema = "shooting.shooting")]
pub enum ShootingMutation {
    CreateAsset(crate::standards::v1::subsets::any::schema::mutations::create_asset::CreateAsset),
    DeleteAsset(crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset),
    RenameAsset(crate::standards::v1::subsets::any::schema::mutations::rename_asset::RenameAsset),
    ChangeAssetUrl(crate::standards::v1::subsets::any::schema::mutations::change_asset_url::ChangeAssetUrl),
    ReorderAssets(crate::standards::v1::subsets::any::schema::mutations::reorder_assets::ReorderAssets),
    DragAssets(crate::standards::v1::subsets::any::schema::mutations::drag_assets::DragAssets),
    RotateAssets(crate::standards::v1::subsets::any::schema::mutations::rotate_assets::RotateAssets),
    ScaleAssets(crate::standards::v1::subsets::any::schema::mutations::scale_assets::ScaleAssets),
    CreateShot(crate::standards::v1::subsets::any::schema::mutations::create_shot::CreateShot),
    DeleteShot(crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot),
    RenameShot(crate::standards::v1::subsets::any::schema::mutations::rename_shot::RenameShot),
    ChangeShotWidth(crate::standards::v1::subsets::any::schema::mutations::change_shot_width::ChangeShotWidth),
    ChangeShotHeight(crate::standards::v1::subsets::any::schema::mutations::change_shot_height::ChangeShotHeight),
    ChangeShotFormat(crate::standards::v1::subsets::any::schema::mutations::change_shot_format::ChangeShotFormat),
    ChangeShotShape(crate::standards::v1::subsets::any::schema::mutations::change_shot_shape::ChangeShotShape),
    ReorderShots(crate::standards::v1::subsets::any::schema::mutations::reorder_shots::ReorderShots),
    ReplaceShotCamera(crate::standards::v1::subsets::any::schema::mutations::replace_shot_camera::ReplaceShotCamera),
    CreateSavedCamera(crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera),
    DeleteSavedCamera(crate::standards::v1::subsets::any::schema::mutations::delete_saved_camera::DeleteSavedCamera),
    RenameSavedCamera(crate::standards::v1::subsets::any::schema::mutations::rename_saved_camera::RenameSavedCamera),
    ReplaceSavedCameraView(crate::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::ReplaceSavedCameraView),
    ReorderSavedCameras(crate::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::ReorderSavedCameras),
    SetActiveShot(crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot),
    SetActiveAsset(crate::standards::v1::subsets::any::schema::mutations::set_active_asset::SetActiveAsset),
    ChangeSceneSunEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::ChangeSceneSunEnabled),
    ChangeSceneSunAzimuth(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::ChangeSceneSunAzimuth),
    ChangeSceneSunElevation(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::ChangeSceneSunElevation),
    ChangeSceneSunIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::ChangeSceneSunIntensity),
    ChangeSceneAmbientIntensity(crate::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::ChangeSceneAmbientIntensity),
    ChangeSceneShadowEnabled(crate::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::ChangeSceneShadowEnabled),
    ChangeSceneMaterialRoughness(crate::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::ChangeSceneMaterialRoughness),
}

/// 🏷️ The kebab spelling of every [`ShootingMutation`] variant, in DECLARATION ORDER — the one list
/// the language-neutral test platform is measured against. It is duplicated in exactly two other
/// places on purpose: this subset's own oracle manifest catalog `shooting-1-any`
/// (`../../🔣️oracle.json`), which the completeness gate counts, and the
/// `🎥️mutate-shooting-1` case adapter, which must not link this crate in the oracle role.
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
/// item. Same shape as `🎬️presentation`'s `apply_presentation_mutation`.
pub fn apply_shooting_mutation(snapshot: &ShootingSnapshot, mutation: &ShootingMutation) -> protocol::MutationApplyResult<ShootingSnapshot> {
    store::apply_mutation(snapshot, mutation).map(|(next, _messages)| next)
}

/// ↩️ Computes `mutation`'s inverse mutations against `snapshot` (pre-state).
pub fn inverse_shooting_mutation(snapshot: &ShootingSnapshot, mutation: &ShootingMutation) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
    <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::inverse(mutation, snapshot)?

    })
}
//#endregion 🔖️Apply

//#region 🔖️CaseBridges





//#endregion 🔖️CaseBridges

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
