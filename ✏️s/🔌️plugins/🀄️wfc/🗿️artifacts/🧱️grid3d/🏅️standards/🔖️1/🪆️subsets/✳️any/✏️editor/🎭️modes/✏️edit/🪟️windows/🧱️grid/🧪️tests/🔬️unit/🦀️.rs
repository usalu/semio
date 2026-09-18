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
        let node = render(&document, &config, &[], None, UTILITY_SELECT).expect("grid window renders");
        assert!(!format!("{node:?}").is_empty());
        let cells = (document.width * document.height * document.depth) as usize;
        assert_eq!(grid_instances_json(&document).matches("\"meshId\"").count(), cells);
    }
}

/// 🖱️ A framework selection write can never become a document edit, so the two writing utilities have
/// to leave `World3dHost` on its plugin-private pick lane: with `domainId` set the host dispatches
/// `interactionSelect` and the app never hears about the click at all.
#[test]
fn a_writing_utility_hands_the_pick_to_the_app_instead_of_the_framework_selection() {
    let document = crate::examples::blocks::snapshot();
    let config = Grid3dWindowConfig::default();
    for (utility, expects_domain) in [(UTILITY_SELECT, true), (UTILITY_PIN, false), (UTILITY_MASK, false)] {
        let painted = scene(&document, &config, &[], None, utility);
        assert_eq!(painted.domain_id.as_deref(), expects_domain.then_some(INTERACTION_DOMAIN), "utility '{utility}' bound the wrong pick lane");
        assert_eq!(painted.domain_granularity_id.as_deref(), expects_domain.then_some(INTERACTION_GRANULARITY_CELL), "a domain-bound pick must spell the granularity, or the host prunes it as `handle`");
        let method = if expects_domain { "interactionSelect" } else { ACTION_WORLD_SELECT };
        assert!(painted.selection_json.contains(method), "utility '{utility}' must report through {method}: {}", painted.selection_json);
        assert!(painted.selection_json.contains(INTERACTION_GRANULARITY_CELL), "the selection lane must carry the cell granularity");
        assert!(render(&document, &config, &[], None, utility).is_ok(), "utility '{utility}' must still render");
    }
}

/// 📚️ The navbar example picker dispatches `setActiveExample` against the FOCUSED window's kind; a
/// kind that does not declare it makes the whole picker dead (`undeclared-action`).
#[test]
fn the_grid_window_declares_the_example_picker_and_the_world_pick() {
    let ids = definition().actions.iter().map(|action| action.id.clone()).collect::<Vec<String>>();
    assert!(ids.contains(&ACTION_SET_ACTIVE_EXAMPLE.to_string()), "the example picker verb is undeclared: {ids:?}");
    assert!(ids.contains(&ACTION_WORLD_SELECT.to_string()), "the plugin-private world pick verb is undeclared: {ids:?}");
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
