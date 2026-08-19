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
pub async fn enumerate(model: &CompiledModel, node_count: usize, arcs: &[ArcSpec], init_domains: &[PatternSet], limit: usize) -> OracleResult {
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
async fn search(
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
pub async fn check_assignment(model: &CompiledModel, assignment: &[PatternId], arcs: &[ArcSpec]) -> Result<(), Violation> {
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

// #region 🔖️Testgen
/// 🧪️ Seeded generators and named fixtures shared by this crate's differential tests. `pub(crate)`
/// and `cfg(test)`-gated because these helpers are test infrastructure, not public API — every
/// module's `#[cfg(test)] mod tests` can `use crate::wfc_engine::oracle::testgen::*` once `cargo test` enables
/// `cfg(test)` crate-wide.
#[cfg(test)]
pub(crate) mod testgen {
    use super::*;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::weights::WeightTable;

    /// 🧪️ A self-contained tiny instance: a compiled model, its node count, arcs, and per-node
    /// initial domains — everything [`super::enumerate`] and a real solver both need.
    pub struct Fixture {
        pub model: CompiledModel,
        pub node_count: usize,
        pub arcs: Vec<ArcSpec>,
        pub init_domains: Vec<PatternSet>,
    }

    /// 🧪️ Two patterns (black/white) that must differ across every edge of a path graph
    /// `0 - 1 - ... - (n-1)`. Always satisfiable (paths are bipartite).
    pub async fn checkerboard_path(n: usize) -> Fixture {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut arcs = Vec::new();
        for i in 0..n.saturating_sub(1) {
            arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj });
            arcs.push(ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj });
        }
        let init_domains = vec![model.full_domain(); n];
        Fixture { model, node_count: n, arcs, init_domains }
    }

    /// 🧪️ Two patterns that must differ across every edge of an odd cycle `0-1-...-(n-1)-0` with
    /// `n` odd — unsatisfiable, since odd cycles are not bipartite. `n` must be odd and >= 3.
    pub async fn unsat_odd_cycle(n: usize) -> Fixture {
        assert!(n >= 3 && n % 2 == 1, "unsat_odd_cycle requires an odd n >= 3");
        let mut fx = checkerboard_path(n);
        let adj = RelationId(0);
        fx.arcs.push(ArcSpec { from: NodeId::from_index(n - 1), to: NodeId::from_index(0), relation: adj });
        fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(n - 1), relation: adj });
        fx
    }

    /// 🧪️ A complete graph `K_n` over `k` patterns that must all differ pairwise — a proper
    /// `k`-coloring of `K_n`, satisfiable iff `k >= n`.
    pub async fn complete_graph_coloring(n: usize, k: usize) -> Fixture {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
        let ne = b.add_relation("not_equal");
        for &a in &patterns {
            for &c in &patterns {
                if a != c {
                    b.allow(ne, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        let mut arcs = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation: ne });
                arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation: ne });
            }
        }
        let init_domains = vec![model.full_domain(); n];
        Fixture { model, node_count: n, arcs, init_domains }
    }

    /// 🧪️ A uniformly-random tiny compiled model: `pattern_count` patterns each with a random
    /// weight in `[1, 5]`, one relation whose compatibility pairs are each independently kept with
    /// probability `density`.
    pub async fn random_model(rng: &mut geometry::random::Rng, pattern_count: usize, density: f64) -> (CompiledModel, RelationId) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..pattern_count).map(|_| b.add_pattern(1.0 + rng.next_range(0, 5) as f64)).collect();
        let r = b.add_relation("r");
        for &a in &patterns {
            for &c in &patterns {
                if rng.next_bool(density) {
                    b.allow(r, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        (model, r)
    }

    /// 🧪️ A random small connected graph over `node_count` nodes (a random spanning tree plus a
    /// few extra random edges), with both directions registered under `relation`.
    pub async fn random_arcs(rng: &mut geometry::random::Rng, node_count: usize, relation: RelationId) -> Vec<ArcSpec> {
        let mut arcs = Vec::new();
        for i in 1..node_count {
            let j = rng.next_range(0, i as u64) as usize;
            arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation });
            arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation });
        }
        let extra = rng.next_range(0, node_count as u64) as usize;
        for _ in 0..extra {
            if node_count < 2 {
                break;
            }
            let i = rng.next_range(0, node_count as u64) as usize;
            let j = rng.next_range(0, node_count as u64) as usize;
            if i != j {
                arcs.push(ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(j), relation });
                arcs.push(ArcSpec { from: NodeId::from_index(j), to: NodeId::from_index(i), relation });
            }
        }
        arcs
    }

    #[allow(dead_code)]
    pub async fn full_domains(model: &CompiledModel, node_count: usize) -> Vec<PatternSet> {
        vec![model.full_domain(); node_count]
    }

    #[allow(dead_code)]
    pub async fn weight_table_of(model: &CompiledModel) -> &WeightTable {
        model.weights()
    }
}
// #endregion 🔖️Testgen

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use testgen::*;

    #[semio_framework_async_macros::async_test]
    async fn checkerboard_path_is_satisfiable_and_alternates() {
        let fx = checkerboard_path(4);
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
        assert!(result.complete);
        assert!(!result.solutions.is_empty());
        for sol in &result.solutions {
            assert!(check_assignment(&fx.model, sol, &fx.arcs).is_ok());
            for w in sol.windows(2) {
                assert_ne!(w[0], w[1]);
            }
        }
        // Exactly 2 solutions on a path with 2 colors: BWBW... or WBWB...
        assert_eq!(result.solutions.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn unsat_odd_cycle_has_no_solutions() {
        let fx = unsat_odd_cycle(5);
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
        assert!(result.complete);
        assert!(result.solutions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn even_cycle_is_satisfiable() {
        let mut fx = checkerboard_path(6);
        let adj = RelationId(0);
        fx.arcs.push(ArcSpec { from: NodeId::from_index(5), to: NodeId::from_index(0), relation: adj });
        fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(5), relation: adj });
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
        assert!(result.complete);
        assert_eq!(result.solutions.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn complete_graph_coloring_matches_chromatic_condition() {
        let sat = complete_graph_coloring(4, 4);
        let r1 = enumerate(&sat.model, sat.node_count, &sat.arcs, &sat.init_domains, 1000);
        assert!(r1.complete);
        assert!(!r1.solutions.is_empty());
        assert_eq!(r1.solutions.len(), 24); // 4! proper colorings of K4 with exactly 4 colors

        let unsat = complete_graph_coloring(5, 4);
        let r2 = enumerate(&unsat.model, unsat.node_count, &unsat.arcs, &unsat.init_domains, 1000);
        assert!(r2.complete);
        assert!(r2.solutions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn check_assignment_detects_violation() {
        let fx = checkerboard_path(3);
        let bad = vec![PatternId(0), PatternId(0), PatternId(1)];
        assert!(check_assignment(&fx.model, &bad, &fx.arcs).is_err());
        let good = vec![PatternId(0), PatternId(1), PatternId(0)];
        assert!(check_assignment(&fx.model, &good, &fx.arcs).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn limit_caps_collected_solutions() {
        let fx = complete_graph_coloring(4, 4);
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 5);
        assert_eq!(result.solutions.len(), 5);
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn random_instances_every_solution_passes_check_assignment() {
            let mut rng = geometry::random::Rng::from_seed(2024);
            for _ in 0..200 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 8) as usize;
                let (model, r) = random_model(&mut rng, pattern_count, 0.5);
                let arcs = random_arcs(&mut rng, node_count, r);
                let init_domains = full_domains(&model, node_count);
                let result = enumerate(&model, node_count, &arcs, &init_domains, 50);
                for sol in &result.solutions {
                    assert!(check_assignment(&model, sol, &arcs).is_ok());
                }
            }
        }
    }
}
// #endregion 🔖️Tests
