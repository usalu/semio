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
use crate::mutations::rotate_frame::RotateFrame;
use crate::mutations::update_text_frame::UpdateTextFrame;
use crate::mutations::set_frame_flags::SetFrameFlags;
use crate::mutations::set_story_runs::SetStoryRuns;
use crate::mutations::update_link::UpdateLink;
use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::text_to_rgba;
use crate::{Frame, LayoutSnapshot, TextStyleRun};
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
        "fill" | "stroke" => {
            let mut color = text_to_rgba(&payload.value);
            let hex = payload.value.trim();
            if let (Some(parsed), Frame::Rect { fill, stroke, .. }) = (color.as_mut(), frame) {
                if hex.starts_with('#') && hex.len() <= 7 {
                    let previous = if payload.field == "fill" { fill } else { stroke };
                    if let Some(previous) = previous {
                        parsed[3] = previous[3];
                    }
                }
            }
            if payload.field == "fill" {
                Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameFill(ChangeFrameFill { page_id, frame_id, new_fill: color })]))
            } else {
                Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameStroke(ChangeFrameStroke { page_id, frame_id, new_stroke: color })]))
            }
        }
        "wrapMode" => Ok(Emit::mutations(vec![LayoutMutation::ChangeFrameWrapMode(ChangeFrameWrapMode { page_id, frame_id, new_wrap_mode: payload.value.clone() })])),
        "rotation" => match payload.value.parse::<f64>() {
            Ok(number) => Ok(Emit::mutations(vec![LayoutMutation::RotateFrame(RotateFrame { page_id, frame_id, new_rotation: number })])),
            Err(_) => Ok(Emit::default()),
        },
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
        "characterStyle" => {
            let Frame::Text { story_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(story) = document.stories.iter().find(|story| story.id == *story_id) else { return Ok(Emit::default()) };
            let style_id = payload.value.trim();
            let runs = if style_id.is_empty() {
                Vec::new()
            } else if document.character_styles.iter().any(|style| style.id == style_id) && !story.content.is_empty() {
                vec![TextStyleRun { start: 0, end: story.content.len(), paragraph_style_id: None, character_style_id: Some(style_id.to_string()) }]
            } else {
                return Ok(Emit::default());
            };
            Ok(Emit::mutations(vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story.id.clone(), runs })]))
        }
        "linkWidth" | "linkHeight" | "dpi" | "colorProfile" => {
            let Frame::Image { link_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(link) = document.links.iter().find(|link| link.id == *link_id) else { return Ok(Emit::default()) };
            let mut next = UpdateLink { id: link.id.clone(), width: link.width, height: link.height, dpi: link.dpi, color_profile: link.color_profile.clone() };
            match payload.field.as_str() {
                "colorProfile" => {
                    let trimmed = payload.value.trim();
                    next.color_profile = (!trimmed.is_empty()).then(|| trimmed.to_string());
                }
                _ => match payload.value.parse::<f64>() {
                    Ok(number) if number > 0.0 => {
                        let whole = number as u32;
                        match payload.field.as_str() {
                            "linkWidth" => next.width = whole,
                            "linkHeight" => next.height = whole,
                            _ => next.dpi = whole,
                        }
                    }
                    _ => return Ok(Emit::default()),
                },
            }
            Ok(Emit::mutations(vec![LayoutMutation::UpdateLink(next)]))
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
        "insetX" | "insetY" | "insetWidth" | "insetHeight" | "storyId" | "threadNext" => {
            let Frame::Text { story_id, thread_next, inset, .. } = frame else {
                return Ok(Emit::default());
            };
            let mut next = UpdateTextFrame { page_id, frame_id, story_id: story_id.clone(), thread_next: thread_next.clone(), inset_x: inset.x, inset_y: inset.y, inset_width: inset.width, inset_height: inset.height };
            match payload.field.as_str() {
                "insetX" | "insetY" | "insetWidth" | "insetHeight" => match payload.value.parse::<f64>() {
                    Ok(number) => match payload.field.as_str() {
                        "insetX" => next.inset_x = number,
                        "insetY" => next.inset_y = number,
                        "insetWidth" => next.inset_width = number,
                        _ => next.inset_height = number,
                    },
                    Err(_) => return Ok(Emit::default()),
                },
                "storyId" => next.story_id = payload.value.clone(),
                "threadNext" => {
                    let trimmed = payload.value.trim();
                    next.thread_next = (!trimmed.is_empty()).then(|| trimmed.to_string());
                }
                _ => return Ok(Emit::default()),
            }
            Ok(Emit::mutations(vec![LayoutMutation::UpdateTextFrame(next)]))
        }
        "locked" | "visible" => {
            let on = matches!(payload.value.trim(), "true" | "1");
            let (locked, visible) = if payload.field == "locked" { (Some(on), None) } else { (None, Some(on)) };
            Ok(Emit::mutations(vec![LayoutMutation::SetFrameFlags(SetFrameFlags { page_id, frame_id, locked, visible })]))
        }
        _ => Ok(Emit::default()),
    }
}
