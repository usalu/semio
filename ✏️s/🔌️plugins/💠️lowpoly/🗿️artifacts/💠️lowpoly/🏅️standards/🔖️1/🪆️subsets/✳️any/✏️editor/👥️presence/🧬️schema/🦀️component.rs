//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly.presence")]
pub struct LowpolyPresence {
    #[state(presence)]
    pub world_camera_position: [f64; 3],
    #[state(presence)]
    pub world_camera_target: [f64; 3],
    #[state(presence)]
    pub world_camera_fov: f64,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(presence)]
    pub paint_utility: String,
}
