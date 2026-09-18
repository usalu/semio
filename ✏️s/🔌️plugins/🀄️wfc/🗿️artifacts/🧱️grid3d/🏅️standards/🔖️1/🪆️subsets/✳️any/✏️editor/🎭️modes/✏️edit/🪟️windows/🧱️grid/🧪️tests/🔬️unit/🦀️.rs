//! 🔬️ Grid-window laws — the scene is non-empty for every example and the opening pose frames the
//! document instead of standing inside it.

use super::*;
use crate::schema::scene_internals::grid_instances_json;

#[test]
fn the_window_kind_is_a_world3d_surface_bound_to_its_own_interaction_domain() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert!(matches!(definition.surface_kind, semio_framework_plugin::SurfaceKind::World3d));
    assert!(format!("{:?}", definition.interactions).contains(INTERACTION_DOMAIN), "the grid window binds its own interaction domain");
}

#[test]
fn every_example_renders_a_non_empty_grid_scene() {
    for document in [crate::examples::blocks::snapshot(), crate::examples::pipes_3d::snapshot()] {
        let config = Grid3dWindowConfig::default();
        let node = render(&document, &config, &[], None).expect("grid window renders");
        assert!(!format!("{node:?}").is_empty());
        let cells = (document.width * document.height * document.depth) as usize;
        assert_eq!(grid_instances_json(&document).matches("\"meshId\"").count(), cells);
    }
}

#[test]
fn an_unposed_pane_opens_framed_on_what_the_document_spans() {
    let document = crate::examples::blocks::snapshot();
    let (position, target) = framed_camera(&document, &Grid3dWindowConfig::default());
    assert_ne!(position, target, "a camera standing on its own target has no view direction");
    assert_eq!(target, [3.0, 3.0, 2.5]);
}

#[test]
fn a_posed_pane_is_left_exactly_as_the_user_left_it() {
    let document = crate::examples::blocks::snapshot();
    let config = Grid3dWindowConfig { camera_x: 9.0, ..Grid3dWindowConfig::default() };
    let (position, target) = framed_camera(&document, &config);
    assert_eq!(position, [9.0, 0.0, 0.0]);
    assert_eq!(target, [0.0, 0.0, 0.0]);
}
