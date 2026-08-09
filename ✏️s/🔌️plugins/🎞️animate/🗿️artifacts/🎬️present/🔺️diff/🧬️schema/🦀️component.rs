//! 🧬️ Present diff schema — sparse field delta over the artifact.

use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the present artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::present::schema::PresentArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub source: Option<FigureTileSource>,
    #[state(persistent)]
    pub tiles: Option<PresentTilesDelta>,
    #[state(shared_ui)]
    pub selected_ids: Option<PresentStringList>,
    #[state(local_ui)]
    pub engagement_input: Option<String>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PresentStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `tiles`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PresentTilesDelta {
    pub added: Vec<FigureTileDraft>,
    pub removed: Vec<String>,
    pub patched: Vec<PresentTilePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched tile entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentTilePatchEntry {
    pub id: String,
    pub patch: FigureTileDraftPatch,
}
//#endregion 🔖️DeltaHelpers
