//! 🎭️ Hand-rolled dynamic bitset over `PatternId` — the WFC domain representation. Modeled on
//! `crate::sampling::TokenBitset` (word-packed `Vec<u64>`) with a solver-specific fused
//! restrict-and-collect operation used by every propagation engine's hot path.

use crate::wfc_engine::ids::PatternId;

// #region 🔖️Bitset
/// 🎭️ A dynamic word-packed bitset over `0..len` pattern indices. `len` is the size of the
/// universe this set is defined over, not its popcount — use [`PatternSet::count_ones`] /
/// [`PatternSet::is_all_zero`] for cardinality. A zero [`PatternSet::len`] identifies an empty universe.
#[derive(Clone, PartialEq, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct PatternSet {
    words: Vec<u64>,
    len: usize,
}

impl PatternSet {
    /// 🎭️ All-zero (empty) set over `len` patterns.
    pub fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭️ All-one (full) set over `len` patterns.
    #[cfg(test)]
    pub fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len);
        set.fill();
        set
    }

    /// 🧱 Starts a domain whose backing words are appended by resumable builders.
    pub(crate) fn with_word_capacity(len: usize) -> Self {
        Self { words: Vec::new(), len }
    }

    /// 🧱 Restores a validated domain from checkpoint words.
    pub(crate) fn from_words(len: usize, words: Vec<u64>) -> Option<Self> {
        let set = Self { words, len };
        set.is_well_formed().then_some(set)
    }

    #[inline]
    pub(crate) fn word_count(&self) -> usize {
        self.len.div_ceil(64)
    }

    #[inline]
    pub(crate) fn word(&self, index: usize) -> u64 {
        self.words[index]
    }

    #[inline]
    pub(crate) fn push_word(&mut self, word: u64) {
        debug_assert!(self.words.len() < self.word_count());
        self.words.push(word);
    }

    /// 🎭️ Number of patterns this set is defined over (not the popcount).
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
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

    /// 🎭️ Sets every bit `0..len` (trailing bits in the final word stay zero).
    #[cfg(test)]
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

    #[cfg(test)]
    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭️ In-place `self &= other`.
    #[cfg(test)]
    pub fn and_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭️ In-place `self |= other`.
    #[cfg(test)]
    pub fn or_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭️ In-place `self &= !other`.
    #[cfg(test)]
    pub fn and_not_with(&mut self, other: &PatternSet) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    #[cfg(test)]
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }

    #[cfg(test)]
    pub fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭️ Lowest set bit, word-skipping past all-zero words.
    pub fn first_set(&self) -> Option<PatternId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(PatternId::from_index(word_idx * 64 + bit));
            }
        }
        None
    }

    /// 🎭️ Iterates set bits in ascending order, skipping whole zero words at a time.
    #[cfg(test)]
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

    /// 🎭️ Read-only access to the backing words, e.g. for stable-hash fingerprinting.
    #[inline]
    #[cfg(test)]
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    #[cfg(test)]
    pub fn is_subset_of(&self, other: &PatternSet) -> bool {
        debug_assert_eq!(self.len, other.len);
        self.words.iter().zip(other.words.iter()).all(|(a, b)| a & !b == 0)
    }

    #[cfg(test)]
    pub fn intersects(&self, other: &PatternSet) -> bool {
        debug_assert_eq!(self.len, other.len);
        self.words.iter().zip(other.words.iter()).any(|(a, b)| a & b != 0)
    }

    /// 🎭️ Structural invariant check for data crossing a trust boundary (deserialization): word
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
    // #endregion 🔖️Bitset

    // #region 🔖️Ops
    /// 🎭️ Fused restrict-and-collect: `removed_out = self & !allowed; self &= allowed`. Returns the
    /// number of bits actually cleared. The single fused pass this crate's hot loop needs — avoids
    /// computing the removed mask and the restricted set in two separate scans.
    #[cfg(test)]
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
    // #endregion 🔖️Ops
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
