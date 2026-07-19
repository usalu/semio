//! 📏 Probability divergences and distances: KL/JS/Rényi/Tsallis families, classical distances
//! (Hellinger, Bhattacharyya, total variation, chi-square), empirical Wasserstein-1D and energy
//! distance over raw samples, and a closure-based Bregman divergence for arbitrary convex `phi`.

use crate::counts::validate_probabilities;
use crate::numeric::neumaier_sum;
use crate::{EntropyError, LogBase, Tolerances};

// #region 🔖Shared
fn validate_pair(p: &[f64], q: &[f64]) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let q = validate_probabilities(q, Tolerances::default())?;
    Ok((p, q))
}
// #endregion 🔖Shared

// #region 🔖KlFamily
/// 📏 Forward KL divergence `D(p || q) = sum p_i ln(p_i / q_i)`. Mathematically honest: returns
/// `f64::INFINITY` when `p_i > 0` and `q_i == 0` for some `i` (no silent smoothing).
pub fn kl_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let mut nats = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        nats += pi * (pi / qi).ln();
    }
    Ok(base.from_nats(nats))
}

/// 📏 Reverse KL divergence `D(q || p)`.
pub fn reverse_kl_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    kl_divergence(q, p, base)
}

/// 📏 Jeffreys divergence, the symmetrized KL: `D(p||q) + D(q||p)`.
pub fn jeffreys_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    let a = kl_divergence(p, q, base)?;
    let b = kl_divergence(q, p, base)?;
    Ok(a + b)
}
// #endregion 🔖KlFamily

// #region 🔖JensenFamily
/// 📏 Jensen-Shannon divergence: `0.5*D(p||m) + 0.5*D(q||m)` with `m = 0.5*(p+q)`. Always finite
/// and bounded by `ln(2)` in nats regardless of support overlap.
pub fn js_divergence(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let m: Vec<f64> = p.iter().zip(q.iter()).map(|(&pi, &qi)| 0.5 * (pi + qi)).collect();
    let d_pm = kl_divergence(&p, &m, LogBase::Nats)?;
    let d_qm = kl_divergence(&q, &m, LogBase::Nats)?;
    Ok(base.from_nats(0.5 * d_pm + 0.5 * d_qm))
}

/// 📏 Jensen-Shannon distance, the square root of [`js_divergence`] in nats (a true metric).
pub fn js_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    Ok(js_divergence(p, q, LogBase::Nats)?.sqrt())
}

/// 📏 Weighted Jensen-Shannon divergence with mixture weight `pi_p` for `p` (`pi_q = 1 - pi_p`).
pub fn weighted_js_divergence(p: &[f64], q: &[f64], pi_p: f64, base: LogBase) -> Result<f64, EntropyError> {
    if !(0.0..=1.0).contains(&pi_p) {
        return Err(EntropyError::InvalidConfig { field: "pi_p", reason: "must be in [0, 1]" });
    }
    base.validate()?;
    let (p, q) = validate_pair(p, q)?;
    let m: Vec<f64> = p.iter().zip(q.iter()).map(|(&pi, &qi)| pi_p * pi + (1.0 - pi_p) * qi).collect();
    let d_pm = kl_divergence(&p, &m, LogBase::Nats)?;
    let d_qm = kl_divergence(&q, &m, LogBase::Nats)?;
    Ok(base.from_nats(pi_p * d_pm + (1.0 - pi_p) * d_qm))
}
// #endregion 🔖JensenFamily

// #region 🔖RenyiTsallis
/// 📏 Rényi divergence of order `alpha != 1`: `D_alpha(p||q) = 1/(alpha-1) * ln(sum p_i^alpha
/// q_i^(1-alpha))`. `alpha == 1` is the KL limit — call [`kl_divergence`] instead.
pub fn renyi_divergence(p: &[f64], q: &[f64], alpha: f64, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if !alpha.is_finite() || alpha <= 0.0 {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
    }
    if (alpha - 1.0).abs() < 1e-12 {
        return Err(EntropyError::UndefinedResult { reason: "Renyi divergence at alpha=1 is the KL limit; call kl_divergence() instead" });
    }
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        sum += pi.powf(alpha) * qi.powf(1.0 - alpha);
    }
    if sum <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum is non-positive" });
    }
    Ok(base.from_nats(sum.ln() / (alpha - 1.0)))
}

/// 📏 Tsallis divergence of entropic index `alpha != 1`: `(sum p_i^alpha q_i^(1-alpha) - 1) /
/// (alpha - 1)`. Unitless (no logarithm base).
pub fn tsallis_divergence(p: &[f64], q: &[f64], alpha: f64) -> Result<f64, EntropyError> {
    if !alpha.is_finite() || alpha <= 0.0 {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
    }
    if (alpha - 1.0).abs() < 1e-12 {
        return kl_divergence(p, q, LogBase::Nats);
    }
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        sum += pi.powf(alpha) * qi.powf(1.0 - alpha);
    }
    Ok((sum - 1.0) / (alpha - 1.0))
}
// #endregion 🔖RenyiTsallis

// #region 🔖ClassicalDistances
/// 📏 Hellinger distance: `sqrt(0.5 * sum (sqrt(p_i) - sqrt(q_i))^2)`, bounded in `[0, 1]`.
pub fn hellinger_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    let sum = neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi.sqrt() - qi.sqrt()).powi(2)));
    Ok((0.5 * sum).max(0.0).sqrt())
}

/// 📏 Bhattacharyya coefficient `BC(p,q) = sum sqrt(p_i q_i)`, in `[0, 1]`.
pub fn bhattacharyya_coefficient(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi * qi).sqrt())).clamp(0.0, 1.0))
}

/// 📏 Bhattacharyya distance `-ln(BC(p,q))`.
pub fn bhattacharyya_distance(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let bc = bhattacharyya_coefficient(p, q)?;
    if bc <= 0.0 {
        return Ok(f64::INFINITY);
    }
    Ok(-bc.ln())
}

/// 📏 Total variation distance: `0.5 * sum |p_i - q_i|`, bounded in `[0, 1]`.
pub fn total_variation(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    Ok(0.5 * neumaier_sum(p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi - qi).abs())))
}

/// 📏 Pearson chi-square divergence: `sum (p_i - q_i)^2 / q_i`.
pub fn chi_square_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    let (p, q) = validate_pair(p, q)?;
    let mut sum = 0.0_f64;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if qi <= 0.0 {
            if pi > 0.0 {
                return Ok(f64::INFINITY);
            }
            continue;
        }
        sum += (pi - qi).powi(2) / qi;
    }
    Ok(sum)
}

/// 📏 Neyman (reverse) chi-square divergence: `sum (p_i - q_i)^2 / p_i`.
pub fn neyman_chi_square_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    chi_square_divergence(q, p)
}
// #endregion 🔖ClassicalDistances

// #region 🔖EmpiricalDistances
/// 📏 Empirical 1-D Wasserstein (earth-mover) distance between two raw sample sets, computed as
/// the area between their empirical CDFs (`integral |F_x(t) - F_y(t)| dt`), which is exact and
/// requires no binning.
pub fn wasserstein_1d(x: &[f64], y: &[f64]) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let mut sx = x.to_vec();
    let mut sy = y.to_vec();
    sx.sort_by(|a, b| a.total_cmp(b));
    sy.sort_by(|a, b| a.total_cmp(b));
    let (nx, ny) = (sx.len() as f64, sy.len() as f64);

    let mut breakpoints: Vec<f64> = sx.iter().chain(sy.iter()).copied().collect();
    breakpoints.sort_by(|a, b| a.total_cmp(b));
    breakpoints.dedup();

    let cdf = |sorted: &[f64], t: f64| -> f64 {
        let idx = match sorted.binary_search_by(|v| v.total_cmp(&t)) {
            Ok(mut i) => {
                while i + 1 < sorted.len() && sorted[i + 1] == t {
                    i += 1;
                }
                i + 1
            }
            Err(i) => i,
        };
        idx as f64
    };

    let mut area = 0.0_f64;
    for w in breakpoints.windows(2) {
        let (a, b) = (w[0], w[1]);
        let fx = cdf(&sx, a) / nx;
        let fy = cdf(&sy, a) / ny;
        area += (fx - fy).abs() * (b - a);
    }
    Ok(area)
}

/// 📏 Szekely-Rizzo energy distance: `2*E|X-Y| - E|X-X'| - E|Y-Y'|`, estimated by the standard
/// `O(n*m)` U-statistic over raw samples.
pub fn energy_distance(x: &[f64], y: &[f64]) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let (n, m) = (x.len(), y.len());
    let cross: f64 = x.iter().map(|&xi| y.iter().map(|&yj| (xi - yj).abs()).sum::<f64>()).sum::<f64>() / (n * m) as f64;
    let within_x: f64 = x.iter().map(|&xi| x.iter().map(|&xj| (xi - xj).abs()).sum::<f64>()).sum::<f64>() / (n * n) as f64;
    let within_y: f64 = y.iter().map(|&yi| y.iter().map(|&yj| (yi - yj).abs()).sum::<f64>()).sum::<f64>() / (m * m) as f64;
    Ok((2.0 * cross - within_x - within_y).max(0.0))
}
// #endregion 🔖EmpiricalDistances

// #region 🔖MatrixDivergences
/// 📏 Log-det (Stein) divergence between two `n x n` covariance-like SPD matrices (row-major):
/// `ln|det(Sigma_q)| - ln|det(Sigma_p)| + tr(Sigma_q^-1 Sigma_p) - n`, computed via Cholesky
/// solves rather than an explicit matrix inverse.
pub fn log_det_divergence(cov_p: &[f64], cov_q: &[f64], n: usize) -> Result<f64, EntropyError> {
    let ld_p = crate::matrix::log_det(cov_p, n)?;
    let ld_q = crate::matrix::log_det(cov_q, n)?;
    let l_q = crate::matrix::cholesky(cov_q, n)?;
    // 🔢 tr(Sigma_q^-1 Sigma_p) via solving L_q L_q^T X = Sigma_p column-by-column, then summing
    // the diagonal of X (forward/backward substitution, no explicit inverse).
    let mut trace = 0.0_f64;
    for col in 0..n {
        let rhs: Vec<f64> = (0..n).map(|row| cov_p[row * n + col]).collect();
        let mut y = vec![0.0_f64; n];
        for i in 0..n {
            let mut sum = rhs[i];
            for k in 0..i {
                sum -= l_q[i * n + k] * y[k];
            }
            y[i] = sum / l_q[i * n + i];
        }
        let mut xcol = vec![0.0_f64; n];
        for i in (0..n).rev() {
            let mut sum = y[i];
            for k in (i + 1)..n {
                sum -= l_q[k * n + i] * xcol[k];
            }
            xcol[i] = sum / l_q[i * n + i];
        }
        trace += xcol[col];
    }
    Ok(ld_q - ld_p + trace - n as f64)
}

/// 📏 Bregman divergence `D_phi(p, q) = phi(p) - phi(q) - grad_phi(q) . (p - q)` for an arbitrary
/// convex `phi`, supplied as closures so callers can instantiate squared-Euclidean, KL
/// (`phi = negentropy`), Itakura-Saito, or any other Bregman generator without a new function.
pub fn bregman_divergence(p: &[f64], q: &[f64], phi: impl Fn(&[f64]) -> f64, grad_phi_q: impl Fn(&[f64]) -> Vec<f64>) -> Result<f64, EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let grad = grad_phi_q(q);
    if grad.len() != p.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: grad.len() });
    }
    let dot: f64 = p.iter().zip(q.iter()).zip(grad.iter()).map(|((&pi, &qi), &gi)| gi * (pi - qi)).sum();
    Ok(phi(p) - phi(q) - dot)
}

/// 📏 Itakura-Saito divergence between two positive spectra: `sum (p_i/q_i - ln(p_i/q_i) - 1)`.
pub fn itakura_saito_divergence(p: &[f64], q: &[f64]) -> Result<f64, EntropyError> {
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let mut sum = 0.0_f64;
    for (i, (&pi, &qi)) in p.iter().zip(q.iter()).enumerate() {
        if pi <= 0.0 || qi <= 0.0 {
            return Err(EntropyError::InvalidProbability { index: i, value: pi.min(qi) });
        }
        let ratio = pi / qi;
        sum += ratio - ratio.ln() - 1.0;
    }
    Ok(sum)
}
// #endregion 🔖MatrixDivergences

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kl_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(kl_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn kl_is_non_negative_for_random_distributions() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        for _ in 0..200 {
            let k = 2 + rng.next_below(5);
            let mut p: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let mut q: Vec<f64> = (0..k).map(|_| rng.next_f64() + 0.01).collect();
            let sp: f64 = p.iter().sum();
            let sq: f64 = q.iter().sum();
            p.iter_mut().for_each(|v| *v /= sp);
            q.iter_mut().for_each(|v| *v /= sq);
            let d = kl_divergence(&p, &q, LogBase::Nats).unwrap();
            assert!(d >= -1e-9, "d={d}");
        }
    }

    #[test]
    fn kl_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(kl_divergence(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn js_divergence_symmetric_and_bounded_by_ln2() {
        let p = [0.9, 0.1];
        let q = [0.1, 0.9];
        let a = js_divergence(&p, &q, LogBase::Nats).unwrap();
        let b = js_divergence(&q, &p, LogBase::Nats).unwrap();
        assert!((a - b).abs() < 1e-9);
        assert!(a <= core::f64::consts::LN_2 + 1e-9);
        assert!(a >= 0.0);
    }

    #[test]
    fn js_of_identical_distributions_is_zero() {
        let p = [0.3, 0.7];
        assert!(js_divergence(&p, &p, LogBase::Bits).unwrap().abs() < 1e-9);
    }

    #[test]
    fn hellinger_distance_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(hellinger_distance(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        let d = hellinger_distance(&p, &q).unwrap();
        assert!((0.0..=1.0).contains(&d));
    }

    #[test]
    fn total_variation_bounds_and_identity() {
        let p = [0.5, 0.5];
        assert!(total_variation(&p, &p).unwrap().abs() < 1e-9);
        let q = [1.0, 0.0];
        assert!((total_variation(&p, &q).unwrap() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn chi_square_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        assert!(chi_square_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    #[test]
    fn renyi_divergence_rejects_alpha_one() {
        assert!(matches!(
            renyi_divergence(&[0.5, 0.5], &[0.3, 0.7], 1.0, LogBase::Bits),
            Err(EntropyError::UndefinedResult { .. })
        ));
    }

    #[test]
    fn renyi_divergence_of_identical_distributions_is_zero() {
        let p = [0.2, 0.3, 0.5];
        let d = renyi_divergence(&p, &p, 2.0, LogBase::Nats).unwrap();
        assert!(d.abs() < 1e-9);
    }

    #[test]
    fn tsallis_divergence_limit_matches_kl() {
        let p = [0.2, 0.3, 0.5];
        let q = [0.3, 0.3, 0.4];
        let kl = kl_divergence(&p, &q, LogBase::Nats).unwrap();
        let tsallis = tsallis_divergence(&p, &q, 1.0).unwrap();
        assert!((kl - tsallis).abs() < 1e-6);
    }

    #[test]
    fn wasserstein_1d_of_identical_samples_is_zero() {
        let x = [1.0, 2.0, 3.0, 4.0];
        assert!(wasserstein_1d(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn wasserstein_1d_matches_hand_computation_equal_size() {
        // 🔐 equal-size sorted samples: W1 = mean |x_sorted - y_sorted|.
        let x = [1.0, 2.0, 3.0];
        let y = [4.0, 5.0, 6.0];
        let w = wasserstein_1d(&x, &y).unwrap();
        assert!((w - 3.0).abs() < 1e-9);
    }

    #[test]
    fn energy_distance_of_identical_distributions_is_near_zero() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(energy_distance(&x, &x).unwrap().abs() < 1e-9);
    }

    #[test]
    fn energy_distance_is_non_negative() {
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..30).map(|_| rng.next_gaussian()).collect();
        let y: Vec<f64> = (0..30).map(|_| rng.next_gaussian() + 2.0).collect();
        assert!(energy_distance(&x, &y).unwrap() > 0.0);
    }

    #[test]
    fn log_det_divergence_of_identical_matrices_is_zero() {
        let cov = vec![4.0, 1.0, 1.0, 3.0];
        let d = log_det_divergence(&cov, &cov, 2).unwrap();
        assert!(d.abs() < 1e-7);
    }

    #[test]
    fn bregman_squared_euclidean_matches_direct_formula() {
        let p = [1.0, 2.0, 3.0];
        let q = [0.5, 2.5, 2.0];
        let phi = |x: &[f64]| x.iter().map(|v| v * v).sum::<f64>();
        let grad = |x: &[f64]| -> Vec<f64> { x.iter().map(|v| 2.0 * v).collect() };
        let d = bregman_divergence(&p, &q, phi, grad).unwrap();
        let expected: f64 = p.iter().zip(q.iter()).map(|(a, b)| (a - b).powi(2)).sum();
        assert!((d - expected).abs() < 1e-9);
    }

    #[test]
    fn itakura_saito_of_identical_spectra_is_zero() {
        let p = [1.0, 2.0, 3.0];
        assert!(itakura_saito_divergence(&p, &p).unwrap().abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_divergence_monotone_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let q = [0.2, 0.3, 0.5];
            let alphas = [0.1, 0.5, 2.0, 5.0];
            let mut prev = renyi_divergence(&p, &q, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let d = renyi_divergence(&p, &q, a, LogBase::Nats).unwrap();
                assert!(d >= prev - 1e-9, "alpha={a}");
                prev = d;
            }
        }
    }
}
// #endregion 🔖Tests
