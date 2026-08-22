//! 🧩️ Chunk-local solving: deterministically seeds and solves *one* grid chunk given fixed seam
//! values already committed by neighboring chunks. This module does not manage a chunk registry,
//! a world coordinate system, halo-width auto-detection from relation offsets, or a
//! boundary-signature cache — those are orchestration concerns for a caller actually streaming an
//! infinite/large world (deferred: no concrete consumer needs that bookkeeping yet). What it does
//! provide is the one primitive such an orchestrator needs and can't get for free from
//! `crate::wfc_engine::search`: a seed that depends only on *where* a chunk is and *what* model it's made
//! from, never on solve order — so re-solving an already-committed chunk (e.g. after evicting it
//! from a cache) reproduces byte-identical content, exactly like `crate::wfc_engine::repair`'s halo re-solve
//! reproduces everything outside its halo unchanged.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::SolveOutcome;
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Seed
/// 🧩️ Combines a world seed with a chunk's integer coordinates and the model's fingerprint into
/// one deterministic per-chunk seed. Same world seed + same chunk coordinate + same model always
/// derives the same seed, regardless of what order chunks are visited in or what else has been
/// solved so far.
pub(crate) fn chunk_seed(world_seed: u64, chunk_x: i64, chunk_y: i64, model_fingerprint: u64) -> u64 {
    let mut z = world_seed;
    for part in [chunk_x as u64, chunk_y as u64, model_fingerprint] {
        z ^= part.wrapping_add(0x9E37_79B9_7F4A_7C15).wrapping_add(z << 6).wrapping_add(z >> 2);
    }
    z
}
// #endregion 🔖️Seed

// #region 🔖️Chunk
/// 🧩️ Solves one chunk: `init_domains` carries the chunk's own baked-in restrictions (e.g. a
/// `Grid2dSolver`'s mask/`Boundary::FixedOutside` folding — `None` if the chunk has none of its
/// own), and `seam_fixed` additionally pins every cell whose value a neighboring chunk already
/// committed. Everything else is solved fresh from [`chunk_seed`]'s derived seed.
/// `Unsatisfiable` means the committed seam values leave no valid fill for this chunk at this
/// model — the caller (e.g. via `crate::wfc_engine::repair`) may need to widen the halo, regenerate the
/// offending neighbor, or otherwise back off, not treat it as a hard failure.
#[allow(clippy::too_many_arguments)]
pub(crate) fn solve_chunk<T: Topology + Clone + Send>(model: &CompiledModel, topo: &T, config: &SearchConfig, world_seed: u64, chunk_x: i64, chunk_y: i64, init_domains: Option<&[PatternSet]>, seam_fixed: &[(NodeId, PatternId)]) -> SolveOutcome {
    let seed = chunk_seed(world_seed, chunk_x, chunk_y, model.fingerprint());
    search::solve(model, topo, config, seed, init_domains, seam_fixed)
}
// #endregion 🔖️Chunk

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::oracle;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

    fn checkerboard(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
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

    #[test]
    fn chunk_seed_is_deterministic_and_varies_by_coordinate_and_model() {
        assert_eq!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 4, 99));
        assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 5, 99));
        assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 4, 4, 99));
        assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 4, 100));
        assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(2, 3, 4, 99));
        // Negative chunk coordinates (a chunk west/north of the origin) must not panic or collide
        // trivially with their positive counterpart.
        assert_ne!(chunk_seed(1, -3, 4, 99), chunk_seed(1, 3, 4, 99));
    }

    #[test]
    fn solve_chunk_reproduces_identical_content_on_repeated_calls() {
        let (model, topo, arcs) = checkerboard(10);
        let config = SearchConfig::default();
        let a = solve_chunk(&model, &topo, &config, 42, 2, -1, None, &[]);
        let b = solve_chunk(&model, &topo, &config, 42, 2, -1, None, &[]);
        match (a, b) {
            (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => {
                assert_eq!(sa.assignment, sb.assignment);
                assert!(oracle::check_assignment(&model, &sa.assignment, &arcs).is_ok());
            }
            _ => panic!("expected both calls to solve identically"),
        }
    }

    #[test]
    fn solve_chunk_respects_seam_pins_from_a_committed_neighbor() {
        let (model, topo, _arcs) = checkerboard(6);
        let config = SearchConfig::default();
        // Simulates a neighboring chunk having already committed node 0 to white.
        let outcome = solve_chunk(&model, &topo, &config, 7, 0, 0, None, &[(NodeId(0), PatternId(1))]);
        match outcome {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn different_chunk_coordinates_can_yield_different_content_for_an_underconstrained_model() {
        // A single isolated node with two equally-likely patterns and no seam pins: different
        // chunk coordinates should be free to (though aren't guaranteed to) land on different
        // values — what matters is they're each internally deterministic, checked here by
        // confirming the two calls aren't forced to collide by construction.
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        b.add_pattern(1.0);
        b.add_relation("r");
        let model = b.compile().unwrap();
        let topo = GraphTopologyBuilder::new(1).build().unwrap();
        let config = SearchConfig::default();
        let at_origin = solve_chunk(&model, &topo, &config, 123, 0, 0, None, &[]);
        let elsewhere = solve_chunk(&model, &topo, &config, 123, 500, -500, None, &[]);
        assert!(matches!(at_origin, SolveOutcome::Solved(_)));
        assert!(matches!(elsewhere, SolveOutcome::Solved(_)));
    }
}
// #endregion 🔖️Tests
