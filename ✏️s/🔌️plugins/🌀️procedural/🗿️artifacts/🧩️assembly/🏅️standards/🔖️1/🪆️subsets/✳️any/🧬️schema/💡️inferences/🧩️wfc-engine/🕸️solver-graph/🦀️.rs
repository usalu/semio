//! 🕸️ `GraphSolver`: the semantic reference solver over an arbitrary [`GraphTopology`]. Thin
//! wiring — every real behavior lives in [`crate::wfc_engine::search`], [`crate::wfc_engine::prop_ac3`], and friends;
//! this module only validates a model/topology pairing and forwards to the generic kernel.

use crate::wfc_engine::beam::{self, BeamConfig};
use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::{build_adjacency_view, AdjacencyView, ConstraintSet, Constraints};
use crate::wfc_engine::error::SolveError;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::{Solution, SolveOutcome};
use crate::wfc_engine::parallel;
use crate::wfc_engine::repair;
use crate::wfc_engine::search::{self, CancelToken, SearchConfig};
use crate::wfc_engine::topology::GraphTopology;
use crate::wfc_engine::trail::Checkpoint;

// #region 🔖️Builder
/// 🏗️ Builds a [`GraphSolver`] from a compiled model and a fixed graph topology.
pub struct GraphSolverBuilder {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Constraints>,
}

impl GraphSolverBuilder {
    pub fn new(model: CompiledModel, topology: GraphTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    /// 🏗️ Restricts `n`'s initial domain (heterogeneous per-node domains). Nodes never touched
    /// keep the full pattern universe.
    pub fn domain(mut self, n: NodeId, allowed: PatternSet) -> Self {
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        self
    }

    pub fn fix(mut self, n: NodeId, p: PatternId) -> Self {
        self.fixed.push((n, p));
        self
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

    pub fn build(self) -> Result<GraphSolver, SolveError> {
        for &(n, _) in &self.fixed {
            if n.index() >= self.topology.node_count() {
                return Err(SolveError::UnknownNode(n));
            }
        }
        let adjacency = build_adjacency_view(&self.topology);
        Ok(GraphSolver { model: self.model, topology: self.topology, init_domains: self.init_domains, fixed: self.fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖️Builder

// #region 🔖️Solver
/// 🕸️ The reference WFC solver over an arbitrary fixed directed graph.
pub struct GraphSolver {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Constraints>,
    adjacency: AdjacencyView,
}

impl GraphSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() {
            None
        } else {
            Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency })
        }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, cancel),
        }
    }

    /// 🕸️ Exhaustively enumerates up to `limit` solutions; the returned `bool` is `true` iff the
    /// whole search tree was explored (a `false` means `limit` or a budget cut it short).
    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, limit),
        }
    }

    /// 🕸️ Resumes from a [`Checkpoint`] taken from this same model (fingerprint-checked). See
    /// [`Checkpoint`]'s docs for the resumability fidelity this provides.
    pub fn resume(&mut self, checkpoint: &Checkpoint) -> Result<SolveOutcome, SolveError> {
        if checkpoint.model_fingerprint != self.model.fingerprint() {
            return Err(SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
        }
        if checkpoint.domains.len() != self.topology.node_count() {
            return Err(SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
        }
        Ok(search::solve(&self.model, &self.topology, &self.config, checkpoint.seed, Some(&checkpoint.domains), &[]))
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &GraphTopology {
        &self.topology
    }

    /// 🩹️ Re-solves only the region within `radius` relation-hops of `centers`, pinning every
    /// other node to its value in `previous_assignment` (typically a prior `Solved` outcome's
    /// assignment). See [`crate::wfc_engine::repair`]'s module docs for the exact contract — a returned
    /// `Unsatisfiable` means no fix exists at this radius, not that the whole model is unsat.
    pub fn repair(&self, previous_assignment: &[PatternId], centers: &[NodeId], radius: usize, seed: u64) -> SolveOutcome {
        repair::repair_region(&self.model, &self.topology, &self.adjacency, previous_assignment, centers, radius, &self.config, seed)
    }

    /// 🌊️🔦️ Runs incomplete beam search instead of the exact backtracking kernel — see
    /// [`crate::wfc_engine::beam`]'s module docs for the (intentionally incomplete) guarantees. Ignores
    /// `init_domains`/constraints/soft scoring; only `fixed` pins are honored.
    pub fn solve_beam(&self, beam_config: BeamConfig, seed: u64) -> SolveOutcome {
        beam::beam_search(&self.model, &self.topology, beam_config, seed, self.init_domains.as_deref(), &self.fixed)
    }

    /// 🧵️ Runs `attempts` independent solves in stable index order, seeded deterministically
    /// from `base_seed`, and deterministically reduces them — see
    /// [`crate::wfc_engine::parallel::multi_start`]'s docs for the exact reduction rule. Ignores constraints
    /// (like `solve`/`solve_all` without constraints attached); ignores soft scoring.
    pub fn solve_multi_start(&self, base_seed: u64, attempts: usize) -> SolveOutcome {
        parallel::multi_start(&self.model, &self.topology, &self.config, base_seed, self.init_domains.as_deref(), &self.fixed, attempts)
    }
}
// #endregion 🔖️Solver

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
