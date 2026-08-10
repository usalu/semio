//! 🧬️ schema leaf
use crate::artifacts::shooting::ShootingCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting.config")]
pub struct ShootingConfig {
    #[state(local_ui)] pub default_shot_format: String,
    #[state(local_ui)] pub default_shot_shape: String,
    #[state(local_ui)] pub default_asset_format: String,
    #[state(local_ui)] pub selected_shot_ids: Vec<String>,
    #[state(local_ui)] pub selected_asset_ids: Vec<String>,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_asset_id: Option<String>,
    #[state(local_ui)] pub center_model: bool,
    #[state(local_ui)] pub fit_revision: u32,
    #[state(local_ui)] pub camera_draft_label: String,
    #[state(local_ui)] pub camera: ShootingCamera,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

