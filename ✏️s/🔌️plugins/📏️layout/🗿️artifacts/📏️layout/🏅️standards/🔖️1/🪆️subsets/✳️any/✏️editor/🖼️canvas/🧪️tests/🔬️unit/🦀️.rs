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
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&doc, &config, &transient, &interaction, false);
    assert!(json.contains("\"kind\":\"text\"") && json.contains("Hello layout"), "preview must emit readable story text: {json}");
    assert!(!json.contains(".glyphs"), "placeholder glyph bars must not be emitted: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_the_page_background() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&doc, &config, &transient, &interaction, true);
    assert!(json.contains("layout.page-bg"));
}

#[semio_framework_async_macros::async_test]
async fn selected_and_hovered_frames_get_chrome_strokes() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let selected = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], hovered_ids: vec!["frame-text-1".into()] };
    let json = canvas_layers(&doc, &config, &transient, &selected, true);
    assert!(json.contains("frame-1") && json.contains("\"width\":2.0"), "selected frame gets a thicker stroke: {json}");
    assert!(json.contains("frame-text-1") && json.contains("\"width\":1.75"), "hovered frame gets a hover stroke: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_draw_a_rotated_frame() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").expect("frame");
    let crate::Frame::Rect { bounds, .. } = frame else { panic!("rect") };
    *bounds = crate::LayoutBounds { x: 0.0, y: 0.0, width: 100.0, height: 20.0, rotation: std::f64::consts::FRAC_PI_2 };
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("frame-1"), "the rotated frame is still painted: {json}");
    assert!(json.contains("\"to\":[40.0,60.0]") || json.contains("\"to\":[60.0,-40.0]") || json.contains("40.0"), "rotation moves a corner off the axis-aligned box: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_rotated_proxy_is_not_painted_as_an_upright_image() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.links[0].state = Some("ready".into());
    doc.links[0].proxy_data_url = Some("data:image/png;base64,AA==".into());
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let crate::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!json.contains("data:image/png;base64,AA=="), "a rotated proxy is not an upright bitmap: {json}");
    assert!(json.contains("frame-image-1"), "the rotated frame is still drawn: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_paints_a_ready_proxy_as_an_image() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.links[0].state = Some("ready".into());
    doc.links[0].proxy_data_url = Some("data:image/png;base64,AA==".into());
    let config = LayoutWindowConfig::default();
    let json = canvas_layers(&doc, &config, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("\"kind\":\"image\"") && json.contains("data:image/png;base64,AA=="), "a ready proxy is a host image layer: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_omits_story_text_when_zoomed_out() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let mut config = LayoutWindowConfig::default();
    config.camera.zoom = 0.1;
    let json = canvas_layers(&doc, &config, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), false);
    assert!(!json.contains("Hello layout"), "zoomed-out preview keeps the page workable without story text: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_arms_a_world_gumball_for_the_selection() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let selected = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], hovered_ids: Vec::new() };
    let mut transform = LayoutWindowConfig::default();
    transform.active_utility = "transform".into();
    let json = canvas_layers(&doc, &transform, &LayoutWindowTransient::default(), &selected, true);
    assert!(json.contains("\"space\":\"world\"") && json.contains("meta:gumball"), "transform plus a selection arms the world gumball: {json}");
    let selecting = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &selected, true);
    assert!(selecting.contains("meta:utility") && selecting.contains("select") && !selecting.contains("meta:gumball"), "select omits the gumball: {selecting}");
    let idle = canvas_layers(&doc, &transform, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!idle.contains("meta:gumball"), "transform with an empty selection omits the gumball: {idle}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_labels_a_linked_pdf_when_it_has_no_proxy() {
    let mut doc = crate::standards::v1::subsets::any::schema::default_document();
    doc.links[0].artifact_kind = "s.stdio.pdf".into();
    doc.links[0].artifact_ref = "sheet".into();
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("s.stdio.pdf") && json.contains(".preview"), "a linked pdf without a proxy shows its kind and a page mark: {json}");
    let mut far = LayoutWindowConfig::default();
    far.camera.zoom = 0.1;
    let overview = canvas_layers(&doc, &far, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!overview.contains(".preview") && !overview.contains("s.stdio.pdf"), "a zoomed-out sheet keeps the frame and drops the accurate mark: {overview}");
}
