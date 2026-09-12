//! 🔮️ Brute-force reference solver. Deliberately shares no code with [`crate::wfc_engine::propagate`]/
//! [`crate::wfc_engine::search`] — a naive, obviously-correct DFS enumerator (with only per-step consistency
//! pruning against already-assigned neighbors, never arc-consistency propagation) that every
//! optimized engine is checked against in this crate's differential tests.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId, RelationId};
use crate::wfc_engine::model::CompiledModel;

// #region 🔖️Enumerate
/// 🔮️ One directed compatibility arc the oracle must respect, exactly mirroring what a solver's
/// propagation kernel would enumerate for the same topology.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArcSpec {
    pub from: NodeId,
    pub to: NodeId,
    pub relation: RelationId,
}

/// 🔮️ The result of [`enumerate`]: every found solution (up to `limit`), plus whether the search
/// tree was fully explored (`complete = false` means either `limit` or the internal step budget
/// was hit first — an [`Unsatisfiable`](crate)-style conclusion can only be drawn when `complete`).
#[derive(Clone, Debug)]
pub struct OracleResult {
    pub solutions: Vec<Vec<PatternId>>,
    pub complete: bool,
}

/// 🔮️ One arc-compatibility violation found by [`check_assignment`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Violation {
    ArcViolated { from: NodeId, to: NodeId, relation: RelationId },
}

const DEFAULT_STEP_BUDGET: u64 = 5_000_000;

/// 🔮️ Exhaustively enumerates every complete assignment respecting `init_domains` and every arc in
/// `arcs`, via chronological DFS over nodes `0..node_count` with only-check-against-assigned-neighbors
/// pruning (no propagation). Intended for tiny instances only; `limit` bounds collected solutions,
/// an internal step budget bounds worst-case runtime regardless of `limit`.
pub fn enumerate(model: &CompiledModel, node_count: usize, arcs: &[ArcSpec], init_domains: &[PatternSet], limit: usize) -> OracleResult {
    debug_assert_eq!(init_domains.len(), node_count);
    let mut incoming: Vec<Vec<(NodeId, RelationId)>> = vec![Vec::new(); node_count];
    let mut outgoing: Vec<Vec<(NodeId, RelationId)>> = vec![Vec::new(); node_count];
    for a in arcs {
        outgoing[a.from.index()].push((a.to, a.relation));
        incoming[a.to.index()].push((a.from, a.relation));
    }

    let mut assignment: Vec<Option<PatternId>> = vec![None; node_count];
    let mut solutions = Vec::new();
    let mut budget = DEFAULT_STEP_BUDGET;
    let complete = search(model, node_count, &outgoing, &incoming, init_domains, &mut assignment, 0, &mut solutions, limit, &mut budget);
    OracleResult { solutions, complete }
}

#[allow(clippy::too_many_arguments)]
fn search(
    model: &CompiledModel,
    node_count: usize,
    outgoing: &[Vec<(NodeId, RelationId)>],
    incoming: &[Vec<(NodeId, RelationId)>],
    init_domains: &[PatternSet],
    assignment: &mut Vec<Option<PatternId>>,
    i: usize,
    solutions: &mut Vec<Vec<PatternId>>,
    limit: usize,
    budget: &mut u64,
) -> bool {
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    if solutions.len() >= limit {
        return false;
    }
    if i == node_count {
        solutions.push(assignment.iter().map(|o| o.expect("every node assigned at depth == node_count")).collect());
        return true;
    }
    let mut explored_fully = true;
    for p in init_domains[i].iter_ones() {
        let mut ok = true;
        for &(from, rel) in &incoming[i] {
            if let Some(fp) = assignment[from.index()] {
                if !model.allowed(rel, fp).get(p) {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            for &(to, rel) in &outgoing[i] {
                if let Some(tp) = assignment[to.index()] {
                    if !model.allowed(rel, p).get(tp) {
                        ok = false;
                        break;
                    }
                }
            }
        }
        if !ok {
            continue;
        }
        assignment[i] = Some(p);
        let sub_complete = search(model, node_count, outgoing, incoming, init_domains, assignment, i + 1, solutions, limit, budget);
        assignment[i] = None;
        if !sub_complete {
            explored_fully = false;
        }
        if *budget == 0 {
            return false;
        }
    }
    explored_fully
}

/// 🔮️ Checks a complete assignment against every arc, independent of [`enumerate`]'s search code.
pub fn check_assignment(model: &CompiledModel, assignment: &[PatternId], arcs: &[ArcSpec]) -> Result<(), Violation> {
    for a in arcs {
        let src_p = assignment[a.from.index()];
        let dst_p = assignment[a.to.index()];
        if !model.allowed(a.relation, src_p).get(dst_p) {
            return Err(Violation::ArcViolated { from: a.from, to: a.to, relation: a.relation });
        }
    }
    Ok(())
}
// #endregion 🔖️Enumerate

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
