//! 📶 Welch-method spectral entropy: segment a real time series, window and FFT each segment,
//! average the one-sided power spectra into a periodogram, and treat the normalized periodogram
//! as a probability distribution over frequency bins whose Shannon entropy summarizes how
//! concentrated (tonal) vs. spread (noise-like) the signal's power is across frequency. See
//! Welch, P. (1967), "The use of fast Fourier transform for the estimation of power spectra."

use crate::fft::{window, Complex, Fft, WindowKind};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Config
/// 📶 Configuration for [`spectral_entropy`]'s Welch periodogram estimate.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpectralConfig {
    /// 📶 Analysis window applied to each segment before its FFT.
    pub window: WindowKind,
    /// 📶 Samples per segment. `0` means "auto: `min(256, x.len())`".
    pub segment_len: usize,
    /// 📶 Fractional overlap between consecutive segments, in `[0, 1)` (`0.5` = 50% overlap).
    pub overlap: f64,
    /// 📶 Optional normalized-frequency band `(lo, hi)` in `[0, 0.5]` (as a fraction of the
    /// sample rate; `0.5` is Nyquist) restricting which PSD bins contribute to the entropy.
    pub band: Option<(f64, f64)>,
    /// 📶 If `true`, the returned value is divided by `ln(bins used)` into a dimensionless
    /// `[0, 1]` ratio instead of a proper entropy (see [`spectral_entropy`] docs).
    pub normalize: bool,
}

impl Default for SpectralConfig {
    fn default() -> Self {
        Self { window: WindowKind::Hann, segment_len: 0, overlap: 0.5, band: None, normalize: false }
    }
}

impl SpectralConfig {
    /// 📶 Builds a config, validating that `overlap` is a finite fraction in `[0, 1)`.
    pub fn new(window: WindowKind, segment_len: usize, overlap: f64) -> Result<Self, EntropyError> {
        validate_overlap(overlap)?;
        Ok(Self { window, segment_len, overlap, band: None, normalize: false })
    }
}

fn validate_overlap(overlap: f64) -> Result<(), EntropyError> {
    if !(overlap.is_finite() && overlap >= 0.0 && overlap < 1.0) {
        return Err(EntropyError::InvalidConfig {
            field: "overlap",
            reason: "must be a finite fraction in [0, 1)",
        });
    }
    Ok(())
}
// #endregion 🔖Config

// #region 🔖Welch
/// 📶 Welch-method spectral entropy of `x`: segments `x` (dropping any incomplete tail segment),
/// windows and FFTs each segment, averages the one-sided power spectra into a periodogram,
/// normalizes it (optionally restricted to `cfg.band`) into a probability vector over frequency
/// bins, and reports its Shannon entropy in nats.
///
/// When `cfg.normalize` is `true` the returned `Estimate.value` is instead the dimensionless
/// ratio `H / ln(bins used)` in `[0, 1]` — a unitless measure of spectral flatness, not a proper
/// entropy in any log base.
pub fn spectral_entropy(x: &[f64], cfg: SpectralConfig) -> Result<Estimate, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "spectral entropy input" });
    }
    for (i, &v) in x.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "spectral entropy input", index: i });
        }
    }
    validate_overlap(cfg.overlap)?;

    let segment_len = if cfg.segment_len == 0 { x.len().min(256) } else { cfg.segment_len };
    if segment_len < 2 {
        return Err(EntropyError::InvalidConfig { field: "segment_len", reason: "must be at least 2" });
    }
    if segment_len > x.len() {
        return Err(EntropyError::InvalidConfig {
            field: "segment_len",
            reason: "must not exceed input length",
        });
    }

    let hop = (((segment_len as f64) * (1.0 - cfg.overlap)).round().max(1.0)) as usize;
    let win = window(cfg.window, segment_len);
    let plan = Fft::new(segment_len);
    let n_bins = segment_len / 2 + 1;

    let mut power_sum = vec![0.0_f64; n_bins];
    let mut n_segments = 0usize;
    let mut start = 0usize;
    while start + segment_len <= x.len() {
        let segment: Vec<Complex> = x[start..start + segment_len]
            .iter()
            .zip(win.iter())
            .map(|(&xv, &wv)| Complex::new(xv * wv, 0.0))
            .collect();
        let spectrum = plan.forward(&segment);
        for k in 0..n_bins {
            power_sum[k] += spectrum[k].norm_sq();
        }
        n_segments += 1;
        start += hop;
    }
    if n_segments == 0 {
        return Err(EntropyError::InsufficientData { what: "Welch segments", needed: 1, actual: 0 });
    }
    let power_avg: Vec<f64> = power_sum.iter().map(|&p| p / n_segments as f64).collect();

    let selected: Vec<f64> = match cfg.band {
        None => power_avg,
        Some((lo, hi)) => {
            if !(lo.is_finite() && hi.is_finite() && lo >= 0.0 && hi <= 0.5 && lo < hi) {
                return Err(EntropyError::InvalidConfig {
                    field: "band",
                    reason: "must satisfy 0 <= lo < hi <= 0.5",
                });
            }
            let sub: Vec<f64> = (0..n_bins)
                .filter(|&k| {
                    let freq = k as f64 / segment_len as f64;
                    freq >= lo && freq <= hi
                })
                .map(|k| power_avg[k])
                .collect();
            if sub.is_empty() {
                return Err(EntropyError::InvalidConfig { field: "band", reason: "selects zero frequency bins" });
            }
            sub
        }
    };

    let total_power: f64 = selected.iter().sum();
    if !(total_power > 0.0) {
        return Err(EntropyError::DegenerateInput { what: "power spectrum has zero total power" });
    }
    let p: Vec<f64> = selected.iter().map(|&v| v / total_power).collect();
    let bins = p.len();

    let entropy_nats = crate::discrete::entropy(&p, LogBase::Nats)?;
    let value = if cfg.normalize {
        if bins <= 1 { 0.0 } else { entropy_nats / (bins as f64).ln() }
    } else {
        entropy_nats
    };

    let mut warnings = Vec::new();
    if n_segments < 4 {
        warnings.push(Warning::SmallSample { n: n_segments, recommended: 4 });
    }

    Ok(Estimate {
        value,
        base: LogBase::Nats,
        method: "welch_spectral_entropy",
        n: x.len(),
        n_effective: n_segments as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("segments", n_segments as f64),
            ("segment_len", segment_len as f64),
            ("bins", bins as f64),
        ],
    })
}
// #endregion 🔖Welch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sine(n: usize, freq: f64, sample_rate: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * freq * i as f64 / sample_rate).sin()).collect()
    }

    fn white_noise(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_f64() - 0.5).collect()
    }

    #[test]
    fn pure_sine_has_low_normalized_spectral_entropy() {
        let x = sine(4096, 50.0, 1000.0);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value < 0.3, "got {}", est.value);
        assert!(est.n_effective >= 4.0);
    }

    #[test]
    fn white_noise_has_high_normalized_spectral_entropy() {
        let x = white_noise(4096, 7);
        let cfg = SpectralConfig { window: WindowKind::Hann, normalize: true, ..Default::default() };
        let est = spectral_entropy(&x, cfg).unwrap();
        assert!(est.value > 0.85, "got {}", est.value);
    }

    #[test]
    fn band_restriction_changes_entropy_and_validates_range() {
        let x = sine(4096, 50.0, 1000.0);
        let full = spectral_entropy(&x, SpectralConfig { normalize: true, ..Default::default() }).unwrap();
        let in_band = spectral_entropy(
            &x,
            SpectralConfig { normalize: true, band: Some((0.02, 0.08)), ..Default::default() },
        )
        .unwrap();
        let out_of_band = spectral_entropy(
            &x,
            SpectralConfig { normalize: true, band: Some((0.3, 0.5)), ..Default::default() },
        )
        .unwrap();
        assert!(in_band.value != full.value);
        assert!(out_of_band.value > in_band.value, "in_band={} out_of_band={}", in_band.value, out_of_band.value);

        assert!(matches!(
            spectral_entropy(&x, SpectralConfig { band: Some((0.4, 0.1)), ..Default::default() }),
            Err(EntropyError::InvalidConfig { field: "band", .. })
        ));
        assert!(matches!(
            spectral_entropy(&x, SpectralConfig { band: Some((0.6, 0.7)), ..Default::default() }),
            Err(EntropyError::InvalidConfig { field: "band", .. })
        ));
    }

    #[test]
    fn rejects_segment_len_larger_than_input() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let cfg = SpectralConfig { segment_len: 100, ..Default::default() };
        assert!(matches!(
            spectral_entropy(&x, cfg),
            Err(EntropyError::InvalidConfig { field: "segment_len", .. })
        ));
    }

    #[test]
    fn config_new_rejects_overlap_at_or_above_one() {
        assert!(matches!(
            SpectralConfig::new(WindowKind::Hann, 128, 1.0),
            Err(EntropyError::InvalidConfig { field: "overlap", .. })
        ));
        assert!(matches!(
            SpectralConfig::new(WindowKind::Hann, 128, 1.5),
            Err(EntropyError::InvalidConfig { field: "overlap", .. })
        ));
        assert!(SpectralConfig::new(WindowKind::Hann, 128, 0.75).is_ok());
    }
}
// #endregion 🔖Tests
