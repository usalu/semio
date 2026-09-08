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
    pub fn new(config: NogoodConfig) -> Self {
        Self { config, nogoods: Vec::new(), watchers: HashMap::new() }
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.nogoods.len()
    }

    fn evict_lowest_activity(&mut self) {
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
    pub fn record(&mut self, mut literals: Vec<(NodeId, PatternId)>) {
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
    pub fn rewatch_for_new_attempt<T: Topology>(&mut self, model: &CompiledModel, topo: &T, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut crate::wfc_engine::diag::Metrics) -> Option<NodeId> {
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
    pub fn on_decision<T: Topology>(&mut self, model: &CompiledModel, topo: &T, node: NodeId, pattern: PatternId, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut crate::wfc_engine::diag::Metrics) -> Option<NodeId> {
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
fn is_false(domains: &DomainStore, (node, pattern): (NodeId, PatternId)) -> bool {
    domains.get(node).singleton() == Some(pattern)
}

/// 🧠️ Removes `pattern` from `node`'s domain (which must still contain it) and, if that alone
/// didn't already wipe it out, re-runs AC-3 to a fresh fixed point queued from `node` — exactly
/// what any other propagation-causing mutation in this crate does, so a nogood-forced exclusion
/// can never leave the domain store arc-inconsistent. Returns the first node this drives empty.
#[allow(clippy::too_many_arguments)]
fn force_exclude_and_propagate<T: Topology>(
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
