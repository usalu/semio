//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gis2d.presence")]
pub struct Gis2dPresence {
    #[state(presence)] pub selected_ids: Vec<String>,
    #[state(presence)] pub camera_json: String,
    #[state(presence)] pub feature_selection_json: String,
    #[state(presence)] pub hover_json: String,
    #[state(presence)] pub selection_method: String,
    #[state(presence)] pub selection_mode: String,
}
