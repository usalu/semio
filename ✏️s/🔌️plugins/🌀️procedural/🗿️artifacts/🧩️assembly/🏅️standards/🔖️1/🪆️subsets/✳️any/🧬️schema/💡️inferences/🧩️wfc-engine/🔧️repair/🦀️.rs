//! 🩹️ Local repair: re-solve only a bounded neighborhood around a set of "damaged" nodes (a
//! contradiction site, or a user edit), leaving everything else exactly as it was. Implemented as
//! a thin wrapper over the existing kernel — every node outside the halo is pinned to its previous
//! value via ordinary `fixed` pins (the same mechanism a solver's `.fix()` builder method already
//! uses), and the whole model/topology is handed back to `search::solve`. Correctness rides
//! entirely on the already-proven propagation/backtracking kernel; this module only computes the
//! halo and the pin set, so it introduces no new correctness surface of its own.

use crate::wfc_engine::constraint::AdjacencyView;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::SolveOutcome;
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Halo
/// 🩹️ Every node within `radius` relation-hops of any node in `centers` (inclusive of the centers
/// themselves), found via breadth-first search over `adjacency`. `radius = 0` returns exactly
/// `centers` (deduplicated).
pub(crate) fn halo(adjacency: &AdjacencyView, centers: &[NodeId], radius: usize) -> Vec<NodeId> {
    let mut visited = vec![false; adjacency.node_count()];
    let mut frontier: Vec<NodeId> = Vec::new();
    for &c in centers {
        if !visited[c.index()] {
            visited[c.index()] = true;
            frontier.push(c);
        }
    }
    for _ in 0..radius {
        let mut next = Vec::new();
        for &n in &frontier {
            for &m in adjacency.neighbors(n) {
                if !visited[m.index()] {
                    visited[m.index()] = true;
                    next.push(m);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    (0..adjacency.node_count()).map(NodeId::from_index).filter(|&n| visited[n.index()]).collect()
}
// #endregion 🔖️Halo

// #region 🔖️Repair
/// 🩹️ Re-solves only the halo around `centers` (every node within `radius` relation-hops),
/// pinning every OTHER node to its value in `previous_assignment` and leaving the halo's own
/// domains fully open (propagating inward from the pinned exterior). Returns whatever
/// `search::solve` returns for the whole model/topology: `Solved` gives back a complete
/// assignment (byte-identical to `previous_assignment` outside the halo, freshly resolved
/// inside); `Unsatisfiable` means no halo-local fix exists at this radius — the caller may retry
/// with a larger radius, exactly the escalation a chunk-seam or user-edit repair loop needs.
///
/// `previous_assignment` must have one entry per node (as produced by a prior `Solved` outcome on
/// this same model/topology).
#[allow(clippy::too_many_arguments)]
pub(crate) fn repair_region<T: Topology + Clone + Send>(model: &CompiledModel, topo: &T, adjacency: &AdjacencyView, previous_assignment: &[PatternId], centers: &[NodeId], radius: usize, config: &SearchConfig, seed: u64) -> SolveOutcome {
    let region = halo(adjacency, centers, radius);
    let mut in_region = vec![false; adjacency.node_count()];
    for &n in &region {
        in_region[n.index()] = true;
    }
    let fixed: Vec<(NodeId, PatternId)> = (0..adjacency.node_count()).map(NodeId::from_index).filter(|&n| !in_region[n.index()]).map(|n| (n, previous_assignment[n.index()])).collect();
    search::solve(model, topo, config, seed, None, &fixed)
}
// #endregion 🔖️Repair

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
