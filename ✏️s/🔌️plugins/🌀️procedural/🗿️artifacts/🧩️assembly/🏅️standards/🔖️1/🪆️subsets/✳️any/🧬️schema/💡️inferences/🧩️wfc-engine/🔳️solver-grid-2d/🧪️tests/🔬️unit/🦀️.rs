
use super::*;
use crate::wfc_engine::grid2d::{Boundary, Stencil2d, declare_stencil_relations_tiled};
use crate::wfc_engine::tiled::TiledModelBuilder;

fn checkerboard(width: usize, height: usize, boundary: Boundary) -> (CompiledModel, Grid2dTopology) {
    let mut b = TiledModelBuilder::new();
    let black = b.tile(1.0);
    let white = b.tile(1.0);
    let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
    for &r in &rels {
        b.allow_mirrored(r, black, white);
    }
    let model = b.compile().unwrap();
    let topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, boundary, boundary, None).unwrap();
    (model, topo)
}

#[test]
fn solves_a_checkerboard_grid() {
    let (model, topo) = checkerboard(5, 5, Boundary::Open);
    let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
    let outcome = solver.solve(1);
    assert!(matches!(outcome, SolveOutcome::Solved(_)));
}

#[test]
fn fix_pins_a_cell_and_propagates() {
    let (model, topo) = checkerboard(4, 4, Boundary::Open);
    let black = PatternId(0);
    let white = PatternId(1);
    let mut solver = Grid2dSolverBuilder::new(model, topo).fix(0, 0, black).unwrap().build().unwrap();
    match solver.solve(1) {
        SolveOutcome::Solved(sol) => {
            assert_eq!(solver.get(&sol, 0, 0), Some(black));
            assert_eq!(solver.get(&sol, 1, 0), Some(white));
            assert_eq!(solver.get(&sol, 0, 1), Some(white));
        }
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn solve_chunk_respects_seam_pins_and_reproduces_deterministically() {
    let (model, topo) = checkerboard(5, 5, Boundary::Open);
    let seam_node = topo.node_at(0, 0).unwrap();
    let solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();

    let white = PatternId(1);
    let a = solver.solve_chunk(42, 3, -2, &[(seam_node, white)]);
    let b = solver.solve_chunk(42, 3, -2, &[(seam_node, white)]);
    match (a, b) {
        (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => {
            assert_eq!(sa.assignment, sb.assignment);
            assert_eq!(solver.get(&sa, 0, 0), Some(white));
        }
        other => panic!("expected both calls to solve identically, got {other:?}"),
    }
}

#[test]
fn masked_cells_are_excluded_and_solve_completes() {
    let mut b = TiledModelBuilder::new();
    let black = b.tile(1.0);
    let white = b.tile(1.0);
    let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
    for &r in &rels {
        b.allow_mirrored(r, black, white);
    }
    let model = b.compile().unwrap();
    let mut mask = vec![true; 9];
    mask[4] = false;
    let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
    let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
    let outcome = solver.solve(1);
    assert!(matches!(outcome, SolveOutcome::Solved(_)));
}

#[test]
fn wrap_boundary_solves_consistently() {
    let (model, topo) = checkerboard(4, 4, Boundary::Wrap);
    let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
    let outcome = solver.solve(1);
    assert!(matches!(outcome, SolveOutcome::Solved(_)));
}

#[test]
fn odd_size_wrap_is_unsatisfiable_for_two_color_checkerboard() {
    // A 3x3 wrapped grid forces an odd cycle along each axis; two colors can't 2-color it.
    let (model, topo) = checkerboard(3, 3, Boundary::Wrap);
    let mut solver = Grid2dSolverBuilder::new(model, topo).config(SearchConfig { mode: search::SearchMode::Backtrack, ..Default::default() }).build().unwrap();
    let outcome = solver.solve(1);
    assert!(matches!(outcome, SolveOutcome::Unsatisfiable(_)));
}

#[test]
fn graph_vs_grid2d_strict_equivalence_von_neumann_open() {
    // Independently hand-enumerated arcs for a 3x4 VonNeumann/Open grid (not derived from
    // 🦀️grid2d.rs's own resolve_coord logic) fed into a GraphTopology, compared against the
    // same model solved through Grid2dTopology: both must produce byte-identical assignments
    // and identical observation counts under the same seed/config.
    let width = 3usize;
    let height = 4usize;
    let mut b = TiledModelBuilder::new();
    let tiles: Vec<_> = (0..3).map(|i| b.tile(1.0 + i as f64)).collect();
    let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
    for &r in &rels {
        for &a in &tiles {
            for &c in &tiles {
                if a != c {
                    b.allow(r, a, c);
                }
            }
        }
    }
    let model = b.compile().unwrap();

    let mut hand_arcs = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let idx = |x: usize, y: usize| crate::wfc_engine::ids::NodeId::from_index(y * width + x);
            if x + 1 < width {
                hand_arcs.push((idx(x, y), idx(x + 1, y), rels[0])); // east: offset (1,0)
                hand_arcs.push((idx(x + 1, y), idx(x, y), rels[1])); // west: offset (-1,0)
            }
            if y + 1 < height {
                hand_arcs.push((idx(x, y), idx(x, y + 1), rels[2])); // south: offset (0,1)
                hand_arcs.push((idx(x, y + 1), idx(x, y), rels[3])); // north: offset (0,-1)
            }
        }
    }
    let mut gb = crate::wfc_engine::topology::GraphTopologyBuilder::new(width * height);
    for (from, to, r) in hand_arcs {
        gb.arc(from, to, r);
    }
    let graph_topo = gb.build().unwrap();

    let grid_topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();

    let config = SearchConfig::default();
    for seed in 0..10u64 {
        let mut graph_solver = crate::wfc_engine::solver_graph::GraphSolverBuilder::new(model.clone(), graph_topo.clone()).config(config).build().unwrap();
        let mut grid_solver = Grid2dSolverBuilder::new(model.clone(), grid_topo.clone()).config(config).build().unwrap();
        let graph_outcome = graph_solver.solve(seed);
        let grid_outcome = grid_solver.solve(seed);
        match (graph_outcome, grid_outcome) {
            (SolveOutcome::Solved(g), SolveOutcome::Solved(r)) => {
                assert_eq!(g.assignment, r.assignment, "seed {seed}: graph and grid2d solutions diverged");
                assert_eq!(g.report.metrics.observations, r.report.metrics.observations, "seed {seed}: observation counts diverged");
            }
            (a, b) => panic!("seed {seed}: outcome mismatch, graph={a:?} grid={b:?}"),
        }
    }
}

#[test]
fn decode_tiles_round_trips_tile_provenance() {
    let (model, topo) = checkerboard(2, 2, Boundary::Open);
    let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
    match solver.solve(1) {
        SolveOutcome::Solved(sol) => {
            let tiles = solver.decode_tiles(&sol);
            assert_eq!(tiles.len(), 4);
            assert!(tiles.iter().all(|t| t.is_some()));
        }
        other => panic!("expected Solved, got {other:?}"),
    }
}
