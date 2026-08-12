//! ✏️ Layout play app commands — document-mutating content authoring: add/patch pages and frames.
//! Dispatched as VCS operations with a true inverse (see `crate::artifacts::layout::op`).

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::engine::text_to_rgba;
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

//#region 🔖️Shared
/// 🎯️ Builds the exact semantic mutation for a `patchPage` field write from the command's text
/// `value`; unknown fields/mistyped (non-numeric where a number is expected) values yield `None`.
/// `marginTop`/`marginRight`/`marginBottom`/`marginLeft`/`columnsCount`/`columnsGutter` read the
/// page's OTHER current value(s) so `update-page-margins`/`update-page-columns` stay atomic.
fn page_field_mutation(page: &Page, field: &str, value: &str) -> Option<LayoutMutation> {
    let id = page.id.clone();
    match field {
        "name" => Some(LayoutMutation::RenamePage(RenamePage { id, new_name: value.into() })),
        "width" => value.parse::<f64>().ok().map(|new_width| LayoutMutation::ChangePageWidth(ChangePageWidth { id, new_width })),
        "height" => value.parse::<f64>().ok().map(|new_height| LayoutMutation::ChangePageHeight(ChangePageHeight { id, new_height })),
        "marginTop" => value.parse::<f64>().ok().map(|top| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top, right: page.margins.right, bottom: page.margins.bottom, left: page.margins.left })),
        "marginRight" => value.parse::<f64>().ok().map(|right| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right, bottom: page.margins.bottom, left: page.margins.left })),
        "marginBottom" => value.parse::<f64>().ok().map(|bottom| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right: page.margins.right, bottom, left: page.margins.left })),
        "marginLeft" => value.parse::<f64>().ok().map(|left| LayoutMutation::UpdatePageMargins(UpdatePageMargins { id, top: page.margins.top, right: page.margins.right, bottom: page.margins.bottom, left })),
        "columnsCount" => value.parse::<f64>().ok().map(|v| LayoutMutation::UpdatePageColumns(UpdatePageColumns { id, count: v.max(0.0) as u32, gutter: page.columns.gutter })),
        "columnsGutter" => value.parse::<f64>().ok().map(|gutter| LayoutMutation::UpdatePageColumns(UpdatePageColumns { id, count: page.columns.count, gutter })),
        _ => None,
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddFrame
pub mod add_frame {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
        Ok(Emit { artifact_mutations: vec![LayoutMutation::CreateFrame(CreateFrame { page_id, frame, index: Some(index), layer_id: Some(layer_id) })], config_mutations: vec![LayoutConfigMutation::SetSelection { ids: vec![frame_id] }], ..Default::default() })
    }
}
//#endregion 🔖️AddFrame

//#region 🔖️AddPage
pub mod add_page {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-page")]
    pub struct AddPage {}

    pub fn handle(_payload: &AddPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let template = document.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| document.pages.first());
        let (width, height, spread_id, parent_page_id, margins, columns) = template.map_or(
            (595.0, 842.0, "spread-1".into(), None, PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 }, PageColumns { count: 1, gutter: 0.0 }),
            |page| (page.width, page.height, page.spread_id.clone(), page.parent_page_id.clone(), page.margins.clone(), page.columns.clone()),
        );
        let page_id = format!("page-{}", document.pages.len() + 1);
        let layer_id = format!("layer-{page_id}");
        let index = document.pages.len();
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
            artifact_mutations: vec![LayoutMutation::CreatePage(CreatePage { page, index: Some(index) })],
            config_mutations: vec![LayoutConfigMutation::SetActivePage { page_id: page_id.clone() }, LayoutConfigMutation::SetSelection { ids: vec![page_id] }],
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

    pub fn handle(payload: &PatchPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
        let page_id = payload.page_id.clone().unwrap_or_else(|| cfg.snapshot.active_page_id.clone());
        match doc.snapshot.pages.iter().find(|page| page.id == page_id).and_then(|page| page_field_mutation(page, &payload.field, &payload.value)) {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Ok(Emit::default()),
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

    pub fn handle(payload: &PatchFrame, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
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
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        let result = dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
        assert_eq!(result.mutations.len(), 1);
        assert_eq!(app.snapshot().expect("projection").pages[0].frames.len(), before + 1);
    }

    #[test]
    fn undo_redo_round_trips_add_frame() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }), |app| app.snapshot().expect("projection").pages[0].frames.len(), before, before + 1);
    }

    #[test]
    fn patch_page_supports_margins_and_columns() {
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
    fn patch_frame_supports_rect_fill_and_stroke() {
        let mut app = layout_app();
        let before = app.snapshot().expect("projection").pages[0].frames.len();
        dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
        let frame_id = format!("frame-{}", before + 1);
        let result = dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: frame_id.clone(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }));
        assert_eq!(result.mutations.len(), 1);
        let doc = app.snapshot().expect("projection");
        let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
        let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
        assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
    }

    #[test]
    fn patch_frame_supports_text_story_content_and_wrap_mode() {
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
    fn patch_frame_supports_image_link_path() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "linkPath".into(), value: "assets/updated.png".into() }));
        let link = app.snapshot().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
        assert_eq!(link.path, "assets/updated.png");
    }
}
//#endregion 🧪️Tests
