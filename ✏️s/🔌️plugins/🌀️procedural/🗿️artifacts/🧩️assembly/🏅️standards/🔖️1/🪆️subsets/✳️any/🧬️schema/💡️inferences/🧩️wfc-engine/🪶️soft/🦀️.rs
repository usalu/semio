//! 🎯️ Soft scoring: a purely additive layer over the hard-constraint kernel. A [`SoftConstraint`]
//! never affects validity — it only ranks *already-valid* solutions — so [`BestOfN`] can be
//! implemented entirely in terms of the public solver API (run N independent seeded solves, score
//! each, keep the best) without touching search internals.

use crate::wfc_engine::ids::{NodeId, PatternId};

// #region 🔖️SoftConstraint
/// 🎯️ Scores a complete assignment. Lower is not inherently better or worse — [`BestOfN::keep`]
/// decides the direction.
pub trait SoftConstraint {
    fn name(&self) -> &'static str;
    fn score(&self, assignment: &[PatternId]) -> f64;
}

/// 🎯️ A [`SoftConstraint`] built from a plain closure, for one-off scoring without a named type.
pub struct ScoreFn<F: Fn(&[PatternId]) -> f64> {
    pub name: &'static str,
    pub f: F,
}

impl<F: Fn(&[PatternId]) -> f64> SoftConstraint for ScoreFn<F> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn score(&self, assignment: &[PatternId]) -> f64 {
        (self.f)(assignment)
    }
}
// #endregion 🔖️SoftConstraint

// #region 🔖️BestOfN
/// 🎯️ Whether [`BestOfN`] keeps the highest- or lowest-scoring solution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BestOfNKeep {
    Highest,
    Lowest,
}

/// 🎯️ One scored solve attempt.
#[derive(Clone, Debug)]
pub struct Attempt {
    pub seed: u64,
    pub assignment: Vec<PatternId>,
    pub score: f64,
}

/// 🎯️ Runs `n` independent seeded attempts through a caller-supplied solve closure, scores each
/// successful one, and returns the best-scoring [`Attempt`] alongside every attempt's outcome (so
/// a caller can see how many of the `n` seeds actually found a solution at all).
///
/// 🚦️ De-dyn (O1/R11 open-set case): a caller's scorer is an open extension point (any
/// [`SoftConstraint`] impl, not a fixed set this crate enumerates), so this is the trivially-generic
/// parameter-position shape — `&dyn SoftConstraint` becomes `&S` — never an enum.
pub fn best_of_n<S: SoftConstraint>(base_seed: u64, n: u64, keep: BestOfNKeep, scorer: &S, mut solve_one: impl FnMut(u64) -> Option<Vec<PatternId>>) -> (Option<Attempt>, usize) {
    let mut best: Option<Attempt> = None;
    let mut solved_count = 0usize;
    for i in 0..n {
        let seed = base_seed.wrapping_add(i);
        let Some(assignment) = solve_one(seed) else { continue };
        solved_count += 1;
        let score = scorer.score(&assignment);
        let is_better = match &best {
            None => true,
            Some(b) => match keep {
                BestOfNKeep::Highest => score > b.score,
                BestOfNKeep::Lowest => score < b.score,
            },
        };
        if is_better {
            best = Some(Attempt { seed, assignment, score });
        }
    }
    (best, solved_count)
}
// #endregion 🔖️BestOfN

// #region 🔖️WeightField
/// 🎯️ A dense per-node multiplicative weight modifier, e.g. installed by a soft brush or a
/// soft-guided sampler. `1.0` everywhere is a no-op.
#[derive(Clone, Debug)]
pub struct WeightField {
    node_count: usize,
    pattern_count: usize,
    factors: Vec<f64>,
}

impl WeightField {
    pub fn identity(node_count: usize, pattern_count: usize) -> Self {
        Self { node_count, pattern_count, factors: vec![1.0; node_count * pattern_count] }
    }

    pub fn set(&mut self, n: NodeId, p: PatternId, factor: f64) {
        debug_assert!(factor.is_finite() && factor >= 0.0, "weight field factor must be finite and non-negative");
        self.factors[n.index() * self.pattern_count + p.index()] = factor;
    }

    pub fn get(&self, n: NodeId, p: PatternId) -> f64 {
        self.factors[n.index() * self.pattern_count + p.index()]
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }
}
// #endregion 🔖️WeightField

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
