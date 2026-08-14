//! 🧬️ Writer diff schema — sparse field delta over the artifact.

use crate::artifacts::writer::{WriterDocumentChild, WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta; `document` carries a whole-handle replacement (content-addressed, so a
/// changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; writer's `document` slot is never absent,
/// only ever replaced, so a single `Option<WriterDocumentChild>` — not the double-`Option` an
/// optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterDiff {
    #[state(artifact)] pub artifact: Option<Box<crate::artifacts::writer::schema::WriterArtifact>>,
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub id: Option<String>,
    #[state(artifact)] pub language_id: Option<String>,
    #[state(artifact)] pub uri: Option<String>,
    #[state(artifact)] pub document: Option<WriterDocumentChild>,
    #[state(presence)] pub editor_selection: Option<Option<WriterEditorSelection>>,
    #[state(presence)] pub editor_settings: Option<WriterEditorSettings>,
    #[state(config)] pub format_signal: Option<u32>,
    #[state(config)] pub lint_signal: Option<u32>,
    #[state(config)] pub revision: Option<u32>,
    #[state(config)] pub engagement_input: Option<String>,
    #[state(config)] pub camera_x: Option<f64>,
    #[state(config)] pub camera_y: Option<f64>,
    #[state(config)] pub camera_zoom: Option<f64>,
    #[state(config)] pub locale: Option<String>,
}
//#endregion 🔖️Diff
