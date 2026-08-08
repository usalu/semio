//! 🧬️ Shooting diff schema — sparse field delta over the artifact.

use crate::artifacts::shooting::{
    ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingSavedCamera, ShootingSavedCameraPatch,
    ShootingSceneLighting, ShootingShot, ShootingShotPatch,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::shooting::schema::ShootingArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub assets: Option<ShootingAssetsDelta>,
    #[state(persistent)]
    pub saved_cameras: Option<ShootingSavedCamerasDelta>,
    #[state(persistent)]
    pub scene: Option<ShootingSceneLighting>,
    #[state(persistent)]
    pub shots: Option<ShootingShotsDelta>,
    #[state(persistent)]
    pub active_shot_id: Option<String>,
    #[state(persistent)]
    pub active_asset_id: Option<String>,
    #[state(shared_ui)]
    pub selected_shot_ids: Option<ShootingStringList>,
    #[state(shared_ui)]
    pub selected_asset_ids: Option<ShootingStringList>,
    #[state(shared_ui)]
    pub active_utility_id: Option<String>,
    #[state(local_ui)]
    pub default_shot_format: Option<String>,
    #[state(local_ui)]
    pub default_shot_shape: Option<String>,
    #[state(local_ui)]
    pub default_asset_format: Option<String>,
    #[state(local_ui)]
    pub selection_method: Option<String>,
    #[state(local_ui)]
    pub center_model: Option<bool>,
    #[state(local_ui)]
    pub fit_revision: Option<u32>,
    #[state(local_ui)]
    pub camera_draft_label: Option<String>,
    #[state(local_ui)]
    pub camera: Option<ShootingCamera>,
    #[state(local_ui)]
    pub locale: Option<String>,
    #[state(preview)]
    pub hovered_asset_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `assets`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingAssetsDelta {
    pub added: Vec<ShootingAsset>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingAssetPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `shots`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingShotsDelta {
    pub added: Vec<ShootingShot>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingShotPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `savedCameras`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShootingSavedCamerasDelta {
    pub added: Vec<ShootingSavedCamera>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingSavedCameraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched asset entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingAssetPatchEntry {
    pub id: String,
    pub patch: ShootingAssetPatch,
}

/// 🩹 One patched shot entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingShotPatchEntry {
    pub id: String,
    pub patch: ShootingShotPatch,
}

/// 🩹 One patched saved-camera entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShootingSavedCameraPatchEntry {
    pub id: String,
    pub patch: ShootingSavedCameraPatch,
}
//#endregion 🔖️DeltaHelpers
