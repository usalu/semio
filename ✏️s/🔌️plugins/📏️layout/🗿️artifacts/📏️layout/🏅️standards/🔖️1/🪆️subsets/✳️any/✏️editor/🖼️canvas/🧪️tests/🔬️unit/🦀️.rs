use super::*;

#[semio_framework_async_macros::async_test]
async fn active_page_falls_back_to_first_page_when_config_id_unresolved() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig { active_page_id: "no-such-page".into(), ..LayoutWindowConfig::default() };
    let page = active_page(&doc, &config).expect("falls back to first page");
    assert_eq!(page.id, doc.pages[0].id);
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_the_page_background() {
    let doc = crate::standards::v1::subsets::any::schema::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let mut engine = LayoutEngine::new();
    let json = canvas_layers(&mut engine, &doc, &config, &transient, true);
    assert!(json.contains("layout.page-bg"));
}
