//! 🌳️ The search driver: observe → sample → propagate → (on contradiction) chronologically
//! backtrack, until every domain is a singleton (solved), the trail's root frame is exhausted
//! (unsatisfiable — every branch of the search tree was visited), or a budget/restart/cancel
//! signal stops the attempt short of either conclusion.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::ConstraintSet;
use crate::wfc_engine::diag::{DiagLevel, Event, EventSink, Metrics};
use crate::wfc_engine::domain::{DomainStore, RestrictResult};
use crate::wfc_engine::heuristics::{self, ObserveHeuristic};
use crate::wfc_engine::ids::{DecisionId, NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::nogood::{NogoodConfig, NogoodIndex};
use crate::wfc_engine::outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
use crate::wfc_engine::prop_ac3;
use crate::wfc_engine::propagate::PropQueue;
use crate::wfc_engine::sample::{self, ValueSampler};
use crate::wfc_engine::topology::Topology;
use crate::wfc_engine::trail::Trail;
use geometry::random::Rng;

// #region 🔖️Config
/// 🌳️ Whether a failed attempt restarts from scratch or resumes chronological backtracking.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SearchMode {
    /// 🌳️ On contradiction, undo decisions up to [`SearchConfig::restart_schedule`]'s per-attempt
    /// backtrack budget (or the whole tree, if [`RestartSchedule::Never`]); when that budget or
    /// the tree itself is exhausted, discard the attempt and start fresh with a new seed. Never
    /// proves unsatisfiability, even if an attempt happens to exhaust its whole local tree.
    RestartOnly,
    /// 🌳️ On contradiction, undo the most recent decision and try the next candidate. Exhausting
    /// every alternative back to the first decision proves unsatisfiability.
    #[default]
    Backtrack,
    /// 🌳️ Semantically identical to [`SearchMode::Backtrack`] today (same completeness and
    /// soundness guarantees) — true conflict-directed jump-target selection is deferred to land
    /// alongside nogood learning (a later phase), since accelerating the jump without also
    /// recording *why* the skipped decisions were irrelevant risks silently losing completeness.
    /// Selecting this mode is forward-compatible: behavior only gets faster later, never different.
    Backjump,
}

/// 🌳️ Limits that stop a solve attempt before it concludes either way.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Budget {
    pub max_observations: Option<u64>,
    pub max_backtracks: Option<u64>,
    pub max_millis: Option<u64>,
}

/// 🌳️ The per-attempt backtrack budget schedule for [`SearchMode::RestartOnly`]. Ignored by
/// [`SearchMode::Backtrack`]/[`SearchMode::Backjump`], which always run to full completion (or an
/// explicit [`Budget`] limit) to preserve their unsat-proof guarantee.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum RestartSchedule {
    /// 🌳️ No schedule-driven cap — an attempt only restarts when its own local tree is fully
    /// exhausted or the global [`Budget`] stops it.
    #[default]
    Never,
    /// 🌳️ Every attempt gets the same backtrack budget.
    Fixed(u64),
    /// 🌳️ Attempt `i`'s budget is `base * factor.powi(i)`.
    Geometric { base: u64, factor: f64 },
    /// 🌳️ Attempt `i`'s budget is `unit * luby(i + 1)` (the standard Luby restart sequence).
    Luby(u64),
}

impl RestartSchedule {
    async fn backtrack_budget(&self, attempt: u64) -> Option<u64> {
        match *self {
            RestartSchedule::Never => None,
            RestartSchedule::Fixed(n) => Some(n),
            RestartSchedule::Geometric { base, factor } => Some((base as f64 * factor.powi(attempt.min(62) as i32)) as u64),
            RestartSchedule::Luby(unit) => Some(luby(attempt + 1).saturating_mul(unit)),
        }
    }
}

/// 🌳️ The standard Luby sequence (1-indexed): `1,1,2,1,1,2,4,1,1,2,1,1,2,4,8,...`.
async fn luby(i: u64) -> u64 {
    let mut k = 1u32;
    while (1u64 << k) - 1 < i {
        k += 1;
    }
    if i == (1u64 << k) - 1 {
        1u64 << (k - 1)
    } else {
        luby(i - (1u64 << (k - 1)) + 1)
    }
}

/// 🌳️ A shareable, thread-safe flag a caller can set to stop a solve early.
#[derive(Clone, Debug, Default)]
pub struct CancelToken(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl CancelToken {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// 🌳️ Everything [`crate::wfc_engine::solver_graph::GraphSolver`] (and later grid solvers) needs to configure
/// one solve.
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchConfig {
    pub mode: SearchMode,
    pub heuristic: ObserveHeuristic,
    pub sampler: ValueSampler,
    pub budget: Budget,
    /// 🌳️ [`SearchMode::RestartOnly`] only: gives up entirely after this many failed attempts.
    pub max_restarts: Option<u64>,
    /// 🌳️ [`SearchMode::RestartOnly`] only: per-attempt backtrack budget schedule.
    pub restart_schedule: RestartSchedule,
    pub diag_level: DiagLevel,
    /// 🧠️ Opt-in nogood learning (see [`crate::wfc_engine::nogood`]); disabled by default.
    pub nogood: NogoodConfig,
}
// #endregion 🔖️Config

// #region 🔖️Repair
enum RepairOutcome {
    Repaired,
    /// 🌳️ No more frames to pop — the (local or whole) search tree is exhausted.
    Exhausted,
    BudgetExceeded,
    /// 🌳️ [`SearchMode::RestartOnly`]'s per-attempt backtrack budget ran out before either
    /// repairing or exhausting the tree.
    LocalLimitReached,
}

/// 🌳️ Chronologically unwinds decisions until the most recently wiped domain (if any) is resolved
/// — undoing a single frame is not always enough, since a contradiction can be the combined
/// consequence of several decisions; this keeps unwinding until no domain is left wiped rather
/// than trusting the next propagation pass to notice on its own (an already-empty domain can only
/// ever report `Unchanged`, never re-report `Wipeout`, so a silent leftover wipeout would
/// otherwise never be caught).
#[allow(clippy::too_many_arguments)]
async fn backtrack_and_repair<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    budget: &Budget,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    local_remaining: &mut Option<u64>,
    sink: &mut EventSink,
    nogoods: &mut NogoodIndex,
) -> RepairOutcome {
    // The trail's current decision prefix is exactly what caused this contradiction (whether an
    // ordinary propagation wipeout or a rejected complete assignment) — record it once, before any
    // unwinding, while it still reflects the combination that actually failed. Guarded on
    // `is_enabled()` so a disabled (default) store costs not even the `Vec` allocation.
    if nogoods.is_enabled() {
        nogoods.record(trail.active_decisions());
    }
    loop {
        if let Some(rem) = local_remaining {
            if *rem == 0 {
                return RepairOutcome::LocalLimitReached;
            }
            *rem -= 1;
        }
        metrics.backtracks += 1;
        if let Some(max_bt) = budget.max_backtracks {
            if metrics.backtracks >= max_bt {
                return RepairOutcome::BudgetExceeded;
            }
        }
        let frame = match trail.pop_frame() {
            Some(f) => f,
            None => return RepairOutcome::Exhausted,
        };
        trail.undo_to(frame.trail_mark, domains, model.weights());
        if domains.any_wiped() {
            // This frame's decision was not the (sole) cause; keep unwinding without wasting a
            // repair attempt on a node that can't possibly fix a still-wiped domain elsewhere.
            continue;
        }
        let repair_result = domains.get_mut(frame.node).remove(frame.candidate, model.weights());
        trail.record_removed(frame.node, frame.candidate);
        sink.emit_detailed(Event::Backtracked { node: frame.node, candidate: frame.candidate });

        let contradiction = match repair_result {
            RestrictResult::Wipeout => Some(frame.node),
            _ => {
                queue.clear();
                queue.push(frame.node);
                prop_ac3::run_to_fixed_point(model, topo, domains, queue, trail, metrics).err()
            }
        };
        if contradiction.is_none() {
            debug_assert!(!domains.any_wiped());
            return RepairOutcome::Repaired;
        }
    }
}
// #endregion 🔖️Repair

// #region 🔖️Drive
enum StepOutcome {
    Solved,
    /// 🌳️ Every alternative at the root decision has been tried — the (local or whole) search
    /// tree is exhausted.
    Exhausted,
    BudgetExceeded,
    LocalLimitReached,
    Cancelled,
}

/// 🧷️ Whether every constraint accepts the current (assumed all-singleton) domain state. `true`
/// (vacuously) when there are no constraints to check.
async fn constraints_accept(domains: &DomainStore, constraints: Option<&ConstraintSet<'_>>) -> bool {
    let Some(cs) = constraints else { return true };
    let assignment: Vec<PatternId> = domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect();
    cs.constraints.iter().all(|c| c.validate_complete(&assignment, cs.adjacency).is_ok())
}

#[allow(clippy::too_many_arguments)]
async fn decide_and_propagate<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    node: NodeId,
    sink: &mut EventSink,
    nogoods: &mut NogoodIndex,
) -> Option<NodeId> {
    let rng_snapshot = rng.state();
    let candidate = sample::sample_pattern(config.sampler, domains.get(node), model, rng);
    trail.push_frame(DecisionId(*decision_counter), node, candidate, rng_snapshot);
    *decision_counter += 1;
    sink.emit_detailed(Event::Observed { node, chosen: candidate });

    let mut removed = PatternSet::new_empty(model.pattern_count());
    domains.get_mut(node).assign_collecting(candidate, model.weights(), &mut removed);
    trail.record_removed_set(node, &removed);

    queue.clear();
    queue.push(node);
    let contradiction = prop_ac3::run_to_fixed_point(model, topo, domains, queue, trail, metrics).err();
    contradiction.or_else(|| nogoods.on_decision(model, topo, node, candidate, domains, queue, trail, metrics))
}

#[allow(clippy::too_many_arguments)]
async fn drive<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    start: std::time::Instant,
    cancel: Option<&CancelToken>,
    local_backtrack_budget: Option<u64>,
    constraints: Option<&ConstraintSet<'_>>,
    sink: &mut EventSink,
    nogoods: &mut NogoodIndex,
) -> StepOutcome {
    let mut local_remaining = local_backtrack_budget;
    loop {
        if domains.all_singleton() {
            if constraints_accept(domains, constraints) {
                return StepOutcome::Solved;
            }
            // A global constraint rejected this complete assignment: exactly like a contradiction
            // needing a backtrack, reusing the same proven repair machinery.
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink, nogoods) {
                RepairOutcome::Repaired => continue,
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => return StepOutcome::LocalLimitReached,
            }
        }
        let node = match heuristics::select_unresolved(config.heuristic, domains) {
            Some(n) => n,
            None => unreachable!("not all singleton but no unresolved candidate: domain invariant violated"),
        };

        if let Some(max_obs) = config.budget.max_observations {
            if metrics.observations >= max_obs {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(max_ms) = config.budget.max_millis {
            if start.elapsed().as_millis() as u64 >= max_ms {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(c) = cancel {
            if c.is_cancelled() {
                return StepOutcome::Cancelled;
            }
        }
        metrics.observations += 1;

        let contradiction = decide_and_propagate(model, topo, config, rng, domains, queue, trail, metrics, decision_counter, node, sink, nogoods);
        if contradiction.is_some() {
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink, nogoods) {
                RepairOutcome::Repaired => {}
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => return StepOutcome::LocalLimitReached,
            }
        }
    }
}

/// 🌳️ Like [`drive`], but keeps searching for further solutions after each one is found (by
/// treating "solved" the same as a contradiction that must be repaired) until `limit` solutions
/// are collected or the tree is exhausted.
#[allow(clippy::too_many_arguments)]
async fn drive_all<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    start: std::time::Instant,
    solutions: &mut Vec<Vec<PatternId>>,
    limit: usize,
    constraints: Option<&ConstraintSet<'_>>,
    sink: &mut EventSink,
    nogoods: &mut NogoodIndex,
) -> StepOutcome {
    let mut local_remaining = None;
    loop {
        if domains.all_singleton() {
            if constraints_accept(domains, constraints) {
                solutions.push(domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect());
                if solutions.len() >= limit {
                    return StepOutcome::Solved;
                }
            }
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink, nogoods) {
                RepairOutcome::Repaired => continue,
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => unreachable!("solve_all never sets a local backtrack budget"),
            }
        }
        let node = match heuristics::select_unresolved(config.heuristic, domains) {
            Some(n) => n,
            None => unreachable!("not all singleton but no unresolved candidate: domain invariant violated"),
        };
        if let Some(max_obs) = config.budget.max_observations {
            if metrics.observations >= max_obs {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(max_ms) = config.budget.max_millis {
            if start.elapsed().as_millis() as u64 >= max_ms {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        metrics.observations += 1;

        let contradiction = decide_and_propagate(model, topo, config, rng, domains, queue, trail, metrics, decision_counter, node, sink, nogoods);
        if contradiction.is_some() {
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink, nogoods) {
                RepairOutcome::Repaired => {}
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => unreachable!("solve_all never sets a local backtrack budget"),
            }
        }
    }
}
// #endregion 🔖️Drive

// #region 🔖️Solve
struct InitResult {
    domains: DomainStore,
    trail: Trail,
    queue: PropQueue,
    metrics: Metrics,
    wipeout: Option<NodeId>,
}

async fn initialize<T: Topology>(model: &CompiledModel, topo: &T, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], constraints: Option<&ConstraintSet<'_>>) -> InitResult {
    let node_count = topo.node_count();
    let mut domains = DomainStore::new_full(node_count, model.weights());
    let mut trail = Trail::new();
    let mut metrics = Metrics::default();
    let mut wipeout: Option<NodeId> = None;

    if let Some(overrides) = init_domains {
        for (i, allowed) in overrides.iter().enumerate() {
            let n = NodeId::from_index(i);
            let mut removed = PatternSet::new_empty(model.pattern_count());
            if let RestrictResult::Wipeout = domains.get_mut(n).restrict_collecting(allowed, model.weights(), &mut removed) {
                wipeout = Some(n);
            }
            trail.record_removed_set(n, &removed);
        }
    }
    for &(n, p) in fixed {
        let mut removed = PatternSet::new_empty(model.pattern_count());
        if let RestrictResult::Wipeout = domains.get_mut(n).assign_collecting(p, model.weights(), &mut removed) {
            wipeout = Some(n);
        }
        trail.record_removed_set(n, &removed);
    }
    if let Some(cs) = constraints {
        for c in cs.constraints {
            let Ok(restrictions) = c.initialize(&domains, model.weights(), cs.adjacency) else {
                continue; // a misconfigured constraint is a build-time concern, not a solve-time one
            };
            for (n, allowed) in restrictions {
                let mut removed = PatternSet::new_empty(model.pattern_count());
                if let RestrictResult::Wipeout = domains.get_mut(n).restrict_collecting(&allowed, model.weights(), &mut removed) {
                    wipeout = Some(n);
                }
                trail.record_removed_set(n, &removed);
            }
        }
    }

    let mut queue = PropQueue::new(node_count);
    queue.push_all(node_count);
    if wipeout.is_none() {
        wipeout = prop_ac3::run_to_fixed_point(model, topo, &mut domains, &mut queue, &mut trail, &mut metrics).err();
    }

    InitResult { domains, trail, queue, metrics, wipeout }
}

/// 🌳️ Applies `init_domains` (or full domains) and `fixed` pins, runs initial propagation, then
/// drives search per `config` until solved, proven unsatisfiable, or a budget/restart limit stops
/// the attempt. `init_domains`, when present, must have one entry per node.
pub(crate) async fn solve<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)]) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, None, None)
}

pub(crate) async fn solve_cancellable<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], cancel: &CancelToken) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, Some(cancel), None)
}

/// 🌳️ Like [`solve`], but also applies every constraint's initial restriction and rejects (via an
/// ordinary backtrack) any complete assignment a constraint does not accept.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn solve_with_constraints<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    seed: u64,
    init_domains: Option<&[PatternSet]>,
    fixed: &[(NodeId, PatternId)],
    cancel: Option<&CancelToken>,
    constraints: &ConstraintSet<'_>,
) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, cancel, Some(constraints))
}

#[allow(clippy::too_many_arguments)]
async fn solve_inner<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    seed: u64,
    init_domains: Option<&[PatternSet]>,
    fixed: &[(NodeId, PatternId)],
    cancel: Option<&CancelToken>,
    constraints: Option<&ConstraintSet<'_>>,
) -> SolveOutcome {
    let start = std::time::Instant::now();
    let mut rng = Rng::from_seed(seed);
    let mut restarts = 0u64;
    // Persists across restarts: every attempt shares the same `init_domains`/`fixed`/constraints,
    // so a nogood learned in one restart is exactly as valid — and as watchable — in the next.
    let mut nogood_store = NogoodIndex::new(config.nogood);

    loop {
        let mut init = initialize(model, topo, init_domains, fixed, constraints);
        let mut sink = EventSink::new(config.diag_level);
        let mut decision_counter = 0u32;

        if init.wipeout.is_none() {
            init.wipeout = nogood_store.rewatch_for_new_attempt(model, topo, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics);
        }
        if let Some(wiped) = init.wipeout {
            sink.emit(Event::Contradiction { node: wiped });
            return conclude_failed_attempt(config, wiped, init.metrics, seed, &mut restarts, sink, model.fingerprint());
        }

        let local_budget = match config.mode {
            SearchMode::RestartOnly => config.restart_schedule.backtrack_budget(restarts),
            SearchMode::Backtrack | SearchMode::Backjump => None,
        };
        let step = drive(model, topo, config, &mut rng, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics, &mut decision_counter, start, cancel, local_budget, constraints, &mut sink, &mut nogood_store);
        init.metrics.elapsed_millis = start.elapsed().as_millis() as u64;

        match step {
            StepOutcome::Solved => {
                sink.emit(Event::Solved);
                let assignment: Vec<PatternId> = init.domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect();
                return SolveOutcome::Solved(Solution { assignment, report: report(init.metrics, seed, model.fingerprint(), sink) });
            }
            StepOutcome::Exhausted => match config.mode {
                SearchMode::Backtrack | SearchMode::Backjump => {
                    return SolveOutcome::Unsatisfiable(UnsatReport { proven: true, report: report(init.metrics, seed, model.fingerprint(), sink) });
                }
                SearchMode::RestartOnly => {
                    if !restart_or_give_up(config, &mut restarts, &mut sink) {
                        init.metrics.restarts = restarts;
                        return SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: report(init.metrics, seed, model.fingerprint(), sink) });
                    }
                    continue;
                }
            },
            StepOutcome::LocalLimitReached => {
                debug_assert_eq!(config.mode, SearchMode::RestartOnly);
                if !restart_or_give_up(config, &mut restarts, &mut sink) {
                    init.metrics.restarts = restarts;
                    return SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: report(init.metrics, seed, model.fingerprint(), sink) });
                }
                continue;
            }
            StepOutcome::BudgetExceeded => {
                return SolveOutcome::BudgetExceeded { partial: partial_state(&init.domains), report: report(init.metrics, seed, model.fingerprint(), sink) };
            }
            StepOutcome::Cancelled => {
                return SolveOutcome::Cancelled { partial: partial_state(&init.domains), report: report(init.metrics, seed, model.fingerprint(), sink) };
            }
        }
    }
}

/// 🌳️ Exhaustively enumerates up to `limit` solutions, proving `complete = true` iff the whole
/// tree was explored (never stopped early by `limit` or a budget).
pub(crate) async fn solve_all<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], limit: usize) -> (Vec<Solution>, bool) {
    solve_all_inner(model, topo, config, seed, init_domains, fixed, limit, None)
}

/// 🌳️ Like [`solve_all`], but also applies every constraint's initial restriction and excludes any
/// complete assignment a constraint does not accept.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn solve_all_with_constraints<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    seed: u64,
    init_domains: Option<&[PatternSet]>,
    fixed: &[(NodeId, PatternId)],
    limit: usize,
    constraints: &ConstraintSet<'_>,
) -> (Vec<Solution>, bool) {
    solve_all_inner(model, topo, config, seed, init_domains, fixed, limit, Some(constraints))
}

#[allow(clippy::too_many_arguments)]
async fn solve_all_inner<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], limit: usize, constraints: Option<&ConstraintSet<'_>>) -> (Vec<Solution>, bool) {
    let start = std::time::Instant::now();
    let mut rng = Rng::from_seed(seed);
    let mut init = initialize(model, topo, init_domains, fixed, constraints);
    let mut decision_counter = 0u32;
    let mut raw_solutions = Vec::new();
    let mut nogood_store = NogoodIndex::new(config.nogood);
    if init.wipeout.is_none() {
        init.wipeout = nogood_store.rewatch_for_new_attempt(model, topo, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics);
    }

    if init.wipeout.is_some() {
        return (Vec::new(), true);
    }

    let mut sink = EventSink::new(config.diag_level);
    let step = drive_all(model, topo, config, &mut rng, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics, &mut decision_counter, start, &mut raw_solutions, limit, constraints, &mut sink, &mut nogood_store);
    init.metrics.elapsed_millis = start.elapsed().as_millis() as u64;

    let complete = matches!(step, StepOutcome::Exhausted);
    let fingerprint = model.fingerprint();
    // Every returned solution shares the same cumulative event trace from the whole exhaustive
    // search (not a solution-specific slice) — slicing per solution would need each `Solution` to
    // remember its own trail-position range, which isn't worth the bookkeeping until a caller
    // actually needs per-solution replay for `solve_all`.
    let events = sink.into_events();
    let solutions: Vec<Solution> = raw_solutions.into_iter().map(|assignment| Solution { assignment, report: RunReport { metrics: init.metrics, model_fingerprint: fingerprint, seed, events: events.clone() } }).collect();
    (solutions, complete)
}

async fn restart_or_give_up(config: &SearchConfig, restarts: &mut u64, sink: &mut EventSink) -> bool {
    sink.emit(Event::Restarted);
    *restarts += 1;
    !matches!(config.max_restarts, Some(max_r) if *restarts > max_r)
}

async fn partial_state(domains: &DomainStore) -> PartialState {
    PartialState { domains: domains.iter().map(|(_, d)| d.bits().clone()).collect(), decided: domains.iter().map(|(_, d)| d.singleton()).collect() }
}

async fn conclude_failed_attempt(config: &SearchConfig, wiped: NodeId, mut metrics: Metrics, seed: u64, restarts: &mut u64, sink: EventSink, fingerprint: u64) -> SolveOutcome {
    match config.mode {
        SearchMode::Backtrack | SearchMode::Backjump => {
            // A wipeout during the very first propagation (before any decision) with nothing on
            // the trail to undo means every branch is already excluded: unsatisfiable, proven.
            SolveOutcome::Unsatisfiable(UnsatReport { proven: true, report: report(metrics, seed, fingerprint, sink) })
        }
        SearchMode::RestartOnly => {
            *restarts += 1;
            metrics.restarts = *restarts;
            SolveOutcome::Contradiction(ContradictionReport { node: wiped, report: report(metrics, seed, fingerprint, sink) })
        }
    }
}

async fn report(metrics: Metrics, seed: u64, model_fingerprint: u64, sink: EventSink) -> RunReport {
    RunReport { metrics, model_fingerprint, seed, events: sink.into_events() }
}
// #endregion 🔖️Solve

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::oracle;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

    async fn checkerboard_topology(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n.saturating_sub(1) {
            let a = NodeId::from_index(i);
            let c = NodeId::from_index(i + 1);
            tb.arc(a, c, adj);
            tb.arc(c, a, adj);
            arcs.push(oracle::ArcSpec { from: a, to: c, relation: adj });
            arcs.push(oracle::ArcSpec { from: c, to: a, relation: adj });
        }
        (model, tb.build().unwrap(), arcs)
    }

    async fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
        let ne = b.add_relation("ne");
        for &a in &patterns {
            for &c in &patterns {
                if a != c {
                    b.allow(ne, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = NodeId::from_index(i);
                let c = NodeId::from_index(j);
                tb.arc(a, c, ne);
                tb.arc(c, a, ne);
                arcs.push(oracle::ArcSpec { from: a, to: c, relation: ne });
                arcs.push(oracle::ArcSpec { from: c, to: a, relation: ne });
            }
        }
        (model, tb.build().unwrap(), arcs)
    }

    #[test]
    async fn solves_a_satisfiable_path() {
        let (model, topo, arcs) = checkerboard_topology(6);
        let config = SearchConfig::default();
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Solved(sol) => {
                assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    async fn proves_unsat_on_odd_cycle_with_backtrack_mode() {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(5);
        for i in 0..4 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        tb.arc(NodeId(4), NodeId(0), adj);
        tb.arc(NodeId(0), NodeId(4), adj);
        let topo = tb.build().unwrap();

        let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    async fn backtracking_solves_graph_coloring_needing_multiple_decisions() {
        let (model, topo, arcs) = k_graph(4, 4);
        for seed in 0..20 {
            let config = SearchConfig::default();
            let outcome = solve(&model, &topo, &config, seed, None, &[]);
            match outcome {
                SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
                other => panic!("seed {seed}: expected Solved, got {other:?}"),
            }
        }
    }

    #[test]
    async fn unsatisfiable_k5_with_four_colors_proves_unsat() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 7, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    async fn backjump_mode_matches_backtrack_completeness() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 7, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }

        let (model2, topo2, arcs2) = k_graph(4, 4);
        let config2 = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
        let outcome2 = solve(&model2, &topo2, &config2, 3, None, &[]);
        match outcome2 {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model2, &sol.assignment, &arcs2).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    async fn fixed_pins_are_respected() {
        let (model, topo, _arcs) = checkerboard_topology(3);
        let config = SearchConfig::default();
        let outcome = solve(&model, &topo, &config, 5, None, &[(NodeId(0), PatternId(1))]);
        match outcome {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    async fn budget_exceeded_reports_partial_state() {
        // A checkerboard path fully solves after a single decision (propagation alone forces
        // every other node), so the budget must bite before any decision is even attempted.
        let (model, topo, _arcs) = checkerboard_topology(30);
        let config = SearchConfig { budget: Budget { max_observations: Some(0), ..Default::default() }, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        assert!(matches!(outcome, SolveOutcome::BudgetExceeded { .. }));
    }

    #[test]
    async fn same_seed_is_fully_reproducible() {
        let (model, topo, _arcs) = checkerboard_topology(10);
        let config = SearchConfig::default();
        let o1 = solve(&model, &topo, &config, 123, None, &[]);
        let o2 = solve(&model, &topo, &config, 123, None, &[]);
        match (o1, o2) {
            (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => assert_eq!(s1.assignment, s2.assignment),
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    async fn golden_replay_same_seed_reproduces_the_identical_decision_trace() {
        // Determinism at the level of the final assignment (`same_seed_is_fully_reproducible`)
        // is necessary but not sufficient — this checks the exact decision *sequence* two
        // `DiagLevel::Decisions` solves recorded is byte-identical via `TraceReplay`, catching a
        // divergence that happened to still land on the same final assignment by coincidence.
        use crate::wfc_engine::diag::TraceReplay;
        let (model, topo, _arcs) = k_graph(4, 4);
        let config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };
        let o1 = solve(&model, &topo, &config, 77, None, &[]);
        let o2 = solve(&model, &topo, &config, 77, None, &[]);
        match (o1, o2) {
            (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => {
                let t1 = TraceReplay::from_report(&s1.report);
                let t2 = TraceReplay::from_report(&s2.report);
                assert!(!t1.decisions.is_empty(), "k_graph(4,4) needs at least one real decision");
                assert!(t1.matches(&t2));
            }
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    async fn diag_off_records_no_decision_events_but_summary_and_above_do() {
        let (model, topo, _arcs) = checkerboard_topology(5);
        let off_config = SearchConfig { diag_level: DiagLevel::Off, ..Default::default() };
        let decisions_config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };

        let off_outcome = solve(&model, &topo, &off_config, 1, None, &[]);
        let decisions_outcome = solve(&model, &topo, &decisions_config, 1, None, &[]);
        match (off_outcome, decisions_outcome) {
            (SolveOutcome::Solved(off_sol), SolveOutcome::Solved(dec_sol)) => {
                assert!(off_sol.report.events.is_empty());
                assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Observed { .. })));
                assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Solved)));
            }
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    async fn cancellation_stops_search_and_reports_partial() {
        let (model, topo, _arcs) = k_graph(6, 4);
        let cancel = CancelToken::new();
        cancel.cancel();
        let config = SearchConfig::default();
        let outcome = solve_cancellable(&model, &topo, &config, 1, None, &[], &cancel);
        assert!(matches!(outcome, SolveOutcome::Cancelled { .. }));
    }

    #[test]
    async fn cancel_token_reflects_state() {
        let cancel = CancelToken::new();
        assert!(!cancel.is_cancelled());
        cancel.cancel();
        assert!(cancel.is_cancelled());
    }

    #[test]
    async fn restart_only_never_proves_unsat_on_unsatisfiable_instance() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(3), restart_schedule: RestartSchedule::Fixed(5), ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        assert!(matches!(outcome, SolveOutcome::Contradiction(_)));
    }

    #[test]
    async fn restart_only_still_solves_satisfiable_instances() {
        let (model, topo, arcs) = k_graph(4, 4);
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(50), restart_schedule: RestartSchedule::Luby(4), ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    async fn luby_sequence_matches_known_values() {
        let expected = [1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(luby((i + 1) as u64), e, "luby({})", i + 1);
        }
    }

    #[test]
    async fn solve_all_finds_every_solution_and_proves_complete() {
        let (model, topo, arcs) = k_graph(3, 3);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
        assert!(complete);
        assert_eq!(solutions.len(), 6); // 3! proper colorings of K3 with exactly 3 colors
        for sol in &solutions {
            assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
        }
        let mut assignments: Vec<_> = solutions.iter().map(|s| s.assignment.clone()).collect();
        assignments.sort();
        assignments.dedup();
        assert_eq!(assignments.len(), 6, "solve_all must not report the same solution twice");
    }

    #[test]
    async fn solve_all_on_unsat_instance_returns_empty_and_complete() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
        assert!(complete);
        assert!(solutions.is_empty());
    }

    #[test]
    async fn solve_all_respects_limit_and_reports_incomplete() {
        let (model, topo, _arcs) = k_graph(4, 4);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 3);
        assert_eq!(solutions.len(), 3);
        assert!(!complete);
    }

    #[test]
    async fn nogood_learning_still_proves_unsat_on_pigeonhole_instance() {
        use crate::wfc_engine::nogood::NogoodConfig;
        let (model, topo, _arcs) = k_graph(5, 4); // K5 needs 5 colors, only 4 available: unsat
        let config = SearchConfig { nogood: NogoodConfig { enabled: true, ..Default::default() }, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(rep) => assert!(rep.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    async fn nogood_learning_survives_restarts_and_still_proves_unsat() {
        use crate::wfc_engine::nogood::NogoodConfig;
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(20), restart_schedule: RestartSchedule::Fixed(10), nogood: NogoodConfig { enabled: true, ..Default::default() }, ..Default::default() };
        // RestartOnly never proves unsat by itself (it just gives up) — this exercises nogoods
        // persisting and being re-watched across every restart without ever panicking or
        // corrupting the search, over many independent seeds.
        for seed in 0..10 {
            let outcome = solve(&model, &topo, &config, seed, None, &[]);
            assert!(matches!(outcome, SolveOutcome::Contradiction(_)), "seed {seed}: expected Contradiction, got {outcome:?}");
        }
    }

    mod quick {
        use super::*;

        #[test]
        async fn random_instances_solved_or_proven_unsat_match_oracle() {
            let mut rng = Rng::from_seed(777);
            for trial in 0..100 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 7) as usize;
                let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.5);
                let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();
                let init_domains = oracle::testgen::full_domains(&model, node_count);

                let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 1);
                let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
                let outcome = solve(&model, &topo, &config, trial as u64, None, &[]);

                match outcome {
                    SolveOutcome::Solved(sol) => {
                        assert!(!oracle_result.solutions.is_empty(), "trial {trial}: solver found a solution but oracle found none");
                        assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "trial {trial}: solver's solution violates an arc");
                    }
                    SolveOutcome::Unsatisfiable(rep) => {
                        assert!(rep.proven);
                        assert!(oracle_result.solutions.is_empty(), "trial {trial}: solver proved unsat but oracle found a solution");
                    }
                    other => panic!("trial {trial}: unexpected outcome {other:?}"),
                }
            }
        }

        #[test]
        async fn random_instances_with_nogoods_enabled_still_match_oracle() {
            use crate::wfc_engine::nogood::NogoodConfig;
            // Same sweep as `random_instances_solved_or_proven_unsat_match_oracle`, but with
            // nogood learning turned on — nogoods are supposed to be a purely redundant pruning
            // layer (see 🦀️nogood.rs's module doc), so this must reach the exact same
            // Solved-or-proven-Unsatisfiable verdict as the oracle on every trial, never a
            // different one.
            let mut rng = Rng::from_seed(778);
            for trial in 0..100 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 7) as usize;
                let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.5);
                let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();
                let init_domains = oracle::testgen::full_domains(&model, node_count);

                let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 1);
                let config = SearchConfig { mode: SearchMode::Backtrack, nogood: NogoodConfig { enabled: true, max_len: 8, max_count: 64 }, ..Default::default() };
                let outcome = solve(&model, &topo, &config, trial as u64, None, &[]);

                match outcome {
                    SolveOutcome::Solved(sol) => {
                        assert!(!oracle_result.solutions.is_empty(), "trial {trial}: solver found a solution but oracle found none");
                        assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "trial {trial}: solver's solution violates an arc");
                    }
                    SolveOutcome::Unsatisfiable(rep) => {
                        assert!(rep.proven);
                        assert!(oracle_result.solutions.is_empty(), "trial {trial}: solver proved unsat but oracle found a solution");
                    }
                    other => panic!("trial {trial}: unexpected outcome {other:?}"),
                }
            }
        }

        #[test]
        async fn solve_all_matches_oracle_solution_set_on_random_instances() {
            let mut rng = Rng::from_seed(2026);
            for trial in 0..40 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 6) as usize;
                let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.6);
                let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();
                let init_domains = oracle::testgen::full_domains(&model, node_count);

                let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 10_000);
                let config = SearchConfig::default();
                let (solutions, complete) = solve_all(&model, &topo, &config, trial as u64, None, &[], 10_000);

                assert!(complete, "trial {trial}: solve_all did not report complete");
                assert_eq!(complete, oracle_result.complete, "trial {trial}: completeness disagreement");
                let mut got: Vec<Vec<PatternId>> = solutions.iter().map(|s| s.assignment.clone()).collect();
                got.sort();
                let mut want = oracle_result.solutions.clone();
                want.sort();
                assert_eq!(got, want, "trial {trial}: solution set mismatch");
            }
        }
    }
}
// #endregion 🔖️Tests
