//! 🧬️ Layout mutation vocabulary, codecs and funded original-source preparation.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

use semio_s_artifact_layout_layout_mutations_content::{change_data_fields, change_link_path, change_page_height, change_page_width, change_print_target, create_link, create_page, create_story, delete_link, delete_page, delete_story, edit_story, rename_layout, rename_page, reorder_pages, create_character_style, delete_character_style, set_page_guides, set_page_parent, set_story_runs, update_link, set_page_overrides, update_character_style, update_page_columns, update_page_margins, update_paragraph_style, update_parent_page, update_spread};
use semio_s_artifact_layout_layout_mutations_frames::{change_frame_columns, change_frame_fill, change_frame_stroke, change_frame_wrap_mode, create_frame, delete_frame, drag_frames, move_frame, rotate_frames, scale_frames, resize_frame, rotate_frame, set_frame_flags, update_grid, create_layer, set_frame_layer, reorder_frame, update_layer, update_text_frame};

use semio_s_artifact_layout_layout_mutations_content::{replace_document, set_parent_page_content, create_paragraph_style, delete_paragraph_style, create_parent_page, delete_parent_page, create_spread, delete_spread, set_page_spread, reorder_layer};

use semio_s_artifact_layout_layout_mutations_content::set_page_background_drawing;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = LayoutSnapshot, diff = LayoutDiff, schema = "s.layout.layout", reference = LayoutSnapshot, reference_diff = LayoutDiff)]
/// 🧩️ Schema-defined document events preserve their authored typed payloads.
pub enum LayoutMutation {
    RenameLayout(rename_layout::RenameLayout),
    ChangePrintTarget(change_print_target::ChangePrintTarget),
    ChangeDataFields(change_data_fields::ChangeDataFields),
    CreatePage(create_page::CreatePage),
    DeletePage(delete_page::DeletePage),
    RenamePage(rename_page::RenamePage),
    ChangePageWidth(change_page_width::ChangePageWidth),
    ChangePageHeight(change_page_height::ChangePageHeight),
    UpdatePageMargins(update_page_margins::UpdatePageMargins),
    UpdatePageColumns(update_page_columns::UpdatePageColumns),
    ReorderPages(reorder_pages::ReorderPages),
    CreateStory(create_story::CreateStory),
    DeleteStory(delete_story::DeleteStory),
    EditStory(edit_story::EditStory),
    CreateLink(create_link::CreateLink),
    DeleteLink(delete_link::DeleteLink),
    ChangeLinkPath(change_link_path::ChangeLinkPath),
    CreateFrame(create_frame::CreateFrame),
    DeleteFrame(delete_frame::DeleteFrame),
    MoveFrame(move_frame::MoveFrame),
    ResizeFrame(resize_frame::ResizeFrame),
    RotateFrame(rotate_frame::RotateFrame),
    ChangeFrameFill(change_frame_fill::ChangeFrameFill),
    ChangeFrameStroke(change_frame_stroke::ChangeFrameStroke),
    ChangeFrameWrapMode(change_frame_wrap_mode::ChangeFrameWrapMode),
    ChangeFrameColumns(change_frame_columns::ChangeFrameColumns),
    UpdateGrid(update_grid::UpdateGrid),
    SetFrameFlags(set_frame_flags::SetFrameFlags),
    UpdateParagraphStyle(update_paragraph_style::UpdateParagraphStyle),
    UpdateTextFrame(update_text_frame::UpdateTextFrame),
    UpdateLayer(update_layer::UpdateLayer),
    CreateCharacterStyle(create_character_style::CreateCharacterStyle),
    DeleteCharacterStyle(delete_character_style::DeleteCharacterStyle),
    UpdateCharacterStyle(update_character_style::UpdateCharacterStyle),
    UpdateParentPage(update_parent_page::UpdateParentPage),
    UpdateSpread(update_spread::UpdateSpread),
    SetPageParent(set_page_parent::SetPageParent),
    SetPageGuides(set_page_guides::SetPageGuides),
    SetStoryRuns(set_story_runs::SetStoryRuns),
    UpdateLink(update_link::UpdateLink),
    SetPageOverrides(set_page_overrides::SetPageOverrides),
    CreateLayer(create_layer::CreateLayer),
    SetFrameLayer(set_frame_layer::SetFrameLayer),
    ReorderFrame(reorder_frame::ReorderFrame),
    DragFrames(drag_frames::DragFrames),
    RotateFrames(rotate_frames::RotateFrames),
    ScaleFrames(scale_frames::ScaleFrames),
    CreateParagraphStyle(create_paragraph_style::CreateParagraphStyle),
    DeleteParagraphStyle(delete_paragraph_style::DeleteParagraphStyle),
    CreateParentPage(create_parent_page::CreateParentPage),
    DeleteParentPage(delete_parent_page::DeleteParentPage),
    CreateSpread(create_spread::CreateSpread),
    DeleteSpread(delete_spread::DeleteSpread),
    SetPageSpread(set_page_spread::SetPageSpread),
    ReorderLayer(reorder_layer::ReorderLayer),
    SetParentPageContent(set_parent_page_content::SetParentPageContent),
    ReplaceDocument(replace_document::ReplaceDocument),
    SetPageBackgroundDrawing(set_page_background_drawing::SetPageBackgroundDrawing),
}

//#endregion 🔖️Mutations

//#region 🔖️FrameSelection


//#endregion 🔖️FrameSelection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
pub const KINDS: &[&str] = &[
    "rename-layout",
    "change-print-target",
    "change-data-fields",
    "create-page",
    "delete-page",
    "rename-page",
    "change-page-width",
    "change-page-height",
    "update-page-margins",
    "update-page-columns",
    "reorder-pages",
    "create-story",
    "delete-story",
    "edit-story",
    "create-link",
    "delete-link",
    "change-link-path",
    "create-frame",
    "delete-frame",
    "move-frame",
    "resize-frame",
    "rotate-frame",
    "change-frame-fill",
    "change-frame-stroke",
    "change-frame-wrap-mode",
    "change-frame-columns",
    "update-grid",
    "set-frame-flags",
    "update-paragraph-style",
    "update-text-frame",
    "update-layer",
    "create-character-style",
    "delete-character-style",
    "update-character-style",
    "update-parent-page",
    "update-spread",
    "set-page-parent",
    "set-page-guides",
    "set-story-runs",
    "update-link",
    "set-page-overrides",
    "create-layer",
    "set-frame-layer",
    "set-drawing-text",
    "reorder-frame",
    "drag-frames",
    "rotate-frames",
    "scale-frames",
    "create-paragraph-style",
    "delete-paragraph-style",
    "create-parent-page",
    "delete-parent-page",
    "create-spread",
    "delete-spread",
    "set-page-spread",
    "reorder-layer",
    "set-parent-page-content",
    "replace-document",
    "set-page-background-drawing",
];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog

//#region 🪢️TaxonomyMounts
#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;
//#endregion 🪢️TaxonomyMounts

#[cfg(test)]
#[path = "🧪️tests/🏗️structure/🦀️.rs"]
mod structure_tests;

#[cfg(test)]
#[path = "🧪️tests/✅️validation/🦀️.rs"]
mod validation_tests;

#[cfg(test)]
#[path = "🧪️tests/🕸️graph/🦀️.rs"]
mod semantic_graph_tests;

#[path="🧳️retained/🦀️.rs"]
pub mod retained;

impl semio_s_artifact_layout_layout_mutations_decisions::LayoutMutationSource for LayoutMutation {
 fn view(&self)->semio_s_artifact_layout_layout_mutations_decisions::LayoutMutationView<'_>{use semio_s_artifact_layout_layout_mutations_decisions::LayoutMutationView as V;match self{Self::RenameLayout(p)=>V::RenameLayout(p),Self::ChangePrintTarget(p)=>V::ChangePrintTarget(p),Self::ChangeDataFields(p)=>V::ChangeDataFields(p),Self::CreatePage(p)=>V::CreatePage(p),Self::DeletePage(p)=>V::DeletePage(p),Self::RenamePage(p)=>V::RenamePage(p),Self::ChangePageWidth(p)=>V::ChangePageWidth(p),Self::ChangePageHeight(p)=>V::ChangePageHeight(p),Self::UpdatePageMargins(p)=>V::UpdatePageMargins(p),Self::UpdatePageColumns(p)=>V::UpdatePageColumns(p),Self::ReorderPages(p)=>V::ReorderPages(p),Self::CreateStory(p)=>V::CreateStory(p),Self::DeleteStory(p)=>V::DeleteStory(p),Self::EditStory(p)=>V::EditStory(p),Self::CreateLink(p)=>V::CreateLink(p),Self::DeleteLink(p)=>V::DeleteLink(p),Self::ChangeLinkPath(p)=>V::ChangeLinkPath(p),Self::CreateFrame(p)=>V::CreateFrame(p),Self::DeleteFrame(p)=>V::DeleteFrame(p),Self::MoveFrame(p)=>V::MoveFrame(p),Self::ResizeFrame(p)=>V::ResizeFrame(p),Self::RotateFrame(p)=>V::RotateFrame(p),Self::ChangeFrameFill(p)=>V::ChangeFrameFill(p),Self::ChangeFrameStroke(p)=>V::ChangeFrameStroke(p),Self::ChangeFrameWrapMode(p)=>V::ChangeFrameWrapMode(p),Self::ChangeFrameColumns(p)=>V::ChangeFrameColumns(p),Self::UpdateGrid(p)=>V::UpdateGrid(p),Self::SetFrameFlags(p)=>V::SetFrameFlags(p),Self::UpdateParagraphStyle(p)=>V::UpdateParagraphStyle(p),Self::UpdateTextFrame(p)=>V::UpdateTextFrame(p),Self::UpdateLayer(p)=>V::UpdateLayer(p),Self::CreateCharacterStyle(p)=>V::CreateCharacterStyle(p),Self::DeleteCharacterStyle(p)=>V::DeleteCharacterStyle(p),Self::UpdateCharacterStyle(p)=>V::UpdateCharacterStyle(p),Self::UpdateParentPage(p)=>V::UpdateParentPage(p),Self::UpdateSpread(p)=>V::UpdateSpread(p),Self::SetPageParent(p)=>V::SetPageParent(p),Self::SetPageGuides(p)=>V::SetPageGuides(p),Self::SetStoryRuns(p)=>V::SetStoryRuns(p),Self::UpdateLink(p)=>V::UpdateLink(p),Self::SetPageOverrides(p)=>V::SetPageOverrides(p),Self::CreateLayer(p)=>V::CreateLayer(p),Self::SetFrameLayer(p)=>V::SetFrameLayer(p),Self::ReorderFrame(p)=>V::ReorderFrame(p),Self::DragFrames(p)=>V::DragFrames(p),Self::RotateFrames(p)=>V::RotateFrames(p),Self::ScaleFrames(p)=>V::ScaleFrames(p),Self::CreateParagraphStyle(p)=>V::CreateParagraphStyle(p),Self::DeleteParagraphStyle(p)=>V::DeleteParagraphStyle(p),Self::CreateParentPage(p)=>V::CreateParentPage(p),Self::DeleteParentPage(p)=>V::DeleteParentPage(p),Self::CreateSpread(p)=>V::CreateSpread(p),Self::DeleteSpread(p)=>V::DeleteSpread(p),Self::SetPageSpread(p)=>V::SetPageSpread(p),Self::ReorderLayer(p)=>V::ReorderLayer(p),Self::SetParentPageContent(p)=>V::SetParentPageContent(p),Self::ReplaceDocument(p)=>V::ReplaceDocument(p),Self::SetPageBackgroundDrawing(p)=>V::SetPageBackgroundDrawing(p)}}
 fn parent_creation_mut(&mut self)->Option<&mut create_parent_page::CreateParentPage>{if let Self::CreateParentPage(p)=self{Some(p)}else{None}}
}
