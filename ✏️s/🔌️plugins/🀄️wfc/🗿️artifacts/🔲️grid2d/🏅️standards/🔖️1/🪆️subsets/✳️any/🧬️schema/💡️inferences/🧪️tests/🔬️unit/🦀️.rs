//! 🧪️ `s.wfc.grid2d.solve` laws — determinism under a fixed seed, the unsatisfiable verdict, the
//! masked-cell omission and the entropy map's own shape.

use super::*;
use crate::schema::snapshot::{WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d};

fn tile(id: &str, weight: f64) -> WfcTile2d {
    WfcTile2d { id: id.into(), label: None, weight, media: WfcTileMedia2d::default() }
}

/// 🧩️ Two tiles that may sit beside each other in either canonical direction — always solvable.
fn permissive(width: u32, height: u32) -> Grid2dSnapshot {
    let mut rules = Vec::new();
    for a in ["ink", "void"] {
        for b in ["ink", "void"] {
            for (direction, slug) in [(WfcDirection2d::Right, "right"), (WfcDirection2d::Bottom, "bottom")] {
                rules.push(WfcAdjacencyRule2d { id: format!("rule-{slug}-{a}-{b}"), tile_a_id: a.into(), tile_b_id: b.into(), direction, allowed: true });
            }
        }
    }
    rules.sort_by(|left, right| left.id.cmp(&right.id));
    Grid2dSnapshot { seed: 7, width, height, tiles: vec![tile("ink", 1.0), tile("void", 3.0)], rules, ..Default::default() }
}

#[test]
fn the_same_seed_solves_to_the_same_assignment() {
    let document = permissive(4, 3);
    let first = solve_with_job(&document).expect("solve completes");
    let second = solve_with_job(&document).expect("solve repeats");
    assert!(!first.contradiction, "a permissive rule set is satisfiable");
    assert_eq!(first.assignments.len(), 12, "every unmasked cell is assigned");
    assert_eq!(first.assignments, second.assignments, "the solve must be a pure function of the document, seed included");
}

#[test]
fn a_different_seed_is_still_a_valid_solve() {
    let mut document = permissive(4, 3);
    document.seed = 4_242;
    let commit = solve_with_job(&document).expect("solve completes");
    assert!(!commit.contradiction);
    assert!(commit.assignments.iter().all(|(_, _, tile)| tile == "ink" || tile == "void"));
}

#[test]
fn an_empty_rule_set_contradicts_the_moment_two_cells_are_adjacent() {
    let mut document = permissive(3, 1);
    document.rules.clear();
    let commit = solve_with_job(&document).expect("solve completes");
    assert!(commit.contradiction, "with no rule authored every pair is forbidden, so a 3×1 grid is unsatisfiable");
    assert!(commit.assignments.is_empty());
}

#[test]
fn a_document_with_no_tiles_contradicts_without_reaching_the_solver() {
    let document = Grid2dSnapshot { width: 2, height: 2, ..Default::default() };
    let commit = solve_with_job(&document).expect("solve completes");
    assert!(commit.contradiction);
}

#[test]
fn a_masked_cell_is_omitted_from_the_assignment() {
    let mut document = permissive(3, 2);
    document.masked = vec![WfcCell2d { x: 2, y: 1 }];
    let commit = solve_with_job(&document).expect("solve completes");
    assert_eq!(commit.assignments.len(), 5, "the masked cell carries no assignment");
    assert!(commit.assignments.iter().all(|(x, y, _)| !(*x == 2 && *y == 1)));
}

#[test]
fn a_pinned_cell_keeps_the_tile_it_was_pinned_to() {
    let mut document = permissive(3, 2);
    document.pinned = vec![WfcPinnedCell2d { x: 1, y: 1, tile_id: "ink".into() }];
    let commit = solve_with_job(&document).expect("solve completes");
    let pinned = commit.assignments.iter().find(|(x, y, _)| *x == 1 && *y == 1).expect("the pinned cell is assigned");
    assert_eq!(pinned.2, "ink");
}

#[test]
fn the_entropy_map_states_one_row_per_cell_and_zero_where_the_cell_is_determined() {
    let mut document = permissive(3, 2);
    document.pinned = vec![WfcPinnedCell2d { x: 0, y: 0, tile_id: "ink".into() }];
    document.masked = vec![WfcCell2d { x: 2, y: 1 }];
    let commit = solve_with_job(&document).expect("solve completes");
    assert_eq!(commit.entropy.len(), 6);
    let value_at = |x: u32, y: u32| commit.entropy.iter().find(|(ex, ey, _)| *ex == x && *ey == y).map(|(_, _, value)| *value).expect("cell present");
    assert_eq!(value_at(0, 0), 0.0, "a pinned cell is fully determined");
    assert_eq!(value_at(2, 1), 0.0, "a masked cell is outside the problem");
    assert!(value_at(1, 1) > 0.0, "an open cell carries the prior entropy of the tile distribution");
}

#[test]
fn the_prior_entropy_is_the_shannon_entropy_of_the_weight_distribution() {
    let document = permissive(2, 2);
    let total = 1.0 + 3.0;
    let expected = -((1.0 / total) * (1.0f64 / total).ln() + (3.0 / total) * (3.0f64 / total).ln());
    assert!((shannon_entropy_over_tiles(&document) - expected).abs() < 1e-12);
}

#[test]
fn the_inferred_fields_agree_with_the_job_they_both_drive() {
    let document = permissive(3, 2);
    let solved = <Grid2dSolve as store::InferredField<Grid2dSnapshot>>::compute(&document, &"grid2d".to_string(), &[]);
    match solved {
        Grid2dSolveResult::Solved { assignments } => assert_eq!(assignments.len(), 6),
        Grid2dSolveResult::Unsolved => panic!("a permissive grid must solve"),
    }
    assert!(!<Grid2dContradiction as store::InferredField<Grid2dSnapshot>>::compute(&document, &"grid2d".to_string(), &[]));
    assert_eq!(<Grid2dEntropy as store::InferredField<Grid2dSnapshot>>::plan(&document).len(), 6);
}

#[test]
fn the_routed_metadata_names_this_artifact_and_its_own_tool_id() {
    let metadata = grid2d_inference_metadata();
    assert_eq!(metadata.owner, "wfc");
    assert_eq!(metadata.artifact_kind, "s.wfc.grid2d");
    assert_eq!(metadata.inference_schema, GRID2D_INFERENCE_TOOL_ID);
    assert_eq!(GRID2D_INFERENCE_JOB_KIND, "semio.infer");
}

#[test]
fn the_inference_descriptor_carries_five_real_leaves() {
    let descriptor = grid2d_artifact_inference_descriptor();
    assert_eq!(descriptor.id, "s.wfc.grid2d.solve");
    for leaf in [descriptor.inference.rust, descriptor.inference.typescript, descriptor.inference.graphql, descriptor.inference.json_schema, descriptor.inference.proto] {
        assert!(!leaf.trim().is_empty());
    }
}
