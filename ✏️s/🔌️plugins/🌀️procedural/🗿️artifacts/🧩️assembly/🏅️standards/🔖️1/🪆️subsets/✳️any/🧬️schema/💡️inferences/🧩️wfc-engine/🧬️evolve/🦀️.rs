//! 🧬️ Evolutionary outer loop over seeds: maintains a population of seeds across generations,
//! scoring each via a caller-supplied [`crate::wfc_engine::soft::SoftConstraint`] over the assignment
//! `solve_one` produces, keeping the top `elite_count` survivors each generation and refilling
//! the rest of the population with seeds derived from those survivors. This is the sense in
//! which it's "evolutionary" rather than just [`crate::wfc_engine::soft::best_of_n`] repeated per
//! generation: later generations bias toward seeds descended from ones that scored well, instead
//! of independently resampling from scratch every round.
//!
//! **Scope, stated explicitly**: this evolves only the seed dimension. The original design also
//! sketched evolving weight-fields and tileset parameters as part of the "genotype" — deferred,
//! since neither has an established encoding/mutation operator in this crate yet (unlike a seed,
//! which is already a single `u64` with a natural derive-a-neighbor operation via bit-mixing). A
//! caller wanting to evolve those too can layer it on top of this same population/generation/
//! elitism structure by making `solve_one` itself a function of an evolved parameter set (closing
//! over whatever it's evolving), rather than this module trying to model every possible genotype.

use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::soft::{Attempt, SoftConstraint};

// #region 🔖️Config
#[derive(Clone, Copy, Debug)]
pub struct EvolveConfig {
    pub population_size: usize,
    pub generations: usize,
    /// 🧬️ How many top-scoring seeds survive each generation, both kept as-is and used as parents
    /// for the rest of the next population. Clamped to at least 1 and at most `population_size`.
    pub elite_count: usize,
}

impl Default for EvolveConfig {
    fn default() -> Self {
        Self { population_size: 8, generations: 5, elite_count: 2 }
    }
}

pub struct EvolveResult {
    pub best: Option<Attempt>,
    /// 🧬️ Total successful solves across every generation (population_size * generations, minus
    /// any seed whose `solve_one` returned `None`).
    pub evaluated: usize,
}
// #endregion 🔖️Config

// #region 🔖️Evolve
fn derive_seed(seed: u64, salt: u64) -> u64 {
    let mut z = seed ^ salt.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// 🧬️ Runs the evolutionary loop. `solve_one(seed)` should attempt one solve at that seed and
/// return the resulting assignment (`None` on failure — an unsatisfiable/budget-exceeded/
/// cancelled attempt is simply excluded from selection, not treated as an error). Higher
/// `scorer.score(...)` is always better here (unlike `best_of_n`, which lets the caller pick a
/// direction) — a caller wanting to minimize a quantity should negate it in their scorer.
///
/// 🚦️ De-dyn (O1/R11 open-set case): same reasoning as [`crate::wfc_engine::soft::best_of_n`] — the
/// scorer is caller-supplied and open, so `&dyn SoftConstraint` becomes `&S`, not an enum.
pub fn evolve<S: SoftConstraint>(base_seed: u64, config: EvolveConfig, scorer: &S, mut solve_one: impl FnMut(u64) -> Option<Vec<PatternId>>) -> EvolveResult {
    let population_size = config.population_size.max(1);
    let elite_count = config.elite_count.clamp(1, population_size);
    let mut population: Vec<u64> = (0..population_size as u64).map(|i| derive_seed(base_seed, i)).collect();
    let mut best: Option<Attempt> = None;
    let mut evaluated = 0usize;

    for gen in 0..config.generations {
        let mut scored: Vec<Attempt> = Vec::new();
        for &seed in &population {
            let Some(assignment) = solve_one(seed) else { continue };
            evaluated += 1;
            let score = scorer.score(&assignment);
            if best.as_ref().is_none_or(|b| score > b.score) {
                best = Some(Attempt { seed, assignment: assignment.clone(), score });
            }
            scored.push(Attempt { seed, assignment, score });
        }
        if scored.is_empty() {
            break; // nothing survived this generation to evolve from; further generations can't help
        }
        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).expect("score is never NaN"));
        scored.truncate(elite_count);
        let survivors: Vec<u64> = scored.into_iter().map(|a| a.seed).collect();

        let mut next_population: Vec<u64> = survivors.clone();
        let mut child_salt = (gen as u64 + 1).wrapping_mul(1_000_003);
        while next_population.len() < population_size {
            let parent = survivors[(child_salt as usize) % survivors.len()];
            next_population.push(derive_seed(parent, child_salt));
            child_salt = child_salt.wrapping_add(1);
        }
        population = next_population;
    }
    EvolveResult { best, evaluated }
}
// #endregion 🔖️Evolve

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
