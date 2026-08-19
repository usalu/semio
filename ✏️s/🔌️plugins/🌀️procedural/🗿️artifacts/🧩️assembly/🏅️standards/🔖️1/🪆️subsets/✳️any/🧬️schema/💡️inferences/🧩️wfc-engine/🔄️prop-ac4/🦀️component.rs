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
    pub async fn new<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, metrics: &mut Metrics) -> Result<Self, NodeId> {
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
    pub(crate) async fn count_at(&self, slot: usize, p: PatternId) -> u32 {
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
    pub async fn propagate<T: Topology>(&mut self, model: &CompiledModel, topo: &T, domains: &mut DomainStore, seed_removed: &[(NodeId, PatternId)], metrics: &mut Metrics) -> Result<(), NodeId> {
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
    pub(crate) async fn debug_assert_consistent<T: Topology>(&self, model: &CompiledModel, topo: &T, domains: &DomainStore) {
        let mut scratch = domains.clone();
        let mut scratch_metrics = Metrics::default();
        let fresh = Self::new(model, topo, &mut scratch, &mut scratch_metrics).expect("recomputation from an already-consistent domain state cannot newly wipe out");
        assert_eq!(self.counts, fresh.counts, "AC-4 counters drifted from a from-scratch recomputation");
    }
}
// #endregion 🔖️Engine

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::bitset::PatternSet;
    use crate::wfc_engine::ids::RelationId;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::oracle::testgen;
    use crate::wfc_engine::prop_ac3;
    use crate::wfc_engine::propagate::PropQueue;
    use crate::wfc_engine::topology::GraphTopologyBuilder;
    use crate::wfc_engine::trail::Trail;

    async fn checkerboard(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, RelationId) {
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
        (model, tb.build().unwrap(), adj)
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_counts_match_full_domain_supporter_popcounts() {
        let (model, topo, adj) = checkerboard(3);
        let mut domains = DomainStore::new_full(3, model.weights());
        let mut metrics = Metrics::default();
        let engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        // Node 1 has two incoming arcs (from 0 and from 2); each counted pattern's support should
        // equal the full popcount of allowed(adj, p) intersected with the (full) target domain.
        let mut slots = Vec::new();
        topo.for_each_in_arc(NodeId(1), |_, _, slot| slots.push(slot));
        assert_eq!(slots.len(), 2);
        for &slot in &slots {
            let expected = model.allowed(adj, PatternId(0)).count_ones();
            assert_eq!(engine.count_at(slot, PatternId(0)), expected);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn propagation_forces_alternation_after_one_pin() {
        let (model, topo, _adj) = checkerboard(4);
        let mut domains = DomainStore::new_full(4, model.weights());
        let mut metrics = Metrics::default();
        let mut engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let seed: Vec<_> = removed.iter_ones().map(|p| (NodeId(0), p)).collect();
        engine.propagate(&model, &topo, &mut domains, &seed, &mut metrics).unwrap();
        assert_eq!(domains.get(NodeId(0)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(1)).singleton(), Some(PatternId(1)));
        assert_eq!(domains.get(NodeId(2)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(3)).singleton(), Some(PatternId(1)));
        engine.debug_assert_consistent(&model, &topo, &domains);
    }

    #[semio_framework_async_macros::async_test]
    async fn odd_cycle_pin_propagates_to_wipeout() {
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

        let mut domains = DomainStore::new_full(5, model.weights());
        let mut metrics = Metrics::default();
        let mut engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let seed: Vec<_> = removed.iter_ones().map(|p| (NodeId(0), p)).collect();
        assert!(engine.propagate(&model, &topo, &mut domains, &seed, &mut metrics).is_err());
    }

    /// A random model whose one relation is genuinely symmetric (`allowed(r,a,c) == allowed(r,c,a)`)
    /// and therefore self-consistent under its default self-inverse declaration (`model.validate()`
    /// passes). [`crate::wfc_engine::oracle::testgen::random_model`] does *not* guarantee this — it independently
    /// coin-flips each ordered pair — which is fine for oracle-vs-search differential tests (the
    /// oracle checks whichever arcs are declared, symmetric or not, and a full backtracking search
    /// still converges to the true answer regardless of how tight any one propagator's fixed point
    /// is) but not for comparing two propagators' fixed points directly: this crate's AC-3 engine
    /// (per its own module docs) only reaches *full* arc-consistency when both directions of an
    /// edge encode the same well-formed constraint, which requires a validated, symmetric table.
    async fn random_symmetric_model(rng: &mut geometry::random::Rng, pattern_count: usize, density: f64) -> (CompiledModel, RelationId) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..pattern_count).map(|_| b.add_pattern(1.0 + rng.next_range(0, 5) as f64)).collect();
        let r = b.add_relation("r");
        for i in 0..pattern_count {
            for j in i..pattern_count {
                if rng.next_bool(density) {
                    b.allow(r, patterns[i], patterns[j]);
                    b.allow(r, patterns[j], patterns[i]);
                }
            }
        }
        let model = b.compile().unwrap();
        model.validate().expect("random_symmetric_model must always build a self-consistent relation");
        (model, r)
    }

    /// Regression test for a real bug: `Ac4Engine::new`'s initial "already unsupported" sweep can
    /// discover the same `(node, pattern)` pair via more than one incoming-arc slot, and feeding
    /// that list into `propagate` without deduplicating caused some support counters to be decremented
    /// twice for a single logical removal — over-pruning patterns that still had support. Applying
    /// a batch of removals up front (one `propagate` call with the whole worklist) must give
    /// exactly the same result as applying them one at a time (one `propagate` call each).
    #[semio_framework_async_macros::async_test]
    async fn sequential_and_batch_seed_application_agree() {
        let mut rng = geometry::random::Rng::from_seed(4040);
        for trial in 0..100 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 8) as usize;
            let (model, r) = random_symmetric_model(&mut rng, pattern_count, 0.5);
            let arcs = testgen::random_arcs(&mut rng, node_count, r);
            let mut tb = GraphTopologyBuilder::new(node_count);
            for a in &arcs {
                tb.arc(a.from, a.to, a.relation);
            }
            let topo = tb.build().unwrap();

            let mut scratch = DomainStore::new_full(node_count, model.weights());
            let removal_count = rng.next_range(0, (node_count * pattern_count) as u64) as usize;
            let mut seed_events = Vec::new();
            for _ in 0..removal_count {
                let n = NodeId::from_index(rng.next_range(0, node_count as u64) as usize);
                let p = PatternId::from_index(rng.next_range(0, pattern_count as u64) as usize);
                if scratch.get(n).bits().get(p) {
                    scratch.get_mut(n).remove(p, model.weights());
                    seed_events.push((n, p));
                }
            }

            // Sequential: one propagate() call per removal, applied one at a time.
            let mut domains_seq = DomainStore::new_full(node_count, model.weights());
            let mut metrics_seq = Metrics::default();
            let seq_result = Ac4Engine::new(&model, &topo, &mut domains_seq, &mut metrics_seq).and_then(|mut engine| {
                for &(n, p) in &seed_events {
                    if !domains_seq.get(n).bits().get(p) {
                        continue;
                    }
                    domains_seq.get_mut(n).remove(p, model.weights());
                    engine.propagate(&model, &topo, &mut domains_seq, &[(n, p)], &mut metrics_seq)?;
                }
                Ok(())
            });

            // Batch: all removals applied up front, one propagate() call with the full worklist.
            let mut domains_batch = DomainStore::new_full(node_count, model.weights());
            let mut metrics_batch = Metrics::default();
            let batch_result = Ac4Engine::new(&model, &topo, &mut domains_batch, &mut metrics_batch).and_then(|mut engine| {
                let mut applied = Vec::new();
                for &(n, p) in &seed_events {
                    if domains_batch.get(n).bits().get(p) {
                        domains_batch.get_mut(n).remove(p, model.weights());
                        applied.push((n, p));
                    }
                }
                engine.propagate(&model, &topo, &mut domains_batch, &applied, &mut metrics_batch)
            });

            if let (Err(_), Ok(())) | (Ok(()), Err(_)) = (&seq_result, &batch_result) {
                eprintln!("DEBUG trial {trial}: pattern_count={pattern_count} node_count={node_count}");
                eprintln!("DEBUG arcs={arcs:?}");
                eprintln!("DEBUG seed_events={seed_events:?}");
                for a in 0..pattern_count {
                    for c in 0..pattern_count {
                        eprintln!("DEBUG allowed(r,{a},{c})={}", model.allowed(r, PatternId::from_index(a)).get(PatternId::from_index(c)));
                    }
                }
            }

            match (seq_result, batch_result) {
                (Ok(()), Ok(())) => {
                    for n in 0..node_count {
                        let nid = NodeId::from_index(n);
                        assert_eq!(domains_seq.get(nid).bits(), domains_batch.get(nid).bits(), "trial {trial} node {n}: sequential and batch seed application diverged");
                    }
                }
                (Err(_), Err(_)) => {}
                (a, b) => panic!("trial {trial}: sequential and batch seed application disagreed on satisfiability: sequential={a:?} batch={b:?}"),
            }
        }
    }

    mod quick {
        use super::*;

        /// The P6 gate: AC-3 and AC-4 must reach byte-identical fixed points (or agree that the
        /// state is contradictory) from the same random starting domains. Uses a validated,
        /// symmetric relation (see [`random_symmetric_model`]) — AC-3's forward-only restriction is
        /// only a full arc-consistency algorithm for well-formed models; comparing it against AC-4
        /// on a malformed one (e.g. a self-inverse relation with an asymmetric table, which
        /// `oracle::testgen::random_model` can produce) would compare two *different*, each
        /// internally-valid, propagation strengths rather than testing for a real disagreement.
        #[semio_framework_async_macros::async_test]
        async fn ac3_and_ac4_reach_identical_fixed_points_on_random_instances() {
            let mut rng = geometry::random::Rng::from_seed(4040);
            for trial in 0..300 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 8) as usize;
                let (model, r) = random_symmetric_model(&mut rng, pattern_count, 0.5);
                let arcs = testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();

                let mut domains_a = DomainStore::new_full(node_count, model.weights());
                // Apply a random set of initial single-pattern removals to both copies identically.
                let mut seed_events = Vec::new();
                let removal_count = rng.next_range(0, (node_count * pattern_count) as u64) as usize;
                for _ in 0..removal_count {
                    let n = NodeId::from_index(rng.next_range(0, node_count as u64) as usize);
                    let p = PatternId::from_index(rng.next_range(0, pattern_count as u64) as usize);
                    if domains_a.get(n).bits().get(p) {
                        domains_a.get_mut(n).remove(p, model.weights());
                        seed_events.push((n, p));
                    }
                }
                let mut domains_b = DomainStore::new_full(node_count, model.weights());
                for &(n, p) in &seed_events {
                    domains_b.get_mut(n).remove(p, model.weights());
                }

                // AC-3 must start with every node dirty, not just the seed-touched ones — the same
                // full-graph seeding crate::wfc_engine::search performs at solve start, and necessary so both
                // engines get a fair chance to discover patterns with zero support in the model
                // itself (independent of these specific removals).
                let mut queue = PropQueue::new(node_count);
                queue.push_all(node_count);
                let mut trail = Trail::new();
                let mut metrics_a = Metrics::default();
                let result_a = prop_ac3::run_to_fixed_point(&model, &topo, &mut domains_a, &mut queue, &mut trail, &mut metrics_a);

                // AC-4's own initial sweep (in `Ac4Engine::new`) is the AC-4-side equivalent of
                // AC-3's full-queue seeding above — it must run directly against `domains_b` (not
                // a throwaway full copy) since it mutates whatever store it is given. The random
                // seed removals are then applied on top and fed in as a further worklist, mirroring
                // a real decision's sibling removal.
                let mut metrics_b = Metrics::default();
                let result_b = match Ac4Engine::new(&model, &topo, &mut domains_b, &mut metrics_b) {
                    Err(w) => Err(w),
                    Ok(mut engine) => {
                        let mut applied = Vec::new();
                        for &(n, p) in &seed_events {
                            if domains_b.get(n).bits().get(p) {
                                domains_b.get_mut(n).remove(p, model.weights());
                                applied.push((n, p));
                            }
                        }
                        engine.propagate(&model, &topo, &mut domains_b, &applied, &mut metrics_b)
                    }
                };

                match (result_a, result_b) {
                    (Ok(()), Ok(())) => {
                        for n in 0..node_count {
                            let nid = NodeId::from_index(n);
                            assert_eq!(domains_a.get(nid).bits(), domains_b.get(nid).bits(), "trial {trial}: node {n} domains diverged between AC-3 and AC-4");
                        }
                    }
                    (Err(_), Err(_)) => {} // both detected a contradiction; exact wiped node may differ by processing order
                    (a, b) => panic!("trial {trial}: AC-3 and AC-4 disagreed on satisfiability: ac3={a:?} ac4={b:?}"),
                }
            }
        }
    }
}
// #endregion 🔖️Tests
