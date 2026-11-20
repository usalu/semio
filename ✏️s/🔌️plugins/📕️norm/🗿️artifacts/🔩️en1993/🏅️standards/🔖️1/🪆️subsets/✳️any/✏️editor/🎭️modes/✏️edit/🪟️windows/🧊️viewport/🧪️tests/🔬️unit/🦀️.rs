use crate::modes::edit::windows::viewport::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
    assert_eq!(definition().surface_kind, semio_plugin::SurfaceKind::World3d);
}

#[semio_framework_async_macros::async_test]
async fn renders_a_world_surface_without_json_text() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_VIEWPORT).await;
    assert!(rendered.contains("world-3d") || rendered.contains("World3d") || rendered.contains("world3d"), "the viewport body must render the world surface: {rendered}");
    assert!(!rendered.contains("\"members\""), "no document JSON in the viewport");
    let specs = insert_specs();
    let last = specs.last().expect("insert tool");
    assert!(rendered.contains(&format!("\"total\":{}", specs.len() + 1)), "the palette declares all insert tools: {rendered}");
    let final_row = context::render_with_windows(&mut app, BODY_VIEWPORT, vec![semio_framework::TreeWindowRequest {
        body_key: BODY_VIEWPORT.into(),
        node_key: semio_s_plugin_norm_app_surface::toolbar::VIEWPORT_INSERT_SECTION_ID.into(),
        open: Some(true),
        offset: specs.len() as u32,
        rows: 1,
    }]).await;
    assert!(final_row.contains(&format!("norm-viewport.insert.{}", last.id)), "the host can reach the final insert tool: {final_row}");
    assert!(!final_row.contains("norm-viewport.insert.off"), "the final-row request only renders that row");
    context::close(&mut app);
}
