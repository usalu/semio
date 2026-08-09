//! 🧬️ Layout diff schema — sparse field delta over the artifact.

use crate::artifacts::layout::{
    CharacterStyle, GridSettings, ImageLink, ImageLinkPatch, Page, PagePatch, ParagraphStyle, ParentPage, Spread,
    TextStory, TextStoryPatch,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the layout artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::layout::schema::LayoutArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub name: Option<String>,
    #[state(persistent)]
    pub grid: Option<GridSettings>,
    #[state(persistent)]
    pub paragraph_styles: Option<LayoutParagraphStylesDelta>,
    #[state(persistent)]
    pub character_styles: Option<LayoutCharacterStylesDelta>,
    #[state(persistent)]
    pub stories: Option<LayoutStoriesDelta>,
    #[state(persistent)]
    pub links: Option<LayoutLinksDelta>,
    #[state(persistent)]
    pub parent_pages: Option<LayoutParentPagesDelta>,
    #[state(persistent)]
    pub spreads: Option<LayoutSpreadsDelta>,
    #[state(persistent)]
    pub pages: Option<LayoutPagesDelta>,
    #[state(persistent)]
    pub print_target: Option<Option<String>>,
    #[state(persistent)]
    pub data_fields_json: Option<Option<String>>,
    #[state(shared_ui)]
    pub selected_ids: Option<LayoutStringList>,
    #[state(local_ui)]
    pub active_page_id: Option<String>,
    #[state(local_ui)]
    pub engagement_input: Option<String>,
    #[state(local_ui)]
    pub camera_x: Option<f64>,
    #[state(local_ui)]
    pub camera_y: Option<f64>,
    #[state(local_ui)]
    pub camera_zoom: Option<f64>,
    #[state(local_ui)]
    pub preview_camera_x: Option<f64>,
    #[state(local_ui)]
    pub preview_camera_y: Option<f64>,
    #[state(local_ui)]
    pub preview_camera_zoom: Option<f64>,
    #[state(local_ui)]
    pub drop_preview: Option<crate::artifacts::layout::LayoutDropPreviewState>,
    #[state(local_ui)]
    pub locale: Option<String>,
    #[state(preview)]
    pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutStringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutPagesDelta {
    pub added: Vec<Page>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPagePatchEntry {
    pub id: String,
    pub patch: PagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutStoriesDelta {
    pub added: Vec<TextStory>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutStoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutStoryPatchEntry {
    pub id: String,
    pub patch: TextStoryPatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutLinksDelta {
    pub added: Vec<ImageLink>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutLinkPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutLinkPatchEntry {
    pub id: String,
    pub patch: ImageLinkPatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutParagraphStylesDelta {
    pub added: Vec<ParagraphStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParagraphStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutParagraphStylePatchEntry {
    pub id: String,
    pub patch: ParagraphStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutCharacterStylesDelta {
    pub added: Vec<CharacterStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutCharacterStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutCharacterStylePatchEntry {
    pub id: String,
    pub patch: CharacterStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutParentPagesDelta {
    pub added: Vec<ParentPage>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParentPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutParentPagePatchEntry {
    pub id: String,
    pub patch: ParentPagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LayoutSpreadsDelta {
    pub added: Vec<Spread>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutSpreadPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSpreadPatchEntry {
    pub id: String,
    pub patch: SpreadPatch,
}

/// 🩹 Sparse patch for a {@link ParagraphStyle}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ParagraphStylePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link CharacterStyle}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CharacterStylePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link ParentPage}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ParentPagePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link Spread}.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SpreadPatch {
    pub name: Option<String>,
}
//#endregion 🔖️DeltaHelpers
