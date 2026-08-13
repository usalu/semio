//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting.presence")]
pub struct ShootingPresence {
    #[state(presence)] pub selected_shot_ids: Vec<String>,
    #[state(presence)] pub selected_asset_ids: Vec<String>,
    #[state(presence)] pub hovered_asset_id: Option<String>,
    #[state(presence)] pub camera: ShootingCamera,
    #[state(presence)] pub active_utility_id: String,
}
