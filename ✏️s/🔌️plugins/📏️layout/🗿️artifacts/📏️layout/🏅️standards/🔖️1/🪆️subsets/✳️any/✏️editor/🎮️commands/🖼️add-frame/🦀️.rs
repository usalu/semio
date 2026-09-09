//! ✏️ ✏️ Layout play app commands command — `add-frame`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::mutations::create_frame::CreateFrame;
use crate::mutations::LayoutMutation;
use crate::{Frame, LayoutSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct AddFrame {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

pub fn handle(payload: &AddFrame, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let page_id = config.active_page_id.clone();
    let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
        return Ok(Emit::default());
    };
    let index = page.frames.len();
    let frame_id = format!("frame-{}", index + 1);
    let layer_id = page.layer_ids.first().cloned().unwrap_or_else(|| "layer-1".into());
    let frame = match payload.kind.as_str() {
        "text" => Frame::Text {
            id: frame_id.clone(),
            layer_id: layer_id.clone(),
            bounds: crate::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(120.0), width: 200.0, height: 120.0, rotation: 0.0 },
            locked: None,
            visible: None,
            story_id: document.stories.first().map_or_else(|| "story-1".into(), |story| story.id.clone()),
            thread_next: None,
            columns: 1,
            inset: crate::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
            wrap_mode: "box".into(),
        },
        "image" => Frame::Image {
            id: frame_id.clone(),
            layer_id: layer_id.clone(),
            bounds: crate::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(280.0), width: 160.0, height: 120.0, rotation: 0.0 },
            locked: None,
            visible: None,
            link_id: document.links.first().map_or_else(|| "link-missing".into(), |link| link.id.clone()),
        },
        _ => Frame::Rect {
            id: frame_id.clone(),
            layer_id: layer_id.clone(),
            bounds: crate::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(48.0), width: 120.0, height: 64.0, rotation: 0.0 },
            locked: None,
            visible: None,
            fill: Some([0.2, 0.24, 0.3, 1.0]),
            stroke: None,
        },
    };
    // 🕹️ Used to also `SetSelection { ids: vec![frame_id] }`; selecting the just-created frame is
    // framework-owned now — ask the host to redispatch `interactionSelect` instead (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    Ok(Emit {
        artifact_mutations: vec![LayoutMutation::CreateFrame(CreateFrame { page_id, frame, index: Some(index), layer_id: Some(layer_id) })],
        effects: vec![crate::editor::layout::layout_select_effect(std::slice::from_ref(&frame_id), "replace")],
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
