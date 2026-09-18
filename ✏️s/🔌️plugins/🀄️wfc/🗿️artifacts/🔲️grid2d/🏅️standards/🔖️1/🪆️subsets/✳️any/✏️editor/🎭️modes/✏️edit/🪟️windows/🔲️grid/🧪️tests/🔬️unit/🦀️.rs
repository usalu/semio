//! 🧪️ The grid pane renders a real, non-empty canvas for every bundled example, one cell layer per
//! cell at the authored cell size, and inverts a pointer sample back onto exactly that cell.

use super::*;
use crate::editor::grid2d::window::Grid2dWindowConfig;
use serde_json::Value;

fn layers(document: &Grid2dSnapshot) -> Vec<Value> {
    serde_json::from_str(&layers_json(document)).expect("the layer list is real json")
}

#[test]
fn the_window_declares_a_canvas_surface_with_a_registered_icon() {
    let window = definition();
    assert_eq!(window.id, WINDOW_KIND_ID);
    assert_eq!(window.body_key, BODY_KEY);
    assert_eq!(window.surface_kind, SurfaceKind::Canvas2d, "Board2d needs a per-app wasm board session no wfc app ships");
    assert_eq!(window.icon_id, semio_framework_plugin::IconName::LayoutGrid);
}

/// 🖱️ Every verb `Canvas2dHost` dispatches on its own is declared, or the shell drops it with
/// `refused: undeclared-action` and every hover becomes a console fault.
#[test]
fn the_window_declares_every_canvas_verb_the_host_dispatches() {
    let window = definition();
    for action in CANVAS_ACTIONS {
        assert!(window.actions.iter().any(|declared| declared.id == action), "{action} is not declared");
    }
    assert!(window.actions.iter().any(|declared| declared.id == "setActiveExample"), "the navbar example switcher has no declaring window");
}

/// 🪪️ The React host renames a scene node to `window:<window-kind-id>`, so both spellings of this
/// pane's surface must be recognised — and the sibling preview pane must never be.
#[test]
fn this_pane_owns_both_spellings_of_its_surface() {
    assert!(owns_surface(SURFACE_ID));
    assert!(owns_surface(&format!("window:{WINDOW_KIND_ID}")));
    assert!(!owns_surface("window:wfc-grid2d-preview"));
    assert!(!owns_surface(super::super::preview::SURFACE_ID));
}

#[test]
fn every_bundled_example_renders_one_cell_layer_per_cell() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses");
        assert_eq!(layers(&document).len(), (document.width as usize) * (document.height as usize), "{}", source.id());
    }
}

#[test]
fn a_cell_states_whether_it_is_open_pinned_or_masked() {
    let document = crate::examples::grid2d::pipes::document();
    assert_eq!(cell_kind(&document, 0, 0), "pinned");
    assert_eq!(cell_kind(&document, 5, 5), "masked");
    assert_eq!(cell_kind(&document, 3, 3), "open");
    assert_ne!(cell_color("pinned"), cell_color("masked"));
    assert_ne!(cell_color("open"), cell_color("pinned"));
}

#[test]
fn a_pinned_cell_carries_its_tile_id_and_a_masked_cell_does_not() {
    let document = crate::examples::grid2d::pipes::document();
    let rendered = layers(&document);
    let layer = |id: &str| rendered.iter().find(|layer| layer["id"] == id).expect("layer").clone();
    assert_eq!(layer("cell-0-0")["name"], "elbow-ne");
    assert_eq!(layer("cell-0-0")["selected"], Value::Bool(true));
    assert_eq!(layer("cell-5-5")["name"], "");
    assert_eq!(layer("cell-5-5")["color"], cell_color("masked"));
}

/// 🎥️ An untouched camera centres on the grid, so the pane's own centre pixel is the grid's centre
/// cell rather than its top-left corner.
#[test]
fn a_pointer_sample_inverts_onto_exactly_one_cell() {
    let document = crate::examples::grid2d::pipes::document();
    let config = Grid2dWindowConfig::default();
    let (viewport_w, viewport_h) = (800.0, 600.0);
    let centre = cell_at(&document, &config, viewport_w * 0.5, viewport_h * 0.5, viewport_w, viewport_h).expect("the pane centre is inside the grid");
    assert_eq!(centre, (document.width / 2, document.height / 2));
    let half_w = f64::from(document.width) * document.cell_width * 0.5;
    let half_h = f64::from(document.height) * document.cell_height * 0.5;
    let origin = cell_at(&document, &config, viewport_w * 0.5 - half_w + 1.0, viewport_h * 0.5 - half_h + 1.0, viewport_w, viewport_h);
    assert_eq!(origin, Some((0, 0)));
    assert_eq!(cell_at(&document, &config, 0.0, 0.0, viewport_w, viewport_h), None, "a sample outside the grid picks nothing rather than clamping");
    assert_eq!(cell_at(&document, &config, viewport_w * 0.5, viewport_h * 0.5, 0.0, 0.0), None, "an unmeasured viewport picks nothing");
}

#[test]
fn the_scene_camera_is_the_one_the_pick_inverts() {
    let document = crate::examples::grid2d::pipes::document();
    let published = scene(&document, &Grid2dWindowConfig::default(), UTILITY_PIN);
    let (camera_x, camera_y, zoom) = effective_camera(&document, &Grid2dWindowConfig::default());
    assert_eq!((published.camera_x, published.camera_y, published.zoom), (camera_x, camera_y, zoom));
    let panned = Grid2dWindowConfig { camera_x: 12.0, camera_y: 34.0, camera_zoom: 2.0, ..Default::default() };
    let moved = scene(&document, &panned, UTILITY_PIN);
    assert_eq!((moved.camera_x, moved.camera_y, moved.zoom), (12.0, 34.0, 2.0), "a written camera is honoured verbatim");
}

#[test]
fn the_rendered_surface_is_non_empty_for_every_example() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("example parses");
        render(&document, &Grid2dWindowConfig::default(), UTILITY_SELECT).unwrap_or_else(|error| panic!("{}: the grid pane must render: {error:?}", source.id()));
        let published = scene(&document, &Grid2dWindowConfig::default(), UTILITY_SELECT);
        assert!(published.layers_json.len() > 64, "{}: the canvas carries no cells", source.id());
        assert!(published.layers_json.contains("cell-0-0"), "{}: the canvas carries no addressed cell", source.id());
    }
}
