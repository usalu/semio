//! 🎼 Symbol-based time-series entropies built on `symbolic.rs`: permutation entropy (Bandt-
//! Pompe), dispersion entropy, increment entropy, and slope entropy — each reduces a real-valued
//! series to a finite alphabet, then reports the Shannon entropy of the resulting symbol
//! distribution.

use crate::numeric::{checked_state_count, neumaier_sum, x_ln_x};
use crate::symbolic::{DispersionSymbolizer, OrdinalConfig, OrdinalSymbolizer, Symbolizer};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Shared
fn symbol_distribution_entropy(symbols: &[u32], alphabet_size: usize, base: LogBase, method: &'static str, diagnostics: Vec<(&'static str, f64)>) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if symbols.is_empty() {
        return Err(EntropyError::EmptyInput { what: "symbols" });
    }
    let mut counts = vec![0.0_f64; alphabet_size];
    for &s in symbols {
        let idx = s as usize;
        if idx >= alphabet_size {
            return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
        }
        counts[idx] += 1.0;
    }
    let n = symbols.len() as f64;
    let nats = -neumaier_sum(counts.iter().map(|&c| x_ln_x(c / n)));

    let mut warnings = Vec::new();
    if symbols.len() < 5 * alphabet_size {
        warnings.push(Warning::SmallSample { n: symbols.len(), recommended: 5 * alphabet_size });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method,
        n: symbols.len(),
        n_effective: symbols.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}

fn sample_sd(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    (neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n).sqrt()
}
// #endregion 🔖Shared

// #region 🔖Permutation
/// 🎼 Bandt-Pompe permutation entropy: Shannon entropy of the ordinal-pattern distribution
/// produced by [`OrdinalSymbolizer`].
pub fn permutation_entropy(x: &[f64], cfg: OrdinalConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    let symbolizer = OrdinalSymbolizer::new(cfg);
    let symbols = symbolizer.symbolize(x)?;
    symbol_distribution_entropy(&symbols, symbolizer.alphabet_size(), base, "permutation_entropy", vec![("dim", cfg.dim as f64), ("tau", cfg.tau as f64)])
}
// #endregion 🔖Permutation

// #region 🔖Dispersion
/// 🎼 Configuration for [`dispersion_entropy`], mirroring [`DispersionSymbolizer`]'s fields.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DispersionConfig {
    pub classes: usize,
    pub dim: usize,
    pub tau: usize,
}

impl DispersionConfig {
    pub fn new(classes: usize, dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if classes < 2 {
            return Err(EntropyError::InvalidConfig { field: "classes", reason: "must be at least 2" });
        }
        if dim == 0 || tau == 0 {
            return Err(EntropyError::InvalidConfig { field: "dim/tau", reason: "must be at least 1" });
        }
        Ok(Self { classes, dim, tau })
    }
}

/// 🎼 Dispersion entropy: Shannon entropy of the normal-CDF-class dispersion-pattern
/// distribution produced by [`DispersionSymbolizer`].
pub fn dispersion_entropy(x: &[f64], cfg: DispersionConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    let symbolizer = DispersionSymbolizer { classes: cfg.classes, dim: cfg.dim, tau: cfg.tau };
    let symbols = symbolizer.symbolize(x)?;
    symbol_distribution_entropy(
        &symbols,
        symbolizer.alphabet_size(),
        base,
        "dispersion_entropy",
        vec![("classes", cfg.classes as f64), ("dim", cfg.dim as f64), ("tau", cfg.tau as f64)],
    )
}
// #endregion 🔖Dispersion

// #region 🔖Increment
/// 🎼 Increment entropy: each successive difference `x[i+1] - x[i]` is encoded as a signed
/// magnitude symbol in `0..(2*levels+1)` (`levels` per-side magnitude buckets sized by quantiles
/// of `|increment| / sd(increments)`, symbol `levels` reserved for an exact-zero increment),
/// `word_length` consecutive increment-symbols are packed via mixed-radix into one word, and the
/// Shannon entropy of the resulting word distribution is reported.
pub fn increment_entropy(x: &[f64], word_length: usize, levels: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    if word_length == 0 {
        return Err(EntropyError::InvalidConfig { field: "word_length", reason: "must be at least 1" });
    }
    if levels == 0 {
        return Err(EntropyError::InvalidConfig { field: "levels", reason: "must be at least 1" });
    }
    if x.len() < word_length + 2 {
        return Err(EntropyError::InsufficientData { what: "increment_entropy", needed: word_length + 2, actual: x.len() });
    }
    let increments: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let sd = sample_sd(&increments);
    let alphabet = 2 * levels + 1;

    let symbols: Vec<u32> = increments
        .iter()
        .map(|&d| {
            if d == 0.0 || sd <= 0.0 {
                return levels as u32;
            }
            let normalized = (d.abs() / sd).min(0.999_999);
            let bucket = ((normalized * levels as f64).floor() as usize).min(levels - 1);
            if d > 0.0 {
                (levels + 1 + bucket) as u32
            } else {
                (levels - 1 - bucket) as u32
            }
        })
        .collect();

    let total = checked_state_count(&vec![alphabet; word_length]).ok_or(EntropyError::InvalidConfig {
        field: "word_length",
        reason: "resulting alphabet overflows",
    })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "word_length", reason: "resulting alphabet exceeds u32::MAX" });
    }
    let words: Vec<u32> = symbols
        .windows(word_length)
        .map(|w| w.iter().fold(0u64, |acc, &s| acc * alphabet as u64 + s as u64) as u32)
        .collect();

    symbol_distribution_entropy(&words, total as usize, base, "increment_entropy", vec![("word_length", word_length as f64), ("levels", levels as f64)])
}
// #endregion 🔖Increment

// #region 🔖Slope
/// 🎼 Slope entropy: each successive slope `atan2(x[i+1] - x[i], 1)` (in radians) is classified
/// into one of 5 symbols by two angular thresholds `(gamma1, gamma2)` with `0 < gamma1 < gamma2 <
/// pi/2`: symbol `0`/`4` for a steep negative/positive slope beyond `gamma2`, `1`/`3` for a
/// shallow negative/positive slope in `(gamma1, gamma2]`, and `2` for a near-flat slope within
/// `gamma1`. `dim` consecutive slope-symbols are packed via mixed-radix (base 5) into one word.
pub fn slope_entropy(x: &[f64], thresholds: (f64, f64), dim: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    let (gamma1, gamma2) = thresholds;
    if !(0.0 < gamma1 && gamma1 < gamma2 && gamma2 < core::f64::consts::FRAC_PI_2) {
        return Err(EntropyError::InvalidConfig { field: "thresholds", reason: "must satisfy 0 < gamma1 < gamma2 < pi/2" });
    }
    if dim == 0 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "must be at least 1" });
    }
    if x.len() < dim + 1 {
        return Err(EntropyError::InsufficientData { what: "slope_entropy", needed: dim + 1, actual: x.len() });
    }
    let symbols: Vec<u32> = x
        .windows(2)
        .map(|w| {
            let angle = (w[1] - w[0]).atan();
            let sign = if angle >= 0.0 { 1.0 } else { -1.0 };
            let mag = angle.abs();
            let level = if mag > gamma2 {
                2
            } else if mag > gamma1 {
                1
            } else {
                0
            };
            (2 + (sign * level as f64) as i32) as u32
        })
        .collect();

    const ALPHABET: usize = 5;
    let total = checked_state_count(&vec![ALPHABET; dim]).ok_or(EntropyError::InvalidConfig { field: "dim", reason: "resulting alphabet overflows" })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "resulting alphabet exceeds u32::MAX" });
    }
    let words: Vec<u32> = symbols.windows(dim).map(|w| w.iter().fold(0u64, |acc, &s| acc * ALPHABET as u64 + s as u64) as u32).collect();

    symbol_distribution_entropy(&words, total as usize, base, "slope_entropy", vec![("dim", dim as f64), ("gamma1", gamma1), ("gamma2", gamma2)])
}
// #endregion 🔖Slope

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_entropy_of_monotone_series_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn permutation_entropy_of_noise_approaches_max() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let cfg = OrdinalConfig::new(3, 1).unwrap();
        let est = permutation_entropy(&x, cfg, LogBase::Bits).unwrap();
        let max = 6.0_f64.log2(); // 3! = 6 patterns
        assert!(est.value > 0.9 * max, "got {} max {}", est.value, max);
    }

    #[test]
    fn dispersion_config_rejects_small_classes() {
        assert!(DispersionConfig::new(1, 2, 1).is_err());
    }

    #[test]
    fn dispersion_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let cfg = DispersionConfig::new(4, 2, 1).unwrap();
        let est = dispersion_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_of_constant_series_is_zero() {
        let x = vec![5.0; 100];
        // 🔐 all increments are exactly zero -> single symbol -> zero entropy.
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn increment_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = increment_entropy(&x, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    #[test]
    fn increment_entropy_rejects_zero_word_length() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        assert!(increment_entropy(&x, 0, 3, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_rejects_bad_thresholds() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        assert!(slope_entropy(&x, (0.5, 0.3), 2, LogBase::Bits).is_err());
        assert!(slope_entropy(&x, (-0.1, 0.5), 2, LogBase::Bits).is_err());
    }

    #[test]
    fn slope_entropy_of_straight_line_is_zero() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn slope_entropy_of_noise_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(4);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let est = slope_entropy(&x, (0.2, 0.8), 2, LogBase::Bits).unwrap();
        assert!(est.value > 0.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn permutation_entropy_orders_regularity_correctly() {
            let mut rng = crate::numeric::Xorshift64::new(5);
            let n = 2000;
            let sine: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();
            let noise: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
            let cfg = OrdinalConfig::new(4, 1).unwrap();
            let h_sine = permutation_entropy(&sine, cfg, LogBase::Bits).unwrap().value;
            let h_noise = permutation_entropy(&noise, cfg, LogBase::Bits).unwrap().value;
            assert!(h_sine < h_noise, "sine={h_sine} noise={h_noise}");
        }
    }
}
// #endregion 🔖Tests
