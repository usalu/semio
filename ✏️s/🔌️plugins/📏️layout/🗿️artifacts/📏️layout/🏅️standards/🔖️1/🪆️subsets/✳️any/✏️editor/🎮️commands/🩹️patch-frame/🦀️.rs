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
use crate::mutations::set_frame_layer::SetFrameLayer;
use crate::mutations::set_drawing_text::{drawing_text_index, is_drawing_kind, SetDrawingText};
use crate::mutations::set_story_runs::SetStoryRuns;
use crate::mutations::set_page_overrides::SetPageOverrides;
use crate::mutations::update_link::UpdateLink;
use crate::mutations::reorder_frame::ReorderFrame;
use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::text_to_rgba;
use crate::{Frame, LayoutSnapshot, Page, PageOverride, TextStyleRun};
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


fn inherited_override(document: &LayoutSnapshot, page: &Page, page_id: &str, frame_id: &str, field: &str, value: &str) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(parent_id) = &page.parent_page_id else { return Ok(Emit::default()) };
    let Some(parent) = document.parent_pages.iter().find(|parent| parent.id == *parent_id) else { return Ok(Emit::default()) };
    let Some(frame) = parent.frames.iter().find(|frame| frame.id() == frame_id) else { return Ok(Emit::default()) };
    if crate::layer_locked(document, page, frame.layer_id()) || (frame.locked() && !matches!(field, "locked" | "open")) {
        return Ok(Emit::default());
    }
    let mut overrides = page.overrides.clone();
    let mut entry = overrides.iter().find(|item| item.object_id == frame_id).cloned().unwrap_or(PageOverride { object_id: frame_id.to_string(), bounds: None, visible: None, locked: None });
    match field {
        "x" | "y" | "width" | "w" | "height" | "h" | "rotation" => {
            let Ok(number) = value.parse::<f64>() else { return Ok(Emit::default()) };
            let mut bounds = entry.bounds.clone().unwrap_or_else(|| frame.bounds().clone());
            match field {
                "x" => bounds.x = number,
                "y" => bounds.y = number,
                "rotation" => bounds.rotation = number,
                "width" | "w" => bounds.width = number,
                _ => bounds.height = number,
            }
            entry.bounds = Some(bounds);
        }
        "locked" | "visible" => {
            let on = matches!(value.trim(), "true" | "1");
            if field == "locked" { entry.locked = Some(on) } else { entry.visible = Some(on) }
        }
        _ => return Ok(Emit::default()),
    }
    if let Some(slot) = overrides.iter_mut().find(|item| item.object_id == frame_id) {
        *slot = entry;
    } else {
        overrides.push(entry);
    }
    Ok(Emit::mutations(vec![LayoutMutation::SetPageOverrides(SetPageOverrides { id: page_id.to_string(), overrides })]))
}


/// ✂️ Inspector key for one style run: `styleStart` / `styleEnd`, then `.1` through `.3`.
pub fn style_range_key(index: usize, start: bool) -> Option<&'static str> {
    const START: [&str; 4] = ["styleStart", "styleStart.1", "styleStart.2", "styleStart.3"];
    const END: [&str; 4] = ["styleEnd", "styleEnd.1", "styleEnd.2", "styleEnd.3"];
    if start { START.get(index).copied() } else { END.get(index).copied() }
}

fn style_range_index(field: &str) -> Option<(usize, bool)> {
    (0..4).find_map(|index| {
        if style_range_key(index, true) == Some(field) {
            Some((index, true))
        } else if style_range_key(index, false) == Some(field) {
            Some((index, false))
        } else {
            None
        }
    })
}

fn char_to_byte(content: &str, chars: usize) -> usize {
    content.char_indices().nth(chars).map(|(index, _)| index).unwrap_or(content.len())
}

fn byte_to_char(content: &str, byte: usize) -> usize {
    content.get(..byte.min(content.len())).unwrap_or(content).chars().count()
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
        return inherited_override(document, page, &page_id, &payload.frame_id, &payload.field, &payload.value);
    };
    if crate::layer_locked(document, page, frame.layer_id()) {
        return Ok(Emit::default());
    }
    if frame.locked() && !matches!(payload.field.as_str(), "locked" | "open") {
        return Ok(Emit::default());
    }
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
                if story.style_runs.is_empty() {
                    vec![TextStyleRun { start: 0, end: story.content.len(), paragraph_style_id: None, character_style_id: Some(style_id.to_string()) }]
                } else {
                    let mut runs = story.style_runs.clone();
                    runs[0].character_style_id = Some(style_id.to_string());
                    runs
                }
            } else {
                return Ok(Emit::default());
            };
            Ok(Emit::mutations(vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story.id.clone(), runs })]))
        }
        field if style_range_index(field).is_some() => {
            let Frame::Text { story_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(story) = document.stories.iter().find(|story| story.id == *story_id) else { return Ok(Emit::default()) };
            let Some((index, is_start)) = style_range_index(field) else { return Ok(Emit::default()) };
            let Ok(chars) = payload.value.trim().parse::<usize>() else { return Ok(Emit::default()) };
            let len_chars = story.content.chars().count();
            let chars = chars.min(len_chars);
            let mut runs = story.style_runs.clone();
            if index > runs.len() || (runs.is_empty() && index != 0) {
                return Ok(Emit::default());
            }
            if index == runs.len() {
                runs.push(TextStyleRun { start: 0, end: story.content.len(), paragraph_style_id: None, character_style_id: None });
            }
            let run = &mut runs[index];
            let mut start_chars = byte_to_char(&story.content, run.start);
            let mut end_chars = byte_to_char(&story.content, run.end);
            if is_start {
                start_chars = chars.min(end_chars);
            } else {
                end_chars = chars.max(start_chars);
            }
            run.start = char_to_byte(&story.content, start_chars);
            run.end = char_to_byte(&story.content, end_chars);
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
        field if drawing_text_index(field).is_some() => {
            let Frame::Image { link_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(link) = document.links.iter().find(|link| link.id == *link_id) else { return Ok(Emit::default()) };
            if !is_drawing_kind(&link.artifact_kind) {
                return Ok(Emit::default());
            }
            let text = payload.value.trim();
            if text.chars().count() > 256 {
                return Ok(Emit::default());
            }
            let index = drawing_text_index(field).unwrap_or(0);
            Ok(Emit::mutations(vec![LayoutMutation::SetDrawingText(SetDrawingText { index, text: text.to_string() })]))
        }
        "forward" | "backward" => Ok(Emit::mutations(vec![LayoutMutation::ReorderFrame(ReorderFrame { page_id, frame_id, forward: payload.field == "forward" })])),
        "layerId" => {
            if page.layers.iter().all(|layer| layer.id != payload.value) {
                return Ok(Emit::default());
            }
            Ok(Emit::mutations(vec![LayoutMutation::SetFrameLayer(SetFrameLayer { page_id, frame_id, layer_id: payload.value.clone() })]))
        }
        "open" => {
            let Frame::Image { link_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(link) = document.links.iter().find(|link| link.id == *link_id) else { return Ok(Emit::default()) };
            let Some((standard, subset, schema)) = crate::editor::layout::panels::catalogue::native_open(&link.artifact_kind) else { return Ok(Emit::default()) };
            if link.artifact_ref.trim().is_empty() {
                return Ok(Emit::default());
            }
            let artifact_ref = format!("{}@{}/{}", link.artifact_kind, standard, subset);
            Ok(Emit {
                effects: vec![semio_framework::kernel::Effect::ReplayShellCommand {
                    action_id: "os.open-artifact".into(),
                    args: Some(semio_framework::dsl_value!({ "artifactRef": artifact_ref, "documentId": link.artifact_ref.clone(), "schema": schema, "frameId": frame_id })),
                }],
                ..Default::default()
            })
        }
        "locked" | "visible" => {
            let on = matches!(payload.value.trim(), "true" | "1");
            let (locked, visible) = if payload.field == "locked" { (Some(on), None) } else { (None, Some(on)) };
            Ok(Emit::mutations(vec![LayoutMutation::SetFrameFlags(SetFrameFlags { page_id, frame_id, locked, visible })]))
        }
        _ => Ok(Emit::default()),
    }
}

#[cfg(test)]
mod open_tests {
    use super::*;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};

    fn emit_open(kind: &str, artifact_ref: &str, frame_id: &str) -> Emit<LayoutMutation, NoConfigMutation> {
        let mut document = crate::standards::v1::subsets::any::schema::default_document();
        document.links[0].artifact_kind = kind.into();
        document.links[0].artifact_ref = artifact_ref.into();
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        handle(&PatchFrame { frame_id: frame_id.into(), page_id: Some("page-1".into()), field: "open".into(), value: "true".into() }, &doc, &cfg).expect("open")
    }

    #[test]
    fn patch_frame_opens_the_placed_source_editor() {
        let emit = emit_open("s.stdio.pdf", "doc-pdf", "frame-image-1");
        assert!(emit.artifact_mutations.is_empty());
        let Effect::ReplayShellCommand { action_id, args } = &emit.effects[0] else { panic!("{:?}", emit.effects) };
        assert_eq!(action_id, "os.open-artifact");
        let args = args.clone().expect("open args");
        let text = |key: &str| {
            let semio_framework::DslValue::Object(entries) = &args else { panic!("object") };
            match entries.iter().find(|(name, _)| name == key).map(|(_, value)| value) {
                Some(semio_framework::DslValue::String(value)) => value.clone(),
                other => panic!("{key}: {other:?}"),
            }
        };
        assert_eq!(text("artifactRef"), "s.stdio.pdf@1.7/*");
        assert_eq!(text("documentId"), "doc-pdf");
        assert_eq!(text("schema"), "stdio.pdf");
        assert_eq!(text("frameId"), "frame-image-1");
        let semio_framework::DslValue::Object(entries) = &args else { panic!("object") };
        assert!(entries.iter().all(|(name, _)| name != "role"));
    }

    #[test]
    fn patch_frame_open_refuses_a_frame_without_a_source() {
        assert!(emit_open("s.stdio.pdf", "", "frame-image-1").effects.is_empty());
        assert!(emit_open("", "doc-pdf", "frame-image-1").effects.is_empty());
        assert!(emit_open("s.stdio.pdf", "doc-pdf", "frame-1").effects.is_empty());
    }

    #[test]
    fn every_placeable_kind_can_open_its_editor() {
        for (_, _, kind) in crate::editor::layout::panels::catalogue::NATIVE_PLACEMENTS {
            assert!(crate::editor::layout::panels::catalogue::native_open(kind).is_some(), "{kind}");
        }
    }

    #[test]
    fn patch_frame_renames_drawing_text() {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
        use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawLayer, DrawNode, SemioDrawingSnapshot};
        let point = |x: f64, y: f64| SemioPoint2 { x, y };
        let content = SemioDrawingSnapshot {
            schema: "stdio.semio.drawing".into(),
            canvas: Default::default(),
            styles: Vec::new(),
            layers: vec![DrawLayer { id: "imported".into(), name: "Imported".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None }] } }],
        };
        let mut document = crate::standards::v1::subsets::any::schema::default_document();
        document.links[0].artifact_kind = "s.draw.drawing".into();
        document.background_drawing = Some(crate::background_drawing_child_handle("dwg", &content));
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = handle(&PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "drawingText".into(), value: "Title".into() }, &doc, &cfg).expect("rename");
        let LayoutMutation::SetDrawingText(set) = &emit.artifact_mutations[0] else { panic!("{:?}", emit.artifact_mutations) };
        assert_eq!(set.text, "Title");
    }

    #[test]
    fn a_locked_frame_keeps_its_geometry_and_can_still_unlock() {
        let mut document = crate::standards::v1::subsets::any::schema::default_document();
        match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
            crate::Frame::Rect { locked, .. } => *locked = Some(true),
            _ => panic!("rect"),
        }
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let moved = handle(&PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "x".into(), value: "80".into() }, &doc, &cfg).expect("move");
        assert!(moved.artifact_mutations.is_empty());
        let unlocked = handle(&PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "locked".into(), value: "false".into() }, &doc, &cfg).expect("unlock");
        assert_eq!(unlocked.artifact_mutations.len(), 1);
        document.pages[0].layers[0].locked = true;
        let doc = ArtifactView::new(&document, &history);
        let blocked = handle(&PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "locked".into(), value: "false".into() }, &doc, &cfg).expect("layer");
        assert!(blocked.artifact_mutations.is_empty());
    }
}
