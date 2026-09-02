//! 🧬️ GIS terrain diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS terrain artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::gisterrain::schema::GisTerrainArtifact>>,
    #[state(artifact)]
    pub exaggeration: Option<f64>,
    #[state(artifact)]
    pub imported_features_json: Option<String>,
    #[state(config)]
    pub camera_json: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔹Diff
