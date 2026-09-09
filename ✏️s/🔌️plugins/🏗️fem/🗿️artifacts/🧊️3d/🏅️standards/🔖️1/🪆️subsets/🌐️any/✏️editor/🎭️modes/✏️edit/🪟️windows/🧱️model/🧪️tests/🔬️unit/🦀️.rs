use super::*;
use crate::editor::fem3d::testkit::{fem3d_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_fem3d_model_scene() {
    let mut app = fem3d_app();
    let json = render_body(&mut app, FEM3D_BODY_MODEL);
    assert!(json.contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
    let mut app = fem3d_app();
    crate::editor::fem3d::testkit::dispatch(&mut app, crate::editor::fem3d::Fem3dCommand::SetActiveExample(crate::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &FemCamera::default()).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected world surface") };
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(props).expect("decode world scene");
    assert!(scene.meshes_json.contains("solid-sol1"), "expected a solid mesh for the example fixture: {}", scene.meshes_json);
    assert!(scene.instances_json.contains("el-e1"), "expected a single oriented box instance per member: {}", scene.instances_json);
    assert!(!scene.instances_json.contains("\"sphere\""), "sphere markers should be gone: {}", scene.instances_json);
}
