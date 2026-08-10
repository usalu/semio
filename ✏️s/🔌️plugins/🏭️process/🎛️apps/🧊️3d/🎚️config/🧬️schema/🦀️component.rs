//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.3d.config")]
pub struct Process3dConfig {
    #[state(local_ui)] pub selected_id: Option<String>,
    #[state(local_ui)] pub hovered_id: Option<String>,
    #[state(local_ui)] pub selected_face_id: Option<u32>,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_position: [f64; 3],
    #[state(local_ui)] pub camera_target: [f64; 3],
    #[state(local_ui)] pub camera_fov: f64,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}

