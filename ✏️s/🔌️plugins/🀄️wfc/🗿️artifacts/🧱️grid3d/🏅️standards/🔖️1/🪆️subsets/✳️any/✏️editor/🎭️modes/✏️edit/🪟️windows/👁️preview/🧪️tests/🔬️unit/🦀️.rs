//! 🔬️ Preview-window laws — a solved document publishes one instance per solved cell, the residency
//! makes the delta incremental instead of a second copy of the full set, and the status line stays
//! short enough to admit.

use super::*;
use crate::schema::inferences::Grid3dAssignment;
use crate::schema::scene_internals::preview_instances_json;

fn row(x: u32, y: u32, z: u32, tile: &str) -> Grid3dAssignment {
    Grid3dAssignment { x, y, z, tile_id: tile.to_string() }
}

#[test]
fn the_window_kind_is_a_read_only_world3d_surface() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert!(definition.actions.is_empty(), "the preview shows an inference; it never authors one");
    assert!(definition.utilities.is_empty());
}

#[test]
fn every_example_renders_one_instance_per_solved_cell() {
    for (index, document) in [crate::examples::blocks::snapshot(), crate::examples::pipes_3d::snapshot()].into_iter().enumerate() {
        let window = format!("preview-render-{index}");
        let node = render(&document, &Grid3dWindowConfig::default(), &window).expect("preview renders");
        assert!(!format!("{node:?}").is_empty());
        let cells = (document.width * document.height * document.depth) as usize - document.masked.len();
        let (instances, _) = publish(&window, &document, solve(&document).expect("the example solves").assignments);
        assert_eq!(instances.matches("\"meshId\"").count(), cells);
    }
}

#[test]
fn a_cold_residency_publishes_no_delta_and_a_small_change_publishes_one() {
    let mut residency = Grid3dPreviewResidency::default();
    assert!(residency.refresh(vec![row(0, 0, 0, "a"), row(1, 0, 0, "a"), row(2, 0, 0, "a"), row(3, 0, 0, "a")]));
    assert!(!residency.delta_is_worth_publishing(), "a cold publication buys a consumer nothing");
    assert!(residency.refresh(vec![row(0, 0, 0, "a"), row(1, 0, 0, "b"), row(2, 0, 0, "a"), row(3, 0, 0, "a")]));
    assert!(residency.delta_is_worth_publishing(), "one changed row out of four is worth a delta");
    assert!(!residency.refresh(vec![row(0, 0, 0, "a"), row(1, 0, 0, "b"), row(2, 0, 0, "a"), row(3, 0, 0, "a")]), "a no-op refresh keeps the last delta standing");
}

#[test]
fn a_contradiction_paints_the_empty_scene_and_says_so() {
    let mut document = crate::examples::blocks::snapshot();
    document.rules.clear();
    render(&document, &Grid3dWindowConfig::default(), "preview-contradiction").expect("preview renders");
    let status = status_json(&document, false, "[]");
    assert!(status.contains("contradiction"));
}

#[test]
fn the_status_line_stays_inside_one_ui_text_admission_unit() {
    let document = crate::examples::blocks::snapshot();
    let status = status_json(&document, true, &preview_instances_json(&document, &solve(&document).expect("solves").assignments));
    assert!(status.len() <= 512, "an oversized status kills the whole surface refresh, not just this field");
    assert!(status.contains("\"state\""));
}
