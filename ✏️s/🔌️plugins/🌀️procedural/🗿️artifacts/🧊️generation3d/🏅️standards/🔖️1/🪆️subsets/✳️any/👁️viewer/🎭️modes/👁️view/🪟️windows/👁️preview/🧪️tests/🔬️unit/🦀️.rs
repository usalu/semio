
use super::*;

#[test]
fn definition_declares_the_shared_mesh_window_kit() {
    let def = definition();
    assert_eq!(def.id, MeshWindowKit::KIND_ID);
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::default_snapshot();
    let _node = render(&document);
}

#[test]
fn render_emits_real_tessellated_geometry_for_the_default_fixture() {
    let document = crate::standards::v1::subsets::any::schema::default_snapshot();
    let (meshes_json, instances_json) = evaluated_meshes_and_instances(&document.fixture);
    assert_ne!(meshes_json, "[]", "default fixture should evaluate and tessellate at least one preview mesh");
    assert_ne!(instances_json, "[]");
}
