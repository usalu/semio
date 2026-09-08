
use crate::editor::lowpoly::testkit::{app, render};

#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut a = app().await;
    assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_MAIN).await.contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_mesh_ops_to_main_only() {
    let definition = crate::editor::lowpoly::create_lowpoly_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let main = resolve(super::LOWPOLY_PLAY_WINDOW_MAIN);
    let uv = resolve(crate::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_WINDOW_UV);
    for mesh_operation in ["extrude", "addPrimitive", "bevel", "loopCut", "mirror", "unwrapActive", "markUvSeam"] {
        assert!(main.contains(&mesh_operation.to_string()), "MAIN must expose mesh operation {mesh_operation}");
        assert!(!uv.contains(&mesh_operation.to_string()), "UV must NOT expose mesh operation {mesh_operation}");
    }
    for paint_operation in ["paintFill", "fillBucket", "addPaintLayer"] {
        assert!(main.contains(&paint_operation.to_string()), "MAIN must expose paint operation {paint_operation}");
        assert!(uv.contains(&paint_operation.to_string()), "UV must expose paint operation {paint_operation}");
    }
}
