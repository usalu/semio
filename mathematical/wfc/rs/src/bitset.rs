//! 🎭 Hand-rolled dynamic bitset over `PatternId` — the WFC domain representation. Modeled on
//! `mathematical_sampling::TokenBitset` (word-packed `Vec<u64>`) with a solver-specific fused
//! restrict-and-collect operation used by every propagation engine's hot path.

use crate::ids::PatternId;

// #region 🔖Bitset
/// 🎭 A dynamic word-packed bitset over `0..len` pattern indices. `len` is the size of the
/// universe this set is defined over, not its popcount — use [`PatternSet::count_ones`] /
/// [`PatternSet::is_all_zero`] for cardinality, and [`PatternSet::is_empty_universe`] for the
/// degenerate zero-pattern-universe case.
#[derive(Clone, PartialEq, Debug)]
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
