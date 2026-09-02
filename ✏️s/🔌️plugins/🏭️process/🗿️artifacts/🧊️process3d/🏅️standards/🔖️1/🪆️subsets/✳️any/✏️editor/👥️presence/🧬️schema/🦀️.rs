//! 🧬️ schema leaf
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.3d.presence")]
pub struct Process3dPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub camera_position: [f64; 3],
    #[state(presence)]
    pub camera_target: [f64; 3],
    #[state(presence)]
    pub camera_fov: f64,
    #[state(presence)]
    pub active_utility_id: String,
}
