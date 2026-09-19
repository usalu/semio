use super::*;
use crate::editor::layout::LayoutInteractionSnapshot;

#[semio_framework_async_macros::async_test]
async fn active_page_falls_back_to_first_page_when_config_id_unresolved() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig { active_page_id: "no-such-page".into(), ..LayoutWindowConfig::default() };
    let page = active_page(&doc, &config).expect("falls back to first page");
    assert_eq!(page.id, doc.pages[0].id);
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_story_text_not_glyph_bars() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let mut engine = LayoutEngine::new();
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&mut engine, &doc, &config, &transient, &interaction, false);
    assert!(json.contains("\"kind\":\"text\"") && json.contains("Hello layout"), "preview must emit readable story text: {json}");
    assert!(!json.contains(".glyphs"), "placeholder glyph bars must not be emitted: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_the_page_background() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let mut engine = LayoutEngine::new();
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&mut engine, &doc, &config, &transient, &interaction, true);
    assert!(json.contains("layout.page-bg"));
}

#[semio_framework_async_macros::async_test]
async fn selected_and_hovered_frames_get_chrome_strokes() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let mut engine = LayoutEngine::new();
    let selected = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], hovered_ids: vec!["frame-text-1".into()] };
    let json = canvas_layers(&mut engine, &doc, &config, &transient, &selected, true);
    assert!(json.contains("frame-1") && json.contains("\"width\":2.0"), "selected frame gets a thicker stroke: {json}");
    assert!(json.contains("frame-text-1") && json.contains("\"width\":1.75"), "hovered frame gets a hover stroke: {json}");
}
