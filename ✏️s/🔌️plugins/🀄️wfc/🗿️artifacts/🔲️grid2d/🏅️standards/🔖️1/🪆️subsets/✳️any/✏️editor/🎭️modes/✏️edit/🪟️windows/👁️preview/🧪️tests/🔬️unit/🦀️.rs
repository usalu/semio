//! 🧪️ The preview pane draws the SOLVED assignment when one is cached and the bare cell grid when
//! it is not — and never reads the assignment from the document, which does not hold one.

use super::*;
use crate::editor::grid2d::window::Grid2dWindowConfig;
use crate::schema::inferences::solve_with_job;

fn layers(document: &Grid2dSnapshot, config: &Grid2dWindowConfig) -> Vec<Value> {
    serde_json::from_str(&layers_json(document, cached_commit(document, config).as_ref())).expect("layers are real json")
}

#[test]
fn the_window_declares_a_canvas_surface_with_a_registered_icon() {
    let window = definition();
    assert_eq!(window.id, WINDOW_KIND_ID);
    assert_eq!(window.surface_kind, SurfaceKind::Canvas2d);
    assert_eq!(window.icon_id, semio_framework_plugin::IconName::Preview);
    assert!(window.utilities.is_empty(), "the preview pane accepts no pointer utility");
}

#[test]
fn an_unsolved_pane_draws_one_outline_per_unmasked_cell() {
    let document = crate::examples::grid2d::pipes::document();
    let rendered = layers(&document, &Grid2dWindowConfig::default());
    assert_eq!(rendered.len(), 35, "the 6×6 grid minus its one masked cell");
    assert!(rendered.iter().all(|layer| layer.get("width").is_some() && layer.get("height").is_some()), "every bounds layer needs explicit extents or it silently draws nothing");
}

#[test]
fn a_solved_vector_pane_draws_one_path_layer_per_connector() {
    let document = crate::examples::grid2d::pipes::document();
    let commit = solve_with_job(&document).expect("the example solves");
    let config = Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&commit), ..Default::default() };
    let rendered = layers(&document, &config);
    assert!(!rendered.is_empty());
    assert!(rendered.iter().any(|layer| layer.get("segments").is_some()), "a vector tile must reach the canvas as a real path");
    for layer in &rendered {
        if let Some(transform) = layer.get("transform") {
            assert_eq!(transform.as_array().expect("transform is a 6-tuple").len(), 6);
        }
    }
}

/// 🧱️ A solved bitmap board stays inside the pane's whole-surface budget. Drawing all 64 cells at
/// 4×4 pixels each encoded to 87 337 bytes and the host refused the WHOLE surface
/// (`ui.fixed-capacity`) — the solve ran and the pane stayed blank — so a board over budget draws
/// one averaged swatch per cell instead.
#[test]
fn a_solved_bitmap_pane_stays_inside_the_surface_budget() {
    let document = crate::examples::grid2d::terrain::document();
    let commit = solve_with_job(&document).expect("the example solves");
    let config = Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&commit), ..Default::default() };
    let rendered = layers(&document, &config);
    assert_eq!(rendered.len(), 64, "64 assigned cells, one swatch each — the 1 024-rect form overflows the surface");
    assert!(rendered.iter().all(|layer| layer["color"].as_str().is_some_and(|color| color.starts_with('#'))), "a bounds layer's colour must be a css hex or the host falls back to a hue ramp");
    let distinct: std::collections::BTreeSet<&str> = rendered.iter().filter_map(|layer| layer["color"].as_str()).collect();
    assert!(distinct.len() > 1, "a solved board is not one flat colour: {distinct:?}");
    assert!(layers_json(&document, Some(&commit)).len() < 65_536, "the published surface must fit the host's fixed capacity");
}

/// 🎨️ A small board still draws real pixels: the per-tile ceiling only degrades once the BOARD
/// budget is the binding one.
#[test]
fn a_small_bitmap_board_still_draws_one_rect_per_pixel() {
    let mut document = crate::examples::grid2d::terrain::document();
    document.width = 2;
    document.height = 2;
    document.pinned.retain(|cell| cell.x < 2 && cell.y < 2);
    document.masked.retain(|cell| cell.x < 2 && cell.y < 2);
    let commit = solve_with_job(&document).expect("the trimmed example solves");
    let config = Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&commit), ..Default::default() };
    assert_eq!(layers(&document, &config).len(), 4 * 16, "4 cells × a 4×4 bitmap each");
}

#[test]
fn a_cached_commit_that_contradicts_falls_back_to_the_bare_grid() {
    let document = crate::examples::grid2d::pipes::document();
    let contradicted = Grid2dInferenceCommit { assignments: Vec::new(), contradiction: true, entropy: Vec::new() };
    let config = Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&contradicted), ..Default::default() };
    assert_eq!(layers(&document, &config).len(), 35);
}

#[test]
fn a_malformed_cache_is_ignored_rather_than_failing_the_render() {
    let document = crate::examples::grid2d::pipes::document();
    let config = Grid2dWindowConfig { solve_json: "not json".into(), ..Default::default() };
    assert!(cached_commit(&document, &config).is_none());
    render(&document, &config, None).expect("the pane still renders");
    assert!(layers(&document, &config).len() > 1, "a malformed cache falls back to the bare grid, not to nothing");
}

/// 🧊️ The cache lives in WINDOW CONFIG and the document can be replaced under it (the navbar
/// example switcher), so a commit that does not belong to the document in front of the pane must be
/// dropped — otherwise the pipes assignment paints over the terrain grid as "unknown tile" boxes.
#[test]
fn a_cache_from_another_document_is_dropped_rather_than_painted() {
    let pipes = crate::examples::grid2d::pipes::document();
    let terrain = crate::examples::grid2d::terrain::document();
    let solved = solve_with_job(&pipes).expect("pipes solves");
    let config = Grid2dWindowConfig { solve_json: protocol::json::to_json_string(&solved), ..Default::default() };
    assert!(cached_commit(&pipes, &config).is_some(), "its own document keeps the cache");
    assert!(cached_commit(&terrain, &config).is_none(), "a foreign document drops it");
    let painted = layers(&terrain, &config);
    assert!(painted.iter().all(|layer| layer["name"].is_null()), "the fallback grid carries no tile labels: {painted:?}");
}

#[test]
fn the_rendered_surface_is_non_empty_for_every_example() {
    for source in crate::examples::grid2d::sources() {
        let document = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&source.document()).expect("example parses");
        render(&document, &Grid2dWindowConfig::default(), None).unwrap_or_else(|error| panic!("{}: the preview pane must render: {error:?}", source.id()));
        assert!(scene(&document, &Grid2dWindowConfig::default(), None).layers_json.len() > 64, "{}: the canvas carries no layers", source.id());
    }
}

#[test]
fn a_partial_fill_preview_differs_from_empty_and_finished() {
    let document = crate::examples::grid2d::pipes::document();
    let oracle = solve_with_job(&document).expect("pipes solves");
    let empty = layers_json(&document, None);
    let finished = layers_json(&document, Some(&oracle));
    let mut partial = oracle.clone();
    partial.assignments.truncate((oracle.assignments.len() / 2).max(1));
    assert!(partial.assignments.len() < oracle.assignments.len());
    let mid = layers_json(&document, Some(&partial));
    assert_ne!(mid, empty, "a partial board must not look empty");
    assert_ne!(mid, finished, "a partial board must not look finished");
}
