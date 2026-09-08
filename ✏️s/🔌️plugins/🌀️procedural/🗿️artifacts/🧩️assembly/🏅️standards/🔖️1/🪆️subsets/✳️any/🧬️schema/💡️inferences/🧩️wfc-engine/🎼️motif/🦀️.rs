//! 🔬️ Graph motif extraction via iterative color refinement (1-dimensional Weisfeiler-Leman):
//! assigns each node a signature summarizing its rooted neighborhood out to `rounds` hops — own
//! label plus the iteratively-refined multiset of neighbor signatures — so two nodes with
//! isomorphic local neighborhood structure always end up with identical signatures (the classical
//! soundness guarantee of color refinement: it never conflates non-isomorphic neighborhoods
//! within `rounds` hops into the same color unless a genuine coarser 1-WL merge is unavoidable —
//! most real graphs distinguish all non-isomorphic bounded neighborhoods within a few rounds).
//!
//! **Scope, stated explicitly**: this provides the canonicalization primitive, not the full
//! "motifs as higher-order patterns" pipeline the original design sketched — turning a canonical
//! signature into a `PatternId` (and handling the automorphism-group / port-permutation
//! bookkeeping a *directed*, multi-relation graph's motifs would need, since this module's
//! neighbor-color multiset intentionally ignores which relation each arc used) is deferred until
//! a concrete consumer defines exactly what a "motif pattern" should look like for their model.

use crate::wfc_engine::ids::NodeId;
use crate::wfc_engine::topology::Topology;
use std::collections::HashMap;

// #region 🔖️Refine
/// 🔬️ Runs `rounds` steps of color refinement starting from `initial_labels` (one per node,
/// typically each node's own base pattern/tag as a `u64`). Each round replaces every node's color
/// with a hash of its own current color and the sorted multiset of its out-neighbors' current
/// colors — sorting is what makes the result depend only on the neighborhood's *structure*, not
/// on arc enumeration order.
#[allow(dead_code)] // no consumer yet turns signatures into model patterns (see this module's scope note); exercised today only by this module's own tests
pub(crate) fn refine_colors<T: Topology>(topo: &T, initial_labels: &[u64], rounds: usize) -> Vec<u64> {
    let n = topo.node_count();
    debug_assert_eq!(initial_labels.len(), n);
    let mut colors = initial_labels.to_vec();
    for _ in 0..rounds {
        let mut next = Vec::with_capacity(n);
        for i in 0..n {
            let node = NodeId::from_index(i);
            let mut neighbor_colors = Vec::new();
            topo.for_each_out_arc(node, |m, _r| neighbor_colors.push(colors[m.index()]));
            neighbor_colors.sort_unstable();
            next.push(signature_hash(colors[i], &neighbor_colors));
        }
        colors = next;
    }
    colors
}

/// 🔬️ FNV-1a-style mixing (matching `CompiledModel::fingerprint`'s own convention), folding in a
/// node's own color, its neighbor count, and every neighbor color in sorted order.
fn signature_hash(own: u64, neighbor_colors: &[u64]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |x: u64| {
        for b in x.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    mix(own);
    mix(neighbor_colors.len() as u64);
    for &c in neighbor_colors {
        mix(c);
    }
    h
}

/// 🔬️ Relabels arbitrary `u64` colors into dense small integers `0..k`, in first-seen order (so
/// the mapping is deterministic given a deterministic input order) — convenient for turning raw
/// signature hashes into compact motif ids. Returns the relabeled colors and `k`, the number of
/// distinct colors found.
#[allow(dead_code)] // exercised today only by this module's own tests; see refine_colors' note
pub(crate) fn canonicalize(colors: &[u64]) -> (Vec<u32>, usize) {
    let mut map: HashMap<u64, u32> = HashMap::new();
    let mut out = Vec::with_capacity(colors.len());
    for &c in colors {
        let next_id = map.len() as u32;
        let id = *map.entry(c).or_insert(next_id);
        out.push(id);
    }
    (out, map.len())
}
// #endregion 🔖️Refine

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
