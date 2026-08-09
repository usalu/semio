//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.shooting.shooting.presence")]
pub struct ShootingPresence {
    #[state(shared_ui)] pub selected_shot_ids: Vec<String>,
    #[state(shared_ui)] pub selected_asset_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_asset_id: Option<String>,
    #[state(shared_ui)] pub camera: ShootingCamera,
    #[state(shared_ui)] pub active_utility_id: String,
}
