//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gis2d.presence")]
pub struct Gis2dPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub camera_json: String,
    #[state(shared_ui)] pub feature_selection_json: String,
    #[state(shared_ui)] pub hover_json: String,
    #[state(shared_ui)] pub selection_method: String,
    #[state(shared_ui)] pub selection_mode: String,
}
