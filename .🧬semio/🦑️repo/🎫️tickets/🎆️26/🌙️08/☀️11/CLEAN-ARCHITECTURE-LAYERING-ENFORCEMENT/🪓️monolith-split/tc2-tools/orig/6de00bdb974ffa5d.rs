//! 🧱️ Grid, text style run and page guide deltas of the layout diff: the grid patch, the data-field presence deltas and the keyed row deltas of the text and guide lists. See [`super`].

use super::*;

//#region 🔖️DeltaHelpers
/// 📐 Owned-field patch of the baseline grid.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct GridPatch {
    pub baseline_grid: Option<f64>,
    pub baseline_offset: Option<f64>,
    pub snap_to_baseline: Option<bool>,
}

/// 🖨️ A present change distinguishes clearing the print target from leaving it alone.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(deny_unknown_fields)]
pub struct PrintTargetChange {
    pub target: Option<String>,
}

/// 🧾️ How the data-field dictionary came or went: created from absent, deleted, or replaced wholesale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum DataFieldsPresence {
    Created,
    Deleted,
    Replaced,
}

/// 🧾️ Delta of the optional data-field dictionary: its presence transition and keyed entry rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutDataFieldsDelta {
    pub presence: Option<DataFieldsPresence>,
    pub entries: Option<LayoutDataEntriesDelta>,
}

/// ✏️ One text edit of the imported plan: the text at `index` now reads `text`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutDrawingTextRow {
    pub index: u32,
    pub text: String,
}

/// 📍️ Positional rows of the style runs of one story: the final length (when it changes) and every position whose run differs from the base.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct TextStyleRunsDelta {
    pub len: Option<usize>,
    pub rows: Vec<TextStyleRunRow>,
}

/// 📍️ One positional style-run row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextStyleRunRow {
    pub index: usize,
    pub run: TextStyleRun,
}

/// 📍️ Positional rows of the guides of one page.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct PageGuidesDelta {
    pub len: Option<usize>,
    pub rows: Vec<PageGuideRow>,
}

/// 📍️ One positional guide row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
//#endregion 🔖️DeltaHelpers
