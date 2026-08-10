//! 🧬️ Mathematical app config schema — every local-ui field of MathematicalConfig.

use crate::artifacts::mathematical::MathematicalCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎚️ Mathematical app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical.config")]
pub struct MathematicalConfig {
    #[state(local_ui)]
    pub camera: MathematicalCamera,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Config

