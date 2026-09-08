//! 🎯️ Observation heuristics: which unresolved variable the search collapses next. A plain linear
//! scan by design — the plan's own recommendation for small/reference solving (§23.7) — kept as a
//! single small function so a heap-accelerated variant can be swapped in later without touching
//! the public [`ObserveHeuristic`] enum or any call site.

use crate::wfc_engine::domain::DomainStore;
use crate::wfc_engine::ids::NodeId;

// #region 🔖️Heuristic
/// 🎯️ Which unresolved (`cardinality > 1`) variable to collapse next.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ObserveHeuristic {
    /// 🎯️ Minimum remaining values — smallest domain cardinality first.
    #[default]
    Mrv,
    /// 🎯️ Smallest incremental weighted Shannon entropy first.
    WeightedEntropy,
}
// #endregion 🔖️Heuristic

// #region 🔖️Select
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

/// 🎯️ The unresolved node with the smallest heuristic key, ties broken by ascending [`NodeId`]
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
// #endregion 🔖️Select

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
