use super::*;

#[test]
fn renders_a_canvas_2d_scene_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&document).expect("fixture surface admission"))).expect("fixture projection");
    assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
}

#[test]
fn renders_mesh_edge_preview_for_the_default_example() {
    use store::ArtifactDsl;
    let document = Fem2dSnapshot::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM2D_EXAMPLE_TEXT).expect("parse default example");
    let node = render(&document).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
    assert!(scene.layers_json.contains("mesh-edge-"), "expected mesh-edge preview layers in the view scene: {}", scene.layers_json);
}
