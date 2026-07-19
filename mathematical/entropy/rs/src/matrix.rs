//! 🔢 Dense linear algebra hand-rolled for matrix-based entropy: a cyclic Jacobi eigensolver for
//! symmetric matrices, one-sided Jacobi SVD, and Cholesky decomposition — feeding SVD entropy,
//! eigenvalue entropy, and von Neumann (density-matrix) entropy.

use crate::counts::validate_probabilities;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Tolerances, Warning};

// #region 🔖Jacobi
/// 🔢 Cyclic Jacobi eigenvalue algorithm for a real symmetric `n x n` matrix (row-major).
/// Returns `(eigenvalues, eigenvectors)` with eigenvalues sorted descending and `eigenvectors`
/// row-major where column `j` is the eigenvector for `eigenvalues[j]`.
pub fn jacobi_eigen_symmetric(a_in: &[f64], n: usize) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a_in.len() != n * n {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: n * n, actual: a_in.len() });
    }
    let mut a = a_in.to_vec();
    let mut v = vec![0.0_f64; n * n];
    for i in 0..n {
        v[i * n + i] = 1.0;
    }
    let mut d = vec![0.0_f64; n];
    for i in 0..n {
        d[i] = a[i * n + i];
    }
    let mut z = vec![0.0_f64; n];

    const MAX_SWEEPS: usize = 100;
    let mut converged = false;
    for sweep in 0..MAX_SWEEPS {
        let mut off_sum = 0.0_f64;
        for p in 0..n.saturating_sub(1) {
            for q in (p + 1)..n {
                off_sum += a[p * n + q].abs();
            }
        }
        if off_sum == 0.0 {
            converged = true;
            break;
        }
        let threshold = if sweep < 3 { 0.2 * off_sum / (n * n) as f64 } else { 0.0 };
        for p in 0..n.saturating_sub(1) {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                let g = 100.0 * apq.abs();
                if sweep > 3 && (d[p].abs() + g == d[p].abs()) && (d[q].abs() + g == d[q].abs()) {
                    a[p * n + q] = 0.0;
                    continue;
                }
                if apq.abs() <= threshold {
                    continue;
                }
                let mut h = d[q] - d[p];
                let t = if h.abs() + g == h.abs() {
                    apq / h
                } else {
                    let theta = 0.5 * h / apq;
                    let t0 = 1.0 / (theta.abs() + (1.0 + theta * theta).sqrt());
                    if theta < 0.0 { -t0 } else { t0 }
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let tau = s / (1.0 + c);
                h = t * apq;
                z[p] -= h;
                z[q] += h;
                d[p] -= h;
                d[q] += h;
                a[p * n + q] = 0.0;

                let mut rotate = |ip: f64, iq: f64| -> (f64, f64) { (ip - s * (iq + ip * tau), iq + s * (ip - iq * tau)) };
                for i in 0..p {
                    let (np, nq) = rotate(a[i * n + p], a[i * n + q]);
                    a[i * n + p] = np;
                    a[i * n + q] = nq;
                }
                for i in (p + 1)..q {
                    let (np, nq) = rotate(a[p * n + i], a[i * n + q]);
                    a[p * n + i] = np;
                    a[i * n + q] = nq;
                }
                for i in (q + 1)..n {
                    let (np, nq) = rotate(a[p * n + i], a[q * n + i]);
                    a[p * n + i] = np;
                    a[q * n + i] = nq;
                }
                for i in 0..n {
                    let (np, nq) = rotate(v[i * n + p], v[i * n + q]);
                    v[i * n + p] = np;
                    v[i * n + q] = nq;
                }
            }
        }
        // 🔢 `d` was already updated incrementally by `-= h` / `+= h` during the sweep above;
        // only the per-sweep accumulator `z` needs resetting for the next sweep.
        for i in 0..n {
            z[i] = 0.0;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "Jacobi eigensolver", iterations: MAX_SWEEPS });
    }

    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| d[j].partial_cmp(&d[i]).unwrap_or(std::cmp::Ordering::Equal));
    let eigenvalues: Vec<f64> = idx.iter().map(|&i| d[i]).collect();
    let mut eigenvectors = vec![0.0_f64; n * n];
    for (new_col, &old_col) in idx.iter().enumerate() {
        for row in 0..n {
            eigenvectors[row * n + new_col] = v[row * n + old_col];
        }
    }
    Ok((eigenvalues, eigenvectors))
}
// #endregion 🔖Jacobi

// #region 🔖Cholesky
/// 🔢 Cholesky decomposition `A = L L^T` of a symmetric positive-(semi)definite `n x n` matrix
/// (row-major), returning the lower-triangular factor `L` (row-major, zeros above the diagonal).
/// Falls back to progressively larger diagonal (Tikhonov) regularization if a pivot is
/// non-positive due to floating-point noise near singularity.
pub fn cholesky(a: &[f64], n: usize) -> Result<Vec<f64>, EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a.len() != n * n {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: n * n, actual: a.len() });
    }
    let mut jitter = 0.0_f64;
    for _attempt in 0..12 {
        let mut l = vec![0.0_f64; n * n];
        let mut ok = true;
        'outer: for i in 0..n {
            for j in 0..=i {
                let mut sum = a[i * n + j] + if i == j { jitter } else { 0.0 };
                for k in 0..j {
                    sum -= l[i * n + k] * l[j * n + k];
                }
                if i == j {
                    if sum <= 0.0 {
                        ok = false;
                        break 'outer;
                    }
                    l[i * n + i] = sum.sqrt();
                } else {
                    l[i * n + j] = sum / l[j * n + j];
                }
            }
        }
        if ok {
            return Ok(l);
        }
        jitter = if jitter == 0.0 { 1e-12 } else { jitter * 10.0 };
    }
    Err(EntropyError::UndefinedResult { reason: "matrix is not positive-definite even after regularization" })
}

/// 🔢 `ln|det(A)|` of a symmetric positive-definite matrix via `2 * sum(ln(L_ii))` from its
/// Cholesky factor.
pub fn log_det(a: &[f64], n: usize) -> Result<f64, EntropyError> {
    let l = cholesky(a, n)?;
    Ok(2.0 * (0..n).map(|i| l[i * n + i].ln()).sum::<f64>())
}
// #endregion 🔖Cholesky

// #region 🔖Svd
/// 🔢 One-sided Jacobi SVD of an `rows x cols` matrix (row-major) with `rows >= cols`. Returns
/// `(u, singular_values, v)`: `u` is `rows x cols` (row-major, orthonormal columns),
/// `singular_values` is sorted descending, `v` is `cols x cols` (row-major, orthogonal).
pub fn svd_jacobi(a: &[f64], rows: usize, cols: usize) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), EntropyError> {
    if rows == 0 || cols == 0 {
        return Err(EntropyError::EmptyInput { what: "matrix" });
    }
    if a.len() != rows * cols {
        return Err(EntropyError::ShapeMismatch { what: "matrix", expected: rows * cols, actual: a.len() });
    }
    if rows < cols {
        return Err(EntropyError::InvalidConfig { field: "rows", reason: "svd_jacobi requires rows >= cols" });
    }
    let mut u = a.to_vec();
    let mut v = vec![0.0_f64; cols * cols];
    for i in 0..cols {
        v[i * cols + i] = 1.0;
    }

    let col = |m: &[f64], stride: usize, c: usize| -> Vec<f64> { (0..rows).map(|r| m[r * stride + c]).collect() };

    const MAX_SWEEPS: usize = 60;
    let mut converged = false;
    for _sweep in 0..MAX_SWEEPS {
        let mut max_gamma = 0.0_f64;
        for p in 0..cols.saturating_sub(1) {
            for q in (p + 1)..cols {
                let col_p = col(&u, cols, p);
                let col_q = col(&u, cols, q);
                let alpha: f64 = col_p.iter().map(|x| x * x).sum();
                let beta: f64 = col_q.iter().map(|x| x * x).sum();
                let gamma: f64 = col_p.iter().zip(col_q.iter()).map(|(&x, &y)| x * y).sum();
                let norm = (alpha * beta).sqrt().max(1e-300);
                max_gamma = max_gamma.max(gamma.abs() / norm);
                if gamma.abs() < 1e-15 * norm {
                    continue;
                }
                let zeta = (beta - alpha) / (2.0 * gamma);
                let t = if zeta >= 0.0 {
                    1.0 / (zeta + (1.0 + zeta * zeta).sqrt())
                } else {
                    -1.0 / (-zeta + (1.0 + zeta * zeta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for r in 0..rows {
                    let up = u[r * cols + p];
                    let uq = u[r * cols + q];
                    u[r * cols + p] = c * up - s * uq;
                    u[r * cols + q] = s * up + c * uq;
                }
                for r in 0..cols {
                    let vp = v[r * cols + p];
                    let vq = v[r * cols + q];
                    v[r * cols + p] = c * vp - s * vq;
                    v[r * cols + q] = s * vp + c * vq;
                }
            }
        }
        if max_gamma < 1e-13 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "Jacobi SVD", iterations: MAX_SWEEPS });
    }

    let mut singular_values: Vec<f64> = (0..cols).map(|c| col(&u, cols, c).iter().map(|x| x * x).sum::<f64>().sqrt()).collect();
    let mut idx: Vec<usize> = (0..cols).collect();
    idx.sort_by(|&i, &j| singular_values[j].partial_cmp(&singular_values[i]).unwrap_or(std::cmp::Ordering::Equal));

    let mut u_sorted = vec![0.0_f64; rows * cols];
    let mut v_sorted = vec![0.0_f64; cols * cols];
    let mut sv_sorted = vec![0.0_f64; cols];
    for (new_c, &old_c) in idx.iter().enumerate() {
        sv_sorted[new_c] = singular_values[old_c];
        let sigma = singular_values[old_c].max(1e-300);
        for r in 0..rows {
            u_sorted[r * cols + new_c] = u[r * cols + old_c] / sigma;
        }
        for r in 0..cols {
            v_sorted[r * cols + new_c] = v[r * cols + old_c];
        }
    }
    singular_values = sv_sorted;
    Ok((u_sorted, singular_values, v_sorted))
}
// #endregion 🔖Svd

// #region 🔖Entropy
/// 🔢 Shannon entropy of the normalized singular-value spectrum of `data` (`rows x cols`,
/// row-major), a measure of the matrix's effective rank / concentration of variance.
pub fn svd_entropy(data: &[f64], rows: usize, cols: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (transposed, r, c) = if rows >= cols {
        (data.to_vec(), rows, cols)
    } else {
        let mut t = vec![0.0_f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j * rows + i] = data[i * cols + j];
            }
        }
        (t, cols, rows)
    };
    let (_, singular_values, _) = svd_jacobi(&transposed, r, c)?;
    let sum: f64 = singular_values.iter().sum();
    if sum <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "all singular values are zero" });
    }
    let p: Vec<f64> = singular_values.iter().map(|&s| s / sum).collect();
    let h = crate::discrete::entropy(&p, base)?;
    let nats = base.to_nats(h);
    let effective_rank = nats.exp();
    let stable_rank = singular_values.iter().map(|s| s * s).sum::<f64>() / singular_values[0].max(1e-300).powi(2);

    Ok(Estimate {
        value: h,
        base,
        method: "svd_entropy",
        n: r.min(c),
        n_effective: effective_rank,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("effective_rank", effective_rank), ("stable_rank", stable_rank)],
    })
}

/// 🔢 Von Neumann entropy `-tr(rho ln rho)` of a symmetric positive-semidefinite density matrix
/// `rho` (row-major `n x n`, trace approximately `1`). Eigenvalues within `n * eps * max|lambda|`
/// of zero are clipped; further-negative eigenvalues are rejected as not positive-semidefinite.
pub fn von_neumann_entropy(density: &[f64], n: usize, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let (eigenvalues, _) = jacobi_eigen_symmetric(density, n)?;
    let max_abs = eigenvalues.iter().cloned().fold(0.0_f64, |acc, v| acc.max(v.abs()));
    let tol_neg = n as f64 * f64::EPSILON * max_abs;
    let mut warnings = Vec::new();
    let mut clipped = Vec::with_capacity(n);
    for &lambda in &eigenvalues {
        if lambda < 0.0 {
            if lambda >= -tol_neg {
                clipped.push(0.0);
                warnings.push(Warning::ClippedNegative);
            } else {
                return Err(EntropyError::UndefinedResult { reason: "density matrix is not positive-semidefinite beyond numerical tolerance" });
            }
        } else {
            clipped.push(lambda);
        }
    }
    let p = validate_probabilities(&clipped, Tolerances::default())?;
    let h = crate::discrete::entropy(&p, base)?;
    let rank = p.iter().filter(|&&v| v > 1e-12).count();

    Ok(Estimate {
        value: h,
        base,
        method: "von_neumann",
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("rank", rank as f64)],
    })
}
// #endregion 🔖Entropy

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn matmul(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for t in 0..k {
                    sum += a[i * k + t] * b[t * n + j];
                }
                out[i * n + j] = sum;
            }
        }
        out
    }

    fn transpose(a: &[f64], rows: usize, cols: usize) -> Vec<f64> {
        let mut out = vec![0.0_f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                out[j * rows + i] = a[i * cols + j];
            }
        }
        out
    }

    #[test]
    fn jacobi_matches_diagonal_matrix_eigenvalues() {
        let n = 3;
        let a = vec![5.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 9.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, n).unwrap();
        assert!((eigenvalues[0] - 9.0).abs() < 1e-9);
        assert!((eigenvalues[1] - 5.0).abs() < 1e-9);
        assert!((eigenvalues[2] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn jacobi_reconstructs_symmetric_matrix() {
        let n = 3;
        let a = vec![4.0, 1.0, 2.0, 1.0, 3.0, 0.5, 2.0, 0.5, 5.0];
        let (eigenvalues, eigenvectors) = jacobi_eigen_symmetric(&a, n).unwrap();
        let mut d = vec![0.0_f64; n * n];
        for i in 0..n {
            d[i * n + i] = eigenvalues[i];
        }
        let vt = transpose(&eigenvectors, n, n);
        let reconstructed = matmul(&matmul(&eigenvectors, &d, n, n, n), &vt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-7, "index {i}: {} vs {}", reconstructed[i], a[i]);
        }
    }

    #[test]
    fn jacobi_hand_3x3_matches_known_eigenvalues() {
        // 🔐 A = [[2,-1,0],[-1,2,-1],[0,-1,2]] has eigenvalues 2, 2±sqrt(2).
        let a = vec![2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0];
        let (eigenvalues, _) = jacobi_eigen_symmetric(&a, 3).unwrap();
        let expected = {
            let mut v = vec![2.0 + 2.0_f64.sqrt(), 2.0, 2.0 - 2.0_f64.sqrt()];
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        for (a, b) in eigenvalues.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-8);
        }
    }

    #[test]
    fn cholesky_reconstructs_positive_definite_matrix() {
        let n = 3;
        let a = vec![4.0, 2.0, 2.0, 2.0, 5.0, 1.0, 2.0, 1.0, 6.0];
        let l = cholesky(&a, n).unwrap();
        let lt = transpose(&l, n, n);
        let reconstructed = matmul(&l, &lt, n, n, n);
        for i in 0..n * n {
            assert!((reconstructed[i] - a[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn cholesky_regularizes_near_singular_matrix() {
        let a = vec![1.0, 1.0, 1.0, 1.0]; // rank-1, singular
        let result = cholesky(&a, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn log_det_matches_known_determinant() {
        let a = vec![4.0, 0.0, 0.0, 9.0];
        let ld = log_det(&a, 2).unwrap();
        assert!((ld - 36.0_f64.ln()).abs() < 1e-9);
    }

    #[test]
    fn svd_reconstructs_matrix() {
        let rows = 4;
        let cols = 3;
        let a = vec![
            1.0, 0.0, 0.0, //
            0.0, 2.0, 0.0, //
            0.0, 0.0, 3.0, //
            1.0, 1.0, 1.0,
        ];
        let (u, s, v) = svd_jacobi(&a, rows, cols).unwrap();
        let mut sigma = vec![0.0_f64; cols * cols];
        for i in 0..cols {
            sigma[i * cols + i] = s[i];
        }
        let vt = transpose(&v, cols, cols);
        let reconstructed = matmul(&matmul(&u, &sigma, rows, cols, cols), &vt, rows, cols, cols);
        for i in 0..rows * cols {
            assert!((reconstructed[i] - a[i]).abs() < 1e-6, "index {i}");
        }
    }

    #[test]
    fn svd_entropy_of_equal_singular_values_is_maximal() {
        // 🔐 identity-like: all singular values equal -> normalized entropy = 1.
        let a = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let est = svd_entropy(&a, 3, 3, LogBase::Bits).unwrap();
        assert!((est.value - 3.0_f64.log2()).abs() < 1e-6);
    }

    #[test]
    fn svd_entropy_of_rank_one_matrix_is_zero() {
        let a = vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0]; // rank 1, 2x3
        let est = svd_entropy(&a, 2, 3, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-6);
    }

    #[test]
    fn von_neumann_entropy_of_pure_state_is_zero() {
        let density = vec![1.0, 0.0, 0.0, 0.0];
        let est = von_neumann_entropy(&density, 2, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn von_neumann_entropy_of_maximally_mixed_state_is_log_n() {
        let n = 4;
        let mut density = vec![0.0_f64; n * n];
        for i in 0..n {
            density[i * n + i] = 1.0 / n as f64;
        }
        let est = von_neumann_entropy(&density, n, LogBase::Nats).unwrap();
        assert!((est.value - (n as f64).ln()).abs() < 1e-9);
    }

    #[test]
    fn von_neumann_clips_tiny_negative_eigenvalues() {
        let density = vec![0.5 + 1e-16, 0.5, 0.5, 0.5 - 1e-16];
        // 🔐 near-singular; should not error, should clip.
        let result = von_neumann_entropy(&density, 2, LogBase::Nats);
        assert!(result.is_ok());
    }
}
// #endregion 🔖Tests
