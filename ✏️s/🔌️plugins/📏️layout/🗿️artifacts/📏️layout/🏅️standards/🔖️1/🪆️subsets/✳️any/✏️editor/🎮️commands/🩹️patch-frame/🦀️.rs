//! ✏️ ✏️ Layout play app commands command — `patch-frame`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::modes::edit::windows::blueprint::config::current;
use semio_s_artifact_layout_layout_mutations_frames::change_frame_columns::ChangeFrameColumns;
use semio_s_artifact_layout_layout_mutations_frames::change_frame_fill::ChangeFrameFill;
use semio_s_artifact_layout_layout_mutations_frames::change_frame_stroke::ChangeFrameStroke;
use semio_s_artifact_layout_layout_mutations_frames::change_frame_wrap_mode::ChangeFrameWrapMode;
use semio_s_artifact_layout_layout_mutations_content::change_link_path::ChangeLinkPath;
use semio_s_artifact_layout_layout_mutations_content::edit_story::EditStory;
use semio_s_artifact_layout_layout_mutations_frames::move_frame::MoveFrame;
use semio_s_artifact_layout_layout_mutations_frames::resize_frame::ResizeFrame;
use semio_s_artifact_layout_layout_mutations_frames::rotate_frame::RotateFrame;
use semio_s_artifact_layout_layout_mutations_frames::update_text_frame::UpdateTextFrame;
use semio_s_artifact_layout_layout_mutations_frames::set_frame_flags::SetFrameFlags;
use semio_s_artifact_layout_layout_mutations_frames::set_frame_layer::SetFrameLayer;
use semio_s_artifact_layout_layout_mutations_content::set_story_runs::SetStoryRuns;
use semio_s_artifact_layout_layout_mutations_content::set_page_overrides::SetPageOverrides;
use semio_s_artifact_layout_layout_mutations_content::update_link::UpdateLink;
use semio_s_artifact_layout_layout_mutations_frames::reorder_frame::ReorderFrame;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_model::text_to_rgba;
use semio_s_artifact_layout_layout_model::{Frame, Page, PageOverride, TextStyleRun};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "📖️parent/🦀️.rs"]
mod parent;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-frame")]
pub struct PatchFrame {
    pub frame_id: String,
    pub page_id: Option<String>,
    pub field: String,
    pub value: String,
}


/// ✂️ The single contiguous splice (character `offset`, characters `delete`d, text `insert`ed) the field edit changed; `None` for an unchanged value.
pub(crate) fn story_splice(content: &str, value: &str) -> Option<(usize, usize, String)> {
    if content == value {
        return None;
    }
    let (old, new): (Vec<char>, Vec<char>) = (content.chars().collect(), value.chars().collect());
    let prefix = old.iter().zip(&new).take_while(|(left, right)| left == right).count();
    let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
    Some((prefix, old.len() - prefix - suffix, new[prefix..new.len() - suffix].iter().collect()))
}

fn inherited_override(document: &LayoutSnapshot, page: &Page, page_id: &str, frame_id: &str, field: &str, value: &str) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(parent_id) = &page.parent_page_id else { return Ok(Emit::default()) };
    let Some(parent) = document.parent_pages.iter().find(|parent| parent.id == *parent_id) else { return Ok(Emit::default()) };
    let Some(frame) = parent.frames.iter().find(|frame| frame.id() == frame_id) else { return Ok(Emit::default()) };
    let mut resolved = frame.clone();
    if let Some(page_override) = page.overrides.iter().find(|item| item.object_id == frame_id) { semio_s_artifact_layout_layout_model::apply_page_override(&mut resolved, page_override); }
    let frame = &resolved;
    if semio_s_artifact_layout_layout_model::layer_locked(document, page, frame.layer_id()) || (frame.locked() && !matches!(field, "locked" | "visible" | "open")) {
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


fn run_field(field: &str) -> Option<(usize, &str)> {
    let (index, key) = field.strip_prefix("run.")?.split_once('.')?;
    Some((index.parse().ok()?, key))
}

fn char_to_byte(content: &str, chars: usize) -> usize {
    content.char_indices().nth(chars).map(|(index, _)| index).unwrap_or(content.len())
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
    if payload.field.starts_with("parent.") { return parent::handle(payload, document, page) }
    let local = page.frames.iter().find(|frame| frame.id() == payload.frame_id);
    if local.is_none() && matches!(payload.field.as_str(), "x" | "y" | "width" | "height" | "rotation" | "locked" | "visible") {
        return inherited_override(document, page, &page_id, &payload.frame_id, &payload.field, &payload.value);
    }
    let inherited = page.parent_page_id.as_ref().and_then(|id| document.parent_pages.iter().find(|parent| parent.id == *id)).and_then(|parent| parent.frames.iter().find(|frame| frame.id() == payload.frame_id));
    let Some(frame) = local.or(inherited) else { return Ok(Emit::default()) };
    if local.is_none() && !matches!(payload.field.as_str(), "storyContent" | "addStyleRun" | "clearStyleRuns" | "linkPath" | "linkWidth" | "linkHeight" | "dpi" | "colorProfile" | "open") && run_field(&payload.field).is_none() {
        return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.parent-frame.scope", "Use the shared parent-frame controls for this property."));
    }
    if semio_s_artifact_layout_layout_model::layer_locked(document, page, frame.layer_id()) {
        return Ok(Emit::default());
    }
    let locked=if local.is_none(){page.overrides.iter().find(|entry|entry.object_id==frame.id()).and_then(|entry|entry.locked).unwrap_or_else(||frame.locked())}else{frame.locked()};
    if locked && !matches!(payload.field.as_str(), "locked" | "visible" | "open") {
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
                Some(id) => match document.stories.iter().find(|story| story.id == id) {
                    Some(story) => Ok(Emit::mutations(story_splice(&story.content, &payload.value).map(|(offset, delete, insert)| LayoutMutation::EditStory(EditStory { id, offset, delete, insert })).into_iter().collect())),
                    None => Ok(Emit::default()),
                },
                _ => Ok(Emit::default()),
            }
        }
        "clearStyleRuns" => {
            let Frame::Text { story_id, .. } = frame else { return Ok(Emit::default()) };
            Ok(Emit::mutations(vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story_id.clone(), runs: Vec::new() })]))
        }
        "addStyleRun" => {
            let Frame::Text { story_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(story) = document.stories.iter().find(|story| story.id == *story_id) else { return Ok(Emit::default()) };
            let mut runs = story.style_runs.clone();
            runs.push(TextStyleRun { start: 0, end: story.content.len() as u64, paragraph_style_id: None, character_style_id: None });
            Ok(Emit::mutations(vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story.id.clone(), runs })]))
        }
        field if run_field(field).is_some() => {
            let Frame::Text { story_id, .. } = frame else { return Ok(Emit::default()) };
            let Some(story) = document.stories.iter().find(|story| story.id == *story_id) else { return Ok(Emit::default()) };
            let Some((index, key)) = run_field(field) else { return Ok(Emit::default()) };
            let mut runs = story.style_runs.clone();
            let Some(run) = runs.get_mut(index) else { return Ok(Emit::default()) };
            match key {
                "start" | "end" => {
                    let Ok(chars) = payload.value.trim().parse::<usize>() else { return Ok(Emit::default()) };
                    let byte = char_to_byte(&story.content, chars) as u64;
                    if key == "start" { run.start = byte.min(run.end) } else { run.end = byte.max(run.start) }
                }
                "paragraphStyle" => run.paragraph_style_id = (!payload.value.is_empty()).then(|| payload.value.clone()),
                "characterStyle" => run.character_style_id = (!payload.value.is_empty()).then(|| payload.value.clone()),
                "delete" => { runs.remove(index); }
                _ => return Ok(Emit::default()),
            }
            Ok(Emit::mutations(vec![LayoutMutation::SetStoryRuns(SetStoryRuns { id: story.id.clone(), runs })]))
        }
        "fillEnabled" | "strokeEnabled" | "fillAlpha" | "strokeAlpha" => {
            let Frame::Rect { fill, stroke, .. } = frame else { return Ok(Emit::default()) };
            let is_fill = payload.field.starts_with("fill");
            let previous = if is_fill { fill } else { stroke };
            let color = if payload.field.ends_with("Enabled") {
                matches!(payload.value.as_str(), "true" | "1").then(|| previous.unwrap_or([0.0, 0.0, 0.0, 1.0]))
            } else {
                let Ok(alpha) = payload.value.parse::<f32>() else { return Ok(Emit::default()) };
                if !alpha.is_finite() { return Ok(Emit::default()) }
                let mut color = previous.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                color[3] = alpha.clamp(0.0, 1.0);
                Some(color)
            };
            let mutation = if is_fill { LayoutMutation::ChangeFrameFill(ChangeFrameFill { page_id, frame_id, new_fill: color }) } else { LayoutMutation::ChangeFrameStroke(ChangeFrameStroke { page_id, frame_id, new_stroke: color }) };
            Ok(Emit::mutations(vec![mutation]))
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
            let Some((standard, subset, schema)) = crate::native_open(&link.artifact_kind) else { return Ok(Emit::default()) };
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
        let mut document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
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
        for (_, _, kind) in crate::NATIVE_PLACEMENTS {
            assert!(crate::native_open(kind).is_some(), "{kind}");
        }
    }

    #[test]
    fn parent_patch_cannot_issue_child_text_commands() {
        let document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
        let config = NoConfig::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let emit = handle(&PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "drawingText".into(), value: "Title".into() }, &doc, &cfg).expect("parent command");
        assert!(emit.artifact_mutations.is_empty());
    }

    #[test]
    fn a_locked_frame_keeps_its_geometry_and_can_still_unlock() {
        let mut document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
        match document.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap() {
            semio_s_artifact_layout_layout_model::Frame::Rect { locked, .. } => *locked = Some(true),
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

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod inherited_tests;
