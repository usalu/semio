//! ✏️ Layout play app commands — document-mutating content authoring: add/patch pages and frames.
//! Dispatched as VCS operations with a true inverse (see `crate::artifacts::layout::op`).

use crate::apps::layout::config::{LayoutConfig, LayoutConfigOperation};
use crate::artifacts::layout::engine::text_to_rgba;
use crate::artifacts::layout::op::LayoutOperation;
use crate::artifacts::layout::{Frame, FramePatch, ImageLinkPatch, LayoutDocument, PageColumns, PageMargins, PagePatch, TextStoryPatch};
use protocol::CollectionOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🩹️ Builds the `PagePatch` for a `patchPage` field write from the command's text `value`; unknown
/// fields/mistyped (non-numeric where a number is expected) values yield `None`.
fn page_patch_for_field(field: &str, value: &str) -> Option<PagePatch> {
    match field {
        "name" => Some(PagePatch { name: Some(value.into()), ..Default::default() }),
        "width" => value.parse::<f64>().ok().map(|v| PagePatch { width: Some(v), ..Default::default() }),
        "height" => value.parse::<f64>().ok().map(|v| PagePatch { height: Some(v), ..Default::default() }),
        "marginTop" => value.parse::<f64>().ok().map(|v| PagePatch { margin_top: Some(v), ..Default::default() }),
        "marginRight" => value.parse::<f64>().ok().map(|v| PagePatch { margin_right: Some(v), ..Default::default() }),
        "marginBottom" => value.parse::<f64>().ok().map(|v| PagePatch { margin_bottom: Some(v), ..Default::default() }),
        "marginLeft" => value.parse::<f64>().ok().map(|v| PagePatch { margin_left: Some(v), ..Default::default() }),
        "columnsCount" => value.parse::<f64>().ok().map(|v| PagePatch { columns_count: Some(v.max(0.0) as u32), ..Default::default() }),
        "columnsGutter" => value.parse::<f64>().ok().map(|v| PagePatch { columns_gutter: Some(v), ..Default::default() }),
        _ => None,
    }
}

/// 🩹️ Builds the bounds `FramePatch` for an `x`/`y`/`width`/`height` frame field write.
fn frame_bounds_patch(field: &str, value: f64) -> FramePatch {
    match field {
        "x" => FramePatch { x: Some(value), ..Default::default() },
        "y" => FramePatch { y: Some(value), ..Default::default() },
        "width" | "w" => FramePatch { width: Some(value), ..Default::default() },
        "height" | "h" => FramePatch { height: Some(value), ..Default::default() },
        _ => FramePatch::default(),
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddFrame
pub mod add_frame {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-frame")]
    pub struct AddFrame {
        pub kind: String,
        pub x: Option<f64>,
        pub y: Option<f64>,
    }

    pub fn handle(payload: &AddFrame, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
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
        Ok(Emit { document_operations: vec![LayoutOperation::AddFrame { page_id, index, frame, layer_id: Some(layer_id) }], config_operations: vec![LayoutConfigOperation::SetSelection { ids: vec![frame_id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddFrame

//#region 🔖️AddPage
pub mod add_page {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-page")]
    pub struct AddPage {}

    pub fn handle(_payload: &AddPage, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        let template = document.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| document.pages.first());
        let (width, height, spread_id, parent_page_id, margins, columns) = template.map_or(
            (595.0, 842.0, "spread-1".into(), None, PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 }, PageColumns { count: 1, gutter: 0.0 }),
            |page| (page.width, page.height, page.spread_id.clone(), page.parent_page_id.clone(), page.margins.clone(), page.columns.clone()),
        );
        let page_id = format!("page-{}", document.pages.len() + 1);
        let layer_id = format!("layer-{page_id}");
        let page = crate::artifacts::layout::Page {
            id: page_id.clone(),
            name: format!("Page {}", document.pages.len() + 1),
            spread_id,
            parent_page_id,
            width,
            height,
            margins,
            columns,
            guides: Vec::new(),
            layer_ids: vec![layer_id.clone()],
            layers: vec![crate::artifacts::layout::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
            frames: Vec::new(),
            overrides: Vec::new(),
        };
        Ok(Emit {
            document_operations: vec![LayoutOperation::Pages(CollectionOperation::Add { index: document.pages.len(), item: page })],
            config_operations: vec![LayoutConfigOperation::SetActivePage { page_id: page_id.clone() }, LayoutConfigOperation::SetSelection { ids: vec![page_id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddPage

//#region 🔖️PatchPage
pub mod patch_page {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-page")]
    pub struct PatchPage {
        pub page_id: Option<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchPage, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.projection.active_page_id.clone());
        match page_patch_for_field(&payload.field, &payload.value) {
            Some(patch) if doc.projection.pages.iter().any(|page| page.id == page_id) => Ok(Emit::operations(vec![LayoutOperation::Pages(CollectionOperation::Patch { id: page_id, patch })])),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchPage

//#region 🔖️PatchFrame
pub mod patch_frame {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-frame")]
    pub struct PatchFrame {
        pub frame_id: String,
        pub page_id: Option<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchFrame, doc: &DocumentView<'_, LayoutDocument>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let document = doc.projection;
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.projection.active_page_id.clone());
        if payload.frame_id.is_empty() {
            return Ok(Emit::default());
        }
        let Some(page) = document.pages.iter().find(|page| page.id == page_id) else {
            return Ok(Emit::default());
        };
        let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
            return Ok(Emit::default());
        };
        match payload.field.as_str() {
            "x" | "y" | "width" | "w" | "height" | "h" => match payload.value.parse::<f64>() {
                Ok(number) => Ok(Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: payload.frame_id.clone(), patch: frame_bounds_patch(&payload.field, number) }])),
                Err(_) => Ok(Emit::default()),
            },
            "fill" | "stroke" => {
                let rgba = text_to_rgba(&payload.value);
                let patch = if payload.field == "fill" { FramePatch { fill: Some(rgba), ..Default::default() } } else { FramePatch { stroke: Some(rgba), ..Default::default() } };
                Ok(Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: payload.frame_id.clone(), patch }]))
            }
            "wrapMode" => Ok(Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: payload.frame_id.clone(), patch: FramePatch { wrap_mode: Some(payload.value.clone()), ..Default::default() } }])),
            "columns" => match payload.value.parse::<f64>() {
                Ok(count) => Ok(Emit::operations(vec![LayoutOperation::PatchFrame { page_id, frame_id: payload.frame_id.clone(), patch: FramePatch { columns: Some(count.max(0.0) as u32), ..Default::default() } }])),
                Err(_) => Ok(Emit::default()),
            },
            "storyContent" => {
                let story_id = match frame {
                    Frame::Text { story_id, .. } => Some(story_id.clone()),
                    _ => None,
                };
                match story_id {
                    Some(story_id) if document.stories.iter().any(|story| story.id == story_id) => Ok(Emit::operations(vec![LayoutOperation::Stories(CollectionOperation::Patch { id: story_id, patch: TextStoryPatch { content: Some(payload.value.clone()) } })])),
                    _ => Ok(Emit::default()),
                }
            }
            "linkPath" => {
                let link_id = match frame {
                    Frame::Image { link_id, .. } => Some(link_id.clone()),
                    _ => None,
                };
                match link_id {
                    Some(link_id) if document.links.iter().any(|link| link.id == link_id) => Ok(Emit::operations(vec![LayoutOperation::Links(CollectionOperation::Patch { id: link_id, patch: ImageLinkPatch { path: Some(payload.value.clone()) } })])),
                    _ => Ok(Emit::default()),
                }
            }
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchFrame

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{dispatch, layout_app};
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn add_frame_action_appends_rect() {
        let mut app = layout_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        let result = dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    fn undo_redo_round_trips_add_frame() {
        let mut app = layout_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }), |app| app.projection().expect("projection").pages[0].frames.len(), before, before + 1);
    }

    #[test]
    fn patch_page_supports_margins_and_columns() {
        let mut app = layout_app();
        for (field, value) in [("marginTop", 60.0), ("marginRight", 40.0), ("marginBottom", 60.0), ("marginLeft", 40.0), ("columnsGutter", 18.0)] {
            let result = dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: field.into(), value: value.to_string() }));
            assert_eq!(result.operations.len(), 1, "field {field} should apply");
        }
        dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "columnsCount".into(), value: "3".into() }));
        let page = app.projection().expect("projection").pages.into_iter().find(|page| page.id == "page-1").unwrap();
        assert_eq!(page.columns.count, 3);
    }

    #[test]
    fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = layout_app();
        let before = app.projection().expect("projection").pages[0].frames.len();
        dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
        let frame_id = format!("frame-{}", before + 1);
        let result = dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: frame_id.clone(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }));
        assert_eq!(result.operations.len(), 1);
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    fn patch_frame_supports_text_story_content_and_wrap_mode() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "storyContent".into(), value: "Edited story body.".into() }));
        let story = app.projection().expect("projection").stories.into_iter().find(|story| story.id == "story-1").unwrap();
        assert_eq!(story.content, "Edited story body.");

        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "wrapMode".into(), value: "contour".into() }));
        let doc = app.projection().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
        let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
        assert_eq!(wrap_mode, "contour");
    }

    #[test]
    fn patch_frame_supports_image_link_path() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "linkPath".into(), value: "assets/updated.png".into() }));
        let link = app.projection().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }
}
//#endregion 🧪️Tests
