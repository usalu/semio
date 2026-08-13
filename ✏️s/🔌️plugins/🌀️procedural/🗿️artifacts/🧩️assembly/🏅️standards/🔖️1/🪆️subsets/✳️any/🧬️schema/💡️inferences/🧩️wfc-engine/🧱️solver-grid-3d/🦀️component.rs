//! 🧊️ `Grid3dSolver`: dense 3D grid solving. Exactly [`crate::wfc_engine::solver_grid2d`]'s design extended to
//! a third axis — masked-out voxels and [`crate::wfc_engine::grid2d::Boundary::FixedOutside`] faces fold into
//! ordinary domain overrides and fixed pins before delegating to the same generic kernel.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::{build_adjacency_view, AdjacencyView, Constraint, ConstraintSet};
use crate::wfc_engine::error::SolveError;
use crate::wfc_engine::grid3d::Grid3dTopology;
use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::{Solution, SolveOutcome};
use crate::wfc_engine::search::{self, CancelToken, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Builder
/// 🏗️ Builds a [`Grid3dSolver`] over a dense `width × height × depth` grid.
pub struct Grid3dSolverBuilder {
    model: CompiledModel,
    topology: Grid3dTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(crate::wfc_engine::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
}

impl Grid3dSolverBuilder {
    pub fn new(model: CompiledModel, topology: Grid3dTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    pub fn fix(mut self, x: usize, y: usize, z: usize, p: PatternId) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y, z).ok_or(SolveError::ModelTopologyMismatch { reason: "fix() coordinate out of range" })?;
        self.fixed.push((n, p));
        Ok(self)
    }

    pub fn domain(mut self, x: usize, y: usize, z: usize, allowed: PatternSet) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y, z).ok_or(SolveError::ModelTopologyMismatch { reason: "domain() coordinate out of range" })?;
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        Ok(self)
    }

    pub fn config(mut self, cfg: SearchConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 🏗️ Adds a global constraint. See [`crate::wfc_engine::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<Grid3dSolver, SolveError> {
        let node_count = self.topology.node_count();
        let mut init_domains = self.init_domains.unwrap_or_else(|| vec![self.model.full_domain(); node_count]);
        let mut fixed = self.fixed;

        for (n, rel, outside_pattern) in self.topology.fixed_outside_restrictions() {
            init_domains[n.index()].and_with(self.model.allowed(rel, outside_pattern));
        }
        let placeholder = PatternId(0);
        for n in self.topology.inactive_cells() {
            fixed.push((n, placeholder));
        }

        let adjacency = build_adjacency_view(&self.topology);
        Ok(Grid3dSolver { model: self.model, topology: self.topology, init_domains, fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖️Builder

// #region 🔖️Solver
/// 🧊️ A WFC solver over a dense 3D grid.
pub struct Grid3dSolver {
    model: CompiledModel,
    topology: Grid3dTopology,
    init_domains: Vec<PatternSet>,
    fixed: Vec<(crate::wfc_engine::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl Grid3dSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() {
            None
        } else {
            Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency })
        }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, cancel),
        }
    }

    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit),
        }
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &Grid3dTopology {
        &self.topology
    }

    pub fn get(&self, solution: &Solution, x: usize, y: usize, z: usize) -> Option<PatternId> {
        let n = self.topology.node_at(x, y, z)?;
        solution.assignment.get(n.index()).copied()
    }

    pub fn decode_tiles(&self, solution: &Solution) -> Vec<Option<crate::wfc_engine::ids::TileId>> {
        solution.assignment.iter().map(|&p| self.model.pattern_info(p).tile).collect()
    }
}
// #endregion 🔖️Solver

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::grid2d::Boundary;
    use crate::wfc_engine::grid3d::{declare_stencil_relations_3d_tiled, Stencil3d};
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
}
// #endregion 🔖️Tests
