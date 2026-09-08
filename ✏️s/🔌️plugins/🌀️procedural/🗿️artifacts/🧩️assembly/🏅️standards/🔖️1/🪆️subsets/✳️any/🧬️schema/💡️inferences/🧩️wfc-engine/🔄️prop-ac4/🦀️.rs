//! ⚡️ Support-count (AC-4-style) propagation: for each incoming arc slot and each candidate
//! source pattern, maintain how many currently-live target patterns it is still compatible with;
//! when that count hits zero the source pattern has lost its last support and is removed.
//! Removal-driven (a worklist of `(node, pattern)` events, not dirty nodes) so it never re-scans a
//! whole domain to find what changed.
//!
//! **Scope of this phase**: forward fixed-point propagation only, validated against
//! [`crate::wfc_engine::prop_ac3`] for identical results from identical starting domains (this crate's P6
//! gate). Wiring this engine into [`crate::wfc_engine::search`]'s backtracking loop is deliberately deferred:
//! `counts` is auxiliary state a chronological backtrack would also need to roll back exactly
//! (the trail today only knows how to re-add removed *domain* bits), and getting that rollback
//! subtly wrong is exactly the class of bug this crate's own [`crate::wfc_engine::search`] history already hit
//! once with trail-recording gaps — better to land it deliberately, with its own rollback-soak
//! tests, than to rush it into the hot path now.

use crate::wfc_engine::diag::Metrics;
use crate::wfc_engine::domain::{DomainStore, RestrictResult};
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::topology::Topology;

// #region 🔖️Engine
/// ⚡️ Dense support counters, indexed `[slot * pattern_count + pattern_index]` where `slot` comes
/// from [`Topology::for_each_in_arc`].
#[allow(dead_code)] // exercised by this module's own differential tests today; wired into
                    // crate::wfc_engine::search's runtime engine selection once trail-integrated rollback lands (see module docs)
pub(crate) struct Ac4Engine {
    counts: Vec<u32>,
    pattern_count: usize,
}

#[allow(dead_code)]
impl Ac4Engine {
    /// ⚡️ Computes every counter from scratch against `domains`' current state (`O(arcs * pattern_count^2)`,
    /// a one-time initialization cost) and immediately removes — cascading via [`Ac4Engine::propagate`]
    /// — any pattern that already has zero support given the model alone (independent of any
    /// decision yet made). This initial sweep is not optional: unlike AC-3 (which a caller makes
    /// complete by seeding *every* node into the propagation queue at solve start), AC-4 only
    /// reacts to counters that are *decremented* to zero by a removal event: a counter that starts
    /// at zero — a pattern with literally no compatible neighbor anywhere in the model — would
    /// never trigger a removal without this pass. Returns `Err(node)` if that alone empties a domain.
    pub fn new<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, metrics: &mut Metrics) -> Result<Self, NodeId> {
        let pattern_count = model.pattern_count();
        let total_slots = topo.node_count() * topo.max_in_degree().max(1);
        let mut counts = vec![0u32; total_slots * pattern_count];
        let mut initially_unsupported: Vec<(NodeId, PatternId)> = Vec::new();
        for v in 0..topo.node_count() {
            let vn = NodeId::from_index(v);
            topo.for_each_in_arc(vn, |u, r, slot| {
                for a_idx in 0..pattern_count {
                    let ap = PatternId::from_index(a_idx);
                    let mut m = model.allowed(r, ap).clone();
                    m.and_with(domains.get(vn).bits());
                    let c = m.count_ones();
                    counts[slot * pattern_count + a_idx] = c;
                    if c == 0 && domains.get(u).bits().get(ap) {
                        initially_unsupported.push((u, ap));
                    }
                }
            });
        }

        // The same (node, pattern) pair can be discovered independently via more than one
        // incoming arc slot (each slot's own zero count triggers its own push); deduplicate before
        // treating this as a worklist — a duplicate entry would otherwise make `propagate` decrement
        // that removal's downstream support counters twice, over-pruning patterns that still had
        // support after the *single* real removal.
        initially_unsupported.sort_unstable_by_key(|&(n, p)| (n.get(), p.get()));
        initially_unsupported.dedup();

        let mut engine = Self { counts, pattern_count };
        for &(u, a) in &initially_unsupported {
            if !domains.get(u).bits().get(a) {
                continue; // already removed as a cascading consequence of an earlier entry below
            }
            if let RestrictResult::Wipeout = domains.get_mut(u).remove(a, model.weights()) {
                metrics.removals += 1;
                return Err(u);
            }
            metrics.removals += 1;
        }
        engine.propagate(model, topo, domains, &initially_unsupported, metrics)?;
        Ok(engine)
    }

    #[cfg(test)]
    pub(crate) fn count_at(&self, slot: usize, p: PatternId) -> u32 {
        self.counts[slot * self.pattern_count + p.index()]
    }

    /// ⚡️ Propagates from a worklist of already-removed `(node, pattern)` pairs to a fixed point.
    /// `seed_removed` must reflect patterns actually absent from `domains` relative to whatever
    /// state this engine was [`Ac4Engine::new`]-initialized against. Returns `Err(node)` for the
    /// first domain wiped to empty — including a domain the *caller's own* pre-applied removal
    /// already wiped before this call even started: `propagate`'s internal decrement loop only
    /// ever inspects domains it removes patterns from itself, so a seed removal that was the last
    /// straw for its own node would otherwise be invisible here (an already-empty domain can only
    /// ever report `Unchanged`, never re-report `Wipeout`, the same hazard this crate's AC-3 search
    /// integration hit once with un-checked `Domain::remove` results).
    pub fn propagate<T: Topology>(&mut self, model: &CompiledModel, topo: &T, domains: &mut DomainStore, seed_removed: &[(NodeId, PatternId)], metrics: &mut Metrics) -> Result<(), NodeId> {
        for &(v, _) in seed_removed {
            if domains.get(v).is_wiped() {
                return Err(v);
            }
        }
        let mut queue: std::collections::VecDeque<(NodeId, PatternId)> = seed_removed.iter().copied().collect();
        let mut wipeout: Option<NodeId> = None;
        while let Some((v, b)) = queue.pop_front() {
            if wipeout.is_some() {
                break;
            }
            metrics.propagations += 1;
            topo.for_each_in_arc(v, |u, r, slot| {
                if wipeout.is_some() {
                    return;
                }
                for a in model.supporters(r, b).iter_ones() {
                    let idx = slot * self.pattern_count + a.index();
                    if self.counts[idx] == 0 {
                        continue;
                    }
                    self.counts[idx] -= 1;
                    if self.counts[idx] == 0 && domains.get(u).bits().get(a) {
                        let result = domains.get_mut(u).remove(a, model.weights());
                        metrics.removals += 1;
                        match result {
                            RestrictResult::Wipeout => wipeout = Some(u),
                            _ => queue.push_back((u, a)),
                        }
                    }
                }
            });
        }
        match wipeout {
            Some(n) => Err(n),
            None => Ok(()),
        }
    }

    /// 🩺️ Debug-only: recomputes every counter from `domains`' current state and asserts it
    /// matches. `O(arcs * pattern_count^2)` — a correctness oracle, not a hot-path check. Clones
    /// `domains` so this stays read-only from the caller's perspective even though [`Ac4Engine::new`]
    /// itself mutates whatever store it is given.
    #[cfg(test)]
    pub(crate) fn debug_assert_consistent<T: Topology>(&self, model: &CompiledModel, topo: &T, domains: &DomainStore) {
        let mut scratch = domains.clone();
        let mut scratch_metrics = Metrics::default();
        let fresh = Self::new(model, topo, &mut scratch, &mut scratch_metrics).expect("recomputation from an already-consistent domain state cannot newly wipe out");
        assert_eq!(self.counts, fresh.counts, "AC-4 counters drifted from a from-scratch recomputation");
    }
}
// #endregion 🔖️Engine

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
