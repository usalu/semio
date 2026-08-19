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
pub(crate) async fn run_to_fixed_point<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut Metrics) -> Result<(), NodeId> {
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
mod tests {
    use super::*;
    use crate::wfc_engine::ids::{PatternId, RelationId};
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

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

    #[test]
    async fn propagation_forces_alternation_after_one_pin() {
        let (model, topo, _adj) = checkerboard(4);
        let mut domains = DomainStore::new_full(4, model.weights());
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let mut queue = PropQueue::new(4);
        queue.push(NodeId(0));
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
        assert_eq!(domains.get(NodeId(0)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(1)).singleton(), Some(PatternId(1)));
        assert_eq!(domains.get(NodeId(2)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(3)).singleton(), Some(PatternId(1)));
        assert!(metrics.propagations > 0);
    }

    #[test]
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
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let mut queue = PropQueue::new(5);
        queue.push(NodeId(0));
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        let result = run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(result.is_err());
    }

    #[test]
    async fn no_dirty_nodes_means_no_op() {
        let (model, topo, _adj) = checkerboard(3);
        let mut domains = DomainStore::new_full(3, model.weights());
        let mut queue = PropQueue::new(3);
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
        for (_, d) in domains.iter() {
            assert_eq!(d.cardinality(), 2);
        }
        assert_eq!(metrics.propagations, 0);
    }
}
// #endregion 🔖️Tests
