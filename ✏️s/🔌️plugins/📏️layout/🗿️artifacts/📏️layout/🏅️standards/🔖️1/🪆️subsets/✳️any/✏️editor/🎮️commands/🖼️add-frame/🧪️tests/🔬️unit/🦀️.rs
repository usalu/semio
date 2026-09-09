use super::*;
use crate::editor::layout::commands::{patch_frame, patch_page};
use crate::editor::layout::testkit::{dispatch, layout_app};
use crate::editor::layout::LayoutCommand;

#[semio_framework_async_macros::async_test]
async fn add_frame_action_appends_rect() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection").pages[0].frames.len();
    let result = dispatch(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None })).await;
    assert_eq!(result.mutations.len(), 1);
    assert_eq!(app.snapshot().expect("projection").pages[0].frames.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_add_frame() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection").pages[0].frames.len();
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None }), |app| app.snapshot().expect("projection").pages[0].frames.len(), before, before + 1).await;
}

#[semio_framework_async_macros::async_test]
async fn patch_page_supports_margins_and_columns() {
    let mut app = layout_app().await;
    for (field, value) in [("marginTop", 60.0), ("marginRight", 40.0), ("marginBottom", 60.0), ("marginLeft", 40.0), ("columnsGutter", 18.0)] {
        let result = dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: field.into(), value: value.to_string() })).await;
        assert_eq!(result.mutations.len(), 1, "field {field} should apply");
    }
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "columnsCount".into(), value: "3".into() })).await;
    let page = app.snapshot().expect("projection").pages.into_iter().find(|page| page.id == "page-1").unwrap();
    assert_eq!(page.columns.count, 3);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_supports_rect_fill_and_stroke() {
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection").pages[0].frames.len();
    dispatch(&mut app, LayoutCommand::AddFrame(AddFrame { kind: "rect".into(), x: None, y: None })).await;
    let frame_id = format!("frame-{}", before + 1);
    let result = dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: frame_id.clone(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() })).await;
    assert_eq!(result.mutations.len(), 1);
    let doc = app.snapshot().expect("projection");
    let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == frame_id).unwrap();
    let Frame::Rect { fill, .. } = frame else { panic!("expected rect frame") };
    assert_eq!(fill.unwrap(), [0.5, 0.4, 0.3, 1.0]);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_supports_text_story_content_and_wrap_mode() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "storyContent".into(), value: "Edited story body.".into() })).await;
    let story = app.snapshot().expect("projection").stories.into_iter().find(|story| story.id == "story-1").unwrap();
    assert_eq!(story.content, "Edited story body.");

    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "wrapMode".into(), value: "contour".into() })).await;
    let doc = app.snapshot().expect("projection");
    let frame = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
    let Frame::Text { wrap_mode, .. } = frame else { panic!("expected text frame") };
    assert_eq!(wrap_mode, "contour");
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_supports_image_link_path() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "linkPath".into(), value: "assets/updated.png".into() })).await;
    let link = app.snapshot().expect("projection").links.into_iter().find(|link| link.id == "link-missing").unwrap();
    assert_eq!(link.path, "assets/updated.png");
}
