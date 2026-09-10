use super::*;

#[test]
fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = crate::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshotRead::new(crate::standards::v1::subsets::any::schema::default_snapshot());
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: render(&document).expect("viewer fixture") }).expect("render json");
    assert!(json.contains("canvas-2d"));
}
