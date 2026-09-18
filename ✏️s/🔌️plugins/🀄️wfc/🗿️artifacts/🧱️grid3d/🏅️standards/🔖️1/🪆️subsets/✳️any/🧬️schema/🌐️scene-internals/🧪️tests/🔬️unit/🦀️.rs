//! 🔬️ Scene projection laws — one instance per cell, one mesh per distinct look, and a cell's world
//! box is exactly the non-uniform offsets its own document declares.

use super::*;
use crate::schema::inferences::Grid3dAssignment;

fn document() -> Grid3dSnapshot {
    crate::examples::blocks::snapshot()
}

fn row(x: u32, y: u32, z: u32, tile: &str) -> Grid3dAssignment {
    Grid3dAssignment { x, y, z, tile_id: tile.to_string() }
}

fn array_len(json_text: &str) -> usize {
    json::parse(json_text).expect("payload is json").as_array().expect("payload is an array").len()
}

#[test]
fn a_cell_sits_at_the_cumulative_offsets_of_its_own_axes() {
    let snapshot = document();
    assert_eq!(cell_origin(&snapshot, 0, 0, 0), [0.0, 0.0, 0.0]);
    assert_eq!(cell_origin(&snapshot, 2, 1, 3), [3.0, 3.0, 3.0]);
    assert_eq!(cell_extent(&snapshot, 2, 1, 3), [2.0, 1.5, 2.0]);
    assert_eq!(grid_extent(&snapshot), [6.0, 6.0, 5.0]);
}

#[test]
fn the_grid_window_publishes_exactly_one_instance_per_cell() {
    let snapshot = document();
    let cells = (snapshot.width * snapshot.height * snapshot.depth) as usize;
    assert_eq!(array_len(&grid_instances_json(&snapshot)), cells);
}

#[test]
fn the_grid_catalogue_holds_one_mesh_per_distinct_look_never_one_per_cell() {
    let snapshot = document();
    let meshes = array_len(&grid_meshes_json(&snapshot));
    assert_eq!(meshes, 3, "neutral cage, masked cage, and one tinted cage for the single pinned tile");
    assert!(meshes < (snapshot.width * snapshot.height * snapshot.depth) as usize);
}

#[test]
fn the_preview_catalogue_holds_exactly_one_mesh_per_tile() {
    let snapshot = document();
    assert_eq!(array_len(&preview_meshes_json(&snapshot)), snapshot.tiles.len());
}

#[test]
fn a_preview_instance_is_emitted_only_for_a_cell_inside_the_grid() {
    let snapshot = document();
    let assignments = vec![row(0, 0, 0, "floor"), row(99, 0, 0, "wall")];
    assert_eq!(array_len(&preview_instances_json(&snapshot, &assignments)), 1);
}

#[test]
fn the_delta_names_only_what_moved_and_keeps_the_full_set_authoritative() {
    let snapshot = document();
    let previous = vec![row(0, 0, 0, "floor"), row(1, 0, 0, "wall")];
    let next = vec![row(0, 0, 0, "floor"), row(1, 0, 0, "roof")];
    let delta = json::parse(&preview_instances_delta_json(&snapshot, &previous, &next, 4)).expect("delta is json");
    assert_eq!(delta.get("base").and_then(json::Value::as_u64), Some(3));
    assert_eq!(delta.get("revision").and_then(json::Value::as_u64), Some(4));
    assert_eq!(delta.get("count").and_then(json::Value::as_u64), Some(2));
    assert_eq!(delta.get("changed").and_then(|value| value.as_array()).map(Vec::len), Some(1));
    assert_eq!(delta.get("removed").and_then(|value| value.as_array()).map(Vec::len), Some(0));
}

#[test]
fn a_mesh_child_tile_falls_back_to_a_placeholder_instead_of_failing_the_surface() {
    let tile = Grid3dTile { id: "child".into(), label: None, weight: 1.0, media: Grid3dTileMedia::MeshChild { child: crate::mesh_child_handle("mesh-a") } };
    let mesh = tile_mesh(&tile);
    assert_eq!(mesh.positions.len(), 24, "the placeholder is the unit box");
    assert_eq!(mesh.indices.len(), 36);
}
