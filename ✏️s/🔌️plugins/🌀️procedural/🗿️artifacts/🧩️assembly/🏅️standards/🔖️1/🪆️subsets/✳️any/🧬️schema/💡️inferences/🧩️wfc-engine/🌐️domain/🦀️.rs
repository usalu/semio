//! 📦️ A variable's live domain: which patterns remain possible, plus cached weight sums so the
//! weighted-entropy heuristic reads in O(1). Every mutation returns a [`RestrictResult`] so
//! callers (propagation, search) can react to wipeouts/singletons without a second query.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::weights::WeightTable;

// #region 🔖️Result
/// 📦️ What happened to a [`Domain`] after a mutating operation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RestrictResult {
    /// 📦️ No pattern was removed.
    Unchanged,
    /// 📦️ `n` patterns were removed; more than one remains.
    Reduced(u32),
    /// 📦️ Exactly one pattern remains.
    Singleton(PatternId),
    /// 📦️ Zero patterns remain — a contradiction at this variable.
    Wipeout,
}
// #endregion 🔖️Result

// #region 🔖️Domain
/// ⏱️ How many revisions between exact cache resyncs, bounding `f64` subtraction drift without
/// paying `O(domain size)` on every single removal.
const RESYNC_INTERVAL: u64 = 4096;

/// 📦️ One variable's live domain plus incrementally-maintained weight/entropy caches.
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
    /// 📦️ The full domain (every pattern in `w` possible), with caches seeded from `w`'s totals.
    pub fn new_full(w: &WeightTable) -> Self {
        let bits = PatternSet::new_full(w.len());
        let sum_w = (0..w.len()).map(|i| w.w(PatternId::from_index(i))).sum();
        let sum_w_ln_w = (0..w.len()).map(|i| w.w_ln_w(PatternId::from_index(i))).sum();
        let sum_w_int = w.has_integer_weights().then(|| (0..w.len()).filter_map(|i| w.w_int(PatternId::from_index(i))).sum());
        Self { bits, cardinality: w.len() as u32, sum_w, sum_w_ln_w, sum_w_int, revision: 0 }
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
    pub fn is_wiped(&self) -> bool {
        self.cardinality == 0
    }

    /// 📦️ `Some(pattern)` iff this domain is a singleton.
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

    /// 📊️ Incremental weighted Shannon entropy in nats: `ln(Σw) - Σ(w·ln w)/Σw`. `0.0` for an
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

    /// 📦️ Classifies the post-mutation state, given how many patterns this specific operation
    /// removed (only the caller knows that count — `self.cardinality` alone cannot distinguish
    /// "removed 1 of 4" from "removed 3 of 4" when both leave the same remaining count).
    fn result_for(&self, removed_count: u32) -> RestrictResult {
        match self.cardinality {
            0 => RestrictResult::Wipeout,
            1 => RestrictResult::Singleton(self.bits.first_set().expect("cardinality 1 domain must have a set bit")),
            _ => RestrictResult::Reduced(removed_count),
        }
    }

    /// 📦️ Intersects with `allowed`, collecting the removed-pattern mask into `removed_out`
    /// (caller-supplied to avoid a per-call allocation on the propagation hot path).
    pub fn restrict_collecting(&mut self, allowed: &PatternSet, w: &WeightTable, removed_out: &mut PatternSet) -> RestrictResult {
        let cleared = self.bits.restrict_returning_removed(allowed, removed_out);
        if cleared == 0 {
            return RestrictResult::Unchanged;
        }
        self.apply_removed(removed_out, w);
        self.result_for(cleared)
    }

    /// 📦️ Convenience over [`Domain::restrict_collecting`] that allocates its own scratch buffer.
    pub fn restrict(&mut self, allowed: &PatternSet, w: &WeightTable) -> RestrictResult {
        let mut removed = PatternSet::new_empty(self.bits.len());
        self.restrict_collecting(allowed, w, &mut removed)
    }

    /// 📦️ Removes exactly one pattern (a no-op if it is already absent).
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

    /// 📦️ Forces this domain to exactly `{p}` — the WFC "observe" operation. Collects every
    /// removed pattern into `removed_out` (used by the trail and by AC-4's decrement fan-out).
    pub fn assign_collecting(&mut self, p: PatternId, w: &WeightTable, removed_out: &mut PatternSet) -> RestrictResult {
        let mut singleton = PatternSet::new_empty(self.bits.len());
        singleton.set(p, true);
        self.restrict_collecting(&singleton, w, removed_out)
    }

    /// 📦️ Convenience over [`Domain::assign_collecting`] that allocates its own scratch buffer.
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

    /// 🩺️ Debug-only: recomputes every cache from `bits` and asserts it matches the incremental
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
// #endregion 🔖️Domain

// #region 🔖️Store
/// 📦️ One [`Domain`] per solver variable, stored contiguously (struct-of-arrays friendly).
///
/// Algorithm-local scratch during WFC search — caller-owned, not OS document authority (`Vec` only).
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
    pub fn get(&self, n: crate::wfc_engine::ids::NodeId) -> &Domain {
        &self.domains[n.index()]
    }

    #[inline]
    pub fn get_mut(&mut self, n: crate::wfc_engine::ids::NodeId) -> &mut Domain {
        &mut self.domains[n.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (crate::wfc_engine::ids::NodeId, &Domain)> {
        self.domains.iter().enumerate().map(|(i, d)| (crate::wfc_engine::ids::NodeId::from_index(i), d))
    }

    pub fn all_singleton(&self) -> bool {
        self.domains.iter().all(|d| d.cardinality() == 1)
    }

    pub fn any_wiped(&self) -> bool {
        self.domains.iter().any(|d| d.is_wiped())
    }
}
// #endregion 🔖️Store

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
