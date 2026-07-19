//! 🪢 Discrete Wavelet Transform (Mallat filter-bank algorithm) and wavelet entropy: decompose a
//! signal into approximation/detail subbands across dyadic scales, then report the Shannon
//! entropy of the subbands' relative energies as a measure of how concentrated (smooth) vs.
//! spread (noisy) the signal's energy is across scale. See Mallat, S. (1989), "A theory for
//! multiresolution signal decomposition: the wavelet representation."

use crate::numeric::neumaier_sum;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Families
/// 🪢 Orthonormal wavelet family selecting the low-pass decomposition filter. Higher-order
/// Daubechies filters trade a longer support (more taps) for a smoother, more frequency-selective
/// split between scales.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WaveletFamily {
    Haar,
    Daubechies4,
    Daubechies6,
    Daubechies8,
}

impl WaveletFamily {
    /// 🪢 Number of taps in this family's filters; also the minimum signal length a single
    /// decomposition level can consume.
    pub fn filter_len(self) -> usize {
        match self {
            Self::Haar => 2,
            Self::Daubechies4 => 4,
            Self::Daubechies6 => 6,
            Self::Daubechies8 => 8,
        }
    }

    /// 🪢 Orthonormal low-pass (scaling/approximation) decomposition filter coefficients, taps
    /// summing to `sqrt(2)`.
    fn low_pass(self) -> Vec<f64> {
        match self {
            Self::Haar => {
                let s = 1.0 / core::f64::consts::SQRT_2;
                vec![s, s]
            }
            Self::Daubechies4 => {
                let r3 = 3f64.sqrt();
                let d = 4.0 * core::f64::consts::SQRT_2;
                vec![(1.0 + r3) / d, (3.0 + r3) / d, (3.0 - r3) / d, (1.0 - r3) / d]
            }
            Self::Daubechies6 => vec![
                0.332_670_552_950,
                0.806_891_509_311,
                0.459_877_502_118,
                -0.135_011_020_010,
                -0.085_441_273_882,
                0.035_226_291_882,
            ],
            Self::Daubechies8 => vec![
                0.230_377_813_309,
                0.714_846_570_553,
                0.630_880_767_930,
                -0.027_983_769_417,
                -0.187_034_811_719,
                0.030_841_381_836,
                0.032_883_011_667,
                -0.010_597_401_785,
            ],
        }
    }
}

/// 🪢 Derives the high-pass (wavelet/detail) filter from a low-pass filter `h` via the quadrature
/// mirror relation `g[n] = (-1)^n * h[len-1-n]`.
fn high_pass(h: &[f64]) -> Vec<f64> {
    let len = h.len();
    (0..len)
        .map(|n| {
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            sign * h[len - 1 - n]
        })
        .collect()
}
// #endregion 🔖Families

// #region 🔖Boundary
/// 🪢 How a filter tap that reads outside `[0, signal.len())` is resolved.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoundaryMode {
    /// 🪢 Out-of-range samples contribute `0.0`.
    Zero,
    /// 🪢 The index wraps modulo `signal.len()` (circular convolution).
    Periodic,
    /// 🪢 The index reflects off the boundary, whole-point style, duplicating the edge sample
    /// (`-1 -> 0`, `-2 -> 1`, `len -> len-1`, `len+1 -> len-2`).
    Symmetric,
}

/// 🪢 Resolves a possibly out-of-range tap index `idx` against a signal of length `n` under
/// `mode`. Returns `None` only for [`BoundaryMode::Zero`] out-of-range reads (contribute nothing).
fn resolve_index(idx: i64, n: usize, mode: BoundaryMode) -> Option<usize> {
    let n_i = n as i64;
    if idx >= 0 && idx < n_i {
        return Some(idx as usize);
    }
    match mode {
        BoundaryMode::Zero => None,
        BoundaryMode::Periodic => Some((((idx % n_i) + n_i) % n_i) as usize),
        BoundaryMode::Symmetric => {
            if n_i == 1 {
                return Some(0);
            }
            let period = 2 * n_i;
            let mut m = idx % period;
            if m < 0 {
                m += period;
            }
            let mapped = if m < n_i { m } else { period - 1 - m };
            Some(mapped as usize)
        }
    }
}
// #endregion 🔖Boundary

// #region 🔖FilterBank
/// 🪢 One decomposition level: convolves `s` with `filter` and downsamples by 2, keeping output
/// index `k` computed from input taps at `2*k + i` (boundary-resolved), `i` in `0..filter.len()`.
/// Output length is `ceil(s.len() / 2)`.
fn convolve_downsample(s: &[f64], filter: &[f64], boundary: BoundaryMode) -> Vec<f64> {
    let n = s.len();
    let out_len = n.div_ceil(2);
    (0..out_len)
        .map(|k| {
            neumaier_sum((0..filter.len()).filter_map(|i| {
                let idx = 2 * k as i64 + i as i64;
                resolve_index(idx, n, boundary).map(|j| filter[i] * s[j])
            }))
        })
        .collect()
}
// #endregion 🔖FilterBank

// #region 🔖Config
/// 🪢 Configuration for [`Dwt::decompose`] / [`wavelet_entropy`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WaveletConfig {
    pub family: WaveletFamily,
    /// 🪢 Requested number of decomposition levels; the actual number achieved may be fewer if
    /// the approximation subband shrinks below the filter length first (see
    /// [`Dwt::decompose`]).
    pub levels: usize,
    pub boundary: BoundaryMode,
}

impl WaveletConfig {
    /// 🪢 Builds a config, rejecting `levels == 0`.
    pub fn new(family: WaveletFamily, levels: usize, boundary: BoundaryMode) -> Result<Self, EntropyError> {
        if levels == 0 {
            return Err(EntropyError::InvalidConfig { field: "levels", reason: "must be at least 1" });
        }
        Ok(Self { family, levels, boundary })
    }
}
// #endregion 🔖Config

// #region 🔖Dwt
/// 🪢 A fitted multi-level Discrete Wavelet Transform: the detail subband captured at each level
/// (finest first) plus the final approximation subband.
pub struct Dwt {
    details: Vec<Vec<f64>>,
    approximation: Vec<f64>,
    levels_achieved: usize,
}

impl Dwt {
    /// 🪢 Decomposes `x` into up to `cfg.levels` levels of the Mallat filter-bank algorithm,
    /// repeatedly splitting the current approximation into a coarser approximation and a detail
    /// subband. Stops early (without erroring) once the current approximation's length drops
    /// below the chosen family's filter length; the number of levels actually produced is
    /// reported by [`Dwt::levels_achieved`]. Rejects `x` shorter than the filter length outright.
    pub fn decompose(x: &[f64], cfg: WaveletConfig) -> Result<Self, EntropyError> {
        let filter_len = cfg.family.filter_len();
        if x.len() < filter_len {
            return Err(EntropyError::InsufficientData {
                what: "wavelet decomposition",
                needed: filter_len,
                actual: x.len(),
            });
        }
        let h = cfg.family.low_pass();
        let g = high_pass(&h);
        let mut approximation = x.to_vec();
        let mut details = Vec::with_capacity(cfg.levels);
        let mut levels_achieved = 0usize;
        for _ in 0..cfg.levels {
            if approximation.len() < filter_len {
                break;
            }
            let detail = convolve_downsample(&approximation, &g, cfg.boundary);
            let next_approximation = convolve_downsample(&approximation, &h, cfg.boundary);
            details.push(detail);
            approximation = next_approximation;
            levels_achieved += 1;
        }
        Ok(Self { details, approximation, levels_achieved })
    }

    /// 🪢 Number of levels actually produced (`<= cfg.levels`; see [`Dwt::decompose`]).
    pub fn levels_achieved(&self) -> usize {
        self.levels_achieved
    }

    /// 🪢 Energy (sum of squared coefficients) of each subband, detail levels finest-first
    /// followed by the final approximation.
    pub fn subband_energies(&self) -> Vec<f64> {
        let mut energies: Vec<f64> =
            self.details.iter().map(|d| neumaier_sum(d.iter().map(|&v| v * v))).collect();
        energies.push(neumaier_sum(self.approximation.iter().map(|&v| v * v)));
        energies
    }
}
// #endregion 🔖Dwt

// #region 🔖Entropy
/// 🪢 Wavelet entropy of `x`: decomposes `x` per `cfg`, takes each subband's relative energy
/// (energy / total energy across all subbands), and reports the Shannon entropy (in nats) of that
/// distribution — low when energy concentrates in a few subbands (smooth or tonal signals), high
/// when it spreads evenly across scales (noise-like signals).
pub fn wavelet_entropy(x: &[f64], cfg: WaveletConfig) -> Result<Estimate, EntropyError> {
    let dwt = Dwt::decompose(x, cfg)?;
    let energies = dwt.subband_energies();
    let total = neumaier_sum(energies.iter().copied());
    if !(total > 1e-300) {
        return Err(EntropyError::DegenerateInput { what: "wavelet subband energies sum to ~0" });
    }
    let p: Vec<f64> = energies.iter().map(|&e| (e / total).max(0.0)).collect();
    let nats = crate::discrete::entropy(&p, LogBase::Nats)?;

    let n = x.len();
    let mut warnings = Vec::new();
    if n < 30 {
        warnings.push(Warning::SmallSample { n, recommended: 30 });
    }

    Ok(Estimate {
        value: nats,
        base: LogBase::Nats,
        method: "wavelet_entropy",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("levels_achieved", dwt.levels_achieved() as f64),
            ("num_subbands", p.len() as f64),
        ],
    })
}
// #endregion 🔖Entropy

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haar_periodic_one_level_preserves_energy() {
        // 🔐 Parseval / energy-preservation: Haar under periodic boundary is exactly orthonormal,
        // so sum(a^2) + sum(d^2) == sum(s^2) to tight tolerance.
        let x = [1.0, 3.0, -2.0, 5.0, 0.5, -1.5, 4.0, 2.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-9, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn haar_periodic_multi_level_preserves_energy() {
        let mut rng = crate::numeric::Xorshift64::new(11);
        let x: Vec<f64> = (0..64).map(|_| rng.next_f64() * 10.0 - 5.0).collect();
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 4, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let subband_total: f64 = dwt.subband_energies().iter().sum();
        let signal_total: f64 = x.iter().map(|&v| v * v).sum();
        assert!((subband_total - signal_total).abs() < 1e-6, "subband={subband_total} signal={signal_total}");
    }

    #[test]
    fn constant_signal_has_zero_haar_detail_energy() {
        let x = [3.0; 8];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 1, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        let energies = dwt.subband_energies();
        assert!(energies[0].abs() < 1e-9, "detail energy = {}", energies[0]);
    }

    #[test]
    fn smooth_ramp_has_lower_wavelet_entropy_than_noise() {
        let n = 256;
        let ramp: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let mut rng = crate::numeric::Xorshift64::new(21);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies4, 4, BoundaryMode::Symmetric).unwrap();
        let ramp_est = wavelet_entropy(&ramp, cfg).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        assert!(ramp_est.value < noise_est.value, "ramp={} noise={}", ramp_est.value, noise_est.value);
    }

    #[test]
    fn white_noise_has_higher_wavelet_entropy_than_pure_tone() {
        let n = 512;
        let mut rng = crate::numeric::Xorshift64::new(33);
        let noise: Vec<f64> = (0..n).map(|_| rng.next_f64() - 0.5).collect();
        let sine: Vec<f64> = (0..n)
            .map(|i| (2.0 * core::f64::consts::PI * 4.0 * i as f64 / n as f64).sin())
            .collect();

        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 5, BoundaryMode::Symmetric).unwrap();
        let noise_est = wavelet_entropy(&noise, cfg).unwrap();
        let sine_est = wavelet_entropy(&sine, cfg).unwrap();
        assert!(noise_est.value > sine_est.value, "noise={} sine={}", noise_est.value, sine_est.value);
    }

    #[test]
    fn config_new_rejects_zero_levels() {
        assert!(matches!(
            WaveletConfig::new(WaveletFamily::Haar, 0, BoundaryMode::Zero),
            Err(EntropyError::InvalidConfig { field: "levels", .. })
        ));
    }

    #[test]
    fn decompose_rejects_input_shorter_than_filter() {
        let x = [1.0, 2.0, 3.0];
        let cfg = WaveletConfig::new(WaveletFamily::Daubechies8, 1, BoundaryMode::Zero).unwrap();
        assert!(matches!(
            Dwt::decompose(&x, cfg),
            Err(EntropyError::InsufficientData { needed: 8, actual: 3, .. })
        ));
    }

    #[test]
    fn decompose_stops_early_when_signal_runs_out_of_levels() {
        // 🔐 8 samples with Haar (filter_len 2) can only produce 3 dyadic halvings (8->4->2->1)
        // before the approximation would drop below the filter length on a 4th level.
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let dwt = Dwt::decompose(&x, cfg).unwrap();
        assert!(dwt.levels_achieved() < 10);
        assert!(dwt.levels_achieved() >= 1);
    }

    #[test]
    fn wavelet_entropy_reports_levels_achieved_diagnostic() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 10, BoundaryMode::Periodic).unwrap();
        let est = wavelet_entropy(&x, cfg).unwrap();
        let levels = est.diagnostics.iter().find(|(k, _)| *k == "levels_achieved").unwrap().1;
        assert!(levels < 10.0);
    }

    #[test]
    fn wavelet_entropy_rejects_all_zero_input() {
        let x = [0.0; 16];
        let cfg = WaveletConfig::new(WaveletFamily::Haar, 2, BoundaryMode::Zero).unwrap();
        assert!(matches!(wavelet_entropy(&x, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn daubechies_filters_are_orthonormal_sum_sqrt2() {
        for family in [
            WaveletFamily::Haar,
            WaveletFamily::Daubechies4,
            WaveletFamily::Daubechies6,
            WaveletFamily::Daubechies8,
        ] {
            let h = family.low_pass();
            let sum: f64 = h.iter().sum();
            // 🔐 published Daubechies-6 coefficients are truncated to 12 decimal digits, so their
            // sum only approximates sqrt(2) to about 1e-8, not full f64 precision.
            assert!((sum - core::f64::consts::SQRT_2).abs() < 1e-6, "{family:?} sum={sum}");
        }
    }
}
// #endregion 🔖Tests
