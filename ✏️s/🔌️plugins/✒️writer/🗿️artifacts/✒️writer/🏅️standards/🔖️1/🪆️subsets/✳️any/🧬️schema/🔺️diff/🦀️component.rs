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
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::writer::schema::WriterArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub language_id: Option<String>,
    #[state(persistent)] pub uri: Option<String>,
    #[state(persistent)] pub document: Option<WriterDocumentChild>,
    #[state(shared_ui)] pub selected_ast_ids: Option<WriterStringList>,
    #[state(shared_ui)] pub editor_selection: Option<Option<WriterEditorSelection>>,
    #[state(shared_ui)] pub editor_settings: Option<WriterEditorSettings>,
    #[state(local_ui)] pub format_signal: Option<u32>,
    #[state(local_ui)] pub lint_signal: Option<u32>,
    #[state(local_ui)] pub revision: Option<u32>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub camera_x: Option<f64>,
    #[state(local_ui)] pub camera_y: Option<f64>,
    #[state(local_ui)] pub camera_zoom: Option<f64>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub tree_hovered_ast_id: Option<Option<String>>,
    #[state(preview)] pub editor_hover_offset: Option<Option<usize>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper for optional list diffs across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
