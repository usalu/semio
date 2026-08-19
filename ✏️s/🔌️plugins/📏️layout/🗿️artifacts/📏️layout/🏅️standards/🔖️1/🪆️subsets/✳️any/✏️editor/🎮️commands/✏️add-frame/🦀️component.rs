//! ✏️ ✏️ Layout play app commands command — `add-frame`.

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
pub struct AddFrame {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

pub async fn handle(payload: &AddFrame, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
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
            bounds: crate::artifacts::layout::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(120.0), width: 200.0, height: 120.0, rotation: 0.0 },
            locked: None,
            visible: None,
            story_id: document.stories.first().map_or_else(|| "story-1".into(), |story| story.id.clone()),
            thread_next: None,
            columns: 1,
            inset: crate::artifacts::layout::LayoutRect { x: 4.0, y: 4.0, width: 192.0, height: 112.0 },
            wrap_mode: "box".into(),
        },
        "image" => Frame::Image {
            id: frame_id.clone(),
            layer_id: layer_id.clone(),
            bounds: crate::artifacts::layout::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(280.0), width: 160.0, height: 120.0, rotation: 0.0 },
            locked: None,
            visible: None,
            link_id: document.links.first().map_or_else(|| "link-missing".into(), |link| link.id.clone()),
        },
        _ => Frame::Rect {
            id: frame_id.clone(),
            layer_id: layer_id.clone(),
            bounds: crate::artifacts::layout::LayoutBounds { x: payload.x.unwrap_or(48.0), y: payload.y.unwrap_or(48.0), width: 120.0, height: 64.0, rotation: 0.0 },
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
mod tests {
    use super::*;
    use crate::editor::layout::commands::{patch_frame, patch_page};
    use crate::editor::layout::testkit::{dispatch, layout_app};
    use crate::editor::layout::LayoutCommand;

    #[test]
    async fn add_frame_action_appends_rect() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        let result = dispatch(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None }));
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(app.snapshot().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    async fn undo_redo_round_trips_add_frame() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None }), |app| app.snapshot().expect("projection").pages[0].frames.len(), before, before + 1);
    }

    #[test]
    async fn patch_page_supports_margins_and_columns() {
        let mut app = layout_app();
        for (field, value) in [("marginTop", 60.0), ("marginRight", 40.0), ("marginBottom", 60.0), ("marginLeft", 40.0), ("columnsGutter", 18.0)] {
            let result = dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: field.into(), value: value.to_string() }));
            assert_eq!(result.mutations.len(), 1, "field {field} should apply");
        }
        dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "columnsCount".into(), value: "3".into() }));
        let page = app.snapshot().expect("projection").pages.into_iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    async fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        dispatch(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None }));
        let frame_id = format!("frame-{}", before + 1);
        let result = dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: frame_id.clone(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }));
        assert_eq!(result.mutations.len(), 1);
        let doc = app.snapshot().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    async fn patch_frame_supports_text_story_content_and_wrap_mode() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "storyContent".into(), value: "Edited story body.".into() }));
        let story = app.snapshot().expect("projection").stories.into_iter().find(|story| story.id == "story-1").unwrap();
        assert_eq!(story.content, "Edited story body.");

        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "wrapMode".into(), value: "contour".into() }));
        let doc = app.snapshot().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
        let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "contour");
    }

    #[test]
    async fn patch_frame_supports_image_link_path() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "linkPath".into(), value: "assets/updated.png".into() }));
        let link = app.snapshot().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }
}
//#endregion 🧪️Tests
