//! 🧬️ schema leaf
use crate::artifacts::layout::{LayoutCamera, LayoutDropPreviewState};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.layout.layout.presence")]
pub struct LayoutPresence {
    #[state(presence)] pub active_page_id: String,
    #[state(presence)] pub selected_ids: Vec<String>,
    #[state(presence)] pub hovered_id: Option<String>,
    #[state(presence)] pub drop_preview: LayoutDropPreviewState,
    #[state(presence)] pub camera: LayoutCamera,
    #[state(presence)] pub preview_camera: LayoutCamera,
}
