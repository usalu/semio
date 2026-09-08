
use super::*;
use crate::wfc_engine::grid2d::Boundary;
use crate::wfc_engine::grid3d::{Stencil3d, declare_stencil_relations_3d_tiled};
use crate::wfc_engine::tiled::TiledModelBuilder;

fn checkerboard3d(size: usize, boundary: Boundary) -> (CompiledModel, Grid3dTopology) {
    let mut b = TiledModelBuilder::new();
    let black = b.tile(1.0);
    let white = b.tile(1.0);
    let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
    for &r in &rels {
        b.allow_mirrored(r, black, white);
    }
    let model = b.compile().unwrap();
    let topo = Grid3dTopology::new(size, size, size, &Stencil3d::Face6, rels, boundary, boundary, boundary, None).unwrap();
    (model, topo)
}

#[test]
fn solves_a_checkerboard_volume() {
    let (model, topo) = checkerboard3d(4, Boundary::Open);
    let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
    let outcome = solver.solve(1);
    assert!(matches!(outcome, SolveOutcome::Solved(_)));
}

#[test]
fn fix_pins_a_voxel_and_propagates() {
    let (model, topo) = checkerboard3d(3, Boundary::Open);
    let black = PatternId(0);
    let white = PatternId(1);
    let mut solver = Grid3dSolverBuilder::new(model, topo).fix(0, 0, 0, black).unwrap().build().unwrap();
    match solver.solve(1) {
        SolveOutcome::Solved(sol) => {
            assert_eq!(solver.get(&sol, 0, 0, 0), Some(black));
            assert_eq!(solver.get(&sol, 1, 0, 0), Some(white));
            assert_eq!(solver.get(&sol, 0, 0, 1), Some(white));
        }
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn masked_voxels_are_excluded_and_solve_completes() {
    let mut b = TiledModelBuilder::new();
    let black = b.tile(1.0);
    let white = b.tile(1.0);
    let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
    for &r in &rels {
        b.allow_mirrored(r, black, white);
    }
    let model = b.compile().unwrap();
    let mut mask = vec![true; 27];
    mask[13] = false;
    let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
    let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
    assert!(matches!(solver.solve(1), SolveOutcome::Solved(_)));
}

#[test]
fn wrap_boundary_solves_consistently() {
    let (model, topo) = checkerboard3d(4, Boundary::Wrap);
    let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
    assert!(matches!(solver.solve(1), SolveOutcome::Solved(_)));
}

#[test]
fn odd_size_wrap_is_unsatisfiable_for_two_color_checkerboard() {
    let (model, topo) = checkerboard3d(3, Boundary::Wrap);
    let mut solver = Grid3dSolverBuilder::new(model, topo).config(SearchConfig { mode: search::SearchMode::Backtrack, ..Default::default() }).build().unwrap();
    assert!(matches!(solver.solve(1), SolveOutcome::Unsatisfiable(_)));
}

#[test]
fn graph_vs_grid3d_strict_equivalence_face6_open() {
    // Independently hand-enumerated arcs for a 2x2x3 Face6/Open grid, fed into a
    // GraphTopology, compared against the same model solved through Grid3dTopology.
    let width = 2usize;
    let height = 2usize;
    let depth = 3usize;
    let mut b = TiledModelBuilder::new();
    let tiles: Vec<_> = (0..3).map(|i| b.tile(1.0 + i as f64)).collect();
    let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
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

    let idx = |x: usize, y: usize, z: usize| crate::wfc_engine::ids::NodeId::from_index(z * width * height + y * width + x);
    let mut hand_arcs = Vec::new();
    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                if x + 1 < width {
                    hand_arcs.push((idx(x, y, z), idx(x + 1, y, z), rels[0]));
                    hand_arcs.push((idx(x + 1, y, z), idx(x, y, z), rels[1]));
                }
                if y + 1 < height {
                    hand_arcs.push((idx(x, y, z), idx(x, y + 1, z), rels[2]));
                    hand_arcs.push((idx(x, y + 1, z), idx(x, y, z), rels[3]));
                }
                if z + 1 < depth {
                    hand_arcs.push((idx(x, y, z), idx(x, y, z + 1), rels[4]));
                    hand_arcs.push((idx(x, y, z + 1), idx(x, y, z), rels[5]));
                }
            }
        }
    }
    let mut gb = crate::wfc_engine::topology::GraphTopologyBuilder::new(width * height * depth);
    for (from, to, r) in hand_arcs {
        gb.arc(from, to, r);
    }
    let graph_topo = gb.build().unwrap();
    let grid_topo = Grid3dTopology::new(width, height, depth, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();

    let config = SearchConfig::default();
    for seed in 0..10u64 {
        let mut graph_solver = crate::wfc_engine::solver_graph::GraphSolverBuilder::new(model.clone(), graph_topo.clone()).config(config).build().unwrap();
        let mut grid_solver = Grid3dSolverBuilder::new(model.clone(), grid_topo.clone()).config(config).build().unwrap();
        let graph_outcome = graph_solver.solve(seed);
        let grid_outcome = grid_solver.solve(seed);
        match (graph_outcome, grid_outcome) {
            (SolveOutcome::Solved(g), SolveOutcome::Solved(r)) => {
                assert_eq!(g.assignment, r.assignment, "seed {seed}: graph and grid3d solutions diverged");
                assert_eq!(g.report.metrics.observations, r.report.metrics.observations, "seed {seed}: observation counts diverged");
            }
            (a, b) => panic!("seed {seed}: outcome mismatch, graph={a:?} grid={b:?}"),
        }
    }
}
