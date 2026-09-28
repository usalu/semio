use super::*;
use crate::editor::layout::commands::{patch_frame, patch_page};
use crate::editor::layout::unit_tests::context::{dispatch, layout_app};
use crate::editor::layout::LayoutCommand;

#[semio_framework_async_macros::async_test]
async fn patch_document_renames_sets_print_target_and_clears_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "name".into(), value: "Press sheet".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "cmyk".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "dataFields".into(), value: "{\"title\":\"A\"}".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.name, "Press sheet");
    assert_eq!(snapshot.print_target.as_deref(), Some("cmyk"));
    assert_eq!(snapshot.data_fields_json.as_deref(), Some("{\"title\":\"A\"}"));
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "  ".into() })).await;
    assert_eq!(app.snapshot().expect("projection").print_target, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_updates_the_baseline_grid() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "baselineGrid".into(), value: "18".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "baselineOffset".into(), value: "4".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "snapToBaseline".into(), value: "false".into() })).await;
    let grid = app.snapshot().expect("projection").grid;
    assert_eq!(grid.baseline_grid, 18.0);
    assert_eq!(grid.baseline_offset, 4.0);
    assert!(!grid.snap_to_baseline);
}

#[semio_framework_async_macros::async_test]
async fn patch_page_reorders_and_deletes_and_patch_frame_sets_flags() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "moveLater".into(), value: "true".into() })).await;
    let ids: Vec<String> = app.snapshot().expect("projection").pages.iter().map(|page| page.id.clone()).collect();
    assert_eq!(ids, vec!["page-2".to_string(), "page-1".to_string()]);
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "locked".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "visible".into(), value: "false".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let frame = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(frame.locked());
    assert!(!frame.visible());
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-2".into()), field: "delete".into(), value: "true".into() })).await;
    let left: Vec<String> = app.snapshot().expect("projection").pages.iter().map(|page| page.id.clone()).collect();
    assert_eq!(left, vec!["page-1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_updates_a_paragraph_style() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "paragraph.body.fontSize".into(), value: "18".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "paragraph.body.alignment".into(), value: "center".into() })).await;
    let style = app.snapshot().expect("projection").paragraph_styles.into_iter().find(|style| style.id == "paragraph.body").unwrap();
    assert_eq!(style.font_size, 18.0);
    assert_eq!(style.alignment, "center");
    assert_eq!(style.name, "Body");
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_sets_text_inset_and_patch_page_updates_a_layer() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "insetX".into(), value: "8".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "layer-1.name".into(), value: "Art".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "layer-1.locked".into(), value: "true".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let frame = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
    let crate::Frame::Text { inset, .. } = frame else { panic!("text frame") };
    assert_eq!(inset.x, 8.0);
    let layer = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().layers.iter().find(|layer| layer.id == "layer-1").unwrap();
    assert_eq!(layer.name, "Art");
    assert!(layer.locked);
    assert!(layer.visible);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_adds_a_character_style_and_renames_the_spread() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "addCharacterStyle".into(), value: "Emphasis".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "character-1.italic".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "spread-1.name".into(), value: "Opening".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "parentPageId".into(), value: "".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.character_styles[0].name.as_deref(), Some("Emphasis"));
    assert_eq!(snapshot.character_styles[0].italic, Some(true));
    assert_eq!(snapshot.spreads[0].name, "Opening");
    assert_eq!(snapshot.pages[0].parent_page_id, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_page_adds_a_guide_and_moves_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "addGuide".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "guide.0.y".into(), value: "48".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let guide = &snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().guides[0];
    assert_eq!(guide.y, 48.0);
    assert_eq!(guide.height, 0.0);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_applies_a_character_style_to_the_story() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "addCharacterStyle".into(), value: "Emphasis".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "character-1.fontSize".into(), value: "24".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "characterStyle".into(), value: "character-1".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let run = &snapshot.stories.iter().find(|story| story.id == "story-1").unwrap().style_runs[0];
    assert_eq!(run.character_style_id.as_deref(), Some("character-1"));
    assert_eq!(run.start, 0);
    assert_eq!(run.end, snapshot.stories[0].content.len());
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "characterStyle".into(), value: "".into() })).await;
    assert!(app.snapshot().expect("projection").stories[0].style_runs.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_sets_link_resolution_and_color_profile() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "dpi".into(), value: "150".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "colorProfile".into(), value: "CMYK".into() })).await;
    let link = &app.snapshot().expect("projection").links[0];
    assert_eq!(link.dpi, 150);
    assert_eq!(link.color_profile.as_deref(), Some("CMYK"));
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "colorProfile".into(), value: "".into() })).await;
    assert_eq!(app.snapshot().expect("projection").links[0].color_profile, None);
}
