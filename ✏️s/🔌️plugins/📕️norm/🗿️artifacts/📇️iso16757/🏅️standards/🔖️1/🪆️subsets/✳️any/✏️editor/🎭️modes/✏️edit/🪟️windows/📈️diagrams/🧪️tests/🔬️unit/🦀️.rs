use crate::modes::edit::windows::diagrams::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_DIAGRAMS);
    assert_eq!(definition().id, WINDOW_DIAGRAMS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_plan_diagram_of_the_default_geometry_object() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_DIAGRAMS).await;
    assert!(rendered.contains("Plan view") || rendered.contains("Grundriss"), "the diagrams body must show the plan diagram: {rendered}");
    context::close(&mut app);
}
