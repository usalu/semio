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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
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
pub fn inverse_shooting_mutation(snapshot: &ShootingSnapshot, mutation: &ShootingMutation) -> Vec<ShootingMutation> {
    <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🔖️CaseBridges
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "createAsset", …}`) JSON projection
/// — the shape the `🎥️mutate-shooting-1` case's `Examples` rows carry, and the shape every committed
/// per-kind leaf fixture under `<kind>/🧪️tests/*/🦠️mutation/🔣️.json` already is — into a real
/// [`ShootingMutation`], via this crate's own `dsl::os_pack::json` parser/bridge (no `serde_json`).
pub fn decode_shooting_mutation_json(text: &str) -> Result<ShootingMutation, String> {
    let json_value = dsl::os_pack::json::parse(text).map_err(|error| error.to_string())?;
    let dsl_value = dsl::os_pack::json::to_dsl_value(&json_value);
    dsl::FromValue::from_value(dsl_value).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed snapshot document — the `📸️snapshot/⬅️before/🔣️.json` every leaf
/// fixture of this vocabulary shares — into a real [`ShootingSnapshot`].
pub fn decode_shooting_snapshot_json(text: &str) -> Result<ShootingSnapshot, String> {
    let json_value = dsl::os_pack::json::parse(text).map_err(|error| error.to_string())?;
    let dsl_value = dsl::os_pack::json::to_dsl_value(&json_value);
    dsl::FromValue::from_value(dsl_value).map_err(|error| error.to_string())
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through. It belongs to the subset rather than
/// to a test adapter, because what counts as this document's meaning is this subset's ruling, not a
/// case's. Every inline artifact-lane field is present; the composed `emblem` child handle is not,
/// because it is a content address for an `s.stdio.semio.image` child that no kind of this
/// vocabulary addresses.
pub fn encode_shooting_projection_json(snapshot: &ShootingSnapshot) -> String {
    let assets = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&snapshot.assets));
    let saved_cameras = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&snapshot.saved_cameras));
    let scene = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&snapshot.scene));
    let shots = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&snapshot.shots));
    dsl::json!({
        "schema": snapshot.schema.as_str(),
        "assets": assets,
        "savedCameras": saved_cameras,
        "scene": scene,
        "shots": shots,
        "activeShotId": snapshot.active_shot_id.as_str(),
        "activeAssetId": snapshot.active_asset_id.as_str()
    })
    .to_string()
}
//#endregion 🔖️CaseBridges

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
