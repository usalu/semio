use super::*;

#[test]
fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead::new(semio_s_artifact_procedural_2d_io_text::snapshot::default_snapshot());
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render(&document).expect("viewer fixture"))).expect("render json");
    assert!(json.contains("canvas-2d"));
}
