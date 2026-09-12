//! ✏️ ✏️ Layout play app commands command — `patch-frame`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::layout::modes::edit::windows::blueprint::config::current;
use crate::mutations::change_frame_columns::ChangeFrameColumns;
use crate::mutations::change_frame_fill::ChangeFrameFill;
use crate::mutations::change_frame_stroke::ChangeFrameStroke;
use crate::mutations::change_frame_wrap_mode::ChangeFrameWrapMode;
use crate::mutations::change_link_path::ChangeLinkPath;
use crate::mutations::edit_story::EditStory;
use crate::mutations::move_frame::MoveFrame;
use crate::mutations::resize_frame::ResizeFrame;
use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::text_to_rgba;
use crate::{Frame, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-frame")]
pub struct PatchFrame {
    pub frame_id: String,
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchFrame, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let page_id = payload.page_id.clone().unwrap_or_else(|| current(cfg).active_page_id);
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
