//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gis3d.config")]
pub struct Gis3dConfig {
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
}

