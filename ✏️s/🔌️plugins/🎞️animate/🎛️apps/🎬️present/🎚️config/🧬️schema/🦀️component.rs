//! 🧬️ Present app config schema — every local-ui field of PresentConfig.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎚️ Animate present app config — unshared local app state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present.config")]
pub struct PresentConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔖️Config

