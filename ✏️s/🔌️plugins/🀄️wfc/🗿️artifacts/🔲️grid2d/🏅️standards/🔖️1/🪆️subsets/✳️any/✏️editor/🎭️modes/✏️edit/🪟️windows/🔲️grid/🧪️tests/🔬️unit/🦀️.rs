//! 🧪️ The grid pane renders a real, non-empty board for every bundled example, one node per cell,
//! snapped to the authored cell size.

use super::*;
use crate::editor::grid2d::window::Grid2dWindowConfig;
use serde_json::Value;

fn fixture(document: &Grid2dSnapshot) -> Value {
    serde_json::from_str(&fixture_json(document)).expect("the fixture is real json")
}

#[test]
fn the_window_declares_a_board_surface_with_a_registered_icon() {
    let window = definition();
    assert_eq!(window.id, WINDOW_KIND_ID);
    assert_eq!(window.body_key, BODY_KEY);
    assert_eq!(window.surface_kind, SurfaceKind::Board2d);
    assert_eq!(window.icon_id, semio_framework_plugin::IconName::LayoutGrid);
}

#[test]
fn every_bundled_example_renders_one_board_node_per_cell() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses");
        let nodes = fixture(&document)["nodes"].as_array().expect("nodes array").len();
        assert_eq!(nodes, (document.width as usize) * (document.height as usize), "{}", source.id());
    }
}

#[test]
fn a_cell_states_whether_it_is_open_pinned_or_masked() {
    let document = crate::examples::grid2d::pipes::document();
    assert_eq!(cell_kind(&document, 0, 0), "pinned");
    assert_eq!(cell_kind(&document, 5, 5), "masked");
    assert_eq!(cell_kind(&document, 3, 3), "open");
    let catalogs: Value = serde_json::from_str(&glyph_catalogs_json()).expect("catalogue is real json");
    let kinds: Vec<&str> = catalogs["nodeKinds"].as_array().expect("rows").iter().filter_map(|row| row["id"].as_str()).collect();
    assert_eq!(kinds, ["open", "pinned", "masked"]);
}

#[test]
fn a_pinned_cell_carries_its_tile_id_and_a_masked_cell_does_not() {
    let document = crate::examples::grid2d::pipes::document();
    let nodes = fixture(&document);
    let node = |id: &str| nodes["nodes"].as_array().expect("nodes").iter().find(|node| node["id"] == id).expect("node").clone();
    assert_eq!(node("cell-0-0")["text"], "elbow-ne");
    assert_eq!(node("cell-5-5")["text"], "");
}

#[test]
fn the_board_snaps_to_the_authored_cell_size() {
    let document = crate::examples::grid2d::pipes::document();
    let armed = scene(&document, &Grid2dWindowConfig::default(), UTILITY_PIN);
    assert!(armed.grid_snap_enabled);
    assert_eq!(armed.grid_factor, Grid2dWindowConfig::default().grid_factor);
    let sized = scene(&document, &Grid2dWindowConfig { grid_factor: 0.0, ..Default::default() }, UTILITY_PIN);
    assert_eq!(sized.grid_factor, document.cell_width, "an unset factor falls back to the authored cell width");
    assert!(armed.interactive, "the grid pane is the one that accepts pointer input");
    assert_eq!(armed.active_utility.as_deref(), Some(UTILITY_PIN));
}

#[test]
fn the_rendered_surface_is_non_empty_for_every_example() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses");
        render(&document, &Grid2dWindowConfig::default(), UTILITY_SELECT).unwrap_or_else(|error| panic!("{}: the grid pane must render: {error:?}", source.id()));
        let published = scene(&document, &Grid2dWindowConfig::default(), UTILITY_SELECT);
        assert!(published.fixture_json.len() > 64, "{}: the board carries no cells", source.id());
        assert!(published.glyph_catalogs_json.contains("pinned"), "{}: the board carries no cell-state catalogue", source.id());
    }
}
