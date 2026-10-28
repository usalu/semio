//! 🧬️ Layout diff schema — positional keyed deltas (framework `protocol::list_delta`) over the artifact.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_layout_layout_model::set_frame_layer;
use semio_s_artifact_layout_layout_model::{CharacterStyle, Frame, GridSettings, ImageLink, Layer, LayoutDrawingMember, LayoutRect, Page, PageOverride, ParagraphStyle, ParentPage, Spread, TextStory, TextStyleRun};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::list_delta::RowPatch;
use protocol::{ApplyCapability, ColdDiffAlgebra, ColdMutationDiff, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the layout artifact: id-keyed collection rows, owned-field patches and positional list rows.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    pub background_drawing: Option<Option<LayoutDrawingMember>>,
    /// 🔗️ Same double-`Option` shape as `background_drawing`, for the forward link slot.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    pub referenced_model: Option<Option<store::ArtifactLink>>,
}
//#endregion 🔖️Diff

#[path = "🧱️grid-text-and-guide-deltas/🦀️.rs"]
mod grid_text_and_guide_deltas;
pub use grid_text_and_guide_deltas::*;

#[path = "🗂️data-entries-delta/🦀️.rs"]
mod data_entries_delta;
pub use data_entries_delta::*;

#[path = "🖼️page-frames-delta/🦀️.rs"]
mod page_frames_delta;
pub use page_frames_delta::*;

#[path = "🩹️row-patches/🦀️.rs"]
mod row_patches;
pub use row_patches::*;

#[path = "📍️positional-algebra/🦀️.rs"]
mod positional_algebra;
pub use positional_algebra::*;

#[path = "🧮️diff-apply-and-algebra/🦀️.rs"]
mod diff_apply_and_algebra;

#[path = "🧩️patchable-frames/🦀️.rs"]
mod patchable_frames;
pub use patchable_frames::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod io {
    #[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
    pub mod binary;
    #[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
    pub mod text;
}
//#endregion 🪢️TaxonomyMounts

/// 🫙️ A present nullable patch field retains its presence when the value is null.
pub(crate) fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

#[cfg(test)]
#[path = "🧪️tests/⚖️portable/🦀️.rs"]
mod portable_diff_tests;

#[cfg(any(test, feature = "testkit"))]
fn deserialize_double_option_serde<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> {
    <Option<T> as serde::Deserialize>::deserialize(deserializer).map(Some)
}

/// 🚦️ Decodes a present nullable setter under cumulative ownership admission.
pub(crate) fn deserialize_double_option_controlled<T: semio_framework_value::FromValue>(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Some)
}

/// 🫥️ Admits an absent nullable setter without turning it into an explicit clear.
pub(crate) fn default_double_option_controlled<T: semio_framework_value::FromValue>(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<Option<T>> as semio_framework_value::FromValue>::default_value_controlled(control)
}

/// 🧹️ Retires nullable setter owners through their declared value contracts.
pub(crate) fn retire_double_option<T: semio_framework_value::FromValue>(value: Option<Option<T>>) {
    <Option<Option<T>> as semio_framework_value::FromValue>::retire_decoded(value);
}

#[cfg(test)]
#[path = "🧪️tests/🫙️nullable/🦀️.rs"]
mod nullable_controlled_tests;

#[path="🧳️retained/🦀️.rs"]
pub mod retained;
