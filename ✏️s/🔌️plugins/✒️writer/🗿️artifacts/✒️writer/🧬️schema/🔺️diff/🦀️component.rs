//! 🧬️ Writer diff schema — sparse field delta over the artifact.

use crate::artifacts::writer::{WriterEditorSelection, WriterEditorSettings};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta; `text` uses scalar replacement (not character-collection deltas).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::writer::schema::WriterArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub language_id: Option<String>,
    #[state(persistent)] pub uri: Option<String>,
    #[state(persistent)] pub text: Option<WriterTextDelta>,
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
/// ✂️ Text-sequence delta: optional whole-string replacement plus honest range edits.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterTextDelta {
    pub replacement: Option<String>,
    pub edits: Vec<WriterTextRangeEdit>,
}

/// ✂️ One byte-range replace (delete `[start, end)`, insert `insert`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterTextRangeEdit {
    pub start: u32,
    pub end: u32,
    pub insert: String,
}

/// 📋 String-list wrapper for optional list diffs across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
