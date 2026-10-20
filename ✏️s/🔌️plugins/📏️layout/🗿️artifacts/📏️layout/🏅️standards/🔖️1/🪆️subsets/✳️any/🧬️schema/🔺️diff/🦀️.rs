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
use semio_s_artifact_layout_layout_model::{CharacterStyle, Frame, GridSettings, ImageLink, Layer, LayoutDrawingChild, LayoutRect, Page, PageOverride, ParagraphStyle, ParentPage, Spread, TextStory, TextStyleRun};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::list_delta::RowPatch;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the layout artifact: id-keyed collection rows, owned-field patches and positional list rows.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
