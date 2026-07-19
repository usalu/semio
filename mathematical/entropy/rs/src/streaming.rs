//! 🌊 Online/streaming entropy estimation: a mergeable `StreamingEstimator` trait plus exact
//! incremental counts, a fixed sliding window, and exponentially decayed counts.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};
use std::collections::VecDeque;

// #region 🔖Trait
/// 🌊 Mergeable online state: `update` folds one observation in, `remove` undoes one (where
/// supported), `merge` combines two independently accumulated states, `estimate` reports the
/// current entropy, `reset` clears all state, and `snapshot`/`restore` round-trip the state
/// through a plain-data representation (no serde — see [`StreamingSnapshot`]).
pub trait StreamingEstimator {
    type Item;
    fn update(&mut self, x: Self::Item);
    fn remove(&mut self, x: Self::Item) -> Result<(), EntropyError>;
    fn merge(&mut self, other: &Self) -> Result<(), EntropyError>;
    fn estimate(&self) -> Result<Estimate, EntropyError>;
    fn reset(&mut self);
    fn snapshot(&self) -> StreamingSnapshot;
    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError>
    where
        Self: Sized;
}

/// 🌊 Plain-data snapshot of a streaming estimator's internal counts, used for `snapshot`/
/// `restore` round-tripping without depending on an external serialization crate.
#[derive(Clone, PartialEq, Debug)]
pub struct StreamingSnapshot {
    pub counts: Vec<f64>,
    pub alphabet_size: usize,
    pub base: LogBase,
    pub method: &'static str,
    pub extra: Vec<f64>,
}
// #endregion 🔖Trait

// #region 🔖Shared
fn plugin_entropy_from_counts(counts: &[f64], base: LogBase, method: &'static str, n_raw: usize) -> Estimate {
    let total: f64 = counts.iter().sum();
    let nats = if total > 0.0 { -counts.iter().map(|&c| x_ln_x(c / total)).sum::<f64>() } else { 0.0 };
    let mut warnings = Vec::new();
    let occupied = counts.iter().filter(|&&c| c > 0.0).count();
    if occupied * 2 < counts.len() {
        warnings.push(Warning::Undersampled { occupied_bins: occupied, total_bins: counts.len() });
    }
    Estimate {
        value: base.from_nats(nats),
        base,
        method,
        n: n_raw,
        n_effective: total,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("alphabet_size", counts.len() as f64), ("total_weight", total)],
    }
}
// #endregion 🔖Shared

// #region 🔖StreamingCounts
/// 🌊 Exact incremental symbol counts over a fixed `0..alphabet_size` alphabet.
pub struct StreamingCounts {
    counts: Vec<f64>,
    base: LogBase,
    n_raw: usize,
}

impl StreamingCounts {
    pub fn new(alphabet_size: usize, base: LogBase) -> Self {
        Self { counts: vec![0.0; alphabet_size], base, n_raw: 0 }
    }
}

impl StreamingEstimator for StreamingCounts {
    type Item = u32;

    fn update(&mut self, x: u32) {
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
            self.n_raw += 1;
        }
    }

    fn remove(&mut self, x: u32) -> Result<(), EntropyError> {
        let idx = x as usize;
        if idx >= self.counts.len() || self.counts[idx] < 1.0 {
            return Err(EntropyError::InvalidConfig { field: "x", reason: "no observation of this symbol to remove" });
        }
        self.counts[idx] -= 1.0;
        self.n_raw -= 1;
        Ok(())
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        self.n_raw += other.n_raw;
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        if self.n_raw == 0 {
            return Err(EntropyError::EmptyInput { what: "streaming counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "streaming_counts", self.n_raw))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
        self.n_raw = 0;
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "streaming_counts", extra: vec![self.n_raw as f64] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        Ok(Self { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra.first().copied().unwrap_or(0.0) as usize })
    }
}
// #endregion 🔖StreamingCounts

// #region 🔖SlidingWindow
/// 🌊 Entropy over the most recent `capacity` observations only (older ones are evicted exactly,
/// via [`StreamingCounts::remove`]).
pub struct SlidingWindowEntropy {
    window: VecDeque<u32>,
    capacity: usize,
    counts: StreamingCounts,
}

impl SlidingWindowEntropy {
    pub fn new(alphabet_size: usize, capacity: usize, base: LogBase) -> Result<Self, EntropyError> {
        if capacity == 0 {
            return Err(EntropyError::InvalidConfig { field: "capacity", reason: "must be at least 1" });
        }
        Ok(Self { window: VecDeque::with_capacity(capacity), capacity, counts: StreamingCounts::new(alphabet_size, base) })
    }
}

impl StreamingEstimator for SlidingWindowEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        self.counts.update(x);
        self.window.push_back(x);
        if self.window.len() > self.capacity {
            if let Some(evicted) = self.window.pop_front() {
                let _ = self.counts.remove(evicted);
            }
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "SlidingWindowEntropy evicts automatically; explicit remove is unsupported" })
    }

    fn merge(&mut self, _other: &Self) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "merge", reason: "SlidingWindowEntropy carries order-dependent state and cannot be merged" })
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        self.counts.estimate()
    }

    fn reset(&mut self) {
        self.window.clear();
        self.counts.reset();
    }

    fn snapshot(&self) -> StreamingSnapshot {
        let mut snap = self.counts.snapshot();
        snap.method = "sliding_window_entropy";
        snap.extra.push(self.capacity as f64);
        for &v in &self.window {
            snap.extra.push(v as f64);
        }
        snap
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        if snapshot.extra.len() < 2 {
            return Err(EntropyError::InvalidConfig { field: "snapshot", reason: "missing capacity/window data" });
        }
        let capacity = snapshot.extra[1] as usize;
        let window: VecDeque<u32> = snapshot.extra[2..].iter().map(|&v| v as u32).collect();
        let counts = StreamingCounts { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra[0] as usize };
        Ok(Self { window, capacity, counts })
    }
}
// #endregion 🔖SlidingWindow

// #region 🔖Decayed
/// 🌊 Exponentially forgetting counts: each `update` first multiplies every count by `decay` (in
/// `(0, 1]`) before incrementing the observed symbol, so older observations fade geometrically.
/// `remove` is semantically unsupported (there is no well-defined inverse of decay) and always
/// errors.
pub struct DecayedEntropy {
    counts: Vec<f64>,
    decay: f64,
    base: LogBase,
}

impl DecayedEntropy {
    pub fn new(alphabet_size: usize, decay: f64, base: LogBase) -> Result<Self, EntropyError> {
        if !(0.0 < decay && decay <= 1.0) {
            return Err(EntropyError::InvalidConfig { field: "decay", reason: "must be in (0, 1]" });
        }
        Ok(Self { counts: vec![0.0; alphabet_size], decay, base })
    }
}

impl StreamingEstimator for DecayedEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        for c in &mut self.counts {
            *c *= self.decay;
        }
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "DecayedEntropy has no well-defined inverse of exponential decay" })
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        let total: f64 = self.counts.iter().sum();
        if total <= 0.0 {
            return Err(EntropyError::EmptyInput { what: "decayed counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "decayed_entropy", self.counts.len()))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "decayed_entropy", extra: vec![self.decay] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        let decay = snapshot.extra.first().copied().unwrap_or(1.0);
        Ok(Self { counts: snapshot.counts.clone(), decay, base: snapshot.base })
    }
}
// #endregion 🔖Decayed

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_counts_update_matches_batch_entropy() {
        let mut sc = StreamingCounts::new(4, LogBase::Bits);
        for &x in &[0u32, 1, 1, 2, 2, 2, 3] {
            sc.update(x);
        }
        let est = sc.estimate().unwrap();
        let counts = crate::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 2, 3], 4).unwrap();
        let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Bits).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_remove_undoes_update() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        sc.update(1);
        sc.remove(0).unwrap();
        assert_eq!(sc.n_raw, 1);
    }

    #[test]
    fn streaming_counts_remove_rejects_unobserved_symbol() {
        let mut sc = StreamingCounts::new(3, LogBase::Nats);
        sc.update(0);
        assert!(sc.remove(1).is_err());
    }

    #[test]
    fn streaming_counts_merge_matches_combined_batch() {
        let mut a = StreamingCounts::new(3, LogBase::Nats);
        let mut b = StreamingCounts::new(3, LogBase::Nats);
        for &x in &[0u32, 1, 1] {
            a.update(x);
        }
        for &x in &[2u32, 2, 0] {
            b.update(x);
        }
        a.merge(&b).unwrap();
        let est = a.estimate().unwrap();
        let counts = crate::counts::Counts::from_symbols(&[0, 1, 1, 2, 2, 0], 3).unwrap();
        let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn streaming_counts_snapshot_restore_roundtrips() {
        let mut sc = StreamingCounts::new(3, LogBase::Bits);
        for &x in &[0u32, 1, 2, 2] {
            sc.update(x);
        }
        let snap = sc.snapshot();
        let restored = StreamingCounts::restore(&snap).unwrap();
        assert_eq!(sc.estimate().unwrap().value, restored.estimate().unwrap().value);
    }

    #[test]
    fn sliding_window_matches_batch_recomputed_at_every_step() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let capacity = 20;
        let mut sw = SlidingWindowEntropy::new(4, capacity, LogBase::Nats).unwrap();
        let mut history: Vec<u32> = Vec::new();
        for _ in 0..200 {
            let x = rng.next_below(4) as u32;
            sw.update(x);
            history.push(x);
            let window_start = history.len().saturating_sub(capacity);
            let window = &history[window_start..];
            let counts = crate::counts::Counts::from_symbols(window, 4).unwrap();
            let expected = crate::discrete::entropy(&counts.probabilities(), LogBase::Nats).unwrap();
            let got = sw.estimate().unwrap().value;
            assert!((got - expected).abs() < 1e-9, "mismatch at len {}", history.len());
        }
    }

    #[test]
    fn sliding_window_remove_and_merge_are_unsupported() {
        let mut sw = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        sw.update(0);
        assert!(sw.remove(0).is_err());
        let other = SlidingWindowEntropy::new(3, 5, LogBase::Nats).unwrap();
        assert!(sw.merge(&other).is_err());
    }

    #[test]
    fn decayed_entropy_rejects_bad_decay() {
        assert!(DecayedEntropy::new(3, 0.0, LogBase::Nats).is_err());
        assert!(DecayedEntropy::new(3, 1.5, LogBase::Nats).is_err());
    }

    #[test]
    fn decayed_entropy_remove_is_unsupported() {
        let mut de = DecayedEntropy::new(3, 0.9, LogBase::Nats).unwrap();
        de.update(0);
        assert!(de.remove(0).is_err());
    }

    #[test]
    fn decayed_entropy_forgets_old_symbols() {
        let mut de = DecayedEntropy::new(2, 0.5, LogBase::Bits).unwrap();
        for _ in 0..50 {
            de.update(0);
        }
        // 🔐 after many decayed updates of the same symbol, entropy should be near zero
        // (essentially deterministic), then adding a burst of the other symbol should raise it.
        let before = de.estimate().unwrap().value;
        assert!(before < 0.1, "got {before}");
        for _ in 0..50 {
            de.update(1);
        }
        let after = de.estimate().unwrap().value;
        assert!(after < 0.5, "got {after}"); // 🔐 decay erased symbol-0 history; now near-deterministic on symbol 1
    }

    #[test]
    fn decayed_entropy_snapshot_restore_roundtrips() {
        let mut de = DecayedEntropy::new(3, 0.8, LogBase::Nats).unwrap();
        de.update(0);
        de.update(1);
        let snap = de.snapshot();
        let restored = DecayedEntropy::restore(&snap).unwrap();
        assert!((de.estimate().unwrap().value - restored.estimate().unwrap().value).abs() < 1e-12);
    }
}
// #endregion 🔖Tests
