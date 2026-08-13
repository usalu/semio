//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gis3d.presence")]
pub struct Gis3dPresence {
    #[state(presence)] pub camera_json: String,
    #[state(presence)] pub selected_ids: Vec<String>,
}
