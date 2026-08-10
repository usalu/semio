//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.config")]
pub struct WriterConfig {
    #[state(local_ui)] pub selected_ast_ids: Vec<String>,
    #[state(local_ui)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(local_ui)] pub format_signal: u32,
    #[state(local_ui)] pub lint_signal: u32,
    #[state(local_ui)] pub revision: u32,
    #[state(local_ui)] pub editor_settings: WriterEditorSettings,
    #[state(local_ui)] pub tree_hovered_ast_id: Option<String>,
    #[state(local_ui)] pub editor_hover_offset: Option<usize>,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera: WriterCamera,
    #[state(local_ui)] pub locale: String,
}

