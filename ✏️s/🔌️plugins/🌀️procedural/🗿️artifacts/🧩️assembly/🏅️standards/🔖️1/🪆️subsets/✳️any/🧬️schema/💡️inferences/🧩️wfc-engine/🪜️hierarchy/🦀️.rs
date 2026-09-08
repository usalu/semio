//! 🏔️ Two-level hierarchical generation: solve a coarse "macro" model first, then solve one
//! independent "micro" (child) model per macro node — the child model selected by a
//! caller-supplied mapping from the macro node's decided pattern. Each child is solved with a
//! seed deterministically derived from the macro solve's seed and the macro node's id, so
//! re-running with the same macro seed reproduces identical micro content everywhere.
//!
//! **Scope, stated explicitly**: this does not implement backtrack-to-macro (undoing and
//! retrying a *specific* macro decision when its child turns out unsatisfiable while keeping
//! every other macro decision fixed) or a boundary-contract mechanism (macro-adjacent micro
//! regions constraining each other's seam cells) — both need meaningfully deeper integration
//! with `crate::wfc_engine::search`'s trail (essentially a `crate::wfc_engine::repair`-style halo re-solve scoped to one
//! macro node's region, triggered from a different layer's failure) than a first hierarchical
//! pass warrants without a concrete consumer driving the requirements. What this module *does*
//! provide, and what every hierarchical generator needs regardless of that fallback machinery: a
//! sound way to solve the macro layer once, deterministically fan out to per-node child solves,
//! and report exactly which macro node's child failed if any did — `HierarchyOutcome::ChildFailed`
//! carries that node, so a caller can implement its own retry/fallback ladder on top (e.g. retry
//! the whole hierarchy with a new seed, or narrow `child_model_for` to avoid the failing pattern
//! at that node next time) without this module dictating the policy.

use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::{Solution, SolveOutcome};
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Outcome
/// 🏔️ The result of a two-level hierarchical solve.
#[allow(dead_code)] // no solver-level public wrapper yet (deferred — see this module's docs); exercised today only by this module's own tests
pub(crate) enum HierarchyOutcome {
    /// 🏔️ The macro layer solved and every macro node's child model also solved.
    Solved {
        macro_solution: Solution,
        /// 🏔️ Indexed by macro `NodeId`; `children[i]` is macro node `i`'s child solution.
        children: Vec<Solution>,
    },
    /// 🏔️ The macro layer itself had no solution — nothing below it was attempted.
    MacroUnsatisfiable,
    /// 🏔️ The macro layer solved, but the child model at `node` (given its macro-decided
    /// pattern) had no solution. Every macro node before `node` (in iteration order) already
    /// solved its child successfully; nothing after `node` was attempted.
    ChildFailed { node: NodeId },
}
// #endregion 🔖️Outcome

// #region 🔖️Solve
/// 🏔️ Deterministically derives macro node `node`'s child seed from the macro solve's own seed —
/// a single splitmix64-style mixing step (matching `crate::wfc_engine::chunk::chunk_seed`'s approach), so the
/// same macro seed always regenerates identical children.
#[allow(dead_code)] // exercised today only by this module's own tests; see HierarchyOutcome's note
fn child_seed(macro_seed: u64, node: NodeId) -> u64 {
    let mut z = macro_seed ^ (node.get() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// 🏔️ Solves `macro_model`/`macro_topo`, then for every macro node calls `child_model_for(node,
/// macro_pattern)` to get that node's child model/topology/config, solving each independently
/// with a seed derived from `macro_seed`. Stops at the first child failure (see
/// [`HierarchyOutcome::ChildFailed`]) rather than attempting the remaining nodes — a partially-
/// generated hierarchy usually isn't useful to a caller that hasn't decided its own fallback
/// policy yet.
#[allow(dead_code)] // exercised today only by this module's own tests; see HierarchyOutcome's note
pub(crate) fn solve_hierarchy<MT, CT>(macro_model: &CompiledModel, macro_topo: &MT, macro_config: &SearchConfig, macro_seed: u64, child_model_for: impl Fn(NodeId, PatternId) -> (CompiledModel, CT, SearchConfig)) -> HierarchyOutcome
where
    MT: Topology + Clone + Send,
    CT: Topology + Clone + Send,
{
    let macro_solution = match search::solve(macro_model, macro_topo, macro_config, macro_seed, None, &[]) {
        SolveOutcome::Solved(sol) => sol,
        _ => return HierarchyOutcome::MacroUnsatisfiable,
    };

    let mut children = Vec::with_capacity(macro_solution.assignment.len());
    for (i, &pattern) in macro_solution.assignment.iter().enumerate() {
        let node = NodeId::from_index(i);
        let (child_model, child_topo, child_config) = child_model_for(node, pattern);
        let seed = child_seed(macro_seed, node);
        match search::solve(&child_model, &child_topo, &child_config, seed, None, &[]) {
            SolveOutcome::Solved(sol) => children.push(sol),
            _ => return HierarchyOutcome::ChildFailed { node },
        }
    }
    HierarchyOutcome::Solved { macro_solution, children }
}
// #endregion 🔖️Solve

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
