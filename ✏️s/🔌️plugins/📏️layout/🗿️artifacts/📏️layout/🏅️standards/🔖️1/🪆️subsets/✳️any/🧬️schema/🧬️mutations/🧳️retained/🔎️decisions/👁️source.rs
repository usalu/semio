//! 👁️ Borrowed schema vocabulary for bounded original-source decisions.
pub use semio_s_artifact_layout_layout_mutations_content::{change_data_fields, change_link_path, change_page_height, change_page_width, change_print_target, create_link, create_page, create_story, delete_link, delete_page, delete_story, edit_story, rename_layout, rename_page, reorder_pages, create_character_style, delete_character_style, set_page_guides, set_page_parent, set_story_runs, update_link, set_page_overrides, update_character_style, update_page_columns, update_page_margins, update_paragraph_style, update_parent_page, update_spread};
pub use semio_s_artifact_layout_layout_mutations_frames::{change_frame_columns, change_frame_fill, change_frame_stroke, change_frame_wrap_mode, create_frame, delete_frame, drag_frames, move_frame, rotate_frames, scale_frames, resize_frame, rotate_frame, set_frame_flags, update_grid, create_layer, set_frame_layer, reorder_frame, update_layer, update_text_frame};
pub use semio_s_artifact_layout_layout_mutations_content::{replace_document, set_parent_page_content, create_paragraph_style, delete_paragraph_style, create_parent_page, delete_parent_page, create_spread, delete_spread, set_page_spread, reorder_layer};
pub use semio_s_artifact_layout_layout_mutations_content::set_page_background_drawing;

#[derive(Clone, Copy)]
pub enum LayoutMutationView<'a> {
    RenameLayout(&'a rename_layout::RenameLayout),
    ChangePrintTarget(&'a change_print_target::ChangePrintTarget),
    ChangeDataFields(&'a change_data_fields::ChangeDataFields),
    CreatePage(&'a create_page::CreatePage),
    DeletePage(&'a delete_page::DeletePage),
    RenamePage(&'a rename_page::RenamePage),
    ChangePageWidth(&'a change_page_width::ChangePageWidth),
    ChangePageHeight(&'a change_page_height::ChangePageHeight),
    UpdatePageMargins(&'a update_page_margins::UpdatePageMargins),
    UpdatePageColumns(&'a update_page_columns::UpdatePageColumns),
    ReorderPages(&'a reorder_pages::ReorderPages),
    CreateStory(&'a create_story::CreateStory),
    DeleteStory(&'a delete_story::DeleteStory),
    EditStory(&'a edit_story::EditStory),
    CreateLink(&'a create_link::CreateLink),
    DeleteLink(&'a delete_link::DeleteLink),
    ChangeLinkPath(&'a change_link_path::ChangeLinkPath),
    CreateFrame(&'a create_frame::CreateFrame),
    DeleteFrame(&'a delete_frame::DeleteFrame),
    MoveFrame(&'a move_frame::MoveFrame),
    ResizeFrame(&'a resize_frame::ResizeFrame),
    RotateFrame(&'a rotate_frame::RotateFrame),
    ChangeFrameFill(&'a change_frame_fill::ChangeFrameFill),
    ChangeFrameStroke(&'a change_frame_stroke::ChangeFrameStroke),
    ChangeFrameWrapMode(&'a change_frame_wrap_mode::ChangeFrameWrapMode),
    ChangeFrameColumns(&'a change_frame_columns::ChangeFrameColumns),
    UpdateGrid(&'a update_grid::UpdateGrid),
    SetFrameFlags(&'a set_frame_flags::SetFrameFlags),
    UpdateParagraphStyle(&'a update_paragraph_style::UpdateParagraphStyle),
    UpdateTextFrame(&'a update_text_frame::UpdateTextFrame),
    UpdateLayer(&'a update_layer::UpdateLayer),
    CreateCharacterStyle(&'a create_character_style::CreateCharacterStyle),
    DeleteCharacterStyle(&'a delete_character_style::DeleteCharacterStyle),
    UpdateCharacterStyle(&'a update_character_style::UpdateCharacterStyle),
    UpdateParentPage(&'a update_parent_page::UpdateParentPage),
    UpdateSpread(&'a update_spread::UpdateSpread),
    SetPageParent(&'a set_page_parent::SetPageParent),
    SetPageGuides(&'a set_page_guides::SetPageGuides),
    SetStoryRuns(&'a set_story_runs::SetStoryRuns),
    UpdateLink(&'a update_link::UpdateLink),
    SetPageOverrides(&'a set_page_overrides::SetPageOverrides),
    CreateLayer(&'a create_layer::CreateLayer),
    SetFrameLayer(&'a set_frame_layer::SetFrameLayer),
    ReorderFrame(&'a reorder_frame::ReorderFrame),
    DragFrames(&'a drag_frames::DragFrames),
    RotateFrames(&'a rotate_frames::RotateFrames),
    ScaleFrames(&'a scale_frames::ScaleFrames),
    CreateParagraphStyle(&'a create_paragraph_style::CreateParagraphStyle),
    DeleteParagraphStyle(&'a delete_paragraph_style::DeleteParagraphStyle),
    CreateParentPage(&'a create_parent_page::CreateParentPage),
    DeleteParentPage(&'a delete_parent_page::DeleteParentPage),
    CreateSpread(&'a create_spread::CreateSpread),
    DeleteSpread(&'a delete_spread::DeleteSpread),
    SetPageSpread(&'a set_page_spread::SetPageSpread),
    ReorderLayer(&'a reorder_layer::ReorderLayer),
    SetParentPageContent(&'a set_parent_page_content::SetParentPageContent),
    ReplaceDocument(&'a replace_document::ReplaceDocument),
    SetPageBackgroundDrawing(&'a set_page_background_drawing::SetPageBackgroundDrawing),
}
/// 🔎️ A mutation lends its authored schema fields without cloning or changing owner identity.
pub trait LayoutMutationSource {
    fn view(&self) -> LayoutMutationView<'_>;
    fn parent_creation_mut(&mut self) -> Option<&mut create_parent_page::CreateParentPage>;
}
