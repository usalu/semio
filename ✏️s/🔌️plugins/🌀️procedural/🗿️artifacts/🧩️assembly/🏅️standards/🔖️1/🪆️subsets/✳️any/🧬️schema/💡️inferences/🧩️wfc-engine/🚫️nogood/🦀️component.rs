//! 🧠️ Nogood learning: on every contradiction, the current decision prefix (every still-active
//! `(node, chosen-pattern)` decision on the trail) is recorded as a *nogood* — a clause asserting
//! that combination can never all hold simultaneously. Learned clauses are propagated via a
//! two-watched-literal scheme (the standard SAT technique, mapped onto WFC decisions: a literal
//! `(node, pattern)` is "false" exactly when `node` is singleton-decided to exactly `pattern`, and
//! "not false" otherwise — whether still open or already permanently excluded).
//!
//! **Soundness/completeness scope, stated explicitly so it isn't mistaken for a gap**: this store
//! only reacts to explicit search *decisions* (the `on_decision` hook, called from
//! `crate::wfc_engine::search::decide_and_propagate`), never to ordinary propagation-caused domain reductions.
//! A watch can therefore go stale (its partner literal quietly excluded by ordinary propagation)
//! without the store noticing until the next decision touches that nogood — every check below
//! re-derives liveness from live domain state rather than trusting cached watch positions, so a
//! stale watch is self-correcting (checked fresh, never acted on if already false) rather than a
//! source of bugs. This makes nogood learning a purely *optional, redundant* pruning layer on top
//! of the already-complete AC-3 + chronological-backtracking base algorithm: disabling it, or the
//! engine missing an early-prune opportunity, can only change how fast a solve converges, never
//! whether `Solved`/`Unsatisfiable` is correct. Backtracking needs no watch-undo bookkeeping either
//! — once a decision is undone, its literal reverts to "not yet false" automatically (the liveness
//! check reads live domain state), the same property that lets standard SAT solvers skip watch
//! rollback on backtrack.
//!
//! **Every forced exclusion below re-runs AC-3 to a fresh fixed point from the affected node.** A
//! unit-propagated exclusion doesn't just shrink one domain — it can invalidate support at that
//! node's neighbors too, exactly like an ordinary decision's own propagation cascade. Skipping
//! this step would leave the domain store arc-inconsistent (locally correct at the excluded node,
//! stale everywhere the exclusion should have cascaded to), which can silently let a genuinely
//! invalid combination survive to `all_singleton()` — this crate already hit the general shape of
//! this bug once before (the P2 trail-recording gap); re-propagating here is what closes it for
//! nogood-driven exclusions specifically.

use crate::wfc_engine::domain::{DomainStore, RestrictResult};
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::prop_ac3;
use crate::wfc_engine::propagate::PropQueue;
use crate::wfc_engine::topology::Topology;
use crate::wfc_engine::trail::Trail;
use crate::wfc_engine::weights::WeightTable;
use std::collections::HashMap;

// #region 🔖️Config
/// 🧠️ Opt-in — disabled by default so a solve that never asks for this pays no cost (no watcher
/// table allocated, no per-decision lookup).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NogoodConfig {
    pub enabled: bool,
    /// 🧠️ Nogoods longer than this are discarded unrecorded (a long clause rarely prunes early
    /// enough to be worth its upkeep).
    pub max_len: usize,
    /// 🧠️ Above this many stored nogoods, the lowest-activity one is evicted to make room.
    pub max_count: usize,
}

impl Default for NogoodConfig {
    fn default() -> Self {
        Self { enabled: false, max_len: 32, max_count: 4096 }
    }
}
// #endregion 🔖️Config

// #region 🔖️Store
#[derive(Clone, Debug)]
struct Nogood {
    literals: Vec<(NodeId, PatternId)>,
    watch_a: usize,
    watch_b: usize,
    activity: f64,
}

/// 🧠️ Learned-nogood storage plus its two-watched-literal index. Persists across restarts within
/// one `search::solve*` call (constructed once, outside the per-attempt loop) — restarts share the
/// same `init_domains`/`fixed`/constraints, so a nogood learned in attempt N is just as valid, and
/// just as watchable, in attempt N+1.
pub(crate) struct NogoodIndex {
    config: NogoodConfig,
    nogoods: Vec<Nogood>,
    /// 🧠️ `(node, pattern) -> indices into `nogoods` currently watching that exact literal.
    watchers: HashMap<(NodeId, PatternId), Vec<u32>>,
}

impl NogoodIndex {
    pub async fn new(config: NogoodConfig) -> Self {
        Self { config, nogoods: Vec::new(), watchers: HashMap::new() }
    }

    #[inline]
    pub async fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    #[cfg(test)]
    pub async fn len(&self) -> usize {
        self.nogoods.len()
    }

    async fn evict_lowest_activity(&mut self) {
        let Some((idx, _)) = self.nogoods.iter().enumerate().min_by(|(_, a), (_, b)| a.activity.partial_cmp(&b.activity).expect("activity is never NaN")) else { return };
        let removed = self.nogoods.swap_remove(idx);
        for lit in &removed.literals {
            if let Some(v) = self.watchers.get_mut(lit) {
                v.retain(|&i| i as usize != idx);
            }
        }
        // `swap_remove` moved the former last element into `idx`; every watcher entry pointing at
        // the old last index must be renumbered to `idx`, or a future lookup would silently watch
        // the wrong (or a now-nonexistent) nogood.
        let moved_from = self.nogoods.len();
        if idx < moved_from {
            for v in self.watchers.values_mut() {
                for slot in v.iter_mut() {
                    if *slot as usize == moved_from {
                        *slot = idx as u32;
                    }
                }
            }
        }
    }

    /// 🧠️ Records the given decision prefix as a nogood (skipped if disabled, empty, or over
    /// `max_len`). Does not watch it yet — [`NogoodIndex::rewatch_for_new_attempt`] does that once
    /// per attempt, against that attempt's actual domain state.
    pub async fn record(&mut self, mut literals: Vec<(NodeId, PatternId)>) {
        if !self.config.enabled || literals.is_empty() || literals.len() > self.config.max_len {
            return;
        }
        literals.sort_unstable_by_key(|(n, _)| n.get());
        if self.nogoods.len() >= self.config.max_count {
            self.evict_lowest_activity();
        }
        self.nogoods.push(Nogood { literals, watch_a: 0, watch_b: 0, activity: 1.0 });
    }

    /// 🧠️ Re-establishes every stored nogood's watched literals against a fresh attempt's domain
    /// state, applying any immediate unit-propagation this reveals (e.g. a nogood whose every
    /// literal but one is already excluded by `init_domains`/`fixed`/constraints alone, before any
    /// decision). Returns the first node this drives to an empty domain (directly, or via the
    /// AC-3 cascade a forced exclusion triggers), if any — the caller treats that exactly like an
    /// `initialize()`-time constraint wipeout.
    pub async fn rewatch_for_new_attempt<T: Topology>(&mut self, model: &CompiledModel, topo: &T, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut crate::wfc_engine::diag::Metrics) -> Option<NodeId> {
        if !self.config.enabled {
            return None;
        }
        let w = model.weights();
        self.watchers.clear();
        let mut conflict = None;
        for idx in 0..self.nogoods.len() {
            let not_false: Vec<usize> = {
                let ng = &self.nogoods[idx];
                (0..ng.literals.len()).filter(|&j| !is_false(domains, ng.literals[j])).collect()
            };
            match not_false.as_slice() {
                [] => {
                    // Every literal is already decided to exactly its nogood value, with nothing
                    // watched yet to have caught it incrementally: the combination this nogood
                    // forbids already holds outright, before any search decision.
                    conflict.get_or_insert(self.nogoods[idx].literals[0].0);
                }
                [only] => {
                    let (n, p) = self.nogoods[idx].literals[*only];
                    // Only force an exclusion if this literal is genuinely still open (present,
                    // undecided) — if it's already excluded some other way, the clause is already
                    // satisfied via it and needs no forcing.
                    if domains.get(n).bits().get(p) {
                        if let Some(c) = force_exclude_and_propagate(model, topo, domains, queue, trail, metrics, n, p, w) {
                            conflict.get_or_insert(c);
                        }
                    }
                }
                _ => {
                    let a = not_false[0];
                    let b = not_false[1];
                    let ng = &mut self.nogoods[idx];
                    ng.watch_a = a;
                    ng.watch_b = b;
                    self.watchers.entry(ng.literals[a]).or_default().push(idx as u32);
                    self.watchers.entry(ng.literals[b]).or_default().push(idx as u32);
                }
            }
        }
        conflict
    }

    /// 🧠️ Reacts to `node` just having been decided to exactly `pattern` — every nogood watching
    /// this literal just had that watch turn false (by definition: "false" means the node is
    /// decided to exactly the watched pattern) and must find a fresh not-false literal to watch
    /// instead, unit-propagate its partner watch, or (if both watches — and every other literal —
    /// are simultaneously false) report the conflict this combination was learned from.
    #[allow(clippy::too_many_arguments)]
    pub async fn on_decision<T: Topology>(&mut self, model: &CompiledModel, topo: &T, node: NodeId, pattern: PatternId, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut crate::wfc_engine::diag::Metrics) -> Option<NodeId> {
        if !self.config.enabled {
            return None;
        }
        let indices = self.watchers.get(&(node, pattern))?.clone();
        let w = model.weights();
        let mut conflict = None;
        let mut moved: Vec<(usize, (NodeId, PatternId))> = Vec::new();

        for idx in indices {
            let i = idx as usize;
            self.nogoods[i].activity += 1.0;
            let (slot_is_a, other_idx) = {
                let ng = &self.nogoods[i];
                if ng.literals[ng.watch_a] == (node, pattern) {
                    (true, ng.watch_b)
                } else {
                    debug_assert_eq!(ng.literals[ng.watch_b], (node, pattern));
                    (false, ng.watch_a)
                }
            };
            let other_literal = self.nogoods[i].literals[other_idx];

            let replacement = {
                let ng = &self.nogoods[i];
                (0..ng.literals.len()).filter(|&j| j != ng.watch_a && j != ng.watch_b).find(|&j| !is_false(domains, ng.literals[j]))
            };
            if let Some(j) = replacement {
                let ng = &mut self.nogoods[i];
                if slot_is_a {
                    ng.watch_a = j;
                } else {
                    ng.watch_b = j;
                }
                moved.push((i, ng.literals[j]));
                continue;
            }

            // No replacement among the rest of the clause: the partner watch is the sole
            // remaining hope, re-checked fresh (it may have quietly gone false via ordinary
            // propagation this store never reacted to — never trust a cached assumption here).
            if is_false(domains, other_literal) {
                conflict.get_or_insert(node);
            } else if domains.get(other_literal.0).bits().get(other_literal.1) {
                // Still genuinely open (present, undecided): force it out to keep the clause
                // satisfied, exactly the unit-propagation this combination's learning intended.
                if let Some(c) = force_exclude_and_propagate(model, topo, domains, queue, trail, metrics, other_literal.0, other_literal.1, w) {
                    conflict.get_or_insert(c);
                }
            }
            // else: already excluded some other way — the clause is already satisfied via this
            // literal, nothing to force.
        }

        // Every nogood in `moved` had its watch relocated away from `(node, pattern)` — drop its
        // now-stale registration there (a nogood that instead unit-propagated or conflicted kept
        // its watch exactly where it was, so it must stay registered and is never in `moved`).
        // Forgetting this step leaves a dangling index behind: if `(node, pattern)` is ever
        // decided again later (backtracked past and re-explored), `on_decision` would look up
        // this stale index and find `watch_a`/`watch_b` pointing at a completely different
        // literal by then.
        if !moved.is_empty() {
            if let Some(v) = self.watchers.get_mut(&(node, pattern)) {
                v.retain(|idx| !moved.iter().any(|(i, _)| *i as u32 == *idx));
            }
        }
        for (idx, literal) in moved {
            self.watchers.entry(literal).or_default().push(idx as u32);
        }
        conflict
    }
}

/// 🧠️ Whether `(node, pattern)`'s corresponding search-decision literal is definitively false —
/// i.e. `node` is singleton-decided to exactly `pattern`. The complement ("not false") covers both
/// "already permanently excluded" (the negated literal is definitively *true*, the clause already
/// satisfied via it) and "still open" (undecided, pattern still present) — 2WL only ever needs to
/// distinguish false from not-false, never those two sub-cases, when choosing what to watch.
#[inline]
async fn is_false(domains: &DomainStore, (node, pattern): (NodeId, PatternId)) -> bool {
    domains.get(node).singleton() == Some(pattern)
}

/// 🧠️ Removes `pattern` from `node`'s domain (which must still contain it) and, if that alone
/// didn't already wipe it out, re-runs AC-3 to a fresh fixed point queued from `node` — exactly
/// what any other propagation-causing mutation in this crate does, so a nogood-forced exclusion
/// can never leave the domain store arc-inconsistent. Returns the first node this drives empty.
#[allow(clippy::too_many_arguments)]
async fn force_exclude_and_propagate<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut crate::wfc_engine::diag::Metrics,
    node: NodeId,
    pattern: PatternId,
    w: &WeightTable,
) -> Option<NodeId> {
    let result = domains.get_mut(node).remove(pattern, w);
    trail.record_removed(node, pattern);
    if matches!(result, RestrictResult::Wipeout) {
        return Some(node);
    }
    queue.clear();
    queue.push(node);
    prop_ac3::run_to_fixed_point(model, topo, domains, queue, trail, metrics).err()
}
// #endregion 🔖️Store

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::diag::Metrics;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::topology::GraphTopologyBuilder;
    use crate::wfc_engine::weights::WeightTable;

    /// 🧪️ A trivial arc-less topology sized to `node_count` — every test here only needs the
    /// AC-3 re-propagation hook to be a safe no-op (no neighbors to cascade to), not to exercise
    /// AC-3 itself (that's `🦀️prop_ac3.rs`'s own job).
    async fn no_arcs_topology(node_count: usize) -> crate::wfc_engine::topology::GraphTopology {
        GraphTopologyBuilder::new(node_count).build().unwrap()
    }

    async fn w3() -> WeightTable {
        WeightTable::new(&[1.0, 1.0, 1.0]).unwrap()
    }

    async fn model3() -> CompiledModel {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        b.add_pattern(1.0);
        b.add_pattern(1.0);
        b.add_relation("r");
        b.compile().unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn disabled_store_records_and_watches_nothing() {
        let mut store = NogoodIndex::new(NogoodConfig { enabled: false, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
        assert_eq!(store.len(), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn record_skips_empty_and_over_length_clauses() {
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, max_len: 2, max_count: 10 });
        store.record(vec![]);
        assert_eq!(store.len(), 0);
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0)), (NodeId(2), PatternId(0))]);
        assert_eq!(store.len(), 0, "3-literal clause exceeds max_len=2");
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);
        assert_eq!(store.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn eviction_keeps_store_at_max_count() {
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, max_len: 8, max_count: 2 });
        store.record(vec![(NodeId(0), PatternId(0))]);
        store.record(vec![(NodeId(1), PatternId(0))]);
        store.record(vec![(NodeId(2), PatternId(0))]);
        assert_eq!(store.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewatch_unit_propagates_a_length_one_nogood_at_attempt_start() {
        // A length-1 nogood means "node=pattern alone is impossible" — rewatch should exclude it
        // immediately, before any decision.
        let model = model3();
        let topo = no_arcs_topology(2);
        let w = w3();
        let mut domains = DomainStore::new_full(2, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(2);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(1))]);

        let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
        assert!(!domains.get(NodeId(0)).bits().get(PatternId(1)));
        assert_eq!(domains.get(NodeId(0)).cardinality(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn rewatch_reports_conflict_when_length_one_nogood_wipes_a_singleton_domain() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        b.add_relation("r");
        let model = b.compile().unwrap();
        let topo = no_arcs_topology(1);
        let w = WeightTable::new(&[1.0]).unwrap();
        let mut domains = DomainStore::new_full(1, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(1);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0))]);

        let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
        assert_eq!(conflict, Some(NodeId(0)));
    }

    #[semio_framework_async_macros::async_test]
    async fn rewatch_leaves_a_fully_excluded_nogood_inert() {
        // Every literal already impossible before any decision (e.g. `fixed` pinned node 0 away
        // from pattern 0 independent of this nogood): nothing to watch, nothing to propagate.
        let model = model3();
        let topo = no_arcs_topology(2);
        let w = w3();
        let mut domains = DomainStore::new_full(2, &w);
        domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(2);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);

        let conflict = store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
        assert!(domains.get(NodeId(1)).bits().get(PatternId(0)), "the other literal was never touched");
    }

    #[semio_framework_async_macros::async_test]
    async fn on_decision_unit_propagates_the_partner_literal() {
        let model = model3();
        let topo = no_arcs_topology(2);
        let w = w3();
        let mut domains = DomainStore::new_full(2, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(2);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
        store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

        // Decide node0 = pattern0: the nogood's other literal (node1=pattern1) must now be forced
        // out of node1's domain, since keeping it open risks completing the known-bad combination.
        domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
        assert!(!domains.get(NodeId(1)).bits().get(PatternId(1)));
    }

    #[semio_framework_async_macros::async_test]
    async fn on_decision_detects_conflict_when_the_other_watch_was_already_resolved_true() {
        // Models the realistic conflict path: node1 reaches singleton=pattern1 via ordinary
        // propagation the store never reacts to (per its documented scope — only explicit
        // decisions trigger `on_decision`), leaving its watch stale-but-unprocessed. When node0 is
        // then genuinely decided to pattern0, the fresh re-check of the (unmoved) partner watch
        // must discover it's already false too — the exact combination this nogood forbids.
        let model = model3();
        let topo = no_arcs_topology(2);
        let w = w3();
        let mut domains = DomainStore::new_full(2, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(2);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
        store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

        let mut singleton_p1 = crate::wfc_engine::bitset::PatternSet::new_empty(3);
        singleton_p1.set(PatternId(1), true);
        domains.get_mut(NodeId(1)).restrict(&singleton_p1, &w); // propagation-forced, not a decision

        domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
        assert_eq!(conflict, Some(NodeId(0)));
    }

    #[semio_framework_async_macros::async_test]
    async fn on_decision_finds_a_third_literal_as_replacement_watch_instead_of_propagating() {
        // A 3-literal nogood: deciding node0=pattern0 (one watch) must find node2's still-open
        // literal as a replacement watch rather than prematurely forcing node1's exclusion.
        let model = model3();
        let topo = no_arcs_topology(3);
        let w = w3();
        let mut domains = DomainStore::new_full(3, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(3);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1)), (NodeId(2), PatternId(2))]);
        store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

        domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
        assert!(domains.get(NodeId(1)).bits().get(PatternId(1)), "node1's literal must not be forced yet — node2's is still a valid watch");
    }

    #[semio_framework_async_macros::async_test]
    async fn on_decision_ignores_a_watch_whose_partner_is_already_stale() {
        // The partner literal was excluded by something outside this store's notice (simulated by
        // directly removing it); on_decision must re-check liveness live and do nothing, not act
        // on a stale cached assumption.
        let model = model3();
        let topo = no_arcs_topology(2);
        let w = w3();
        let mut domains = DomainStore::new_full(2, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(2);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
        store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

        domains.get_mut(NodeId(1)).remove(PatternId(1), &w); // partner literal silently goes false
        domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn on_decision_forced_exclusion_cascades_through_ac3_to_a_third_node() {
        // node0 --r--> node1 --r--> node2, where `r` only allows equal patterns (so excluding a
        // pattern at node1 must cascade to node2 too). The nogood forbids node0=0 AND node1=1
        // simultaneously; deciding node0=0 forces node1 away from pattern1 — and that exclusion
        // must cascade via AC-3 to remove pattern1 from node2 as well, not just node1.
        let mut b = ModelBuilder::new();
        let p0 = b.add_pattern(1.0);
        let p1 = b.add_pattern(1.0);
        let r = b.add_relation("eq");
        b.allow_mirrored(r, p0, p0);
        b.allow_mirrored(r, p1, p1);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(3);
        tb.arc(NodeId(1), NodeId(2), r);
        tb.arc(NodeId(2), NodeId(1), r);
        let topo = tb.build().unwrap();

        let w = model.weights().clone();
        let mut domains = DomainStore::new_full(3, &w);
        let mut trail = Trail::new();
        let mut queue = PropQueue::new(3);
        let mut metrics = Metrics::default();
        let mut store = NogoodIndex::new(NogoodConfig { enabled: true, ..Default::default() });
        store.record(vec![(NodeId(0), PatternId(0)), (NodeId(1), PatternId(1))]);
        store.rewatch_for_new_attempt(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);

        domains.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let conflict = store.on_decision(&model, &topo, NodeId(0), PatternId(0), &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(conflict.is_none());
        assert!(!domains.get(NodeId(1)).bits().get(PatternId(1)), "node1's forced exclusion");
        assert!(!domains.get(NodeId(2)).bits().get(PatternId(1)), "the exclusion must cascade to node2 via AC-3, or node2 could later be wrongly decided to pattern1");
    }
}
// #endregion 🔖️Tests
