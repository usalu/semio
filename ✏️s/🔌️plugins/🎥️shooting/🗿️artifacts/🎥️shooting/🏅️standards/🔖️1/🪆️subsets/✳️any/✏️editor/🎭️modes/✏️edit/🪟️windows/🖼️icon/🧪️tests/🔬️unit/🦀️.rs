use crate::editor::shooting::modes::edit::windows::icon::options;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use crate::editor::shooting::config::ShootingConfig;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::text::snapshot::shooting_icon_render_request_json;
#[allow(unused_imports)]
use crate::ShootingSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::IconRenderScene;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementInput;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagementStatus;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
use crate::editor::shooting::modes::edit::windows::icon::*;
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
    assert_eq!(request["material"]["stroke"], json!("#000000"));
    assert_eq!(request["lights"]["sunAzimuth"], json!(45.0));
    assert_eq!(request["camera"]["projection"], json!("perspective"));
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
