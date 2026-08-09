//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer.presence")]
pub struct WriterPresence {
    #[state(shared_ui)] pub selected_ast_ids: Vec<String>,
    #[state(shared_ui)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(shared_ui)] pub tree_hovered_ast_id: Option<String>,
    #[state(shared_ui)] pub editor_hover_offset: Option<usize>,
    #[state(shared_ui)] pub camera: WriterCamera,
}
