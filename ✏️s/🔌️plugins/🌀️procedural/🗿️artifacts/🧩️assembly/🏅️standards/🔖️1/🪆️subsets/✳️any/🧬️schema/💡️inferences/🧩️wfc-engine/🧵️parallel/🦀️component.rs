//! 🧵️ `std::thread`-based parallel multi-start: launches several independent solve attempts
//! against the *same* model/topology, each with a deterministically-derived seed, and reduces
//! them to one outcome. Every attempt runs to completion before the reduction happens — no
//! "first thread to finish wins" racing — specifically so the result is bit-for-bit identical
//! across runs regardless of thread scheduling, matching every other entry point in this crate's
//! determinism guarantee. `CompiledModel`/`GraphTopology`/`Grid2dTopology`/`Grid3dTopology` hold
//! only plain, interior-mutability-free data, so they're `Sync` automatically — no cloning is
//! needed to share one across threads; each thread allocates its own scratch (domains/trail/
//! queue) exactly as a single-threaded `search::solve` call already does internally.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::SolveOutcome;
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Seed
/// 🧵️ Deterministically derives attempt `i`'s seed from `base_seed` — a single splitmix64-style
/// mixing step, not a call into `geometry::random::Rng` (this only needs to be a fast,
/// collision-resistant *derivation*, not a full PRNG stream).
async fn derive_seed(base_seed: u64, i: usize) -> u64 {
    let mut z = base_seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
// #endregion 🔖️Seed

// #region 🔖️MultiStart
/// 🧵️ Runs `attempts` independent [`search::solve`] calls in parallel (each with a seed derived
/// from `base_seed` via [`derive_seed`]), waits for all of them, then deterministically reduces:
/// the *lowest-index* attempt that reports `Solved` wins; if none solved, the lowest-index
/// attempt's outcome is returned as-is (so a genuine `Unsatisfiable(proven: true)` from attempt 0
/// still correctly proves the whole model unsatisfiable — every attempt targets the same model,
/// so if attempt 0 exhausts its search tree, the model has no solution regardless of what other
/// seeds found).
///
/// `T` must be `Sync` (every topology this crate ships is) since every thread borrows the same
/// `topo` concurrently.
pub(crate) async fn multi_start<T: Topology + Sync>(model: &CompiledModel, topo: &T, config: &SearchConfig, base_seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], attempts: usize) -> SolveOutcome {
    let attempts = attempts.max(1);
    let mut outcomes: Vec<SolveOutcome> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..attempts)
            .map(|i| {
                let seed = derive_seed(base_seed, i);
                scope.spawn(move || search::solve(model, topo, config, seed, init_domains, fixed))
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("solve attempt thread panicked")).collect()
    });

    if let Some(i) = outcomes.iter().position(|o| matches!(o, SolveOutcome::Solved(_))) {
        return outcomes.swap_remove(i);
    }
    outcomes.into_iter().next().expect("attempts.max(1) guarantees at least one outcome")
}
// #endregion 🔖️MultiStart

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::oracle;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

    async fn checkerboard(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
            arcs.push(oracle::ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj });
            arcs.push(oracle::ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj });
        }
        (model, tb.build().unwrap(), arcs)
    }

    async fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
        let ne = b.add_relation("ne");
        for &a in &patterns {
            for &c in &patterns {
                if a != c {
                    b.allow(ne, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                tb.arc(NodeId::from_index(i), NodeId::from_index(j), ne);
                tb.arc(NodeId::from_index(j), NodeId::from_index(i), ne);
            }
        }
        (model, tb.build().unwrap())
    }

    #[semio_framework_async_macros::async_test]
    async fn derive_seed_is_deterministic_and_varies_by_index() {
        assert_eq!(derive_seed(42, 3), derive_seed(42, 3));
        assert_ne!(derive_seed(42, 0), derive_seed(42, 1));
        assert_ne!(derive_seed(42, 5), derive_seed(43, 5));
    }

    #[semio_framework_async_macros::async_test]
    async fn multi_start_finds_a_valid_solution() {
        let (model, topo, arcs) = checkerboard(20);
        let config = SearchConfig::default();
        let outcome = multi_start(&model, &topo, &config, 1, None, &[], 4);
        match outcome {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn multi_start_proves_unsat_when_every_attempt_would() {
        let (model, topo) = k_graph(5, 4); // pigeonhole unsat regardless of seed
        let config = SearchConfig::default();
        let outcome = multi_start(&model, &topo, &config, 7, None, &[], 6);
        match outcome {
            SolveOutcome::Unsatisfiable(rep) => assert!(rep.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn multi_start_is_deterministic_across_repeated_calls() {
        let (model, topo, _arcs) = checkerboard(12);
        let config = SearchConfig::default();
        let a = multi_start(&model, &topo, &config, 99, None, &[], 8);
        let b = multi_start(&model, &topo, &config, 99, None, &[], 8);
        match (a, b) {
            (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => assert_eq!(sa.assignment, sb.assignment),
            _ => panic!("expected both calls to solve"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn single_attempt_matches_a_plain_solve_call() {
        let (model, topo, _arcs) = checkerboard(10);
        let config = SearchConfig::default();
        let direct = search::solve(&model, &topo, &config, derive_seed(5, 0), None, &[]);
        let via_multi_start = multi_start(&model, &topo, &config, 5, None, &[], 1);
        match (direct, via_multi_start) {
            (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => assert_eq!(sa.assignment, sb.assignment),
            _ => panic!("expected both to solve identically"),
        }
    }
}
// #endregion 🔖️Tests
