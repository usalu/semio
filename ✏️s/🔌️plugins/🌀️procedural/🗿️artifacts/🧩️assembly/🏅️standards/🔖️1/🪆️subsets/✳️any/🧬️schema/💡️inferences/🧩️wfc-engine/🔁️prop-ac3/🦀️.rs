//! ⚙️ Bitset arc-revision propagation — the reference engine every optimized engine (AC-4,
//! watched-support, added in a later phase) is checked against. For a dirty node `n` and out-arc
//! `n --r--> m`, computes `union = OR of allowed(r, p) for p in domain(n)` and intersects it into
//! `domain(m)`. Simple, obviously correct, no auxiliary state to roll back.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::diag::Metrics;
use crate::wfc_engine::domain::{DomainStore, RestrictResult};
use crate::wfc_engine::ids::NodeId;
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::propagate::PropQueue;
use crate::wfc_engine::topology::Topology;
use crate::wfc_engine::trail::Trail;

// #region 🔖️Engine
/// ⚙️ Drains `queue`, running arc revision to a fixed point. Every pattern actually removed is
/// recorded on `trail` — propagation-caused removals vastly outnumber decision-caused ones, and a
/// backtrack that only undid the decision's own removals (not their propagated consequences) would
/// leave contradictions permanently invisible to future arc-consistency checks (an already-empty
/// domain can never re-report `Wipeout`, it can only ever report `Unchanged`). Returns `Err(node)`
/// for the first node whose domain is wiped to empty; `Ok(())` means every queued node's out-arcs
/// are consistent (`domain(m)` is a subset of what every arc from an assigned/reduced neighbor allows).
pub(crate) fn run_to_fixed_point<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut Metrics) -> Result<(), NodeId> {
    let p = model.pattern_count();
    let mut union = PatternSet::new_empty(p);
    let mut removed = PatternSet::new_empty(p);
    let mut wipeout: Option<NodeId> = None;

    while let Some(n) = queue.pop() {
        if wipeout.is_some() {
            break;
        }
        metrics.propagations += 1;
        let n_bits = domains.get(n).bits().clone();
        topo.for_each_out_arc(n, |m, r| {
            if wipeout.is_some() {
                return;
            }
            union.clear_all();
            for pat in n_bits.iter_ones() {
                union.or_with(model.allowed(r, pat));
            }
            let result = domains.get_mut(m).restrict_collecting(&union, model.weights(), &mut removed);
            match result {
                RestrictResult::Unchanged => {}
                RestrictResult::Wipeout => {
                    trail.record_removed_set(m, &removed);
                    wipeout = Some(m);
                }
                RestrictResult::Reduced(count) => {
                    trail.record_removed_set(m, &removed);
                    metrics.removals += count as u64;
                    queue.push(m);
                }
                RestrictResult::Singleton(_) => {
                    trail.record_removed_set(m, &removed);
                    metrics.removals += 1;
                    queue.push(m);
                }
            }
        });
    }

    match wipeout {
        Some(n) => Err(n),
        None => Ok(()),
    }
}
// #endregion 🔖️Engine

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
