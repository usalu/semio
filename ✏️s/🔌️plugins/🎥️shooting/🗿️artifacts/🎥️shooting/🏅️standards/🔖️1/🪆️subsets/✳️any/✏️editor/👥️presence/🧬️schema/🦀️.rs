//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting.presence")]
pub struct ShootingPresence {
    #[state(presence)]
    pub selected_shot_ids: Vec<String>,
    #[state(presence)]
    pub camera: ShootingCamera,
    #[state(presence)]
    pub active_utility_id: String,
}
