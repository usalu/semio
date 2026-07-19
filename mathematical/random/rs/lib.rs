//! 🎲 Seeded, reproducible pseudo-random generation: xoshiro256** core, distributions, and sequence samplers shared by every randomized graph algorithm.

// #region 🔖SplitMix64
/// 🌱 SplitMix64 seed-mixing step (Vigna's `splitmix64`): turns a `u64` seed into a well-mixed
/// stream, used only to derive [`Rng`]'s initial state words — never as the generator itself.
pub struct SplitMix64(u64);

impl SplitMix64 {
    /// 🌱 Starts a mixing stream from a raw seed.
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// 🌱 Advances the stream and returns the next mixed 64-bit word.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
// #endregion 🔖SplitMix64

// #region 🔖Rng
#[inline]
fn rotl(x: u64, k: u32) -> u64 {
    x.rotate_left(k)
}

/// 🎲 xoshiro256** (Blackman & Vigna, public domain), a 256-bit-state generator with a 2^256-1
/// period and excellent statistical quality; chosen over xorshift128+ for its stronger equidistribution
/// and larger period margin, both useful once nested graph algorithms draw many correlated sub-sequences
/// from independently-seeded generators. All arithmetic is `u64` wrapping — no floats in the core step —
/// so a given seed produces the exact same bit sequence on every platform.
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// 🌱 Seeds all four state words via [`SplitMix64`] so even adjacent seeds decorrelate immediately.
    pub fn from_seed(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        let mut s = [0u64; 4];
        for slot in s.iter_mut() {
            *slot = sm.next_u64();
        }
        Self { s }
    }

    /// 🎲 Next raw 64-bit word (the xoshiro256** `scramble` output), advancing the state.
    pub fn next_u64(&mut self) -> u64 {
        let result = rotl(self.s[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = rotl(self.s[3], 45);
        result
    }

    /// 🎯 Uniform `f64` in `[0, 1)`, built from the top 53 bits of a raw draw (the mantissa width of `f64`).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎯 Uniform `u64` in `[lo, hi)`. Rejects draws that fall in the trailing partial bucket of
    /// `u64::MAX / range` instead of using `% range` directly: naive modulo keeps that partial bucket,
    /// which over-weights the low end of the range by a hair — rejection sampling discards it so every
    /// remaining value maps to exactly the same number of raw draws.
    pub fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64();
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🪙 `true` with probability `p` (clamped semantics: `p <= 0.0` never fires, `p >= 1.0` always fires).
    pub fn next_bool(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// 🔀 In-place Fisher-Yates shuffle: uniform over all `n!` permutations.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        let n = items.len();
        for i in (1..n).rev() {
            let j = self.next_range(0, (i + 1) as u64) as usize;
            items.swap(i, j);
        }
    }

    /// 🎯 Picks one uniformly random element, or `None` for an empty slice.
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let idx = self.next_range(0, items.len() as u64) as usize;
        items.get(idx)
    }

    /// 🎲 Raw 256-bit state words, for external snapshot/restore of a generator mid-stream.
    pub fn state(&self) -> [u64; 4] {
        self.s
    }

    /// 🌱 Rebuilds a generator from previously captured state words (inverse of [`Rng::state`]).
    /// The all-zero state is xoshiro256**'s fixed point (every subsequent draw is also zero), so
    /// callers must never pass it; debug builds catch the mistake immediately.
    pub fn from_state(s: [u64; 4]) -> Self {
        debug_assert!(s != [0u64; 4], "from_state: all-zero state is the xoshiro256** fixed point");
        Self { s }
    }

    /// 🎯 `k` distinct indices drawn uniformly from `0..n`, via Floyd's O(k)-time, O(k)-space partial
    /// sampling algorithm — it never materializes the full `0..n` universe, which matters once `n` is a
    /// graph's node count and `k` is a small subsample. Order is not itself a uniform permutation.
    pub fn sample_without_replacement(&mut self, n: usize, k: usize) -> Vec<usize> {
        assert!(k <= n, "sample_without_replacement: k must not exceed n");
        let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::with_capacity(k);
        let mut result = Vec::with_capacity(k);
        for j in (n - k)..n {
            let t = self.next_range(0, (j + 1) as u64) as usize;
            let picked = if selected.contains(&t) { j } else { t };
            selected.insert(picked);
            result.push(picked);
        }
        result
    }
}
// #endregion 🔖Rng

// #region 🔖AliasTable
/// ⚖️ Walker's alias method: O(n) setup, O(1) per draw, for sampling an index `0..weights.len()`
/// proportional to arbitrary non-negative weights. Needed by degree-sequence-weighted random graph
/// generators (configuration model, Chung-Lu) where millions of draws share one weight vector.
pub struct AliasTable {
    prob: Vec<f64>,
    alias: Vec<usize>,
}

impl AliasTable {
    /// ⚖️ Builds the table, normalizing `weights` internally. An empty slice or all-zero weights are
    /// degenerate (no valid probability distribution exists), so both are defined to always sample
    /// index `0` rather than panic — callers that pass a plain degree/weight vector don't need to
    /// special-case the all-zero graph before building a table.
    pub fn new(weights: &[f64]) -> Self {
        let n = weights.len();
        if n == 0 {
            return Self { prob: vec![1.0], alias: vec![0] };
        }
        let sum: f64 = weights.iter().sum();
        if sum <= 0.0 {
            let mut prob = vec![0.0; n];
            prob[0] = 1.0;
            return Self { prob, alias: vec![0; n] };
        }
        let mut scaled: Vec<f64> = weights.iter().map(|w| w / sum * n as f64).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        for (i, &p) in scaled.iter().enumerate() {
            if p < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut prob = vec![0.0; n];
        let mut alias = vec![0usize; n];
        // `(small.pop(), large.pop())` would evaluate both pops unconditionally even when one list is
        // already empty, silently dropping an element from the non-empty side — check lengths first.
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().expect("small is non-empty per the loop condition");
            let l = large.pop().expect("large is non-empty per the loop condition");
            prob[s] = scaled[s];
            alias[s] = l;
            scaled[l] = scaled[l] + scaled[s] - 1.0;
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        for l in large {
            prob[l] = 1.0;
        }
        for s in small {
            prob[s] = 1.0;
        }
        Self { prob, alias }
    }

    /// ⚖️ Draws one index in O(1): pick a bucket uniformly, then coin-flip between its own item and its alias.
    pub fn sample(&self, rng: &mut Rng) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64) as usize;
        if rng.next_f64() < self.prob[i] {
            i
        } else {
            self.alias[i]
        }
    }
}
// #endregion 🔖AliasTable

// #region 🔖Distributions
/// 🔔 Standard normal via the Box-Muller transform, scaled to `(mean, std_dev)`. Draws `u1` from
/// `(0, 1]` (not `[0, 1)`) so `ln(u1)` never sees an exact zero.
pub fn normal(rng: &mut Rng, mean: f64, std_dev: f64) -> f64 {
    let u1 = 1.0 - rng.next_f64();
    let u2 = rng.next_f64();
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + std_dev * z0
}

/// 🎲 Number of Bernoulli(`p`) trials up to and including the first success (support `1, 2, 3, ...`),
/// sampled by inverse-CDF transform.
pub fn geometric(rng: &mut Rng, p: f64) -> u64 {
    debug_assert!(p > 0.0 && p <= 1.0, "geometric: p must be in (0, 1]");
    if p >= 1.0 {
        return 1;
    }
    let u = 1.0 - rng.next_f64();
    (u.ln() / (1.0 - p).ln()).floor() as u64 + 1
}

/// 🎲 Poisson(`lambda`) via Knuth's product-of-uniforms algorithm: O(lambda) draws per sample, so it
/// stays fast for moderate `lambda` (roughly up to a few dozen) but degrades for very large `lambda`,
/// where a transformed-rejection method would be preferable.
pub fn poisson(rng: &mut Rng, lambda: f64) -> u64 {
    debug_assert!(lambda >= 0.0, "poisson: lambda must be non-negative");
    let l = (-lambda).exp();
    let mut k = 0u64;
    let mut p = 1.0;
    loop {
        k += 1;
        p *= rng.next_f64();
        if p <= l {
            break;
        }
    }
    k - 1
}

/// 📐 `n` samples from a Pareto-like power-law distribution with the given `exponent`, matching
/// NetworkX `utils.powerlaw_sequence`'s `random() ** (-1 / (exponent - 1))` inverse-transform.
pub fn powerlaw_sequence(rng: &mut Rng, n: usize, exponent: f64) -> Vec<f64> {
    debug_assert!(exponent != 1.0, "powerlaw_sequence: exponent must not be 1.0");
    (0..n)
        .map(|_| {
            let u = 1.0 - rng.next_f64();
            u.powf(-1.0 / (exponent - 1.0))
        })
        .collect()
}

/// 📊 Samples a rank in `1..=n` from a Zipf distribution via rejection sampling: draws from the
/// unbounded-support Zipf algorithm (Devroye) and re-rejects any draw landing outside `1..=n`, so no
/// O(n) setup is needed even when `n` is huge. Requires `exponent > 1.0`.
pub fn zipf(rng: &mut Rng, n: usize, exponent: f64) -> u64 {
    debug_assert!(exponent > 1.0, "zipf: exponent must be > 1.0");
    debug_assert!(n >= 1, "zipf: n must be >= 1");
    let am1 = exponent - 1.0;
    let b = 2f64.powf(am1);
    loop {
        let u = 1.0 - rng.next_f64();
        let v = rng.next_f64();
        let x = u.powf(-1.0 / am1).floor();
        if x < 1.0 {
            continue;
        }
        let t = (1.0 + 1.0 / x).powf(am1);
        if v * x * (t - 1.0) / (b - 1.0) <= t / b && x <= n as f64 {
            return x as u64;
        }
    }
}

/// 🎯 `n` draws from a discrete distribution given as a weight vector, matching NetworkX
/// `utils.discrete_sequence`; builds one [`AliasTable`] and draws from it `n` times.
pub fn discrete_sequence(rng: &mut Rng, n: usize, distribution: &[f64]) -> Vec<usize> {
    let table = AliasTable::new(distribution);
    (0..n).map(|_| table.sample(rng)).collect()
}

/// ➕ Running sum of `weights`, normalized so the last entry is `1.0` (a no-op on an all-zero or
/// empty input, returned unnormalized since there is no meaningful scale to normalize to).
pub fn cumulative_distribution(weights: &[f64]) -> Vec<f64> {
    let mut cumulative = Vec::with_capacity(weights.len());
    let mut running = 0.0;
    for &w in weights {
        running += w;
        cumulative.push(running);
    }
    if let Some(&last) = cumulative.last() {
        if last > 0.0 {
            for v in cumulative.iter_mut() {
                *v /= last;
            }
        }
    }
    cumulative
}
// #endregion 🔖Distributions

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖SplitMix64Tests
    #[test]
    fn split_mix64_is_deterministic_for_same_seed() {
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);
        for _ in 0..16 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn split_mix64_differs_across_seeds() {
        let mut a = SplitMix64::new(1);
        let mut b = SplitMix64::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }
    // #endregion 🔖SplitMix64Tests

    // #region 🔖RngDeterminismTests
    #[test]
    fn rng_next_u64_is_deterministic_for_same_seed() {
        let mut a = Rng::from_seed(1234);
        let mut b = Rng::from_seed(1234);
        let seq_a: Vec<u64> = (0..64).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..64).map(|_| b.next_u64()).collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn rng_next_f64_is_deterministic_for_same_seed() {
        let mut a = Rng::from_seed(9876);
        let mut b = Rng::from_seed(9876);
        let seq_a: Vec<f64> = (0..64).map(|_| a.next_f64()).collect();
        let seq_b: Vec<f64> = (0..64).map(|_| b.next_f64()).collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn rng_state_round_trip_resumes_identically() {
        let mut original = Rng::from_seed(4242);
        for _ in 0..17 {
            original.next_u64();
        }
        let snapshot = original.state();
        let mut resumed = Rng::from_state(snapshot);
        let expected: Vec<u64> = (0..32).map(|_| original.next_u64()).collect();
        let actual: Vec<u64> = (0..32).map(|_| resumed.next_u64()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn rng_different_seeds_diverge() {
        let mut a = Rng::from_seed(1);
        let mut b = Rng::from_seed(2);
        let seq_a: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..8).map(|_| b.next_u64()).collect();
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn rng_has_no_obvious_short_cycle() {
        for seed in [0u64, 1, 42, u64::MAX, 0xDEAD_BEEF] {
            let mut rng = Rng::from_seed(seed);
            let draws: Vec<u64> = (0..2000).map(|_| rng.next_u64()).collect();
            let unique: std::collections::HashSet<u64> = draws.iter().copied().collect();
            assert!(unique.len() > 1990, "seed {seed} produced too many repeats: {} unique of 2000", unique.len());
        }
    }
    // #endregion 🔖RngDeterminismTests

    // #region 🔖RngStatisticalTests
    #[test]
    fn next_f64_stays_within_unit_interval() {
        let mut rng = Rng::from_seed(7);
        for _ in 0..10_000 {
            let x = rng.next_f64();
            assert!((0.0..1.0).contains(&x));
        }
    }

    #[test]
    fn next_range_stays_within_bounds_and_is_roughly_uniform() {
        let mut rng = Rng::from_seed(2024);
        let buckets = 10;
        let mut counts = vec![0u64; buckets];
        let draws = 100_000;
        for _ in 0..draws {
            let x = rng.next_range(0, buckets as u64);
            assert!(x < buckets as u64);
            counts[x as usize] += 1;
        }
        let expected = draws as f64 / buckets as f64;
        for count in counts {
            let ratio = count as f64 / expected;
            assert!((0.85..1.15).contains(&ratio), "bucket count {count} too far from expected {expected}");
        }
    }

    #[test]
    fn next_range_degenerate_empty_range_returns_lo() {
        let mut rng = Rng::from_seed(5);
        assert_eq!(rng.next_range(3, 3), 3);
    }

    #[test]
    fn next_bool_respects_extremes() {
        let mut rng = Rng::from_seed(11);
        for _ in 0..100 {
            assert!(!rng.next_bool(0.0));
        }
        for _ in 0..100 {
            assert!(rng.next_bool(1.0));
        }
    }

    #[test]
    fn shuffle_preserves_multiset() {
        let mut rng = Rng::from_seed(99);
        let original: Vec<i32> = (0..50).collect();
        let mut shuffled = original.clone();
        rng.shuffle(&mut shuffled);
        let mut sorted_shuffled = shuffled.clone();
        sorted_shuffled.sort_unstable();
        assert_eq!(sorted_shuffled, original);
        assert_ne!(shuffled, original, "a 50-element shuffle landing on the identity permutation is astronomically unlikely");
    }

    #[test]
    fn choose_returns_none_for_empty_slice() {
        let mut rng = Rng::from_seed(3);
        let empty: Vec<i32> = Vec::new();
        assert_eq!(rng.choose(&empty), None);
    }

    #[test]
    fn choose_always_returns_an_element_from_the_slice() {
        let mut rng = Rng::from_seed(3);
        let items = [10, 20, 30, 40];
        for _ in 0..50 {
            let picked = rng.choose(&items).expect("non-empty slice");
            assert!(items.contains(picked));
        }
    }

    #[test]
    fn sample_without_replacement_returns_k_distinct_indices_in_range() {
        let mut rng = Rng::from_seed(456);
        for (n, k) in [(10, 3), (100, 100), (1000, 1), (50, 0)] {
            let sample = rng.sample_without_replacement(n, k);
            assert_eq!(sample.len(), k);
            let unique: std::collections::HashSet<usize> = sample.iter().copied().collect();
            assert_eq!(unique.len(), k);
            assert!(sample.iter().all(|&i| i < n));
        }
    }
    // #endregion 🔖RngStatisticalTests

    // #region 🔖AliasTableTests
    #[test]
    fn alias_table_sampling_frequency_matches_weights() {
        let weights = [1.0, 2.0, 3.0, 4.0];
        let table = AliasTable::new(&weights);
        let mut rng = Rng::from_seed(321);
        let draws = 200_000;
        let mut counts = [0u64; 4];
        for _ in 0..draws {
            counts[table.sample(&mut rng)] += 1;
        }
        let total: f64 = weights.iter().sum();
        for (i, &w) in weights.iter().enumerate() {
            let expected = draws as f64 * w / total;
            let ratio = counts[i] as f64 / expected;
            assert!((0.9..1.1).contains(&ratio), "index {i} count {} too far from expected {expected}", counts[i]);
        }
    }

    #[test]
    fn alias_table_empty_weights_always_samples_index_zero() {
        let table = AliasTable::new(&[]);
        let mut rng = Rng::from_seed(1);
        for _ in 0..20 {
            assert_eq!(table.sample(&mut rng), 0);
        }
    }

    #[test]
    fn alias_table_all_zero_weights_always_samples_index_zero() {
        let table = AliasTable::new(&[0.0, 0.0, 0.0]);
        let mut rng = Rng::from_seed(2);
        for _ in 0..20 {
            assert_eq!(table.sample(&mut rng), 0);
        }
    }

    #[test]
    fn alias_table_single_weight_always_samples_it() {
        let table = AliasTable::new(&[5.0]);
        let mut rng = Rng::from_seed(3);
        for _ in 0..20 {
            assert_eq!(table.sample(&mut rng), 0);
        }
    }
    // #endregion 🔖AliasTableTests

    // #region 🔖DistributionTests
    #[test]
    fn normal_stays_within_six_std_devs_of_mean() {
        let mut rng = Rng::from_seed(55);
        let mean = 10.0;
        let std_dev = 2.0;
        for _ in 0..10_000 {
            let x = normal(&mut rng, mean, std_dev);
            assert!((x - mean).abs() < 6.0 * std_dev, "normal draw {x} outside 6-sigma band");
        }
    }

    #[test]
    fn geometric_is_always_at_least_one() {
        let mut rng = Rng::from_seed(66);
        for _ in 0..1000 {
            assert!(geometric(&mut rng, 0.3) >= 1);
        }
    }

    #[test]
    fn geometric_p_one_always_returns_one() {
        let mut rng = Rng::from_seed(67);
        for _ in 0..100 {
            assert_eq!(geometric(&mut rng, 1.0), 1);
        }
    }

    #[test]
    fn poisson_lambda_zero_always_returns_zero() {
        let mut rng = Rng::from_seed(77);
        for _ in 0..100 {
            assert_eq!(poisson(&mut rng, 0.0), 0);
        }
    }

    #[test]
    fn poisson_mean_is_roughly_lambda() {
        let mut rng = Rng::from_seed(78);
        let lambda = 4.0;
        let draws = 20_000;
        let sum: u64 = (0..draws).map(|_| poisson(&mut rng, lambda)).sum();
        let mean = sum as f64 / draws as f64;
        assert!((mean - lambda).abs() < 0.2, "poisson mean {mean} too far from lambda {lambda}");
    }

    #[test]
    fn powerlaw_sequence_values_are_positive_and_at_least_one() {
        let mut rng = Rng::from_seed(88);
        let values = powerlaw_sequence(&mut rng, 500, 2.5);
        assert_eq!(values.len(), 500);
        for v in values {
            assert!(v >= 1.0, "powerlaw value {v} below the theoretical minimum of 1.0");
            assert!(v.is_finite());
        }
    }

    #[test]
    fn zipf_ranks_are_within_bounds() {
        let mut rng = Rng::from_seed(89);
        let n = 100;
        for _ in 0..2000 {
            let rank = zipf(&mut rng, n, 2.0);
            assert!((1..=n as u64).contains(&rank));
        }
    }

    #[test]
    fn zipf_favors_low_ranks() {
        let mut rng = Rng::from_seed(90);
        let n = 20;
        let draws = 20_000;
        let mut low_rank_hits = 0u64;
        for _ in 0..draws {
            if zipf(&mut rng, n, 2.0) <= 2 {
                low_rank_hits += 1;
            }
        }
        assert!(low_rank_hits as f64 / draws as f64 > 0.5, "zipf should heavily favor the lowest ranks");
    }

    #[test]
    fn discrete_sequence_draws_only_from_nonzero_weight_indices() {
        let mut rng = Rng::from_seed(91);
        let distribution = [0.0, 1.0, 0.0, 3.0];
        let draws = discrete_sequence(&mut rng, 200, &distribution);
        assert_eq!(draws.len(), 200);
        for d in draws {
            assert!(d == 1 || d == 3);
        }
    }

    #[test]
    fn cumulative_distribution_ends_at_one_and_is_nondecreasing() {
        let cdf = cumulative_distribution(&[1.0, 2.0, 3.0, 4.0]);
        assert!((cdf.last().unwrap() - 1.0).abs() < 1e-12);
        for pair in cdf.windows(2) {
            assert!(pair[1] >= pair[0]);
        }
    }

    #[test]
    fn cumulative_distribution_of_empty_weights_is_empty() {
        let cdf = cumulative_distribution(&[]);
        assert!(cdf.is_empty());
    }
    // #endregion 🔖DistributionTests
}
// #endregion 🔖Tests
