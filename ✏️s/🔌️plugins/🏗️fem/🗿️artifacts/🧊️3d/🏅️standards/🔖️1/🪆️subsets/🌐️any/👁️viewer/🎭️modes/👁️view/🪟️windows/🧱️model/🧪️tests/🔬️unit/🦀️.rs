use super::*;

#[test]
fn renders_the_document_scene_without_a_whole_scene_bypass() {
    let doc = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotRetiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("the shipped demo parses"));
    let node = render(&doc, None, semio_plugin::Locale::En).expect("fixture surface admission");
    let scene: semio_ui_scene::World3dScene = semio_plugin_testing_laws::artifact_app_laws::built_surface_scene(&node).expect("assemble world scene");
    assert!(scene.meshes_json.contains("solid-sol1"), "the viewer draws the document's solids: {}", scene.meshes_json);
    assert!(scene.instances_json.contains(&format!("\"id\":\"{}\"", semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_entity_target("node", "n00_g").expect("node address"))), "the viewer draws the document's nodes: {}", scene.instances_json);
    assert!(scene.domain_id.is_none(), "the read-only viewer binds no interaction domain");
    let json = semio_plugin_testing_laws::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains("world-3d"));
}
