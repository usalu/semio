//! 🧬️ Shooting diff schema — sparse field delta over the artifact.

use crate::{ShootingAsset, ShootingEmblemChild, ShootingSavedCamera, ShootingSceneLighting, ShootingShot};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the shooting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting")]
pub struct ShootingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::ShootingArtifact>>,
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
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `assets`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingAssetsDelta {
    pub added: Vec<ShootingAsset>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingAssetPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `shots`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingShotsDelta {
    pub added: Vec<ShootingShot>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingShotPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `savedCameras`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ShootingSavedCamerasDelta {
    pub added: Vec<ShootingSavedCamera>,
    pub removed: Vec<String>,
    pub patched: Vec<ShootingSavedCameraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched asset entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingAssetPatchEntry {
    pub id: String,
    pub patch: ShootingAssetPatch,
}

/// 🩹 One patched shot entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingShotPatchEntry {
    pub id: String,
    pub patch: ShootingShotPatch,
}

/// 🩹 One patched saved-camera entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShootingSavedCameraPatchEntry {
    pub id: String,
    pub patch: ShootingSavedCameraPatch,
}
//#endregion 🔖️DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::ShootingAssetPatch;
pub use crate::ShootingSavedCameraPatch;
pub use crate::ShootingShotPatch;
//#endregion 🔁️Re-exports
