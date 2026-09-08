//! 🌊️🔦️ Beam search: an incomplete alternative to full backtracking. Maintains up to `width`
//! independent partial-domain states ("beams"); each round, every surviving beam's heuristically
//! chosen frontier node is expanded into one child beam per still-possible pattern (not just one
//! sampled value — the width of exploration comes from branching wide then pruning, not from
//! random sampling), all children are scored, and only the best `width` survive into the next
//! round. Trades completeness (a beam that was actually still solvable can be pruned away) for
//! bounded memory and no exponential blowup.
//!
//! Unlike [`crate::wfc_engine::search`]'s kernel, a dropped beam is never revisited: this is **not** a
//! sound-or-complete search strategy. [`beam_search`] never returns `Unsatisfiable` — only
//! `Solved` (some beam reached an all-singleton, arc-consistent state) or `Contradiction` (every
//! beam died, or `max_steps` ran out, before any reached completion). A `Contradiction` from this
//! function is not proof the model is unsatisfiable, only that beam search didn't find a solution
//! at this `width`/`max_steps`.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::diag::Metrics;
use crate::wfc_engine::domain::{DomainStore, RestrictResult};
use crate::wfc_engine::heuristics::{self, ObserveHeuristic};
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::{ContradictionReport, RunReport, Solution, SolveOutcome};
use crate::wfc_engine::prop_ac3;
use crate::wfc_engine::propagate::PropQueue;
use crate::wfc_engine::topology::Topology;
use crate::wfc_engine::trail::Trail;
use semio_framework_geometry::random::Rng;

// #region 🔖️Config
#[derive(Clone, Copy, Debug)]
pub struct BeamConfig {
    /// 🌊️🔦️ Maximum number of beams kept after each round's prune.
    pub width: usize,
    /// 🌊️🔦️ Hard cap on rounds (roughly one decision per surviving beam per round).
    pub max_steps: usize,
    pub heuristic: ObserveHeuristic,
}

impl Default for BeamConfig {
    fn default() -> Self {
        Self { width: 4, max_steps: 100_000, heuristic: ObserveHeuristic::default() }
    }
}
// #endregion 🔖️Config

// #region 🔖️Search
#[derive(Clone)]
struct Beam {
    domains: DomainStore,
}

/// 🌊️🔦️ Higher is better: fewer total remaining candidates across every domain, i.e. closer to a
/// complete assignment. A cheap, purely domain-shape-based progress proxy — no soft-constraint
/// scoring hook yet (deferred: a caller wanting soft-guided beam search can layer
/// `crate::wfc_engine::soft::SoftConstraint` scoring on top once there's a concrete consumer).
fn score(domains: &DomainStore) -> f64 {
    -(domains.iter().map(|(_, d)| (d.cardinality().max(1) - 1) as f64).sum::<f64>())
}

fn build_root<T: Topology>(model: &CompiledModel, topo: &T, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], metrics: &mut Metrics) -> Option<DomainStore> {
    let node_count = topo.node_count();
    let w = model.weights();
    let mut domains = DomainStore::new_full(node_count, w);
    let mut trail = Trail::new();
    let mut wiped = false;

    if let Some(overrides) = init_domains {
        for (i, allowed) in overrides.iter().enumerate() {
            if matches!(domains.get_mut(NodeId::from_index(i)).restrict(allowed, w), RestrictResult::Wipeout) {
                wiped = true;
            }
        }
    }
    for &(n, p) in fixed {
        if matches!(domains.get_mut(n).assign(p, w), RestrictResult::Wipeout) {
            wiped = true;
        }
    }

    let mut queue = PropQueue::new(node_count);
    queue.push_all(node_count);
    if !wiped && prop_ac3::run_to_fixed_point(model, topo, &mut domains, &mut queue, &mut trail, metrics).is_err() {
        wiped = true;
    }
    if wiped {
        None
    } else {
        Some(domains)
    }
}

/// 🌊️🔦️ Runs beam search to either a solution or exhaustion of every beam / `max_steps`. See this
/// module's docs for the exact (incomplete) guarantees.
pub(crate) fn beam_search<T: Topology>(model: &CompiledModel, topo: &T, beam_config: BeamConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)]) -> SolveOutcome {
    let fingerprint = model.fingerprint();
    let mut metrics = Metrics::default();
    let contradiction = |metrics: Metrics| SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: RunReport { metrics, model_fingerprint: fingerprint, seed, events: Vec::new() } });

    let Some(root_domains) = build_root(model, topo, init_domains, fixed, &mut metrics) else {
        return contradiction(metrics);
    };
    let mut beams = vec![Beam { domains: root_domains }];
    let mut rng = Rng::from_seed(seed);
    let w = model.weights();

    for _ in 0..beam_config.max_steps {
        if let Some(i) = beams.iter().position(|b| b.domains.all_singleton()) {
            let assignment: Vec<PatternId> = beams[i].domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guarantees a singleton")).collect();
            return SolveOutcome::Solved(Solution { assignment, report: RunReport { metrics, model_fingerprint: fingerprint, seed, events: Vec::new() } });
        }

        let mut candidates: Vec<Beam> = Vec::new();
        for beam in &beams {
            let Some(node) = heuristics::select_unresolved(beam_config.heuristic, &beam.domains) else { continue };
            let live_patterns: Vec<PatternId> = beam.domains.get(node).bits().iter_ones().collect();
            for pattern in live_patterns {
                let mut next_domains = beam.domains.clone();
                let mut next_trail = Trail::new();
                let mut next_queue = PropQueue::new(next_domains.len());
                metrics.observations += 1;
                next_domains.get_mut(node).assign(pattern, w);
                next_queue.push(node);
                if prop_ac3::run_to_fixed_point(model, topo, &mut next_domains, &mut next_queue, &mut next_trail, &mut metrics).is_ok() {
                    candidates.push(Beam { domains: next_domains });
                }
            }
        }
        if candidates.is_empty() {
            return contradiction(metrics);
        }

        // Shuffle before the stable sort so equally-scored candidates aren't always ordered by
        // ascending pattern id — `seed` gives every beam search a genuinely different tie-break
        // rather than a purely deterministic-by-pattern-index one.
        rng.shuffle(&mut candidates);
        candidates.sort_by(|a, b| score(&b.domains).partial_cmp(&score(&a.domains)).expect("score is never NaN"));
        candidates.truncate(beam_config.width.max(1));
        beams = candidates;
    }
    contradiction(metrics)
}
// #endregion 🔖️Search

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
