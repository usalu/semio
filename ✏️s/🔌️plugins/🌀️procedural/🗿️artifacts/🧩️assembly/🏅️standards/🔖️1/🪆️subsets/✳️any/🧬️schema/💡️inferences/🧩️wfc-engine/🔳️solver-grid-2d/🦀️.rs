//! 🧱️ `Grid2dSolver`: dense 2D grid solving on top of [`crate::wfc_engine::grid2d`] and the shared kernel.
//! Masked-out cells and [`Boundary::FixedOutside`] edges are folded into ordinary domain overrides
//! and fixed pins before delegating to the same generic [`crate::wfc_engine::search::solve`] every solver uses.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::chunk;
use crate::wfc_engine::constraint::{build_adjacency_view, AdjacencyView, ConstraintSet, Constraints};
use crate::wfc_engine::error::SolveError;
use crate::wfc_engine::grid2d::Grid2dTopology;
use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::{Solution, SolveOutcome};
use crate::wfc_engine::search::{self, CancelToken, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Builder
/// 🏗️ Builds a [`Grid2dSolver`] over a dense `width × height` grid.
pub struct Grid2dSolverBuilder {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(crate::wfc_engine::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Constraints>,
}

impl Grid2dSolverBuilder {
    pub fn new(model: CompiledModel, topology: Grid2dTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    pub fn fix(mut self, x: usize, y: usize, p: PatternId) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "fix() coordinate out of range" })?;
        self.fixed.push((n, p));
        Ok(self)
    }

    pub fn domain(mut self, x: usize, y: usize, allowed: PatternSet) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "domain() coordinate out of range" })?;
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
    pub fn constraint(mut self, c: Constraints) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<Grid2dSolver, SolveError> {
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
        Ok(Grid2dSolver { model: self.model, topology: self.topology, init_domains, fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖️Builder

// #region 🔖️Solver
/// 🧱️ A WFC solver over a dense 2D grid.
pub struct Grid2dSolver {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Vec<PatternSet>,
    fixed: Vec<(crate::wfc_engine::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Constraints>,
    adjacency: AdjacencyView,
}

impl Grid2dSolver {
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

    pub fn topology(&self) -> &Grid2dTopology {
        &self.topology
    }

    /// 🧩️ Solves this grid as one chunk of a larger streamed/tiled world: `seam_fixed` pins every
    /// cell whose value a neighboring chunk already committed, and the seed is deterministically
    /// derived from `world_seed` plus `(chunk_x, chunk_y)` — see [`crate::wfc_engine::chunk`]'s docs for the
    /// exact contract. Re-solving the same chunk coordinate later reproduces identical content.
    pub fn solve_chunk(&self, world_seed: u64, chunk_x: i64, chunk_y: i64, seam_fixed: &[(crate::wfc_engine::ids::NodeId, PatternId)]) -> SolveOutcome {
        chunk::solve_chunk(&self.model, &self.topology, &self.config, world_seed, chunk_x, chunk_y, Some(&self.init_domains), seam_fixed)
    }

    /// 🧱️ The pattern assigned at `(x, y)` in `solution`.
    pub fn get(&self, solution: &Solution, x: usize, y: usize) -> Option<PatternId> {
        let n = self.topology.node_at(x, y)?;
        solution.assignment.get(n.index()).copied()
    }

    /// 🧱️ Row-major `width * height` tile decode via each pattern's authored tile provenance.
    /// Patterns with no tile provenance (e.g. built directly via [`crate::wfc_engine::model::ModelBuilder`])
    /// decode to `None` at that cell.
    pub fn decode_tiles(&self, solution: &Solution) -> Vec<Option<crate::wfc_engine::ids::TileId>> {
        solution.assignment.iter().map(|&p| self.model.pattern_info(p).tile).collect()
    }
}
// #endregion 🔖️Solver

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
