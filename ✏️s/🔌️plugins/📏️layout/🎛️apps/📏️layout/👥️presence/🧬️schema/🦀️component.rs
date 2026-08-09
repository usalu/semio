//! 🧬️ schema leaf
use crate::artifacts::layout::{LayoutCamera, LayoutDropPreviewState};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.layout.layout.presence")]
pub struct LayoutPresence {
    #[state(shared_ui)] pub active_page_id: String,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_id: Option<String>,
    #[state(shared_ui)] pub drop_preview: LayoutDropPreviewState,
    #[state(shared_ui)] pub camera: LayoutCamera,
    #[state(shared_ui)] pub preview_camera: LayoutCamera,
}
