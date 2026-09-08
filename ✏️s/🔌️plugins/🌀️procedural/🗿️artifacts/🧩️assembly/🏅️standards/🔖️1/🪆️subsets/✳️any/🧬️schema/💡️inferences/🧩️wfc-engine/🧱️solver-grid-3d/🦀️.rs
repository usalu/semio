//! 🧊️ `Grid3dSolver`: dense 3D grid solving. Exactly [`crate::wfc_engine::solver_grid2d`]'s design extended to
//! a third axis — masked-out voxels and [`crate::wfc_engine::grid2d::Boundary::FixedOutside`] faces fold into
//! ordinary domain overrides and fixed pins before delegating to the same generic kernel.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::{build_adjacency_view, AdjacencyView, ConstraintSet, Constraints};
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
    constraints: Vec<Constraints>,
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
    pub fn constraint(mut self, c: Constraints) -> Self {
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
    constraints: Vec<Constraints>,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
