//! 🎯 Soft scoring: a purely additive layer over the hard-constraint kernel. A [`SoftConstraint`]
//! never affects validity — it only ranks *already-valid* solutions — so [`BestOfN`] can be
//! implemented entirely in terms of the public solver API (run N independent seeded solves, score
//! each, keep the best) without touching search internals.

use crate::ids::{NodeId, PatternId};

// #region 🔖SoftConstraint
/// 🎯 Scores a complete assignment. Lower is not inherently better or worse — [`BestOfN::keep`]
/// decides the direction.
pub trait SoftConstraint {
    fn name(&self) -> &'static str;
    fn score(&self, assignment: &[PatternId]) -> f64;
}

/// 🎯 A [`SoftConstraint`] built from a plain closure, for one-off scoring without a named type.
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
// #endregion 🔖SoftConstraint

// #region 🔖BestOfN
/// 🎯 Whether [`BestOfN`] keeps the highest- or lowest-scoring solution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BestOfNKeep {
    Highest,
    Lowest,
}

/// 🎯 One scored solve attempt.
#[derive(Clone, Debug)]
pub struct Attempt {
    pub seed: u64,
    pub assignment: Vec<PatternId>,
    pub score: f64,
}

/// 🎯 Runs `n` independent seeded attempts through a caller-supplied solve closure, scores each
/// successful one, and returns the best-scoring [`Attempt`] alongside every attempt's outcome (so
/// a caller can see how many of the `n` seeds actually found a solution at all).
pub fn best_of_n(base_seed: u64, n: u64, keep: BestOfNKeep, scorer: &dyn SoftConstraint, mut solve_one: impl FnMut(u64) -> Option<Vec<PatternId>>) -> (Option<Attempt>, usize) {
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
// #endregion 🔖BestOfN

// #region 🔖WeightField
/// 🎯 A dense per-node multiplicative weight modifier, e.g. installed by a soft brush or a
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
// #endregion 🔖WeightField

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_of_n_keeps_the_highest_scoring_attempt() {
        let scorer = ScoreFn { name: "sum", f: |a: &[PatternId]| a.iter().map(|p| p.get() as f64).sum() };
        let (best, solved) = best_of_n(0, 5, BestOfNKeep::Highest, &scorer, |seed| Some(vec![PatternId(seed as u32 % 10)]));
        assert_eq!(solved, 5);
        let best = best.unwrap();
        assert_eq!(best.assignment, vec![PatternId(4)]); // seeds 0..5 -> patterns 0..5, max is 4
    }

    #[test]
    fn best_of_n_keeps_the_lowest_scoring_attempt() {
        let scorer = ScoreFn { name: "sum", f: |a: &[PatternId]| a.iter().map(|p| p.get() as f64).sum() };
        let (best, _) = best_of_n(0, 5, BestOfNKeep::Lowest, &scorer, |seed| Some(vec![PatternId(seed as u32 % 10)]));
        assert_eq!(best.unwrap().assignment, vec![PatternId(0)]);
    }

    #[test]
    fn best_of_n_skips_failed_attempts() {
        let scorer = ScoreFn { name: "const", f: |_: &[PatternId]| 0.0 };
        let (best, solved) = best_of_n(0, 5, BestOfNKeep::Highest, &scorer, |seed| if seed == 2 { Some(vec![PatternId(0)]) } else { None });
        assert_eq!(solved, 1);
        assert!(best.is_some());
    }

    #[test]
    fn best_of_n_returns_none_when_every_attempt_fails() {
        let scorer = ScoreFn { name: "const", f: |_: &[PatternId]| 0.0 };
        let (best, solved) = best_of_n(0, 3, BestOfNKeep::Highest, &scorer, |_| None);
        assert_eq!(solved, 0);
        assert!(best.is_none());
    }

    #[test]
    fn weight_field_identity_is_all_ones() {
        let field = WeightField::identity(2, 3);
        for n in 0..2 {
            for p in 0..3 {
                assert_eq!(field.get(NodeId::from_index(n), PatternId::from_index(p)), 1.0);
            }
        }
    }

    #[test]
    fn weight_field_set_and_get_roundtrip() {
        let mut field = WeightField::identity(2, 2);
        field.set(NodeId(0), PatternId(1), 2.5);
        assert_eq!(field.get(NodeId(0), PatternId(1)), 2.5);
        assert_eq!(field.get(NodeId(0), PatternId(0)), 1.0);
    }
}
// #endregion 🔖Tests
