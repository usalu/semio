//! 🧵️ Deterministic multi-start orchestration. Attempts are sequenced by index until the
//! resumable WFC job owns bounded parallel domain scans. This keeps the solver off private CPU
//! threads: the process-wide worker pool is the only production CPU-thread owner.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::outcome::SolveOutcome;
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::topology::Topology;

// #region 🔖️Seed
/// 🧵️ Deterministically derives attempt `i`'s seed from `base_seed` — a single splitmix64-style
/// mixing step, not a call into `semio_framework_geometry::random::Rng` (this only needs to be a fast,
/// collision-resistant *derivation*, not a full PRNG stream).
fn derive_seed(base_seed: u64, i: usize) -> u64 {
    let mut z = base_seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
// #endregion 🔖️Seed

// #region 🔖️MultiStart
/// 🧵️ Runs `attempts` independent [`search::solve`] calls in stable index order (each with a seed derived
/// from `base_seed` via [`derive_seed`]), waits for all of them, then deterministically reduces:
/// the *lowest-index* attempt that reports `Solved` wins; if none solved, the lowest-index
/// attempt's outcome is returned as-is (so a genuine `Unsatisfiable(proven: true)` from attempt 0
/// still correctly proves the whole model unsatisfiable — every attempt targets the same model,
/// so if attempt 0 exhausts its search tree, the model has no solution regardless of what other
/// seeds found).
///
pub(crate) fn multi_start<T: Topology + Clone + Send>(model: &CompiledModel, topo: &T, config: &SearchConfig, base_seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], attempts: usize) -> SolveOutcome {
    let attempts = attempts.max(1);
    let mut first = None;
    for i in 0..attempts {
        let outcome = search::solve(model, topo, config, derive_seed(base_seed, i), init_domains, fixed);
        if matches!(outcome, SolveOutcome::Solved(_)) {
            return outcome;
        }
        if first.is_none() {
            first = Some(outcome);
        }
    }
    first.expect("attempts.max(1) guarantees at least one outcome")
}
// #endregion 🔖️MultiStart

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
