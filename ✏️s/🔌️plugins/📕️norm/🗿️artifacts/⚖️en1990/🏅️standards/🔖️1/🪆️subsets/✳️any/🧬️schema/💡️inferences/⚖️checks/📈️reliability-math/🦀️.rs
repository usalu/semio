//! 📈️ Reliability mathematics of EN 1990 Annexes B, C and D: the normal distribution, the Gumbel fractile, FORM with Rackwitz-Fiessler transformation, Palmgren-Miner damage and the k_n table. See [`super`].

use super::*;

//#region 🔖️NormalDistribution
const SQRT_PI: f64 = 1.772_453_850_905_516;

fn erf_series(x: f64) -> f64 {
    let (x2, mut term, mut sum) = (x * x, x, x);
    let mut n = 0.0;
    while n < 200.0 {
        n += 1.0;
        term *= -x2 / n;
        let add = term / (2.0 * n + 1.0);
        sum += add;
        if add.abs() < 1e-17 * sum.abs() {
            break;
        }
    }
    2.0 / SQRT_PI * sum
}

fn erfc_tail(x: f64) -> f64 {
    let mut t = x;
    for k in (1..=120).rev() {
        t = x + (k as f64 / 2.0) / t;
    }
    (-x * x).exp() / (SQRT_PI * t)
}

/// 📈️ The complementary error function, accurate in the far tails (relative 1e-12).
pub fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        2.0 - erfc(-x)
    } else if x < 2.0 {
        1.0 - erf_series(x)
    } else {
        erfc_tail(x)
    }
}

/// 📈️ The standard normal distribution function Φ.
pub fn phi_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / std::f64::consts::SQRT_2)
}

/// 📈️ The standard normal density φ.
pub fn phi_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

/// 📈️ The inverse standard normal distribution Φ⁻¹ (Acklam start, one Halley refinement).
pub fn phi_inv(p: f64) -> f64 {
    if !(p > 0.0 && p < 1.0) {
        return if p <= 0.0 { f64::NEG_INFINITY } else { f64::INFINITY };
    }
    const A: [f64; 6] = [-3.969_683_028_665_376e1, 2.209_460_984_245_205e2, -2.759_285_104_469_687e2, 1.383_577_518_672_69e2, -3.066_479_806_614_716e1, 2.506_628_277_459_239];
    const B: [f64; 5] = [-5.447_609_879_822_406e1, 1.615_858_368_580_409e2, -1.556_989_798_598_866e2, 6.680_131_188_771_972e1, -1.328_068_155_288_572e1];
    const C: [f64; 6] = [-7.784_894_002_430_293e-3, -3.223_964_580_411_365e-1, -2.400_758_277_161_838, -2.549_732_539_343_734, 4.374_664_141_464_968, 2.938_163_982_698_783];
    const D: [f64; 4] = [7.784_695_709_041_462e-3, 3.224_671_290_700_398e-1, 2.445_134_137_142_996, 3.754_408_661_907_416];
    let x = if p < 0.024_25 {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p > 1.0 - 0.024_25 {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    };
    let e = phi_cdf(x) - p;
    let u = e * (2.0 * std::f64::consts::PI).sqrt() * (0.5 * x * x).exp();
    x - u / (1.0 + 0.5 * x * u)
}
//#endregion 🔖️NormalDistribution

//#region 🔖️Distributions
const EULER_GAMMA: f64 = 0.577_215_664_901_532_9;

fn ln_gamma(x: f64) -> f64 {
    const G: [f64; 9] = [0.999_999_999_999_809_9, 676.520_368_121_885_1, -1_259.139_216_722_402_8, 771.323_428_777_653_1, -176.615_029_162_140_6, 12.507_343_278_686_905, -0.138_571_095_265_720_12, 9.984_369_578_019_572e-6, 1.505_632_735_149_311_6e-7];
    let x = x - 1.0;
    let t = x + 7.5;
    let sum = G.iter().enumerate().skip(1).fold(G[0], |acc, (i, g)| acc + g / (x + i as f64));
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}

fn gamma_fn(x: f64) -> f64 {
    ln_gamma(x).exp()
}

/// 📈️ A basic-variable distribution of Annex C, parameterised by mean and standard deviation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dist {
    Normal { mean: f64, sd: f64 },
    LogNormal { mean: f64, sd: f64 },
    Gumbel { mean: f64, sd: f64 },
    Weibull { mean: f64, sd: f64 },
}

impl Dist {
    pub fn parse(kind: &str, mean: f64, sd: f64) -> Result<Self, String> {
        if !(mean.is_finite() && sd.is_finite() && sd > 0.0) {
            return Err(format!("{kind}: mean and a positive standard deviation are required"));
        }
        match kind {
            "normal" => Ok(Self::Normal { mean, sd }),
            "lognormal" if mean > 0.0 => Ok(Self::LogNormal { mean, sd }),
            "gumbel" => Ok(Self::Gumbel { mean, sd }),
            "weibull" if mean > 0.0 => Ok(Self::Weibull { mean, sd }),
            other => Err(format!("unknown distribution {other}")),
        }
    }

    pub fn mean(&self) -> f64 {
        match *self {
            Self::Normal { mean, .. } | Self::LogNormal { mean, .. } | Self::Gumbel { mean, .. } | Self::Weibull { mean, .. } => mean,
        }
    }

    pub fn sd(&self) -> f64 {
        match *self {
            Self::Normal { sd, .. } | Self::LogNormal { sd, .. } | Self::Gumbel { sd, .. } | Self::Weibull { sd, .. } => sd,
        }
    }

    fn weibull_shape(mean: f64, sd: f64) -> f64 {
        let v2 = (sd / mean).powi(2);
        let (mut lo, mut hi) = (0.2, 200.0);
        for _ in 0..200 {
            let k = 0.5 * (lo + hi);
            let ratio = gamma_fn(1.0 + 2.0 / k) / gamma_fn(1.0 + 1.0 / k).powi(2) - 1.0;
            if ratio > v2 {
                lo = k;
            } else {
                hi = k;
            }
        }
        0.5 * (lo + hi)
    }

    pub fn cdf(&self, x: f64) -> f64 {
        match *self {
            Self::Normal { mean, sd } => phi_cdf((x - mean) / sd),
            Self::LogNormal { mean, sd } => {
                if x <= 0.0 {
                    return 0.0;
                }
                let zeta2 = (1.0 + (sd / mean).powi(2)).ln();
                phi_cdf((x.ln() - (mean.ln() - 0.5 * zeta2)) / zeta2.sqrt())
            }
            Self::Gumbel { mean, sd } => {
                let alpha = std::f64::consts::PI / (sd * 6.0_f64.sqrt());
                let u = mean - EULER_GAMMA / alpha;
                (-(-alpha * (x - u)).exp()).exp()
            }
            Self::Weibull { mean, sd } => {
                if x <= 0.0 {
                    return 0.0;
                }
                let k = Self::weibull_shape(mean, sd);
                let lambda = mean / gamma_fn(1.0 + 1.0 / k);
                1.0 - (-(x / lambda).powf(k)).exp()
            }
        }
    }

    pub fn pdf(&self, x: f64) -> f64 {
        match *self {
            Self::Normal { mean, sd } => phi_pdf((x - mean) / sd) / sd,
            Self::LogNormal { mean, sd } => {
                if x <= 0.0 {
                    return 0.0;
                }
                let zeta2 = (1.0 + (sd / mean).powi(2)).ln();
                let zeta = zeta2.sqrt();
                phi_pdf((x.ln() - (mean.ln() - 0.5 * zeta2)) / zeta) / (x * zeta)
            }
            Self::Gumbel { mean, sd } => {
                let alpha = std::f64::consts::PI / (sd * 6.0_f64.sqrt());
                let u = mean - EULER_GAMMA / alpha;
                let z = (-alpha * (x - u)).exp();
                alpha * z * (-z).exp()
            }
            Self::Weibull { mean, sd } => {
                if x <= 0.0 {
                    return 0.0;
                }
                let k = Self::weibull_shape(mean, sd);
                let lambda = mean / gamma_fn(1.0 + 1.0 / k);
                k / lambda * (x / lambda).powf(k - 1.0) * (-(x / lambda).powf(k)).exp()
            }
        }
    }

    /// 📈️ The p-fractile of the distribution.
    pub fn fractile(&self, p: f64) -> f64 {
        match *self {
            Self::Normal { mean, sd } => mean + sd * phi_inv(p),
            Self::LogNormal { mean, sd } => {
                let zeta2 = (1.0 + (sd / mean).powi(2)).ln();
                (mean.ln() - 0.5 * zeta2 + zeta2.sqrt() * phi_inv(p)).exp()
            }
            Self::Gumbel { mean, sd } => {
                let alpha = std::f64::consts::PI / (sd * 6.0_f64.sqrt());
                mean - EULER_GAMMA / alpha - (-p.ln()).ln() / alpha
            }
            Self::Weibull { mean, sd } => {
                let k = Self::weibull_shape(mean, sd);
                mean / gamma_fn(1.0 + 1.0 / k) * (-(1.0 - p).ln()).powf(1.0 / k)
            }
        }
    }
}

/// 📈️ The characteristic value of a variable action: the 98 % fractile of its annual maximum (Gumbel), i.e. the 50-year return value.
pub fn annual_maximum_characteristic(mean: f64, cov: f64) -> f64 {
    Dist::Gumbel { mean, sd: mean * cov }.fractile(0.98)
}
//#endregion 🔖️Distributions

//#region 🔖️Form
/// 📈️ The outcome of one FORM run for the limit state g = R − E.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormResult {
    pub beta: f64,
    pub alpha_r: f64,
    pub alpha_e: f64,
    pub design_r: f64,
    pub design_e: f64,
    pub iterations: u32,
    pub converged: bool,
}

impl FormResult {
    pub fn failure_probability(&self) -> f64 {
        phi_cdf(-self.beta)
    }
}

fn normal_tail(dist: &Dist, x: f64) -> (f64, f64) {
    let cdf = dist.cdf(x).clamp(1e-300, 1.0 - 1e-16);
    let z = phi_inv(cdf);
    let sd_n = phi_pdf(z) / dist.pdf(x).max(1e-300);
    (x - z * sd_n, sd_n)
}

/// 📈️ First-order reliability method for g = R − E with Rackwitz-Fiessler normal-tail transformation (Annex C.5-C.7), at most 100 iterations, tolerance 1e-8.
pub fn form_r_minus_e(resistance: &Dist, action: &Dist) -> FormResult {
    let (mut xr, mut xe) = ((resistance.mean() - 0.8 * 3.8 * resistance.sd()).max(1e-6 * resistance.mean().abs()), action.mean() + 0.7 * 3.8 * action.sd());
    let mut result = FormResult { beta: 0.0, alpha_r: 0.0, alpha_e: 0.0, design_r: xr, design_e: xe, iterations: 0, converged: false };
    for iteration in 1..=100 {
        let (mu_r, sd_r) = normal_tail(resistance, xr);
        let (mu_e, sd_e) = normal_tail(action, xe);
        let norm = (sd_r * sd_r + sd_e * sd_e).sqrt();
        let (alpha_r, alpha_e) = (sd_r / norm, -sd_e / norm);
        let beta = (mu_r - mu_e) / norm;
        let (next_r, next_e) = (mu_r - alpha_r * beta * sd_r, mu_e - alpha_e * beta * sd_e);
        let moved = (next_r - xr).abs() / xr.abs().max(1e-12) + (next_e - xe).abs() / xe.abs().max(1e-12);
        (xr, xe) = (next_r, next_e);
        result = FormResult { beta, alpha_r, alpha_e, design_r: xr, design_e: xe, iterations: iteration, converged: moved < 1e-8 };
        if result.converged {
            break;
        }
    }
    result
}

/// 📈️ The outcome of one FORM run for g = R − ΣE_i with one resistance and any number of independent actions; `alpha_e`, `design_e` follow the order of the actions.
#[derive(Clone, Debug, PartialEq)]
pub struct MultiFormResult {
    pub beta: f64,
    pub alpha_r: f64,
    pub alpha_e: Vec<f64>,
    pub design_r: f64,
    pub design_e: Vec<f64>,
    pub iterations: u32,
    pub converged: bool,
}

impl MultiFormResult {
    pub fn failure_probability(&self) -> f64 {
        phi_cdf(-self.beta)
    }
}

/// 📈️ First-order reliability method for the linear limit state g = R − ΣE_i (Annex C.5-C.7 with several actions, for example permanent G and variable Q) by Rackwitz-Fiessler normal-tail iteration, at most 200 iterations, tolerance 1e-8; the start point is 0.8·3.8 σ_R below the resistance mean and 0.7·3.8 σ_E above each action mean.
pub fn form_multi(resistance: &Dist, actions: &[Dist]) -> MultiFormResult {
    let mut xr = (resistance.mean() - 0.8 * 3.8 * resistance.sd()).max(1e-6 * resistance.mean().abs());
    let mut xe: Vec<f64> = actions.iter().map(|a| a.mean() + 0.7 * 3.8 * a.sd()).collect();
    let mut result = MultiFormResult { beta: 0.0, alpha_r: 0.0, alpha_e: vec![0.0; actions.len()], design_r: xr, design_e: xe.clone(), iterations: 0, converged: false };
    for iteration in 1..=200 {
        let (mu_r, sd_r) = normal_tail(resistance, xr);
        let tails: Vec<(f64, f64)> = actions.iter().zip(&xe).map(|(a, x)| normal_tail(a, *x)).collect();
        let norm = (sd_r * sd_r + tails.iter().map(|(_, sd)| sd * sd).sum::<f64>()).sqrt();
        let beta = (mu_r - tails.iter().map(|(mu, _)| mu).sum::<f64>()) / norm;
        let alpha_r = sd_r / norm;
        let alpha_e: Vec<f64> = tails.iter().map(|(_, sd)| -sd / norm).collect();
        let next_r = mu_r - alpha_r * beta * sd_r;
        let next_e: Vec<f64> = tails.iter().zip(&alpha_e).map(|((mu, sd), alpha)| mu - alpha * beta * sd).collect();
        let moved = (next_r - xr).abs() / xr.abs().max(1e-12) + next_e.iter().zip(&xe).map(|(n, o)| (n - o).abs() / o.abs().max(1e-12)).sum::<f64>();
        (xr, xe) = (next_r, next_e);
        result = MultiFormResult { beta, alpha_r, alpha_e, design_r: xr, design_e: xe.clone(), iterations: iteration, converged: moved < 1e-8 };
        if result.converged {
            break;
        }
    }
    result
}

/// 📈️ Annex C.7 design values with the fixed sensitivity factors α_R = 0.8 and α_E = −0.7, valid for 0.16 < σ_E/σ_R < 7.6; outside, ±1.0 for the dominant and ±0.4 for the other variable.
pub fn annex_c_alphas(annex: AnnexChoice, sd_e: f64, sd_r: f64) -> (f64, f64) {
    let ratio = sd_e / sd_r;
    let (lo, hi) = (ndp(annex, "ndp.en1990.form.ratio_lo"), ndp(annex, "ndp.en1990.form.ratio_hi"));
    if ratio > lo && ratio < hi {
        (ndp(annex, "ndp.en1990.form.alpha_r"), ndp(annex, "ndp.en1990.form.alpha_e"))
    } else if ratio >= hi {
        (0.4, -1.0)
    } else {
        (1.0, -0.4)
    }
}
/// 📈️ Annex C.7 sensitivity factor of the accompanying (secondary) action: the leading action keeps α_E, the others use 0.4 α_E (registry `ndp.en1990.form.alpha_e_secondary_factor`, recalled and unverified).
pub fn annex_c_secondary_alpha(annex: AnnexChoice, alpha_leading: f64) -> f64 {
    ndp(annex, "ndp.en1990.form.alpha_e_secondary_factor") * alpha_leading
}
//#endregion 🔖️Form

//#region 🔖️PalmgrenMiner
/// ✂️ The constant-amplitude fatigue limit Δσ_D = (2/5)^(1/3) Δσ_C (N_D = 5·10⁶) and the cut-off limit Δσ_L = (5/100)^(1/5) Δσ_D (N_L = 10⁸) of the detail category Δσ_C. Constants unverified against the printed EN 1993-1-9.
pub fn en1993_cut_off_ranges(delta_sigma_c: f64) -> (f64, f64) {
    let delta_d = (2.0_f64 / 5.0).powf(1.0 / 3.0) * delta_sigma_c;
    (delta_d, (5.0_f64 / 100.0).powf(1.0 / 5.0) * delta_d)
}

/// 🔩️ Endurance N of a stress range Δσ on an EN 1993-1-9 style S-N curve with slopes m=3 above Δσ_D (N_D = 5·10⁶), m=5 between Δσ_D and the cut-off Δσ_L (N_L = 10⁸); `None` below the cut-off.
pub fn en1993_endurance_cycles(delta_sigma: f64, delta_sigma_c: f64) -> Option<f64> {
    let (delta_d, delta_l) = en1993_cut_off_ranges(delta_sigma_c);
    if delta_sigma >= delta_d {
        Some(2.0e6 * (delta_sigma_c / delta_sigma).powi(3))
    } else if delta_sigma > delta_l {
        Some(5.0e6 * (delta_d / delta_sigma).powi(5))
    } else {
        None
    }
}

/// 🔩️ Palmgren-Miner damage D = Σ n_i / N_i of the spectrum rows (Δσ_i, n_i) at detail category Δσ_C; ranges below the cut-off contribute nothing.
pub fn miner_damage(spectrum: &[(f64, f64)], delta_sigma_c: f64) -> f64 {
    spectrum.iter().filter_map(|(range, cycles)| en1993_endurance_cycles(*range, delta_sigma_c).map(|n| cycles / n)).sum()
}
//#endregion 🔖️PalmgrenMiner

//#region 🔖️TestBasedDesign
const KN_NODES: [f64; 11] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 20.0, 30.0, f64::INFINITY];
const KN_V_KNOWN: [f64; 11] = [2.31, 2.01, 1.89, 1.83, 1.80, 1.77, 1.74, 1.72, 1.68, 1.67, 1.64];
const KN_V_UNKNOWN: [f64; 11] = [f64::NAN, f64::NAN, 3.37, 2.63, 2.33, 2.18, 2.00, 1.92, 1.76, 1.73, 1.64];

/// 📈️ k_n of Table D1 for n test results; values between table nodes are interpolated linearly in 1/n, `None` for V unknown with n < 3.
pub fn k_n(n: f64, v_known: bool) -> Option<f64> {
    let table = if v_known { &KN_V_KNOWN } else { &KN_V_UNKNOWN };
    if n < 1.0 || (!v_known && n < 3.0) {
        return None;
    }
    if let Some(node) = KN_NODES.iter().position(|candidate| *candidate == n) {
        return Some(table[node]);
    }
    let inv = if n.is_finite() { 1.0 / n } else { 0.0 };
    for window in 0..KN_NODES.len() - 1 {
        let (n0, n1) = (KN_NODES[window], KN_NODES[window + 1]);
        if n >= n0 && n <= n1 {
            let (i0, i1) = (1.0 / n0, if n1.is_finite() { 1.0 / n1 } else { 0.0 });
            let t = if (i0 - i1).abs() < 1e-15 { 0.0 } else { (inv - i0) / (i1 - i0) };
            return Some(table[window] + t * (table[window + 1] - table[window]));
        }
    }
    Some(table[KN_NODES.len() - 1])
}

const KD_V_KNOWN: [f64; 11] = [4.36, 3.77, 3.56, 3.44, 3.37, 3.33, 3.27, 3.23, 3.16, 3.13, 3.04];

/// 🧪️ k_d,n of Table D2 (fractile factor of the design value, V_X known) for n test results, interpolated linearly in 1/n between the table nodes of Table D1; `None` for V unknown (that row is not transcribed) and for n < 1. Row recalled from the standard, unverified against the printed table; the closed form u_(α_R β) √(1 + 1/n) with α_R β = 0.8 · 3.8 bounds it from below within 0.07.
pub fn k_d_n(n: f64, v_known: bool) -> Option<f64> {
    if !v_known || n < 1.0 {
        return None;
    }
    let inv = if n.is_finite() { 1.0 / n } else { 0.0 };
    let node = KN_NODES.iter().position(|candidate| *candidate == n);
    if let Some(node) = node {
        return Some(KD_V_KNOWN[node]);
    }
    let window = KN_NODES.windows(2).position(|pair| n >= pair[0] && n <= pair[1])?;
    let (i0, i1) = (1.0 / KN_NODES[window], if KN_NODES[window + 1].is_finite() { 1.0 / KN_NODES[window + 1] } else { 0.0 });
    Some(KD_V_KNOWN[window] + (inv - i0) / (i1 - i0) * (KD_V_KNOWN[window + 1] - KD_V_KNOWN[window]))
}

/// 🧪️ The result of the Annex D.7.2 standard evaluation (method a) of a resistance function against test results.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardEvaluation {
    pub b: f64,
    pub v_delta: f64,
    pub v_r: f64,
    pub k_n: f64,
    pub k_infinity: f64,
    pub r_k: f64,
}

/// 🧪️ Annex D.7.2 standard evaluation: with the test results `(r_e, r_t)` (experimental and theoretical resistance of the same specimen), b = Σ r_e r_t / Σ r_t², δ_i = r_e,i / (b r_t,i), the log-error spread Q_δ = s(ln δ_i) (V_δ² = exp(s²) − 1), the model scatter V_rt of the basic variables, Q² = Q_δ² + ln(V_rt² + 1) and the characteristic resistance r_k = b g_rt(X_m) exp(−k_∞ α_rt Q_rt − k_n α_δ Q_δ − Q²/2) with k_n of Table D1 (V unknown) and k_∞ its last node; `Err` for fewer than 3 results, non-positive values or a degenerate fit. Formula transcribed from memory of D.7.2, unverified against the printed standard.
pub fn standard_evaluation(results: &[(f64, f64)], v_rt: f64, g_rt_mean: f64) -> Result<StandardEvaluation, String> {
    let n = results.len();
    if n < 3 || results.iter().any(|(e, t)| !(e.is_finite() && t.is_finite() && *e > 0.0 && *t > 0.0)) || !(v_rt.is_finite() && v_rt >= 0.0 && g_rt_mean.is_finite() && g_rt_mean > 0.0) {
        return Err("the standard evaluation needs at least 3 positive (r_e, r_t) pairs, V_rt ≥ 0 and a positive g_rt(X_m)".into());
    }
    let b = results.iter().map(|(e, t)| e * t).sum::<f64>() / results.iter().map(|(_, t)| t * t).sum::<f64>();
    let deltas: Vec<f64> = results.iter().map(|(e, t)| (e / (b * t)).ln()).collect();
    let mean = deltas.iter().sum::<f64>() / n as f64;
    let q_delta = (deltas.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n as f64 - 1.0)).sqrt();
    let q_rt = (v_rt * v_rt).ln_1p().sqrt();
    let q = (q_delta * q_delta + q_rt * q_rt).sqrt();
    let k_n_value = k_n(n as f64, false).ok_or("Table D1 has no k_n for this n")?;
    let k_infinity = k_n(f64::INFINITY, false).ok_or("Table D1 has no k_infinity")?;
    let exponent = if q > 0.0 { -k_infinity * q_rt * q_rt / q - k_n_value * q_delta * q_delta / q - 0.5 * q * q } else { 0.0 };
    Ok(StandardEvaluation { b, v_delta: (q_delta * q_delta).exp_m1().sqrt(), v_r: (q * q).exp_m1().sqrt(), k_n: k_n_value, k_infinity, r_k: b * g_rt_mean * exponent.exp() })
}
//#endregion 🔖️TestBasedDesign

//#region 🔖️DocumentAnalyses
/// 🔩️ Palmgren-Miner damage of spectrum bins `(Δσ, n, Δσ_C)`, each bin on the S-N curve of its own detail category; `Err` for a bin that is not a finite non-negative range and count at a positive category.
pub fn spectrum_damage(bins: &[(f64, f64, f64)]) -> Result<f64, String> {
    if let Some((range, cycles, category)) = bins.iter().find(|(range, cycles, category)| !(range.is_finite() && cycles.is_finite() && category.is_finite() && *range >= 0.0 && *cycles >= 0.0 && *category > 0.0)) {
        return Err(format!("spectrum bin (Δσ = {range}, n = {cycles}, Δσ_C = {category}) needs finite non-negative Δσ and n and a positive Δσ_C"));
    }
    Ok(bins.iter().filter_map(|(range, cycles, category)| en1993_endurance_cycles(*range, *category).map(|endurance| cycles / endurance)).sum())
}

/// 🔩️ Index of the spectrum bin `(Δσ, n, Δσ_C)` with the largest damage share n / N; `None` for an empty spectrum.
pub fn governing_spectrum_bin(bins: &[(f64, f64, f64)]) -> Option<usize> {
    let share = |bin: &(f64, f64, f64)| en1993_endurance_cycles(bin.0, bin.2).map_or(0.0, |endurance| bin.1 / endurance);
    (0..bins.len()).max_by(|a, b| share(&bins[*a]).total_cmp(&share(&bins[*b])))
}

/// 🔩️ The smallest detail category Δσ_C of bin `index` (all other bins kept) with a spectrum damage D ≤ 1, by bisection between the current category and the category 2.5 Δσ above which the bin lies below the cut-off; `None` when the other bins alone exceed 1.
pub fn required_detail_category(bins: &[(f64, f64, f64)], index: usize) -> Option<f64> {
    let damage_at = |category: f64| {
        let mut changed = bins.to_vec();
        changed[index].2 = category;
        spectrum_damage(&changed).ok()
    };
    let (mut low, mut high) = (bins[index].2, (bins[index].0 * 2.5).max(bins[index].2));
    if !damage_at(high).is_some_and(|damage| damage <= 1.0) {
        return None;
    }
    for _ in 0..80 {
        let middle = 0.5 * (low + high);
        if damage_at(middle).is_some_and(|damage| damage <= 1.0) { high = middle } else { low = middle }
    }
    Some(high)
}

/// 🔩️ The largest cycle count n of bin `index` (all other bins kept) with a spectrum damage D ≤ 1, a margin of 10⁻⁹ below the exact value; `None` when the other bins alone reach 1 or the bin lies below the cut-off.
pub fn allowed_spectrum_cycles(bins: &[(f64, f64, f64)], index: usize) -> Option<f64> {
    let mut others = bins.to_vec();
    others[index].1 = 0.0;
    let rest = spectrum_damage(&others).ok()?;
    let endurance = en1993_endurance_cycles(bins[index].0, bins[index].2)?;
    (rest < 1.0).then(|| (1.0 - rest) * endurance * (1.0 - 1.0e-9))
}

/// 🧮️ The exact failure probability P_f = P(R < E) = ∫ f_E(x) F_R(x) dx of two independent basic variables by composite Simpson quadrature between the 10⁻¹⁰ and 1 − 10⁻¹⁰ fractiles of the action; the benchmark of the FORM linearisation.
pub fn exact_failure_probability(resistance: &Dist, action: &Dist) -> f64 {
    const INTERVALS: usize = 20_000;
    let (low, high) = (action.fractile(1.0e-10), action.fractile(1.0 - 1.0e-10));
    let step = (high - low) / INTERVALS as f64;
    let integrand = |x: f64| action.pdf(x) * resistance.cdf(x);
    let inner: f64 = (1..INTERVALS).map(|i| integrand(low + step * i as f64) * if i % 2 == 1 { 4.0 } else { 2.0 }).sum();
    (integrand(low) + inner + integrand(high)) * step / 3.0
}

/// 📈️ The FORM result of g = R − E for two named distributions; `Err` for an unknown distribution, non-positive scatter or a run that does not converge.
pub fn analysis_form(resistance: (&str, f64, f64), action: (&str, f64, f64)) -> Result<FormResult, String> {
    let (r, e) = (Dist::parse(resistance.0, resistance.1, resistance.2)?, Dist::parse(action.0, action.1, action.2)?);
    let form = form_r_minus_e(&r, &e);
    if form.converged && form.beta.is_finite() { Ok(form) } else { Err("FORM did not converge in 100 iterations".into()) }
}

/// 📈️ The FORM result of g = R − ΣE_i for named distributions (Annex C.7 with several actions); `Err` for an unknown distribution, non-positive scatter or a run that does not converge.
pub fn analysis_form_multi(resistance: (&str, f64, f64), actions: &[(&str, f64, f64)]) -> Result<MultiFormResult, String> {
    let r = Dist::parse(resistance.0, resistance.1, resistance.2)?;
    let e: Vec<Dist> = actions.iter().map(|a| Dist::parse(a.0, a.1, a.2)).collect::<Result<_, _>>()?;
    let form = form_multi(&r, &e);
    if form.converged && form.beta.is_finite() { Ok(form) } else { Err(format!("FORM did not converge in {} iterations", form.iterations)) }
}

/// 📈️ The smallest resistance mean (standard deviation and distribution kept) with β ≥ `target`, by doubling and bisection; `None` when no mean up to 2⁴⁰ times the current one reaches it.
pub fn required_resistance_mean(resistance: (&str, f64, f64), action: (&str, f64, f64), target: f64) -> Option<f64> {
    let beta_at = |mean: f64| analysis_form((resistance.0, mean, resistance.2), action).map(|form| form.beta).ok();
    let (mut low, mut high) = (resistance.1, resistance.1);
    for _ in 0..40 {
        if beta_at(high)? >= target {
            break;
        }
        (low, high) = (high, high * 2.0);
    }
    if beta_at(high)? < target {
        return None;
    }
    for _ in 0..60 {
        let middle = 0.5 * (low + high);
        if beta_at(middle).is_some_and(|beta| beta >= target) { high = middle } else { low = middle }
    }
    Some(high)
}

/// 📈️ The smallest resistance mean (standard deviation and distribution kept) with β ≥ `target` of g = R − ΣE_i, by doubling and bisection on the multi-action FORM run; `None` when no mean up to 2⁴⁰ times the current one reaches it.
pub fn required_resistance_mean_multi(resistance: (&str, f64, f64), actions: &[(&str, f64, f64)], target: f64) -> Option<f64> {
    let beta_at = |mean: f64| analysis_form_multi((resistance.0, mean, resistance.2), actions).map(|form| form.beta).ok();
    let (mut low, mut high) = (resistance.1, resistance.1);
    for _ in 0..40 {
        if beta_at(high)? >= target {
            break;
        }
        (low, high) = (high, high * 2.0);
    }
    if beta_at(high)? < target {
        return None;
    }
    for _ in 0..60 {
        let middle = 0.5 * (low + high);
        if beta_at(middle).is_some_and(|beta| beta >= target) { high = middle } else { low = middle }
    }
    Some(high)
}

/// 🧪️ Annex D.7.2 design value X_d = η_d·(m − k_n·s) of a normal quantity, or η_d·exp(m_y − k_n·s_y) of a lognormal one (m_y, s_y from the sample mean and standard deviation); V_X known (> 0) uses the first row of Table D1 with that coefficient of variation; `Err` for invalid statistics or n outside Table D1.
pub fn test_design_value(distribution: &str, sample_size: u32, mean: f64, std_dev: f64, variation_known: f64, conversion_factor: f64) -> Result<f64, String> {
    if !(mean.is_finite() && mean > 0.0 && std_dev.is_finite() && std_dev >= 0.0 && variation_known.is_finite() && variation_known >= 0.0 && conversion_factor.is_finite() && conversion_factor > 0.0) {
        return Err("test statistics need a positive mean, a non-negative standard deviation and V_X and a positive η_d".into());
    }
    let known = variation_known > 0.0;
    let kn = k_n(f64::from(sample_size), known).ok_or_else(|| format!("Table D1 has no k_n for n = {sample_size} with V_X {}", if known { "known" } else { "unknown" }))?;
    let variation = if known { variation_known } else { std_dev / mean };
    match distribution {
        "normal" => Ok(conversion_factor * mean * (1.0 - kn * variation)),
        "lognormal" => {
            let s_y = (variation * variation).ln_1p().sqrt();
            Ok(conversion_factor * (mean.ln() - 0.5 * s_y * s_y - kn * s_y).exp())
        }
        other => Err(format!("unknown test distribution {other}")),
    }
}
//#endregion 🔖️DocumentAnalyses
