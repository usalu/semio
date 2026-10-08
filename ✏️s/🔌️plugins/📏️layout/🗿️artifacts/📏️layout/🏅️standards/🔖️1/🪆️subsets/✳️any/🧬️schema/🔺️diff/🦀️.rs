//! 🧬️ Layout diff schema — positional keyed deltas (framework `protocol::list_delta`) over the artifact.

use crate::{CharacterStyle, Frame, FramePatch, GridSettings, ImageLink, ImageLinkPatch, Layer, LayoutDrawingChild, LayoutRect, Page, PageOverride, ParagraphStyle, ParentPage, Spread, TextStory, TextStoryPatch, TextStyleRun};
use crate::LayoutSnapshot;
use protocol::list_delta::RowPatch;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Diff
/// 🔺️ Sparse delta for the layout artifact: id-keyed collection rows, owned-field patches and positional list rows.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    pub grid: Option<GridPatch>,
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
    pub print_target: Option<PrintTargetChange>,
    #[state(artifact)]
    pub data_fields: Option<LayoutDataFieldsDelta>,
    /// 🖇️ Optional composed-child slot: outer `Option` = "did the presence/identity change", inner
    /// `Option` = "is it now present" — the same double-`Option` shape `✳️object`'s own `mesh` diff
    /// already established, per the migration recipe's §8 diff-shape convention.
    #[state(artifact)]
    pub background_drawing: Option<Option<LayoutDrawingChild>>,
    /// ✏️ Text edits of the imported plan, one row per text index, applied after any `background_drawing` replacement.
    #[state(artifact)]
    pub drawing_texts: Vec<LayoutDrawingTextRow>,
    /// 🔗️ Same double-`Option` shape as `background_drawing`, for the forward link slot.
    #[state(artifact)]
    pub referenced_model: Option<Option<store::ArtifactLink>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📐 Owned-field patch of the baseline grid.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GridPatch {
    pub baseline_grid: Option<f64>,
    pub baseline_offset: Option<f64>,
    pub snap_to_baseline: Option<bool>,
}

/// 🖨️ A present change distinguishes clearing the print target from leaving it alone.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(deny_unknown_fields)]
pub struct PrintTargetChange {
    pub target: Option<String>,
}

/// 🧾️ How the data-field dictionary came or went: created from absent, deleted, or replaced wholesale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DataFieldsPresence {
    Created,
    Deleted,
    Replaced,
}

/// 🧾️ Delta of the optional data-field dictionary: its presence transition and keyed entry rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutDataFieldsDelta {
    pub presence: Option<DataFieldsPresence>,
    pub entries: Option<LayoutDataEntriesDelta>,
}

/// ✏️ One text edit of the imported plan: the text at `index` now reads `text`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutDrawingTextRow {
    pub index: u32,
    pub text: String,
}

/// 📍️ Positional rows of the style runs of one story: the final length (when it changes) and every position whose run differs from the base.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct TextStyleRunsDelta {
    pub len: Option<usize>,
    pub rows: Vec<TextStyleRunRow>,
}

/// 📍️ One positional style-run row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextStyleRunRow {
    pub index: usize,
    pub run: TextStyleRun,
}

/// 📍️ Positional rows of the guides of one page.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct PageGuidesDelta {
    pub len: Option<usize>,
    pub rows: Vec<PageGuideRow>,
}

/// 📍️ One positional guide row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PageGuideRow {
    pub index: usize,
    pub guide: LayoutRect,
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `pages` list.
    pub LayoutPagesDelta {
        removal: LayoutPageRemoval,
        insertion: LayoutPageInsertion,
        relocation: LayoutPageRelocation,
        modification: LayoutPagesModification,
        row: Page,
        patch: PagePatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `stories` list.
    pub LayoutStoriesDelta {
        removal: LayoutStoryRemoval,
        insertion: LayoutStoryInsertion,
        relocation: LayoutStoryRelocation,
        modification: LayoutStoriesModification,
        row: TextStory,
        patch: TextStoryPatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `links` list.
    pub LayoutLinksDelta {
        removal: LayoutLinkRemoval,
        insertion: LayoutLinkInsertion,
        relocation: LayoutLinkRelocation,
        modification: LayoutLinksModification,
        row: ImageLink,
        patch: ImageLinkPatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `paragraphStyles` list.
    pub LayoutParagraphStylesDelta {
        removal: LayoutParagraphStyleRemoval,
        insertion: LayoutParagraphStyleInsertion,
        relocation: LayoutParagraphStyleRelocation,
        modification: LayoutParagraphStylesModification,
        row: ParagraphStyle,
        patch: ParagraphStylePatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `characterStyles` list.
    pub LayoutCharacterStylesDelta {
        removal: LayoutCharacterStyleRemoval,
        insertion: LayoutCharacterStyleInsertion,
        relocation: LayoutCharacterStyleRelocation,
        modification: LayoutCharacterStylesModification,
        row: CharacterStyle,
        patch: CharacterStylePatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `parentPages` list.
    pub LayoutParentPagesDelta {
        removal: LayoutParentPageRemoval,
        insertion: LayoutParentPageInsertion,
        relocation: LayoutParentPageRelocation,
        modification: LayoutParentPagesModification,
        row: ParentPage,
        patch: ParentPagePatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `spreads` list.
    pub LayoutSpreadsDelta {
        removal: LayoutSpreadRemoval,
        insertion: LayoutSpreadInsertion,
        relocation: LayoutSpreadRelocation,
        modification: LayoutSpreadsModification,
        row: Spread,
        patch: SpreadPatch,
        key: id
    }
}

protocol::plain_list_delta! {
    /// 🧩 Positional keyed rows of the `overrides` list.
    pub PageOverridesDelta {
        removal: PageOverrideRemoval,
        insertion: PageOverrideInsertion,
        relocation: PageOverrideRelocation,
        row: PageOverride,
        key: object_id
    }
}

/// 🧾️ One dictionary row keyed by its question; the forms crate owns the entry type, so the delta keys it through this wrapper.
#[derive(Clone, Debug, PartialEq)]
struct LayoutDataEntryRow(crate::FormDictionaryEntry);

impl protocol::list_delta::Keyed for LayoutDataEntryRow {
    type Key = String;
    fn key(&self) -> String {
        self.0.question_id.clone()
    }
}

/// ➖️ One `entries` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `entries` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryInsertion {
    pub index: usize,
    pub row: crate::FormDictionaryEntry,
}

/// ↕️ One `entries` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct LayoutDataEntryRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🧩 Positional keyed rows of the `entries` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct LayoutDataEntriesDelta {
    pub removed: Vec<LayoutDataEntryRemoval>,
    pub inserted: Vec<LayoutDataEntryInsertion>,
    pub moved: Vec<LayoutDataEntryRelocation>,
}

type LayoutDataEntriesDeltaParts = protocol::list_delta::Parts<LayoutDataEntryRow, protocol::list_delta::NoPatch>;

impl LayoutDataEntriesDelta {
    fn into_parts(self) -> LayoutDataEntriesDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, LayoutDataEntryRow(entry.row))).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: Vec::new(),
        }
    }

    fn from_parts(parts: LayoutDataEntriesDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| LayoutDataEntryRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| LayoutDataEntryInsertion { index, row: row.0 }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| LayoutDataEntryRelocation { id, from, to }).collect(),
        }
    }

    /// ➕️ The delta that inserts `row` at `index` of the after list.
    pub fn insertion(index: usize, row: crate::FormDictionaryEntry) -> Self {
        Self::from_parts(protocol::list_delta::Parts::insertion(index, LayoutDataEntryRow(row)))
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
    pub fn relocation_by_id(id: impl Into<String>, from: usize, to: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::relocation_by_id(id.into(), from, to))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &[crate::FormDictionaryEntry], capability: protocol::ApplyCapability) -> Result<Vec<crate::FormDictionaryEntry>, protocol::list_delta::ApplyError> {
        let rows = base.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<LayoutDataEntryRow>>();
        Ok(self.clone().into_parts().commit_onto(&rows, capability)?.into_iter().map(|row| row.0).collect())
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &[crate::FormDictionaryEntry]) -> Self {
        let rows = base.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<LayoutDataEntryRow>>();
        Self::from_parts(self.clone().into_parts().inverse(&rows))
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty()
    }
}

protocol::row_patch! {
    /// 🗂️ Owned-field patch of one layer: its name and the two flags.
    pub LayerPatch of Layer { set { name: String, visible: bool, locked: bool } }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `layers` list.
    pub PageLayersDelta {
        removal: PageLayerRemoval,
        insertion: PageLayerInsertion,
        relocation: PageLayerRelocation,
        modification: PageLayersModification,
        row: Layer,
        patch: LayerPatch,
        key: id
    }
}

impl protocol::list_delta::Keyed for Frame {
    type Key = String;
    fn key(&self) -> String {
        self.id().to_string()
    }
}

/// ➖️ One `frames` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PageFrameRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `frames` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PageFrameInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: Frame,
}

/// ↕️ One `frames` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PageFrameRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `frames` row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PageFramesModification {
    pub id: String,
    pub patch: FramePatch,
}

/// 🧩 Positional keyed rows of the `frames` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct PageFramesDelta {
    pub removed: Vec<PageFrameRemoval>,
    pub inserted: Vec<PageFrameInsertion>,
    pub moved: Vec<PageFrameRelocation>,
    pub modified: Vec<PageFramesModification>,
}

type PageFramesDeltaParts = protocol::list_delta::Parts<Frame, FramePatch>;

impl PageFramesDelta {
    fn into_parts(self) -> PageFramesDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    fn from_parts(parts: PageFramesDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| PageFrameRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| PageFrameInsertion { index, row: row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| PageFrameRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| PageFramesModification { id, patch }).collect(),
        }
    }

    /// ➕️ The delta that inserts `row` at `index` of the after list.
    pub fn insertion(index: usize, row: Frame) -> Self {
        Self::from_parts(protocol::list_delta::Parts::insertion(index, row))
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
    pub fn relocation_by_id(id: impl Into<String>, from: usize, to: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::relocation_by_id(id.into(), from, to))
    }

    /// 🩹 The delta that patches the row `id`.
    pub fn modification(id: impl Into<String>, patch: FramePatch) -> Self {
        Self::from_parts(protocol::list_delta::Parts::modification(id.into(), patch))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<Frame>, capability: protocol::ApplyCapability) -> Result<Vec<Frame>, protocol::list_delta::ApplyError> {
        self.clone().into_parts().commit_onto(base, capability)
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &Vec<Frame>) -> Self {
        Self::from_parts(self.clone().into_parts().inverse(base))
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| protocol::list_delta::RowPatch::<Frame>::is_empty(&entry.patch))
    }
}

/// 🩹 Sparse patch for a {@link ParagraphStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParagraphStylePatch {
    pub name: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub leading: Option<f64>,
    pub tracking: Option<f64>,
    pub alignment: Option<String>,
}

/// 🩹 Sparse patch for a {@link CharacterStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CharacterStylePatch {
    pub name: Option<Option<String>>,
    pub font_family: Option<Option<String>>,
    pub font_size: Option<Option<f64>>,
    pub font_weight: Option<Option<u32>>,
    pub italic: Option<Option<bool>>,
    pub color: Option<Option<[f32; 4]>>,
    pub tracking: Option<Option<f64>>,
}

/// 🩹 Sparse patch for a {@link ParentPage}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParentPagePatch {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// 🩹 Sparse patch for a {@link Spread}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct SpreadPatch {
    pub name: Option<String>,
}
/// 📄️ Owned-field patch of one page: its dimensions, margins, columns and parent, the positional guides, and the keyed overrides, frames and layers.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(default, deny_unknown_fields)]
pub struct PagePatch {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_right: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_left: Option<f64>,
    pub columns_count: Option<u32>,
    pub columns_gutter: Option<f64>,
    pub parent_page_id: Option<Option<String>>,
    pub guides: Option<PageGuidesDelta>,
    pub overrides: PageOverridesDelta,
    pub frames: PageFramesDelta,
    pub layers: PageLayersDelta,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️RowPatches
macro_rules! field_row_patch {
    ($patch:ty, $row:ty, |$patch_id:ident, $base_id:ident| $inverse:expr) => {
        impl RowPatch<$row> for $patch {
            fn commit_into(&self, row: &mut $row, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
                row.apply_patch(self);
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                let first = std::mem::take(self);
                *self = later.or(first);
            }
            fn inverse(&self, row: &$row) -> Self {
                let ($patch_id, $base_id) = (self, row);
                $inverse
            }
            fn is_empty(&self) -> bool {
                *self == Self::default()
            }
        }
    };
}

impl ParagraphStylePatch {
    fn or(self, earlier: Self) -> Self {
        Self {
            name: self.name.or(earlier.name),
            font_family: self.font_family.or(earlier.font_family),
            font_size: self.font_size.or(earlier.font_size),
            font_weight: self.font_weight.or(earlier.font_weight),
            leading: self.leading.or(earlier.leading),
            tracking: self.tracking.or(earlier.tracking),
            alignment: self.alignment.or(earlier.alignment),
        }
    }
}

impl CharacterStylePatch {
    fn or(self, earlier: Self) -> Self {
        Self {
            name: self.name.or(earlier.name),
            font_family: self.font_family.or(earlier.font_family),
            font_size: self.font_size.or(earlier.font_size),
            font_weight: self.font_weight.or(earlier.font_weight),
            italic: self.italic.or(earlier.italic),
            color: self.color.or(earlier.color),
            tracking: self.tracking.or(earlier.tracking),
        }
    }
}

impl ParentPagePatch {
    fn or(self, earlier: Self) -> Self {
        Self { name: self.name.or(earlier.name), width: self.width.or(earlier.width), height: self.height.or(earlier.height) }
    }
}

impl SpreadPatch {
    fn or(self, earlier: Self) -> Self {
        Self { name: self.name.or(earlier.name) }
    }
}

impl ImageLinkPatch {
    fn or(self, earlier: Self) -> Self {
        Self { path: self.path.or(earlier.path), width: self.width.or(earlier.width), height: self.height.or(earlier.height), dpi: self.dpi.or(earlier.dpi), color_profile: self.color_profile.or(earlier.color_profile) }
    }
}

field_row_patch!(ParagraphStylePatch, ParagraphStyle, |patch, base| ParagraphStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    leading: patch.leading.map(|_| base.leading),
    tracking: patch.tracking.map(|_| base.tracking),
    alignment: patch.alignment.as_ref().map(|_| base.alignment.clone()),
});

field_row_patch!(CharacterStylePatch, CharacterStyle, |patch, base| CharacterStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    italic: patch.italic.map(|_| base.italic),
    color: patch.color.map(|_| base.color),
    tracking: patch.tracking.map(|_| base.tracking),
});

field_row_patch!(ParentPagePatch, ParentPage, |patch, base| ParentPagePatch { name: patch.name.as_ref().map(|_| base.name.clone()), width: patch.width.map(|_| base.width), height: patch.height.map(|_| base.height) });

field_row_patch!(SpreadPatch, Spread, |patch, base| SpreadPatch { name: patch.name.as_ref().map(|_| base.name.clone()) });

field_row_patch!(ImageLinkPatch, ImageLink, |patch, base| ImageLinkPatch {
    path: patch.path.as_ref().map(|_| base.path.clone()),
    width: patch.width.map(|_| base.width),
    height: patch.height.map(|_| base.height),
    dpi: patch.dpi.map(|_| base.dpi),
    color_profile: patch.color_profile.as_ref().map(|_| base.color_profile.clone().unwrap_or_default()),
});

impl RowPatch<TextStory> for TextStoryPatch {
    fn commit_into(&self, row: &mut TextStory, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(content) = &self.content {
            row.content = content.clone();
        }
        if let Some(delta) = &self.style_runs {
            row.style_runs = apply_positional(&row.style_runs, delta).map_err(|error| error.under(["styleRuns"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.content = later.content.or_else(|| self.content.take());
        self.style_runs = match (self.style_runs.take(), later.style_runs) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
    }
    fn inverse(&self, row: &TextStory) -> Self {
        Self { content: self.content.as_ref().map(|_| row.content.clone()), style_runs: self.style_runs.as_ref().map(|delta| inverse_positional(delta, &row.style_runs)) }
    }
    fn is_empty(&self) -> bool {
        self.content.is_none() && self.style_runs.as_ref().is_none_or(is_empty_positional)
    }
}

impl crate::FramePatch {
    fn or(self, earlier: Self) -> Self {
        Self {
            x: self.x.or(earlier.x),
            y: self.y.or(earlier.y),
            width: self.width.or(earlier.width),
            height: self.height.or(earlier.height),
            rotation: self.rotation.or(earlier.rotation),
            fill: self.fill.or(earlier.fill),
            stroke: self.stroke.or(earlier.stroke),
            wrap_mode: self.wrap_mode.or(earlier.wrap_mode),
            columns: self.columns.or(earlier.columns),
            locked: self.locked.or(earlier.locked),
            visible: self.visible.or(earlier.visible),
            story_id: self.story_id.or(earlier.story_id),
            thread_next: self.thread_next.or(earlier.thread_next),
            inset_x: self.inset_x.or(earlier.inset_x),
            inset_y: self.inset_y.or(earlier.inset_y),
            inset_width: self.inset_width.or(earlier.inset_width),
            inset_height: self.inset_height.or(earlier.inset_height),
            layer_id: self.layer_id.or(earlier.layer_id),
        }
    }

    /// ↩️ The absolute setters restoring the fields this patch sets to the values `frame` holds.
    fn restoring(&self, frame: &crate::Frame) -> Self {
        let bounds = frame.bounds();
        let (fill, stroke) = match frame {
            crate::Frame::Rect { fill, stroke, .. } => (Some(*fill), Some(*stroke)),
            _ => (None, None),
        };
        let text = match frame {
            crate::Frame::Text { wrap_mode, columns, story_id, thread_next, inset, .. } => Some((wrap_mode, *columns, story_id, thread_next, inset)),
            _ => None,
        };
        Self {
            x: self.x.map(|_| bounds.x),
            y: self.y.map(|_| bounds.y),
            width: self.width.map(|_| bounds.width),
            height: self.height.map(|_| bounds.height),
            rotation: self.rotation.map(|_| bounds.rotation),
            fill: self.fill.and(fill),
            stroke: self.stroke.and(stroke),
            wrap_mode: self.wrap_mode.as_ref().and(text.map(|text| text.0.clone())),
            columns: self.columns.and(text.map(|text| text.1)),
            locked: self.locked.map(|_| frame.locked()),
            visible: self.visible.map(|_| frame.visible()),
            story_id: self.story_id.as_ref().and(text.map(|text| text.2.clone())),
            thread_next: self.thread_next.as_ref().and(text.map(|text| text.3.clone())),
            inset_x: self.inset_x.and(text.map(|text| text.4.x)),
            inset_y: self.inset_y.and(text.map(|text| text.4.y)),
            inset_width: self.inset_width.and(text.map(|text| text.4.width)),
            inset_height: self.inset_height.and(text.map(|text| text.4.height)),
            layer_id: self.layer_id.as_ref().map(|_| frame.layer_id().to_string()),
        }
    }
}

impl RowPatch<Frame> for FramePatch {
    fn commit_into(&self, row: &mut Frame, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        crate::apply_frame_field_patch(row, self);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let first = std::mem::take(self);
        *self = later.or(first);
    }
    fn inverse(&self, row: &Frame) -> Self {
        self.restoring(row)
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl RowPatch<Page> for PagePatch {
    fn commit_into(&self, row: &mut Page, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(value) = self.width {
            row.width = value;
        }
        if let Some(value) = self.height {
            row.height = value;
        }
        if let Some(value) = self.margin_top {
            row.margins.top = value;
        }
        if let Some(value) = self.margin_right {
            row.margins.right = value;
        }
        if let Some(value) = self.margin_bottom {
            row.margins.bottom = value;
        }
        if let Some(value) = self.margin_left {
            row.margins.left = value;
        }
        if let Some(value) = self.columns_count {
            row.columns.count = value;
        }
        if let Some(value) = self.columns_gutter {
            row.columns.gutter = value;
        }
        if let Some(parent) = &self.parent_page_id {
            row.parent_page_id = parent.clone();
        }
        if let Some(delta) = &self.guides {
            row.guides = apply_positional(&row.guides, delta).map_err(|error| error.under(["guides"]))?;
        }
        row.overrides = self.overrides.commit_onto(&row.overrides, capability).map_err(|error| error.under(["overrides"]))?;
        row.frames = self.frames.commit_onto(&row.frames, capability).map_err(|error| error.under(["frames"]))?;
        row.layers = self.layers.commit_onto(&row.layers, capability).map_err(|error| error.under(["layers"]))?;
        crate::derive_page_layers(row);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.width = later.width.or(self.width);
        self.height = later.height.or(self.height);
        self.margin_top = later.margin_top.or(self.margin_top);
        self.margin_right = later.margin_right.or(self.margin_right);
        self.margin_bottom = later.margin_bottom.or(self.margin_bottom);
        self.margin_left = later.margin_left.or(self.margin_left);
        self.columns_count = later.columns_count.or(self.columns_count);
        self.columns_gutter = later.columns_gutter.or(self.columns_gutter);
        self.parent_page_id = later.parent_page_id.or_else(|| self.parent_page_id.take());
        self.guides = match (self.guides.take(), later.guides) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        self.overrides.absorb(later.overrides);
        self.frames.absorb(later.frames);
        self.layers.absorb(later.layers);
    }
    fn inverse(&self, row: &Page) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.name.clone()),
            width: self.width.map(|_| row.width),
            height: self.height.map(|_| row.height),
            margin_top: self.margin_top.map(|_| row.margins.top),
            margin_right: self.margin_right.map(|_| row.margins.right),
            margin_bottom: self.margin_bottom.map(|_| row.margins.bottom),
            margin_left: self.margin_left.map(|_| row.margins.left),
            columns_count: self.columns_count.map(|_| row.columns.count),
            columns_gutter: self.columns_gutter.map(|_| row.columns.gutter),
            parent_page_id: self.parent_page_id.as_ref().map(|_| row.parent_page_id.clone()),
            guides: self.guides.as_ref().map(|delta| inverse_positional(delta, &row.guides)),
            overrides: self.overrides.inverse(&row.overrides),
            frames: self.frames.inverse(&row.frames),
            layers: self.layers.inverse(&row.layers),
        }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.margin_top.is_none()
            && self.margin_right.is_none()
            && self.margin_bottom.is_none()
            && self.margin_left.is_none()
            && self.columns_count.is_none()
            && self.columns_gutter.is_none()
            && self.parent_page_id.is_none()
            && self.guides.as_ref().is_none_or(is_empty_positional)
            && self.overrides.is_empty()
            && self.frames.is_empty()
            && self.layers.is_empty()
    }
}

impl GridPatch {
    fn applied(&self, base: &GridSettings) -> GridSettings {
        GridSettings {
            baseline_grid: self.baseline_grid.unwrap_or(base.baseline_grid),
            baseline_offset: self.baseline_offset.unwrap_or(base.baseline_offset),
            snap_to_baseline: self.snap_to_baseline.unwrap_or(base.snap_to_baseline),
        }
    }
    fn inverse_against(&self, base: &GridSettings) -> Self {
        Self { baseline_grid: self.baseline_grid.map(|_| base.baseline_grid), baseline_offset: self.baseline_offset.map(|_| base.baseline_offset), snap_to_baseline: self.snap_to_baseline.map(|_| base.snap_to_baseline) }
    }
    fn composed(&mut self, later: Self) {
        self.baseline_grid = later.baseline_grid.or(self.baseline_grid);
        self.baseline_offset = later.baseline_offset.or(self.baseline_offset);
        self.snap_to_baseline = later.snap_to_baseline.or(self.snap_to_baseline);
    }
}
//#endregion 🔖️RowPatches

//#region 🔖️PositionalAlgebra
/// 📍️ The shared shape of every positional list delta: the final length when it changes plus the rows whose value differs from the base.
pub(crate) trait Positional: Default + Clone {
    type Item: Clone + PartialEq;
    fn len(&self) -> Option<usize>;
    fn rows(&self) -> Vec<(usize, &Self::Item)>;
    fn from_parts(len: Option<usize>, rows: Vec<(usize, Self::Item)>) -> Self;
}

macro_rules! impl_positional {
    ($delta:ty, $item:ty, $row:ident, $field:ident) => {
        impl Positional for $delta {
            type Item = $item;
            fn len(&self) -> Option<usize> {
                self.len
            }
            fn rows(&self) -> Vec<(usize, &$item)> {
                self.rows.iter().map(|row| (row.index, &row.$field)).collect()
            }
            fn from_parts(len: Option<usize>, rows: Vec<(usize, $item)>) -> Self {
                Self { len, rows: rows.into_iter().map(|(index, $field)| $row { index, $field }).collect() }
            }
        }
    };
}

impl_positional!(TextStyleRunsDelta, TextStyleRun, TextStyleRunRow, run);
impl_positional!(PageGuidesDelta, LayoutRect, PageGuideRow, guide);

pub(crate) fn apply_positional<D: Positional>(items: &[D::Item], delta: &D) -> MutationApplyResult<Vec<D::Item>> {
    let target = delta.len().unwrap_or(items.len());
    let rows = delta.rows();
    if let Some((index, _)) = rows.iter().find(|(index, _)| *index >= target) {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "positional row lies beyond the final length").at(["rows".to_string(), index.to_string()]));
    }
    let mut next = Vec::with_capacity(target);
    for index in 0..target {
        match rows.iter().rev().find(|(row, _)| *row == index).map(|(_, value)| (*value).clone()).or_else(|| items.get(index).cloned()) {
            Some(value) => next.push(value),
            None => return Err(MutationApplyError::new("mutation.apply.missing-target", "extended position carries no row").at(["rows".to_string(), index.to_string()])),
        }
    }
    Ok(next)
}

pub(crate) fn absorb_positional<D: Positional>(first: D, second: D) -> D {
    let len = second.len().or(first.len());
    let mut rows: std::collections::BTreeMap<usize, D::Item> = first.rows().into_iter().map(|(index, value)| (index, value.clone())).collect();
    rows.extend(second.rows().into_iter().map(|(index, value)| (index, value.clone())));
    D::from_parts(len, rows.into_iter().filter(|(index, _)| len.is_none_or(|len| *index < len)).collect())
}

pub(crate) fn inverse_positional<D: Positional>(delta: &D, base: &[D::Item]) -> D {
    let target = delta.len().unwrap_or(base.len());
    let mut indices: std::collections::BTreeSet<usize> = delta.rows().into_iter().map(|(index, _)| index).filter(|index| *index < base.len()).collect();
    indices.extend(target..base.len());
    D::from_parts((target != base.len()).then_some(base.len()), indices.into_iter().map(|index| (index, base[index].clone())).collect())
}

/// 📍️ Whether a positional delta changes nothing.
fn is_empty_positional<D: Positional>(delta: &D) -> bool {
    delta.len().is_none() && delta.rows().is_empty()
}

/// 📍️ The style runs of a story after `delta` — the fallible half of [`Patchable`] for [`TextStoryPatch`].
pub fn apply_story_runs(runs: &[TextStyleRun], delta: &TextStyleRunsDelta) -> MutationApplyResult<Vec<TextStyleRun>> {
    apply_positional(runs, delta)
}
//#endregion 🔖️PositionalAlgebra

//#region 🔖️PayloadRows
/// 🎯️ The keyed rows that turn the held list into a whole-list payload: absent rows leave at their base index, new and changed rows enter at their payload index, and the unchanged rows outside the longest already-ordered run are relocated.
fn payload_parts<R: protocol::list_delta::Keyed<Key = String> + Clone + PartialEq>(held: &[R], payload: &[R]) -> protocol::list_delta::Parts<R, protocol::list_delta::NoPatch> {
    use protocol::list_delta::Keyed;
    let unchanged = |row: &R| payload.iter().any(|next| next.key() == row.key() && next == row);
    let removed: Vec<(String, usize)> = held.iter().enumerate().filter(|(_, row)| !unchanged(row)).map(|(at, row)| (row.key(), at)).collect();
    let inserted: Vec<(usize, R)> = payload.iter().enumerate().filter(|(_, next)| !held.iter().any(|row| row.key() == next.key() && row == *next)).map(|(at, next)| (at, next.clone())).collect();
    let survivors: Vec<(usize, usize)> = held.iter().enumerate().filter(|(_, row)| unchanged(row)).filter_map(|(from, row)| payload.iter().position(|next| next.key() == row.key()).map(|to| (from, to))).collect();
    let mut tails: Vec<usize> = Vec::new();
    let mut link: Vec<Option<usize>> = vec![None; survivors.len()];
    for (at, (_, to)) in survivors.iter().enumerate() {
        let slot = tails.partition_point(|tail| survivors[*tail].1 < *to);
        link[at] = slot.checked_sub(1).map(|before| tails[before]);
        if slot == tails.len() {
            tails.push(at);
        } else {
            tails[slot] = at;
        }
    }
    let mut kept = vec![false; survivors.len()];
    let mut cursor = tails.last().copied();
    while let Some(at) = cursor {
        kept[at] = true;
        cursor = link[at];
    }
    let moved: Vec<(String, usize, usize)> = survivors.iter().zip(&kept).filter(|(_, kept)| !**kept).map(|((from, to), _)| (held[*from].key(), *from, *to)).collect();
    protocol::list_delta::Parts { removed, inserted, moved, modified: Vec::new() }
}

/// 🎯️ The override rows that turn a page's held overrides into the payload overrides.
pub fn page_override_rows(held: &[PageOverride], payload: &[PageOverride]) -> PageOverridesDelta {
    PageOverridesDelta::from_parts(payload_parts(held, payload))
}

/// 🎯️ The dictionary rows that turn the held entries into the payload entries.
pub fn data_entry_rows(held: &[crate::FormDictionaryEntry], payload: &[crate::FormDictionaryEntry]) -> LayoutDataEntriesDelta {
    let wrap = |rows: &[crate::FormDictionaryEntry]| rows.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<_>>();
    LayoutDataEntriesDelta::from_parts(payload_parts(&wrap(held), &wrap(payload)))
}
//#endregion 🔖️PayloadRows

//#region 🔖️Apply
macro_rules! absorb_lists {
    ($first:ident, $second:ident, $($field:ident),+ $(,)?) => {$(
        if let Some(later) = $second.$field {
            let mut merged = $first.$field.take().unwrap_or_default();
            merged.absorb(later);
            $first.$field = (!merged.is_empty()).then_some(merged);
        }
    )+};
}

impl LayoutDiff {
    fn apply_data_fields(&self, base: &Option<crate::FormDictionary>, capability: ApplyCapability) -> MutationApplyResult<Option<crate::FormDictionary>> {
        let Some(delta) = &self.data_fields else { return Ok(base.clone()) };
        let entries = delta.entries.clone().unwrap_or_default();
        let commit = |held: &[crate::FormDictionaryEntry]| entries.commit_onto(held, capability).map_err(|error| error.under(["dataFields", "entries"]));
        Ok(match (base, delta.presence) {
            (_, Some(DataFieldsPresence::Deleted)) => None,
            (None, Some(DataFieldsPresence::Created)) | (Some(_), Some(DataFieldsPresence::Replaced)) | (None, Some(DataFieldsPresence::Replaced)) => Some(crate::FormDictionary { entries: commit(&[])? }),
            (Some(dictionary), _) => Some(crate::FormDictionary { entries: commit(&dictionary.entries)? }),
            (None, None) => {
                if entries.is_empty() {
                    None
                } else {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "data-field rows need an existing dictionary").at(["dataFields"]));
                }
            }
        })
    }
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {
    fn apply(&self, snapshot: &LayoutSnapshot, capability: ApplyCapability) -> MutationApplyResult<LayoutSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(patch) = &self.grid {
            next.grid = patch.applied(&next.grid);
        }
        if let Some(delta) = &self.paragraph_styles {
            next.paragraph_styles = delta.commit_onto(&next.paragraph_styles, capability).map_err(|error| error.under(["paragraphStyles"]))?;
        }
        if let Some(delta) = &self.character_styles {
            next.character_styles = delta.commit_onto(&next.character_styles, capability).map_err(|error| error.under(["characterStyles"]))?;
        }
        if let Some(delta) = &self.parent_pages {
            next.parent_pages = delta.commit_onto(&next.parent_pages, capability).map_err(|error| error.under(["parentPages"]))?;
        }
        if let Some(delta) = &self.spreads {
            next.spreads = delta.commit_onto(&next.spreads, capability).map_err(|error| error.under(["spreads"]))?;
        }
        if let Some(delta) = &self.pages {
            next.pages = delta.commit_onto(&next.pages, capability).map_err(|error| error.under(["pages"]))?;
        }
        if let Some(delta) = &self.stories {
            next.stories = delta.commit_onto(&next.stories, capability).map_err(|error| error.under(["stories"]))?;
        }
        if let Some(delta) = &self.links {
            next.links = delta.commit_onto(&next.links, capability).map_err(|error| error.under(["links"]))?;
        }
        if let Some(change) = &self.print_target {
            next.print_target = change.target.clone();
        }
        next.data_fields = self.apply_data_fields(&next.data_fields, capability)?;
        if let Some(value) = &self.background_drawing {
            next.background_drawing = value.clone();
        }
        if !self.drawing_texts.is_empty() {
            let child = next.background_drawing.take().ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "the document has no imported plan").at(["drawingTexts"]))?;
            let mut content = child.content.clone();
            for row in &self.drawing_texts {
                if !crate::set_drawing_text(&mut content, row.index, &row.text) {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "the imported plan has no text at that index").at(["drawingTexts".to_string(), row.index.to_string()]));
                }
            }
            next.background_drawing = Some(crate::background_drawing_child_handle("edit", &content));
        }
        if let Some(value) = &self.referenced_model {
            next.referenced_model = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if let Some(later) = other.grid {
            match &mut self.grid {
                Some(earlier) => earlier.composed(later),
                None => self.grid = Some(later),
            }
        }
        absorb_lists!(self, other, paragraph_styles, character_styles, parent_pages, spreads, pages, stories, links);
        if other.print_target.is_some() {
            self.print_target = other.print_target;
        }
        if let Some(later) = other.data_fields {
            self.data_fields = absorb_data_fields(self.data_fields.take(), later);
        }
        if other.background_drawing.is_some() {
            self.background_drawing = other.background_drawing;
            self.drawing_texts = other.drawing_texts;
        } else {
            let mut rows: std::collections::BTreeMap<u32, String> = std::mem::take(&mut self.drawing_texts).into_iter().map(|row| (row.index, row.text)).collect();
            rows.extend(other.drawing_texts.into_iter().map(|row| (row.index, row.text)));
            self.drawing_texts = rows.into_iter().map(|(index, text)| LayoutDrawingTextRow { index, text }).collect();
        }
        if other.referenced_model.is_some() {
            self.referenced_model = other.referenced_model;
        }
    }
}

fn absorb_data_fields(first: Option<LayoutDataFieldsDelta>, second: LayoutDataFieldsDelta) -> Option<LayoutDataFieldsDelta> {
    use DataFieldsPresence::{Created, Deleted, Replaced};
    let first = first.unwrap_or_default();
    let entries = |first: Option<LayoutDataEntriesDelta>, second: Option<LayoutDataEntriesDelta>| match (first, second) {
        (Some(mut first), Some(second)) => {
            first.absorb(second);
            (!first.is_empty()).then_some(first)
        }
        (first, None) => first,
        (None, second) => second,
    };
    let merged = match (first.presence, second.presence) {
        (_, Some(Deleted)) if first.presence == Some(Created) => LayoutDataFieldsDelta::default(),
        (_, Some(Deleted)) => LayoutDataFieldsDelta { presence: Some(Deleted), entries: None },
        (Some(Deleted), Some(Created | Replaced)) => LayoutDataFieldsDelta { presence: Some(Replaced), entries: second.entries },
        (_, Some(presence)) => LayoutDataFieldsDelta { presence: Some(presence), entries: second.entries },
        (Some(Deleted), None) => LayoutDataFieldsDelta { presence: Some(Deleted), entries: None },
        (presence, None) => LayoutDataFieldsDelta { presence, entries: entries(first.entries, second.entries) },
    };
    (merged != LayoutDataFieldsDelta::default()).then_some(merged)
}

impl DiffAlgebra<LayoutSnapshot> for LayoutDiff {
    fn inverse(&self, base: &LayoutSnapshot) -> Self {
        let data_fields = self.data_fields.as_ref().map(|delta| {
            let base_entries = base.data_fields.as_ref().map(|dictionary| dictionary.entries.as_slice()).unwrap_or_default();
            let all_base_rows = || LayoutDataEntriesDelta { inserted: base_entries.iter().cloned().enumerate().map(|(index, row)| LayoutDataEntryInsertion { index, row }).collect(), ..Default::default() };
            match delta.presence {
                Some(DataFieldsPresence::Created) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None },
                Some(DataFieldsPresence::Deleted) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(all_base_rows()) },
                Some(DataFieldsPresence::Replaced) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Replaced), entries: Some(all_base_rows()) },
                None => LayoutDataFieldsDelta { presence: None, entries: delta.entries.as_ref().map(|entries| entries.inverse(base_entries)) },
            }
        });
        let labels = crate::mutations::set_drawing_text::drawing_labels(base);
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            grid: self.grid.as_ref().map(|patch| patch.inverse_against(&base.grid)),
            paragraph_styles: self.paragraph_styles.as_ref().map(|delta| delta.inverse(&base.paragraph_styles)),
            character_styles: self.character_styles.as_ref().map(|delta| delta.inverse(&base.character_styles)),
            stories: self.stories.as_ref().map(|delta| delta.inverse(&base.stories)),
            links: self.links.as_ref().map(|delta| delta.inverse(&base.links)),
            parent_pages: self.parent_pages.as_ref().map(|delta| delta.inverse(&base.parent_pages)),
            spreads: self.spreads.as_ref().map(|delta| delta.inverse(&base.spreads)),
            pages: self.pages.as_ref().map(|delta| delta.inverse(&base.pages)),
            print_target: self.print_target.as_ref().map(|_| PrintTargetChange { target: base.print_target.clone() }),
            data_fields,
            background_drawing: self.background_drawing.as_ref().map(|_| base.background_drawing.clone()),
            drawing_texts: self.drawing_texts.iter().filter_map(|row| labels.get(row.index as usize).map(|text| LayoutDrawingTextRow { index: row.index, text: text.clone() })).collect(),
            referenced_model: self.referenced_model.as_ref().map(|_| base.referenced_model.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.name.is_none()
            && self.grid.as_ref().is_none_or(|patch| patch == &GridPatch::default())
            && self.paragraph_styles.as_ref().is_none_or(|delta| delta.is_empty())
            && self.character_styles.as_ref().is_none_or(|delta| delta.is_empty())
            && self.stories.as_ref().is_none_or(|delta| delta.is_empty())
            && self.links.as_ref().is_none_or(|delta| delta.is_empty())
            && self.parent_pages.as_ref().is_none_or(|delta| delta.is_empty())
            && self.spreads.as_ref().is_none_or(|delta| delta.is_empty())
            && self.pages.as_ref().is_none_or(|delta| delta.is_empty())
            && self.print_target.is_none()
            && self.data_fields.as_ref().is_none_or(|delta| delta == &LayoutDataFieldsDelta::default())
            && self.background_drawing.is_none()
            && self.drawing_texts.is_empty()
            && self.referenced_model.is_none()
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
