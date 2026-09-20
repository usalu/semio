use super::*;

#[test]
fn renders_the_document_scene_without_a_whole_scene_bypass() {
    let doc = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_demo_snapshot();
    let node = render(&doc, None).expect("fixture surface admission");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assemble world scene");
    assert!(scene.meshes_json.contains("solid-sol1"), "the viewer draws the document's solids: {}", scene.meshes_json);
    assert!(scene.instances_json.contains("\"id\":\"n00_g\""), "the viewer draws the document's nodes: {}", scene.instances_json);
    assert!(scene.domain_id.is_none(), "the read-only viewer binds no interaction domain");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains("world-3d"));
}
