use crate::modes::edit::windows::diagrams::*;
use crate::unit_tests::context;
use semio_s_artifact_norm_en1997_scene::diagrams;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_DIAGRAMS);
    assert_eq!(definition().id, WINDOW_DIAGRAMS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_earth_pressure_diagram_of_the_default_wall() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_DIAGRAMS).await;
    assert!(rendered.contains("Earth pressure on") || rendered.contains("Erddruck auf"), "the diagrams body must show the earth pressure diagram: {rendered}");
    context::close(&mut app);
}
