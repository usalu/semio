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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
