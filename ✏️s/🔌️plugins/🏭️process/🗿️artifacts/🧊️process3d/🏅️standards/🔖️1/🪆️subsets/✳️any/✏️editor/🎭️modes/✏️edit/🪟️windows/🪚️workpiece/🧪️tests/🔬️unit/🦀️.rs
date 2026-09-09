use super::*;
use crate::editor::process3d::testkit;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world3d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    let doc = Process3dSnapshot::default();
    let engagement = engagement(&doc, &Process3dConfig::default(), crate::editor::process3d::config::PROCESS3D_DEFAULT_UTILITY, &crate::editor::process3d::terminology::Process3dLabels::NATIVE_EN);
    assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options");
}

#[semio_framework_async_macros::async_test]
async fn render_world_scene_contains_processed_mesh() {
    let mut app = testkit::app();
    let node = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
}
