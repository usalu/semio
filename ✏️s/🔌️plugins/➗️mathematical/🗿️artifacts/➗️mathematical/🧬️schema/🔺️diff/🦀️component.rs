//! 🧬️ Mathematical diff schema — sparse field delta over the artifact.

use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the mathematical artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::mathematical::schema::MathematicalArtifact>>,
    #[state(persistent)]
    pub graph: Option<MathematicalGraph>,
    #[state(persistent)]
    pub geometry: Option<MathematicalGeometry>,
    #[state(local_ui)]
    pub camera_x: Option<f64>,
    #[state(local_ui)]
    pub camera_y: Option<f64>,
    #[state(local_ui)]
    pub camera_zoom: Option<f64>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff
