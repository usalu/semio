//! 🕸️ `GraphSolver`: the semantic reference solver over an arbitrary [`GraphTopology`]. Thin
//! wiring — every real behavior lives in [`crate::search`], [`crate::prop_ac3`], and friends;
//! this module only validates a model/topology pairing and forwards to the generic kernel.

use crate::beam::{self, BeamConfig};
use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::parallel;
use crate::repair;
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

    /// 🩹 Re-solves only the region within `radius` relation-hops of `centers`, pinning every
    /// other node to its value in `previous_assignment` (typically a prior `Solved` outcome's
    /// assignment). See [`crate::repair`]'s module docs for the exact contract — a returned
    /// `Unsatisfiable` means no fix exists at this radius, not that the whole model is unsat.
    pub fn repair(&self, previous_assignment: &[PatternId], centers: &[NodeId], radius: usize, seed: u64) -> SolveOutcome {
        repair::repair_region(&self.model, &self.topology, &self.adjacency, previous_assignment, centers, radius, &self.config, seed)
    }

    /// 🌊🔦 Runs incomplete beam search instead of the exact backtracking kernel — see
    /// [`crate::beam`]'s module docs for the (intentionally incomplete) guarantees. Ignores
    /// `init_domains`/constraints/soft scoring; only `fixed` pins are honored.
    pub fn solve_beam(&self, beam_config: BeamConfig, seed: u64) -> SolveOutcome {
        beam::beam_search(&self.model, &self.topology, beam_config, seed, self.init_domains.as_deref(), &self.fixed)
    }

    /// 🧵 Runs `attempts` independent solves in parallel (one `std::thread` each, seeded
    /// deterministically from `base_seed`) and deterministically reduces them — see
    /// [`crate::parallel::multi_start`]'s docs for the exact reduction rule. Ignores constraints
    /// (like `solve`/`solve_all` without constraints attached); ignores soft scoring.
    pub fn solve_multi_start(&self, base_seed: u64, attempts: usize) -> SolveOutcome {
        parallel::multi_start(&self.model, &self.topology, &self.config, base_seed, self.init_domains.as_deref(), &self.fixed, attempts)
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

    #[test]
    fn repair_reopens_only_the_requested_halo() {
        let (model, topo) = checkerboard(6);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let previous = match solver.solve(1) {
            SolveOutcome::Solved(sol) => sol.assignment,
            other => panic!("expected an initial Solved baseline, got {other:?}"),
        };

        match solver.repair(&previous, &[NodeId(3)], 1, 2) {
            SolveOutcome::Solved(sol) => {
                for i in [0usize, 1, 5] {
                    assert_eq!(sol.assignment[i], previous[i], "node {i} is outside the radius-1 halo and must be unchanged");
                }
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn solve_beam_finds_a_valid_solution_and_respects_fixed_pins() {
        use crate::beam::BeamConfig;
        let (model, topo) = checkerboard(6);
        let solver = GraphSolverBuilder::new(model, topo).fix(NodeId(0), PatternId(1)).build().unwrap();
        match solver.solve_beam(BeamConfig::default(), 5) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn solve_multi_start_finds_a_valid_solution_and_respects_fixed_pins() {
        let (model, topo) = checkerboard(10);
        let solver = GraphSolverBuilder::new(model, topo).fix(NodeId(0), PatternId(0)).build().unwrap();
        match solver.solve_multi_start(11, 4) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    // End-to-end constraint wiring: unlike `crate::constraints_card`'s own tests (which exercise
    // `Constraint::initialize`/`validate_complete` directly), these drive a real
    // `GraphSolverBuilder::constraint(...)` through `solve()`, proving the `search::solve_with_constraints`
    // path — initial restriction, per-complete-assignment rejection, and backtrack-and-retry on
    // rejection — actually wires together end to end.
    #[test]
    fn cardinality_constraint_forces_the_unique_matching_checkerboard_coloring() {
        use crate::constraint::PatternSelector;
        use crate::constraints_card::{CardinalityConstraint, Scope};

        // checkerboard(5) is a 5-node path with exactly two valid 2-colorings: [B,W,B,W,B] (3
        // black) and [W,B,W,B,W] (2 black). Neither the constraint's `initialize` (domains start
        // full, so its possible/required bounds can't detect infeasibility up front) nor a trivial
        // propagation pass rules either coloring out — only `validate_complete`, invoked per
        // candidate via `backtrack_and_repair`, can. Requiring exactly 3 black therefore forces the
        // first coloring and proves the reject-and-backtrack path actually runs.
        let (model, topo) = checkerboard(5);
        let black = PatternId(0);
        let constraint = CardinalityConstraint::new(model.clone(), PatternSelector::Pattern(black), Scope::All, 3, 3).unwrap();
        let mut solver = GraphSolverBuilder::new(model, topo).constraint(Box::new(constraint)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                assert_eq!(sol.assignment, vec![PatternId(0), PatternId(1), PatternId(0), PatternId(1), PatternId(0)]);
                assert_eq!(sol.assignment.iter().filter(|&&p| p == black).count(), 3);
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn cardinality_constraint_beyond_both_colorings_is_unsatisfiable() {
        use crate::constraint::PatternSelector;
        use crate::constraints_card::{CardinalityConstraint, Scope};

        // Neither valid coloring of checkerboard(5) has 4+ black nodes (max achievable is 3), so
        // this must exhaust the full (small) search tree via repeated constraint rejection and
        // report a proven-unsatisfiable outcome, not merely an initial-domain wipeout.
        let (model, topo) = checkerboard(5);
        let black = PatternId(0);
        let constraint = CardinalityConstraint::new(model.clone(), PatternSelector::Pattern(black), Scope::All, 4, 5).unwrap();
        let mut solver = GraphSolverBuilder::new(model, topo).constraint(Box::new(constraint)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    fn flow_constraint_end_to_end_forces_a_connected_path_through_a_real_solve() {
        use crate::constraint::PatternSelector;
        use crate::flow::FlowConstraint;
        use crate::model::ModelBuilder;
        use crate::topology::GraphTopologyBuilder;

        // Two floor/wall patterns on a 4-node path; requiring flow 1 from node0 to node3 forces
        // every node to be floor (the only way an edge-disjoint path can exist end to end),
        // proving the constraint's initialize-through-solve-through-validate_complete path runs
        // for real, not just FlowConstraint::validate_complete in isolation.
        let mut b = ModelBuilder::new();
        let floor = b.add_pattern(1.0);
        let wall = b.add_pattern(1.0);
        let adj = b.add_relation("adj");
        b.allow_mirrored(adj, floor, floor);
        b.allow_mirrored(adj, floor, wall);
        b.allow_mirrored(adj, wall, wall);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(4);
        for i in 0..3 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        let topo = tb.build().unwrap();

        let constraint = FlowConstraint::new(model.clone(), PatternSelector::Pattern(floor), vec![NodeId(0)], vec![NodeId(3)], 1);
        let mut solver = GraphSolverBuilder::new(model, topo).constraint(Box::new(constraint)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => assert!(sol.assignment.iter().all(|&p| p == floor)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }
}
// #endregion 🔖Tests
