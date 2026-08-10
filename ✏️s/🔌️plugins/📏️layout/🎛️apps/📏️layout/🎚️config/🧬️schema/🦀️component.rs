//! 🧬️ schema leaf
use crate::artifacts::layout::{LayoutCamera, LayoutDropPreviewState};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout.config")]
pub struct LayoutConfig {
    #[state(local_ui)] pub active_page_id: String,
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub hovered_id: Option<String>,
    #[state(local_ui)] pub drop_preview: LayoutDropPreviewState,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera: LayoutCamera,
    #[state(local_ui)] pub preview_camera: LayoutCamera,
    #[state(local_ui)] pub locale: String,
}

