//! 🎲️ Value sampling: which pattern a decision assigns from the selected node's live domain.

use crate::wfc_engine::domain::Domain;
use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::model::CompiledModel;
use semio_framework_geometry::random::Rng;

// #region 🔖️Sampler
/// 🎲️ How one pattern is chosen from an unresolved domain.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ValueSampler {
    /// 🎲️ Probability proportional to pattern weight.
    #[default]
    WeightedRoulette,
    /// 🎲️ Every live pattern equally likely, ignoring weight.
    Uniform,
}

/// 🎲️ Draws one pattern from `domain` (must be non-empty) according to `sampler`.
pub(crate) fn sample_pattern(sampler: ValueSampler, domain: &Domain, model: &CompiledModel, rng: &mut Rng) -> PatternId {
    debug_assert!(domain.cardinality() > 0, "sample_pattern: domain must be non-empty");
    match sampler {
        ValueSampler::Uniform => {
            let k = rng.next_range(0, domain.cardinality() as u64) as usize;
            domain.bits().iter_ones().nth(k).expect("domain non-empty per precondition")
        }
        ValueSampler::WeightedRoulette => {
            let total = domain.sum_w();
            if total <= 0.0 {
                let k = rng.next_range(0, domain.cardinality() as u64) as usize;
                return domain.bits().iter_ones().nth(k).expect("domain non-empty per precondition");
            }
            let target = rng.next_f64() * total;
            let mut acc = 0.0;
            let mut last = None;
            for p in domain.bits().iter_ones() {
                acc += model.weights().w(p);
                last = Some(p);
                if acc >= target {
                    return p;
                }
            }
            last.expect("domain non-empty per precondition")
        }
    }
}
// #endregion 🔖️Sampler

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
