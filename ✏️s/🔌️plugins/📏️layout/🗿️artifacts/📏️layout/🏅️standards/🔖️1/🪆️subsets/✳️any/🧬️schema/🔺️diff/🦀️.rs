//! 🧬️ Layout diff schema — sparse field delta over the artifact.

use crate::artifacts::layout::{CharacterStyle, GridSettings, ImageLink, ImageLinkPatch, LayoutDrawingChild, Page, PagePatch, ParagraphStyle, ParentPage, Spread, TextStory, TextStoryPatch};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the layout artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::layout::schema::LayoutArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    pub grid: Option<GridSettings>,
    #[state(artifact)]
    pub paragraph_styles: Option<LayoutParagraphStylesDelta>,
    #[state(artifact)]
    pub character_styles: Option<LayoutCharacterStylesDelta>,
    #[state(artifact)]
    pub stories: Option<LayoutStoriesDelta>,
    #[state(artifact)]
    pub links: Option<LayoutLinksDelta>,
    #[state(artifact)]
    pub parent_pages: Option<LayoutParentPagesDelta>,
    #[state(artifact)]
    pub spreads: Option<LayoutSpreadsDelta>,
    #[state(artifact)]
    pub pages: Option<LayoutPagesDelta>,
    #[state(artifact)]
    pub print_target: Option<Option<String>>,
    #[state(artifact)]
    pub data_fields_json: Option<Option<String>>,
    /// 🖇️ Optional composed-child slot: outer `Option` = "did the presence/identity change", inner
    /// `Option` = "is it now present" — the same double-`Option` shape `✳️object`'s own `mesh` diff
    /// already established, per the migration recipe's §8 diff-shape convention.
    #[state(artifact)]
    pub background_drawing: Option<Option<LayoutDrawingChild>>,
    /// 🔗️ Same double-`Option` shape as `background_drawing`, for the forward link slot.
    #[state(artifact)]
    pub referenced_model: Option<Option<store::ArtifactLink>>,
    #[state(presence)]
    pub selected_ids: Option<LayoutStringList>,
    #[state(config)]
    pub active_page_id: Option<String>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub camera_x: Option<f64>,
    #[state(config)]
    pub camera_y: Option<f64>,
    #[state(config)]
    pub camera_zoom: Option<f64>,
    #[state(config)]
    pub preview_camera_x: Option<f64>,
    #[state(config)]
    pub preview_camera_y: Option<f64>,
    #[state(config)]
    pub preview_camera_zoom: Option<f64>,
    #[state(config)]
    pub drop_preview: Option<crate::artifacts::layout::LayoutDropPreviewState>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(artifact)]
    pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutStringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutPagesDelta {
    pub added: Vec<Page>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutPagePatchEntry {
    pub id: String,
    pub patch: PagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutStoriesDelta {
    pub added: Vec<TextStory>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutStoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutStoryPatchEntry {
    pub id: String,
    pub patch: TextStoryPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutLinksDelta {
    pub added: Vec<ImageLink>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutLinkPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutLinkPatchEntry {
    pub id: String,
    pub patch: ImageLinkPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutParagraphStylesDelta {
    pub added: Vec<ParagraphStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParagraphStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutParagraphStylePatchEntry {
    pub id: String,
    pub patch: ParagraphStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutCharacterStylesDelta {
    pub added: Vec<CharacterStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutCharacterStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutCharacterStylePatchEntry {
    pub id: String,
    pub patch: CharacterStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutParentPagesDelta {
    pub added: Vec<ParentPage>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParentPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutParentPagePatchEntry {
    pub id: String,
    pub patch: ParentPagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutSpreadsDelta {
    pub added: Vec<Spread>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutSpreadPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayoutSpreadPatchEntry {
    pub id: String,
    pub patch: SpreadPatch,
}

/// 🩹 Sparse patch for a {@link ParagraphStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct ParagraphStylePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link CharacterStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct CharacterStylePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link ParentPage}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct ParentPagePatch {
    pub name: Option<String>,
}

/// 🩹 Sparse patch for a {@link Spread}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct SpreadPatch {
    pub name: Option<String>,
}
//#endregion 🔖️DeltaHelpers
