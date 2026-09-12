use super::*;
use crate::editor::shooting::unit_tests::context::{icon_window_measures, shooting_app};
use serde_json::{json, Value};

#[semio_framework_async_macros::async_test]
async fn renders_icon_render_scene_with_real_request() {
    let mut app = shooting_app().await;
    let scene = crate::editor::shooting::unit_tests::context::icon_scene(&mut app).await;
    let request: Value = serde_json::from_str(&scene.request_json).unwrap();
    assert_eq!(request["assetUrl"], json!("/mesh/🧊️base.glb"));
    assert_eq!(request["format"], json!("svg"));
    assert_eq!(request["shape"], json!("rectangle"));
    assert!(request.get("background").is_none(), "transparent default fixture background is omitted");
    assert_eq!(request["lights"]["sunAzimuth"], json!(45.0));
    assert!(scene.footer.as_deref().unwrap().contains("256×256"));
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_three_icon_measures() {
    let mut app = shooting_app().await;
    let measures = icon_window_measures(&mut app).await;
    assert_eq!(measures.len(), 3);
    assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Select { .. })));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_icon_render_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SHOOTING_PLAY_BODY_ICON);
    assert!(matches!(definition.surface_kind, SurfaceKind::IconRender));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
