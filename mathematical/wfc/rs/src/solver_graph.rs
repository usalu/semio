//! 🕸️ `GraphSolver`: the semantic reference solver over an arbitrary [`GraphTopology`]. Thin
//! wiring — every real behavior lives in [`crate::search`], [`crate::prop_ac3`], and friends;
//! this module only validates a model/topology pairing and forwards to the generic kernel.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, CancelToken, SearchConfig};
use crate::topology::GraphTopology;
use crate::trail::Checkpoint;

// #region 🔖Builder
/// 🏗️ Builds a [`GraphSolver`] from a compiled model and a fixed graph topology.
pub struct GraphSolverBuilder {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
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

    /// 🏗️ Adds a global constraint. See [`crate::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
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
// #endregion 🔖Builder

// #region 🔖Solver
/// 🕸️ The reference WFC solver over an arbitrary fixed directed graph.
pub struct GraphSolver {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl GraphSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() { None } else { Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency }) }
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
}
// #endregion 🔖Solver

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::outcome::SolveOutcome;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard(n: usize) -> (CompiledModel, GraphTopology) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        (model, tb.build().unwrap())
    }

    #[test]
    fn builds_and_solves() {
        let (model, topo) = checkerboard(5);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn fix_pins_a_node() {
        let (model, topo) = checkerboard(4);
        let mut solver = GraphSolverBuilder::new(model, topo).fix(NodeId(0), PatternId(0)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn fix_on_unknown_node_is_rejected() {
        let (model, topo) = checkerboard(2);
        let result = GraphSolverBuilder::new(model, topo).fix(NodeId(99), PatternId(0)).build();
        assert!(result.is_err());
    }

    #[test]
    fn domain_override_restricts_a_node() {
        let (model, topo) = checkerboard(3);
        let mut allowed = PatternSet::new_empty(2);
        allowed.set(PatternId(1), true);
        let mut solver = GraphSolverBuilder::new(model, topo).domain(NodeId(0), allowed).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn solve_all_finds_both_checkerboard_colorings() {
        let (model, topo) = checkerboard(4);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let (solutions, complete) = solver.solve_all(1, 100);
        assert!(complete);
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn solve_cancellable_reports_cancelled_when_pre_cancelled() {
        let (model, topo) = checkerboard(5);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let cancel = CancelToken::new();
        cancel.cancel();
        let outcome = solver.solve_cancellable(1, &cancel);
        assert!(matches!(outcome, SolveOutcome::Cancelled { .. }));
    }

    #[test]
    fn resume_from_checkpoint_completes_the_solve() {
        let (model, topo) = checkerboard(5);
        let fingerprint = model.fingerprint();
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();

        let mut domains = vec![solver.model().full_domain(); solver.topology().node_count()];
        let mut pinned = PatternSet::new_empty(2);
        pinned.set(PatternId(0), true);
        domains[0] = pinned;
        let checkpoint = Checkpoint::new(domains, fingerprint, 9);

        match solver.resume(&checkpoint).unwrap() {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn resume_rejects_mismatched_fingerprint() {
        let (model, topo) = checkerboard(3);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let domains = vec![solver.model().full_domain(); solver.topology().node_count()];
        let checkpoint = Checkpoint::new(domains, 0xDEAD_BEEF, 1);
        assert!(solver.resume(&checkpoint).is_err());
    }
}
// #endregion 🔖Tests
