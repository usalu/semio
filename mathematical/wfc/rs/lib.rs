//! 🌊 Wave Function Collapse as a finite-domain constraint solver: one propagation/search kernel
//! under three solvers (arbitrary graphs, dense 2D grids, dense 3D grids), with compiled
//! tile/pattern models, global constraints, and deterministic replayable search.

// #region 🔖Ids
pub mod ids {
//! 🔖 Typed integer newtype identifiers used throughout the crate. Kept as plain `u32` newtypes
//! (never raw `usize`) so pattern/tile/node/relation/constraint/decision/region/port indices can
//! never be silently swapped at a call site.

// #region 🔖Macro
macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub u32);

        impl $name {
            /// 🔖 The raw `u32` value.
            #[inline]
            pub const fn get(self) -> u32 {
                self.0
            }

            /// 🔖 The value as a `usize` index, for slice/vec indexing.
            #[inline]
            pub const fn index(self) -> usize {
                self.0 as usize
            }

            /// 🔖 Builds an id from a `usize` index (e.g. a loop counter). Truncates silently only
            /// if `i > u32::MAX`, which every builder in this crate rejects long before this point.
            #[inline]
            pub const fn from_index(i: usize) -> Self {
                Self(i as u32)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
// #endregion 🔖Macro

// #region 🔖Ids
id_newtype!(
    /// 🧩 One distinct pattern/tile value a variable can be assigned (the WFC "value").
    PatternId
);
id_newtype!(
    /// 🧱 A tile identity as authored (may map to several `PatternId`s under symmetry expansion).
    TileId
);
id_newtype!(
    /// 📍 One solver variable (grid cell or graph node). Distinct from `mathematical_graph::NodeId`
    /// (a `u64`); the only conversion boundary is `GraphTopology::from_graph_view`.
    NodeId
);
id_newtype!(
    /// ↔️ One directed compatibility relation (e.g. "north", "+X", or a graph edge label).
    RelationId
);
id_newtype!(
    /// 🧷 One registered global/soft constraint instance.
    ConstraintId
);
id_newtype!(
    /// 🌳 One search decision (a branch point in the backtracking tree).
    DecisionId
);
id_newtype!(
    /// 🗺️ One named region/zone used for scoped constraints and priorities.
    RegionId
);
id_newtype!(
    /// 🔌 One connector/socket slot on a tile or graph node.
    PortId
);
// #endregion 🔖Ids

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_index_roundtrip() {
        let p = PatternId::from_index(7);
        assert_eq!(p.index(), 7);
        assert_eq!(p.get(), 7);
        assert_eq!(format!("{p}"), "7");
    }

    #[test]
    fn id_ordering_and_equality() {
        let a = NodeId(1);
        let b = NodeId(2);
        assert!(a < b);
        assert_eq!(a, NodeId(1));
        assert_ne!(a, b);
    }

    #[test]
    fn id_serde_roundtrip() {
        let r = RelationId(42);
        let json = serde_json::to_string(&r).unwrap();
        let back: RelationId = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Ids

// #region 🔖Bitset
pub mod bitset {
//! 🎭 Hand-rolled dynamic bitset over `PatternId` — the WFC domain representation. Modeled on
//! `mathematical_sampling::TokenBitset` (word-packed `Vec<u64>`) with a solver-specific fused
//! restrict-and-collect operation used by every propagation engine's hot path.

use crate::ids::PatternId;

// #region 🔖Bitset
/// 🎭 A dynamic word-packed bitset over `0..len` pattern indices. `len` is the size of the
/// universe this set is defined over, not its popcount — use [`PatternSet::count_ones`] /
/// [`PatternSet::is_all_zero`] for cardinality, and [`PatternSet::is_empty_universe`] for the
/// degenerate zero-pattern-universe case.
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct PatternSet {
    words: Vec<u64>,
    len: usize,
}

impl PatternSet {
    /// 🎭 All-zero (empty) set over `len` patterns.
    pub fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭 All-one (full) set over `len` patterns.
    pub fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len);
        set.fill();
        set
    }

    /// 🎭 Number of patterns this set is defined over (not the popcount).
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty_universe(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, p: PatternId) -> bool {
        let idx = p.index();
        debug_assert!(idx < self.len);
        (self.words[idx / 64] >> (idx % 64)) & 1 != 0
    }

    #[inline]
    pub fn set(&mut self, p: PatternId, value: bool) {
        let idx = p.index();
        debug_assert!(idx < self.len);
        let mask = 1u64 << (idx % 64);
        if value {
            self.words[idx / 64] |= mask;
        } else {
            self.words[idx / 64] &= !mask;
        }
    }

    /// 🎭 Sets every bit `0..len` (trailing bits in the final word stay zero).
    pub fn fill(&mut self) {
        if self.len == 0 {
            return;
        }
        let full_words = self.len / 64;
        self.words[..full_words].fill(u64::MAX);
        let rem = self.len % 64;
        if rem > 0 {
            self.words[full_words] = (1u64 << rem) - 1;
        }
    }

    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭 In-place `self &= other`.
    pub fn and_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭 In-place `self |= other`.
    pub fn or_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭 In-place `self &= !other`.
    pub fn and_not_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }

    pub fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭 Lowest set bit, word-skipping past all-zero words.
    pub fn first_set(&self) -> Option<PatternId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(PatternId::from_index(word_idx * 64 + bit));
            }
        }
        None
    }

    /// 🎭 Iterates set bits in ascending order, skipping whole zero words at a time.
    pub fn iter_ones(&self) -> impl Iterator<Item = PatternId> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            let mut remaining = word;
            core::iter::from_fn(move || {
                if remaining == 0 {
                    return None;
                }
                let bit = remaining.trailing_zeros();
                remaining &= remaining - 1;
                Some(PatternId::from_index(word_idx * 64 + bit as usize))
            })
        })
    }

    /// 🎭 Read-only access to the backing words, e.g. for stable-hash fingerprinting.
    #[inline]
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    pub fn is_subset_of(&self, other: &PatternSet) -> bool {
        debug_assert_eq!(self.len, other.len);
        self.words.iter().zip(other.words.iter()).all(|(a, b)| a & !b == 0)
    }

    pub fn intersects(&self, other: &PatternSet) -> bool {
        debug_assert_eq!(self.len, other.len);
        self.words.iter().zip(other.words.iter()).any(|(a, b)| a & b != 0)
    }

    /// 🎭 Structural invariant check for data crossing a trust boundary (deserialization): word
    /// count matches `len`, and no stray bits are set past `len` in the final word. Every method
    /// above assumes this holds (e.g. `count_ones`/`iter_ones` would over-report, `set` would
    /// panic on an out-of-bounds word); a freshly built `PatternSet` always satisfies it, so this
    /// only needs calling on data this crate did not construct itself.
    pub fn is_well_formed(&self) -> bool {
        if self.words.len() != self.len.div_ceil(64) {
            return false;
        }
        let rem = self.len % 64;
        if rem != 0 {
            let mask = !((1u64 << rem) - 1);
            if self.words[self.words.len() - 1] & mask != 0 {
                return false;
            }
        }
        true
    }
    // #endregion 🔖Bitset

    // #region 🔖Ops
    /// 🎭 Fused restrict-and-collect: `removed_out = self & !allowed; self &= allowed`. Returns the
    /// number of bits actually cleared. The single fused pass this crate's hot loop needs — avoids
    /// computing the removed mask and the restricted set in two separate scans.
    pub fn restrict_returning_removed(&mut self, allowed: &PatternSet, removed_out: &mut PatternSet) -> u32 {
        debug_assert_eq!(self.len, allowed.len);
        debug_assert_eq!(self.len, removed_out.len);
        let mut removed = 0u32;
        for ((a, b), r) in self.words.iter_mut().zip(allowed.words.iter()).zip(removed_out.words.iter_mut()) {
            let cleared = *a & !*b;
            *r = cleared;
            removed += cleared.count_ones();
            *a &= *b;
        }
        removed
    }
    // #endregion 🔖Ops
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn from_indices(len: usize, idxs: &[usize]) -> PatternSet {
        let mut s = PatternSet::new_empty(len);
        for &i in idxs {
            s.set(PatternId::from_index(i), true);
        }
        s
    }

    #[test]
    fn empty_and_full() {
        let e = PatternSet::new_empty(10);
        assert!(e.is_all_zero());
        assert_eq!(e.count_ones(), 0);
        let f = PatternSet::new_full(10);
        assert_eq!(f.count_ones(), 10);
        for i in 0..10 {
            assert!(f.get(PatternId::from_index(i)));
        }
    }

    #[test]
    fn full_respects_boundary_bits() {
        // 70 patterns spans two words; the second word must not have stray set bits above 70.
        let f = PatternSet::new_full(70);
        assert_eq!(f.count_ones(), 70);
    }

    #[test]
    fn set_get_roundtrip() {
        let mut s = PatternSet::new_empty(5);
        s.set(PatternId::from_index(2), true);
        assert!(s.get(PatternId::from_index(2)));
        assert!(!s.get(PatternId::from_index(3)));
        s.set(PatternId::from_index(2), false);
        assert!(!s.get(PatternId::from_index(2)));
    }

    #[test]
    fn and_or_and_not() {
        let a = from_indices(8, &[0, 1, 2, 3]);
        let b = from_indices(8, &[2, 3, 4, 5]);

        let mut and_result = a.clone();
        and_result.and_with(&b);
        assert_eq!(and_result.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![2, 3]);

        let mut or_result = a.clone();
        or_result.or_with(&b);
        assert_eq!(or_result.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);

        let mut sub = a;
        sub.and_not_with(&b);
        assert_eq!(sub.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1]);
    }

    #[test]
    fn first_set_and_iter_ones_skip_zero_words() {
        let s = from_indices(200, &[130, 199]);
        assert_eq!(s.first_set().unwrap().index(), 130);
        assert_eq!(s.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![130, 199]);
    }

    #[test]
    fn subset_and_intersects() {
        let a = from_indices(8, &[0, 1]);
        let b = from_indices(8, &[0, 1, 2]);
        assert!(a.is_subset_of(&b));
        assert!(!b.is_subset_of(&a));
        assert!(a.intersects(&b));
        let c = from_indices(8, &[6, 7]);
        assert!(!a.intersects(&c));
    }

    #[test]
    fn restrict_returning_removed_matches_naive() {
        let mut s = from_indices(10, &[0, 1, 2, 3, 4]);
        let allowed = from_indices(10, &[2, 3, 5]);
        let mut removed = PatternSet::new_empty(10);
        let count = s.restrict_returning_removed(&allowed, &mut removed);
        assert_eq!(count, 3); // 0, 1, 4 removed
        assert_eq!(s.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(removed.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1, 4]);
    }

    #[test]
    fn restrict_no_change_returns_zero() {
        let mut s = from_indices(6, &[1, 2]);
        let allowed = PatternSet::new_full(6);
        let mut removed = PatternSet::new_empty(6);
        let count = s.restrict_returning_removed(&allowed, &mut removed);
        assert_eq!(count, 0);
        assert!(removed.is_all_zero());
    }

    #[test]
    fn freshly_built_sets_are_well_formed() {
        assert!(PatternSet::new_empty(0).is_well_formed());
        assert!(PatternSet::new_empty(70).is_well_formed());
        assert!(PatternSet::new_full(70).is_well_formed());
        assert!(from_indices(200, &[130, 199]).is_well_formed());
    }

    #[test]
    fn wrong_word_count_is_not_well_formed() {
        let mut s = from_indices(70, &[10]);
        s.words.push(0); // one extra word beyond what 70 patterns needs
        assert!(!s.is_well_formed());
    }

    #[test]
    fn stray_bits_past_len_in_final_word_are_not_well_formed() {
        let mut s = from_indices(10, &[2]);
        s.words[0] |= 1 << 20; // bit 20 is past `len = 10`, still within the single backing word
        assert!(!s.is_well_formed());
    }

    #[test]
    fn serde_round_trip_preserves_bits_and_len() {
        let s = from_indices(70, &[3, 64, 69]);
        let json = serde_json::to_string(&s).unwrap();
        let back: PatternSet = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
        assert!(back.is_well_formed());
    }

    mod quick {
        use super::*;

        #[test]
        fn random_and_or_matches_vec_bool_model() {
            let mut rng = mathematical_random::Rng::from_seed(12345);
            for _ in 0..200 {
                let len = 1 + (rng.next_range(0, 200) as usize);
                let mut model_a = vec![false; len];
                let mut model_b = vec![false; len];
                let mut a = PatternSet::new_empty(len);
                let mut b = PatternSet::new_empty(len);
                for i in 0..len {
                    if rng.next_bool(0.5) {
                        model_a[i] = true;
                        a.set(PatternId::from_index(i), true);
                    }
                    if rng.next_bool(0.5) {
                        model_b[i] = true;
                        b.set(PatternId::from_index(i), true);
                    }
                }
                let mut and_r = a.clone();
                and_r.and_with(&b);
                for i in 0..len {
                    assert_eq!(and_r.get(PatternId::from_index(i)), model_a[i] && model_b[i]);
                }
                let mut or_r = a.clone();
                or_r.or_with(&b);
                for i in 0..len {
                    assert_eq!(or_r.get(PatternId::from_index(i)), model_a[i] || model_b[i]);
                }
                let expected_count = model_a.iter().filter(|&&x| x).count() as u32;
                assert_eq!(a.count_ones(), expected_count);
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Bitset

// #region 🔖Weights
pub mod weights {
//! ⚖️ Pattern weight storage: precomputed `w`, `ln w`, `w·ln w` per pattern (the three terms the
//! incremental Shannon-entropy heuristic needs at O(1) per update), plus an optional exact-integer
//! parallel table for [`WeightMode::StrictInteger`] determinism.

use crate::error::ModelError;
use crate::ids::PatternId;

// #region 🔖Mode
/// ⚖️ Whether heuristics/sampling read `f64` weights (fast, platform-stable but not
/// refactor-proof) or exact `u64` weights (slower, bit-for-bit reproducible everywhere — used by
/// every differential/golden-replay test in this crate).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WeightMode {
    #[default]
    Real,
    StrictInteger,
}

/// ⚖️ What a weight of exactly zero means for sampling and pruning.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ZeroWeightPolicy {
    /// ⚖️ Zero weight is a validation error at compile time.
    Reject,
    /// ⚖️ Zero-weight patterns stay in the domain (may be forced) but are never sampled unless
    /// they are the only remaining candidate.
    #[default]
    SampleNeverUnlessForced,
    /// ⚖️ Zero weight is treated exactly like an explicit deny — the pattern is compiled out.
    Forbidden,
}
// #endregion 🔖Mode

// #region 🔖Weights
/// ⚖️ Per-pattern weight table with precomputed entropy terms.
#[derive(Clone, Debug)]
pub struct WeightTable {
    w: Vec<f64>,
    ln_w: Vec<f64>,
    w_ln_w: Vec<f64>,
    w_int: Option<Vec<u64>>,
}

impl WeightTable {
    /// ⚖️ Builds a table from raw positive-finite weights. `w_int` is populated only when every
    /// weight is already an exact non-negative integer value (the common case for hand-authored
    /// tilesets and frequency-counted extraction).
    pub fn new(weights: &[f64]) -> Result<Self, ModelError> {
        let mut w = Vec::with_capacity(weights.len());
        let mut ln_w = Vec::with_capacity(weights.len());
        let mut w_ln_w = Vec::with_capacity(weights.len());
        let mut all_integral = true;
        let mut w_int = Vec::with_capacity(weights.len());
        for (i, &value) in weights.iter().enumerate() {
            if !value.is_finite() || value < 0.0 {
                return Err(ModelError::InvalidWeight { pattern_index: i, value });
            }
            w.push(value);
            let lnv = if value > 0.0 { value.ln() } else { 0.0 };
            ln_w.push(lnv);
            w_ln_w.push(if value > 0.0 { value * lnv } else { 0.0 });
            if value.fract() == 0.0 && value >= 0.0 && value <= u64::MAX as f64 {
                w_int.push(value as u64);
            } else {
                all_integral = false;
            }
        }
        Ok(Self { w, ln_w, w_ln_w, w_int: if all_integral { Some(w_int) } else { None } })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.w.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.w.is_empty()
    }

    #[inline]
    pub fn w(&self, p: PatternId) -> f64 {
        self.w[p.index()]
    }

    #[inline]
    pub fn ln_w(&self, p: PatternId) -> f64 {
        self.ln_w[p.index()]
    }

    #[inline]
    pub fn w_ln_w(&self, p: PatternId) -> f64 {
        self.w_ln_w[p.index()]
    }

    /// ⚖️ Exact integer weight, when [`WeightTable::has_integer_weights`] is `true`.
    #[inline]
    pub fn w_int(&self, p: PatternId) -> Option<u64> {
        self.w_int.as_ref().map(|v| v[p.index()])
    }

    pub fn has_integer_weights(&self) -> bool {
        self.w_int.is_some()
    }

    /// ⚖️ `(sum_w, sum_w_ln_w)` restricted to the patterns present in `set`. O(domain size); used
    /// only to rebuild caches from scratch (initialization, periodic drift correction, debug
    /// verification) — never on the hot incremental path.
    pub fn sum_over(&self, set: &crate::bitset::PatternSet) -> (f64, f64) {
        let mut sum_w = 0.0;
        let mut sum_w_ln_w = 0.0;
        for p in set.iter_ones() {
            sum_w += self.w(p);
            sum_w_ln_w += self.w_ln_w(p);
        }
        (sum_w, sum_w_ln_w)
    }

    /// ⚖️ Exact-integer analogue of [`WeightTable::sum_over`], `None` if this table lacks integer
    /// weights.
    pub fn sum_int_over(&self, set: &crate::bitset::PatternSet) -> Option<u64> {
        self.w_int.as_ref().map(|w_int| set.iter_ones().map(|p| w_int[p.index()]).sum())
    }
}
// #endregion 🔖Weights

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitset::PatternSet;

    #[test]
    fn rejects_negative_and_nonfinite() {
        assert!(WeightTable::new(&[1.0, -1.0]).is_err());
        assert!(WeightTable::new(&[1.0, f64::NAN]).is_err());
        assert!(WeightTable::new(&[1.0, f64::INFINITY]).is_err());
    }

    #[test]
    fn precomputes_terms() {
        let t = WeightTable::new(&[1.0, core::f64::consts::E]).unwrap();
        assert_eq!(t.w(PatternId(0)), 1.0);
        assert!((t.ln_w(PatternId(1)) - 1.0).abs() < 1e-12);
        assert!((t.w_ln_w(PatternId(1)) - core::f64::consts::E).abs() < 1e-9);
    }

    #[test]
    fn zero_weight_terms_are_zero_not_nan() {
        let t = WeightTable::new(&[0.0, 2.0]).unwrap();
        assert_eq!(t.ln_w(PatternId(0)), 0.0);
        assert_eq!(t.w_ln_w(PatternId(0)), 0.0);
    }

    #[test]
    fn integer_detection() {
        let t = WeightTable::new(&[1.0, 3.0, 5.0]).unwrap();
        assert!(t.has_integer_weights());
        assert_eq!(t.w_int(PatternId(1)), Some(3));

        let f = WeightTable::new(&[1.5, 2.0]).unwrap();
        assert!(!f.has_integer_weights());
    }

    #[test]
    fn sum_over_matches_manual() {
        let t = WeightTable::new(&[1.0, 2.0, 4.0]).unwrap();
        let mut set = PatternSet::new_empty(3);
        set.set(PatternId(0), true);
        set.set(PatternId(2), true);
        let (sw, swlw) = t.sum_over(&set);
        assert_eq!(sw, 5.0);
        assert!((swlw - (0.0 + 4.0 * 4.0f64.ln())).abs() < 1e-12);
        assert_eq!(t.sum_int_over(&set), Some(5));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Weights

// #region 🔖Error
pub mod error {
//! 🚨 Every way building a model/topology/constraint or configuring a solve can fail validation.
//! Kept flat (no nested `source()` chain, no external error crate) so callers can match
//! exhaustively — the entropy crate's convention. `Contradiction`/`Unsatisfiable` are normal
//! [`crate::outcome::SolveOutcome`] variants, never errors: a search finding no solution is not a
//! bug, a malformed model or an internal invariant violation is.

// #region 🔖ModelError
/// 🚨 Everything that can go wrong while building or compiling a [`crate::model::CompiledModel`].
#[derive(Clone, PartialEq, Debug)]
pub enum ModelError {
    /// 🚨 A model was compiled with zero patterns.
    EmptyPatternUniverse,
    /// 🚨 A `PatternId`/`TileId`/`RelationId`/`PortId` referenced during building was never added.
    UnknownPattern(crate::ids::PatternId),
    UnknownTile(crate::ids::TileId),
    UnknownRelation(crate::ids::RelationId),
    UnknownPort(crate::ids::PortId),
    /// 🚨 The same relation name/id was registered twice.
    DuplicateRelation(crate::ids::RelationId),
    /// 🚨 A weight failed validation (`NaN`, infinite, or negative).
    InvalidWeight { pattern_index: usize, value: f64 },
    /// 🚨 `allowed[r][a].get(b) != allowed[inv(r)][b].get(a)` — the declared inverse relation is
    /// not actually the transpose of the forward relation's compatibility table.
    AsymmetricInverse { relation: crate::ids::RelationId },
    /// 🚨 A checked multiplication/addition needed to size an internal table overflowed.
    CapacityOverflow { what: &'static str },
    /// 🚨 A symmetry transform did not close under composition/inverse (generator set is broken).
    InvalidSymmetryGroup { reason: &'static str },
    /// 🚨 A socket rule referenced a socket label that was never declared compatible with anything.
    IncompatibleSocketRule { reason: &'static str },
    /// 🚨 A [`crate::serial::SourceModelDoc`]'s schema version does not match this build's. No
    /// migration — this crate has no users yet, so an unrecognized version is simply rejected.
    SchemaVersionMismatch { expected: u32, actual: u32 },
}

impl core::fmt::Display for ModelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyPatternUniverse => write!(f, "model has zero patterns"),
            Self::UnknownPattern(p) => write!(f, "unknown pattern id {p}"),
            Self::UnknownTile(t) => write!(f, "unknown tile id {t}"),
            Self::UnknownRelation(r) => write!(f, "unknown relation id {r}"),
            Self::UnknownPort(p) => write!(f, "unknown port id {p}"),
            Self::DuplicateRelation(r) => write!(f, "relation {r} registered twice"),
            Self::InvalidWeight { pattern_index, value } => {
                write!(f, "invalid weight at pattern index {pattern_index}: {value}")
            }
            Self::AsymmetricInverse { relation } => {
                write!(f, "relation {relation} and its declared inverse disagree on compatibility (not a true transpose)")
            }
            Self::CapacityOverflow { what } => write!(f, "capacity overflow computing {what}"),
            Self::InvalidSymmetryGroup { reason } => write!(f, "invalid symmetry group: {reason}"),
            Self::IncompatibleSocketRule { reason } => write!(f, "incompatible socket rule: {reason}"),
            Self::SchemaVersionMismatch { expected, actual } => {
                write!(f, "source model schema version mismatch: expected {expected}, found {actual}")
            }
        }
    }
}

impl std::error::Error for ModelError {}
// #endregion 🔖ModelError

// #region 🔖TopologyError
/// 🚨 Everything that can go wrong while building a grid or graph topology.
#[derive(Clone, PartialEq, Debug)]
pub enum TopologyError {
    /// 🚨 A grid dimension was zero where the topology forbids it.
    ZeroDimension { axis: &'static str },
    /// 🚨 `width * height` (or `* depth`) overflowed its checked integer type.
    SizeOverflow,
    /// 🚨 A mask's length did not match `width * height` (`* depth`).
    MaskShapeMismatch { expected: usize, actual: usize },
    /// 🚨 A referenced `NodeId` is out of range for this topology.
    UnknownNode(crate::ids::NodeId),
    /// 🚨 An arc referenced a node that does not exist (e.g. after `from_graph_view` truncation).
    DanglingArc { from: crate::ids::NodeId },
    /// 🚨 A custom stencil declared the same offset twice, or a self-offset without opting in.
    InvalidStencil { reason: &'static str },
    /// 🚨 A boundary mode is incompatible with the requested grid size (e.g. `Mirror` on a
    /// size-0 axis) or with another configured boundary on the same axis.
    BoundaryIncompatible { reason: &'static str },
    /// 🚨 A node count exceeded `u32::MAX`, the limit `crate::ids::NodeId` can address.
    TooManyNodes { count: u64 },
}

impl core::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroDimension { axis } => write!(f, "grid dimension `{axis}` must be nonzero"),
            Self::SizeOverflow => write!(f, "grid size computation overflowed"),
            Self::MaskShapeMismatch { expected, actual } => {
                write!(f, "mask length mismatch: expected {expected}, found {actual}")
            }
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
            Self::DanglingArc { from } => write!(f, "arc references a nonexistent node from {from}"),
            Self::InvalidStencil { reason } => write!(f, "invalid stencil: {reason}"),
            Self::BoundaryIncompatible { reason } => write!(f, "incompatible boundary configuration: {reason}"),
            Self::TooManyNodes { count } => write!(f, "{count} nodes exceeds the u32 node-id capacity"),
        }
    }
}

impl std::error::Error for TopologyError {}
// #endregion 🔖TopologyError

// #region 🔖ConstraintError
/// 🚨 Everything that can go wrong while configuring a global/soft constraint.
#[derive(Clone, PartialEq, Debug)]
pub enum ConstraintError {
    /// 🚨 A cardinality/distance bound was internally inconsistent (e.g. `min > max`).
    InvalidBounds { reason: &'static str },
    /// 🚨 A referenced region/tag was never declared.
    UnknownRegion(crate::ids::RegionId),
    UnknownTag(u32),
    /// 🚨 A tuple-table constraint was given zero tuples.
    EmptyTupleTable,
    /// 🚨 A tuple in a tuple-table constraint did not match the declared node-scope arity.
    ArityMismatch { expected: usize, actual: usize },
    /// 🚨 A constraint referenced a node outside the topology.
    UnknownNode(crate::ids::NodeId),
}

impl core::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBounds { reason } => write!(f, "invalid constraint bounds: {reason}"),
            Self::UnknownRegion(r) => write!(f, "unknown region id {r}"),
            Self::UnknownTag(t) => write!(f, "unknown tag id {t}"),
            Self::EmptyTupleTable => write!(f, "tuple-table constraint has zero tuples"),
            Self::ArityMismatch { expected, actual } => {
                write!(f, "tuple arity mismatch: expected {expected}, found {actual}")
            }
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
        }
    }
}

impl std::error::Error for ConstraintError {}
// #endregion 🔖ConstraintError

// #region 🔖SolveError
/// 🚨 Everything that can go wrong configuring or resuming a solve (as opposed to the solve
/// itself finding no solution, which is a [`crate::outcome::SolveOutcome`]).
#[derive(Clone, PartialEq, Debug)]
pub enum SolveError {
    /// 🚨 A solver was built from a model and topology whose relation universes disagree.
    ModelTopologyMismatch { reason: &'static str },
    /// 🚨 Strict-integer determinism was requested but the model has no integer weight table.
    SeedMissingInStrictMode,
    /// 🚨 A checkpoint's format/schema version does not match this build.
    CheckpointVersionMismatch { expected: u32, actual: u32 },
    /// 🚨 A checkpoint failed structural revalidation (bitset length, index bound, or fingerprint).
    CorruptCheckpoint { reason: &'static str },
    /// 🚨 A fixed pattern/domain restriction was given for a node outside the topology.
    UnknownNode(crate::ids::NodeId),
}

impl core::fmt::Display for SolveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ModelTopologyMismatch { reason } => write!(f, "model/topology mismatch: {reason}"),
            Self::SeedMissingInStrictMode => write!(f, "strict-integer mode requires an all-integer weight table"),
            Self::CheckpointVersionMismatch { expected, actual } => {
                write!(f, "checkpoint version mismatch: expected {expected}, found {actual}")
            }
            Self::CorruptCheckpoint { reason } => write!(f, "corrupt checkpoint: {reason}"),
            Self::UnknownNode(n) => write!(f, "unknown node id {n}"),
        }
    }
}

impl std::error::Error for SolveError {}
// #endregion 🔖SolveError

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_are_human_readable() {
        let e = ModelError::InvalidWeight { pattern_index: 3, value: -1.0 };
        assert_eq!(e.to_string(), "invalid weight at pattern index 3: -1");

        let t = TopologyError::ZeroDimension { axis: "width" };
        assert_eq!(t.to_string(), "grid dimension `width` must be nonzero");

        let c = ConstraintError::EmptyTupleTable;
        assert_eq!(c.to_string(), "tuple-table constraint has zero tuples");

        let s = SolveError::CheckpointVersionMismatch { expected: 1, actual: 2 };
        assert_eq!(s.to_string(), "checkpoint version mismatch: expected 1, found 2");
    }

    #[test]
    fn errors_are_std_error() {
        fn assert_std_error<E: std::error::Error>(_e: &E) {}
        assert_std_error(&ModelError::EmptyPatternUniverse);
        assert_std_error(&TopologyError::SizeOverflow);
        assert_std_error(&ConstraintError::EmptyTupleTable);
        assert_std_error(&SolveError::SeedMissingInStrictMode);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Error

// #region 🔖Domain
pub mod domain {
//! 📦 A variable's live domain: which patterns remain possible, plus cached weight sums so the
//! weighted-entropy heuristic reads in O(1). Every mutation returns a [`RestrictResult`] so
//! callers (propagation, search) can react to wipeouts/singletons without a second query.

use crate::bitset::PatternSet;
use crate::ids::PatternId;
use crate::weights::WeightTable;

// #region 🔖Result
/// 📦 What happened to a [`Domain`] after a mutating operation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RestrictResult {
    /// 📦 No pattern was removed.
    Unchanged,
    /// 📦 `n` patterns were removed; more than one remains.
    Reduced(u32),
    /// 📦 Exactly one pattern remains.
    Singleton(PatternId),
    /// 📦 Zero patterns remain — a contradiction at this variable.
    Wipeout,
}
// #endregion 🔖Result

// #region 🔖Domain
/// ⏱️ How many revisions between exact cache resyncs, bounding `f64` subtraction drift without
/// paying `O(domain size)` on every single removal.
const RESYNC_INTERVAL: u64 = 4096;

/// 📦 One variable's live domain plus incrementally-maintained weight/entropy caches.
#[derive(Clone, Debug)]
pub struct Domain {
    bits: PatternSet,
    cardinality: u32,
    sum_w: f64,
    sum_w_ln_w: f64,
    sum_w_int: Option<u64>,
    revision: u64,
}

impl Domain {
    /// 📦 The full domain (every pattern in `w` possible), with caches seeded from `w`'s totals.
    pub fn new_full(w: &WeightTable) -> Self {
        let bits = PatternSet::new_full(w.len());
        let sum_w = (0..w.len()).map(|i| w.w(PatternId::from_index(i))).sum();
        let sum_w_ln_w = (0..w.len()).map(|i| w.w_ln_w(PatternId::from_index(i))).sum();
        let sum_w_int = w.has_integer_weights().then(|| (0..w.len()).filter_map(|i| w.w_int(PatternId::from_index(i))).sum());
        Self { bits, cardinality: w.len() as u32, sum_w, sum_w_ln_w, sum_w_int, revision: 0 }
    }

    /// 📦 An explicitly-restricted starting domain (e.g. a per-node initial mask); caches are
    /// computed exactly from `allowed` and `w` (no assumption `allowed` came from a full domain).
    pub fn new_restricted(allowed: &PatternSet, w: &WeightTable) -> Self {
        let (sum_w, sum_w_ln_w) = w.sum_over(allowed);
        let sum_w_int = w.has_integer_weights().then(|| w.sum_int_over(allowed).unwrap_or(0));
        Self { bits: allowed.clone(), cardinality: allowed.count_ones(), sum_w, sum_w_ln_w, sum_w_int, revision: 0 }
    }

    #[inline]
    pub fn bits(&self) -> &PatternSet {
        &self.bits
    }

    #[inline]
    pub fn cardinality(&self) -> u32 {
        self.cardinality
    }

    #[inline]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    #[inline]
    pub fn is_wiped(&self) -> bool {
        self.cardinality == 0
    }

    /// 📦 `Some(pattern)` iff this domain is a singleton.
    pub fn singleton(&self) -> Option<PatternId> {
        if self.cardinality == 1 {
            self.bits.first_set()
        } else {
            None
        }
    }

    #[inline]
    pub fn sum_w(&self) -> f64 {
        self.sum_w
    }

    #[inline]
    pub fn sum_w_int(&self) -> Option<u64> {
        self.sum_w_int
    }

    /// 📊 Incremental weighted Shannon entropy in nats: `ln(Σw) - Σ(w·ln w)/Σw`. `0.0` for an
    /// empty or singleton domain (a determined or contradictory variable carries no uncertainty).
    pub fn entropy(&self) -> f64 {
        if self.sum_w <= 0.0 {
            return 0.0;
        }
        self.sum_w.ln() - self.sum_w_ln_w / self.sum_w
    }

    fn apply_removed(&mut self, removed: &PatternSet, w: &WeightTable) -> u32 {
        let cleared = removed.count_ones();
        if cleared == 0 {
            return 0;
        }
        for p in removed.iter_ones() {
            self.sum_w -= w.w(p);
            self.sum_w_ln_w -= w.w_ln_w(p);
            if let (Some(sum_int), Some(wi)) = (self.sum_w_int.as_mut(), w.w_int(p)) {
                *sum_int = sum_int.saturating_sub(wi);
            }
        }
        self.cardinality -= cleared;
        self.revision += 1;
        self.maybe_resync(w);
        cleared
    }

    fn maybe_resync(&mut self, w: &WeightTable) {
        if self.revision.is_multiple_of(RESYNC_INTERVAL) {
            let (sw, swlw) = w.sum_over(&self.bits);
            self.sum_w = sw;
            self.sum_w_ln_w = swlw;
            if self.sum_w_int.is_some() {
                self.sum_w_int = w.sum_int_over(&self.bits);
            }
        }
    }

    /// 📦 Classifies the post-mutation state, given how many patterns this specific operation
    /// removed (only the caller knows that count — `self.cardinality` alone cannot distinguish
    /// "removed 1 of 4" from "removed 3 of 4" when both leave the same remaining count).
    fn result_for(&self, removed_count: u32) -> RestrictResult {
        match self.cardinality {
            0 => RestrictResult::Wipeout,
            1 => RestrictResult::Singleton(self.bits.first_set().expect("cardinality 1 domain must have a set bit")),
            _ => RestrictResult::Reduced(removed_count),
        }
    }

    /// 📦 Intersects with `allowed`, collecting the removed-pattern mask into `removed_out`
    /// (caller-supplied to avoid a per-call allocation on the propagation hot path).
    pub fn restrict_collecting(&mut self, allowed: &PatternSet, w: &WeightTable, removed_out: &mut PatternSet) -> RestrictResult {
        let cleared = self.bits.restrict_returning_removed(allowed, removed_out);
        if cleared == 0 {
            return RestrictResult::Unchanged;
        }
        self.apply_removed(removed_out, w);
        self.result_for(cleared)
    }

    /// 📦 Convenience over [`Domain::restrict_collecting`] that allocates its own scratch buffer.
    pub fn restrict(&mut self, allowed: &PatternSet, w: &WeightTable) -> RestrictResult {
        let mut removed = PatternSet::new_empty(self.bits.len());
        self.restrict_collecting(allowed, w, &mut removed)
    }

    /// 📦 Removes exactly one pattern (a no-op if it is already absent).
    pub fn remove(&mut self, p: PatternId, w: &WeightTable) -> RestrictResult {
        if !self.bits.get(p) {
            return RestrictResult::Unchanged;
        }
        self.bits.set(p, false);
        self.sum_w -= w.w(p);
        self.sum_w_ln_w -= w.w_ln_w(p);
        if let (Some(sum_int), Some(wi)) = (self.sum_w_int.as_mut(), w.w_int(p)) {
            *sum_int = sum_int.saturating_sub(wi);
        }
        self.cardinality -= 1;
        self.revision += 1;
        self.maybe_resync(w);
        self.result_for(1)
    }

    /// 📦 Forces this domain to exactly `{p}` — the WFC "observe" operation. Collects every
    /// removed pattern into `removed_out` (used by the trail and by AC-4's decrement fan-out).
    pub fn assign_collecting(&mut self, p: PatternId, w: &WeightTable, removed_out: &mut PatternSet) -> RestrictResult {
        let mut singleton = PatternSet::new_empty(self.bits.len());
        singleton.set(p, true);
        self.restrict_collecting(&singleton, w, removed_out)
    }

    /// 📦 Convenience over [`Domain::assign_collecting`] that allocates its own scratch buffer.
    pub fn assign(&mut self, p: PatternId, w: &WeightTable) -> RestrictResult {
        let mut removed = PatternSet::new_empty(self.bits.len());
        self.assign_collecting(p, w, &mut removed)
    }

    /// ↩️ Trail-undo primitive: re-adds a single previously-removed pattern. Exact inverse of one
    /// bit flip inside [`Domain::remove`]/[`Domain::restrict_collecting`] — never resyncs (the
    /// caller is replaying a trail in exact reverse order, so incremental restoration is exact).
    pub fn re_add(&mut self, p: PatternId, w: &WeightTable) {
        debug_assert!(!self.bits.get(p), "re_add: pattern was not actually removed");
        self.bits.set(p, true);
        self.sum_w += w.w(p);
        self.sum_w_ln_w += w.w_ln_w(p);
        if let (Some(sum_int), Some(wi)) = (self.sum_w_int.as_mut(), w.w_int(p)) {
            *sum_int += wi;
        }
        self.cardinality += 1;
        self.revision += 1;
    }

    /// 🩺 Debug-only: recomputes every cache from `bits` and asserts it matches the incremental
    /// value within tolerance. Called by tests and by callers wanting an expensive sanity pass.
    pub fn debug_assert_consistent(&self, w: &WeightTable) {
        let recomputed_card = self.bits.count_ones();
        assert_eq!(self.cardinality, recomputed_card, "cardinality cache drifted");
        let (sw, swlw) = w.sum_over(&self.bits);
        assert!((self.sum_w - sw).abs() < 1e-6, "sum_w cache drifted: cached {} vs recomputed {}", self.sum_w, sw);
        assert!((self.sum_w_ln_w - swlw).abs() < 1e-6, "sum_w_ln_w cache drifted: cached {} vs recomputed {}", self.sum_w_ln_w, swlw);
        if let Some(sum_int) = self.sum_w_int {
            assert_eq!(Some(sum_int), w.sum_int_over(&self.bits), "sum_w_int cache drifted");
        }
    }
}
// #endregion 🔖Domain

// #region 🔖Store
/// 📦 One [`Domain`] per solver variable, stored contiguously (struct-of-arrays friendly).
#[derive(Clone, Debug)]
pub struct DomainStore {
    domains: Vec<Domain>,
}

impl DomainStore {
    pub fn new_full(node_count: usize, w: &WeightTable) -> Self {
        Self { domains: (0..node_count).map(|_| Domain::new_full(w)).collect() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.domains.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.domains.is_empty()
    }

    #[inline]
    pub fn get(&self, n: crate::ids::NodeId) -> &Domain {
        &self.domains[n.index()]
    }

    #[inline]
    pub fn get_mut(&mut self, n: crate::ids::NodeId) -> &mut Domain {
        &mut self.domains[n.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (crate::ids::NodeId, &Domain)> {
        self.domains.iter().enumerate().map(|(i, d)| (crate::ids::NodeId::from_index(i), d))
    }

    pub fn all_singleton(&self) -> bool {
        self.domains.iter().all(|d| d.cardinality() == 1)
    }

    pub fn any_wiped(&self) -> bool {
        self.domains.iter().any(|d| d.is_wiped())
    }

    pub fn debug_assert_consistent(&self, w: &WeightTable) {
        for d in &self.domains {
            d.debug_assert_consistent(w);
        }
    }
}
// #endregion 🔖Store

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> WeightTable {
        WeightTable::new(&[1.0, 2.0, 4.0, 8.0]).unwrap()
    }

    #[test]
    fn new_full_has_all_patterns_and_correct_sums() {
        let w = table();
        let d = Domain::new_full(&w);
        assert_eq!(d.cardinality(), 4);
        assert_eq!(d.sum_w(), 15.0);
        assert_eq!(d.sum_w_int(), Some(15));
        d.debug_assert_consistent(&w);
    }

    #[test]
    fn restrict_reduces_and_updates_caches() {
        let w = table();
        let mut d = Domain::new_full(&w);
        let mut allowed = PatternSet::new_empty(4);
        allowed.set(PatternId(0), true);
        allowed.set(PatternId(2), true);
        let result = d.restrict(&allowed, &w);
        assert_eq!(result, RestrictResult::Reduced(2));
        assert_eq!(d.cardinality(), 2);
        assert_eq!(d.sum_w(), 5.0);
        d.debug_assert_consistent(&w);
    }

    #[test]
    fn restrict_unchanged_when_already_subset() {
        let w = table();
        let mut d = Domain::new_full(&w);
        let mut allowed = PatternSet::new_empty(4);
        allowed.set(PatternId(0), true);
        d.assign(PatternId(0), &w);
        let result = d.restrict(&allowed, &w);
        assert_eq!(result, RestrictResult::Unchanged);
    }

    #[test]
    fn reduced_count_is_removed_not_remaining() {
        // 6 patterns, remove exactly 1 -> remaining 5. Reduced(_) must report 1, not 5.
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut d = Domain::new_full(&w);
        let result = d.remove(PatternId(0), &w);
        assert_eq!(result, RestrictResult::Reduced(1));
        assert_eq!(d.cardinality(), 5);

        // restrict from 6 down to 3 (remove 3) -> remaining 3. Reduced(_) must report 3.
        let mut d2 = Domain::new_full(&w);
        let mut allowed = PatternSet::new_empty(6);
        allowed.set(PatternId(0), true);
        allowed.set(PatternId(1), true);
        allowed.set(PatternId(2), true);
        let result2 = d2.restrict(&allowed, &w);
        assert_eq!(result2, RestrictResult::Reduced(3));
        assert_eq!(d2.cardinality(), 3);
    }

    #[test]
    fn remove_to_singleton_and_wipeout() {
        let w = WeightTable::new(&[1.0, 1.0]).unwrap();
        let mut d = Domain::new_full(&w);
        let r1 = d.remove(PatternId(0), &w);
        assert_eq!(r1, RestrictResult::Singleton(PatternId(1)));
        let r2 = d.remove(PatternId(1), &w);
        assert_eq!(r2, RestrictResult::Wipeout);
        assert!(d.is_wiped());
    }

    #[test]
    fn assign_forces_single_pattern() {
        let w = table();
        let mut d = Domain::new_full(&w);
        let result = d.assign(PatternId(2), &w);
        assert_eq!(result, RestrictResult::Singleton(PatternId(2)));
        assert_eq!(d.singleton(), Some(PatternId(2)));
        assert_eq!(d.sum_w(), 4.0);
        d.debug_assert_consistent(&w);
    }

    #[test]
    fn re_add_exactly_reverses_remove() {
        let w = table();
        let mut d = Domain::new_full(&w);
        d.remove(PatternId(1), &w);
        assert_eq!(d.cardinality(), 3);
        d.re_add(PatternId(1), &w);
        assert_eq!(d.cardinality(), 4);
        assert_eq!(d.sum_w(), 15.0);
        d.debug_assert_consistent(&w);
    }

    #[test]
    fn singleton_has_zero_entropy_regardless_of_weight() {
        let w = WeightTable::new(&[1.0, 100.0]).unwrap();
        let mut d = Domain::new_full(&w);
        d.assign(PatternId(1), &w);
        assert!(d.entropy().abs() < 1e-9);
    }

    #[test]
    fn wiped_domain_has_zero_entropy() {
        let w = WeightTable::new(&[1.0]).unwrap();
        let mut d = Domain::new_full(&w);
        d.remove(PatternId(0), &w);
        assert_eq!(d.entropy(), 0.0);
    }

    #[test]
    fn uniform_weights_entropy_matches_ln_cardinality() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let d = Domain::new_full(&w);
        assert!((d.entropy() - 4.0f64.ln()).abs() < 1e-9);
    }

    #[test]
    fn domain_store_all_singleton() {
        let w = table();
        let mut store = DomainStore::new_full(2, &w);
        assert!(!store.all_singleton());
        store.get_mut(crate::ids::NodeId(0)).assign(PatternId(0), &w);
        store.get_mut(crate::ids::NodeId(1)).assign(PatternId(1), &w);
        assert!(store.all_singleton());
        assert!(!store.any_wiped());
    }

    mod quick {
        use super::*;

        #[test]
        fn random_remove_re_add_sequences_preserve_invariants() {
            let w = WeightTable::new(&[1.0, 3.0, 5.0, 2.0, 7.0, 1.0, 9.0, 4.0]).unwrap();
            let mut rng = mathematical_random::Rng::from_seed(999);
            for _ in 0..100 {
                let mut d = Domain::new_full(&w);
                let mut removed_stack: Vec<PatternId> = Vec::new();
                for _ in 0..50 {
                    if d.cardinality() > 1 && rng.next_bool(0.7) {
                        let idx = rng.next_range(0, w.len() as u64) as usize;
                        let p = PatternId::from_index(idx);
                        if d.bits().get(p) {
                            d.remove(p, &w);
                            removed_stack.push(p);
                        }
                    } else if let Some(p) = removed_stack.pop() {
                        d.re_add(p, &w);
                    }
                    d.debug_assert_consistent(&w);
                }
            }
        }

        #[test]
        fn resync_boundary_does_not_change_observable_state() {
            let w = WeightTable::new(&(0..70).map(|i| 1.0 + i as f64).collect::<Vec<_>>()).unwrap();
            let mut d = Domain::new_full(&w);
            // Cross the RESYNC_INTERVAL boundary via single-pattern removals on a large domain.
            for i in 0..70u32 {
                if i % 2 == 0 {
                    continue;
                }
                d.remove(PatternId(i), &w);
                d.debug_assert_consistent(&w);
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Domain

// #region 🔖Model
pub mod model {
//! 🗂️ Compiled WFC model: the pattern universe, the directed relation universe, and the
//! `allowed[relation][source] → PatternSet(targets)` compatibility table plus its transpose
//! (`supporters`). Everything downstream (domains, propagation, constraints) reads only from
//! [`CompiledModel`] — never from a builder — so compilation is the single place non-bitset
//! representations (predicates, sockets, symmetry orbits) get resolved into bitsets.

use crate::bitset::PatternSet;
use crate::error::ModelError;
use crate::ids::{PatternId, RelationId, TileId};
use crate::weights::WeightTable;

// #region 🔖Info
/// 🧩 Per-pattern metadata carried alongside the compiled compatibility tables.
#[derive(Clone, Debug)]
pub struct PatternInfo {
    pub weight: f64,
    /// 🧩 Interned tag ids (see [`CompiledModel::tag_name`]); order-independent, deduplicated.
    pub tags: Vec<u32>,
    /// 🧩 The authored tile this pattern was compiled from, when built via a tiled/extracted model.
    pub tile: Option<TileId>,
    /// 🧩 Symmetry-orbit canonical pattern id, when built via symmetry expansion (`P5`); `None`
    /// for patterns with no declared symmetry.
    pub orbit_canonical: Option<PatternId>,
}

/// ↔️ Per-relation metadata: a display name and its declared directed inverse.
#[derive(Clone, Debug)]
pub struct RelationInfo {
    pub name: String,
    pub inverse: RelationId,
}
// #endregion 🔖Info

// #region 🔖Builder
/// 🏗️ Accumulates patterns, relations, and directed compatibility pairs before [`ModelBuilder::compile`]
/// resolves everything into dense bitset tables. The lowest-level builder in the crate — [`crate::tiled::TiledModelBuilder`]
/// and pattern extraction both compile down to this shape.
#[derive(Clone, Debug, Default)]
pub struct ModelBuilder {
    weights: Vec<f64>,
    tags: Vec<Vec<u32>>,
    tiles: Vec<Option<TileId>>,
    orbit_canonical: Vec<Option<PatternId>>,
    tag_names: Vec<String>,
    tag_ids: std::collections::HashMap<String, u32>,
    relation_names: Vec<String>,
    relation_inverse: Vec<RelationId>,
    allow_pairs: Vec<Vec<(PatternId, PatternId)>>,
    deny_pairs: Vec<Vec<(PatternId, PatternId)>>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🏗️ Registers a new pattern with the given weight, returning its dense id.
    pub fn add_pattern(&mut self, weight: f64) -> PatternId {
        let id = PatternId::from_index(self.weights.len());
        self.weights.push(weight);
        self.tags.push(Vec::new());
        self.tiles.push(None);
        self.orbit_canonical.push(None);
        id
    }

    pub fn set_tile(&mut self, p: PatternId, tile: TileId) {
        self.tiles[p.index()] = Some(tile);
    }

    pub fn set_orbit_canonical(&mut self, p: PatternId, canonical: PatternId) {
        self.orbit_canonical[p.index()] = Some(canonical);
    }

    /// 🏗️ Tags `p` with `name`, interning the name on first use. Idempotent.
    pub fn add_tag(&mut self, p: PatternId, name: &str) -> u32 {
        let id = self.intern_tag(name);
        let tags = &mut self.tags[p.index()];
        if !tags.contains(&id) {
            tags.push(id);
        }
        id
    }

    fn intern_tag(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.tag_ids.get(name) {
            return id;
        }
        let id = self.tag_names.len() as u32;
        self.tag_names.push(name.to_string());
        self.tag_ids.insert(name.to_string(), id);
        id
    }

    /// 🏗️ Registers a new directed relation, self-inverse by default until paired via
    /// [`ModelBuilder::set_relation_inverse`].
    pub fn add_relation(&mut self, name: &str) -> RelationId {
        let id = RelationId::from_index(self.relation_names.len());
        self.relation_names.push(name.to_string());
        self.relation_inverse.push(id);
        self.allow_pairs.push(Vec::new());
        self.deny_pairs.push(Vec::new());
        id
    }

    /// 🏗️ Declares `a` and `b` as each other's directed inverse (e.g. north ↔ south).
    pub fn set_relation_inverse(&mut self, a: RelationId, b: RelationId) {
        self.relation_inverse[a.index()] = b;
        self.relation_inverse[b.index()] = a;
    }

    pub fn allow(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        self.allow_pairs[r.index()].push((src, dst));
    }

    /// 🏗️ `deny` always wins over `allow` at compile time, regardless of call order.
    pub fn deny(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        self.deny_pairs[r.index()].push((src, dst));
    }

    /// 🏗️ Convenience: `allow(r, src, dst)` plus `allow(inverse(r), dst, src)` in one call — the
    /// common case where compatibility is meant to hold symmetrically under the declared inverse.
    pub fn allow_mirrored(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        let inv = self.relation_inverse[r.index()];
        self.allow(r, src, dst);
        self.allow(inv, dst, src);
    }

    /// 🏗️ Resolves every accumulated pair into dense `allowed`/`supporters` bitset tables and
    /// returns the immutable [`CompiledModel`]. Consumes `self` — a builder compiles exactly once.
    pub fn compile(self) -> Result<CompiledModel, ModelError> {
        let pattern_count = self.weights.len();
        if pattern_count == 0 {
            return Err(ModelError::EmptyPatternUniverse);
        }
        let relation_count = self.relation_names.len();
        let weights = WeightTable::new(&self.weights)?;

        let table_len = relation_count.checked_mul(pattern_count).ok_or(ModelError::CapacityOverflow { what: "relation_count * pattern_count" })?;
        let mut allowed: Vec<PatternSet> = (0..table_len).map(|_| PatternSet::new_empty(pattern_count)).collect();
        for r in 0..relation_count {
            for &(src, dst) in &self.allow_pairs[r] {
                allowed[r * pattern_count + src.index()].set(dst, true);
            }
            for &(src, dst) in &self.deny_pairs[r] {
                allowed[r * pattern_count + src.index()].set(dst, false);
            }
        }

        let mut supporters: Vec<PatternSet> = (0..table_len).map(|_| PatternSet::new_empty(pattern_count)).collect();
        for r in 0..relation_count {
            for src in 0..pattern_count {
                let src_id = PatternId::from_index(src);
                for dst in allowed[r * pattern_count + src].iter_ones() {
                    supporters[r * pattern_count + dst.index()].set(src_id, true);
                }
            }
        }
        let base_support: Vec<u32> = supporters.iter().map(|s| s.count_ones()).collect();

        let patterns: Vec<PatternInfo> = (0..pattern_count)
            .map(|i| PatternInfo { weight: self.weights[i], tags: self.tags[i].clone(), tile: self.tiles[i], orbit_canonical: self.orbit_canonical[i] })
            .collect();
        let relations: Vec<RelationInfo> = (0..relation_count).map(|i| RelationInfo { name: self.relation_names[i].clone(), inverse: self.relation_inverse[i] }).collect();

        let mut model = CompiledModel {
            patterns,
            relations,
            allowed,
            supporters,
            base_support,
            weights,
            tag_names: self.tag_names,
            tag_ids: self.tag_ids,
            fingerprint: 0,
        };
        model.fingerprint = model.compute_fingerprint();
        Ok(model)
    }
}
// #endregion 🔖Builder

// #region 🔖Compiled
/// 🗂️ The immutable, validated result of [`ModelBuilder::compile`]. Every solver reads
/// compatibility exclusively through [`CompiledModel::allowed`]/[`CompiledModel::supporters`].
#[derive(Clone, Debug)]
pub struct CompiledModel {
    patterns: Vec<PatternInfo>,
    relations: Vec<RelationInfo>,
    /// 🗂️ Indexed `[relation.index() * pattern_count + source.index()]`.
    allowed: Vec<PatternSet>,
    /// 🗂️ The transpose of `allowed`: indexed `[relation.index() * pattern_count + target.index()]`.
    supporters: Vec<PatternSet>,
    base_support: Vec<u32>,
    weights: WeightTable,
    tag_names: Vec<String>,
    tag_ids: std::collections::HashMap<String, u32>,
    fingerprint: u64,
}

impl CompiledModel {
    #[inline]
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    #[inline]
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    #[inline]
    pub fn weights(&self) -> &WeightTable {
        &self.weights
    }

    #[inline]
    pub fn pattern_info(&self, p: PatternId) -> &PatternInfo {
        &self.patterns[p.index()]
    }

    #[inline]
    pub fn relation_info(&self, r: RelationId) -> &RelationInfo {
        &self.relations[r.index()]
    }

    #[inline]
    pub fn inverse(&self, r: RelationId) -> RelationId {
        self.relations[r.index()].inverse
    }

    #[inline]
    pub fn allowed(&self, r: RelationId, src: PatternId) -> &PatternSet {
        &self.allowed[r.index() * self.pattern_count() + src.index()]
    }

    #[inline]
    pub fn supporters(&self, r: RelationId, tgt: PatternId) -> &PatternSet {
        &self.supporters[r.index() * self.pattern_count() + tgt.index()]
    }

    #[inline]
    pub fn base_support(&self, r: RelationId, tgt: PatternId) -> u32 {
        self.base_support[r.index() * self.pattern_count() + tgt.index()]
    }

    pub fn tag_id(&self, name: &str) -> Option<u32> {
        self.tag_ids.get(name).copied()
    }

    pub fn tag_name(&self, id: u32) -> Option<&str> {
        self.tag_names.get(id as usize).map(|s| s.as_str())
    }

    pub fn full_domain(&self) -> PatternSet {
        PatternSet::new_full(self.pattern_count())
    }

    fn compute_fingerprint(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |bytes: &[u8]| {
            for &b in bytes {
                h ^= b as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        mix(&(self.patterns.len() as u64).to_le_bytes());
        for p in &self.patterns {
            mix(&p.weight.to_bits().to_le_bytes());
            mix(&(p.tags.len() as u64).to_le_bytes());
            for &t in &p.tags {
                mix(&t.to_le_bytes());
            }
        }
        mix(&(self.relations.len() as u64).to_le_bytes());
        for r in &self.relations {
            mix(r.name.as_bytes());
            mix(&r.inverse.get().to_le_bytes());
        }
        for set in &self.allowed {
            for &w in set.words() {
                mix(&w.to_le_bytes());
            }
        }
        h
    }

    #[inline]
    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// ✅ Checks that every relation's compiled table is the exact transpose of its declared
    /// inverse's table (`allowed(r,a,b) == allowed(inv(r),b,a)` for every `a, b`).
    pub fn validate(&self) -> Result<(), ModelError> {
        if self.patterns.is_empty() {
            return Err(ModelError::EmptyPatternUniverse);
        }
        let p = self.pattern_count();
        for ri in 0..self.relations.len() {
            let r = RelationId::from_index(ri);
            let inv = self.inverse(r);
            for src in 0..p {
                let src_id = PatternId::from_index(src);
                for dst in self.allowed(r, src_id).iter_ones() {
                    if !self.allowed(inv, dst).get(src_id) {
                        return Err(ModelError::AsymmetricInverse { relation: r });
                    }
                }
            }
        }
        Ok(())
    }

    /// 🔍 Non-fatal structural findings a model author probably wants to know about.
    pub fn lint(&self) -> Vec<LintFinding> {
        let mut findings = Vec::new();
        let p = self.pattern_count();
        for ri in 0..self.relations.len() {
            let r = RelationId::from_index(ri);
            let mut allowed_pairs = 0usize;
            for src in 0..p {
                allowed_pairs += self.allowed(r, PatternId::from_index(src)).count_ones() as usize;
            }
            let total_pairs = p * p;
            if allowed_pairs == total_pairs {
                findings.push(LintFinding::UnconstrainedRelation { relation: r });
            } else if p > 1 && allowed_pairs > 0 && allowed_pairs < p {
                findings.push(LintFinding::NearlyForbiddenRelation { relation: r, allowed_pairs, total_pairs });
            }
            for dst in 0..p {
                let dst_id = PatternId::from_index(dst);
                if self.supporters(r, dst_id).is_all_zero() {
                    findings.push(LintFinding::UnsupportedPattern { pattern: dst_id, relation: r });
                }
            }
        }
        findings
    }

    pub fn stats(&self) -> ModelStats {
        let p = self.pattern_count();
        let r = self.relation_count();
        let mut allowed_pair_count = 0usize;
        for ri in 0..r {
            for src in 0..p {
                allowed_pair_count += self.allowed(RelationId::from_index(ri), PatternId::from_index(src)).count_ones() as usize;
            }
        }
        let total_pairs = (r * p * p).max(1);
        let min_support = self.base_support.iter().copied().min().unwrap_or(0);
        let avg_support = if self.base_support.is_empty() { 0.0 } else { self.base_support.iter().sum::<u32>() as f64 / self.base_support.len() as f64 };
        let weight_min = self.patterns.iter().map(|p| p.weight).fold(f64::INFINITY, f64::min);
        let weight_max = self.patterns.iter().map(|p| p.weight).fold(f64::NEG_INFINITY, f64::max);
        ModelStats {
            pattern_count: p,
            relation_count: r,
            allowed_pair_count,
            density: allowed_pair_count as f64 / total_pairs as f64,
            min_support,
            avg_support,
            weight_min,
            weight_max,
        }
    }
}
// #endregion 🔖Compiled

// #region 🔖Lint
/// 🔍 One non-fatal structural observation from [`CompiledModel::lint`].
#[derive(Clone, PartialEq, Debug)]
pub enum LintFinding {
    /// 🔍 No pattern supports `pattern` as a neighbor under `relation` — it can never appear
    /// adjacent to anything along that relation and will always be pruned immediately.
    UnsupportedPattern { pattern: PatternId, relation: RelationId },
    /// 🔍 `relation` allows every pair — it imposes no constraint at all.
    UnconstrainedRelation { relation: RelationId },
    /// 🔍 `relation` allows very few pairs relative to the pattern universe — likely a modeling
    /// mistake rather than an intentionally tight constraint.
    NearlyForbiddenRelation { relation: RelationId, allowed_pairs: usize, total_pairs: usize },
}

/// 📊 Aggregate statistics over a [`CompiledModel`], useful for diagnostics and capacity planning.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ModelStats {
    pub pattern_count: usize,
    pub relation_count: usize,
    pub allowed_pair_count: usize,
    pub density: f64,
    pub min_support: u32,
    pub avg_support: f64,
    pub weight_min: f64,
    pub weight_max: f64,
}
// #endregion 🔖Lint

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn checkerboard_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        b.allow_mirrored(adj, white, black);
        b.compile().unwrap()
    }

    #[test]
    fn compile_rejects_empty_pattern_universe() {
        let b = ModelBuilder::new();
        assert_eq!(b.compile().unwrap_err(), ModelError::EmptyPatternUniverse);
    }

    #[test]
    fn compile_rejects_invalid_weight() {
        let mut b = ModelBuilder::new();
        b.add_pattern(-1.0);
        assert!(matches!(b.compile().unwrap_err(), ModelError::InvalidWeight { .. }));
    }

    #[test]
    fn allowed_and_supporters_are_transposes() {
        let m = checkerboard_model();
        let adj = RelationId(0);
        for src in 0..m.pattern_count() {
            let src_id = PatternId::from_index(src);
            for dst in m.allowed(adj, src_id).iter_ones() {
                assert!(m.supporters(adj, dst).get(src_id));
            }
        }
    }

    #[test]
    fn validate_passes_on_mirrored_model() {
        let m = checkerboard_model();
        assert!(m.validate().is_ok());
    }

    #[test]
    fn validate_fails_on_asymmetric_declaration() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let r = b.add_relation("one_way");
        b.allow(r, a, c); // only one direction declared; r is self-inverse by default
        let m = b.compile().unwrap();
        assert!(matches!(m.validate().unwrap_err(), ModelError::AsymmetricInverse { .. }));
    }

    #[test]
    fn deny_wins_over_allow_regardless_of_order() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let r = b.add_relation("r");
        b.deny(r, a, c);
        b.allow(r, a, c);
        let m = b.compile().unwrap();
        assert!(!m.allowed(r, a).get(c));
    }

    #[test]
    fn fingerprint_is_deterministic_and_sensitive() {
        let m1 = checkerboard_model();
        let m2 = checkerboard_model();
        assert_eq!(m1.fingerprint(), m2.fingerprint());

        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(2.0); // different weight
        let r = b.add_relation("adjacent");
        b.allow_mirrored(r, a, c);
        b.allow_mirrored(r, c, a);
        let m3 = b.compile().unwrap();
        assert_ne!(m1.fingerprint(), m3.fingerprint());
    }

    #[test]
    fn lint_flags_unconstrained_and_unsupported() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let free = b.add_relation("free");
        b.allow_mirrored(free, a, c);
        b.allow_mirrored(free, a, a);
        b.allow_mirrored(free, c, c);
        let starved = b.add_relation("starved");
        b.allow(starved, a, a); // c has no supporters at all under `starved`
        let m = b.compile().unwrap();
        let findings = m.lint();
        assert!(findings.contains(&LintFinding::UnconstrainedRelation { relation: free }));
        assert!(findings.iter().any(|f| matches!(f, LintFinding::UnsupportedPattern { pattern, relation } if *pattern == c && *relation == starved)));
    }

    #[test]
    fn stats_report_sane_values() {
        let m = checkerboard_model();
        let stats = m.stats();
        assert_eq!(stats.pattern_count, 2);
        assert_eq!(stats.relation_count, 1);
        assert_eq!(stats.allowed_pair_count, 2);
        assert_eq!(stats.weight_min, 1.0);
        assert_eq!(stats.weight_max, 1.0);
    }

    #[test]
    fn tags_are_interned_and_deduplicated() {
        let mut b = ModelBuilder::new();
        let p = b.add_pattern(1.0);
        let id1 = b.add_tag(p, "solid");
        let id2 = b.add_tag(p, "solid");
        assert_eq!(id1, id2);
        b.add_relation("r");
        let m = b.compile().unwrap();
        assert_eq!(m.pattern_info(p).tags, vec![id1]);
        assert_eq!(m.tag_name(id1), Some("solid"));
        assert_eq!(m.tag_id("solid"), Some(id1));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Model

// #region 🔖Oracle
pub mod oracle {
//! 🔮 Brute-force reference solver. Deliberately shares no code with [`crate::propagate`]/
//! [`crate::search`] — a naive, obviously-correct DFS enumerator (with only per-step consistency
//! pruning against already-assigned neighbors, never arc-consistency propagation) that every
//! optimized engine is checked against in this crate's differential tests.

use crate::bitset::PatternSet;
use crate::ids::{NodeId, PatternId, RelationId};
use crate::model::CompiledModel;

// #region 🔖Enumerate
/// 🔮 One directed compatibility arc the oracle must respect, exactly mirroring what a solver's
/// propagation kernel would enumerate for the same topology.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArcSpec {
    pub from: NodeId,
    pub to: NodeId,
    pub relation: RelationId,
}

/// 🔮 The result of [`enumerate`]: every found solution (up to `limit`), plus whether the search
/// tree was fully explored (`complete = false` means either `limit` or the internal step budget
/// was hit first — an [`Unsatisfiable`](crate)-style conclusion can only be drawn when `complete`).
#[derive(Clone, Debug)]
pub struct OracleResult {
    pub solutions: Vec<Vec<PatternId>>,
    pub complete: bool,
}

/// 🔮 One arc-compatibility violation found by [`check_assignment`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Violation {
    ArcViolated { from: NodeId, to: NodeId, relation: RelationId },
}

const DEFAULT_STEP_BUDGET: u64 = 5_000_000;

/// 🔮 Exhaustively enumerates every complete assignment respecting `init_domains` and every arc in
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

/// 🔮 Checks a complete assignment against every arc, independent of [`enumerate`]'s search code.
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
// #endregion 🔖Enumerate

// #region 🔖Testgen
/// 🧪 Seeded generators and named fixtures shared by this crate's differential tests. `pub(crate)`
/// and `cfg(test)`-gated because these helpers are test infrastructure, not public API — every
/// module's `#[cfg(test)] mod tests` can `use crate::oracle::testgen::*` once `cargo test` enables
/// `cfg(test)` crate-wide.
#[cfg(test)]
pub(crate) mod testgen {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::weights::WeightTable;

    /// 🧪 A self-contained tiny instance: a compiled model, its node count, arcs, and per-node
    /// initial domains — everything [`super::enumerate`] and a real solver both need.
    pub struct Fixture {
        pub model: CompiledModel,
        pub node_count: usize,
        pub arcs: Vec<ArcSpec>,
        pub init_domains: Vec<PatternSet>,
    }

    /// 🧪 Two patterns (black/white) that must differ across every edge of a path graph
    /// `0 - 1 - ... - (n-1)`. Always satisfiable (paths are bipartite).
    pub fn checkerboard_path(n: usize) -> Fixture {
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

    /// 🧪 Two patterns that must differ across every edge of an odd cycle `0-1-...-(n-1)-0` with
    /// `n` odd — unsatisfiable, since odd cycles are not bipartite. `n` must be odd and >= 3.
    pub fn unsat_odd_cycle(n: usize) -> Fixture {
        assert!(n >= 3 && n % 2 == 1, "unsat_odd_cycle requires an odd n >= 3");
        let mut fx = checkerboard_path(n);
        let adj = RelationId(0);
        fx.arcs.push(ArcSpec { from: NodeId::from_index(n - 1), to: NodeId::from_index(0), relation: adj });
        fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(n - 1), relation: adj });
        fx
    }

    /// 🧪 A complete graph `K_n` over `k` patterns that must all differ pairwise — a proper
    /// `k`-coloring of `K_n`, satisfiable iff `k >= n`.
    pub fn complete_graph_coloring(n: usize, k: usize) -> Fixture {
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

    /// 🧪 A uniformly-random tiny compiled model: `pattern_count` patterns each with a random
    /// weight in `[1, 5]`, one relation whose compatibility pairs are each independently kept with
    /// probability `density`.
    pub fn random_model(rng: &mut mathematical_random::Rng, pattern_count: usize, density: f64) -> (CompiledModel, RelationId) {
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

    /// 🧪 A random small connected graph over `node_count` nodes (a random spanning tree plus a
    /// few extra random edges), with both directions registered under `relation`.
    pub fn random_arcs(rng: &mut mathematical_random::Rng, node_count: usize, relation: RelationId) -> Vec<ArcSpec> {
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
    pub fn full_domains(model: &CompiledModel, node_count: usize) -> Vec<PatternSet> {
        vec![model.full_domain(); node_count]
    }

    #[allow(dead_code)]
    pub fn weight_table_of(model: &CompiledModel) -> &WeightTable {
        model.weights()
    }
}
// #endregion 🔖Testgen

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use testgen::*;

    #[test]
    fn checkerboard_path_is_satisfiable_and_alternates() {
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

    #[test]
    fn unsat_odd_cycle_has_no_solutions() {
        let fx = unsat_odd_cycle(5);
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
        assert!(result.complete);
        assert!(result.solutions.is_empty());
    }

    #[test]
    fn even_cycle_is_satisfiable() {
        let mut fx = checkerboard_path(6);
        let adj = RelationId(0);
        fx.arcs.push(ArcSpec { from: NodeId::from_index(5), to: NodeId::from_index(0), relation: adj });
        fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(5), relation: adj });
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
        assert!(result.complete);
        assert_eq!(result.solutions.len(), 2);
    }

    #[test]
    fn complete_graph_coloring_matches_chromatic_condition() {
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

    #[test]
    fn check_assignment_detects_violation() {
        let fx = checkerboard_path(3);
        let bad = vec![PatternId(0), PatternId(0), PatternId(1)];
        assert!(check_assignment(&fx.model, &bad, &fx.arcs).is_err());
        let good = vec![PatternId(0), PatternId(1), PatternId(0)];
        assert!(check_assignment(&fx.model, &good, &fx.arcs).is_ok());
    }

    #[test]
    fn limit_caps_collected_solutions() {
        let fx = complete_graph_coloring(4, 4);
        let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 5);
        assert_eq!(result.solutions.len(), 5);
    }

    mod quick {
        use super::*;

        #[test]
        fn random_instances_every_solution_passes_check_assignment() {
            let mut rng = mathematical_random::Rng::from_seed(2024);
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
// #endregion 🔖Tests
}
// #endregion 🔖Oracle

// #region 🔖Tiled
pub mod tiled {
//! 🧱 Explicit tiled model construction: one authored tile ↔ one pattern (until [`crate::symmetry`]
//! expands orbits in a later phase), with allow/deny pair lists and eagerly-compiled predicates.
//! A thin `TileId`-facing wrapper over [`crate::model::ModelBuilder`]'s `PatternId`-facing API.

use crate::error::ModelError;
use crate::ids::{RelationId, TileId};
use crate::model::{CompiledModel, ModelBuilder};

// #region 🔖Builder
/// 🧱 Builds a [`CompiledModel`] from tiles, weights, and directional allow/deny pairs.
#[derive(Clone, Debug, Default)]
pub struct TiledModelBuilder {
    builder: ModelBuilder,
    tile_pattern: Vec<crate::ids::PatternId>,
}

impl TiledModelBuilder {
    pub fn new() -> Self {
        Self { builder: ModelBuilder::new(), tile_pattern: Vec::new() }
    }

    /// 🧱 Registers a new tile with the given sampling weight.
    pub fn tile(&mut self, weight: f64) -> TileId {
        let p = self.builder.add_pattern(weight);
        let id = TileId::from_index(self.tile_pattern.len());
        self.tile_pattern.push(p);
        self.builder.set_tile(p, id);
        id
    }

    pub fn tag(&mut self, tile: TileId, name: &str) -> u32 {
        self.builder.add_tag(self.tile_pattern[tile.index()], name)
    }

    pub fn relation(&mut self, name: &str) -> RelationId {
        self.builder.add_relation(name)
    }

    pub fn set_relation_inverse(&mut self, a: RelationId, b: RelationId) {
        self.builder.set_relation_inverse(a, b);
    }

    pub fn allow(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱 `deny` always wins over `allow`, regardless of call order.
    pub fn deny(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.deny(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    pub fn allow_mirrored(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow_mirrored(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱 Bulk allow from a predicate over every pair in `tiles`, compiled eagerly right now (the
    /// predicate itself is never stored — only its resolved allow pairs survive into the model).
    pub fn allow_where(&mut self, r: RelationId, tiles: &[TileId], pred: impl Fn(TileId, TileId) -> bool) {
        for &a in tiles {
            for &b in tiles {
                if pred(a, b) {
                    self.allow(r, a, b);
                }
            }
        }
    }

    pub fn pattern_of(&self, tile: TileId) -> crate::ids::PatternId {
        self.tile_pattern[tile.index()]
    }

    pub fn tile_count(&self) -> usize {
        self.tile_pattern.len()
    }

    pub fn compile(self) -> Result<CompiledModel, ModelError> {
        self.builder.compile()
    }
}
// #endregion 🔖Builder

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_to_pattern_is_one_to_one() {
        let mut b = TiledModelBuilder::new();
        let grass = b.tile(3.0);
        let water = b.tile(1.0);
        assert_ne!(b.pattern_of(grass), b.pattern_of(water));
        assert_eq!(b.tile_count(), 2);
    }

    #[test]
    fn allow_and_deny_compile_correctly() {
        let mut b = TiledModelBuilder::new();
        let a = b.tile(1.0);
        let c = b.tile(1.0);
        let d = b.tile(1.0);
        let r = b.relation("r");
        b.allow_mirrored(r, a, c);
        b.allow(r, a, d);
        b.deny(r, a, d);
        let (pa, pc, pd) = (b.pattern_of(a), b.pattern_of(c), b.pattern_of(d));
        let m = b.compile().unwrap();
        assert!(m.allowed(r, pa).get(pc));
        assert!(!m.allowed(r, pa).get(pd));
    }

    #[test]
    fn allow_where_compiles_predicate_eagerly() {
        let mut b = TiledModelBuilder::new();
        let tiles: Vec<TileId> = (0..4).map(|_| b.tile(1.0)).collect();
        let r = b.relation("le");
        b.allow_where(r, &tiles, |x, y| x.get() <= y.get());
        let pattern_of: Vec<_> = tiles.iter().map(|&t| b.pattern_of(t)).collect();
        let m = b.compile().unwrap();
        for (xi, &x) in tiles.iter().enumerate() {
            for (yi, &y) in tiles.iter().enumerate() {
                let expected = x.get() <= y.get();
                assert_eq!(m.allowed(r, pattern_of[xi]).get(pattern_of[yi]), expected);
            }
        }
    }

    #[test]
    fn tags_round_trip_through_tiles() {
        let mut b = TiledModelBuilder::new();
        let t = b.tile(1.0);
        let id = b.tag(t, "solid");
        b.relation("r");
        let pt = b.pattern_of(t);
        let m = b.compile().unwrap();
        assert_eq!(m.pattern_info(pt).tags, vec![id]);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Tiled

// #region 🔖Topology
pub(crate) mod topology {
//! 🗺️ The shared topology abstraction every kernel routine (`propagate`, `search`) is generic
//! over, plus [`GraphTopology`] — the CSR-backed arbitrary-graph implementation and semantic
//! reference for `Grid2dTopology`/`Grid3dTopology` (added in later phases). `Topology` is
//! `pub(crate)`: not public API, free to evolve, and never boxed as `dyn` — internal-iteration
//! methods take `impl FnMut` so every implementor's arc loop inlines into pure index arithmetic
//! (grids) or a CSR slice walk (graphs) with zero adjacency storage for grids and zero indirect
//! calls in the propagation hot path.

use crate::ids::{NodeId, RegionId, RelationId};

// #region 🔖Trait
/// 🗺️ What the kernel needs from any topology: how many variables, how they connect, and where
/// each one's incoming arcs live in a dense per-arc-slot indexing scheme (used by AC-4's support
/// counters). Implementors are always monomorphized into the kernel — never called through `dyn`.
#[allow(dead_code)] // arc_count/region_of/for_each_in_arc/max_in_degree are consumed starting
// with the AC-4 propagator (P6) and region-scoped constraints (P7); the trait's full shape is
// fixed now so those phases never need to touch this sealed boundary.
pub(crate) trait Topology {
    fn node_count(&self) -> usize;
    fn arc_count(&self) -> usize;
    fn region_of(&self, n: NodeId) -> RegionId;
    /// 🗺️ Calls `f(target, relation)` once per outgoing arc of `n`, in a stable order.
    fn for_each_out_arc(&self, n: NodeId, f: impl FnMut(NodeId, RelationId));
    /// 🗺️ Calls `f(source, relation, slot)` once per incoming arc of `n`. `slot` is a dense id
    /// unique to this specific incoming arc, always `< node_count() * max_in_degree()` — AC-4
    /// keys its support counters by it. Bundling the slot into the same callback (rather than a
    /// separate `in_arc_slot(target, ordinal)` lookup) is deliberate: it is the only way to
    /// guarantee the slot a caller records for an arc is the same slot the topology itself means,
    /// since "ordinal" has no meaning independent of how a specific implementor enumerates arcs.
    fn for_each_in_arc(&self, n: NodeId, f: impl FnMut(NodeId, RelationId, usize));
    /// 🗺️ Upper bound on any single node's incoming-arc count, for sizing dense counter tables.
    fn max_in_degree(&self) -> usize;
}
// #endregion 🔖Trait

// #region 🔖Graph
/// 🗺️ Arbitrary directed graph topology: CSR-style outgoing and incoming arc storage. Supports
/// multiedges (repeated `(from, to)` under different or identical relations) and self-loops.
#[derive(Clone, Debug)]
#[allow(dead_code)] // in_sources/in_relations/regions back for_each_in_arc/in_arc_slot/region_of,
// unread until AC-4 (P6) and region-scoped constraints (P7) call those trait methods.
pub struct GraphTopology {
    node_count: usize,
    out_starts: Vec<u32>,
    out_targets: Vec<u32>,
    out_relations: Vec<u32>,
    in_starts: Vec<u32>,
    in_sources: Vec<u32>,
    in_relations: Vec<u32>,
    regions: Vec<u32>,
}

impl GraphTopology {
    #[inline]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    #[inline]
    pub fn arc_count(&self) -> usize {
        self.out_targets.len()
    }

    pub fn out_degree(&self, n: NodeId) -> usize {
        (self.out_starts[n.index() + 1] - self.out_starts[n.index()]) as usize
    }

    pub fn in_degree(&self, n: NodeId) -> usize {
        (self.in_starts[n.index() + 1] - self.in_starts[n.index()]) as usize
    }
}

impl Topology for GraphTopology {
    #[inline]
    fn node_count(&self) -> usize {
        self.node_count
    }

    #[inline]
    fn arc_count(&self) -> usize {
        self.out_targets.len()
    }

    #[inline]
    fn region_of(&self, n: NodeId) -> RegionId {
        RegionId(self.regions[n.index()])
    }

    #[inline]
    fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let start = self.out_starts[n.index()] as usize;
        let end = self.out_starts[n.index() + 1] as usize;
        for i in start..end {
            f(NodeId(self.out_targets[i]), RelationId(self.out_relations[i]));
        }
    }

    #[inline]
    fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let start = self.in_starts[n.index()] as usize;
        let end = self.in_starts[n.index() + 1] as usize;
        for i in start..end {
            f(NodeId(self.in_sources[i]), RelationId(self.in_relations[i]), i);
        }
    }

    fn max_in_degree(&self) -> usize {
        (0..self.node_count).map(|i| (self.in_starts[i + 1] - self.in_starts[i]) as usize).max().unwrap_or(0)
    }
}
// #endregion 🔖Graph

// #region 🔖Builder
/// 🏗️ Accumulates directed arcs and per-node regions before [`GraphTopologyBuilder::build`]
/// buckets them into the two CSR arrays [`GraphTopology`] reads.
#[derive(Clone, Debug)]
pub struct GraphTopologyBuilder {
    node_count: usize,
    arcs: Vec<(NodeId, NodeId, RelationId)>,
    regions: Vec<RegionId>,
}

impl GraphTopologyBuilder {
    pub fn new(node_count: usize) -> Self {
        Self { node_count, arcs: Vec::new(), regions: vec![RegionId(0); node_count] }
    }

    pub fn arc(&mut self, from: NodeId, to: NodeId, relation: RelationId) -> &mut Self {
        self.arcs.push((from, to, relation));
        self
    }

    pub fn region(&mut self, n: NodeId, r: RegionId) -> &mut Self {
        self.regions[n.index()] = r;
        self
    }

    pub fn build(self) -> Result<GraphTopology, crate::error::TopologyError> {
        use crate::error::TopologyError;
        for &(from, to, _) in &self.arcs {
            if from.index() >= self.node_count {
                return Err(TopologyError::DanglingArc { from });
            }
            if to.index() >= self.node_count {
                return Err(TopologyError::DanglingArc { from: to });
            }
        }

        let mut out_sorted = self.arcs.clone();
        out_sorted.sort_by_key(|&(from, to, r)| (from.get(), to.get(), r.get()));
        let mut out_starts = vec![0u32; self.node_count + 1];
        for &(from, _, _) in &out_sorted {
            out_starts[from.index() + 1] += 1;
        }
        for i in 0..self.node_count {
            out_starts[i + 1] += out_starts[i];
        }
        let out_targets: Vec<u32> = out_sorted.iter().map(|&(_, to, _)| to.get()).collect();
        let out_relations: Vec<u32> = out_sorted.iter().map(|&(_, _, r)| r.get()).collect();

        let mut in_sorted = self.arcs.clone();
        in_sorted.sort_by_key(|&(from, to, r)| (to.get(), from.get(), r.get()));
        let mut in_starts = vec![0u32; self.node_count + 1];
        for &(_, to, _) in &in_sorted {
            in_starts[to.index() + 1] += 1;
        }
        for i in 0..self.node_count {
            in_starts[i + 1] += in_starts[i];
        }
        let in_sources: Vec<u32> = in_sorted.iter().map(|&(from, _, _)| from.get()).collect();
        let in_relations: Vec<u32> = in_sorted.iter().map(|&(_, _, r)| r.get()).collect();

        let regions: Vec<u32> = self.regions.iter().map(|r| r.get()).collect();

        Ok(GraphTopology { node_count: self.node_count, out_starts, out_targets, out_relations, in_starts, in_sources, in_relations, regions })
    }
}
// #endregion 🔖Builder

// #region 🔖FromGraphView
/// 🔁 Builds a [`GraphTopology`] from any [`mathematical_graph::GraphView`]. Nodes are assigned
/// dense ids in ascending order of their `mathematical_graph::NodeId` (deterministic regardless of
/// the view's internal iteration order). Directed views get one arc per edge via `rel_of`;
/// undirected views get the same relation registered in both directions (the model relation is
/// expected to be self-inverse in that case, matching every other symmetric-adjacency convention
/// in this crate).
pub fn from_graph_view(view: &impl mathematical_graph::GraphView, rel_of: impl Fn(mathematical_graph::EdgeRef) -> RelationId) -> Result<GraphTopology, crate::error::TopologyError> {
    use crate::error::TopologyError;
    let mut sorted_nodes: Vec<mathematical_graph::NodeId> = view.nodes().collect();
    sorted_nodes.sort_unstable();
    if sorted_nodes.len() > u32::MAX as usize {
        return Err(TopologyError::TooManyNodes { count: sorted_nodes.len() as u64 });
    }
    let index_of: std::collections::HashMap<mathematical_graph::NodeId, usize> = sorted_nodes.iter().enumerate().map(|(i, &n)| (n, i)).collect();

    let mut builder = GraphTopologyBuilder::new(sorted_nodes.len());
    for edge in view.edges() {
        let from = NodeId::from_index(index_of[&edge.u]);
        let to = NodeId::from_index(index_of[&edge.v]);
        let r = rel_of(edge);
        builder.arc(from, to, r);
        if !view.is_directed() {
            builder.arc(to, from, r);
        }
    }
    builder.build()
}
// #endregion 🔖FromGraphView

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_correct_out_and_in_arcs() {
        let mut b = GraphTopologyBuilder::new(3);
        b.arc(NodeId(0), NodeId(1), RelationId(0));
        b.arc(NodeId(1), NodeId(2), RelationId(0));
        b.arc(NodeId(0), NodeId(2), RelationId(1));
        let topo = b.build().unwrap();
        assert_eq!(topo.node_count(), 3);
        assert_eq!(topo.arc_count(), 3);

        let mut out0 = Vec::new();
        topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
        assert_eq!(out0, vec![(NodeId(1), RelationId(0)), (NodeId(2), RelationId(1))]);

        let mut in2 = Vec::new();
        topo.for_each_in_arc(NodeId(2), |m, r, _slot| in2.push((m, r)));
        assert_eq!(in2, vec![(NodeId(0), RelationId(1)), (NodeId(1), RelationId(0))]);
    }

    #[test]
    fn self_loops_and_multiedges_are_supported() {
        let mut b = GraphTopologyBuilder::new(2);
        b.arc(NodeId(0), NodeId(0), RelationId(0)); // self-loop
        b.arc(NodeId(0), NodeId(1), RelationId(0));
        b.arc(NodeId(0), NodeId(1), RelationId(1)); // multiedge under a different relation
        let topo = b.build().unwrap();
        assert_eq!(topo.arc_count(), 3);
        assert_eq!(topo.out_degree(NodeId(0)), 3);
        let mut out0 = Vec::new();
        topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
        assert!(out0.contains(&(NodeId(0), RelationId(0))));
        assert!(out0.contains(&(NodeId(1), RelationId(0))));
        assert!(out0.contains(&(NodeId(1), RelationId(1))));
    }

    #[test]
    fn dangling_arc_is_rejected() {
        let mut b = GraphTopologyBuilder::new(2);
        b.arc(NodeId(0), NodeId(5), RelationId(0));
        assert!(b.build().is_err());
    }

    #[test]
    fn in_arc_slots_are_dense_and_unique_per_node() {
        let mut b = GraphTopologyBuilder::new(3);
        b.arc(NodeId(0), NodeId(2), RelationId(0));
        b.arc(NodeId(1), NodeId(2), RelationId(0));
        let topo = b.build().unwrap();
        assert_eq!(topo.in_degree(NodeId(2)), 2);
        let mut slots = Vec::new();
        topo.for_each_in_arc(NodeId(2), |_, _, slot| slots.push(slot));
        assert_eq!(slots.len(), 2);
        assert_ne!(slots[0], slots[1]);
        for &slot in &slots {
            assert!(slot < topo.node_count() * topo.max_in_degree());
        }
        assert_eq!(topo.max_in_degree(), 2);
    }

    #[test]
    fn regions_default_to_zero_and_are_settable() {
        let mut b = GraphTopologyBuilder::new(2);
        b.region(NodeId(1), RegionId(7));
        let topo = b.build().unwrap();
        assert_eq!(topo.region_of(NodeId(0)), RegionId(0));
        assert_eq!(topo.region_of(NodeId(1)), RegionId(7));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Topology

// #region 🔖Propagate
pub(crate) mod propagate {
//! 🌀 The propagation queue shared by every propagation engine. The engines themselves (AC-3 in
//! this phase, AC-4/watched-support in a later phase) live in sibling `prop_*.rs` modules.

use crate::ids::NodeId;

// #region 🔖Queue
/// 🌀 A FIFO node queue with membership-bit dedup — pushing an already-queued node is a no-op, so
/// a node dirtied twice before being processed is still visited exactly once per drain.
#[derive(Clone, Debug)]
pub(crate) struct PropQueue {
    queue: std::collections::VecDeque<u32>,
    in_queue: Vec<bool>,
}

impl PropQueue {
    pub fn new(node_count: usize) -> Self {
        Self { queue: std::collections::VecDeque::new(), in_queue: vec![false; node_count] }
    }

    pub fn push(&mut self, n: NodeId) {
        let idx = n.index();
        if !self.in_queue[idx] {
            self.in_queue[idx] = true;
            self.queue.push_back(n.get());
        }
    }

    pub fn pop(&mut self) -> Option<NodeId> {
        let raw = self.queue.pop_front()?;
        self.in_queue[raw as usize] = false;
        Some(NodeId(raw))
    }

    #[allow(dead_code)] // queue-introspection API exercised by the step/resume API added in a later phase
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.in_queue.iter_mut().for_each(|b| *b = false);
    }

    pub fn push_all(&mut self, node_count: usize) {
        self.clear();
        for i in 0..node_count {
            self.push(NodeId::from_index(i));
        }
    }
}
// #endregion 🔖Queue

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_dedups_and_fifo_orders() {
        let mut q = PropQueue::new(4);
        q.push(NodeId(1));
        q.push(NodeId(2));
        q.push(NodeId(1)); // dedup
        assert_eq!(q.pop(), Some(NodeId(1)));
        assert_eq!(q.pop(), Some(NodeId(2)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn popped_node_can_be_repushed() {
        let mut q = PropQueue::new(2);
        q.push(NodeId(0));
        q.pop();
        q.push(NodeId(0));
        assert_eq!(q.pop(), Some(NodeId(0)));
    }

    #[test]
    fn clear_resets_membership() {
        let mut q = PropQueue::new(2);
        q.push(NodeId(0));
        q.clear();
        assert!(q.is_empty());
        q.push(NodeId(0));
        assert_eq!(q.pop(), Some(NodeId(0)));
    }

    #[test]
    fn push_all_enqueues_every_node_once() {
        let mut q = PropQueue::new(3);
        q.push_all(3);
        let mut seen = Vec::new();
        while let Some(n) = q.pop() {
            seen.push(n);
        }
        assert_eq!(seen, vec![NodeId(0), NodeId(1), NodeId(2)]);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Propagate

// #region 🔖Propac3
pub(crate) mod prop_ac3 {
//! ⚙️ Bitset arc-revision propagation — the reference engine every optimized engine (AC-4,
//! watched-support, added in a later phase) is checked against. For a dirty node `n` and out-arc
//! `n --r--> m`, computes `union = OR of allowed(r, p) for p in domain(n)` and intersects it into
//! `domain(m)`. Simple, obviously correct, no auxiliary state to roll back.

use crate::bitset::PatternSet;
use crate::diag::Metrics;
use crate::domain::{DomainStore, RestrictResult};
use crate::ids::NodeId;
use crate::model::CompiledModel;
use crate::propagate::PropQueue;
use crate::topology::Topology;
use crate::trail::Trail;

// #region 🔖Engine
/// ⚙️ Drains `queue`, running arc revision to a fixed point. Every pattern actually removed is
/// recorded on `trail` — propagation-caused removals vastly outnumber decision-caused ones, and a
/// backtrack that only undid the decision's own removals (not their propagated consequences) would
/// leave contradictions permanently invisible to future arc-consistency checks (an already-empty
/// domain can never re-report `Wipeout`, it can only ever report `Unchanged`). Returns `Err(node)`
/// for the first node whose domain is wiped to empty; `Ok(())` means every queued node's out-arcs
/// are consistent (`domain(m)` is a subset of what every arc from an assigned/reduced neighbor allows).
pub(crate) fn run_to_fixed_point<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, queue: &mut PropQueue, trail: &mut Trail, metrics: &mut Metrics) -> Result<(), NodeId> {
    let p = model.pattern_count();
    let mut union = PatternSet::new_empty(p);
    let mut removed = PatternSet::new_empty(p);
    let mut wipeout: Option<NodeId> = None;

    while let Some(n) = queue.pop() {
        if wipeout.is_some() {
            break;
        }
        metrics.propagations += 1;
        let n_bits = domains.get(n).bits().clone();
        topo.for_each_out_arc(n, |m, r| {
            if wipeout.is_some() {
                return;
            }
            union.clear_all();
            for pat in n_bits.iter_ones() {
                union.or_with(model.allowed(r, pat));
            }
            let result = domains.get_mut(m).restrict_collecting(&union, model.weights(), &mut removed);
            match result {
                RestrictResult::Unchanged => {}
                RestrictResult::Wipeout => {
                    trail.record_removed_set(m, &removed);
                    wipeout = Some(m);
                }
                RestrictResult::Reduced(count) => {
                    trail.record_removed_set(m, &removed);
                    metrics.removals += count as u64;
                    queue.push(m);
                }
                RestrictResult::Singleton(_) => {
                    trail.record_removed_set(m, &removed);
                    metrics.removals += 1;
                    queue.push(m);
                }
            }
        });
    }

    match wipeout {
        Some(n) => Err(n),
        None => Ok(()),
    }
}
// #endregion 🔖Engine

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{PatternId, RelationId};
    use crate::model::ModelBuilder;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard(n: usize) -> (CompiledModel, crate::topology::GraphTopology, RelationId) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        (model, tb.build().unwrap(), adj)
    }

    #[test]
    fn propagation_forces_alternation_after_one_pin() {
        let (model, topo, _adj) = checkerboard(4);
        let mut domains = DomainStore::new_full(4, model.weights());
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let mut queue = PropQueue::new(4);
        queue.push(NodeId(0));
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
        assert_eq!(domains.get(NodeId(0)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(1)).singleton(), Some(PatternId(1)));
        assert_eq!(domains.get(NodeId(2)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(3)).singleton(), Some(PatternId(1)));
        assert!(metrics.propagations > 0);
    }

    #[test]
    fn odd_cycle_pin_propagates_to_wipeout() {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();

        let mut tb = GraphTopologyBuilder::new(5);
        for i in 0..4 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        tb.arc(NodeId(4), NodeId(0), adj);
        tb.arc(NodeId(0), NodeId(4), adj);
        let topo = tb.build().unwrap();

        let mut domains = DomainStore::new_full(5, model.weights());
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let mut queue = PropQueue::new(5);
        queue.push(NodeId(0));
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        let result = run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics);
        assert!(result.is_err());
    }

    #[test]
    fn no_dirty_nodes_means_no_op() {
        let (model, topo, _adj) = checkerboard(3);
        let mut domains = DomainStore::new_full(3, model.weights());
        let mut queue = PropQueue::new(3);
        let mut trail = Trail::new();
        let mut metrics = Metrics::default();
        run_to_fixed_point(&model, &topo, &mut domains, &mut queue, &mut trail, &mut metrics).unwrap();
        for (_, d) in domains.iter() {
            assert_eq!(d.cardinality(), 2);
        }
        assert_eq!(metrics.propagations, 0);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Propac3

// #region 🔖Propac4
pub(crate) mod prop_ac4 {
//! ⚡ Support-count (AC-4-style) propagation: for each incoming arc slot and each candidate
//! source pattern, maintain how many currently-live target patterns it is still compatible with;
//! when that count hits zero the source pattern has lost its last support and is removed.
//! Removal-driven (a worklist of `(node, pattern)` events, not dirty nodes) so it never re-scans a
//! whole domain to find what changed.
//!
//! **Scope of this phase**: forward fixed-point propagation only, validated against
//! [`crate::prop_ac3`] for identical results from identical starting domains (this crate's P6
//! gate). Wiring this engine into [`crate::search`]'s backtracking loop is deliberately deferred:
//! `counts` is auxiliary state a chronological backtrack would also need to roll back exactly
//! (the trail today only knows how to re-add removed *domain* bits), and getting that rollback
//! subtly wrong is exactly the class of bug this crate's own [`crate::search`] history already hit
//! once with trail-recording gaps — better to land it deliberately, with its own rollback-soak
//! tests, than to rush it into the hot path now.

use crate::domain::{DomainStore, RestrictResult};
use crate::diag::Metrics;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::topology::Topology;

// #region 🔖Engine
/// ⚡ Dense support counters, indexed `[slot * pattern_count + pattern_index]` where `slot` comes
/// from [`Topology::for_each_in_arc`].
#[allow(dead_code)] // exercised by this module's own differential tests today; wired into
// crate::search's runtime engine selection once trail-integrated rollback lands (see module docs)
pub(crate) struct Ac4Engine {
    counts: Vec<u32>,
    pattern_count: usize,
}

#[allow(dead_code)]
impl Ac4Engine {
    /// ⚡ Computes every counter from scratch against `domains`' current state (`O(arcs * pattern_count^2)`,
    /// a one-time initialization cost) and immediately removes — cascading via [`Ac4Engine::propagate`]
    /// — any pattern that already has zero support given the model alone (independent of any
    /// decision yet made). This initial sweep is not optional: unlike AC-3 (which a caller makes
    /// complete by seeding *every* node into the propagation queue at solve start), AC-4 only
    /// reacts to counters that are *decremented* to zero by a removal event: a counter that starts
    /// at zero — a pattern with literally no compatible neighbor anywhere in the model — would
    /// never trigger a removal without this pass. Returns `Err(node)` if that alone empties a domain.
    pub fn new<T: Topology>(model: &CompiledModel, topo: &T, domains: &mut DomainStore, metrics: &mut Metrics) -> Result<Self, NodeId> {
        let pattern_count = model.pattern_count();
        let total_slots = topo.node_count() * topo.max_in_degree().max(1);
        let mut counts = vec![0u32; total_slots * pattern_count];
        let mut initially_unsupported: Vec<(NodeId, PatternId)> = Vec::new();
        for v in 0..topo.node_count() {
            let vn = NodeId::from_index(v);
            topo.for_each_in_arc(vn, |u, r, slot| {
                for a_idx in 0..pattern_count {
                    let ap = PatternId::from_index(a_idx);
                    let mut m = model.allowed(r, ap).clone();
                    m.and_with(domains.get(vn).bits());
                    let c = m.count_ones();
                    counts[slot * pattern_count + a_idx] = c;
                    if c == 0 && domains.get(u).bits().get(ap) {
                        initially_unsupported.push((u, ap));
                    }
                }
            });
        }

        // The same (node, pattern) pair can be discovered independently via more than one
        // incoming arc slot (each slot's own zero count triggers its own push); deduplicate before
        // treating this as a worklist — a duplicate entry would otherwise make `propagate` decrement
        // that removal's downstream support counters twice, over-pruning patterns that still had
        // support after the *single* real removal.
        initially_unsupported.sort_unstable_by_key(|&(n, p)| (n.get(), p.get()));
        initially_unsupported.dedup();

        let mut engine = Self { counts, pattern_count };
        for &(u, a) in &initially_unsupported {
            if !domains.get(u).bits().get(a) {
                continue; // already removed as a cascading consequence of an earlier entry below
            }
            if let RestrictResult::Wipeout = domains.get_mut(u).remove(a, model.weights()) {
                metrics.removals += 1;
                return Err(u);
            }
            metrics.removals += 1;
        }
        engine.propagate(model, topo, domains, &initially_unsupported, metrics)?;
        Ok(engine)
    }

    #[cfg(test)]
    pub(crate) fn count_at(&self, slot: usize, p: PatternId) -> u32 {
        self.counts[slot * self.pattern_count + p.index()]
    }

    /// ⚡ Propagates from a worklist of already-removed `(node, pattern)` pairs to a fixed point.
    /// `seed_removed` must reflect patterns actually absent from `domains` relative to whatever
    /// state this engine was [`Ac4Engine::new`]-initialized against. Returns `Err(node)` for the
    /// first domain wiped to empty — including a domain the *caller's own* pre-applied removal
    /// already wiped before this call even started: `propagate`'s internal decrement loop only
    /// ever inspects domains it removes patterns from itself, so a seed removal that was the last
    /// straw for its own node would otherwise be invisible here (an already-empty domain can only
    /// ever report `Unchanged`, never re-report `Wipeout`, the same hazard this crate's AC-3 search
    /// integration hit once with un-checked `Domain::remove` results).
    pub fn propagate<T: Topology>(&mut self, model: &CompiledModel, topo: &T, domains: &mut DomainStore, seed_removed: &[(NodeId, PatternId)], metrics: &mut Metrics) -> Result<(), NodeId> {
        for &(v, _) in seed_removed {
            if domains.get(v).is_wiped() {
                return Err(v);
            }
        }
        let mut queue: std::collections::VecDeque<(NodeId, PatternId)> = seed_removed.iter().copied().collect();
        let mut wipeout: Option<NodeId> = None;
        while let Some((v, b)) = queue.pop_front() {
            if wipeout.is_some() {
                break;
            }
            metrics.propagations += 1;
            topo.for_each_in_arc(v, |u, r, slot| {
                if wipeout.is_some() {
                    return;
                }
                for a in model.supporters(r, b).iter_ones() {
                    let idx = slot * self.pattern_count + a.index();
                    if self.counts[idx] == 0 {
                        continue;
                    }
                    self.counts[idx] -= 1;
                    if self.counts[idx] == 0 && domains.get(u).bits().get(a) {
                        let result = domains.get_mut(u).remove(a, model.weights());
                        metrics.removals += 1;
                        match result {
                            RestrictResult::Wipeout => wipeout = Some(u),
                            _ => queue.push_back((u, a)),
                        }
                    }
                }
            });
        }
        match wipeout {
            Some(n) => Err(n),
            None => Ok(()),
        }
    }

    /// 🩺 Debug-only: recomputes every counter from `domains`' current state and asserts it
    /// matches. `O(arcs * pattern_count^2)` — a correctness oracle, not a hot-path check. Clones
    /// `domains` so this stays read-only from the caller's perspective even though [`Ac4Engine::new`]
    /// itself mutates whatever store it is given.
    #[cfg(test)]
    pub(crate) fn debug_assert_consistent<T: Topology>(&self, model: &CompiledModel, topo: &T, domains: &DomainStore) {
        let mut scratch = domains.clone();
        let mut scratch_metrics = Metrics::default();
        let fresh = Self::new(model, topo, &mut scratch, &mut scratch_metrics).expect("recomputation from an already-consistent domain state cannot newly wipe out");
        assert_eq!(self.counts, fresh.counts, "AC-4 counters drifted from a from-scratch recomputation");
    }
}
// #endregion 🔖Engine

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitset::PatternSet;
    use crate::ids::RelationId;
    use crate::model::ModelBuilder;
    use crate::oracle::testgen;
    use crate::prop_ac3;
    use crate::propagate::PropQueue;
    use crate::topology::GraphTopologyBuilder;
    use crate::trail::Trail;

    fn checkerboard(n: usize) -> (CompiledModel, crate::topology::GraphTopology, RelationId) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        (model, tb.build().unwrap(), adj)
    }

    #[test]
    fn initial_counts_match_full_domain_supporter_popcounts() {
        let (model, topo, adj) = checkerboard(3);
        let mut domains = DomainStore::new_full(3, model.weights());
        let mut metrics = Metrics::default();
        let engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        // Node 1 has two incoming arcs (from 0 and from 2); each counted pattern's support should
        // equal the full popcount of allowed(adj, p) intersected with the (full) target domain.
        let mut slots = Vec::new();
        topo.for_each_in_arc(NodeId(1), |_, _, slot| slots.push(slot));
        assert_eq!(slots.len(), 2);
        for &slot in &slots {
            let expected = model.allowed(adj, PatternId(0)).count_ones();
            assert_eq!(engine.count_at(slot, PatternId(0)), expected);
        }
    }

    #[test]
    fn propagation_forces_alternation_after_one_pin() {
        let (model, topo, _adj) = checkerboard(4);
        let mut domains = DomainStore::new_full(4, model.weights());
        let mut metrics = Metrics::default();
        let mut engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let seed: Vec<_> = removed.iter_ones().map(|p| (NodeId(0), p)).collect();
        engine.propagate(&model, &topo, &mut domains, &seed, &mut metrics).unwrap();
        assert_eq!(domains.get(NodeId(0)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(1)).singleton(), Some(PatternId(1)));
        assert_eq!(domains.get(NodeId(2)).singleton(), Some(PatternId(0)));
        assert_eq!(domains.get(NodeId(3)).singleton(), Some(PatternId(1)));
        engine.debug_assert_consistent(&model, &topo, &domains);
    }

    #[test]
    fn odd_cycle_pin_propagates_to_wipeout() {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(5);
        for i in 0..4 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        tb.arc(NodeId(4), NodeId(0), adj);
        tb.arc(NodeId(0), NodeId(4), adj);
        let topo = tb.build().unwrap();

        let mut domains = DomainStore::new_full(5, model.weights());
        let mut metrics = Metrics::default();
        let mut engine = Ac4Engine::new(&model, &topo, &mut domains, &mut metrics).unwrap();
        let mut removed = PatternSet::new_empty(2);
        domains.get_mut(NodeId(0)).assign_collecting(PatternId(0), model.weights(), &mut removed);
        let seed: Vec<_> = removed.iter_ones().map(|p| (NodeId(0), p)).collect();
        assert!(engine.propagate(&model, &topo, &mut domains, &seed, &mut metrics).is_err());
    }

    /// A random model whose one relation is genuinely symmetric (`allowed(r,a,c) == allowed(r,c,a)`)
    /// and therefore self-consistent under its default self-inverse declaration (`model.validate()`
    /// passes). [`crate::oracle::testgen::random_model`] does *not* guarantee this — it independently
    /// coin-flips each ordered pair — which is fine for oracle-vs-search differential tests (the
    /// oracle checks whichever arcs are declared, symmetric or not, and a full backtracking search
    /// still converges to the true answer regardless of how tight any one propagator's fixed point
    /// is) but not for comparing two propagators' fixed points directly: this crate's AC-3 engine
    /// (per its own module docs) only reaches *full* arc-consistency when both directions of an
    /// edge encode the same well-formed constraint, which requires a validated, symmetric table.
    fn random_symmetric_model(rng: &mut mathematical_random::Rng, pattern_count: usize, density: f64) -> (CompiledModel, RelationId) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..pattern_count).map(|_| b.add_pattern(1.0 + rng.next_range(0, 5) as f64)).collect();
        let r = b.add_relation("r");
        for i in 0..pattern_count {
            for j in i..pattern_count {
                if rng.next_bool(density) {
                    b.allow(r, patterns[i], patterns[j]);
                    b.allow(r, patterns[j], patterns[i]);
                }
            }
        }
        let model = b.compile().unwrap();
        model.validate().expect("random_symmetric_model must always build a self-consistent relation");
        (model, r)
    }

    /// Regression test for a real bug: `Ac4Engine::new`'s initial "already unsupported" sweep can
    /// discover the same `(node, pattern)` pair via more than one incoming-arc slot, and feeding
    /// that list into `propagate` without deduplicating caused some support counters to be decremented
    /// twice for a single logical removal — over-pruning patterns that still had support. Applying
    /// a batch of removals up front (one `propagate` call with the whole worklist) must give
    /// exactly the same result as applying them one at a time (one `propagate` call each).
    #[test]
    fn sequential_and_batch_seed_application_agree() {
        let mut rng = mathematical_random::Rng::from_seed(4040);
        for trial in 0..100 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 8) as usize;
            let (model, r) = random_symmetric_model(&mut rng, pattern_count, 0.5);
            let arcs = testgen::random_arcs(&mut rng, node_count, r);
            let mut tb = GraphTopologyBuilder::new(node_count);
            for a in &arcs {
                tb.arc(a.from, a.to, a.relation);
            }
            let topo = tb.build().unwrap();

            let mut scratch = DomainStore::new_full(node_count, model.weights());
            let removal_count = rng.next_range(0, (node_count * pattern_count) as u64) as usize;
            let mut seed_events = Vec::new();
            for _ in 0..removal_count {
                let n = NodeId::from_index(rng.next_range(0, node_count as u64) as usize);
                let p = PatternId::from_index(rng.next_range(0, pattern_count as u64) as usize);
                if scratch.get(n).bits().get(p) {
                    scratch.get_mut(n).remove(p, model.weights());
                    seed_events.push((n, p));
                }
            }

            // Sequential: one propagate() call per removal, applied one at a time.
            let mut domains_seq = DomainStore::new_full(node_count, model.weights());
            let mut metrics_seq = Metrics::default();
            let seq_result = Ac4Engine::new(&model, &topo, &mut domains_seq, &mut metrics_seq).and_then(|mut engine| {
                for &(n, p) in &seed_events {
                    if !domains_seq.get(n).bits().get(p) {
                        continue;
                    }
                    domains_seq.get_mut(n).remove(p, model.weights());
                    engine.propagate(&model, &topo, &mut domains_seq, &[(n, p)], &mut metrics_seq)?;
                }
                Ok(())
            });

            // Batch: all removals applied up front, one propagate() call with the full worklist.
            let mut domains_batch = DomainStore::new_full(node_count, model.weights());
            let mut metrics_batch = Metrics::default();
            let batch_result = Ac4Engine::new(&model, &topo, &mut domains_batch, &mut metrics_batch).and_then(|mut engine| {
                let mut applied = Vec::new();
                for &(n, p) in &seed_events {
                    if domains_batch.get(n).bits().get(p) {
                        domains_batch.get_mut(n).remove(p, model.weights());
                        applied.push((n, p));
                    }
                }
                engine.propagate(&model, &topo, &mut domains_batch, &applied, &mut metrics_batch)
            });

            if let (Err(_), Ok(())) | (Ok(()), Err(_)) = (&seq_result, &batch_result) {
                eprintln!("DEBUG trial {trial}: pattern_count={pattern_count} node_count={node_count}");
                eprintln!("DEBUG arcs={arcs:?}");
                eprintln!("DEBUG seed_events={seed_events:?}");
                for a in 0..pattern_count {
                    for c in 0..pattern_count {
                        eprintln!("DEBUG allowed(r,{a},{c})={}", model.allowed(r, PatternId::from_index(a)).get(PatternId::from_index(c)));
                    }
                }
            }

            match (seq_result, batch_result) {
                (Ok(()), Ok(())) => {
                    for n in 0..node_count {
                        let nid = NodeId::from_index(n);
                        assert_eq!(domains_seq.get(nid).bits(), domains_batch.get(nid).bits(), "trial {trial} node {n}: sequential and batch seed application diverged");
                    }
                }
                (Err(_), Err(_)) => {}
                (a, b) => panic!("trial {trial}: sequential and batch seed application disagreed on satisfiability: sequential={a:?} batch={b:?}"),
            }
        }
    }

    mod quick {
        use super::*;

        /// The P6 gate: AC-3 and AC-4 must reach byte-identical fixed points (or agree that the
        /// state is contradictory) from the same random starting domains. Uses a validated,
        /// symmetric relation (see [`random_symmetric_model`]) — AC-3's forward-only restriction is
        /// only a full arc-consistency algorithm for well-formed models; comparing it against AC-4
        /// on a malformed one (e.g. a self-inverse relation with an asymmetric table, which
        /// `oracle::testgen::random_model` can produce) would compare two *different*, each
        /// internally-valid, propagation strengths rather than testing for a real disagreement.
        #[test]
        fn ac3_and_ac4_reach_identical_fixed_points_on_random_instances() {
            let mut rng = mathematical_random::Rng::from_seed(4040);
            for trial in 0..300 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 8) as usize;
                let (model, r) = random_symmetric_model(&mut rng, pattern_count, 0.5);
                let arcs = testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();

                let mut domains_a = DomainStore::new_full(node_count, model.weights());
                // Apply a random set of initial single-pattern removals to both copies identically.
                let mut seed_events = Vec::new();
                let removal_count = rng.next_range(0, (node_count * pattern_count) as u64) as usize;
                for _ in 0..removal_count {
                    let n = NodeId::from_index(rng.next_range(0, node_count as u64) as usize);
                    let p = PatternId::from_index(rng.next_range(0, pattern_count as u64) as usize);
                    if domains_a.get(n).bits().get(p) {
                        domains_a.get_mut(n).remove(p, model.weights());
                        seed_events.push((n, p));
                    }
                }
                let mut domains_b = DomainStore::new_full(node_count, model.weights());
                for &(n, p) in &seed_events {
                    domains_b.get_mut(n).remove(p, model.weights());
                }

                // AC-3 must start with every node dirty, not just the seed-touched ones — the same
                // full-graph seeding crate::search performs at solve start, and necessary so both
                // engines get a fair chance to discover patterns with zero support in the model
                // itself (independent of these specific removals).
                let mut queue = PropQueue::new(node_count);
                queue.push_all(node_count);
                let mut trail = Trail::new();
                let mut metrics_a = Metrics::default();
                let result_a = prop_ac3::run_to_fixed_point(&model, &topo, &mut domains_a, &mut queue, &mut trail, &mut metrics_a);

                // AC-4's own initial sweep (in `Ac4Engine::new`) is the AC-4-side equivalent of
                // AC-3's full-queue seeding above — it must run directly against `domains_b` (not
                // a throwaway full copy) since it mutates whatever store it is given. The random
                // seed removals are then applied on top and fed in as a further worklist, mirroring
                // a real decision's sibling removal.
                let mut metrics_b = Metrics::default();
                let result_b = match Ac4Engine::new(&model, &topo, &mut domains_b, &mut metrics_b) {
                    Err(w) => Err(w),
                    Ok(mut engine) => {
                        let mut applied = Vec::new();
                        for &(n, p) in &seed_events {
                            if domains_b.get(n).bits().get(p) {
                                domains_b.get_mut(n).remove(p, model.weights());
                                applied.push((n, p));
                            }
                        }
                        engine.propagate(&model, &topo, &mut domains_b, &applied, &mut metrics_b)
                    }
                };

                match (result_a, result_b) {
                    (Ok(()), Ok(())) => {
                        for n in 0..node_count {
                            let nid = NodeId::from_index(n);
                            assert_eq!(domains_a.get(nid).bits(), domains_b.get(nid).bits(), "trial {trial}: node {n} domains diverged between AC-3 and AC-4");
                        }
                    }
                    (Err(_), Err(_)) => {} // both detected a contradiction; exact wiped node may differ by processing order
                    (a, b) => panic!("trial {trial}: AC-3 and AC-4 disagreed on satisfiability: ac3={a:?} ac4={b:?}"),
                }
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Propac4

// #region 🔖Heuristics
pub mod heuristics {
//! 🎯 Observation heuristics: which unresolved variable the search collapses next. A plain linear
//! scan by design — the plan's own recommendation for small/reference solving (§23.7) — kept as a
//! single small function so a heap-accelerated variant can be swapped in later without touching
//! the public [`ObserveHeuristic`] enum or any call site.

use crate::domain::DomainStore;
use crate::ids::NodeId;

// #region 🔖Heuristic
/// 🎯 Which unresolved (`cardinality > 1`) variable to collapse next.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ObserveHeuristic {
    /// 🎯 Minimum remaining values — smallest domain cardinality first.
    #[default]
    Mrv,
    /// 🎯 Smallest incremental weighted Shannon entropy first.
    WeightedEntropy,
}
// #endregion 🔖Heuristic

// #region 🔖Select
#[derive(Clone, Copy, PartialEq, Debug)]
struct Key(f64, u32);

impl Key {
    fn better_than(self, other: Key) -> bool {
        match self.0.partial_cmp(&other.0).expect("heuristic key must be finite") {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => self.1 < other.1,
        }
    }
}

/// 🎯 The unresolved node with the smallest heuristic key, ties broken by ascending [`NodeId`]
/// for full determinism. `None` when every domain is already singleton (or, degenerately, wiped —
/// callers only reach this after propagation has already ruled that out).
pub(crate) fn select_unresolved(heuristic: ObserveHeuristic, domains: &DomainStore) -> Option<NodeId> {
    let mut best: Option<(NodeId, Key)> = None;
    for (n, d) in domains.iter() {
        if d.cardinality() <= 1 {
            continue;
        }
        let primary = match heuristic {
            ObserveHeuristic::Mrv => d.cardinality() as f64,
            ObserveHeuristic::WeightedEntropy => d.entropy(),
        };
        let key = Key(primary, n.get());
        match best {
            None => best = Some((n, key)),
            Some((_, bk)) => {
                if key.better_than(bk) {
                    best = Some((n, key));
                }
            }
        }
    }
    best.map(|(n, _)| n)
}
// #endregion 🔖Select

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::PatternId;
    use crate::weights::WeightTable;

    #[test]
    fn mrv_picks_smallest_domain() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(3, &w);
        store.get_mut(NodeId(1)).remove(PatternId(0), &w); // node 1: cardinality 3
        store.get_mut(NodeId(2)).remove(PatternId(0), &w);
        store.get_mut(NodeId(2)).remove(PatternId(1), &w); // node 2: cardinality 2
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(2));
    }

    #[test]
    fn ties_break_by_ascending_node_id() {
        let w = WeightTable::new(&[1.0, 1.0]).unwrap();
        let store = DomainStore::new_full(3, &w);
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(0));
    }

    #[test]
    fn singleton_and_resolved_domains_are_skipped() {
        let w = WeightTable::new(&[1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(2, &w);
        store.get_mut(NodeId(0)).assign(PatternId(0), &w);
        let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
        assert_eq!(picked, NodeId(1));
    }

    #[test]
    fn none_when_all_singleton() {
        let w = WeightTable::new(&[1.0]).unwrap();
        let store = DomainStore::new_full(2, &w);
        assert!(select_unresolved(ObserveHeuristic::Mrv, &store).is_none());
    }

    #[test]
    fn weighted_entropy_prefers_lower_entropy_domain() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
        let mut store = DomainStore::new_full(2, &w);
        // node 1 has cardinality 2 (lower entropy) after one removal; node 0 stays at cardinality 4.
        store.get_mut(NodeId(1)).remove(PatternId(0), &w);
        store.get_mut(NodeId(1)).remove(PatternId(1), &w);
        let picked = select_unresolved(ObserveHeuristic::WeightedEntropy, &store).unwrap();
        assert_eq!(picked, NodeId(1));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Heuristics

// #region 🔖Sample
pub mod sample {
//! 🎲 Value sampling: which pattern a decision assigns from the selected node's live domain.

use crate::domain::Domain;
use crate::ids::PatternId;
use crate::model::CompiledModel;
use mathematical_random::Rng;

// #region 🔖Sampler
/// 🎲 How one pattern is chosen from an unresolved domain.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ValueSampler {
    /// 🎲 Probability proportional to pattern weight.
    #[default]
    WeightedRoulette,
    /// 🎲 Every live pattern equally likely, ignoring weight.
    Uniform,
}

/// 🎲 Draws one pattern from `domain` (must be non-empty) according to `sampler`.
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
// #endregion 🔖Sampler

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;

    fn model_and_domain(weights: &[f64]) -> (CompiledModel, Domain) {
        let mut b = ModelBuilder::new();
        for &w in weights {
            b.add_pattern(w);
        }
        b.add_relation("r");
        let model = b.compile().unwrap();
        let domain = Domain::new_full(model.weights());
        (model, domain)
    }

    #[test]
    fn uniform_only_ever_returns_live_patterns() {
        let (model, domain) = model_and_domain(&[1.0, 1.0, 1.0]);
        let mut rng = Rng::from_seed(1);
        for _ in 0..50 {
            let p = sample_pattern(ValueSampler::Uniform, &domain, &model, &mut rng);
            assert!(domain.bits().get(p));
        }
    }

    #[test]
    fn weighted_roulette_only_ever_returns_live_patterns() {
        let (model, domain) = model_and_domain(&[1.0, 5.0, 10.0]);
        let mut rng = Rng::from_seed(2);
        for _ in 0..50 {
            let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
            assert!(domain.bits().get(p));
        }
    }

    #[test]
    fn weighted_roulette_is_biased_toward_heavier_pattern() {
        let (model, domain) = model_and_domain(&[1.0, 99.0]);
        let mut rng = Rng::from_seed(3);
        let mut counts = [0u32; 2];
        for _ in 0..2000 {
            let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
            counts[p.index()] += 1;
        }
        assert!(counts[1] > counts[0] * 10);
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let (model, domain) = model_and_domain(&[1.0, 2.0, 3.0]);
        let mut r1 = Rng::from_seed(42);
        let mut r2 = Rng::from_seed(42);
        for _ in 0..20 {
            let p1 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r1);
            let p2 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r2);
            assert_eq!(p1, p2);
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Sample

// #region 🔖Trail
pub(crate) mod trail {
//! ↩️ The search trail: an append-only log of every pattern removal, grouped into decision frames
//! so backtracking can undo exactly one decision's consequences (including everything propagation
//! did because of it) in one call.

use crate::bitset::PatternSet;
use crate::domain::DomainStore;
use crate::ids::{DecisionId, NodeId, PatternId};
use crate::weights::WeightTable;

// #region 🔖Entry
#[derive(Clone, Copy, PartialEq, Debug)]
struct RemovedEntry {
    node: NodeId,
    pattern: PatternId,
}

/// ↩️ Everything needed to undo one decision and retry with a different pattern: which node was
/// decided, which pattern was tried, where the trail stood before the decision, and the RNG state
/// right before that pattern was sampled (so a replay reproduces the exact same draw).
#[derive(Clone, Copy, Debug)]
// `decision`/`rng_state` are read by trace replay and checkpointing (a later phase); the current
// search driver uses the "keep RNG, don't restore" policy, so they're written but not yet read.
#[allow(dead_code)]
pub(crate) struct DecisionFrame {
    pub decision: DecisionId,
    pub node: NodeId,
    pub candidate: PatternId,
    pub trail_mark: usize,
    pub rng_state: [u64; 4],
}
// #endregion 🔖Entry

// #region 🔖Trail
/// ↩️ Append-only removal log plus a decision-frame stack.
#[derive(Clone, Debug, Default)]
pub(crate) struct Trail {
    entries: Vec<RemovedEntry>,
    frames: Vec<DecisionFrame>,
}

impl Trail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_removed(&mut self, node: NodeId, pattern: PatternId) {
        self.entries.push(RemovedEntry { node, pattern });
    }

    pub fn record_removed_set(&mut self, node: NodeId, removed: &PatternSet) {
        for p in removed.iter_ones() {
            self.record_removed(node, p);
        }
    }

    pub fn push_frame(&mut self, decision: DecisionId, node: NodeId, candidate: PatternId, rng_state: [u64; 4]) {
        self.frames.push(DecisionFrame { decision, node, candidate, trail_mark: self.entries.len(), rng_state });
    }

    pub fn pop_frame(&mut self) -> Option<DecisionFrame> {
        self.frames.pop()
    }

    /// ↩️ Replays removal entries in exact reverse order down to (but not including) `mark`,
    /// re-adding each pattern. Order matters: entries must be undone LIFO so a pattern removed
    /// twice by different propagation steps is restored to the state each undo expects.
    pub fn undo_to(&mut self, mark: usize, domains: &mut DomainStore, w: &WeightTable) {
        while self.entries.len() > mark {
            let entry = self.entries.pop().expect("checked entries.len() > mark > 0 above");
            domains.get_mut(entry.node).re_add(entry.pattern, w);
        }
    }

    #[inline]
    #[allow(dead_code)] // used by checkpointing (a later phase); exercised today only by trail tests
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    #[allow(dead_code)] // backs the public decision_depth() query added with the solver step API (a later phase)
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
}
// #endregion 🔖Trail

// #region 🔖Checkpoint
/// ↩️ A resumable snapshot of a solve's domain state. Deliberately lighter than a full trail/
/// decision-stack serialization: resuming re-seeds a fresh search from these domains (via the same
/// `init_domains` path a heterogeneous-domain build already uses), so it is sound — the domains
/// already reflect every permanent removal made before the checkpoint — but backtracking after
/// resume can only undo decisions made *after* resume, not the ones baked into the snapshot.
#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub domains: Vec<PatternSet>,
    pub model_fingerprint: u64,
    pub seed: u64,
}

impl Checkpoint {
    pub fn new(domains: Vec<PatternSet>, model_fingerprint: u64, seed: u64) -> Self {
        Self { domains, model_fingerprint, seed }
    }
}
// #endregion 🔖Checkpoint

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::weights::WeightTable;

    #[test]
    fn undo_to_restores_removed_patterns() {
        let w = WeightTable::new(&[1.0, 2.0, 3.0]).unwrap();
        let mut domains = DomainStore::new_full(1, &w);
        let mut trail = Trail::new();
        domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
        trail.record_removed(NodeId(0), PatternId(0));
        domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
        trail.record_removed(NodeId(0), PatternId(1));
        assert_eq!(domains.get(NodeId(0)).cardinality(), 1);

        trail.undo_to(0, &mut domains, &w);
        assert_eq!(domains.get(NodeId(0)).cardinality(), 3);
        domains.get(NodeId(0)).debug_assert_consistent(&w);
    }

    #[test]
    fn undo_to_partial_mark_restores_only_later_entries() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0]).unwrap();
        let mut domains = DomainStore::new_full(1, &w);
        let mut trail = Trail::new();
        domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
        trail.record_removed(NodeId(0), PatternId(0));
        let mark = trail.len();
        domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
        trail.record_removed(NodeId(0), PatternId(1));

        trail.undo_to(mark, &mut domains, &w);
        assert_eq!(domains.get(NodeId(0)).cardinality(), 2); // pattern 1 restored, pattern 0 still gone
        assert!(!domains.get(NodeId(0)).bits().get(PatternId(0)));
        assert!(domains.get(NodeId(0)).bits().get(PatternId(1)));
    }

    #[test]
    fn decision_frames_push_and_pop() {
        let mut trail = Trail::new();
        trail.push_frame(DecisionId(0), NodeId(1), PatternId(2), [1, 2, 3, 4]);
        assert_eq!(trail.depth(), 1);
        let frame = trail.pop_frame().unwrap();
        assert_eq!(frame.node, NodeId(1));
        assert_eq!(frame.candidate, PatternId(2));
        assert_eq!(trail.depth(), 0);
        assert!(trail.pop_frame().is_none());
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Trail

// #region 🔖Constraint
pub mod constraint {
//! 🧷 Global constraints beyond binary arc compatibility. Deliberately stateless (no per-constraint
//! mutable state to roll back on backtrack — the exact class of subtle bug this crate already hit
//! twice, once in `crate::search`'s trail and once in `crate::prop_ac4`'s counters): a constraint
//! only ever (1) restricts initial domains once, before search starts (no rollback needed — the
//! same treatment `crate::search::solve` already gives fixed pins and per-node domain overrides),
//! and (2) validates a *complete* (all-singleton) candidate assignment, rejecting it — which
//! `crate::search`'s existing, proven `backtrack_and_repair` machinery turns into an ordinary
//! backtrack, exactly as if a domain had been wiped. Full incremental mid-search propagation for
//! global constraints (tightening domains as decisions narrow them, not just at the two safe
//! points above) is deferred alongside AC-4's rollback integration.

use crate::bitset::PatternSet;
use crate::domain::DomainStore;
use crate::error::ConstraintError;
use crate::ids::{NodeId, PatternId, RegionId};
use crate::model::CompiledModel;
use crate::weights::WeightTable;

// #region 🔖Selector
/// 🧷 Which patterns a node counts as "selected" for a constraint — shared by every constraint
/// type in this crate so a caller building, say, a cardinality *and* a connectivity constraint
/// over the same "walkable" patterns writes the selector once.
#[derive(Clone, Debug)]
pub enum PatternSelector {
    Pattern(PatternId),
    Tag(u32),
    Any(PatternSet),
}

impl PatternSelector {
    pub fn matches(&self, model: &CompiledModel, p: PatternId) -> bool {
        match self {
            PatternSelector::Pattern(target) => *target == p,
            PatternSelector::Tag(tag) => model.pattern_info(p).tags.contains(tag),
            PatternSelector::Any(set) => set.get(p),
        }
    }

    pub fn as_pattern_set(&self, model: &CompiledModel) -> PatternSet {
        match self {
            PatternSelector::Any(set) => set.clone(),
            _ => {
                let mut set = PatternSet::new_empty(model.pattern_count());
                for i in 0..model.pattern_count() {
                    let p = PatternId::from_index(i);
                    if self.matches(model, p) {
                        set.set(p, true);
                    }
                }
                set
            }
        }
    }
}
// #endregion 🔖Selector

// #region 🔖Adjacency
/// 🧷 A materialized, object-safe neighbor view — built once per solve from whatever concrete
/// `Topology` is in play, so constraints (which must work identically across `GraphTopology`,
/// `Grid2dTopology`, `Grid3dTopology`) never need `Topology` itself to be object-safe.
#[derive(Clone, Debug)]
pub struct AdjacencyView {
    neighbors: Vec<Vec<NodeId>>,
    regions: Vec<RegionId>,
}

impl AdjacencyView {
    pub(crate) fn new(neighbors: Vec<Vec<NodeId>>, regions: Vec<RegionId>) -> Self {
        debug_assert_eq!(neighbors.len(), regions.len());
        Self { neighbors, regions }
    }

    pub fn node_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn neighbors(&self, n: NodeId) -> &[NodeId] {
        &self.neighbors[n.index()]
    }

    pub fn region_of(&self, n: NodeId) -> RegionId {
        self.regions[n.index()]
    }
}

/// 🧷 Materializes an [`AdjacencyView`] from any concrete `Topology` — the one place this crate
/// converts the hot-path, non-object-safe `Topology` trait into the object-safe shape constraints
/// need. Called once per solver `build()`, not per solve attempt.
pub(crate) fn build_adjacency_view<T: crate::topology::Topology>(topo: &T) -> AdjacencyView {
    let node_count = topo.node_count();
    let mut neighbors = vec![Vec::new(); node_count];
    let mut regions = vec![RegionId(0); node_count];
    for i in 0..node_count {
        let n = NodeId::from_index(i);
        topo.for_each_out_arc(n, |m, _r| neighbors[i].push(m));
        regions[i] = topo.region_of(n);
    }
    AdjacencyView::new(neighbors, regions)
}
// #endregion 🔖Adjacency

// #region 🔖Constraint
/// 🧷 Whether a constraint's [`Constraint::validate_complete`] is a sound-and-complete check (a
/// failure there always means the assignment is genuinely invalid — safe to use in exhaustive/
/// unsat-proof search) or merely a heuristic approximation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exactness {
    Exact,
    Heuristic,
}

/// 🧷 One global constraint.
pub trait Constraint {
    fn name(&self) -> &'static str;
    fn exactness(&self) -> Exactness;

    /// 🧷 Restricts initial per-node domains once, before search starts. Returning a narrower
    /// `PatternSet` than a node's current entry in `domains` intersects it in; returning the same
    /// set is a no-op. Called once per solve attempt, before the first propagation pass.
    fn initialize(&self, domains: &DomainStore, weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError>;

    /// 🧷 Checks one complete (every node singleton) candidate assignment. `Ok(())` means this
    /// constraint accepts it.
    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String>;
}

/// 🧷 A solver's constraints plus the adjacency view they read — bundled so `crate::search`'s
/// internals take one extra `Option<&ConstraintSet>` parameter instead of two.
pub(crate) struct ConstraintSet<'a> {
    pub constraints: &'a [Box<dyn Constraint>],
    pub adjacency: &'a AdjacencyView,
}
// #endregion 🔖Constraint

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacency_view_exposes_neighbors_and_regions() {
        let view = AdjacencyView::new(vec![vec![NodeId(1)], vec![NodeId(0), NodeId(2)], vec![NodeId(1)]], vec![RegionId(0), RegionId(1), RegionId(0)]);
        assert_eq!(view.node_count(), 3);
        assert_eq!(view.neighbors(NodeId(1)), &[NodeId(0), NodeId(2)]);
        assert_eq!(view.region_of(NodeId(1)), RegionId(1));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Constraint

// #region 🔖Constraintscard
pub mod constraints_card {
//! 🔢 Cardinality constraints: bound how many nodes (in some scope) end up assigned a pattern
//! matching a selector.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
use crate::domain::DomainStore;
use crate::error::ConstraintError;
use crate::ids::{NodeId, PatternId, RegionId};
use crate::model::CompiledModel;
use crate::weights::WeightTable;

// #region 🔖Scope
/// 🔢 Which nodes a cardinality bound applies to.
#[derive(Clone, Debug)]
pub enum Scope {
    All,
    Region(RegionId),
    Nodes(Vec<NodeId>),
}

impl Scope {
    fn contains(&self, n: NodeId, adjacency: &AdjacencyView) -> bool {
        match self {
            Scope::All => true,
            Scope::Region(r) => adjacency.region_of(n) == *r,
            Scope::Nodes(nodes) => nodes.contains(&n),
        }
    }
}
// #endregion 🔖Scope

// #region 🔖Constraint
/// 🔢 Requires that between `min` and `max` (inclusive) nodes in `scope` end up matching
/// `selector`. `min == max` is an exact-count constraint.
#[derive(Clone, Debug)]
pub struct CardinalityConstraint {
    pub selector: PatternSelector,
    pub scope: Scope,
    pub min: u32,
    pub max: u32,
    model: CompiledModel,
}

impl CardinalityConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector, scope: Scope, min: u32, max: u32) -> Result<Self, ConstraintError> {
        if min > max {
            return Err(ConstraintError::InvalidBounds { reason: "min must not exceed max" });
        }
        Ok(Self { selector, scope, min, max, model })
    }
}

impl Constraint for CardinalityConstraint {
    fn name(&self) -> &'static str {
        "cardinality"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, domains: &DomainStore, _weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        let selected = self.selector.as_pattern_set(&self.model);
        let scoped_nodes: Vec<NodeId> = (0..adjacency.node_count()).map(NodeId::from_index).filter(|&n| self.scope.contains(n, adjacency)).collect();

        // How many scoped nodes could still take a selected pattern, and how many are already
        // forced to (their domain is a selected-pattern-only singleton)?
        let possible = scoped_nodes.iter().filter(|&&n| domains.get(n).bits().intersects(&selected)).count() as u32;
        let required = scoped_nodes.iter().filter(|&&n| domains.get(n).bits().is_subset_of(&selected)).count() as u32;

        let mut out = Vec::new();
        if self.max < required {
            // Already over-required with no way to satisfy `max` — signal via an emptied domain
            // on the first scoped node so the caller's normal wipeout handling takes over.
            if let Some(&n) = scoped_nodes.first() {
                out.push((n, PatternSet::new_empty(self.model.pattern_count())));
            }
            return Ok(out);
        }
        if possible < self.min {
            if let Some(&n) = scoped_nodes.first() {
                out.push((n, PatternSet::new_empty(self.model.pattern_count())));
            }
            return Ok(out);
        }
        if possible == self.min && self.min > required {
            // Every possible-but-not-yet-required node must take the selected pattern to reach `min`.
            for &n in &scoped_nodes {
                let bits = domains.get(n).bits();
                if bits.intersects(&selected) && !bits.is_subset_of(&selected) {
                    out.push((n, selected.clone()));
                }
            }
        }
        if self.max == required {
            // No further scoped node may take the selected pattern.
            let mut not_selected = selected.clone();
            not_selected.clear_all();
            for i in 0..self.model.pattern_count() {
                not_selected.set(PatternId::from_index(i), !selected.get(PatternId::from_index(i)));
            }
            for &n in &scoped_nodes {
                let bits = domains.get(n).bits();
                if bits.intersects(&selected) && !bits.is_subset_of(&selected) {
                    out.push((n, not_selected.clone()));
                }
            }
        }
        Ok(out)
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let count = (0..adjacency.node_count()).filter(|&n| self.scope.contains(NodeId::from_index(n), adjacency)).filter(|&n| self.selector.matches(&self.model, assignment[n])).count() as u32;
        if count < self.min || count > self.max {
            return Err(format!("cardinality constraint: expected [{}, {}], found {count}", self.min, self.max));
        }
        Ok(())
    }
}
// #endregion 🔖Constraint

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;

    fn two_pattern_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let r = b.add_relation("adj");
        b.allow_mirrored(r, black, white);
        b.allow_mirrored(r, black, black);
        b.compile().unwrap()
    }

    fn adjacency_line(n: usize) -> AdjacencyView {
        let mut neighbors = vec![Vec::new(); n];
        for i in 0..n.saturating_sub(1) {
            neighbors[i].push(NodeId::from_index(i + 1));
            neighbors[i + 1].push(NodeId::from_index(i));
        }
        AdjacencyView::new(neighbors, vec![RegionId(0); n])
    }

    #[test]
    fn rejects_invalid_bounds() {
        let model = two_pattern_model();
        assert!(CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 5, 2).is_err());
    }

    #[test]
    fn validate_complete_accepts_matching_count() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(4);
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
        let assignment = vec![PatternId(0), PatternId(1), PatternId(0), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    fn validate_complete_rejects_out_of_bounds_count() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(4);
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 0, 1).unwrap();
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }

    #[test]
    fn initialize_forces_pattern_when_min_equals_possible() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(2);
        let weights = model.weights().clone();
        let mut domains = DomainStore::new_full(2, &weights);
        // Restrict node1 so only node0 can possibly satisfy "at least 1 pattern0".
        let mut white_only = PatternSet::new_empty(2);
        white_only.set(PatternId(1), true);
        domains.get_mut(NodeId(1)).restrict(&white_only, &weights);

        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
        let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
        assert!(restrictions.iter().any(|(n, set)| *n == NodeId(0) && set.get(PatternId(0)) && !set.get(PatternId(1))));
    }

    #[test]
    fn initialize_signals_infeasible_min_via_empty_domain() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(1);
        let weights = model.weights().clone();
        let domains = DomainStore::new_full(1, &weights);
        // Impossible: need at least 2 nodes matching pattern0, but only 1 node total.
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 2, 2).unwrap();
        let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
        assert!(restrictions.iter().any(|(_, set)| set.is_all_zero()));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Constraintscard

// #region 🔖Constraintsconn
pub mod constraints_conn {
//! 🔗 Connectivity and reachability constraints, checked exactly at completion via a small
//! hand-rolled union-find (`mathematical_graph`'s own union-find lives in a private region of that
//! crate, so this crate owns a minimal one rather than reaching into it).

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
use crate::domain::DomainStore;
use crate::error::ConstraintError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::weights::WeightTable;

// #region 🔖UnionFind
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }
}
// #endregion 🔖UnionFind

// #region 🔖Connectivity
/// 🔗 Requires that every node whose assigned pattern matches `selector` forms exactly one
/// connected component (using the solver's own adjacency — two selected nodes are connected iff
/// there is a path between them through other selected nodes).
#[derive(Clone, Debug)]
pub struct ConnectivityConstraint {
    pub selector: PatternSelector,
    model: CompiledModel,
}

impl ConnectivityConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector) -> Self {
        Self { selector, model }
    }
}

impl Constraint for ConnectivityConstraint {
    fn name(&self) -> &'static str {
        "connectivity"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let selected: Vec<usize> = (0..assignment.len()).filter(|&n| self.selector.matches(&self.model, assignment[n])).collect();
        if selected.len() <= 1 {
            return Ok(());
        }
        let mut uf = UnionFind::new(assignment.len());
        for &n in &selected {
            for &m in adjacency.neighbors(NodeId::from_index(n)) {
                if self.selector.matches(&self.model, assignment[m.index()]) {
                    uf.union(n, m.index());
                }
            }
        }
        let root = uf.find(selected[0]);
        if selected.iter().all(|&n| uf.find(n) == root) {
            Ok(())
        } else {
            Err(format!("connectivity constraint: {} selected nodes do not form one connected component", selected.len()))
        }
    }
}
// #endregion 🔖Connectivity

// #region 🔖Reachability
/// 🔗 Requires that every node in `to` is reachable from every node in `from`, moving only through
/// nodes whose assigned pattern matches `selector` (endpoints themselves must also match).
#[derive(Clone, Debug)]
pub struct ReachabilityConstraint {
    pub from: Vec<NodeId>,
    pub to: Vec<NodeId>,
    pub selector: PatternSelector,
    model: CompiledModel,
}

impl ReachabilityConstraint {
    pub fn new(model: CompiledModel, from: Vec<NodeId>, to: Vec<NodeId>, selector: PatternSelector) -> Self {
        Self { from, to, selector, model }
    }
}

impl Constraint for ReachabilityConstraint {
    fn name(&self) -> &'static str {
        "reachability"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        for &start in &self.from {
            if !self.selector.matches(&self.model, assignment[start.index()]) {
                return Err(format!("reachability constraint: source node {start} is not itself selected"));
            }
            let mut visited = vec![false; assignment.len()];
            let mut stack = vec![start];
            visited[start.index()] = true;
            while let Some(n) = stack.pop() {
                for &m in adjacency.neighbors(n) {
                    if !visited[m.index()] && self.selector.matches(&self.model, assignment[m.index()]) {
                        visited[m.index()] = true;
                        stack.push(m);
                    }
                }
            }
            for &target in &self.to {
                if !visited[target.index()] {
                    return Err(format!("reachability constraint: {target} is not reachable from {start}"));
                }
            }
        }
        Ok(())
    }
}
// #endregion 🔖Reachability

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;

    fn floor_wall_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let floor = b.add_pattern(1.0);
        let wall = b.add_pattern(1.0);
        let r = b.add_relation("adj");
        b.allow_mirrored(r, floor, floor);
        b.allow_mirrored(r, floor, wall);
        b.allow_mirrored(r, wall, wall);
        b.compile().unwrap()
    }

    /// A 5-node "H" shape: 0-1-2, 2-3, 2-4 (so node2 is a hub).
    fn hub_adjacency() -> AdjacencyView {
        let edges = [(0, 1), (1, 2), (2, 3), (2, 4)];
        let mut neighbors = vec![Vec::new(); 5];
        for &(a, b) in &edges {
            neighbors[a].push(NodeId::from_index(b));
            neighbors[b].push(NodeId::from_index(a));
        }
        AdjacencyView::new(neighbors, vec![crate::ids::RegionId(0); 5])
    }

    #[test]
    fn connectivity_accepts_single_connected_component() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        // floor at 0,1,2 (connected through the hub), wall elsewhere.
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    fn connectivity_rejects_split_components() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        // floor at 0 and at 3+4, split by wall at the hub (node2) and node1.
        let assignment = vec![PatternId(0), PatternId(1), PatternId(1), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }

    #[test]
    fn connectivity_trivially_accepts_zero_or_one_selected() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        let all_wall = vec![PatternId(1); 5];
        assert!(c.validate_complete(&all_wall, &adjacency).is_ok());
    }

    #[test]
    fn reachability_accepts_connected_path() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3), NodeId(4)], PatternSelector::Pattern(PatternId(0)));
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    fn reachability_rejects_blocked_path() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3)], PatternSelector::Pattern(PatternId(0)));
        // Wall at the hub (node2) blocks the only path from 0 to 3.
        let assignment = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Constraintsconn

// #region 🔖Soft
pub mod soft {
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

/// 🎯 Runs `n` independent seeded attempts through a caller-supplied solve closure, scores each
/// successful one, and returns the best-scoring [`Attempt`] alongside every attempt's outcome (so
/// a caller can see how many of the `n` seeds actually found a solution at all).
pub struct Attempt {
    pub seed: u64,
    pub assignment: Vec<PatternId>,
    pub score: f64,
}

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
}
// #endregion 🔖Soft

// #region 🔖Search
pub mod search {
//! 🌳 The search driver: observe → sample → propagate → (on contradiction) chronologically
//! backtrack, until every domain is a singleton (solved), the trail's root frame is exhausted
//! (unsatisfiable — every branch of the search tree was visited), or a budget/restart/cancel
//! signal stops the attempt short of either conclusion.

use crate::bitset::PatternSet;
use crate::constraint::ConstraintSet;
use crate::diag::{DiagLevel, Event, EventSink, Metrics};
use crate::domain::{DomainStore, RestrictResult};
use crate::heuristics::{self, ObserveHeuristic};
use crate::ids::{DecisionId, NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
use crate::prop_ac3;
use crate::propagate::PropQueue;
use crate::sample::{self, ValueSampler};
use crate::topology::Topology;
use crate::trail::Trail;
use mathematical_random::Rng;

// #region 🔖Config
/// 🌳 Whether a failed attempt restarts from scratch or resumes chronological backtracking.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SearchMode {
    /// 🌳 On contradiction, undo decisions up to [`SearchConfig::restart_schedule`]'s per-attempt
    /// backtrack budget (or the whole tree, if [`RestartSchedule::Never`]); when that budget or
    /// the tree itself is exhausted, discard the attempt and start fresh with a new seed. Never
    /// proves unsatisfiability, even if an attempt happens to exhaust its whole local tree.
    RestartOnly,
    /// 🌳 On contradiction, undo the most recent decision and try the next candidate. Exhausting
    /// every alternative back to the first decision proves unsatisfiability.
    #[default]
    Backtrack,
    /// 🌳 Semantically identical to [`SearchMode::Backtrack`] today (same completeness and
    /// soundness guarantees) — true conflict-directed jump-target selection is deferred to land
    /// alongside nogood learning (a later phase), since accelerating the jump without also
    /// recording *why* the skipped decisions were irrelevant risks silently losing completeness.
    /// Selecting this mode is forward-compatible: behavior only gets faster later, never different.
    Backjump,
}

/// 🌳 Limits that stop a solve attempt before it concludes either way.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Budget {
    pub max_observations: Option<u64>,
    pub max_backtracks: Option<u64>,
    pub max_millis: Option<u64>,
}

/// 🌳 The per-attempt backtrack budget schedule for [`SearchMode::RestartOnly`]. Ignored by
/// [`SearchMode::Backtrack`]/[`SearchMode::Backjump`], which always run to full completion (or an
/// explicit [`Budget`] limit) to preserve their unsat-proof guarantee.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum RestartSchedule {
    /// 🌳 No schedule-driven cap — an attempt only restarts when its own local tree is fully
    /// exhausted or the global [`Budget`] stops it.
    #[default]
    Never,
    /// 🌳 Every attempt gets the same backtrack budget.
    Fixed(u64),
    /// 🌳 Attempt `i`'s budget is `base * factor.powi(i)`.
    Geometric { base: u64, factor: f64 },
    /// 🌳 Attempt `i`'s budget is `unit * luby(i + 1)` (the standard Luby restart sequence).
    Luby(u64),
}

impl RestartSchedule {
    fn backtrack_budget(&self, attempt: u64) -> Option<u64> {
        match *self {
            RestartSchedule::Never => None,
            RestartSchedule::Fixed(n) => Some(n),
            RestartSchedule::Geometric { base, factor } => Some((base as f64 * factor.powi(attempt.min(62) as i32)) as u64),
            RestartSchedule::Luby(unit) => Some(luby(attempt + 1).saturating_mul(unit)),
        }
    }
}

/// 🌳 The standard Luby sequence (1-indexed): `1,1,2,1,1,2,4,1,1,2,1,1,2,4,8,...`.
fn luby(i: u64) -> u64 {
    let mut k = 1u32;
    while (1u64 << k) - 1 < i {
        k += 1;
    }
    if i == (1u64 << k) - 1 { 1u64 << (k - 1) } else { luby(i - (1u64 << (k - 1)) + 1) }
}

/// 🌳 A shareable, thread-safe flag a caller can set to stop a solve early.
#[derive(Clone, Debug, Default)]
pub struct CancelToken(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl CancelToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// 🌳 Everything [`crate::solver_graph::GraphSolver`] (and later grid solvers) needs to configure
/// one solve.
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchConfig {
    pub mode: SearchMode,
    pub heuristic: ObserveHeuristic,
    pub sampler: ValueSampler,
    pub budget: Budget,
    /// 🌳 [`SearchMode::RestartOnly`] only: gives up entirely after this many failed attempts.
    pub max_restarts: Option<u64>,
    /// 🌳 [`SearchMode::RestartOnly`] only: per-attempt backtrack budget schedule.
    pub restart_schedule: RestartSchedule,
    pub diag_level: DiagLevel,
}
// #endregion 🔖Config

// #region 🔖Repair
enum RepairOutcome {
    Repaired,
    /// 🌳 No more frames to pop — the (local or whole) search tree is exhausted.
    Exhausted,
    BudgetExceeded,
    /// 🌳 [`SearchMode::RestartOnly`]'s per-attempt backtrack budget ran out before either
    /// repairing or exhausting the tree.
    LocalLimitReached,
}

/// 🌳 Chronologically unwinds decisions until the most recently wiped domain (if any) is resolved
/// — undoing a single frame is not always enough, since a contradiction can be the combined
/// consequence of several decisions; this keeps unwinding until no domain is left wiped rather
/// than trusting the next propagation pass to notice on its own (an already-empty domain can only
/// ever report `Unchanged`, never re-report `Wipeout`, so a silent leftover wipeout would
/// otherwise never be caught).
#[allow(clippy::too_many_arguments)]
fn backtrack_and_repair<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    budget: &Budget,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    local_remaining: &mut Option<u64>,
    sink: &mut EventSink,
) -> RepairOutcome {
    loop {
        if let Some(rem) = local_remaining {
            if *rem == 0 {
                return RepairOutcome::LocalLimitReached;
            }
            *rem -= 1;
        }
        metrics.backtracks += 1;
        if let Some(max_bt) = budget.max_backtracks {
            if metrics.backtracks >= max_bt {
                return RepairOutcome::BudgetExceeded;
            }
        }
        let frame = match trail.pop_frame() {
            Some(f) => f,
            None => return RepairOutcome::Exhausted,
        };
        trail.undo_to(frame.trail_mark, domains, model.weights());
        if domains.any_wiped() {
            // This frame's decision was not the (sole) cause; keep unwinding without wasting a
            // repair attempt on a node that can't possibly fix a still-wiped domain elsewhere.
            continue;
        }
        let repair_result = domains.get_mut(frame.node).remove(frame.candidate, model.weights());
        trail.record_removed(frame.node, frame.candidate);
        sink.emit_detailed(Event::Backtracked { node: frame.node, candidate: frame.candidate });

        let contradiction = match repair_result {
            RestrictResult::Wipeout => Some(frame.node),
            _ => {
                queue.clear();
                queue.push(frame.node);
                prop_ac3::run_to_fixed_point(model, topo, domains, queue, trail, metrics).err()
            }
        };
        if contradiction.is_none() {
            debug_assert!(!domains.any_wiped());
            return RepairOutcome::Repaired;
        }
    }
}
// #endregion 🔖Repair

// #region 🔖Drive
enum StepOutcome {
    Solved,
    /// 🌳 Every alternative at the root decision has been tried — the (local or whole) search
    /// tree is exhausted.
    Exhausted,
    BudgetExceeded,
    LocalLimitReached,
    Cancelled,
}

/// 🧷 Whether every constraint accepts the current (assumed all-singleton) domain state. `true`
/// (vacuously) when there are no constraints to check.
fn constraints_accept(domains: &DomainStore, constraints: Option<&ConstraintSet<'_>>) -> bool {
    let Some(cs) = constraints else { return true };
    let assignment: Vec<PatternId> = domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect();
    cs.constraints.iter().all(|c| c.validate_complete(&assignment, cs.adjacency).is_ok())
}

#[allow(clippy::too_many_arguments)]
fn decide_and_propagate<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    node: NodeId,
    sink: &mut EventSink,
) -> Option<NodeId> {
    let rng_snapshot = rng.state();
    let candidate = sample::sample_pattern(config.sampler, domains.get(node), model, rng);
    trail.push_frame(DecisionId(*decision_counter), node, candidate, rng_snapshot);
    *decision_counter += 1;
    sink.emit_detailed(Event::Observed { node, chosen: candidate });

    let mut removed = PatternSet::new_empty(model.pattern_count());
    domains.get_mut(node).assign_collecting(candidate, model.weights(), &mut removed);
    trail.record_removed_set(node, &removed);

    queue.clear();
    queue.push(node);
    prop_ac3::run_to_fixed_point(model, topo, domains, queue, trail, metrics).err()
}

#[allow(clippy::too_many_arguments)]
fn drive<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    start: std::time::Instant,
    cancel: Option<&CancelToken>,
    local_backtrack_budget: Option<u64>,
    constraints: Option<&ConstraintSet<'_>>,
    sink: &mut EventSink,
) -> StepOutcome {
    let mut local_remaining = local_backtrack_budget;
    loop {
        if domains.all_singleton() {
            if constraints_accept(domains, constraints) {
                return StepOutcome::Solved;
            }
            // A global constraint rejected this complete assignment: exactly like a contradiction
            // needing a backtrack, reusing the same proven repair machinery.
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink) {
                RepairOutcome::Repaired => continue,
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => return StepOutcome::LocalLimitReached,
            }
        }
        let node = match heuristics::select_unresolved(config.heuristic, domains) {
            Some(n) => n,
            None => unreachable!("not all singleton but no unresolved candidate: domain invariant violated"),
        };

        if let Some(max_obs) = config.budget.max_observations {
            if metrics.observations >= max_obs {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(max_ms) = config.budget.max_millis {
            if start.elapsed().as_millis() as u64 >= max_ms {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(c) = cancel {
            if c.is_cancelled() {
                return StepOutcome::Cancelled;
            }
        }
        metrics.observations += 1;

        let contradiction = decide_and_propagate(model, topo, config, rng, domains, queue, trail, metrics, decision_counter, node, sink);
        if contradiction.is_some() {
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink) {
                RepairOutcome::Repaired => {}
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => return StepOutcome::LocalLimitReached,
            }
        }
    }
}

/// 🌳 Like [`drive`], but keeps searching for further solutions after each one is found (by
/// treating "solved" the same as a contradiction that must be repaired) until `limit` solutions
/// are collected or the tree is exhausted.
#[allow(clippy::too_many_arguments)]
fn drive_all<T: Topology>(
    model: &CompiledModel,
    topo: &T,
    config: &SearchConfig,
    rng: &mut Rng,
    domains: &mut DomainStore,
    queue: &mut PropQueue,
    trail: &mut Trail,
    metrics: &mut Metrics,
    decision_counter: &mut u32,
    start: std::time::Instant,
    solutions: &mut Vec<Vec<PatternId>>,
    limit: usize,
    constraints: Option<&ConstraintSet<'_>>,
    sink: &mut EventSink,
) -> StepOutcome {
    let mut local_remaining = None;
    loop {
        if domains.all_singleton() {
            if constraints_accept(domains, constraints) {
                solutions.push(domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect());
                if solutions.len() >= limit {
                    return StepOutcome::Solved;
                }
            }
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink) {
                RepairOutcome::Repaired => continue,
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => unreachable!("solve_all never sets a local backtrack budget"),
            }
        }
        let node = match heuristics::select_unresolved(config.heuristic, domains) {
            Some(n) => n,
            None => unreachable!("not all singleton but no unresolved candidate: domain invariant violated"),
        };
        if let Some(max_obs) = config.budget.max_observations {
            if metrics.observations >= max_obs {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        if let Some(max_ms) = config.budget.max_millis {
            if start.elapsed().as_millis() as u64 >= max_ms {
                sink.emit(Event::BudgetExceeded);
                return StepOutcome::BudgetExceeded;
            }
        }
        metrics.observations += 1;

        let contradiction = decide_and_propagate(model, topo, config, rng, domains, queue, trail, metrics, decision_counter, node, sink);
        if contradiction.is_some() {
            match backtrack_and_repair(model, topo, &config.budget, domains, queue, trail, metrics, &mut local_remaining, sink) {
                RepairOutcome::Repaired => {}
                RepairOutcome::Exhausted => return StepOutcome::Exhausted,
                RepairOutcome::BudgetExceeded => {
                    sink.emit(Event::BudgetExceeded);
                    return StepOutcome::BudgetExceeded;
                }
                RepairOutcome::LocalLimitReached => unreachable!("solve_all never sets a local backtrack budget"),
            }
        }
    }
}
// #endregion 🔖Drive

// #region 🔖Solve
struct InitResult {
    domains: DomainStore,
    trail: Trail,
    queue: PropQueue,
    metrics: Metrics,
    wipeout: Option<NodeId>,
}

fn initialize<T: Topology>(model: &CompiledModel, topo: &T, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], constraints: Option<&ConstraintSet<'_>>) -> InitResult {
    let node_count = topo.node_count();
    let mut domains = DomainStore::new_full(node_count, model.weights());
    let mut trail = Trail::new();
    let mut metrics = Metrics::default();
    let mut wipeout: Option<NodeId> = None;

    if let Some(overrides) = init_domains {
        for (i, allowed) in overrides.iter().enumerate() {
            let n = NodeId::from_index(i);
            let mut removed = PatternSet::new_empty(model.pattern_count());
            if let RestrictResult::Wipeout = domains.get_mut(n).restrict_collecting(allowed, model.weights(), &mut removed) {
                wipeout = Some(n);
            }
            trail.record_removed_set(n, &removed);
        }
    }
    for &(n, p) in fixed {
        let mut removed = PatternSet::new_empty(model.pattern_count());
        if let RestrictResult::Wipeout = domains.get_mut(n).assign_collecting(p, model.weights(), &mut removed) {
            wipeout = Some(n);
        }
        trail.record_removed_set(n, &removed);
    }
    if let Some(cs) = constraints {
        for c in cs.constraints {
            let Ok(restrictions) = c.initialize(&domains, model.weights(), cs.adjacency) else {
                continue; // a misconfigured constraint is a build-time concern, not a solve-time one
            };
            for (n, allowed) in restrictions {
                let mut removed = PatternSet::new_empty(model.pattern_count());
                if let RestrictResult::Wipeout = domains.get_mut(n).restrict_collecting(&allowed, model.weights(), &mut removed) {
                    wipeout = Some(n);
                }
                trail.record_removed_set(n, &removed);
            }
        }
    }

    let mut queue = PropQueue::new(node_count);
    queue.push_all(node_count);
    if wipeout.is_none() {
        wipeout = prop_ac3::run_to_fixed_point(model, topo, &mut domains, &mut queue, &mut trail, &mut metrics).err();
    }

    InitResult { domains, trail, queue, metrics, wipeout }
}

/// 🌳 Applies `init_domains` (or full domains) and `fixed` pins, runs initial propagation, then
/// drives search per `config` until solved, proven unsatisfiable, or a budget/restart limit stops
/// the attempt. `init_domains`, when present, must have one entry per node.
pub(crate) fn solve<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)]) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, None, None)
}

pub(crate) fn solve_cancellable<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], cancel: &CancelToken) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, Some(cancel), None)
}

/// 🌳 Like [`solve`], but also applies every constraint's initial restriction and rejects (via an
/// ordinary backtrack) any complete assignment a constraint does not accept.
#[allow(clippy::too_many_arguments)]
pub(crate) fn solve_with_constraints<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], cancel: Option<&CancelToken>, constraints: &ConstraintSet<'_>) -> SolveOutcome {
    solve_inner(model, topo, config, seed, init_domains, fixed, cancel, Some(constraints))
}

#[allow(clippy::too_many_arguments)]
fn solve_inner<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], cancel: Option<&CancelToken>, constraints: Option<&ConstraintSet<'_>>) -> SolveOutcome {
    let start = std::time::Instant::now();
    let mut rng = Rng::from_seed(seed);
    let mut restarts = 0u64;

    loop {
        let mut init = initialize(model, topo, init_domains, fixed, constraints);
        let mut sink = EventSink::new(config.diag_level);
        let mut decision_counter = 0u32;

        if let Some(wiped) = init.wipeout {
            sink.emit(Event::Contradiction { node: wiped });
            return conclude_failed_attempt(config, wiped, init.metrics, seed, &mut restarts, sink, model.fingerprint());
        }

        let local_budget = match config.mode {
            SearchMode::RestartOnly => config.restart_schedule.backtrack_budget(restarts),
            SearchMode::Backtrack | SearchMode::Backjump => None,
        };
        let step = drive(model, topo, config, &mut rng, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics, &mut decision_counter, start, cancel, local_budget, constraints, &mut sink);
        init.metrics.elapsed_millis = start.elapsed().as_millis() as u64;

        match step {
            StepOutcome::Solved => {
                sink.emit(Event::Solved);
                let assignment: Vec<PatternId> = init.domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guaranteed every domain is a singleton")).collect();
                return SolveOutcome::Solved(Solution { assignment, report: report(init.metrics, seed, model.fingerprint(), sink) });
            }
            StepOutcome::Exhausted => match config.mode {
                SearchMode::Backtrack | SearchMode::Backjump => {
                    return SolveOutcome::Unsatisfiable(UnsatReport { proven: true, report: report(init.metrics, seed, model.fingerprint(), sink) });
                }
                SearchMode::RestartOnly => {
                    if !restart_or_give_up(config, &mut restarts, &mut sink) {
                        init.metrics.restarts = restarts;
                        return SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: report(init.metrics, seed, model.fingerprint(), sink) });
                    }
                    continue;
                }
            },
            StepOutcome::LocalLimitReached => {
                debug_assert_eq!(config.mode, SearchMode::RestartOnly);
                if !restart_or_give_up(config, &mut restarts, &mut sink) {
                    init.metrics.restarts = restarts;
                    return SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: report(init.metrics, seed, model.fingerprint(), sink) });
                }
                continue;
            }
            StepOutcome::BudgetExceeded => {
                return SolveOutcome::BudgetExceeded { partial: partial_state(&init.domains), report: report(init.metrics, seed, model.fingerprint(), sink) };
            }
            StepOutcome::Cancelled => {
                return SolveOutcome::Cancelled { partial: partial_state(&init.domains), report: report(init.metrics, seed, model.fingerprint(), sink) };
            }
        }
    }
}

/// 🌳 Exhaustively enumerates up to `limit` solutions, proving `complete = true` iff the whole
/// tree was explored (never stopped early by `limit` or a budget).
pub(crate) fn solve_all<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], limit: usize) -> (Vec<Solution>, bool) {
    solve_all_inner(model, topo, config, seed, init_domains, fixed, limit, None)
}

/// 🌳 Like [`solve_all`], but also applies every constraint's initial restriction and excludes any
/// complete assignment a constraint does not accept.
#[allow(clippy::too_many_arguments)]
pub(crate) fn solve_all_with_constraints<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], limit: usize, constraints: &ConstraintSet<'_>) -> (Vec<Solution>, bool) {
    solve_all_inner(model, topo, config, seed, init_domains, fixed, limit, Some(constraints))
}

#[allow(clippy::too_many_arguments)]
fn solve_all_inner<T: Topology>(model: &CompiledModel, topo: &T, config: &SearchConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], limit: usize, constraints: Option<&ConstraintSet<'_>>) -> (Vec<Solution>, bool) {
    let start = std::time::Instant::now();
    let mut rng = Rng::from_seed(seed);
    let mut init = initialize(model, topo, init_domains, fixed, constraints);
    let mut decision_counter = 0u32;
    let mut raw_solutions = Vec::new();

    if init.wipeout.is_some() {
        return (Vec::new(), true);
    }

    let mut sink = EventSink::new(config.diag_level);
    let step = drive_all(model, topo, config, &mut rng, &mut init.domains, &mut init.queue, &mut init.trail, &mut init.metrics, &mut decision_counter, start, &mut raw_solutions, limit, constraints, &mut sink);
    init.metrics.elapsed_millis = start.elapsed().as_millis() as u64;

    let complete = matches!(step, StepOutcome::Exhausted);
    let fingerprint = model.fingerprint();
    // Every returned solution shares the same cumulative event trace from the whole exhaustive
    // search (not a solution-specific slice) — slicing per solution would need each `Solution` to
    // remember its own trail-position range, which isn't worth the bookkeeping until a caller
    // actually needs per-solution replay for `solve_all`.
    let events = sink.into_events();
    let solutions: Vec<Solution> = raw_solutions
        .into_iter()
        .map(|assignment| Solution { assignment, report: RunReport { metrics: init.metrics, model_fingerprint: fingerprint, seed, events: events.clone() } })
        .collect();
    (solutions, complete)
}

fn restart_or_give_up(config: &SearchConfig, restarts: &mut u64, sink: &mut EventSink) -> bool {
    sink.emit(Event::Restarted);
    *restarts += 1;
    !matches!(config.max_restarts, Some(max_r) if *restarts > max_r)
}

fn partial_state(domains: &DomainStore) -> PartialState {
    PartialState { domains: domains.iter().map(|(_, d)| d.bits().clone()).collect(), decided: domains.iter().map(|(_, d)| d.singleton()).collect() }
}

fn conclude_failed_attempt(config: &SearchConfig, wiped: NodeId, mut metrics: Metrics, seed: u64, restarts: &mut u64, sink: EventSink, fingerprint: u64) -> SolveOutcome {
    match config.mode {
        SearchMode::Backtrack | SearchMode::Backjump => {
            // A wipeout during the very first propagation (before any decision) with nothing on
            // the trail to undo means every branch is already excluded: unsatisfiable, proven.
            SolveOutcome::Unsatisfiable(UnsatReport { proven: true, report: report(metrics, seed, fingerprint, sink) })
        }
        SearchMode::RestartOnly => {
            *restarts += 1;
            metrics.restarts = *restarts;
            SolveOutcome::Contradiction(ContradictionReport { node: wiped, report: report(metrics, seed, fingerprint, sink) })
        }
    }
}

fn report(metrics: Metrics, seed: u64, model_fingerprint: u64, sink: EventSink) -> RunReport {
    RunReport { metrics, model_fingerprint, seed, events: sink.into_events() }
}
// #endregion 🔖Solve

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::oracle;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard_topology(n: usize) -> (CompiledModel, crate::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n.saturating_sub(1) {
            let a = NodeId::from_index(i);
            let c = NodeId::from_index(i + 1);
            tb.arc(a, c, adj);
            tb.arc(c, a, adj);
            arcs.push(oracle::ArcSpec { from: a, to: c, relation: adj });
            arcs.push(oracle::ArcSpec { from: c, to: a, relation: adj });
        }
        (model, tb.build().unwrap(), arcs)
    }

    fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::topology::GraphTopology, Vec<oracle::ArcSpec>) {
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
        let mut arcs = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = NodeId::from_index(i);
                let c = NodeId::from_index(j);
                tb.arc(a, c, ne);
                tb.arc(c, a, ne);
                arcs.push(oracle::ArcSpec { from: a, to: c, relation: ne });
                arcs.push(oracle::ArcSpec { from: c, to: a, relation: ne });
            }
        }
        (model, tb.build().unwrap(), arcs)
    }

    #[test]
    fn solves_a_satisfiable_path() {
        let (model, topo, arcs) = checkerboard_topology(6);
        let config = SearchConfig::default();
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Solved(sol) => {
                assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn proves_unsat_on_odd_cycle_with_backtrack_mode() {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(5);
        for i in 0..4 {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        tb.arc(NodeId(4), NodeId(0), adj);
        tb.arc(NodeId(0), NodeId(4), adj);
        let topo = tb.build().unwrap();

        let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    fn backtracking_solves_graph_coloring_needing_multiple_decisions() {
        let (model, topo, arcs) = k_graph(4, 4);
        for seed in 0..20 {
            let config = SearchConfig::default();
            let outcome = solve(&model, &topo, &config, seed, None, &[]);
            match outcome {
                SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
                other => panic!("seed {seed}: expected Solved, got {other:?}"),
            }
        }
    }

    #[test]
    fn unsatisfiable_k5_with_four_colors_proves_unsat() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 7, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }

    #[test]
    fn backjump_mode_matches_backtrack_completeness() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 7, None, &[]);
        match outcome {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }

        let (model2, topo2, arcs2) = k_graph(4, 4);
        let config2 = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
        let outcome2 = solve(&model2, &topo2, &config2, 3, None, &[]);
        match outcome2 {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model2, &sol.assignment, &arcs2).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn fixed_pins_are_respected() {
        let (model, topo, _arcs) = checkerboard_topology(3);
        let config = SearchConfig::default();
        let outcome = solve(&model, &topo, &config, 5, None, &[(NodeId(0), PatternId(1))]);
        match outcome {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn budget_exceeded_reports_partial_state() {
        // A checkerboard path fully solves after a single decision (propagation alone forces
        // every other node), so the budget must bite before any decision is even attempted.
        let (model, topo, _arcs) = checkerboard_topology(30);
        let config = SearchConfig { budget: Budget { max_observations: Some(0), ..Default::default() }, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        assert!(matches!(outcome, SolveOutcome::BudgetExceeded { .. }));
    }

    #[test]
    fn same_seed_is_fully_reproducible() {
        let (model, topo, _arcs) = checkerboard_topology(10);
        let config = SearchConfig::default();
        let o1 = solve(&model, &topo, &config, 123, None, &[]);
        let o2 = solve(&model, &topo, &config, 123, None, &[]);
        match (o1, o2) {
            (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => assert_eq!(s1.assignment, s2.assignment),
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    fn golden_replay_same_seed_reproduces_the_identical_decision_trace() {
        // Determinism at the level of the final assignment (`same_seed_is_fully_reproducible`)
        // is necessary but not sufficient — this checks the exact decision *sequence* two
        // `DiagLevel::Decisions` solves recorded is byte-identical via `TraceReplay`, catching a
        // divergence that happened to still land on the same final assignment by coincidence.
        use crate::diag::TraceReplay;
        let (model, topo, _arcs) = k_graph(4, 4);
        let config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };
        let o1 = solve(&model, &topo, &config, 77, None, &[]);
        let o2 = solve(&model, &topo, &config, 77, None, &[]);
        match (o1, o2) {
            (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => {
                let t1 = TraceReplay::from_report(&s1.report);
                let t2 = TraceReplay::from_report(&s2.report);
                assert!(!t1.decisions.is_empty(), "k_graph(4,4) needs at least one real decision");
                assert!(t1.matches(&t2));
            }
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    fn diag_off_records_no_decision_events_but_summary_and_above_do() {
        let (model, topo, _arcs) = checkerboard_topology(5);
        let off_config = SearchConfig { diag_level: DiagLevel::Off, ..Default::default() };
        let decisions_config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };

        let off_outcome = solve(&model, &topo, &off_config, 1, None, &[]);
        let decisions_outcome = solve(&model, &topo, &decisions_config, 1, None, &[]);
        match (off_outcome, decisions_outcome) {
            (SolveOutcome::Solved(off_sol), SolveOutcome::Solved(dec_sol)) => {
                assert!(off_sol.report.events.is_empty());
                assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Observed { .. })));
                assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Solved)));
            }
            _ => panic!("expected both solves to succeed"),
        }
    }

    #[test]
    fn cancellation_stops_search_and_reports_partial() {
        let (model, topo, _arcs) = k_graph(6, 4);
        let cancel = CancelToken::new();
        cancel.cancel();
        let config = SearchConfig::default();
        let outcome = solve_cancellable(&model, &topo, &config, 1, None, &[], &cancel);
        assert!(matches!(outcome, SolveOutcome::Cancelled { .. }));
    }

    #[test]
    fn cancel_token_reflects_state() {
        let cancel = CancelToken::new();
        assert!(!cancel.is_cancelled());
        cancel.cancel();
        assert!(cancel.is_cancelled());
    }

    #[test]
    fn restart_only_never_proves_unsat_on_unsatisfiable_instance() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(3), restart_schedule: RestartSchedule::Fixed(5), ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        assert!(matches!(outcome, SolveOutcome::Contradiction(_)));
    }

    #[test]
    fn restart_only_still_solves_satisfiable_instances() {
        let (model, topo, arcs) = k_graph(4, 4);
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(50), restart_schedule: RestartSchedule::Luby(4), ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        match outcome {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn luby_sequence_matches_known_values() {
        let expected = [1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8];
        for (i, &e) in expected.iter().enumerate() {
            assert_eq!(luby((i + 1) as u64), e, "luby({})", i + 1);
        }
    }

    #[test]
    fn solve_all_finds_every_solution_and_proves_complete() {
        let (model, topo, arcs) = k_graph(3, 3);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
        assert!(complete);
        assert_eq!(solutions.len(), 6); // 3! proper colorings of K3 with exactly 3 colors
        for sol in &solutions {
            assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
        }
        let mut assignments: Vec<_> = solutions.iter().map(|s| s.assignment.clone()).collect();
        assignments.sort();
        assignments.dedup();
        assert_eq!(assignments.len(), 6, "solve_all must not report the same solution twice");
    }

    #[test]
    fn solve_all_on_unsat_instance_returns_empty_and_complete() {
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
        assert!(complete);
        assert!(solutions.is_empty());
    }

    #[test]
    fn solve_all_respects_limit_and_reports_incomplete() {
        let (model, topo, _arcs) = k_graph(4, 4);
        let config = SearchConfig::default();
        let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 3);
        assert_eq!(solutions.len(), 3);
        assert!(!complete);
    }

    mod quick {
        use super::*;

        #[test]
        fn random_instances_solved_or_proven_unsat_match_oracle() {
            let mut rng = Rng::from_seed(777);
            for trial in 0..100 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 7) as usize;
                let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.5);
                let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();
                let init_domains = oracle::testgen::full_domains(&model, node_count);

                let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 1);
                let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
                let outcome = solve(&model, &topo, &config, trial as u64, None, &[]);

                match outcome {
                    SolveOutcome::Solved(sol) => {
                        assert!(!oracle_result.solutions.is_empty(), "trial {trial}: solver found a solution but oracle found none");
                        assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "trial {trial}: solver's solution violates an arc");
                    }
                    SolveOutcome::Unsatisfiable(rep) => {
                        assert!(rep.proven);
                        assert!(oracle_result.solutions.is_empty(), "trial {trial}: solver proved unsat but oracle found a solution");
                    }
                    other => panic!("trial {trial}: unexpected outcome {other:?}"),
                }
            }
        }

        #[test]
        fn solve_all_matches_oracle_solution_set_on_random_instances() {
            let mut rng = Rng::from_seed(2026);
            for trial in 0..40 {
                let pattern_count = 1 + rng.next_range(0, 4) as usize;
                let node_count = 1 + rng.next_range(0, 6) as usize;
                let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.6);
                let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
                let mut tb = GraphTopologyBuilder::new(node_count);
                for a in &arcs {
                    tb.arc(a.from, a.to, a.relation);
                }
                let topo = tb.build().unwrap();
                let init_domains = oracle::testgen::full_domains(&model, node_count);

                let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 10_000);
                let config = SearchConfig::default();
                let (solutions, complete) = solve_all(&model, &topo, &config, trial as u64, None, &[], 10_000);

                assert!(complete, "trial {trial}: solve_all did not report complete");
                assert_eq!(complete, oracle_result.complete, "trial {trial}: completeness disagreement");
                let mut got: Vec<Vec<PatternId>> = solutions.iter().map(|s| s.assignment.clone()).collect();
                got.sort();
                let mut want = oracle_result.solutions.clone();
                want.sort();
                assert_eq!(got, want, "trial {trial}: solution set mismatch");
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Search

// #region 🔖Outcome
pub mod outcome {
//! 🏁 What a solve attempt concludes with. `Contradiction`/`Unsatisfiable` are ordinary outcomes,
//! never [`crate::error`] variants — a search finding no solution is not a bug.

use crate::bitset::PatternSet;
use crate::diag::{Event, Metrics};
use crate::ids::{NodeId, PatternId};

// #region 🔖Report
/// 🏁 Bookkeeping attached to every [`SolveOutcome`] variant.
#[derive(Clone, Debug)]
pub struct RunReport {
    pub metrics: Metrics,
    pub model_fingerprint: u64,
    pub seed: u64,
    pub events: Vec<Event>,
}
// #endregion 🔖Report

// #region 🔖Solution
/// 🏁 A complete, hard-constraint-satisfying assignment (index = `NodeId`).
#[derive(Clone, Debug)]
pub struct Solution {
    pub assignment: Vec<PatternId>,
    pub report: RunReport,
}

/// 🏁 A proof of unsatisfiability. `proven = true` only when the entire search tree was exhausted
/// without finding a solution — restart-only search that simply gives up never sets this.
#[derive(Clone, Debug)]
pub struct UnsatReport {
    pub proven: bool,
    pub report: RunReport,
}

/// 🏁 A search branch that hit an empty domain and (for restart-only search) exhausted its
/// restart budget before finding a solution.
#[derive(Clone, Debug)]
pub struct ContradictionReport {
    pub node: NodeId,
    pub report: RunReport,
}

/// 🏁 Domains and decided assignments at the moment a solve stopped without concluding.
#[derive(Clone, Debug)]
pub struct PartialState {
    pub domains: Vec<PatternSet>,
    pub decided: Vec<Option<PatternId>>,
}
// #endregion 🔖Solution

// #region 🔖Outcome
/// 🏁 The five ways a solve attempt can end.
#[derive(Clone, Debug)]
pub enum SolveOutcome {
    Solved(Solution),
    Unsatisfiable(UnsatReport),
    Contradiction(ContradictionReport),
    BudgetExceeded { partial: PartialState, report: RunReport },
    /// 🏁 A caller-supplied [`crate::search::CancelToken`] was set before the attempt concluded.
    Cancelled { partial: PartialState, report: RunReport },
}
// #endregion 🔖Outcome
}
// #endregion 🔖Outcome

// #region 🔖Diag
pub mod diag {
//! 📊 Diagnostics: run metrics plus a level-gated event stream. `DiagLevel::Off` keeps the sink a
//! no-op (no allocation, no branching cost beyond one comparison) so instrumentation never taxes a
//! production solve that doesn't ask for it.

use crate::ids::{NodeId, PatternId};
use crate::outcome::RunReport;

// #region 🔖Level
/// 📊 How much event detail a solve records, from cheapest to most complete. Ordered
/// (`Off < Summary < Decisions < Full`) so call sites can gate emission with a single comparison
/// (`sink.level() >= DiagLevel::Decisions`) instead of matching every variant.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DiagLevel {
    #[default]
    Off,
    /// 📊 High-level outcomes only: `Solved`/`Contradiction`/`Restarted`/`BudgetExceeded`.
    Summary,
    /// 📊 Adds one `Observed`/`Backtracked` event per decision — enough to reconstruct the exact
    /// decision sequence via [`TraceReplay`] for a determinism/golden-replay check.
    Decisions,
    /// 📊 Reserved for finer-grained propagation-level tracing (e.g. per-arc domain reductions);
    /// today behaves identically to `Decisions` — no engine in this crate yet emits anything
    /// beyond decision-level events. Selecting it is forward-compatible: call sites already gate
    /// on `>= Decisions`, so a later phase can add `Full`-only events without revisiting them.
    Full,
}
// #endregion 🔖Level

// #region 🔖Event
/// 📊 One notable occurrence during a solve.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    Solved,
    Contradiction { node: NodeId },
    Restarted,
    BudgetExceeded,
    /// 📊 `DiagLevel::Decisions` and above: a decision was made — `node` was assigned `chosen`.
    Observed { node: NodeId, chosen: PatternId },
    /// 📊 `DiagLevel::Decisions` and above: `candidate` was ruled out at `node` and the decision
    /// undone (chronological backtrack or constraint-rejection repair).
    Backtracked { node: NodeId, candidate: PatternId },
}

/// 📊 Level-gated event buffer.
#[derive(Clone, Debug, Default)]
pub struct EventSink {
    level: DiagLevel,
    events: Vec<Event>,
}

impl EventSink {
    pub fn new(level: DiagLevel) -> Self {
        Self { level, events: Vec::new() }
    }

    #[inline]
    pub fn level(&self) -> DiagLevel {
        self.level
    }

    #[inline]
    pub fn emit(&mut self, event: Event) {
        if self.level != DiagLevel::Off {
            self.events.push(event);
        }
    }

    /// 📊 Only records `event` at `DiagLevel::Decisions` or above — for the fine-grained events a
    /// `Summary`-level caller doesn't want paying allocation cost for.
    #[inline]
    pub fn emit_detailed(&mut self, event: Event) {
        if self.level >= DiagLevel::Decisions {
            self.events.push(event);
        }
    }

    pub fn into_events(self) -> Vec<Event> {
        self.events
    }
}
// #endregion 🔖Event

// #region 🔖Replay
/// 📊 The ordered decision sequence recorded by a `DiagLevel::Decisions`-or-above solve, replayable
/// to verify determinism: the same model + same seed must always reach the same
/// (node, chosen-pattern) sequence in the same order. Extracted from a [`RunReport`]'s event
/// stream — empty if that solve ran below `Decisions` level (no `Observed` events to extract).
#[derive(Clone, PartialEq, Debug)]
pub struct TraceReplay {
    pub model_fingerprint: u64,
    pub seed: u64,
    pub decisions: Vec<(NodeId, PatternId)>,
}

impl TraceReplay {
    pub fn from_report(report: &RunReport) -> Self {
        let decisions = report
            .events
            .iter()
            .filter_map(|e| match e {
                Event::Observed { node, chosen } => Some((*node, *chosen)),
                _ => None,
            })
            .collect();
        Self { model_fingerprint: report.model_fingerprint, seed: report.seed, decisions }
    }

    /// 📊 The golden-replay determinism check: same model, same seed, same decisions in the same
    /// order.
    pub fn matches(&self, other: &TraceReplay) -> bool {
        self.model_fingerprint == other.model_fingerprint && self.seed == other.seed && self.decisions == other.decisions
    }
}
// #endregion 🔖Replay

// #region 🔖Metrics
/// 📊 Aggregate counters for one solve attempt (one restart's worth, unless a caller sums across
/// restarts itself).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Metrics {
    pub observations: u64,
    pub propagations: u64,
    pub removals: u64,
    pub backtracks: u64,
    pub restarts: u64,
    pub elapsed_millis: u64,
}
// #endregion 🔖Metrics

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_sink_records_nothing() {
        let mut sink = EventSink::new(DiagLevel::Off);
        sink.emit(Event::Solved);
        assert!(sink.into_events().is_empty());
    }

    #[test]
    fn summary_sink_records_events() {
        let mut sink = EventSink::new(DiagLevel::Summary);
        sink.emit(Event::Solved);
        sink.emit(Event::Contradiction { node: NodeId(3) });
        let events = sink.into_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], Event::Solved);
    }

    #[test]
    fn metrics_default_to_zero() {
        let m = Metrics::default();
        assert_eq!(m.observations, 0);
        assert_eq!(m.backtracks, 0);
    }

    #[test]
    fn diag_levels_are_ordered() {
        assert!(DiagLevel::Off < DiagLevel::Summary);
        assert!(DiagLevel::Summary < DiagLevel::Decisions);
        assert!(DiagLevel::Decisions < DiagLevel::Full);
    }

    #[test]
    fn emit_detailed_is_suppressed_below_decisions_level() {
        let mut summary_sink = EventSink::new(DiagLevel::Summary);
        summary_sink.emit_detailed(Event::Observed { node: NodeId(0), chosen: PatternId(0) });
        assert!(summary_sink.into_events().is_empty());

        let mut decisions_sink = EventSink::new(DiagLevel::Decisions);
        decisions_sink.emit_detailed(Event::Observed { node: NodeId(0), chosen: PatternId(0) });
        assert_eq!(decisions_sink.into_events().len(), 1);
    }

    #[test]
    fn trace_replay_extracts_only_observed_events_in_order() {
        let report = RunReport {
            metrics: Metrics::default(),
            model_fingerprint: 42,
            seed: 7,
            events: vec![
                Event::Observed { node: NodeId(0), chosen: PatternId(1) },
                Event::Backtracked { node: NodeId(0), candidate: PatternId(1) },
                Event::Observed { node: NodeId(0), chosen: PatternId(2) },
                Event::Solved,
            ],
        };
        let trace = TraceReplay::from_report(&report);
        assert_eq!(trace.model_fingerprint, 42);
        assert_eq!(trace.seed, 7);
        assert_eq!(trace.decisions, vec![(NodeId(0), PatternId(1)), (NodeId(0), PatternId(2))]);
    }

    #[test]
    fn trace_replay_matches_identical_sequences_and_rejects_divergent_ones() {
        let a = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(0))] };
        let b = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(0))] };
        assert!(a.matches(&b));

        let diverged = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(1))] };
        assert!(!a.matches(&diverged));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Diag

// #region 🔖Solvergraph
pub mod solver_graph {
//! 🕸️ `GraphSolver`: the semantic reference solver over an arbitrary [`GraphTopology`]. Thin
//! wiring — every real behavior lives in [`crate::search`], [`crate::prop_ac3`], and friends;
//! this module only validates a model/topology pairing and forwards to the generic kernel.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, CancelToken, SearchConfig};
use crate::topology::GraphTopology;
use crate::trail::Checkpoint;

// #region 🔖Builder
/// 🏗️ Builds a [`GraphSolver`] from a compiled model and a fixed graph topology.
pub struct GraphSolverBuilder {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
}

impl GraphSolverBuilder {
    pub fn new(model: CompiledModel, topology: GraphTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    /// 🏗️ Restricts `n`'s initial domain (heterogeneous per-node domains). Nodes never touched
    /// keep the full pattern universe.
    pub fn domain(mut self, n: NodeId, allowed: PatternSet) -> Self {
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        self
    }

    pub fn fix(mut self, n: NodeId, p: PatternId) -> Self {
        self.fixed.push((n, p));
        self
    }

    pub fn config(mut self, cfg: SearchConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 🏗️ Adds a global constraint. See [`crate::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<GraphSolver, SolveError> {
        for &(n, _) in &self.fixed {
            if n.index() >= self.topology.node_count() {
                return Err(SolveError::UnknownNode(n));
            }
        }
        let adjacency = build_adjacency_view(&self.topology);
        Ok(GraphSolver { model: self.model, topology: self.topology, init_domains: self.init_domains, fixed: self.fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖Builder

// #region 🔖Solver
/// 🕸️ The reference WFC solver over an arbitrary fixed directed graph.
pub struct GraphSolver {
    model: CompiledModel,
    topology: GraphTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl GraphSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() { None } else { Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency }) }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, cancel),
        }
    }

    /// 🕸️ Exhaustively enumerates up to `limit` solutions; the returned `bool` is `true` iff the
    /// whole search tree was explored (a `false` means `limit` or a budget cut it short).
    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, self.init_domains.as_deref(), &self.fixed, limit),
        }
    }

    /// 🕸️ Resumes from a [`Checkpoint`] taken from this same model (fingerprint-checked). See
    /// [`Checkpoint`]'s docs for the resumability fidelity this provides.
    pub fn resume(&mut self, checkpoint: &Checkpoint) -> Result<SolveOutcome, SolveError> {
        if checkpoint.model_fingerprint != self.model.fingerprint() {
            return Err(SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
        }
        if checkpoint.domains.len() != self.topology.node_count() {
            return Err(SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
        }
        Ok(search::solve(&self.model, &self.topology, &self.config, checkpoint.seed, Some(&checkpoint.domains), &[]))
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &GraphTopology {
        &self.topology
    }
}
// #endregion 🔖Solver

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::outcome::SolveOutcome;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard(n: usize) -> (CompiledModel, GraphTopology) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        (model, tb.build().unwrap())
    }

    #[test]
    fn builds_and_solves() {
        let (model, topo) = checkerboard(5);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn fix_pins_a_node() {
        let (model, topo) = checkerboard(4);
        let mut solver = GraphSolverBuilder::new(model, topo).fix(NodeId(0), PatternId(0)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn fix_on_unknown_node_is_rejected() {
        let (model, topo) = checkerboard(2);
        let result = GraphSolverBuilder::new(model, topo).fix(NodeId(99), PatternId(0)).build();
        assert!(result.is_err());
    }

    #[test]
    fn domain_override_restricts_a_node() {
        let (model, topo) = checkerboard(3);
        let mut allowed = PatternSet::new_empty(2);
        allowed.set(PatternId(1), true);
        let mut solver = GraphSolverBuilder::new(model, topo).domain(NodeId(0), allowed).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn solve_all_finds_both_checkerboard_colorings() {
        let (model, topo) = checkerboard(4);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let (solutions, complete) = solver.solve_all(1, 100);
        assert!(complete);
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn solve_cancellable_reports_cancelled_when_pre_cancelled() {
        let (model, topo) = checkerboard(5);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let cancel = CancelToken::new();
        cancel.cancel();
        let outcome = solver.solve_cancellable(1, &cancel);
        assert!(matches!(outcome, SolveOutcome::Cancelled { .. }));
    }

    #[test]
    fn resume_from_checkpoint_completes_the_solve() {
        let (model, topo) = checkerboard(5);
        let fingerprint = model.fingerprint();
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();

        let mut domains = vec![solver.model().full_domain(); solver.topology().node_count()];
        let mut pinned = PatternSet::new_empty(2);
        pinned.set(PatternId(0), true);
        domains[0] = pinned;
        let checkpoint = Checkpoint::new(domains, fingerprint, 9);

        match solver.resume(&checkpoint).unwrap() {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn resume_rejects_mismatched_fingerprint() {
        let (model, topo) = checkerboard(3);
        let mut solver = GraphSolverBuilder::new(model, topo).build().unwrap();
        let domains = vec![solver.model().full_domain(); solver.topology().node_count()];
        let checkpoint = Checkpoint::new(domains, 0xDEAD_BEEF, 1);
        assert!(solver.resume(&checkpoint).is_err());
    }

    // End-to-end constraint wiring: unlike `crate::constraints_card`'s own tests (which exercise
    // `Constraint::initialize`/`validate_complete` directly), these drive a real
    // `GraphSolverBuilder::constraint(...)` through `solve()`, proving the `search::solve_with_constraints`
    // path — initial restriction, per-complete-assignment rejection, and backtrack-and-retry on
    // rejection — actually wires together end to end.
    #[test]
    fn cardinality_constraint_forces_the_unique_matching_checkerboard_coloring() {
        use crate::constraint::PatternSelector;
        use crate::constraints_card::{CardinalityConstraint, Scope};

        // checkerboard(5) is a 5-node path with exactly two valid 2-colorings: [B,W,B,W,B] (3
        // black) and [W,B,W,B,W] (2 black). Neither the constraint's `initialize` (domains start
        // full, so its possible/required bounds can't detect infeasibility up front) nor a trivial
        // propagation pass rules either coloring out — only `validate_complete`, invoked per
        // candidate via `backtrack_and_repair`, can. Requiring exactly 3 black therefore forces the
        // first coloring and proves the reject-and-backtrack path actually runs.
        let (model, topo) = checkerboard(5);
        let black = PatternId(0);
        let constraint = CardinalityConstraint::new(model.clone(), PatternSelector::Pattern(black), Scope::All, 3, 3).unwrap();
        let mut solver = GraphSolverBuilder::new(model, topo).constraint(Box::new(constraint)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                assert_eq!(sol.assignment, vec![PatternId(0), PatternId(1), PatternId(0), PatternId(1), PatternId(0)]);
                assert_eq!(sol.assignment.iter().filter(|&&p| p == black).count(), 3);
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn cardinality_constraint_beyond_both_colorings_is_unsatisfiable() {
        use crate::constraint::PatternSelector;
        use crate::constraints_card::{CardinalityConstraint, Scope};

        // Neither valid coloring of checkerboard(5) has 4+ black nodes (max achievable is 3), so
        // this must exhaust the full (small) search tree via repeated constraint rejection and
        // report a proven-unsatisfiable outcome, not merely an initial-domain wipeout.
        let (model, topo) = checkerboard(5);
        let black = PatternId(0);
        let constraint = CardinalityConstraint::new(model.clone(), PatternSelector::Pattern(black), Scope::All, 4, 5).unwrap();
        let mut solver = GraphSolverBuilder::new(model, topo).constraint(Box::new(constraint)).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
            other => panic!("expected Unsatisfiable, got {other:?}"),
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Solvergraph

// #region 🔖Grid2D
pub mod grid2d {
//! 🗺️ Dense 2D grid topology: arithmetic neighbor lookup (zero adjacency storage — a `for_each_out_arc`
//! call is a handful of integer additions and a boundary-wrap branch, not a CSR slice walk).
//! Relations are supplied by the caller (via [`declare_stencil_relations`]) rather than assumed
//! from stencil-offset order, so a grid model can freely mix stencil relations with others.

use crate::error::{ModelError, TopologyError};
use crate::ids::{NodeId, PatternId, RegionId, RelationId};
use crate::model::ModelBuilder;
use crate::tiled::TiledModelBuilder;
use crate::topology::Topology;

// #region 🔖Stencil
/// 🗺️ Which offsets count as "neighbors" of a 2D cell. Every built-in stencil is symmetric (each
/// offset's negation is also present) so a single relation-per-offset naturally gets a matching
/// inverse; [`Stencil2d::Custom`] must uphold that itself or [`declare_stencil_relations`] rejects it.
#[derive(Clone, PartialEq, Debug)]
pub enum Stencil2d {
    /// 🗺️ 4-neighbor: N, S, E, W.
    VonNeumann,
    /// 🗺️ 8-neighbor: von Neumann plus the four diagonals.
    Moore,
    /// 🗺️ 6-neighbor axial hex grid (cells addressed by `(x, y)` axial coordinates directly).
    Hex,
    /// 🗺️ An arbitrary offset list, each entry's negation required to also be present.
    Custom(Vec<(i32, i32)>),
}

impl Stencil2d {
    pub fn offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Stencil2d::VonNeumann => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Stencil2d::Moore => vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)],
            Stencil2d::Hex => vec![(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)],
            Stencil2d::Custom(v) => v.clone(),
        }
    }

    fn validate(&self) -> Result<(), TopologyError> {
        let offsets = self.offsets();
        if offsets.is_empty() {
            return Err(TopologyError::InvalidStencil { reason: "stencil has zero offsets" });
        }
        for (i, &a) in offsets.iter().enumerate() {
            if a == (0, 0) {
                return Err(TopologyError::InvalidStencil { reason: "self-offset (0,0) is not supported" });
            }
            for &b in &offsets[i + 1..] {
                if a == b {
                    return Err(TopologyError::InvalidStencil { reason: "duplicate offset" });
                }
            }
            if !offsets.contains(&(-a.0, -a.1)) {
                return Err(TopologyError::InvalidStencil { reason: "offset's negation is not present in the stencil" });
            }
        }
        Ok(())
    }
}

/// 🗺️ Registers one directed relation per stencil offset (paired with its negation as inverse)
/// and returns them in `stencil.offsets()` order, ready to pass to [`Grid2dTopology::new`].
pub fn declare_stencil_relations(builder: &mut ModelBuilder, stencil: &Stencil2d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy) in &offsets {
        relations.push(builder.add_relation(&format!("offset({dx},{dy})")));
    }
    for (i, &(dx, dy)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}

/// 🗺️ [`declare_stencil_relations`] for a [`TiledModelBuilder`].
pub fn declare_stencil_relations_tiled(builder: &mut TiledModelBuilder, stencil: &Stencil2d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_tiled" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy) in &offsets {
        relations.push(builder.relation(&format!("offset({dx},{dy})")));
    }
    for (i, &(dx, dy)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}
// #endregion 🔖Stencil

// #region 🔖Boundary
/// 🗺️ Per-axis behavior when a stencil offset points outside `0..size`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Boundary {
    /// 🗺️ No arc is drawn — the edge cell simply has fewer neighbors.
    Open,
    /// 🗺️ No arc is drawn, but the edge cell's initial domain is restricted as if a permanently-
    /// resolved neighbor of the given pattern were there (an init-time unary restriction, not a
    /// propagation-participating virtual node).
    FixedOutside(PatternId),
    /// 🗺️ The axis is periodic: `size - 1` wraps to `0` and vice versa.
    Wrap,
    /// 🗺️ Out-of-range coordinates reflect back into range (`-1` mirrors to `1`, matching the
    /// common "symmetric" image-padding convention). `for_each_in_arc`'s per-offset reverse lookup
    /// can under-count sources at a mirrored boundary on very small grids (it recovers the
    /// *nearest* predecessor per offset, not every predecessor a mirror fold may have created) —
    /// the AC-4 engine's support counts can therefore under-count support (sound but incomplete
    /// pruning) on `Mirror` boundaries; `Open`/`Wrap`/`FixedOutside` are unaffected. Avoid `Mirror`
    /// with AC-4 on grids smaller than roughly `2 * max stencil radius` until this is revisited.
    Mirror,
}

pub(crate) fn resolve_coord(coord: i32, size: usize, boundary: Boundary) -> Option<usize> {
    if coord >= 0 && (coord as usize) < size {
        return Some(coord as usize);
    }
    match boundary {
        Boundary::Open | Boundary::FixedOutside(_) => None,
        Boundary::Wrap => {
            let n = size as i32;
            Some((((coord % n) + n) % n) as usize)
        }
        Boundary::Mirror => {
            if size <= 1 {
                return Some(0);
            }
            let period = 2 * (size as i32 - 1);
            let mut m = coord % period;
            if m < 0 {
                m += period;
            }
            if m >= size as i32 {
                m = period - m;
            }
            Some(m as usize)
        }
    }
}
// #endregion 🔖Boundary

// #region 🔖Topology
/// 🗺️ A dense, row-major 2D grid topology. `NodeId(y * width + x)`.
#[derive(Clone, Debug)]
pub struct Grid2dTopology {
    width: usize,
    height: usize,
    offsets: Vec<(i32, i32)>,
    relations: Vec<RelationId>,
    boundary_x: Boundary,
    boundary_y: Boundary,
    mask: Option<Vec<bool>>,
}

impl Grid2dTopology {
    #[allow(clippy::too_many_arguments)]
    pub fn new(width: usize, height: usize, stencil: &Stencil2d, relations: Vec<RelationId>, boundary_x: Boundary, boundary_y: Boundary, mask: Option<Vec<bool>>) -> Result<Self, TopologyError> {
        if width == 0 {
            return Err(TopologyError::ZeroDimension { axis: "width" });
        }
        if height == 0 {
            return Err(TopologyError::ZeroDimension { axis: "height" });
        }
        width.checked_mul(height).ok_or(TopologyError::SizeOverflow)?;
        stencil.validate()?;
        let offsets = stencil.offsets();
        if offsets.len() != relations.len() {
            return Err(TopologyError::InvalidStencil { reason: "relations length does not match stencil offset count" });
        }
        if let Some(m) = &mask {
            if m.len() != width * height {
                return Err(TopologyError::MaskShapeMismatch { expected: width * height, actual: m.len() });
            }
        }
        Ok(Self { width, height, offsets, relations, boundary_x, boundary_y, mask })
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn node_at(&self, x: usize, y: usize) -> Option<NodeId> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(NodeId::from_index(y * self.width + x))
    }

    #[inline]
    pub fn coords(&self, n: NodeId) -> (usize, usize) {
        let idx = n.index();
        (idx % self.width, idx / self.width)
    }

    #[inline]
    pub fn is_active(&self, x: usize, y: usize) -> bool {
        self.mask.as_ref().is_none_or(|m| m[y * self.width + x])
    }

    /// 🗺️ Cells masked out (inactive) — these must be pinned to a placeholder pattern by the
    /// solver builder so they never participate in the search (see [`crate::solver_grid2d`]).
    pub fn inactive_cells(&self) -> Vec<NodeId> {
        let Some(mask) = &self.mask else { return Vec::new() };
        (0..mask.len()).filter(|&i| !mask[i]).map(NodeId::from_index).collect()
    }

    /// 🗺️ Every `(node, relation, outside_pattern)` an edge cell must be restricted by at init
    /// time, derived from [`Boundary::FixedOutside`] axes.
    pub fn fixed_outside_restrictions(&self) -> Vec<(NodeId, RelationId, PatternId)> {
        let mut out = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_active(x, y) {
                    continue;
                }
                for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
                    let tx = x as i32 + dx;
                    let ty = y as i32 + dy;
                    let x_out = tx < 0 || tx as usize >= self.width;
                    let y_out = ty < 0 || ty as usize >= self.height;
                    if x_out {
                        if let Boundary::FixedOutside(p) = self.boundary_x {
                            out.push((NodeId::from_index(y * self.width + x), self.relations[i], p));
                            continue;
                        }
                    }
                    if y_out {
                        if let Boundary::FixedOutside(p) = self.boundary_y {
                            out.push((NodeId::from_index(y * self.width + x), self.relations[i], p));
                        }
                    }
                }
            }
        }
        out
    }
}

impl Topology for Grid2dTopology {
    #[inline]
    fn node_count(&self) -> usize {
        self.width * self.height
    }

    fn arc_count(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_active(x, y) {
                    continue;
                }
                for &(dx, dy) in &self.offsets {
                    let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
                    let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
                    if let (Some(nx), Some(ny)) = (rx, ry) {
                        if self.is_active(nx, ny) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    #[inline]
    fn region_of(&self, _n: NodeId) -> RegionId {
        RegionId(0)
    }

    fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let (x, y) = self.coords(n);
        if !self.is_active(x, y) {
            return;
        }
        for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
            if let (Some(nx), Some(ny)) = (rx, ry) {
                if self.is_active(nx, ny) {
                    f(NodeId::from_index(ny * self.width + nx), self.relations[i]);
                }
            }
        }
    }

    fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let (x, y) = self.coords(n);
        if !self.is_active(x, y) {
            return;
        }
        for (i, &(dx, dy)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 - dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 - dy, self.height, self.boundary_y);
            if let (Some(sx), Some(sy)) = (rx, ry) {
                if self.is_active(sx, sy) {
                    // Slot = target's node index * offset count + offset index; a fixed dense id
                    // per (target, offset) pair regardless of which candidate source resolved it.
                    f(NodeId::from_index(sy * self.width + sx), self.relations[i], n.index() * self.offsets.len() + i);
                }
            }
        }
    }

    fn max_in_degree(&self) -> usize {
        self.offsets.len()
    }
}
// #endregion 🔖Topology

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn von_neumann_offsets_are_symmetric() {
        Stencil2d::VonNeumann.validate().unwrap();
        Stencil2d::Moore.validate().unwrap();
        Stencil2d::Hex.validate().unwrap();
    }

    #[test]
    fn custom_stencil_rejects_unpaired_offset() {
        let s = Stencil2d::Custom(vec![(1, 0)]);
        assert!(s.validate().is_err());
    }

    #[test]
    fn custom_stencil_rejects_duplicate_and_self_offset() {
        assert!(Stencil2d::Custom(vec![(1, 0), (1, 0), (-1, 0)]).validate().is_err());
        assert!(Stencil2d::Custom(vec![(0, 0)]).validate().is_err());
    }

    #[test]
    fn node_at_and_coords_roundtrip() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(4, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
        let n = topo.node_at(2, 1).unwrap();
        assert_eq!(topo.coords(n), (2, 1));
        assert_eq!(topo.node_at(4, 0), None);
    }

    #[test]
    fn open_boundary_drops_out_of_range_arcs() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 2); // only east and south exist from the top-left corner
    }

    #[test]
    fn wrap_boundary_connects_opposite_edges() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 4); // wraps to all four neighbors
        assert!(out.contains(&topo.node_at(2, 0).unwrap())); // west wraps to x=2
        assert!(out.contains(&topo.node_at(0, 2).unwrap())); // north wraps to y=2
    }

    #[test]
    fn size_one_axis_wrap_self_loops() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(1, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Open, None).unwrap();
        let n = topo.node_at(0, 1).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(n, |m, _| out.push(m));
        // east/west both wrap to the same single column -> self-loop arcs, plus north/south.
        assert_eq!(out.len(), 4);
    }

    #[test]
    fn mirror_boundary_reflects_at_edges() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(5, 5, &Stencil2d::VonNeumann, rels, Boundary::Mirror, Boundary::Mirror, None).unwrap();
        let corner = topo.node_at(0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 4);
        assert!(out.contains(&topo.node_at(1, 0).unwrap())); // west mirrors back to x=1
        assert!(out.contains(&topo.node_at(0, 1).unwrap())); // north mirrors back to y=1
    }

    #[test]
    fn mask_excludes_inactive_cells_from_arcs() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let mut mask = vec![true; 9];
        mask[4] = false; // center of 3x3 inactive
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let north_of_center = topo.node_at(1, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(north_of_center, |m, _| out.push(m));
        assert!(!out.contains(&topo.node_at(1, 1).unwrap())); // no arc into the masked-out center
        assert_eq!(topo.inactive_cells(), vec![NodeId(4)]);
    }

    #[test]
    fn fixed_outside_restrictions_only_on_boundary_facing_axis() {
        let mut b = ModelBuilder::new();
        let solid = b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
        let topo = Grid2dTopology::new(2, 2, &Stencil2d::VonNeumann, rels, Boundary::FixedOutside(solid), Boundary::Open, None).unwrap();
        let restrictions = topo.fixed_outside_restrictions();
        // Only x-axis boundary is FixedOutside; every cell touches an x-edge in a 2x2 grid.
        assert_eq!(restrictions.len(), 4);
        assert!(restrictions.iter().all(|&(_, _, p)| p == solid));
    }

    #[test]
    fn in_arc_matches_out_arc_on_open_boundary() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations(&mut b, &Stencil2d::Moore).unwrap();
        let topo = Grid2dTopology::new(4, 4, &Stencil2d::Moore, rels, Boundary::Open, Boundary::Open, None).unwrap();
        for y in 0..4 {
            for x in 0..4 {
                let n = topo.node_at(x, y).unwrap();
                let mut outgoing = Vec::new();
                topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
                let mut incoming_as_source: Vec<(NodeId, RelationId)> = Vec::new();
                for oy in 0..4 {
                    for ox in 0..4 {
                        let other = topo.node_at(ox, oy).unwrap();
                        let mut theirs = Vec::new();
                        topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                        if theirs.contains(&(n, RelationId(0))) || theirs.iter().any(|&(s, _)| s == n) {
                            for &(s, r) in &theirs {
                                if s == n {
                                    incoming_as_source.push((other, r));
                                }
                            }
                        }
                    }
                }
                let mut a = outgoing;
                let mut b2 = incoming_as_source;
                a.sort_by_key(|&(m, r)| (m.get(), r.get()));
                b2.sort_by_key(|&(m, r)| (m.get(), r.get()));
                assert_eq!(a, b2, "node ({x},{y}) out-arcs must match in-arc reconstruction on an open boundary");
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Grid2D

// #region 🔖Solvergrid2D
pub mod solver_grid2d {
//! 🧱 `Grid2dSolver`: dense 2D grid solving on top of [`crate::grid2d`] and the shared kernel.
//! Masked-out cells and [`Boundary::FixedOutside`] edges are folded into ordinary domain overrides
//! and fixed pins before delegating to the same generic [`crate::search::solve`] every solver uses.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::grid2d::Grid2dTopology;
use crate::ids::PatternId;
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, CancelToken, SearchConfig};
use crate::topology::Topology;

// #region 🔖Builder
/// 🏗️ Builds a [`Grid2dSolver`] over a dense `width × height` grid.
pub struct Grid2dSolverBuilder {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
}

impl Grid2dSolverBuilder {
    pub fn new(model: CompiledModel, topology: Grid2dTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    pub fn fix(mut self, x: usize, y: usize, p: PatternId) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "fix() coordinate out of range" })?;
        self.fixed.push((n, p));
        Ok(self)
    }

    pub fn domain(mut self, x: usize, y: usize, allowed: PatternSet) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "domain() coordinate out of range" })?;
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        Ok(self)
    }

    pub fn config(mut self, cfg: SearchConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 🏗️ Adds a global constraint. See [`crate::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<Grid2dSolver, SolveError> {
        let node_count = self.topology.node_count();
        let mut init_domains = self.init_domains.unwrap_or_else(|| vec![self.model.full_domain(); node_count]);
        let mut fixed = self.fixed;

        for (n, rel, outside_pattern) in self.topology.fixed_outside_restrictions() {
            init_domains[n.index()].and_with(self.model.allowed(rel, outside_pattern));
        }
        let placeholder = PatternId(0);
        for n in self.topology.inactive_cells() {
            fixed.push((n, placeholder));
        }

        let adjacency = build_adjacency_view(&self.topology);
        Ok(Grid2dSolver { model: self.model, topology: self.topology, init_domains, fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖Builder

// #region 🔖Solver
/// 🧱 A WFC solver over a dense 2D grid.
pub struct Grid2dSolver {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Vec<PatternSet>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl Grid2dSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() { None } else { Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency }) }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, cancel),
        }
    }

    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit),
        }
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &Grid2dTopology {
        &self.topology
    }

    /// 🧱 The pattern assigned at `(x, y)` in `solution`.
    pub fn get(&self, solution: &Solution, x: usize, y: usize) -> Option<PatternId> {
        let n = self.topology.node_at(x, y)?;
        solution.assignment.get(n.index()).copied()
    }

    /// 🧱 Row-major `width * height` tile decode via each pattern's authored tile provenance.
    /// Patterns with no tile provenance (e.g. built directly via [`crate::model::ModelBuilder`])
    /// decode to `None` at that cell.
    pub fn decode_tiles(&self, solution: &Solution) -> Vec<Option<crate::ids::TileId>> {
        solution.assignment.iter().map(|&p| self.model.pattern_info(p).tile).collect()
    }
}
// #endregion 🔖Solver

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid2d::{Boundary, Stencil2d, declare_stencil_relations_tiled};
    use crate::tiled::TiledModelBuilder;

    fn checkerboard(width: usize, height: usize, boundary: Boundary) -> (CompiledModel, Grid2dTopology) {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, boundary, boundary, None).unwrap();
        (model, topo)
    }

    #[test]
    fn solves_a_checkerboard_grid() {
        let (model, topo) = checkerboard(5, 5, Boundary::Open);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn fix_pins_a_cell_and_propagates() {
        let (model, topo) = checkerboard(4, 4, Boundary::Open);
        let black = PatternId(0);
        let white = PatternId(1);
        let mut solver = Grid2dSolverBuilder::new(model, topo).fix(0, 0, black).unwrap().build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                assert_eq!(solver.get(&sol, 0, 0), Some(black));
                assert_eq!(solver.get(&sol, 1, 0), Some(white));
                assert_eq!(solver.get(&sol, 0, 1), Some(white));
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn masked_cells_are_excluded_and_solve_completes() {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let mut mask = vec![true; 9];
        mask[4] = false;
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn wrap_boundary_solves_consistently() {
        let (model, topo) = checkerboard(4, 4, Boundary::Wrap);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn odd_size_wrap_is_unsatisfiable_for_two_color_checkerboard() {
        // A 3x3 wrapped grid forces an odd cycle along each axis; two colors can't 2-color it.
        let (model, topo) = checkerboard(3, 3, Boundary::Wrap);
        let mut solver = Grid2dSolverBuilder::new(model, topo).config(SearchConfig { mode: search::SearchMode::Backtrack, ..Default::default() }).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Unsatisfiable(_)));
    }

    #[test]
    fn graph_vs_grid2d_strict_equivalence_von_neumann_open() {
        // Independently hand-enumerated arcs for a 3x4 VonNeumann/Open grid (not derived from
        // grid2d.rs's own resolve_coord logic) fed into a GraphTopology, compared against the
        // same model solved through Grid2dTopology: both must produce byte-identical assignments
        // and identical observation counts under the same seed/config.
        let width = 3usize;
        let height = 4usize;
        let mut b = TiledModelBuilder::new();
        let tiles: Vec<_> = (0..3).map(|i| b.tile(1.0 + i as f64)).collect();
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            for &a in &tiles {
                for &c in &tiles {
                    if a != c {
                        b.allow(r, a, c);
                    }
                }
            }
        }
        let model = b.compile().unwrap();

        let mut hand_arcs = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let idx = |x: usize, y: usize| crate::ids::NodeId::from_index(y * width + x);
                if x + 1 < width {
                    hand_arcs.push((idx(x, y), idx(x + 1, y), rels[0])); // east: offset (1,0)
                    hand_arcs.push((idx(x + 1, y), idx(x, y), rels[1])); // west: offset (-1,0)
                }
                if y + 1 < height {
                    hand_arcs.push((idx(x, y), idx(x, y + 1), rels[2])); // south: offset (0,1)
                    hand_arcs.push((idx(x, y + 1), idx(x, y), rels[3])); // north: offset (0,-1)
                }
            }
        }
        let mut gb = crate::topology::GraphTopologyBuilder::new(width * height);
        for (from, to, r) in hand_arcs {
            gb.arc(from, to, r);
        }
        let graph_topo = gb.build().unwrap();

        let grid_topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();

        let config = SearchConfig::default();
        for seed in 0..10u64 {
            let mut graph_solver = crate::solver_graph::GraphSolverBuilder::new(model.clone(), graph_topo.clone()).config(config).build().unwrap();
            let mut grid_solver = Grid2dSolverBuilder::new(model.clone(), grid_topo.clone()).config(config).build().unwrap();
            let graph_outcome = graph_solver.solve(seed);
            let grid_outcome = grid_solver.solve(seed);
            match (graph_outcome, grid_outcome) {
                (SolveOutcome::Solved(g), SolveOutcome::Solved(r)) => {
                    assert_eq!(g.assignment, r.assignment, "seed {seed}: graph and grid2d solutions diverged");
                    assert_eq!(g.report.metrics.observations, r.report.metrics.observations, "seed {seed}: observation counts diverged");
                }
                (a, b) => panic!("seed {seed}: outcome mismatch, graph={a:?} grid={b:?}"),
            }
        }
    }

    #[test]
    fn decode_tiles_round_trips_tile_provenance() {
        let (model, topo) = checkerboard(2, 2, Boundary::Open);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                let tiles = solver.decode_tiles(&sol);
                assert_eq!(tiles.len(), 4);
                assert!(tiles.iter().all(|t| t.is_some()));
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Solvergrid2D

// #region 🔖Symmetry
pub mod symmetry {
//! 🔄 2D symmetry: the dihedral group D4 (identity, 3 rotations, 4 reflections) and its action on
//! stencil offsets and on rectangular tile windows. The single source of truth is each element's
//! 2×2 integer matrix — offset transform and window transform both derive from it, which is what
//! guarantees a pattern's rotated pixel grid and its rotated neighbor directions stay consistent
//! with each other (the invariant symmetry-aware extraction and orbit expansion depend on).

use crate::ids::TileId;

// #region 🔖Transform
/// 🔄 One element of the dihedral group D4.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Transform2d {
    Identity,
    Rot90,
    Rot180,
    Rot270,
    FlipH,
    FlipV,
    FlipDiag,
    FlipAntiDiag,
}

type Mat2 = (i32, i32, i32, i32);

impl Transform2d {
    /// 🔄 All 8 elements, identity first.
    pub const ALL: [Transform2d; 8] = [
        Transform2d::Identity,
        Transform2d::Rot90,
        Transform2d::Rot180,
        Transform2d::Rot270,
        Transform2d::FlipH,
        Transform2d::FlipV,
        Transform2d::FlipDiag,
        Transform2d::FlipAntiDiag,
    ];

    fn matrix(self) -> Mat2 {
        match self {
            Transform2d::Identity => (1, 0, 0, 1),
            Transform2d::Rot90 => (0, -1, 1, 0),
            Transform2d::Rot180 => (-1, 0, 0, -1),
            Transform2d::Rot270 => (0, 1, -1, 0),
            Transform2d::FlipH => (-1, 0, 0, 1),
            Transform2d::FlipV => (1, 0, 0, -1),
            Transform2d::FlipDiag => (0, 1, 1, 0),
            Transform2d::FlipAntiDiag => (0, -1, -1, 0),
        }
    }

    fn from_matrix(m: Mat2) -> Self {
        Self::ALL.into_iter().find(|t| t.matrix() == m).expect("matrix is not a D4 element")
    }

    /// 🔄 `self` applied first, then `other` (i.e. `other ∘ self`).
    pub fn compose(self, other: Transform2d) -> Transform2d {
        let (a1, b1, c1, d1) = self.matrix();
        let (a2, b2, c2, d2) = other.matrix();
        Self::from_matrix((a2 * a1 + b2 * c1, a2 * b1 + b2 * d1, c2 * a1 + d2 * c1, c2 * b1 + d2 * d1))
    }

    pub fn inverse(self) -> Transform2d {
        match self {
            Transform2d::Rot90 => Transform2d::Rot270,
            Transform2d::Rot270 => Transform2d::Rot90,
            other => other,
        }
    }

    /// 🔄 Whether this transform swaps width and height when applied to a window.
    pub fn swaps_dimensions(self) -> bool {
        matches!(self, Transform2d::Rot90 | Transform2d::Rot270 | Transform2d::FlipDiag | Transform2d::FlipAntiDiag)
    }

    /// 🔄 Transforms a relative grid offset (e.g. a stencil direction).
    pub fn apply_offset(self, (dx, dy): (i32, i32)) -> (i32, i32) {
        let (a, b, c, d) = self.matrix();
        (a * dx + b * dy, c * dx + d * dy)
    }

    /// 🔄 Transforms a `width × height` row-major tile window, returning the new `(width, height)`
    /// (swapped for the four dimension-swapping elements) and the remapped tile content.
    pub fn apply_window(self, width: usize, height: usize, tiles: &[TileId]) -> (usize, usize, Vec<TileId>) {
        debug_assert_eq!(tiles.len(), width * height);
        let (nw, nh) = if self.swaps_dimensions() { (height, width) } else { (width, height) };
        let (a, b, c, d) = self.inverse().matrix();
        let mut out = vec![TileId(0); nw * nh];
        for oy in 0..nh {
            for ox in 0..nw {
                let cx = 2 * ox as i32 - (nw as i32 - 1);
                let cy = 2 * oy as i32 - (nh as i32 - 1);
                let sx2 = a * cx + b * cy;
                let sy2 = c * cx + d * cy;
                let sx = (sx2 + (width as i32 - 1)) / 2;
                let sy = (sy2 + (height as i32 - 1)) / 2;
                out[oy * nw + ox] = tiles[sy as usize * width + sx as usize];
            }
        }
        (nw, nh, out)
    }
}
// #endregion 🔖Transform

// #region 🔖Group
/// 🔄 A subgroup of D4 to expand patterns/tiles under.
#[derive(Clone, PartialEq, Debug)]
pub enum SymmetryGroup2d {
    /// 🔄 Just the identity — no expansion.
    None,
    /// 🔄 The four rotations only.
    C4,
    /// 🔄 Identity, 180° rotation, and both axis flips (no 90°/270°).
    D2,
    /// 🔄 The full 8-element dihedral group.
    D4,
    Custom(Vec<Transform2d>),
}

impl SymmetryGroup2d {
    pub fn elements(&self) -> Vec<Transform2d> {
        use Transform2d::*;
        match self {
            SymmetryGroup2d::None => vec![Identity],
            SymmetryGroup2d::C4 => vec![Identity, Rot90, Rot180, Rot270],
            SymmetryGroup2d::D2 => vec![Identity, Rot180, FlipH, FlipV],
            SymmetryGroup2d::D4 => Transform2d::ALL.to_vec(),
            SymmetryGroup2d::Custom(v) => v.clone(),
        }
    }
}
// #endregion 🔖Group

// #region 🔖Transform3d
type Mat3 = [[i32; 3]; 3];

fn mat3_mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut r = [[0i32; 3]; 3];
    for (i, row) in r.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    r
}

fn mat3_identity() -> Mat3 {
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
}

fn mat3_transpose(a: Mat3) -> Mat3 {
    let mut r = [[0i32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[j][i] = a[i][j];
        }
    }
    r
}

/// 🔄 One element of a cube's rotation/reflection symmetry group, represented by its 3×3 integer
/// orthogonal matrix (every entry in `{-1,0,1}`, exactly one nonzero per row/column). Constructed
/// only via [`cube_rotations_24`]/[`cube_symmetries_48`]'s closure computation, never by hand, so
/// every instance is guaranteed to actually be a symmetry of the cube.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Transform3d(Mat3);

impl Transform3d {
    pub fn identity() -> Self {
        Transform3d(mat3_identity())
    }

    /// 🔄 `self` applied first, then `other`.
    pub fn compose(self, other: Transform3d) -> Transform3d {
        Transform3d(mat3_mul(other.0, self.0))
    }

    /// 🔄 Orthogonal matrices' inverse is their transpose.
    pub fn inverse(self) -> Transform3d {
        Transform3d(mat3_transpose(self.0))
    }

    pub fn apply_offset(self, (dx, dy, dz): (i32, i32, i32)) -> (i32, i32, i32) {
        let m = self.0;
        (m[0][0] * dx + m[0][1] * dy + m[0][2] * dz, m[1][0] * dx + m[1][1] * dy + m[1][2] * dz, m[2][0] * dx + m[2][1] * dy + m[2][2] * dz)
    }

    pub fn determinant(self) -> i32 {
        let m = self.0;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
}

fn rot_x90() -> Mat3 {
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]]
}

fn rot_z90() -> Mat3 {
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
}

fn reflect_x() -> Mat3 {
    [[-1, 0, 0], [0, 1, 0], [0, 0, 1]]
}

fn closure(generators: &[Mat3]) -> Vec<Mat3> {
    let mut group = vec![mat3_identity()];
    let mut frontier = vec![mat3_identity()];
    while !frontier.is_empty() {
        let mut next_frontier = Vec::new();
        for m in &frontier {
            for g in generators {
                let candidate = mat3_mul(*g, *m);
                if !group.contains(&candidate) {
                    group.push(candidate);
                    next_frontier.push(candidate);
                }
            }
        }
        frontier = next_frontier;
    }
    group
}

/// 🔄 The 24 proper (orientation-preserving, determinant `+1`) rotations of a cube, generated by
/// closure from two 90° generators rather than hand-enumerated.
pub fn cube_rotations_24() -> Vec<Transform3d> {
    closure(&[rot_x90(), rot_z90()]).into_iter().map(Transform3d).collect()
}

/// 🔄 The full 48-element octahedral symmetry group (24 rotations plus their mirror images).
pub fn cube_symmetries_48() -> Vec<Transform3d> {
    closure(&[rot_x90(), rot_z90(), reflect_x()]).into_iter().map(Transform3d).collect()
}

/// 🔄 A subgroup of the cube's symmetry group to expand patterns/tiles under.
#[derive(Clone, Debug)]
pub enum SymmetryGroup3d {
    /// 🔄 Just the identity — no expansion.
    None,
    /// 🔄 All 24 proper rotations.
    Rot24,
    /// 🔄 All 48 rotations and reflections.
    Full48,
    /// 🔄 The four rotations about the Z axis only.
    ZRot4,
    Custom(Vec<Transform3d>),
}

impl SymmetryGroup3d {
    pub fn elements(&self) -> Vec<Transform3d> {
        match self {
            SymmetryGroup3d::None => vec![Transform3d::identity()],
            SymmetryGroup3d::Rot24 => cube_rotations_24(),
            SymmetryGroup3d::Full48 => cube_symmetries_48(),
            SymmetryGroup3d::ZRot4 => {
                let mut t = Transform3d::identity();
                let z90 = Transform3d(rot_z90());
                (0..4)
                    .map(|_| {
                        let cur = t;
                        t = t.compose(z90);
                        cur
                    })
                    .collect()
            }
            SymmetryGroup3d::Custom(v) => v.clone(),
        }
    }
}
// #endregion 🔖Transform3d

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_is_neutral() {
        for &t in &Transform2d::ALL {
            assert_eq!(t.compose(Transform2d::Identity), t);
            assert_eq!(Transform2d::Identity.compose(t), t);
        }
    }

    #[test]
    fn inverse_composes_to_identity() {
        for &t in &Transform2d::ALL {
            assert_eq!(t.compose(t.inverse()), Transform2d::Identity);
            assert_eq!(t.inverse().compose(t), Transform2d::Identity);
        }
    }

    #[test]
    fn four_quarter_rotations_is_identity() {
        let mut t = Transform2d::Identity;
        for _ in 0..4 {
            t = t.compose(Transform2d::Rot90);
        }
        assert_eq!(t, Transform2d::Identity);
    }

    #[test]
    fn two_flips_is_identity() {
        for &t in &[Transform2d::FlipH, Transform2d::FlipV, Transform2d::FlipDiag, Transform2d::FlipAntiDiag] {
            assert_eq!(t.compose(t), Transform2d::Identity);
        }
    }

    #[test]
    fn group_closure_every_composition_stays_in_d4() {
        for &a in &Transform2d::ALL {
            for &b in &Transform2d::ALL {
                let c = a.compose(b);
                assert!(Transform2d::ALL.contains(&c));
            }
        }
    }

    #[test]
    fn apply_offset_matches_apply_window_orientation() {
        // Rotating the offset (1,0) ("east") by Rot90 should match where the tile that was at the
        // window's east edge ends up after rotating the window itself.
        let w = 3usize;
        let h = 3usize;
        let tiles: Vec<TileId> = (0..9).map(TileId).collect();
        let (nw, nh, rotated) = Transform2d::Rot90.apply_window(w, h, &tiles);
        assert_eq!((nw, nh), (h, w));
        // The east-offset direction (1,0) rotates to (0,1) under Rot90.
        assert_eq!(Transform2d::Rot90.apply_offset((1, 0)), (0, 1));
        // Sanity: rotated window is a permutation of the same 9 tiles.
        let mut sorted = rotated;
        sorted.sort();
        let mut expected: Vec<TileId> = tiles;
        expected.sort();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn apply_window_round_trips_through_inverse() {
        let w = 3usize;
        let h = 2usize;
        let tiles: Vec<TileId> = (0..6).map(TileId).collect();
        for &t in &Transform2d::ALL {
            let (mw, mh, mid) = t.apply_window(w, h, &tiles);
            let (rw, rh, back) = t.inverse().apply_window(mw, mh, &mid);
            assert_eq!((rw, rh), (w, h), "transform {t:?} did not round-trip dimensions");
            assert_eq!(back, tiles, "transform {t:?} did not round-trip content");
        }
    }

    #[test]
    fn d4_group_has_eight_elements() {
        assert_eq!(SymmetryGroup2d::D4.elements().len(), 8);
        assert_eq!(SymmetryGroup2d::C4.elements().len(), 4);
        assert_eq!(SymmetryGroup2d::D2.elements().len(), 4);
        assert_eq!(SymmetryGroup2d::None.elements().len(), 1);
    }

    #[test]
    fn cube_rotation_group_has_exactly_24_elements() {
        let rots = cube_rotations_24();
        assert_eq!(rots.len(), 24);
        assert!(rots.iter().all(|t| t.determinant() == 1), "every proper rotation must have determinant +1");
    }

    #[test]
    fn cube_full_symmetry_group_has_exactly_48_elements() {
        let full = cube_symmetries_48();
        assert_eq!(full.len(), 48);
        let proper = full.iter().filter(|t| t.determinant() == 1).count();
        let improper = full.iter().filter(|t| t.determinant() == -1).count();
        assert_eq!(proper, 24);
        assert_eq!(improper, 24);
    }

    #[test]
    fn cube_rotations_are_closed_under_composition() {
        let rots = cube_rotations_24();
        for &a in &rots {
            for &b in &rots {
                let c = a.compose(b);
                assert!(rots.contains(&c), "composition left the rotation group");
            }
        }
    }

    #[test]
    fn cube_rotation_inverse_composes_to_identity() {
        let rots = cube_rotations_24();
        let id = Transform3d::identity();
        for &t in &rots {
            assert_eq!(t.compose(t.inverse()), id);
            assert_eq!(t.inverse().compose(t), id);
        }
    }

    #[test]
    fn cube_offset_transform_preserves_unit_offset_length() {
        let rots = cube_rotations_24();
        for &t in &rots {
            for &axis in &[(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                let (x, y, z) = t.apply_offset(axis);
                assert_eq!(x.abs() + y.abs() + z.abs(), 1, "a rotation must map a face offset to another face offset");
            }
        }
    }

    #[test]
    fn z_rot4_is_four_distinct_quarter_turns_returning_to_identity() {
        let elements = SymmetryGroup3d::ZRot4.elements();
        assert_eq!(elements.len(), 4);
        assert_eq!(elements[0], Transform3d::identity());
        for i in 0..4 {
            for j in (i + 1)..4 {
                assert_ne!(elements[i], elements[j], "ZRot4 elements must be pairwise distinct");
            }
        }
        // A fifth quarter-turn from the last element returns to identity.
        let z90 = elements[1];
        assert_eq!(elements[3].compose(z90), Transform3d::identity());
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Symmetry

// #region 🔖Extract
pub mod extract {
//! 🧪 Overlapping-pattern extraction from 2D tile samples: `N × N` windows become patterns,
//! frequency becomes weight, and overlap agreement under the four unit grid offsets becomes
//! compatibility — the classic overlapping-WFC pipeline. Deliberately reuses
//! [`crate::grid2d::declare_stencil_relations`] so an extracted model's relations line up exactly
//! with a [`crate::grid2d::Grid2dTopology`] built with the same (von Neumann) stencil.

use crate::grid2d::{Stencil2d, declare_stencil_relations};
use crate::ids::{PatternId, TileId};
use crate::model::{CompiledModel, ModelBuilder};
use crate::symmetry::SymmetryGroup2d;

// #region 🔖Sample
/// 🧪 A row-major tile-id matrix to learn patterns from.
#[derive(Clone, Debug)]
pub struct Sample2d {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileId>,
}

impl Sample2d {
    pub fn new(width: usize, height: usize, tiles: Vec<TileId>) -> Self {
        debug_assert_eq!(tiles.len(), width * height);
        Self { width, height, tiles }
    }
}
// #endregion 🔖Sample

// #region 🔖Config
/// 🧪 Options for [`extract_2d`].
#[derive(Clone, Debug)]
pub struct Extract2dConfig {
    /// 🧪 Window side length (patterns are `window × window`).
    pub window: usize,
    /// 🧪 Whether windows wrap around sample edges (periodic input) or are only taken from
    /// fully-in-bounds positions.
    pub periodic_input: bool,
    pub symmetry: SymmetryGroup2d,
}

impl Default for Extract2dConfig {
    fn default() -> Self {
        Self { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None }
    }
}
// #endregion 🔖Config

// #region 🔖Decoder
/// 🧪 Maps extracted `PatternId`s back to the tile at each pattern window's top-left corner (the
/// anchor convention: overlap-agreement between neighboring patterns guarantees every cell's
/// anchor tile is mutually consistent with its neighbors' anchor tiles).
#[derive(Clone, Debug)]
pub struct PatternDecoder2d {
    window: usize,
    pattern_windows: Vec<Vec<TileId>>,
}

impl PatternDecoder2d {
    pub fn window(&self) -> usize {
        self.window
    }

    pub fn anchor_tile(&self, p: PatternId) -> TileId {
        self.pattern_windows[p.index()][0]
    }

    pub fn window_of(&self, p: PatternId) -> &[TileId] {
        &self.pattern_windows[p.index()]
    }

    /// 🧪 Decodes a full grid assignment to its anchor-tile image, row-major.
    pub fn decode(&self, assignment: &[PatternId]) -> Vec<TileId> {
        assignment.iter().map(|&p| self.anchor_tile(p)).collect()
    }
}
// #endregion 🔖Decoder

// #region 🔖Extract
/// 🧪 The compiled model plus everything needed to decode its patterns back to tiles.
#[derive(Clone, Debug)]
pub struct ExtractedModel2d {
    pub model: CompiledModel,
    pub decoder: PatternDecoder2d,
}

fn window_at(sample: &Sample2d, x: usize, y: usize, n: usize, periodic: bool) -> Option<Vec<TileId>> {
    if !periodic && (x + n > sample.width || y + n > sample.height) {
        return None;
    }
    let mut w = vec![TileId(0); n * n];
    for wy in 0..n {
        for wx in 0..n {
            let sx = if periodic { (x + wx) % sample.width } else { x + wx };
            let sy = if periodic { (y + wy) % sample.height } else { y + wy };
            w[wy * n + wx] = sample.tiles[sy * sample.width + sx];
        }
    }
    Some(w)
}

/// 🧪 `a` placed at the origin, `b` placed at grid offset `(dx, dy)` — compatible iff every cell
/// where their `n × n` footprints overlap holds the same tile.
fn windows_overlap_compatible(a: &[TileId], b: &[TileId], n: usize, dx: i32, dy: i32) -> bool {
    for y in 0..n as i32 {
        for x in 0..n as i32 {
            let bx = x - dx;
            let by = y - dy;
            if bx >= 0 && bx < n as i32 && by >= 0 && by < n as i32 && a[y as usize * n + x as usize] != b[by as usize * n + bx as usize] {
                return false;
            }
        }
    }
    true
}

/// 🧪 Extracts overlapping patterns from one or more samples (frequencies merge across samples),
/// expanding each window under `cfg.symmetry` before deduplication, and compiles a model whose
/// relations are exactly [`Stencil2d::VonNeumann`]'s four unit offsets.
pub fn extract_2d(samples: &[Sample2d], cfg: &Extract2dConfig) -> Result<ExtractedModel2d, crate::error::ModelError> {
    use crate::error::ModelError;
    let n = cfg.window;
    if n == 0 {
        return Err(ModelError::CapacityOverflow { what: "extract_2d window size" });
    }

    let mut window_freq: std::collections::HashMap<Vec<TileId>, u64> = std::collections::HashMap::new();
    for sample in samples {
        let (x_positions, y_positions): (usize, usize) = if cfg.periodic_input { (sample.width, sample.height) } else { (sample.width.saturating_sub(n - 1), sample.height.saturating_sub(n - 1)) };
        for y in 0..y_positions {
            for x in 0..x_positions {
                let Some(base) = window_at(sample, x, y, n, cfg.periodic_input) else { continue };
                for transform in cfg.symmetry.elements() {
                    let (tw, th, tiles) = transform.apply_window(n, n, &base);
                    debug_assert_eq!((tw, th), (n, n), "square windows are invariant under D4 dimension swap");
                    *window_freq.entry(tiles).or_insert(0) += 1;
                }
            }
        }
    }
    if window_freq.is_empty() {
        return Err(ModelError::EmptyPatternUniverse);
    }

    let mut windows: Vec<(Vec<TileId>, u64)> = window_freq.into_iter().collect();
    windows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut builder = ModelBuilder::new();
    for &(_, freq) in &windows {
        builder.add_pattern(freq as f64);
    }
    let relations = declare_stencil_relations(&mut builder, &Stencil2d::VonNeumann)?;
    let offsets = Stencil2d::VonNeumann.offsets();

    for i in 0..windows.len() {
        for j in 0..windows.len() {
            for (k, &(dx, dy)) in offsets.iter().enumerate() {
                if windows_overlap_compatible(&windows[i].0, &windows[j].0, n, dx, dy) {
                    builder.allow(relations[k], PatternId::from_index(i), PatternId::from_index(j));
                }
            }
        }
    }

    let model = builder.compile()?;
    let decoder = PatternDecoder2d { window: n, pattern_windows: windows.into_iter().map(|(w, _)| w).collect() };
    Ok(ExtractedModel2d { model, decoder })
}
// #endregion 🔖Extract

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn checkerboard_sample(size: usize) -> Sample2d {
        let mut tiles = vec![TileId(0); size * size];
        for y in 0..size {
            for x in 0..size {
                tiles[y * size + x] = TileId(((x + y) % 2) as u32);
            }
        }
        Sample2d::new(size, size, tiles)
    }

    #[test]
    fn extraction_rejects_empty_sample_list() {
        let cfg = Extract2dConfig::default();
        assert!(extract_2d(&[], &cfg).is_err());
    }

    #[test]
    fn window_one_extracts_one_pattern_per_distinct_tile() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 1, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        assert_eq!(extracted.model.pattern_count(), 2);
    }

    #[test]
    fn window_two_deduplicates_repeated_windows() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        // A periodic 2-color checkerboard has exactly 2 distinct 2x2 windows under periodic wrap.
        assert_eq!(extracted.model.pattern_count(), 2);
    }

    #[test]
    fn symmetry_expansion_can_only_add_patterns_never_remove() {
        let sample = checkerboard_sample(4);
        let cfg_none = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let cfg_d4 = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::D4 };
        let none = extract_2d(std::slice::from_ref(&sample), &cfg_none).unwrap();
        let d4 = extract_2d(&[sample], &cfg_d4).unwrap();
        assert!(d4.model.pattern_count() >= none.model.pattern_count());
    }

    #[test]
    fn extracted_model_relations_match_von_neumann_stencil() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        assert_eq!(extracted.model.relation_count(), 4);
    }

    #[test]
    fn periodic_sample_solves_on_a_same_size_wrapped_grid() {
        // The canonical WFC sanity check: a periodic training sample's own tiling must remain a
        // satisfiable solution of the extracted model on a same-size, wrap-boundary grid — if
        // extraction/compatibility were buggy, even the sample's own arrangement could become
        // unsolvable.
        use crate::grid2d::{Boundary, Grid2dTopology};
        use crate::solver_grid2d::Grid2dSolverBuilder;

        let size = 4;
        let sample = checkerboard_sample(size);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();

        let relations = Stencil2d::VonNeumann.offsets().iter().enumerate().map(|(i, _)| crate::ids::RelationId(i as u32)).collect::<Vec<_>>();
        let topo = Grid2dTopology::new(size, size, &Stencil2d::VonNeumann, relations, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let mut solver = Grid2dSolverBuilder::new(extracted.model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, crate::outcome::SolveOutcome::Solved(_)), "extracted model must remain solvable on a same-size wrapped grid");
    }

    #[test]
    fn window_content_is_preserved_for_decode() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        for p in 0..extracted.model.pattern_count() {
            let pid = PatternId::from_index(p);
            assert_eq!(extracted.decoder.window_of(pid).len(), 4);
        }
    }

    #[test]
    fn multiple_samples_merge_frequencies() {
        let a = checkerboard_sample(4);
        let b = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 1, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let single = extract_2d(std::slice::from_ref(&a), &cfg).unwrap();
        let merged = extract_2d(&[a, b], &cfg).unwrap();
        assert_eq!(single.model.pattern_count(), merged.model.pattern_count());
        // Each pattern's weight should double when the same sample is provided twice.
        for p in 0..merged.model.pattern_count() {
            let pid = PatternId::from_index(p);
            assert!((merged.model.pattern_info(pid).weight - 2.0 * single.model.pattern_info(pid).weight).abs() < 1e-9);
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Extract

// #region 🔖Grid3D
pub mod grid3d {
//! 🧊 Dense 3D grid topology: the exact 2D design of [`crate::grid2d`] extended to a third axis
//! (same arithmetic-neighbor, zero-adjacency-storage approach, same [`crate::grid2d::Boundary`]
//! per-axis semantics — reused directly rather than duplicated). `NodeId(z * width * height + y * width + x)`.

use crate::error::{ModelError, TopologyError};
use crate::grid2d::{Boundary, resolve_coord};
use crate::ids::{NodeId, PatternId, RegionId, RelationId};
use crate::model::ModelBuilder;
use crate::tiled::TiledModelBuilder;
use crate::topology::Topology;

// #region 🔖Stencil
/// 🧊 Which offsets count as "neighbors" of a 3D cell.
#[derive(Clone, PartialEq, Debug)]
pub enum Stencil3d {
    /// 🧊 6-neighbor: the six face-adjacent cells.
    Face6,
    /// 🧊 18-neighbor: face- and edge-adjacent cells (Manhattan distance 1 or 2, excluding corners).
    Edge18,
    /// 🧊 26-neighbor: every cell in the surrounding 3×3×3 block.
    Vertex26,
    /// 🧊 An arbitrary offset list, each entry's negation required to also be present.
    Custom(Vec<(i32, i32, i32)>),
}

impl Stencil3d {
    pub fn offsets(&self) -> Vec<(i32, i32, i32)> {
        match self {
            Stencil3d::Face6 => vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)],
            Stencil3d::Edge18 => vertex26_offsets().into_iter().filter(|&(x, y, z)| x.abs() + y.abs() + z.abs() <= 2).collect(),
            Stencil3d::Vertex26 => vertex26_offsets(),
            Stencil3d::Custom(v) => v.clone(),
        }
    }

    fn validate(&self) -> Result<(), TopologyError> {
        let offsets = self.offsets();
        if offsets.is_empty() {
            return Err(TopologyError::InvalidStencil { reason: "stencil has zero offsets" });
        }
        for (i, &a) in offsets.iter().enumerate() {
            if a == (0, 0, 0) {
                return Err(TopologyError::InvalidStencil { reason: "self-offset (0,0,0) is not supported" });
            }
            for &b in &offsets[i + 1..] {
                if a == b {
                    return Err(TopologyError::InvalidStencil { reason: "duplicate offset" });
                }
            }
            if !offsets.contains(&(-a.0, -a.1, -a.2)) {
                return Err(TopologyError::InvalidStencil { reason: "offset's negation is not present in the stencil" });
            }
        }
        Ok(())
    }
}

fn vertex26_offsets() -> Vec<(i32, i32, i32)> {
    let mut v = Vec::with_capacity(26);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if (dx, dy, dz) != (0, 0, 0) {
                    v.push((dx, dy, dz));
                }
            }
        }
    }
    v
}

/// 🧊 Registers one directed relation per stencil offset (paired with its negation as inverse) and
/// returns them in `stencil.offsets()` order, ready to pass to [`Grid3dTopology::new`].
pub fn declare_stencil_relations_3d(builder: &mut ModelBuilder, stencil: &Stencil3d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_3d" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy, dz) in &offsets {
        relations.push(builder.add_relation(&format!("offset({dx},{dy},{dz})")));
    }
    for (i, &(dx, dy, dz)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy, -dz)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}

/// 🧊 [`declare_stencil_relations_3d`] for a [`TiledModelBuilder`].
pub fn declare_stencil_relations_3d_tiled(builder: &mut TiledModelBuilder, stencil: &Stencil3d) -> Result<Vec<RelationId>, ModelError> {
    stencil.validate().map_err(|_| ModelError::InvalidSymmetryGroup { reason: "invalid stencil passed to declare_stencil_relations_3d_tiled" })?;
    let offsets = stencil.offsets();
    let mut relations = Vec::with_capacity(offsets.len());
    for &(dx, dy, dz) in &offsets {
        relations.push(builder.relation(&format!("offset({dx},{dy},{dz})")));
    }
    for (i, &(dx, dy, dz)) in offsets.iter().enumerate() {
        if let Some(j) = offsets.iter().position(|&o| o == (-dx, -dy, -dz)) {
            builder.set_relation_inverse(relations[i], relations[j]);
        }
    }
    Ok(relations)
}
// #endregion 🔖Stencil

// #region 🔖Topology
/// 🧊 A dense, z-major-then-row-major 3D grid topology. `NodeId(z*width*height + y*width + x)`.
#[derive(Clone, Debug)]
pub struct Grid3dTopology {
    width: usize,
    height: usize,
    depth: usize,
    offsets: Vec<(i32, i32, i32)>,
    relations: Vec<RelationId>,
    boundary_x: Boundary,
    boundary_y: Boundary,
    boundary_z: Boundary,
    mask: Option<Vec<bool>>,
}

impl Grid3dTopology {
    #[allow(clippy::too_many_arguments)]
    pub fn new(width: usize, height: usize, depth: usize, stencil: &Stencil3d, relations: Vec<RelationId>, boundary_x: Boundary, boundary_y: Boundary, boundary_z: Boundary, mask: Option<Vec<bool>>) -> Result<Self, TopologyError> {
        if width == 0 {
            return Err(TopologyError::ZeroDimension { axis: "width" });
        }
        if height == 0 {
            return Err(TopologyError::ZeroDimension { axis: "height" });
        }
        if depth == 0 {
            return Err(TopologyError::ZeroDimension { axis: "depth" });
        }
        width.checked_mul(height).and_then(|wh| wh.checked_mul(depth)).ok_or(TopologyError::SizeOverflow)?;
        stencil.validate()?;
        let offsets = stencil.offsets();
        if offsets.len() != relations.len() {
            return Err(TopologyError::InvalidStencil { reason: "relations length does not match stencil offset count" });
        }
        if let Some(m) = &mask {
            if m.len() != width * height * depth {
                return Err(TopologyError::MaskShapeMismatch { expected: width * height * depth, actual: m.len() });
            }
        }
        Ok(Self { width, height, depth, offsets, relations, boundary_x, boundary_y, boundary_z, mask })
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    #[inline]
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.width * self.height + y * self.width + x
    }

    #[inline]
    pub fn node_at(&self, x: usize, y: usize, z: usize) -> Option<NodeId> {
        if x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        Some(NodeId::from_index(self.index(x, y, z)))
    }

    #[inline]
    pub fn coords(&self, n: NodeId) -> (usize, usize, usize) {
        let idx = n.index();
        let plane = self.width * self.height;
        let z = idx / plane;
        let rem = idx % plane;
        (rem % self.width, rem / self.width, z)
    }

    #[inline]
    pub fn is_active(&self, x: usize, y: usize, z: usize) -> bool {
        self.mask.as_ref().is_none_or(|m| m[self.index(x, y, z)])
    }

    pub fn inactive_cells(&self) -> Vec<NodeId> {
        let Some(mask) = &self.mask else { return Vec::new() };
        (0..mask.len()).filter(|&i| !mask[i]).map(NodeId::from_index).collect()
    }

    /// 🧊 Every `(node, relation, outside_pattern)` an edge cell must be restricted by at init
    /// time, derived from [`Boundary::FixedOutside`] axes.
    pub fn fixed_outside_restrictions(&self) -> Vec<(NodeId, RelationId, PatternId)> {
        let mut out = Vec::new();
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    if !self.is_active(x, y, z) {
                        continue;
                    }
                    for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
                        let tx = x as i32 + dx;
                        let ty = y as i32 + dy;
                        let tz = z as i32 + dz;
                        let x_out = tx < 0 || tx as usize >= self.width;
                        let y_out = ty < 0 || ty as usize >= self.height;
                        let z_out = tz < 0 || tz as usize >= self.depth;
                        let node = NodeId::from_index(self.index(x, y, z));
                        if x_out {
                            if let Boundary::FixedOutside(p) = self.boundary_x {
                                out.push((node, self.relations[i], p));
                                continue;
                            }
                        }
                        if y_out {
                            if let Boundary::FixedOutside(p) = self.boundary_y {
                                out.push((node, self.relations[i], p));
                                continue;
                            }
                        }
                        if z_out {
                            if let Boundary::FixedOutside(p) = self.boundary_z {
                                out.push((node, self.relations[i], p));
                            }
                        }
                    }
                }
            }
        }
        out
    }
}

impl Topology for Grid3dTopology {
    #[inline]
    fn node_count(&self) -> usize {
        self.width * self.height * self.depth
    }

    fn arc_count(&self) -> usize {
        let mut count = 0;
        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    if !self.is_active(x, y, z) {
                        continue;
                    }
                    for &(dx, dy, dz) in &self.offsets {
                        let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
                        let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
                        let rz = resolve_coord(z as i32 + dz, self.depth, self.boundary_z);
                        if let (Some(nx), Some(ny), Some(nz)) = (rx, ry, rz) {
                            if self.is_active(nx, ny, nz) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    #[inline]
    fn region_of(&self, _n: NodeId) -> RegionId {
        RegionId(0)
    }

    fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let (x, y, z) = self.coords(n);
        if !self.is_active(x, y, z) {
            return;
        }
        for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 + dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 + dy, self.height, self.boundary_y);
            let rz = resolve_coord(z as i32 + dz, self.depth, self.boundary_z);
            if let (Some(nx), Some(ny), Some(nz)) = (rx, ry, rz) {
                if self.is_active(nx, ny, nz) {
                    f(NodeId::from_index(self.index(nx, ny, nz)), self.relations[i]);
                }
            }
        }
    }

    fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let (x, y, z) = self.coords(n);
        if !self.is_active(x, y, z) {
            return;
        }
        for (i, &(dx, dy, dz)) in self.offsets.iter().enumerate() {
            let rx = resolve_coord(x as i32 - dx, self.width, self.boundary_x);
            let ry = resolve_coord(y as i32 - dy, self.height, self.boundary_y);
            let rz = resolve_coord(z as i32 - dz, self.depth, self.boundary_z);
            if let (Some(sx), Some(sy), Some(sz)) = (rx, ry, rz) {
                if self.is_active(sx, sy, sz) {
                    f(NodeId::from_index(self.index(sx, sy, sz)), self.relations[i], n.index() * self.offsets.len() + i);
                }
            }
        }
    }

    fn max_in_degree(&self) -> usize {
        self.offsets.len()
    }
}
// #endregion 🔖Topology

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face6_edge18_vertex26_offset_counts() {
        assert_eq!(Stencil3d::Face6.offsets().len(), 6);
        assert_eq!(Stencil3d::Edge18.offsets().len(), 18);
        assert_eq!(Stencil3d::Vertex26.offsets().len(), 26);
    }

    #[test]
    fn all_built_in_stencils_validate() {
        Stencil3d::Face6.validate().unwrap();
        Stencil3d::Edge18.validate().unwrap();
        Stencil3d::Vertex26.validate().unwrap();
    }

    #[test]
    fn node_at_and_coords_roundtrip() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 4, 5, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        let n = topo.node_at(1, 2, 3).unwrap();
        assert_eq!(topo.coords(n), (1, 2, 3));
        assert_eq!(topo.node_at(3, 0, 0), None);
    }

    #[test]
    fn open_boundary_corner_has_three_neighbors() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        let corner = topo.node_at(0, 0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn wrap_boundary_connects_all_axes() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Wrap, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let corner = topo.node_at(0, 0, 0).unwrap();
        let mut out = Vec::new();
        topo.for_each_out_arc(corner, |m, _| out.push(m));
        assert_eq!(out.len(), 6);
    }

    #[test]
    fn mask_excludes_inactive_voxels() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
        let mut mask = vec![true; 27];
        mask[13] = false; // center of 3x3x3
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let neighbor = topo.node_at(1, 1, 0).unwrap(); // directly below center
        let mut out = Vec::new();
        topo.for_each_out_arc(neighbor, |m, _| out.push(m));
        assert!(!out.contains(&topo.node_at(1, 1, 1).unwrap()));
        assert_eq!(topo.inactive_cells(), vec![NodeId(13)]);
    }

    #[test]
    fn in_arc_matches_out_arc_on_open_boundary() {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Vertex26).unwrap();
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Vertex26, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    let n = topo.node_at(x, y, z).unwrap();
                    let mut outgoing = Vec::new();
                    topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
                    let mut reconstructed = Vec::new();
                    for oz in 0..3 {
                        for oy in 0..3 {
                            for ox in 0..3 {
                                let other = topo.node_at(ox, oy, oz).unwrap();
                                let mut theirs = Vec::new();
                                topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                                for &(s, r) in &theirs {
                                    if s == n {
                                        reconstructed.push((other, r));
                                    }
                                }
                            }
                        }
                    }
                    outgoing.sort_by_key(|&(m, r)| (m.get(), r.get()));
                    reconstructed.sort_by_key(|&(m, r)| (m.get(), r.get()));
                    assert_eq!(outgoing, reconstructed, "voxel ({x},{y},{z}) out-arcs must match in-arc reconstruction");
                }
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Grid3D

// #region 🔖Solvergrid3D
pub mod solver_grid3d {
//! 🧊 `Grid3dSolver`: dense 3D grid solving. Exactly [`crate::solver_grid2d`]'s design extended to
//! a third axis — masked-out voxels and [`crate::grid2d::Boundary::FixedOutside`] faces fold into
//! ordinary domain overrides and fixed pins before delegating to the same generic kernel.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::grid3d::Grid3dTopology;
use crate::ids::PatternId;
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, CancelToken, SearchConfig};
use crate::topology::Topology;

// #region 🔖Builder
/// 🏗️ Builds a [`Grid3dSolver`] over a dense `width × height × depth` grid.
pub struct Grid3dSolverBuilder {
    model: CompiledModel,
    topology: Grid3dTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
}

impl Grid3dSolverBuilder {
    pub fn new(model: CompiledModel, topology: Grid3dTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    pub fn fix(mut self, x: usize, y: usize, z: usize, p: PatternId) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y, z).ok_or(SolveError::ModelTopologyMismatch { reason: "fix() coordinate out of range" })?;
        self.fixed.push((n, p));
        Ok(self)
    }

    pub fn domain(mut self, x: usize, y: usize, z: usize, allowed: PatternSet) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y, z).ok_or(SolveError::ModelTopologyMismatch { reason: "domain() coordinate out of range" })?;
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        Ok(self)
    }

    pub fn config(mut self, cfg: SearchConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 🏗️ Adds a global constraint. See [`crate::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<Grid3dSolver, SolveError> {
        let node_count = self.topology.node_count();
        let mut init_domains = self.init_domains.unwrap_or_else(|| vec![self.model.full_domain(); node_count]);
        let mut fixed = self.fixed;

        for (n, rel, outside_pattern) in self.topology.fixed_outside_restrictions() {
            init_domains[n.index()].and_with(self.model.allowed(rel, outside_pattern));
        }
        let placeholder = PatternId(0);
        for n in self.topology.inactive_cells() {
            fixed.push((n, placeholder));
        }

        let adjacency = build_adjacency_view(&self.topology);
        Ok(Grid3dSolver { model: self.model, topology: self.topology, init_domains, fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖Builder

// #region 🔖Solver
/// 🧊 A WFC solver over a dense 3D grid.
pub struct Grid3dSolver {
    model: CompiledModel,
    topology: Grid3dTopology,
    init_domains: Vec<PatternSet>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl Grid3dSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() { None } else { Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency }) }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, cancel),
        }
    }

    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit),
        }
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &Grid3dTopology {
        &self.topology
    }

    pub fn get(&self, solution: &Solution, x: usize, y: usize, z: usize) -> Option<PatternId> {
        let n = self.topology.node_at(x, y, z)?;
        solution.assignment.get(n.index()).copied()
    }

    pub fn decode_tiles(&self, solution: &Solution) -> Vec<Option<crate::ids::TileId>> {
        solution.assignment.iter().map(|&p| self.model.pattern_info(p).tile).collect()
    }
}
// #endregion 🔖Solver

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid2d::Boundary;
    use crate::grid3d::{Stencil3d, declare_stencil_relations_3d_tiled};
    use crate::tiled::TiledModelBuilder;

    fn checkerboard3d(size: usize, boundary: Boundary) -> (CompiledModel, Grid3dTopology) {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let topo = Grid3dTopology::new(size, size, size, &Stencil3d::Face6, rels, boundary, boundary, boundary, None).unwrap();
        (model, topo)
    }

    #[test]
    fn solves_a_checkerboard_volume() {
        let (model, topo) = checkerboard3d(4, Boundary::Open);
        let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn fix_pins_a_voxel_and_propagates() {
        let (model, topo) = checkerboard3d(3, Boundary::Open);
        let black = PatternId(0);
        let white = PatternId(1);
        let mut solver = Grid3dSolverBuilder::new(model, topo).fix(0, 0, 0, black).unwrap().build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                assert_eq!(solver.get(&sol, 0, 0, 0), Some(black));
                assert_eq!(solver.get(&sol, 1, 0, 0), Some(white));
                assert_eq!(solver.get(&sol, 0, 0, 1), Some(white));
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn masked_voxels_are_excluded_and_solve_completes() {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let mut mask = vec![true; 27];
        mask[13] = false;
        let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
        assert!(matches!(solver.solve(1), SolveOutcome::Solved(_)));
    }

    #[test]
    fn wrap_boundary_solves_consistently() {
        let (model, topo) = checkerboard3d(4, Boundary::Wrap);
        let mut solver = Grid3dSolverBuilder::new(model, topo).build().unwrap();
        assert!(matches!(solver.solve(1), SolveOutcome::Solved(_)));
    }

    #[test]
    fn odd_size_wrap_is_unsatisfiable_for_two_color_checkerboard() {
        let (model, topo) = checkerboard3d(3, Boundary::Wrap);
        let mut solver = Grid3dSolverBuilder::new(model, topo).config(SearchConfig { mode: search::SearchMode::Backtrack, ..Default::default() }).build().unwrap();
        assert!(matches!(solver.solve(1), SolveOutcome::Unsatisfiable(_)));
    }

    #[test]
    fn graph_vs_grid3d_strict_equivalence_face6_open() {
        // Independently hand-enumerated arcs for a 2x2x3 Face6/Open grid, fed into a
        // GraphTopology, compared against the same model solved through Grid3dTopology.
        let width = 2usize;
        let height = 2usize;
        let depth = 3usize;
        let mut b = TiledModelBuilder::new();
        let tiles: Vec<_> = (0..3).map(|i| b.tile(1.0 + i as f64)).collect();
        let rels = declare_stencil_relations_3d_tiled(&mut b, &Stencil3d::Face6).unwrap();
        for &r in &rels {
            for &a in &tiles {
                for &c in &tiles {
                    if a != c {
                        b.allow(r, a, c);
                    }
                }
            }
        }
        let model = b.compile().unwrap();

        let idx = |x: usize, y: usize, z: usize| crate::ids::NodeId::from_index(z * width * height + y * width + x);
        let mut hand_arcs = Vec::new();
        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    if x + 1 < width {
                        hand_arcs.push((idx(x, y, z), idx(x + 1, y, z), rels[0]));
                        hand_arcs.push((idx(x + 1, y, z), idx(x, y, z), rels[1]));
                    }
                    if y + 1 < height {
                        hand_arcs.push((idx(x, y, z), idx(x, y + 1, z), rels[2]));
                        hand_arcs.push((idx(x, y + 1, z), idx(x, y, z), rels[3]));
                    }
                    if z + 1 < depth {
                        hand_arcs.push((idx(x, y, z), idx(x, y, z + 1), rels[4]));
                        hand_arcs.push((idx(x, y, z + 1), idx(x, y, z), rels[5]));
                    }
                }
            }
        }
        let mut gb = crate::topology::GraphTopologyBuilder::new(width * height * depth);
        for (from, to, r) in hand_arcs {
            gb.arc(from, to, r);
        }
        let graph_topo = gb.build().unwrap();
        let grid_topo = Grid3dTopology::new(width, height, depth, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();

        let config = SearchConfig::default();
        for seed in 0..10u64 {
            let mut graph_solver = crate::solver_graph::GraphSolverBuilder::new(model.clone(), graph_topo.clone()).config(config).build().unwrap();
            let mut grid_solver = Grid3dSolverBuilder::new(model.clone(), grid_topo.clone()).config(config).build().unwrap();
            let graph_outcome = graph_solver.solve(seed);
            let grid_outcome = grid_solver.solve(seed);
            match (graph_outcome, grid_outcome) {
                (SolveOutcome::Solved(g), SolveOutcome::Solved(r)) => {
                    assert_eq!(g.assignment, r.assignment, "seed {seed}: graph and grid3d solutions diverged");
                    assert_eq!(g.report.metrics.observations, r.report.metrics.observations, "seed {seed}: observation counts diverged");
                }
                (a, b) => panic!("seed {seed}: outcome mismatch, graph={a:?} grid={b:?}"),
            }
        }
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Solvergrid3D

// #region 🔖Serial
pub mod serial {
//! 💾 Versioned, human-editable serialization schemas. Compiled runtime state (`CompiledModel`'s
//! bitset tables, a live `Checkpoint`) is never trusted directly from an external source:
//! [`SourceModelDoc`] always recompiles through [`crate::model::ModelBuilder`] (the exact same
//! validation path a freshly authored model goes through) and [`CheckpointDoc`] always
//! structurally revalidates against a live model/topology before becoming a usable
//! [`crate::trail::Checkpoint`]. This is one of the few places outside `ids.rs` this crate derives
//! `serde::Serialize`/`Deserialize` directly on a public type — deliberately, since these types'
//! entire purpose is to cross a serialization boundary; JSON convenience is just `serde_json`
//! applied directly to them (see this module's tests), no wrapper needed.

use crate::bitset::PatternSet;
use crate::error::{ModelError, SolveError};
use crate::ids::{PatternId, RelationId};
use crate::model::{CompiledModel, ModelBuilder};
use crate::trail::Checkpoint;
use serde::{Deserialize, Serialize};

// #region 🔖SourceModel
/// 💾 Current [`SourceModelDoc`] schema version. Bump on any breaking field change; old versions
/// are rejected outright by [`SourceModelDoc::compile`], never migrated.
pub const SOURCE_MODEL_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternDoc {
    pub weight: f64,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationDoc {
    pub name: String,
    /// 💾 Index into the document's own `relations` list this relation is the inverse of;
    /// `None` means self-inverse.
    #[serde(default)]
    pub inverse: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairDoc {
    pub relation: u32,
    pub src: u32,
    pub dst: u32,
}

/// 💾 A versioned, human-editable model schema — the input shape [`crate::model::ModelBuilder`]
/// consumes, not [`CompiledModel`]'s compiled bitset tables. Deliberately does not capture
/// [`crate::tiled::TiledModelBuilder`]'s higher-level socket/symmetry authoring (deferred; a tiled
/// model already compiles down to this exact pattern/relation/allow shape, so round-tripping
/// through here is compile-equivalent, just not re-editable at the socket level). `deny` pairs are
/// not reconstructed from a compiled model either — by compile time `deny` has already been folded
/// into `allow`'s absence, so [`SourceModelDoc::from_model`] only ever emits `allow`; a hand-authored
/// document may still use both.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceModelDoc {
    pub version: u32,
    pub patterns: Vec<PatternDoc>,
    pub relations: Vec<RelationDoc>,
    #[serde(default)]
    pub allow: Vec<PairDoc>,
    #[serde(default)]
    pub deny: Vec<PairDoc>,
}

impl SourceModelDoc {
    /// 💾 Captures `model`'s pattern/relation/tag/allow shape as a serializable document.
    pub fn from_model(model: &CompiledModel) -> Self {
        let patterns = (0..model.pattern_count())
            .map(|i| {
                let info = model.pattern_info(PatternId::from_index(i));
                let tags = info.tags.iter().filter_map(|&t| model.tag_name(t)).map(str::to_string).collect();
                PatternDoc { weight: info.weight, tags }
            })
            .collect();

        let relations = (0..model.relation_count())
            .map(|i| {
                let info = model.relation_info(RelationId::from_index(i));
                let inv = info.inverse.index();
                RelationDoc { name: info.name.clone(), inverse: if inv == i { None } else { Some(inv as u32) } }
            })
            .collect();

        let mut allow = Vec::new();
        for ri in 0..model.relation_count() {
            let r = RelationId::from_index(ri);
            for src in 0..model.pattern_count() {
                let src_id = PatternId::from_index(src);
                for dst in model.allowed(r, src_id).iter_ones() {
                    allow.push(PairDoc { relation: ri as u32, src: src as u32, dst: dst.get() });
                }
            }
        }

        Self { version: SOURCE_MODEL_VERSION, patterns, relations, allow, deny: Vec::new() }
    }

    /// 💾 Recompiles into a validated [`CompiledModel`] via the same `ModelBuilder::compile` +
    /// `validate()` path any hand-written builder code goes through — an untrusted document never
    /// takes a shortcut around inverse-consistency checking.
    pub fn compile(&self) -> Result<CompiledModel, ModelError> {
        if self.version != SOURCE_MODEL_VERSION {
            return Err(ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: self.version });
        }
        let mut b = ModelBuilder::new();
        for p in &self.patterns {
            let id = b.add_pattern(p.weight);
            for tag in &p.tags {
                b.add_tag(id, tag);
            }
        }
        for r in &self.relations {
            b.add_relation(&r.name);
        }
        for (i, r) in self.relations.iter().enumerate() {
            if let Some(inv) = r.inverse {
                b.set_relation_inverse(RelationId::from_index(i), RelationId::from_index(inv as usize));
            }
        }
        for pair in &self.allow {
            b.allow(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        for pair in &self.deny {
            b.deny(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        let compiled = b.compile()?;
        compiled.validate()?;
        Ok(compiled)
    }
}
// #endregion 🔖SourceModel

// #region 🔖Checkpoint
/// 💾 Current [`CheckpointDoc`] schema version.
pub const CHECKPOINT_VERSION: u32 = 1;

/// 💾 A versioned, serializable [`Checkpoint`]. Structurally revalidated against a live model and
/// node count on load ([`CheckpointDoc::into_checkpoint`]) — bitset lengths, per-domain word-count/
/// padding-bit well-formedness, domain count, and model fingerprint are all checked, so a
/// hand-tampered file fails with [`SolveError`] rather than panicking or silently corrupting a
/// resumed solve.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointDoc {
    pub version: u32,
    pub domains: Vec<PatternSet>,
    pub model_fingerprint: u64,
    pub seed: u64,
}

impl CheckpointDoc {
    pub fn from_checkpoint(checkpoint: &Checkpoint) -> Self {
        Self { version: CHECKPOINT_VERSION, domains: checkpoint.domains.clone(), model_fingerprint: checkpoint.model_fingerprint, seed: checkpoint.seed }
    }

    /// 💾 Revalidates every structural invariant a deserialized checkpoint might violate, then
    /// converts into a usable [`Checkpoint`]. `node_count` and `model` should come from the live
    /// topology/model this checkpoint is about to resume against.
    pub fn into_checkpoint(self, model: &CompiledModel, node_count: usize) -> Result<Checkpoint, SolveError> {
        if self.version != CHECKPOINT_VERSION {
            return Err(SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: self.version });
        }
        if self.model_fingerprint != model.fingerprint() {
            return Err(SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
        }
        if self.domains.len() != node_count {
            return Err(SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
        }
        for d in &self.domains {
            if !d.is_well_formed() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
            }
            if d.len() != model.pattern_count() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
            }
        }
        Ok(Checkpoint::new(self.domains, self.model_fingerprint, self.seed))
    }
}
// #endregion 🔖Checkpoint

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard() -> (CompiledModel, crate::topology::GraphTopology) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(2.0);
        b.add_tag(black, "dark");
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(3);
        tb.arc(crate::ids::NodeId(0), crate::ids::NodeId(1), adj);
        tb.arc(crate::ids::NodeId(1), crate::ids::NodeId(0), adj);
        tb.arc(crate::ids::NodeId(1), crate::ids::NodeId(2), adj);
        tb.arc(crate::ids::NodeId(2), crate::ids::NodeId(1), adj);
        (model, tb.build().unwrap())
    }

    #[test]
    fn from_model_compile_round_trip_preserves_fingerprint() {
        let (model, _topo) = checkerboard();
        let doc = SourceModelDoc::from_model(&model);
        let recompiled = doc.compile().unwrap();
        assert_eq!(recompiled.fingerprint(), model.fingerprint());
        assert_eq!(recompiled.pattern_count(), model.pattern_count());
    }

    #[test]
    fn source_model_doc_json_round_trips() {
        let (model, _topo) = checkerboard();
        let doc = SourceModelDoc::from_model(&model);
        let json = serde_json::to_string(&doc).unwrap();
        let back: SourceModelDoc = serde_json::from_str(&json).unwrap();
        assert_eq!(back, doc);
        assert_eq!(back.compile().unwrap().fingerprint(), model.fingerprint());
    }

    #[test]
    fn compile_rejects_unknown_schema_version() {
        let (model, _topo) = checkerboard();
        let mut doc = SourceModelDoc::from_model(&model);
        doc.version = 999;
        assert_eq!(doc.compile().unwrap_err(), ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: 999 });
    }

    #[test]
    fn hand_authored_asymmetric_allow_fails_validate_on_compile() {
        // A hand-edited document declares `adj` self-inverse but only allows black->white, never
        // the reverse: `compile()` must still run `validate()` and reject it, not just build
        // silently-broken bitset tables.
        let doc = SourceModelDoc {
            version: SOURCE_MODEL_VERSION,
            patterns: vec![PatternDoc { weight: 1.0, tags: vec![] }, PatternDoc { weight: 1.0, tags: vec![] }],
            relations: vec![RelationDoc { name: "adj".to_string(), inverse: None }],
            allow: vec![PairDoc { relation: 0, src: 0, dst: 1 }],
            deny: vec![],
        };
        assert!(matches!(doc.compile().unwrap_err(), ModelError::AsymmetricInverse { .. }));
    }

    #[test]
    fn checkpoint_doc_round_trips_and_resumes() {
        let (model, topo) = checkerboard();
        let fingerprint = model.fingerprint();
        let mut domains = vec![model.full_domain(); topo.node_count()];
        let mut pinned = PatternSet::new_empty(model.pattern_count());
        pinned.set(PatternId(0), true);
        domains[0] = pinned;
        let checkpoint = Checkpoint::new(domains, fingerprint, 5);

        let doc = CheckpointDoc::from_checkpoint(&checkpoint);
        let json = serde_json::to_string(&doc).unwrap();
        let back: CheckpointDoc = serde_json::from_str(&json).unwrap();
        let restored = back.into_checkpoint(&model, topo.node_count()).unwrap();
        assert_eq!(restored.model_fingerprint, fingerprint);
        assert_eq!(restored.seed, 5);
        assert!(restored.domains[0].get(PatternId(0)));
    }

    #[test]
    fn checkpoint_doc_rejects_version_mismatch() {
        let (model, topo) = checkerboard();
        let mut doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: model.fingerprint(), seed: 0 };
        doc.version = 7;
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: 7 });
    }

    #[test]
    fn checkpoint_doc_rejects_fingerprint_mismatch() {
        let (model, topo) = checkerboard();
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: 0xDEAD_BEEF, seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
    }

    #[test]
    fn checkpoint_doc_rejects_wrong_domain_count() {
        let (model, topo) = checkerboard();
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count() - 1], model_fingerprint: model.fingerprint(), seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
    }

    #[test]
    fn checkpoint_doc_rejects_wrong_bitset_length() {
        let (model, topo) = checkerboard();
        let mut domains = vec![model.full_domain(); topo.node_count()];
        domains[0] = PatternSet::new_full(model.pattern_count() + 1);
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains, model_fingerprint: model.fingerprint(), seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
    }

    #[test]
    fn checkpoint_doc_rejects_tampered_bitset_from_raw_json() {
        // Simulates a hand-edited file: valid JSON shape, but a bitset with a stray bit set past
        // its declared `len` in the `words` array — must be caught by `is_well_formed`, not panic.
        let (model, topo) = checkerboard();
        let json = format!(
            r#"{{"version":1,"domains":[{{"words":[999999],"len":2}}{}],"model_fingerprint":{},"seed":0}}"#,
            ",{\"words\":[3],\"len\":2}".repeat(topo.node_count() - 1),
            model.fingerprint()
        );
        let doc: CheckpointDoc = serde_json::from_str(&json).unwrap();
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Serial


// #region 🔖Exports
pub use bitset::PatternSet;
pub use constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
pub use constraints_card::{CardinalityConstraint, Scope};
pub use constraints_conn::{ConnectivityConstraint, ReachabilityConstraint};
pub use diag::{DiagLevel, Event, EventSink, Metrics, TraceReplay};
pub use domain::{Domain, DomainStore, RestrictResult};
pub use error::{ConstraintError, ModelError, SolveError, TopologyError};
pub use extract::{Extract2dConfig, ExtractedModel2d, PatternDecoder2d, Sample2d, extract_2d};
pub use grid2d::{Boundary, Grid2dTopology, Stencil2d, declare_stencil_relations, declare_stencil_relations_tiled};
pub use grid3d::{Grid3dTopology, Stencil3d, declare_stencil_relations_3d, declare_stencil_relations_3d_tiled};
pub use heuristics::ObserveHeuristic;
pub use ids::{ConstraintId, DecisionId, NodeId, PatternId, PortId, RegionId, RelationId, TileId};
pub use model::{CompiledModel, LintFinding, ModelBuilder, ModelStats, PatternInfo, RelationInfo};
pub use oracle::{ArcSpec, OracleResult, Violation, check_assignment, enumerate};
pub use outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
pub use sample::ValueSampler;
pub use search::{Budget, CancelToken, RestartSchedule, SearchConfig, SearchMode};
pub use serial::{CHECKPOINT_VERSION, CheckpointDoc, PairDoc, PatternDoc, RelationDoc, SOURCE_MODEL_VERSION, SourceModelDoc};
pub use soft::{Attempt, BestOfNKeep, ScoreFn, SoftConstraint, WeightField, best_of_n};
pub use solver_grid2d::{Grid2dSolver, Grid2dSolverBuilder};
pub use solver_grid3d::{Grid3dSolver, Grid3dSolverBuilder};
pub use solver_graph::{GraphSolver, GraphSolverBuilder};
pub use symmetry::{SymmetryGroup2d, SymmetryGroup3d, Transform2d, Transform3d, cube_rotations_24, cube_symmetries_48};
pub use tiled::TiledModelBuilder;
pub use topology::{GraphTopology, GraphTopologyBuilder, from_graph_view};
pub use trail::Checkpoint;
pub use weights::{WeightMode, WeightTable, ZeroWeightPolicy};
// #endregion 🔖Exports
