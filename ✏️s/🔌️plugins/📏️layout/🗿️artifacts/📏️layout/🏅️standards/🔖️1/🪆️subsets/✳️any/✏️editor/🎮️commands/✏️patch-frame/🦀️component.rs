//! ✏️ ✏️ Layout play app commands command — `patch-frame`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::schema::text_to_rgba;
use crate::artifacts::layout::mutations::change_frame_columns::mutation::ChangeFrameColumns;
use crate::artifacts::layout::mutations::change_frame_fill::mutation::ChangeFrameFill;
use crate::artifacts::layout::mutations::change_frame_stroke::mutation::ChangeFrameStroke;
use crate::artifacts::layout::mutations::change_frame_wrap_mode::mutation::ChangeFrameWrapMode;
use crate::artifacts::layout::mutations::change_link_path::mutation::ChangeLinkPath;
use crate::artifacts::layout::mutations::change_page_height::mutation::ChangePageHeight;
use crate::artifacts::layout::mutations::change_page_width::mutation::ChangePageWidth;
use crate::artifacts::layout::mutations::create_frame::mutation::CreateFrame;
use crate::artifacts::layout::mutations::create_page::mutation::CreatePage;
use crate::artifacts::layout::mutations::edit_story::mutation::EditStory;
use crate::artifacts::layout::mutations::move_frame::mutation::MoveFrame;
use crate::artifacts::layout::mutations::rename_page::mutation::RenamePage;
use crate::artifacts::layout::mutations::resize_frame::mutation::ResizeFrame;
use crate::artifacts::layout::mutations::update_page_columns::mutation::UpdatePageColumns;
use crate::artifacts::layout::mutations::update_page_margins::mutation::UpdatePageMargins;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot, Page, PageColumns, PageMargins};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-frame")]
pub struct PatchFrame {
    pub frame_id: String,
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchFrame, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let document = doc.snapshot;
    let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
    if payload.frame_id.is_empty() {
        return Ok(Emit::default());
    }
    let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
        return Ok(Emit::default());
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Ok(Emit::default());
    };
    let frame_id = payload.frame_id.clone();
    match payload.field.as_str() {
        "x" | "y" => match payload.value.parse::<f64>() {
            Ok(number) => {
                let bounds = frame.bounds();
                let (new_x, new_y) = if payload.field == "x" { (number, bounds.y) } else { (bounds.x, number) };
                Ok(Emit::mutations(vec![LayoutMutation::MoveFrame(MoveFrame { page_id, frame_id, new_x, new_y })]))
            }
            Err(_) => Ok(Emit::default()),
        },
        "width" | "w" | "height" | "h" => match payload.value.parse::<f64>() {
            Ok(number) => {
                let bounds = frame.bounds();
                let (new_width, new_height) = if payload.field == "width" || payload.field == "w" { (number, bounds.height) } else { (bounds.width, number) };
                Ok(Emit::mutations(vec![LayoutMutation::ResizeFrame(ResizeFrame { page_id, frame_id, new_width, new_height })]))
            }
            Err(_) => Ok(Emit::default()),
        },
        "fill" => Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameFill(ChangeFrameFill { page_id, frame_id, new_fill: text_to_rgba(&payload.value) })])),
        "stroke" => Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameStroke(ChangeFrameStroke { page_id, frame_id, new_stroke: text_to_rgba(&payload.value) })])),
        "wrapMode" => Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameWrapMode(ChangeFrameWrapMode { page_id, frame_id, new_wrap_mode: payload.value.clone() })])),
        "columns" => match payload.value.parse::<f64>() {
            Ok(count) => Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameColumns(ChangeFrameColumns { page_id, frame_id, new_columns: count.max(0.0) as u32 })])),
            Err(_) => Ok(Emit::default()),
        },
        "storyContent" => {
            let story_id = match frame {
                Frame::Text { story_id, .. } => Some(story_id.clone()),
                _ => None,
            };
            match story_id {
                Some(id) if document.stories.iter().any(|story| story.id == id) => Ok(Emit::mutations(vec![LayoutMutation::EditStory(EditStory { id, new_content: payload.value.clone() })])),
                _ => Ok(Emit::default()),
            }
        }
        "linkPath" => {
            let link_id = match frame {
                Frame::Image { link_id, .. } => Some(link_id.clone()),
                _ => None,
            };
            match link_id {
                Some(id) if document.links.iter().any(|link| link.id == id) => Ok(Emit::mutations(vec![LayoutMutation::ChangeLinkPath(ChangeLinkPath { id, new_path: payload.value.clone() })])),
                _ => Ok(Emit::default()),
            }
        }
        _ => Ok(Emit::default()),
    }
}
