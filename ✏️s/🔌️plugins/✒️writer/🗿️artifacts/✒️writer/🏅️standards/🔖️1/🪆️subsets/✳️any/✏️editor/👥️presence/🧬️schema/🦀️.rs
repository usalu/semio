//! 🧬️ schema leaf

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer.presence")]
pub struct WriterPresence {
    #[state(presence)]
    pub editor_selection: Option<WriterEditorSelection>,
    #[state(presence)]
    pub camera: WriterCamera,
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::WriterEditorSelection;
pub use crate::WriterCamera;
//#endregion 🔁️Re-exports
