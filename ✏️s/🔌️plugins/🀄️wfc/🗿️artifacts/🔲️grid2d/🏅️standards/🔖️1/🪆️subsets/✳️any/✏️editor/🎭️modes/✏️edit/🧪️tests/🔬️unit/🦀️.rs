//! 🧪️ The `edit` mode states two panes, an even split and the three pointer utilities.

use super::*;

#[test]
fn the_mode_is_localized_and_carries_no_unreferenced_tool() {
    let mode = definition();
    assert_eq!(mode.id, GRID2D_EDIT_MODE_ID);
    assert_eq!(mode.label, LocalizedLabel::native("Edit", "Bearbeiten"));
    assert!(mode.tools.is_empty(), "a declared tool no mode references is refused by the plugin builder");
}

#[test]
fn the_layout_places_the_grid_beside_the_preview() {
    let json = dsl::json::to_json_string(&layout());
    assert!(json.contains(grid::WINDOW_KIND_ID), "{json}");
    assert!(json.contains(preview::WINDOW_KIND_ID), "{json}");
}

#[test]
fn the_three_utilities_are_distinct_and_named_by_the_grid_pane() {
    let utilities = utilities();
    assert_eq!(utilities.len(), 3);
    let ids: Vec<&str> = utilities.iter().map(|utility| utility.id.as_str()).collect();
    for expected in [grid::UTILITY_SELECT, grid::UTILITY_PIN, grid::UTILITY_MASK] {
        assert!(ids.contains(&expected), "{expected} missing from {ids:?}");
    }
}
