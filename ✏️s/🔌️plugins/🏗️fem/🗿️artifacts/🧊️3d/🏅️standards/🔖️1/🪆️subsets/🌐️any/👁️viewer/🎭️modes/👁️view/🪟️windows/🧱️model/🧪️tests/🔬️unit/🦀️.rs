use super::*;

#[test]
fn renders_a_prepared_scene_node_without_a_whole_scene_bypass() {
    let node = render(None).expect("fixture surface admission");
    let json = serde_json::to_string(&node).expect("independent semantic JSON oracle");
    assert!(json.contains("world-3d"));
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::testkit::built_surface_scene(&node).expect("assemble world scene");
    assert_eq!(scene.meshes_json, "[]");
    assert_eq!(scene.instances_json, "[]");
}
