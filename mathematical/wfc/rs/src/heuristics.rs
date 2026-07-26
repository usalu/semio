//! 🎯 Observation heuristics: which unresolved variable the search collapses next. A plain linear
//! scan by design — the plan's own recommendation for small/reference solving (§23.7) — kept as a
//! single small function so a heap-accelerated variant can be swapped in later without touching
//! the public [`ObserveHeuristic`] enum or any call site.

use crate::domain::DomainStore;
use crate::ids::NodeId;

// #region 🔖Heuristic
/// 🎯 Which unresolved (`cardinality > 1`) variable to collapse next.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ObserveHeuristic {
    /// 🎯 Minimum remaining values — smallest domain cardinality first.
    #[default]
    Mrv,
    /// 🎯 Smallest incremental weighted Shannon entropy first.
    WeightedEntropy,
}
// #endregion 🔖Heuristic

// #region 🔖Select
#[derive(Clone, Copy, PartialEq, Debug)]
struct Key(f64, u32);

impl Key {
    fn better_than(self, other: Key) -> bool {
        match self.0.partial_cmp(&other.0).expect("heuristic key must be finite") {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => self.1 < other.1,
        }
    }
}

/// 🎯 The unresolved node with the smallest heuristic key, ties broken by ascending [`NodeId`]
/// for full determinism. `None` when every domain is already singleton (or, degenerately, wiped —
/// callers only reach this after propagation has already ruled that out).
pub(crate) fn select_unresolved(heuristic: ObserveHeuristic, domains: &DomainStore) -> Option<NodeId> {
    let mut best: Option<(NodeId, Key)> = None;
    for (n, d) in domains.iter() {
        if d.cardinality() <= 1 {
            continue;
        }
        let primary = match heuristic {
            ObserveHeuristic::Mrv => d.cardinality() as f64,
            ObserveHeuristic::WeightedEntropy => d.entropy(),
        };
        let key = Key(primary, n.get());
        match best {
            None => best = Some((n, key)),
            Some((_, bk)) => {
                if key.better_than(bk) {
                    best = Some((n, key));
                }
            }
        }
    }
    best.map(|(n, _)| n)
}
// #endregion 🔖Select

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::PatternId;
    use crate::weights::WeightTable;

    #[test]
    fn mrv_picks_smallest_domain() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(3, &w);
        store.get_mut(NodeId(1)).remove(PatternId(0), &w); // node 1: cardinality 3
        store.get_mut(NodeId(2)).remove(PatternId(0), &w);
        store.get_mut(NodeId(2)).remove(PatternId(1), &w); // node 2: cardinality 2
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(2));
    }

    #[test]
    fn ties_break_by_ascending_node_id() {
        let w = WeightTable::new(&[1.0, 1.0]).unwrap();
        let store = DomainStore::new_full(3, &w);
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(0));
    }

    #[test]
    fn singleton_and_resolved_domains_are_skipped() {
        let w = WeightTable::new(&[1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(2, &w);
        store.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(1));
    }

    #[test]
    fn none_when_all_singleton() {
        let w = WeightTable::new(&[1.0]).unwrap();
        let store = DomainStore::new_full(2, &w);
        assert!(select_unresolved(ObserveHeuristic::Mrv, &store).is_none());
    }

    #[test]
    fn weighted_entropy_prefers_lower_entropy_domain() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(2, &w);
        // node 1 has cardinality 2 (lower entropy) after one removal; node 0 stays at cardinality 4.
        store.get_mut(NodeId(1)).remove(PatternId(0), &w);
        store.get_mut(NodeId(1)).remove(PatternId(1), &w);
        let picked = select_unresolved(ObserveHeuristic::WeightedEntropy, &store).unwrap();
        assert_eq!(picked, NodeId(1));
    }
}
// #endregion 🔖Tests
