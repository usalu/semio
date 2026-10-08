//! 🧬️ Layout diff schema — sparse keyed delta over the artifact.

use crate::{CharacterStyle, GridSettings, ImageLink, ImageLinkPatch, LayoutDrawingChild, LayoutRect, Page, PageOverride, PagePatch, ParagraphStyle, ParentPage, Spread, TextStory, TextStoryPatch, TextStyleRun};
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

/// 🧩 Question-keyed rows of the data-field dictionary.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutDataEntriesDelta {
    pub added: Vec<crate::FormDictionaryEntry>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutDataEntryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched dictionary entry (whole-entry replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutDataEntryPatchEntry {
    pub id: String,
    pub item: crate::FormDictionaryEntry,
}

/// ✏️ One text edit of the imported plan: the text at `index` now reads `text`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutDrawingTextRow {
    pub index: u32,
    pub text: String,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutStringList {
    pub values: Vec<String>,
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

/// 🧩 Object-keyed rows of the page overrides.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct PageOverridesDelta {
    pub added: Vec<PageOverride>,
    pub removed: Vec<String>,
    pub patched: Vec<PageOverridePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched page override (whole-override replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PageOverridePatchEntry {
    pub id: String,
    pub item: PageOverride,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutPagesDelta {
    pub added: Vec<Page>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutPagePatchEntry {
    pub id: String,
    pub patch: PagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutStoriesDelta {
    pub added: Vec<TextStory>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutStoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutStoryPatchEntry {
    pub id: String,
    pub patch: TextStoryPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutLinksDelta {
    pub added: Vec<ImageLink>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutLinkPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutLinkPatchEntry {
    pub id: String,
    pub patch: ImageLinkPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutParagraphStylesDelta {
    pub added: Vec<ParagraphStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParagraphStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutParagraphStylePatchEntry {
    pub id: String,
    pub patch: ParagraphStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutCharacterStylesDelta {
    pub added: Vec<CharacterStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutCharacterStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutCharacterStylePatchEntry {
    pub id: String,
    pub patch: CharacterStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutParentPagesDelta {
    pub added: Vec<ParentPage>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParentPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutParentPagePatchEntry {
    pub id: String,
    pub patch: ParentPagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutSpreadsDelta {
    pub added: Vec<Spread>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutSpreadPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutSpreadPatchEntry {
    pub id: String,
    pub patch: SpreadPatch,
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
//#endregion 🔖️DeltaHelpers

use crate::LayoutSnapshot;
use protocol::{DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};

//#region 🔖️Insertion
/// 📍 Complete id order that places `id` at `index` among `ids`; `None` when it lands last, because appending is already the natural order of an added row.
pub fn insertion_order<'a>(ids: impl IntoIterator<Item = &'a str>, id: &str, index: Option<usize>) -> Option<Vec<String>> {
    let at = index?;
    let mut order: Vec<String> = ids.into_iter().map(str::to_owned).collect();
    (at < order.len()).then(|| {
        order.insert(at, id.to_owned());
        order
    })
}
//#endregion 🔖️Insertion

//#region 🔖️RowAlgebra
pub(crate) trait HasId {
    fn id(&self) -> &str;
}

macro_rules! impl_has_id {
    ($($item:ty => $field:ident),* $(,)?) => {
        $(impl HasId for $item {
            fn id(&self) -> &str {
                &self.$field
            }
        })*
    };
}

impl_has_id!(Page => id, TextStory => id, ImageLink => id, ParagraphStyle => id, CharacterStyle => id, ParentPage => id, Spread => id, PageOverride => object_id, crate::FormDictionaryEntry => question_id);

/// 🩹 How one patch row edits one item: applied, inverted against a base item into the sequence that undoes it, merged with a later row when that stays exact, and rebuilt between two items.
pub(crate) trait RowPatch<T>: Clone + PartialEq + Sized {
    fn applied(&self, item: &T) -> MutationApplyResult<T>;
    fn inverse_against(&self, base: &T) -> Vec<Self>;
    fn merged(self, later: Self) -> Result<Self, (Self, Self)>;
    fn between(base: &T, other: &T) -> Vec<Self>;
}

impl<T: Clone + PartialEq> RowPatch<T> for T {
    fn applied(&self, _item: &T) -> MutationApplyResult<T> {
        Ok(self.clone())
    }
    fn inverse_against(&self, base: &T) -> Vec<Self> {
        vec![base.clone()]
    }
    fn merged(self, later: Self) -> Result<Self, (Self, Self)> {
        Ok(later)
    }
    fn between(base: &T, other: &T) -> Vec<Self> {
        if base == other {
            Vec::new()
        } else {
            vec![other.clone()]
        }
    }
}

macro_rules! impl_field_patch {
    ($patch:ty, $item:ty, $inverse:expr) => {
        impl RowPatch<$item> for $patch {
            fn applied(&self, item: &$item) -> MutationApplyResult<$item> {
                let mut next = item.clone();
                next.apply_patch(self);
                Ok(next)
            }
            fn inverse_against(&self, base: &$item) -> Vec<Self> {
                let inverse: fn(&$patch, &$item) -> $patch = $inverse;
                vec![inverse(self, base)]
            }
            fn merged(self, later: Self) -> Result<Self, (Self, Self)> {
                Ok(later.or(self))
            }
            fn between(base: &$item, other: &$item) -> Vec<Self> {
                base.diff_patch(other).into_iter().collect()
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

impl_field_patch!(ParagraphStylePatch, ParagraphStyle, |patch, base| ParagraphStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    leading: patch.leading.map(|_| base.leading),
    tracking: patch.tracking.map(|_| base.tracking),
    alignment: patch.alignment.as_ref().map(|_| base.alignment.clone()),
});

impl_field_patch!(CharacterStylePatch, CharacterStyle, |patch, base| CharacterStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    italic: patch.italic.map(|_| base.italic),
    color: patch.color.map(|_| base.color),
    tracking: patch.tracking.map(|_| base.tracking),
});

impl_field_patch!(ParentPagePatch, ParentPage, |patch, base| ParentPagePatch { name: patch.name.as_ref().map(|_| base.name.clone()), width: patch.width.map(|_| base.width), height: patch.height.map(|_| base.height) });

impl_field_patch!(SpreadPatch, Spread, |patch, base| SpreadPatch { name: patch.name.as_ref().map(|_| base.name.clone()) });

impl_field_patch!(ImageLinkPatch, ImageLink, |patch, base| ImageLinkPatch {
    path: patch.path.as_ref().map(|_| base.path.clone()),
    width: patch.width.map(|_| base.width),
    height: patch.height.map(|_| base.height),
    dpi: patch.dpi.map(|_| base.dpi),
    color_profile: patch.color_profile.as_ref().map(|_| base.color_profile.clone().unwrap_or_default()),
});

impl RowPatch<TextStory> for TextStoryPatch {
    fn applied(&self, item: &TextStory) -> MutationApplyResult<TextStory> {
        let style_runs = match &self.style_runs {
            Some(delta) => apply_positional(&item.style_runs, delta).map_err(|error| error.under(["styleRuns"]))?,
            None => item.style_runs.clone(),
        };
        Ok(TextStory { id: item.id.clone(), content: self.content.clone().unwrap_or_else(|| item.content.clone()), style_runs })
    }
    fn inverse_against(&self, base: &TextStory) -> Vec<Self> {
        vec![Self { content: self.content.as_ref().map(|_| base.content.clone()), style_runs: self.style_runs.as_ref().map(|delta| inverse_positional(delta, &base.style_runs)) }]
    }
    fn merged(self, later: Self) -> Result<Self, (Self, Self)> {
        let style_runs = match (self.style_runs, later.style_runs) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        Ok(Self { content: later.content.or(self.content), style_runs })
    }
    fn between(base: &TextStory, other: &TextStory) -> Vec<Self> {
        let patch = Self { content: (base.content != other.content).then(|| other.content.clone()), style_runs: between_positional(&base.style_runs, &other.style_runs) };
        if patch == Self::default() {
            Vec::new()
        } else {
            vec![patch]
        }
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
    fn between(base: &GridSettings, other: &GridSettings) -> Option<Self> {
        let patch = Self {
            baseline_grid: (base.baseline_grid != other.baseline_grid).then_some(other.baseline_grid),
            baseline_offset: (base.baseline_offset != other.baseline_offset).then_some(other.baseline_offset),
            snap_to_baseline: (base.snap_to_baseline != other.snap_to_baseline).then_some(other.snap_to_baseline),
        };
        (patch != Self::default()).then_some(patch)
    }
}

/// 🧩 The shared shape of every id-keyed collection delta; repeated patch entries of one id apply in order.
pub(crate) trait Delta: Default + Clone {
    type Item: HasId + Clone;
    type Patch: RowPatch<Self::Item>;
    fn added(&self) -> &[Self::Item];
    fn removed(&self) -> &[String];
    fn patched(&self) -> Vec<(&str, &Self::Patch)>;
    fn reordered(&self) -> Option<&[String]>;
    fn from_parts(added: Vec<Self::Item>, removed: Vec<String>, patched: Vec<(String, Self::Patch)>, reordered: Option<Vec<String>>) -> Self;
}

macro_rules! impl_delta {
    ($delta:ty, $item:ty, $patch:ty, $entry:ident, $field:ident) => {
        impl Delta for $delta {
            type Item = $item;
            type Patch = $patch;
            fn added(&self) -> &[$item] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> Vec<(&str, &$patch)> {
                self.patched.iter().map(|entry| (entry.id.as_str(), &entry.$field)).collect()
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn from_parts(added: Vec<$item>, removed: Vec<String>, patched: Vec<(String, $patch)>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched: patched.into_iter().map(|(id, $field)| $entry { id, $field }).collect(), reordered }
            }
        }
    };
}

impl_delta!(LayoutPagesDelta, Page, PagePatch, LayoutPagePatchEntry, patch);
impl_delta!(LayoutStoriesDelta, TextStory, TextStoryPatch, LayoutStoryPatchEntry, patch);
impl_delta!(LayoutLinksDelta, ImageLink, ImageLinkPatch, LayoutLinkPatchEntry, patch);
impl_delta!(LayoutParagraphStylesDelta, ParagraphStyle, ParagraphStylePatch, LayoutParagraphStylePatchEntry, patch);
impl_delta!(LayoutCharacterStylesDelta, CharacterStyle, CharacterStylePatch, LayoutCharacterStylePatchEntry, patch);
impl_delta!(LayoutParentPagesDelta, ParentPage, ParentPagePatch, LayoutParentPagePatchEntry, patch);
impl_delta!(LayoutSpreadsDelta, Spread, SpreadPatch, LayoutSpreadPatchEntry, patch);
impl_delta!(PageOverridesDelta, PageOverride, PageOverride, PageOverridePatchEntry, item);
impl_delta!(LayoutDataEntriesDelta, crate::FormDictionaryEntry, crate::FormDictionaryEntry, LayoutDataEntryPatchEntry, item);
//#endregion 🔖️RowAlgebra

//#region 🔖️DeltaAlgebra
fn rejection(code: &str, message: &str, at: [String; 2]) -> MutationApplyError {
    MutationApplyError::new(code, message).at(at)
}

pub(crate) fn apply_delta<D: Delta>(items: &[D::Item], delta: &D) -> MutationApplyResult<Vec<D::Item>> {
    for (index, id) in delta.removed().iter().enumerate() {
        if !items.iter().any(|item| item.id() == id) {
            return Err(rejection("mutation.apply.missing-target", "removed item does not exist", ["removed".into(), index.to_string()]));
        }
        if delta.removed()[..index].contains(id) {
            return Err(rejection("mutation.apply.duplicate-target", "item is removed more than once", ["removed".into(), index.to_string()]));
        }
    }
    for (index, item) in delta.added().iter().enumerate() {
        let survives = items.iter().any(|existing| existing.id() == item.id()) && !delta.removed().iter().any(|id| id == item.id());
        if survives || delta.added()[..index].iter().any(|existing| existing.id() == item.id()) {
            return Err(rejection("mutation.apply.duplicate-target", "added item identity already exists", ["added".into(), index.to_string()]));
        }
    }
    let patched = delta.patched();
    for (index, (id, _)) in patched.iter().enumerate() {
        if !items.iter().any(|existing| existing.id() == *id) {
            return Err(rejection("mutation.apply.missing-target", "patched item does not exist", ["patched".into(), index.to_string()]));
        }
        if delta.removed().iter().any(|removed| removed == id) {
            return Err(rejection("mutation.apply.conflicting-target", "item cannot be removed and patched", ["patched".into(), index.to_string()]));
        }
    }
    let mut next: Vec<D::Item> = items.iter().filter(|item| !delta.removed().iter().any(|id| id == item.id())).cloned().collect();
    next.extend(delta.added().iter().cloned());
    for (id, patch) in patched {
        if let Some(position) = next.iter().position(|existing| existing.id() == id) {
            next[position] = patch.applied(&next[position]).map_err(|error| error.under(["patched", id]))?;
        }
    }
    if let Some(order) = delta.reordered() {
        if order.len() != next.len() {
            return Err(MutationApplyError::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), next.len())).at(["reordered"]));
        }
        let mut by_id: std::collections::BTreeMap<String, D::Item> = std::collections::BTreeMap::new();
        for item in next {
            if by_id.insert(item.id().to_string(), item).is_some() {
                return Err(MutationApplyError::new("mutation.apply.duplicate-target", "resulting collection contains duplicate identities").at(["identities"]));
            }
        }
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist or appears twice").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    Ok(next)
}

fn is_empty_delta<D: Delta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().is_empty() && delta.reordered().is_none()
}

enum Net<T, P> {
    Patch(Vec<P>),
    Remove,
    Add(T),
    Replace(T),
}

fn push_patch<P: Clone + PartialEq, T>(sequence: &mut Vec<P>, patch: P)
where
    P: RowPatch<T>,
{
    match sequence.pop() {
        Some(last) => match last.merged(patch) {
            Ok(merged) => sequence.push(merged),
            Err((earlier, later)) => {
                sequence.push(earlier);
                sequence.push(later);
            }
        },
        None => sequence.push(patch),
    }
}

/// ➕️ Composes `first` then `second` per id (patch∘patch → one patch when exact, add∘remove → nothing, remove∘add → replace) in a canonical row order.
pub(crate) fn absorb_delta<D: Delta>(first: D, second: D) -> D {
    let mut nets: std::collections::BTreeMap<String, Net<D::Item, D::Patch>> = std::collections::BTreeMap::new();
    let mut appended: Vec<String> = Vec::new();
    let second_removed: Vec<String> = second.removed().to_vec();
    let second_added: Vec<String> = second.added().iter().map(|item| item.id().to_string()).collect();
    let first_order = first.reordered().map(<[String]>::to_vec);
    let second_order = second.reordered().map(<[String]>::to_vec);
    for delta in [first, second] {
        let added = delta.added().to_vec();
        let removed = delta.removed().to_vec();
        let patched: Vec<(String, D::Patch)> = delta.patched().into_iter().map(|(id, patch)| (id.to_string(), patch.clone())).collect();
        for id in removed {
            match nets.remove(&id) {
                Some(Net::Add(_)) => appended.retain(|existing| existing != &id),
                Some(Net::Replace(_)) => {
                    appended.retain(|existing| existing != &id);
                    nets.insert(id, Net::Remove);
                }
                _ => {
                    nets.insert(id, Net::Remove);
                }
            }
        }
        for item in added {
            let id = item.id().to_string();
            let net = match nets.remove(&id) {
                Some(Net::Remove) => Net::Replace(item),
                _ => Net::Add(item),
            };
            appended.retain(|existing| existing != &id);
            appended.push(id.clone());
            nets.insert(id, net);
        }
        for (id, patch) in patched {
            let net = match nets.remove(&id) {
                None => Net::Patch(vec![patch]),
                Some(Net::Remove) => Net::Remove,
                Some(Net::Patch(mut sequence)) => {
                    push_patch::<D::Patch, D::Item>(&mut sequence, patch);
                    Net::Patch(sequence)
                }
                Some(Net::Add(item)) => Net::Add(patch.applied(&item).unwrap_or(item)),
                Some(Net::Replace(item)) => Net::Replace(patch.applied(&item).unwrap_or(item)),
            };
            nets.insert(id, net);
        }
    }
    let reordered = match (second_order, first_order) {
        (Some(order), _) => Some(order),
        (None, Some(order)) => Some(order.into_iter().filter(|id| !second_removed.contains(id)).chain(second_added.into_iter().filter(|id| nets.contains_key(id))).collect()),
        (None, None) => None,
    };
    let mut removed = Vec::new();
    let mut patched = Vec::new();
    let mut adds: std::collections::BTreeMap<String, D::Item> = std::collections::BTreeMap::new();
    for (id, net) in nets {
        match net {
            Net::Patch(sequence) => patched.extend(sequence.into_iter().map(|patch| (id.clone(), patch))),
            Net::Remove => removed.push(id),
            Net::Add(item) => {
                adds.insert(id, item);
            }
            Net::Replace(item) => {
                removed.push(id.clone());
                adds.insert(id, item);
            }
        }
    }
    let mut added: Vec<D::Item> = Vec::with_capacity(adds.len());
    if reordered.is_some() {
        added.extend(adds.into_values());
    } else {
        for id in &appended {
            if let Some(item) = adds.remove(id) {
                added.push(item);
            }
        }
    }
    D::from_parts(added, removed, patched, reordered)
}

fn absorb_optional<D: Delta>(first: &mut Option<D>, second: Option<D>) {
    let Some(second) = second else { return };
    let merged = absorb_delta(first.take().unwrap_or_default(), second);
    *first = (!is_empty_delta(&merged)).then_some(merged);
}

fn forward_order<D: Delta>(base_ids: &[String], delta: &D) -> Vec<String> {
    let mut ids: Vec<String> = base_ids.iter().filter(|id| !delta.removed().contains(id)).cloned().collect();
    ids.extend(delta.added().iter().map(|item| item.id().to_string()));
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => ids,
    }
}

/// 🔁️ The negative delta against `base`: patch sequences are undone in reverse order against the item as it stood before each step.
pub(crate) fn inverse_delta<D: Delta>(delta: &D, base: &[D::Item]) -> D {
    let find = |id: &str| base.iter().find(|item| item.id() == id);
    let removed: Vec<String> = delta.added().iter().map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = delta.removed().iter().filter_map(|id| find(id).cloned()).collect();
    let mut groups: Vec<(String, Vec<&D::Patch>)> = Vec::new();
    for (id, patch) in delta.patched() {
        match groups.iter_mut().find(|(known, _)| known == id) {
            Some((_, sequence)) => sequence.push(patch),
            None => groups.push((id.to_string(), vec![patch])),
        }
    }
    let mut patched: Vec<(String, D::Patch)> = Vec::new();
    for (id, sequence) in groups {
        let Some(item) = find(&id) else { continue };
        let mut state = item.clone();
        let mut undo: Vec<Vec<D::Patch>> = Vec::new();
        for patch in sequence {
            undo.push(patch.inverse_against(&state));
            state = patch.applied(&state).unwrap_or(state);
        }
        patched.extend(undo.into_iter().rev().flatten().map(|patch| (id.clone(), patch)));
    }
    let base_ids: Vec<String> = base.iter().map(|item| item.id().to_string()).collect();
    let mut simulated: Vec<String> = forward_order(&base_ids, delta).into_iter().filter(|id| !removed.contains(id)).collect();
    simulated.extend(added.iter().map(|item| item.id().to_string()));
    let reordered = (simulated != base_ids).then_some(base_ids);
    D::from_parts(added, removed, patched, reordered)
}

fn inverse_optional<D: Delta>(delta: &Option<D>, base: &[D::Item]) -> Option<D> {
    delta.as_ref().map(|delta| inverse_delta(delta, base))
}

pub(crate) fn between_delta<D: Delta>(base: &[D::Item], other: &[D::Item]) -> Option<D> {
    let removed: Vec<String> = base.iter().filter(|item| !other.iter().any(|candidate| candidate.id() == item.id())).map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = other.iter().filter(|item| !base.iter().any(|candidate| candidate.id() == item.id())).cloned().collect();
    let patched: Vec<(String, D::Patch)> = base
        .iter()
        .filter_map(|item| other.iter().find(|candidate| candidate.id() == item.id()).map(|candidate| <D::Patch as RowPatch<D::Item>>::between(item, candidate).into_iter().map(|patch| (item.id().to_string(), patch)).collect::<Vec<_>>()))
        .flatten()
        .collect();
    let mut natural: Vec<String> = base.iter().filter(|item| !removed.iter().any(|id| id == item.id())).map(|item| item.id().to_string()).collect();
    natural.extend(added.iter().map(|item| item.id().to_string()));
    let target: Vec<String> = other.iter().map(|item| item.id().to_string()).collect();
    let reordered = (natural != target).then_some(target);
    (!(removed.is_empty() && added.is_empty() && patched.is_empty() && reordered.is_none())).then(|| D::from_parts(added, removed, patched, reordered))
}
//#endregion 🔖️DeltaAlgebra

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

pub(crate) fn between_positional<D: Positional>(base: &[D::Item], other: &[D::Item]) -> Option<D> {
    let rows: Vec<(usize, D::Item)> = other.iter().enumerate().filter(|(index, value)| base.get(*index) != Some(*value)).map(|(index, value)| (index, value.clone())).collect();
    let len = (base.len() != other.len()).then_some(other.len());
    (!(rows.is_empty() && len.is_none())).then(|| D::from_parts(len, rows))
}

/// 📍️ Whether a positional delta changes nothing.
fn is_empty_positional<D: Positional>(delta: &D) -> bool {
    delta.len().is_none() && delta.rows().is_empty()
}

/// 📍️ The guides of `page` after `delta` — the fallible half of [`Patchable`] for [`PagePatch`].
pub fn apply_page_guides(guides: &[LayoutRect], delta: &PageGuidesDelta) -> MutationApplyResult<Vec<LayoutRect>> {
    apply_positional(guides, delta)
}

/// 🧩 The overrides of a page after `delta` — the fallible half of [`Patchable`] for [`PagePatch`].
pub fn apply_page_overrides(overrides: &[PageOverride], delta: &PageOverridesDelta) -> MutationApplyResult<Vec<PageOverride>> {
    apply_delta(overrides, delta)
}

/// 📍️ The style runs of a story after `delta` — the fallible half of [`Patchable`] for [`TextStoryPatch`].
pub fn apply_story_runs(runs: &[TextStyleRun], delta: &TextStyleRunsDelta) -> MutationApplyResult<Vec<TextStyleRun>> {
    apply_positional(runs, delta)
}
//#endregion 🔖️PositionalAlgebra

//#region 🔖️PageAlgebra
impl RowPatch<Page> for PagePatch {
    fn applied(&self, item: &Page) -> MutationApplyResult<Page> {
        if let Some(delta) = &self.guides {
            apply_positional(&item.guides, delta).map_err(|error| error.under(["guides"]))?;
        }
        if let Some(delta) = &self.overrides {
            apply_delta(&item.overrides, delta).map_err(|error| error.under(["overrides"]))?;
        }
        let mut next = item.clone();
        next.apply_patch(self);
        Ok(next)
    }
    fn inverse_against(&self, base: &Page) -> Vec<Self> {
        self.undo_steps(base)
    }
    fn merged(self, later: Self) -> Result<Self, (Self, Self)> {
        if self.is_structural() || later.is_structural() {
            return Err((self, later));
        }
        Ok(self.merge_plain(later))
    }
    fn between(base: &Page, other: &Page) -> Vec<Self> {
        base.page_changes(other)
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
        }
    }

    /// 🔎️ The fields in which `other` differs from `frame`, or `None` when they agree.
    fn between(frame: &crate::Frame, other: &crate::Frame) -> Option<Self> {
        let (held, next) = (frame.bounds(), other.bounds());
        let mut patch = Self {
            x: (held.x != next.x).then_some(next.x),
            y: (held.y != next.y).then_some(next.y),
            width: (held.width != next.width).then_some(next.width),
            height: (held.height != next.height).then_some(next.height),
            rotation: (held.rotation != next.rotation).then_some(next.rotation),
            locked: (frame.locked() != other.locked()).then_some(other.locked()),
            visible: (frame.visible() != other.visible()).then_some(other.visible()),
            ..Default::default()
        };
        match (frame, other) {
            (crate::Frame::Rect { fill: held_fill, stroke: held_stroke, .. }, crate::Frame::Rect { fill, stroke, .. }) => {
                patch.fill = (held_fill != fill).then_some(*fill);
                patch.stroke = (held_stroke != stroke).then_some(*stroke);
            }
            (
                crate::Frame::Text { wrap_mode: held_wrap, columns: held_columns, story_id: held_story, thread_next: held_thread, inset: held_inset, .. },
                crate::Frame::Text { wrap_mode, columns, story_id, thread_next, inset, .. },
            ) => {
                patch.wrap_mode = (held_wrap != wrap_mode).then(|| wrap_mode.clone());
                patch.columns = (held_columns != columns).then_some(*columns);
                patch.story_id = (held_story != story_id).then(|| story_id.clone());
                patch.thread_next = (held_thread != thread_next).then(|| thread_next.clone());
                patch.inset_x = (held_inset.x != inset.x).then_some(inset.x);
                patch.inset_y = (held_inset.y != inset.y).then_some(inset.y);
                patch.inset_width = (held_inset.width != inset.width).then_some(inset.width);
                patch.inset_height = (held_inset.height != inset.height).then_some(inset.height);
            }
            _ => {}
        }
        (patch != Self::default()).then_some(patch)
    }
}

impl PagePatch {
    fn is_structural(&self) -> bool {
        self.frame_added.is_some() || self.frame_removed.is_some() || self.layer_added.is_some() || self.layer_removed.is_some() || self.frame_order.is_some() || self.frame_layer.is_some() || self.layer_patched.is_some()
    }

    /// 🔀️ Two non-structural patches folded into one: scalars and the parent last-wins, frame patches per frame, guide and override deltas composed.
    fn merge_plain(self, later: Self) -> Self {
        let mut frames_patched = self.frames_patched;
        for entry in later.frames_patched {
            match frames_patched.iter_mut().find(|known| known.frame_id == entry.frame_id) {
                Some(known) => known.patch = entry.patch.or(std::mem::take(&mut known.patch)),
                None => frames_patched.push(entry),
            }
        }
        let guides = match (self.guides, later.guides) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        let overrides = match (self.overrides, later.overrides) {
            (Some(first), Some(second)) => Some(absorb_delta(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        Self {
            name: later.name.or(self.name),
            width: later.width.or(self.width),
            height: later.height.or(self.height),
            margin_top: later.margin_top.or(self.margin_top),
            margin_right: later.margin_right.or(self.margin_right),
            margin_bottom: later.margin_bottom.or(self.margin_bottom),
            margin_left: later.margin_left.or(self.margin_left),
            columns_count: later.columns_count.or(self.columns_count),
            columns_gutter: later.columns_gutter.or(self.columns_gutter),
            frames_patched,
            parent_page_id: later.parent_page_id.or(self.parent_page_id),
            guides,
            overrides,
            ..Default::default()
        }
    }

    /// ↩️ The patch sequence undoing this patch against `base`, in reverse application order, each step an absolute setter of the base value.
    fn undo_steps(&self, base: &Page) -> Vec<Self> {
        let mut steps: Vec<Self> = Vec::new();
        if let Some(change) = &self.layer_patched {
            if let Some(layer) = base.layers.iter().find(|layer| layer.id == change.layer_id) {
                steps.push(Self {
                    layer_patched: Some(crate::PageLayerPatched { layer_id: layer.id.clone(), name: change.name.as_ref().map(|_| layer.name.clone()), visible: change.visible.map(|_| layer.visible), locked: change.locked.map(|_| layer.locked) }),
                    ..Default::default()
                });
            }
        }
        if let Some(change) = &self.frame_layer {
            if let Some(frame) = base.frames.iter().find(|frame| frame.id() == change.frame_id) {
                steps.push(Self { frame_layer: Some(crate::PageFrameLayer { frame_id: change.frame_id.clone(), layer_id: frame.layer_id().to_string() }), ..Default::default() });
            }
        }
        if self.frame_order.is_some() {
            steps.push(Self { frame_order: Some(base.frames.iter().map(|frame| frame.id().to_string()).collect()), ..Default::default() });
        }
        if let Some(id) = &self.layer_removed {
            if let Some(at) = base.layers.iter().position(|layer| layer.id == *id) {
                steps.push(Self { layer_added: Some(crate::PageLayerAdded { layer: base.layers[at].clone(), index: Some(at) }), ..Default::default() });
            }
        }
        if let Some(added) = &self.layer_added {
            steps.push(Self { layer_removed: Some(added.layer.id.clone()), ..Default::default() });
        }
        steps.push(Self {
            overrides: self.overrides.as_ref().map(|delta| inverse_delta(delta, &base.overrides)),
            guides: self.guides.as_ref().map(|delta| inverse_positional(delta, &base.guides)),
            parent_page_id: self.parent_page_id.as_ref().map(|_| base.parent_page_id.clone()),
            frames_patched: self
                .frames_patched
                .iter()
                .filter_map(|entry| base.frames.iter().find(|frame| frame.id() == entry.frame_id).map(|frame| crate::PageFramePatched { frame_id: entry.frame_id.clone(), patch: entry.patch.restoring(frame) }))
                .collect(),
            ..Default::default()
        });
        if let Some(id) = &self.frame_removed {
            if let Some(index) = base.frames.iter().position(|frame| frame.id() == id) {
                let layer_id = base.layers.iter().find(|layer| layer.object_ids.iter().any(|object| object == id)).map(|layer| layer.id.clone());
                steps.push(Self { frame_added: Some(crate::PageFrameAdded { frame: base.frames[index].clone(), index: Some(index), layer_id }), ..Default::default() });
            }
        }
        if let Some(added) = &self.frame_added {
            steps.push(Self { frame_removed: Some(added.frame.id().to_string()), ..Default::default() });
        }
        steps.push(Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            margin_top: self.margin_top.map(|_| base.margins.top),
            margin_right: self.margin_right.map(|_| base.margins.right),
            margin_bottom: self.margin_bottom.map(|_| base.margins.bottom),
            margin_left: self.margin_left.map(|_| base.margins.left),
            columns_count: self.columns_count.map(|_| base.columns.count),
            columns_gutter: self.columns_gutter.map(|_| base.columns.gutter),
            ..Default::default()
        });
        let mut compact: Vec<Self> = Vec::new();
        for step in steps.into_iter().filter(|step| step != &Self::default()) {
            match compact.pop() {
                Some(last) => match last.merged(step) {
                    Ok(merged) => compact.push(merged),
                    Err((earlier, later)) => {
                        compact.push(earlier);
                        compact.push(later);
                    }
                },
                None => compact.push(step),
            }
        }
        compact
    }
}

impl Page {
    /// 🧭️ The patch sequence carrying this page to `other`: plain fields and keyed deltas first, then removed, added and patched frames, layers, and a final frame order.
    fn page_changes(&self, other: &Page) -> Vec<PagePatch> {
        let mut steps: Vec<PagePatch> = Vec::new();
        let plain = PagePatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            width: (self.width != other.width).then_some(other.width),
            height: (self.height != other.height).then_some(other.height),
            margin_top: (self.margins.top != other.margins.top).then_some(other.margins.top),
            margin_right: (self.margins.right != other.margins.right).then_some(other.margins.right),
            margin_bottom: (self.margins.bottom != other.margins.bottom).then_some(other.margins.bottom),
            margin_left: (self.margins.left != other.margins.left).then_some(other.margins.left),
            columns_count: (self.columns.count != other.columns.count).then_some(other.columns.count),
            columns_gutter: (self.columns.gutter != other.columns.gutter).then_some(other.columns.gutter),
            parent_page_id: (self.parent_page_id != other.parent_page_id).then(|| other.parent_page_id.clone()),
            guides: between_positional(&self.guides, &other.guides),
            overrides: between_delta::<PageOverridesDelta>(&self.overrides, &other.overrides),
            ..Default::default()
        };
        if plain != PagePatch::default() {
            steps.push(plain);
        }
        let layer_of = |page: &Page, id: &str| page.layers.iter().find(|layer| layer.object_ids.iter().any(|object| object == id)).map(|layer| layer.id.clone());
        for frame in &self.frames {
            let kept = other.frames.iter().find(|candidate| candidate.id() == frame.id()).is_some_and(|candidate| candidate.kind_str() == frame.kind_str());
            if !kept {
                steps.push(PagePatch { frame_removed: Some(frame.id().to_string()), ..Default::default() });
            }
        }
        for (index, frame) in other.frames.iter().enumerate() {
            match self.frames.iter().find(|candidate| candidate.id() == frame.id()).filter(|held| held.kind_str() == frame.kind_str()) {
                None => steps.push(PagePatch { frame_added: Some(crate::PageFrameAdded { frame: frame.clone(), index: Some(index), layer_id: layer_of(other, frame.id()) }), ..Default::default() }),
                Some(held) => {
                    if let Some(patch) = crate::FramePatch::between(held, frame) {
                        steps.push(PagePatch { frames_patched: vec![crate::PageFramePatched { frame_id: frame.id().to_string(), patch }], ..Default::default() });
                    }
                    if layer_of(self, frame.id()) != layer_of(other, frame.id()) {
                        if let Some(layer_id) = layer_of(other, frame.id()) {
                            steps.push(PagePatch { frame_layer: Some(crate::PageFrameLayer { frame_id: frame.id().to_string(), layer_id }), ..Default::default() });
                        }
                    }
                }
            }
        }
        for layer in &self.layers {
            if other.layers.iter().all(|candidate| candidate.id != layer.id) {
                steps.push(PagePatch { layer_removed: Some(layer.id.clone()), ..Default::default() });
            }
        }
        for (index, layer) in other.layers.iter().enumerate() {
            match self.layers.iter().find(|candidate| candidate.id == layer.id) {
                None => steps.push(PagePatch { layer_added: Some(crate::PageLayerAdded { layer: layer.clone(), index: Some(index) }), ..Default::default() }),
                Some(held) if held.name != layer.name || held.visible != layer.visible || held.locked != layer.locked => {
                    steps.push(PagePatch {
                        layer_patched: Some(crate::PageLayerPatched { layer_id: layer.id.clone(), name: (held.name != layer.name).then(|| layer.name.clone()), visible: (held.visible != layer.visible).then_some(layer.visible), locked: (held.locked != layer.locked).then_some(layer.locked) }),
                        ..Default::default()
                    });
                }
                Some(_) => {}
            }
        }
        let mut state = self.clone();
        for step in &steps {
            state.apply_patch(step);
        }
        let reached: Vec<&str> = state.frames.iter().map(crate::Frame::id).collect();
        let target: Vec<&str> = other.frames.iter().map(crate::Frame::id).collect();
        if reached != target {
            steps.push(PagePatch { frame_order: Some(target.into_iter().map(str::to_string).collect()), ..Default::default() });
        }
        steps
    }
}
//#endregion 🔖️PageAlgebra

//#region 🔖️Apply
impl LayoutDiff {
    fn apply_data_fields(&self, base: &Option<crate::FormDictionary>) -> MutationApplyResult<Option<crate::FormDictionary>> {
        let Some(delta) = &self.data_fields else { return Ok(base.clone()) };
        let entries = delta.entries.clone().unwrap_or_default();
        Ok(match (base, delta.presence) {
            (_, Some(DataFieldsPresence::Deleted)) => None,
            (None, Some(DataFieldsPresence::Created)) | (Some(_), Some(DataFieldsPresence::Replaced)) | (None, Some(DataFieldsPresence::Replaced)) => Some(crate::FormDictionary { entries: apply_delta(&[], &entries).map_err(|error| error.under(["dataFields", "entries"]))? }),
            (Some(dictionary), _) => Some(crate::FormDictionary { entries: apply_delta(&dictionary.entries, &entries).map_err(|error| error.under(["dataFields", "entries"]))? }),
            (None, None) => {
                if is_empty_delta(&entries) {
                    None
                } else {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "data-field rows need an existing dictionary").at(["dataFields"]));
                }
            }
        })
    }
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {
    fn apply(&self, snapshot: &LayoutSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<LayoutSnapshot> {
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
            next.paragraph_styles = apply_delta(&next.paragraph_styles, delta).map_err(|error| error.under(["paragraphStyles"]))?;
        }
        if let Some(delta) = &self.character_styles {
            next.character_styles = apply_delta(&next.character_styles, delta).map_err(|error| error.under(["characterStyles"]))?;
        }
        if let Some(delta) = &self.parent_pages {
            next.parent_pages = apply_delta(&next.parent_pages, delta).map_err(|error| error.under(["parentPages"]))?;
        }
        if let Some(delta) = &self.spreads {
            next.spreads = apply_delta(&next.spreads, delta).map_err(|error| error.under(["spreads"]))?;
        }
        if let Some(delta) = &self.pages {
            next.pages = apply_delta(&next.pages, delta).map_err(|error| error.under(["pages"]))?;
        }
        if let Some(delta) = &self.stories {
            next.stories = apply_delta(&next.stories, delta).map_err(|error| error.under(["stories"]))?;
        }
        if let Some(delta) = &self.links {
            next.links = apply_delta(&next.links, delta).map_err(|error| error.under(["links"]))?;
        }
        if let Some(change) = &self.print_target {
            next.print_target = change.target.clone();
        }
        next.data_fields = self.apply_data_fields(&next.data_fields)?;
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
        absorb_optional(&mut self.paragraph_styles, other.paragraph_styles);
        absorb_optional(&mut self.character_styles, other.character_styles);
        absorb_optional(&mut self.parent_pages, other.parent_pages);
        absorb_optional(&mut self.spreads, other.spreads);
        absorb_optional(&mut self.pages, other.pages);
        absorb_optional(&mut self.stories, other.stories);
        absorb_optional(&mut self.links, other.links);
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
        (Some(first), Some(second)) => {
            let merged = absorb_delta(first, second);
            (!is_empty_delta(&merged)).then_some(merged)
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
            match delta.presence {
                Some(DataFieldsPresence::Created) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None },
                Some(DataFieldsPresence::Deleted) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(LayoutDataEntriesDelta { added: base_entries.to_vec(), ..Default::default() }) },
                Some(DataFieldsPresence::Replaced) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Replaced), entries: Some(LayoutDataEntriesDelta { added: base_entries.to_vec(), ..Default::default() }) },
                None => LayoutDataFieldsDelta { presence: None, entries: delta.entries.as_ref().map(|entries| inverse_delta(entries, base_entries)) },
            }
        });
        let labels = crate::mutations::set_drawing_text::drawing_labels(base);
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            grid: self.grid.as_ref().map(|patch| patch.inverse_against(&base.grid)),
            paragraph_styles: inverse_optional(&self.paragraph_styles, &base.paragraph_styles),
            character_styles: inverse_optional(&self.character_styles, &base.character_styles),
            stories: inverse_optional(&self.stories, &base.stories),
            links: inverse_optional(&self.links, &base.links),
            parent_pages: inverse_optional(&self.parent_pages, &base.parent_pages),
            spreads: inverse_optional(&self.spreads, &base.spreads),
            pages: inverse_optional(&self.pages, &base.pages),
            print_target: self.print_target.as_ref().map(|_| PrintTargetChange { target: base.print_target.clone() }),
            data_fields,
            background_drawing: self.background_drawing.as_ref().map(|_| base.background_drawing.clone()),
            drawing_texts: self.drawing_texts.iter().filter_map(|row| labels.get(row.index as usize).map(|text| LayoutDrawingTextRow { index: row.index, text: text.clone() })).collect(),
            referenced_model: self.referenced_model.as_ref().map(|_| base.referenced_model.clone()),
        }
    }
    fn between(base: &LayoutSnapshot, other: &LayoutSnapshot) -> Self {
        let data_fields = match (&base.data_fields, &other.data_fields) {
            (None, None) => None,
            (None, Some(next)) => Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(LayoutDataEntriesDelta { added: next.entries.clone(), ..Default::default() }) }),
            (Some(_), None) => Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None }),
            (Some(held), Some(next)) => between_delta::<LayoutDataEntriesDelta>(&held.entries, &next.entries).map(|entries| LayoutDataFieldsDelta { presence: None, entries: Some(entries) }),
        };
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            name: (base.name != other.name).then(|| other.name.clone()),
            grid: GridPatch::between(&base.grid, &other.grid),
            paragraph_styles: between_delta(&base.paragraph_styles, &other.paragraph_styles),
            character_styles: between_delta(&base.character_styles, &other.character_styles),
            stories: between_delta(&base.stories, &other.stories),
            links: between_delta(&base.links, &other.links),
            parent_pages: between_delta(&base.parent_pages, &other.parent_pages),
            spreads: between_delta(&base.spreads, &other.spreads),
            pages: between_delta(&base.pages, &other.pages),
            print_target: (base.print_target != other.print_target).then(|| PrintTargetChange { target: other.print_target.clone() }),
            data_fields,
            background_drawing: (base.background_drawing != other.background_drawing).then(|| other.background_drawing.clone()),
            drawing_texts: Vec::new(),
            referenced_model: (base.referenced_model != other.referenced_model).then(|| other.referenced_model.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.name.is_none()
            && self.grid.as_ref().is_none_or(|patch| patch == &GridPatch::default())
            && self.paragraph_styles.as_ref().is_none_or(is_empty_delta)
            && self.character_styles.as_ref().is_none_or(is_empty_delta)
            && self.stories.as_ref().is_none_or(is_empty_delta)
            && self.links.as_ref().is_none_or(is_empty_delta)
            && self.parent_pages.as_ref().is_none_or(is_empty_delta)
            && self.spreads.as_ref().is_none_or(is_empty_delta)
            && self.pages.as_ref().is_none_or(is_empty_delta)
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
