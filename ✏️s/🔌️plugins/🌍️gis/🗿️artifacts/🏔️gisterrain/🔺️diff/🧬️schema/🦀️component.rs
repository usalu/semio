//! 🧬️ GIS terrain diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS terrain artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::gisterrain::schema::GisTerrainArtifact>>,
    #[state(persistent)] pub exaggeration: Option<f64>,
    #[state(persistent)] pub imported_features_json: Option<String>,
    #[state(shared_ui)] pub selected_ids: Option<GisTerrainStringList>,
    #[state(local_ui)] pub camera_json: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GisTerrainStringList {
    pub values: Vec<String>,
}
//#endregion 🔹DeltaHelpers
