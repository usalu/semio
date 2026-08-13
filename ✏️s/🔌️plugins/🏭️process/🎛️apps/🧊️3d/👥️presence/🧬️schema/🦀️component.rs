//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.3d.presence")]
pub struct Process3dPresence {
    #[state(presence)] pub selected_id: Option<String>,
    #[state(presence)] pub hovered_id: Option<String>,
    #[state(presence)] pub selected_face_id: Option<u32>,
    #[state(presence)] pub selection_method: String,
    #[state(presence)] pub engagement_input: String,
    #[state(presence)] pub camera_position: [f64; 3],
    #[state(presence)] pub camera_target: [f64; 3],
    #[state(presence)] pub camera_fov: f64,
    #[state(presence)] pub active_utility_id: String,
}
