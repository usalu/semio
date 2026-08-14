//! 🧬️ schema leaf
use crate::artifacts::writer::{WriterCamera, WriterEditorSelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer.presence")]
pub struct WriterPresence {
    #[state(presence)] pub editor_selection: Option<WriterEditorSelection>,
    #[state(presence)] pub camera: WriterCamera,
}
