//! 🧬️ Shooting diff schema — sparse field delta over the artifact.

use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingCamera, ShootingEmblemChild, ShootingSavedCamera, ShootingSavedCameraPatch, ShootingSceneLighting, ShootingShot, ShootingShotPatch};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::shooting::schema::ShootingArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub assets: Option<ShootingAssetsDelta>,
    #[state(artifact)]
    pub saved_cameras: Option<ShootingSavedCamerasDelta>,
    #[state(artifact)]
    pub scene: Option<ShootingSceneLighting>,
    #[state(artifact)]
    pub shots: Option<ShootingShotsDelta>,
    #[state(artifact)]
    pub active_shot_id: Option<String>,
    #[state(artifact)]
    pub active_asset_id: Option<String>,
    /// 🕸️ Composed `s.stdio.semio.image` child slot. Double-`Option` per the migration recipe's
    /// "optional slot" diff convention: outer = did the presence/identity change, inner = is it now
    /// present. No mutation triad currently sets this (see the artifact root's `🔖️Composition`
    /// doc comment) — present for schema completeness and future writers.
    #[state(artifact)]
    pub emblem: Option<Option<ShootingEmblemChild>>,
    #[state(presence)]
    pub selected_shot_ids: Option<ShootingStringList>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(config)]
    pub default_shot_format: Option<String>,
    #[state(config)]
    pub default_shot_shape: Option<String>,
    #[state(config)]
    pub default_asset_format: Option<String>,
    #[state(config)]
    pub center_model: Option<bool>,
    #[state(config)]
    pub fit_revision: Option<u32>,
    #[state(config)]
    pub camera_draft_label: Option<String>,
    #[state(config)]
    pub camera: Option<ShootingCamera>,
    #[state(config)]
    pub locale: Option<String>,
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
