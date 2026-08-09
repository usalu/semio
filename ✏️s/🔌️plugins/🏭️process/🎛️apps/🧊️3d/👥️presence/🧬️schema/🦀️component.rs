//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.3d.presence")]
pub struct Process3dPresence {
    #[state(shared_ui)] pub selected_id: Option<String>,
    #[state(shared_ui)] pub hovered_id: Option<String>,
    #[state(shared_ui)] pub selected_face_id: Option<u32>,
    #[state(shared_ui)] pub selection_method: String,
    #[state(shared_ui)] pub engagement_input: String,
    #[state(shared_ui)] pub camera_position: [f64; 3],
    #[state(shared_ui)] pub camera_target: [f64; 3],
    #[state(shared_ui)] pub camera_fov: f64,
    #[state(shared_ui)] pub active_utility_id: String,
}
