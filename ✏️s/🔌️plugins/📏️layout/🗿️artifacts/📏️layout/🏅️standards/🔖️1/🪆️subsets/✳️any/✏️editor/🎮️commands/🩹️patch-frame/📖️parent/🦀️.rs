//! 📖️ Explicit shared parent-frame edits use the native parent-content mutation.

use super::*;
use semio_s_artifact_layout_layout_mutations_content::set_parent_page_content::SetParentPageContent;

fn numeric(value: &str) -> Option<f64> { value.parse::<f64>().ok().filter(|number| number.is_finite()) }

pub(super) fn handle(payload: &PatchFrame, document: &LayoutSnapshot, page: &Page) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let Some(field) = payload.field.strip_prefix("parent.") else { return Ok(Emit::default()) };
    let Some(parent) = page.parent_page_id.as_ref().and_then(|id| document.parent_pages.iter().find(|parent| parent.id == *id)) else { return Ok(Emit::default()) };
    let Some(source)=parent.frames.iter().find(|frame|frame.id()==payload.frame_id)else{return Ok(Emit::default())};
    if parent.layers.iter().any(|layer|layer.id==source.layer_id()&&layer.locked)||(source.locked()&&!matches!(field,"locked"|"visible"|"open")){return Ok(Emit::default())}
    let mut frames = parent.frames.clone();
    let Some(index) = frames.iter().position(|frame| frame.id() == payload.frame_id) else { return Ok(Emit::default()) };
    if matches!(field, "forward" | "backward") {
        let target = if field == "forward" { (index + 1).min(frames.len() - 1) } else { index.saturating_sub(1) };
        frames.swap(index, target);
    } else if !update(&mut frames[index], field, &payload.value, document, parent) {
        return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.parent-frame.invalid", "The parent-frame field does not accept this value."));
    }
    Ok(Emit::mutations(vec![LayoutMutation::SetParentPageContent(SetParentPageContent { id: parent.id.clone(), layers: parent.layers.clone(), frames })]))
}

fn update(frame: &mut Frame, field: &str, value: &str, document: &LayoutSnapshot, parent: &semio_s_artifact_layout_layout_model::ParentPage) -> bool {
    if matches!(field, "x" | "y" | "width" | "height" | "rotation") {
        let Some(number) = numeric(value) else { return false };
        if matches!(field, "width" | "height") && number <= 0.0 { return false }
        let bounds = match frame { Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds };
        match field { "x" => bounds.x = number, "y" => bounds.y = number, "width" => bounds.width = number, "height" => bounds.height = number, _ => bounds.rotation = number }
        return true;
    }
    if matches!(field, "locked" | "visible") {
        let on = matches!(value, "true" | "1");
        let (locked, visible) = match frame { Frame::Rect { locked, visible, .. } | Frame::Text { locked, visible, .. } | Frame::Image { locked, visible, .. } => (locked, visible) };
        if field == "locked" { *locked = Some(on) } else { *visible = Some(on) }
        return true;
    }
    if field == "layerId" {
        if parent.layers.iter().all(|layer| layer.id != value) { return false }
        match frame { Frame::Rect { layer_id, .. } | Frame::Text { layer_id, .. } | Frame::Image { layer_id, .. } => *layer_id = value.into() }
        return true;
    }
    match frame {
        Frame::Rect { fill, stroke, .. } => {
            let color = match field { "fill" | "fillEnabled" | "fillAlpha" => fill, "stroke" | "strokeEnabled" | "strokeAlpha" => stroke, _ => return false };
            if field.ends_with("Enabled") { *color = matches!(value, "true" | "1").then(|| color.unwrap_or([0.0, 0.0, 0.0, 1.0])); }
            else if field.ends_with("Alpha") {
                let Some(alpha) = numeric(value) else { return false };
                let mut next = color.unwrap_or([0.0, 0.0, 0.0, 1.0]); next[3] = alpha.clamp(0.0, 1.0) as f32; *color = Some(next);
            } else {
                let Some(mut next) = text_to_rgba(value) else { return false };
                if let Some(previous) = color { next[3] = previous[3]; }
                *color = Some(next);
            }
            true
        }
        Frame::Text { id, columns, inset, story_id, thread_next, wrap_mode, .. } => match field {
            "columns" => { let Some(count) = numeric(value).filter(|count| *count >= 1.0 && count.fract() == 0.0 && *count <= u32::MAX as f64) else { return false }; *columns = count as u32; true }
            "insetX" | "insetY" | "insetWidth" | "insetHeight" => {
                let Some(number) = numeric(value) else { return false };
                match field { "insetX" => inset.x = number, "insetY" => inset.y = number, "insetWidth" => inset.width = number, _ => inset.height = number }
                true
            }
            "storyId" if document.stories.iter().any(|story| story.id == value) => { *story_id = value.into(); true }
            "threadNext" => {
                if !value.is_empty() && (value == id.as_str() || !parent.frames.iter().chain(document.pages.iter().flat_map(|page| &page.frames)).any(|frame| matches!(frame, Frame::Text { id, .. } if id == value))) { return false }
                *thread_next = (!value.is_empty()).then(|| value.into()); true
            }
            "wrapMode" if matches!(value, "none" | "box" | "contour") => { *wrap_mode = value.into(); true }
            _ => false,
        },
        Frame::Image { .. } => false,
    }
}
