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
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::present::schema::PresentArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub source: Option<FigureTileSource>,
    #[state(artifact)]
    pub tiles: Option<PresentTilesDelta>,
    #[state(presence)]
    pub selected_ids: Option<PresentStringList>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
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
