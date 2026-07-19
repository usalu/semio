//! 📊 Statistical inference: descriptive moments, correlation structure, OLS/logistic regression, hypothesis tests, and discrete information measures.

use mathematical_algebra::{MatD, VecD};
use mathematical_probability::{ChiSquared, Continuous, Normal, StudentT};
use mathematical_tabular::Table;
use std::collections::HashMap;

// #region 🔖Error
/// ⚠️ Fallible-computation error type shared by every function in this crate.
#[derive(Debug, thiserror::Error)]
pub enum StatisticsError {
    #[error("need at least {needed} observations, found {found}")]
    InsufficientData { needed: usize, found: usize },
    #[error("dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch { expected: usize, found: usize },
    #[error("singular matrix")]
    SingularMatrix,
    #[error("no convergence after {iterations} iterations")]
    NoConvergence { iterations: usize },
    #[error("invalid argument: {0}")]
    InvalidArgument(&'static str),
    #[error(transparent)]
    Tabular(#[from] mathematical_tabular::TabularError),
    #[error(transparent)]
    Probability(#[from] mathematical_probability::ProbabilityError),
}
// #endregion 🔖Error

// #region 🔖Descriptive
pub fn mean(values: &[f64]) -> Result<f64, StatisticsError> {
    if values.is_empty() {
        return Err(StatisticsError::InsufficientData { needed: 1, found: 0 });
    }
    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn variance(values: &[f64]) -> Result<f64, StatisticsError> {
    if values.len() < 2 {
        return Err(StatisticsError::InsufficientData { needed: 2, found: values.len() });
    }
    let m = mean(values)?;
    let ss: f64 = values.iter().map(|x| (x - m) * (x - m)).sum();
    Ok(ss / (values.len() as f64 - 1.0))
}

pub fn std_dev(values: &[f64]) -> Result<f64, StatisticsError> {
    Ok(variance(values)?.sqrt())
}

pub fn covariance(x: &[f64], y: &[f64]) -> Result<f64, StatisticsError> {
    if x.len() != y.len() {
        return Err(StatisticsError::DimensionMismatch { expected: x.len(), found: y.len() });
    }
    if x.len() < 2 {
        return Err(StatisticsError::InsufficientData { needed: 2, found: x.len() });
    }
    let mx = mean(x)?;
    let my = mean(y)?;
    let ss: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    Ok(ss / (x.len() as f64 - 1.0))
}

/// 📈 Pearson correlation; errors if either column has zero variance.
pub fn correlation(x: &[f64], y: &[f64]) -> Result<f64, StatisticsError> {
    let cov = covariance(x, y)?;
    let sx = std_dev(x)?;
    let sy = std_dev(y)?;
    if sx == 0.0 || sy == 0.0 {
        return Err(StatisticsError::InvalidArgument("zero-variance column has no correlation"));
    }
    Ok(cov / (sx * sy))
}
// #endregion 🔖Descriptive

// #region 🔖Matrices
pub fn covariance_matrix(columns: &[&[f64]]) -> Result<MatD, StatisticsError> {
    let p = columns.len();
    let mut m = MatD::zeros(p, p);
    for i in 0..p {
        for j in i..p {
            let c = covariance(columns[i], columns[j])?;
            m.set(i, j, c);
            m.set(j, i, c);
        }
    }
    Ok(m)
}

pub fn correlation_matrix(columns: &[&[f64]]) -> Result<MatD, StatisticsError> {
    let p = columns.len();
    let mut m = MatD::zeros(p, p);
    for i in 0..p {
        m.set(i, i, 1.0);
        for j in (i + 1)..p {
            let c = correlation(columns[i], columns[j])?;
            m.set(i, j, c);
            m.set(j, i, c);
        }
    }
    Ok(m)
}

/// 📈 Complete-case correlation matrix over the given table columns; returns the matrix alongside
/// the effective row count (post complete-case filtering) that Fisher-z tests need.
pub fn correlation_from_table(table: &Table, columns: &[usize]) -> Result<(MatD, usize), StatisticsError> {
    let complete = table.complete_rows(columns)?;
    let n = complete.len();
    let series: Vec<Vec<f64>> = columns
        .iter()
        .map(|&c| -> Result<Vec<f64>, StatisticsError> {
            let full = table.continuous(c)?;
            Ok(complete.iter().map(|&row| full[row]).collect())
        })
        .collect::<Result<_, _>>()?;
    let refs: Vec<&[f64]> = series.iter().map(Vec::as_slice).collect();
    Ok((correlation_matrix(&refs)?, n))
}
// #endregion 🔖Matrices

// #region 🔖Partial
/// 🔄 Matrix inverse via `lu_solve` against each identity column.
pub fn invert(matrix: &MatD) -> Result<MatD, StatisticsError> {
    if matrix.rows != matrix.cols {
        return Err(StatisticsError::DimensionMismatch { expected: matrix.rows, found: matrix.cols });
    }
    let n = matrix.rows;
    let mut inv = MatD::zeros(n, n);
    for col in 0..n {
        let mut e = VecD::zeros(n);
        e.set(col, 1.0);
        let x = matrix.lu_solve(&e).ok_or(StatisticsError::SingularMatrix)?;
        for row in 0..n {
            inv.set(row, col, x.get(row));
        }
    }
    Ok(inv)
}

/// 🔗 Partial correlation of `i` and `j` given a conditioning set, via the precision matrix of the
/// submatrix restricted to `{i, j} ∪ given`: `r_ij.given = -P_ij / sqrt(P_ii * P_jj)`.
pub fn partial_correlation(corr: &MatD, i: usize, j: usize, given: &[usize]) -> Result<f64, StatisticsError> {
    let mut idx = vec![i, j];
    idx.extend_from_slice(given);
    let k = idx.len();
    let mut sub = MatD::zeros(k, k);
    for (a, &ia) in idx.iter().enumerate() {
        for (b, &ib) in idx.iter().enumerate() {
            sub.set(a, b, corr.get(ia, ib));
        }
    }
    let prec = invert(&sub)?;
    let (p00, p11, p01) = (prec.get(0, 0), prec.get(1, 1), prec.get(0, 1));
    if p00 <= 0.0 || p11 <= 0.0 {
        return Err(StatisticsError::SingularMatrix);
    }
    Ok(-p01 / (p00 * p11).sqrt())
}
// #endregion 🔖Partial

// #region 🔖Ols
/// 📐 Ordinary least squares fit. Internally solved via normal equations `(XᵀX)β = Xᵀy` and
/// `mathematical_algebra::MatD::lu_solve` — adequate at causal-discovery scale (small `p`, modest
/// condition numbers); swap to Householder QR internally if that ever becomes a bottleneck, the
/// public API here would not need to change.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LinearFit {
    pub coefficients: Vec<f64>,
    pub std_errors: Vec<f64>,
    pub residuals: Vec<f64>,
    pub r_squared: f64,
    pub sigma2: f64,
    pub dof: usize,
}

pub fn ols(x: &MatD, y: &[f64], intercept: bool) -> Result<LinearFit, StatisticsError> {
    let n = x.rows;
    if y.len() != n {
        return Err(StatisticsError::DimensionMismatch { expected: n, found: y.len() });
    }
    let p = x.cols + usize::from(intercept);
    if n <= p {
        return Err(StatisticsError::InsufficientData { needed: p + 1, found: n });
    }
    let offset = usize::from(intercept);
    let mut design = MatD::zeros(n, p);
    for row in 0..n {
        if intercept {
            design.set(row, 0, 1.0);
        }
        for col in 0..x.cols {
            design.set(row, col + offset, x.get(row, col));
        }
    }
    let yv = VecD::from_vec(y.to_vec());
    let design_t = design.transpose();
    let xtx = design_t.matmul(&design);
    let xty = design_t.mul_vec(&yv);
    let beta = xtx.lu_solve(&xty).ok_or(StatisticsError::SingularMatrix)?;
    let fitted: Vec<f64> = (0..n).map(|row| (0..p).map(|col| design.get(row, col) * beta.get(col)).sum()).collect();
    let residuals: Vec<f64> = (0..n).map(|row| y[row] - fitted[row]).collect();
    let ss_res: f64 = residuals.iter().map(|r| r * r).sum();
    let y_mean = mean(y)?;
    let ss_tot: f64 = y.iter().map(|v| (v - y_mean).powi(2)).sum();
    let r_squared = if ss_tot > 0.0 { 1.0 - ss_res / ss_tot } else { 1.0 };
    let dof = n - p;
    let sigma2 = ss_res / dof as f64;
    let xtx_inv = invert(&xtx)?;
    let std_errors: Vec<f64> = (0..p).map(|i| (sigma2 * xtx_inv.get(i, i)).max(0.0).sqrt()).collect();
    Ok(LinearFit { coefficients: beta.0, std_errors, residuals, r_squared, sigma2, dof })
}
// #endregion 🔖Ols

// #region 🔖Logistic
/// 📐 Logistic fit via iteratively reweighted least squares (IRLS): `β ← β + (XᵀWX)⁻¹Xᵀ(y − μ)`,
/// `W = diag(μ(1−μ))` clamped to `≥ 1e-10` to avoid weight collapse near separation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogisticFit {
    pub coefficients: Vec<f64>,
    pub std_errors: Vec<f64>,
    pub log_likelihood: f64,
    pub iterations: usize,
}

fn logistic_design(x: &MatD, intercept: bool) -> MatD {
    let n = x.rows;
    let p = x.cols + usize::from(intercept);
    let offset = usize::from(intercept);
    let mut design = MatD::zeros(n, p);
    for row in 0..n {
        if intercept {
            design.set(row, 0, 1.0);
        }
        for col in 0..x.cols {
            design.set(row, col + offset, x.get(row, col));
        }
    }
    design
}

#[allow(clippy::needless_range_loop, reason = "each loop body indexes both a MatD by (row, col) and one or more parallel Vec<f64> by row; enumerate() only removes the Vec index and would leave the MatD access no clearer")]
pub fn logistic(x: &MatD, y: &[f64], intercept: bool) -> Result<LogisticFit, StatisticsError> {
    const MAX_ITER: usize = 50;
    const TOL: f64 = 1e-8;
    let n = x.rows;
    if y.len() != n {
        return Err(StatisticsError::DimensionMismatch { expected: n, found: y.len() });
    }
    let design = logistic_design(x, intercept);
    let p = design.cols;
    let mut beta = VecD::zeros(p);
    let mut converged = false;
    let mut iterations = 0;
    for iter in 0..MAX_ITER {
        iterations = iter + 1;
        let mut mu = vec![0.0; n];
        let mut w = vec![0.0; n];
        for row in 0..n {
            let eta: f64 = (0..p).map(|col| design.get(row, col) * beta.get(col)).sum();
            let m = 1.0 / (1.0 + (-eta).exp());
            mu[row] = m;
            w[row] = (m * (1.0 - m)).max(1e-10);
        }
        let mut xtwx = MatD::zeros(p, p);
        let mut score = VecD::zeros(p);
        for row in 0..n {
            for a in 0..p {
                score.add_at(a, design.get(row, a) * (y[row] - mu[row]));
                for b in 0..p {
                    xtwx.add_at(a, b, design.get(row, a) * w[row] * design.get(row, b));
                }
            }
        }
        let step = xtwx.lu_solve(&score).ok_or(StatisticsError::SingularMatrix)?;
        let step_norm = step.norm2();
        for i in 0..p {
            beta.add_at(i, step.get(i));
        }
        if step_norm < TOL {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(StatisticsError::NoConvergence { iterations });
    }
    let mut w = vec![0.0; n];
    let mut log_likelihood = 0.0;
    for (row, w_row) in w.iter_mut().enumerate() {
        let eta: f64 = (0..p).map(|col| design.get(row, col) * beta.get(col)).sum();
        let m = 1.0 / (1.0 + (-eta).exp());
        *w_row = (m * (1.0 - m)).max(1e-10);
        let mc = m.clamp(1e-12, 1.0 - 1e-12);
        log_likelihood += y[row] * mc.ln() + (1.0 - y[row]) * (1.0 - mc).ln();
    }
    let mut xtwx = MatD::zeros(p, p);
    for row in 0..n {
        for a in 0..p {
            for b in 0..p {
                xtwx.add_at(a, b, design.get(row, a) * w[row] * design.get(row, b));
            }
        }
    }
    let cov = invert(&xtwx)?;
    let std_errors: Vec<f64> = (0..p).map(|i| cov.get(i, i).max(0.0).sqrt()).collect();
    Ok(LogisticFit { coefficients: beta.0, std_errors, log_likelihood, iterations })
}

/// 🎯 Fitted propensity scores `P(y=1 | x)` for new rows.
pub fn logistic_predict(fit: &LogisticFit, x: &MatD, intercept: bool) -> Result<Vec<f64>, StatisticsError> {
    let p = x.cols + usize::from(intercept);
    if fit.coefficients.len() != p {
        return Err(StatisticsError::DimensionMismatch { expected: p, found: fit.coefficients.len() });
    }
    let design = logistic_design(x, intercept);
    Ok((0..x.rows).map(|row| 1.0 / (1.0 + (-(0..p).map(|col| design.get(row, col) * fit.coefficients[col]).sum::<f64>()).exp())).collect())
}
// #endregion 🔖Logistic

// #region 🔖Internal
/// 🗂️ Stratifies `(x, y)` counts by the mixed-radix code of `given`, skipping rows missing in `x`,
/// `y`, or any conditioning column. Shared by [`g2_ci_test`] and [`conditional_mutual_information`].
fn build_strata(x: &[u32], y: &[u32], given: &[&[u32]], nx: usize, ny: usize, given_levels: &[usize]) -> HashMap<usize, MatD> {
    let mut tables: HashMap<usize, MatD> = HashMap::new();
    for row in 0..x.len() {
        if x[row] == mathematical_tabular::MISSING_CODE || y[row] == mathematical_tabular::MISSING_CODE {
            continue;
        }
        let mut stratum = 0usize;
        let mut stride = 1usize;
        let mut missing = false;
        for (k, g) in given.iter().enumerate() {
            if g[row] == mathematical_tabular::MISSING_CODE {
                missing = true;
                break;
            }
            stratum += g[row] as usize * stride;
            stride *= given_levels[k];
        }
        if missing {
            continue;
        }
        tables.entry(stratum).or_insert_with(|| MatD::zeros(nx, ny)).add_at(x[row] as usize, y[row] as usize, 1.0);
    }
    tables
}
// #endregion 🔖Internal

// #region 🔖Tests
/// 📏 A hypothesis-test outcome: statistic, two-sided p-value, and (possibly fractional) degrees of freedom.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TestResult {
    pub statistic: f64,
    pub p_value: f64,
    pub dof: f64,
}

/// 🔗 Fisher-z test of partial correlation `r_ij.given = 0`, i.e. conditional independence for
/// continuous data under a joint-Gaussian assumption.
pub fn fisher_z_test(corr: &MatD, i: usize, j: usize, given: &[usize], n: usize) -> Result<TestResult, StatisticsError> {
    let r = partial_correlation(corr, i, j, given)?.clamp(-1.0 + 1e-15, 1.0 - 1e-15);
    let dof = n as f64 - given.len() as f64 - 3.0;
    if dof <= 0.0 {
        return Err(StatisticsError::InsufficientData { needed: given.len() + 4, found: n });
    }
    let stat = dof.sqrt() * r.atanh().abs();
    let p_value = 2.0 * (1.0 - Normal::STANDARD.cdf(stat));
    Ok(TestResult { statistic: stat, p_value, dof })
}

/// 🗂️ Contingency-table cross-tabulation; rows missing in either column are excluded.
pub fn crosstab(x: &[u32], y: &[u32], nx: usize, ny: usize) -> Result<MatD, StatisticsError> {
    if x.len() != y.len() {
        return Err(StatisticsError::DimensionMismatch { expected: x.len(), found: y.len() });
    }
    let mut m = MatD::zeros(nx, ny);
    for (&xi, &yi) in x.iter().zip(y) {
        if xi == mathematical_tabular::MISSING_CODE || yi == mathematical_tabular::MISSING_CODE {
            continue;
        }
        m.add_at(xi as usize, yi as usize, 1.0);
    }
    Ok(m)
}

/// 🔢 Pearson's chi-squared test of independence on a contingency table.
pub fn chi2_independence(counts: &MatD) -> Result<TestResult, StatisticsError> {
    let (r, c) = (counts.rows, counts.cols);
    if r < 2 || c < 2 {
        return Err(StatisticsError::InvalidArgument("chi2_independence needs at least a 2x2 table"));
    }
    let total: f64 = counts.data.iter().sum();
    if total <= 0.0 {
        return Err(StatisticsError::InsufficientData { needed: 1, found: 0 });
    }
    let row_sums: Vec<f64> = (0..r).map(|i| (0..c).map(|j| counts.get(i, j)).sum()).collect();
    let col_sums: Vec<f64> = (0..c).map(|j| (0..r).map(|i| counts.get(i, j)).sum()).collect();
    let mut stat = 0.0;
    for (i, &row_sum) in row_sums.iter().enumerate() {
        for (j, &col_sum) in col_sums.iter().enumerate() {
            let expected = row_sum * col_sum / total;
            if expected > 0.0 {
                let o = counts.get(i, j);
                stat += (o - expected) * (o - expected) / expected;
            }
        }
    }
    let dof = ((r - 1) * (c - 1)) as f64;
    let p_value = 1.0 - ChiSquared::new(dof)?.cdf(stat);
    Ok(TestResult { statistic: stat, p_value, dof })
}

/// 🔢 G² (likelihood-ratio) conditional-independence test for discrete data, stratified over the
/// conditioning columns; `levels = (nx, ny, given_levels)`.
pub fn g2_ci_test(x: &[u32], y: &[u32], given: &[&[u32]], levels: (usize, usize, &[usize])) -> Result<TestResult, StatisticsError> {
    let (nx, ny, given_levels) = levels;
    if y.len() != x.len() || given.iter().any(|g| g.len() != x.len()) {
        return Err(StatisticsError::DimensionMismatch { expected: x.len(), found: y.len() });
    }
    if given_levels.len() != given.len() {
        return Err(StatisticsError::DimensionMismatch { expected: given.len(), found: given_levels.len() });
    }
    if nx < 2 || ny < 2 {
        return Err(StatisticsError::InvalidArgument("g2_ci_test needs at least 2 levels per tested variable"));
    }
    let tables = build_strata(x, y, given, nx, ny, given_levels);
    let mut g2 = 0.0;
    for table in tables.values() {
        let total: f64 = table.data.iter().sum();
        if total <= 0.0 {
            continue;
        }
        let row_sums: Vec<f64> = (0..nx).map(|i| (0..ny).map(|j| table.get(i, j)).sum()).collect();
        let col_sums: Vec<f64> = (0..ny).map(|j| (0..nx).map(|i| table.get(i, j)).sum()).collect();
        for (i, &row_sum) in row_sums.iter().enumerate() {
            for (j, &col_sum) in col_sums.iter().enumerate() {
                let o = table.get(i, j);
                if o <= 0.0 {
                    continue;
                }
                let e = row_sum * col_sum / total;
                if e > 0.0 {
                    g2 += 2.0 * o * (o / e).ln();
                }
            }
        }
    }
    let n_strata: usize = given_levels.iter().product::<usize>().max(1);
    let dof = ((nx - 1) * (ny - 1) * n_strata) as f64;
    let p_value = 1.0 - ChiSquared::new(dof)?.cdf(g2);
    Ok(TestResult { statistic: g2, p_value, dof })
}

/// 📏 Two-sample t-test; Welch (unequal-variance, fractional Welch–Satterthwaite dof) unless `pooled`.
pub fn t_test_two_sample(a: &[f64], b: &[f64], pooled: bool) -> Result<TestResult, StatisticsError> {
    if a.len() < 2 || b.len() < 2 {
        return Err(StatisticsError::InsufficientData { needed: 2, found: a.len().min(b.len()) });
    }
    let (na, nb) = (a.len() as f64, b.len() as f64);
    let (ma, mb) = (mean(a)?, mean(b)?);
    let (va, vb) = (variance(a)?, variance(b)?);
    let (stat, dof) = if pooled {
        let dof = na + nb - 2.0;
        let sp2 = ((na - 1.0) * va + (nb - 1.0) * vb) / dof;
        let se = (sp2 * (1.0 / na + 1.0 / nb)).sqrt();
        ((ma - mb) / se, dof)
    } else {
        let se2 = va / na + vb / nb;
        let se = se2.sqrt();
        let dof = se2 * se2 / ((va / na).powi(2) / (na - 1.0) + (vb / nb).powi(2) / (nb - 1.0));
        ((ma - mb) / se, dof)
    };
    let p_value = 2.0 * (1.0 - StudentT::new(dof)?.cdf(stat.abs()));
    Ok(TestResult { statistic: stat, p_value, dof })
}
// #endregion 🔖Tests

// #region 🔖Information
/// 🔢 Shannon entropy in nats, missing codes excluded.
pub fn entropy(codes: &[u32], n_levels: usize) -> Result<f64, StatisticsError> {
    let mut counts = vec![0usize; n_levels];
    let mut total = 0usize;
    for &c in codes {
        if c == mathematical_tabular::MISSING_CODE {
            continue;
        }
        counts[c as usize] += 1;
        total += 1;
    }
    if total == 0 {
        return Err(StatisticsError::InsufficientData { needed: 1, found: 0 });
    }
    let mut h = 0.0;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / total as f64;
            h -= p * p.ln();
        }
    }
    Ok(h)
}

/// 🔢 Mutual information `I(X;Y)` in nats.
pub fn mutual_information(x: &[u32], y: &[u32], nx: usize, ny: usize) -> Result<f64, StatisticsError> {
    let table = crosstab(x, y, nx, ny)?;
    let total: f64 = table.data.iter().sum();
    if total <= 0.0 {
        return Err(StatisticsError::InsufficientData { needed: 1, found: 0 });
    }
    let row_sums: Vec<f64> = (0..nx).map(|i| (0..ny).map(|j| table.get(i, j)).sum()).collect();
    let col_sums: Vec<f64> = (0..ny).map(|j| (0..nx).map(|i| table.get(i, j)).sum()).collect();
    let mut mi = 0.0;
    for (i, &row_sum) in row_sums.iter().enumerate() {
        for (j, &col_sum) in col_sums.iter().enumerate() {
            let o = table.get(i, j);
            if o <= 0.0 {
                continue;
            }
            let pij = o / total;
            let pi = row_sum / total;
            let pj = col_sum / total;
            mi += pij * (pij / (pi * pj)).ln();
        }
    }
    Ok(mi)
}

/// 🔢 Conditional mutual information `I(X;Y|Z)` in nats, stratified over `given`.
pub fn conditional_mutual_information(x: &[u32], y: &[u32], given: &[&[u32]], levels: (usize, usize, &[usize])) -> Result<f64, StatisticsError> {
    let (nx, ny, given_levels) = levels;
    let tables = build_strata(x, y, given, nx, ny, given_levels);
    let grand_total: f64 = tables.values().map(|t| t.data.iter().sum::<f64>()).sum();
    if grand_total <= 0.0 {
        return Err(StatisticsError::InsufficientData { needed: 1, found: 0 });
    }
    let mut cmi = 0.0;
    for table in tables.values() {
        let total: f64 = table.data.iter().sum();
        if total <= 0.0 {
            continue;
        }
        let row_sums: Vec<f64> = (0..nx).map(|i| (0..ny).map(|j| table.get(i, j)).sum()).collect();
        let col_sums: Vec<f64> = (0..ny).map(|j| (0..nx).map(|i| table.get(i, j)).sum()).collect();
        for (i, &row_sum) in row_sums.iter().enumerate() {
            for (j, &col_sum) in col_sums.iter().enumerate() {
                let o = table.get(i, j);
                if o <= 0.0 {
                    continue;
                }
                let p_z = total / grand_total;
                let p_xyz = o / grand_total;
                let p_xz = row_sum / grand_total;
                let p_yz = col_sum / grand_total;
                cmi += p_xyz * (p_xyz * p_z / (p_xz * p_yz)).ln();
            }
        }
    }
    Ok(cmi)
}
// #endregion 🔖Information

// #region 🔖UnitTests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖DescriptiveTests
    #[test]
    fn mean_and_variance_hand_computed() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((mean(&values).unwrap() - 5.0).abs() < 1e-12);
        assert!((variance(&values).unwrap() - 32.0 / 7.0).abs() < 1e-9);
    }

    #[test]
    fn correlation_of_perfect_line_is_one() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y: Vec<f64> = x.iter().map(|v| 2.0 * v + 1.0).collect();
        assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn correlation_of_orthogonal_pattern_is_zero() {
        let x = [1.0, -1.0, 1.0, -1.0];
        let y = [1.0, 1.0, -1.0, -1.0];
        assert!(correlation(&x, &y).unwrap().abs() < 1e-9);
    }
    // #endregion 🔖DescriptiveTests

    // #region 🔖MatrixTests
    #[test]
    fn correlation_matrix_diagonal_is_one() {
        let x = [1.0, 2.0, 3.0, 4.0];
        let y = [4.0, 3.0, 2.0, 1.0];
        let m = correlation_matrix(&[&x, &y]).unwrap();
        assert!((m.get(0, 0) - 1.0).abs() < 1e-12);
        assert!((m.get(1, 1) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn invert_round_trips_and_detects_singular() {
        let mut a = MatD::zeros(3, 3);
        for (i, v) in [2.0, 0.0, 1.0, 1.0, 3.0, 2.0, 0.0, 1.0, 4.0].into_iter().enumerate() {
            a.set(i / 3, i % 3, v);
        }
        let inv = invert(&a).unwrap();
        let identity = a.matmul(&inv);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((identity.get(i, j) - expected).abs() < 1e-9);
            }
        }
        let singular = MatD::zeros(2, 2);
        assert!(invert(&singular).is_err());
    }
    // #endregion 🔖MatrixTests

    // #region 🔖PartialTests
    #[test]
    fn partial_correlation_matches_closed_form() {
        let mut corr = MatD::identity(3);
        corr.set(0, 1, 0.5);
        corr.set(1, 0, 0.5);
        corr.set(0, 2, 0.7);
        corr.set(2, 0, 0.7);
        corr.set(1, 2, 0.7);
        corr.set(2, 1, 0.7);
        let r = partial_correlation(&corr, 0, 1, &[2]).unwrap();
        assert!((r - 0.019_607_843_137_254_9).abs() < 1e-9);
    }

    #[test]
    fn partial_correlation_with_empty_given_equals_plain_correlation() {
        let mut corr = MatD::identity(2);
        corr.set(0, 1, 0.4);
        corr.set(1, 0, 0.4);
        let r = partial_correlation(&corr, 0, 1, &[]).unwrap();
        assert!((r - 0.4).abs() < 1e-9);
    }
    // #endregion 🔖PartialTests

    // #region 🔖OlsTests
    #[test]
    fn ols_recovers_exact_line() {
        let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ys: Vec<f64> = xs.iter().map(|x| 3.0 + 2.0 * x).collect();
        let mut design = MatD::zeros(5, 1);
        for (row, &x) in xs.iter().enumerate() {
            design.set(row, 0, x);
        }
        let fit = ols(&design, &ys, true).unwrap();
        assert!((fit.coefficients[0] - 3.0).abs() < 1e-8);
        assert!((fit.coefficients[1] - 2.0).abs() < 1e-8);
        assert!((fit.r_squared - 1.0).abs() < 1e-8);
    }
    // #endregion 🔖OlsTests

    // #region 🔖LogisticTests
    #[test]
    fn logistic_symmetric_data_has_near_zero_intercept() {
        let xs = [-3.0, -2.0, -1.0, 1.0, 2.0, 3.0];
        let ys = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let mut design = MatD::zeros(6, 1);
        for (row, &x) in xs.iter().enumerate() {
            design.set(row, 0, x);
        }
        let fit = logistic(&design, &ys, true).unwrap();
        assert!(fit.coefficients[0].abs() < 0.2, "intercept {} too far from 0", fit.coefficients[0]);
        assert!(fit.coefficients[1] > 0.0, "slope should be positive");
    }

    #[test]
    fn logistic_perfect_separation_returns_error_not_panic() {
        let xs = [1.0, 2.0, 3.0, 4.0];
        let ys = [0.0, 0.0, 1.0, 1.0];
        let mut design = MatD::zeros(4, 1);
        for (row, &x) in xs.iter().enumerate() {
            design.set(row, 0, x);
        }
        // perfectly separable data either fails to converge or returns a large-but-finite fit — must not panic
        match logistic(&design, &ys, true) {
            Ok(fit) => assert!(fit.coefficients.iter().all(|c| c.is_finite())),
            Err(StatisticsError::NoConvergence { .. }) => {}
            Err(other) => panic!("unexpected error: {other}"),
        }
    }
    // #endregion 🔖LogisticTests

    // #region 🔖HypothesisTests
    #[test]
    fn fisher_z_test_matches_hand_computation() {
        let mut corr = MatD::identity(2);
        corr.set(0, 1, 0.5);
        corr.set(1, 0, 0.5);
        let result = fisher_z_test(&corr, 0, 1, &[], 28).unwrap();
        assert!((result.statistic - 2.746_530_71).abs() < 1e-6);
        assert!((result.p_value - 0.006_021).abs() < 1e-4);
    }

    #[test]
    fn chi2_independence_matches_hand_computation() {
        let mut counts = MatD::zeros(2, 2);
        counts.set(0, 0, 10.0);
        counts.set(0, 1, 20.0);
        counts.set(1, 0, 20.0);
        counts.set(1, 1, 10.0);
        let result = chi2_independence(&counts).unwrap();
        assert!((result.statistic - 6.666_666_666_666_667).abs() < 1e-6);
        assert!((result.dof - 1.0).abs() < 1e-12);
    }

    #[test]
    fn g2_ci_test_is_zero_for_margin_product_counts() {
        let x: Vec<u32> = [0u32, 0, 1, 1].repeat(25);
        let y: Vec<u32> = [0u32, 1, 0, 1].repeat(25);
        let result = g2_ci_test(&x, &y, &[], (2, 2, &[])).unwrap();
        assert!(result.statistic.abs() < 1e-9, "G2 {} should be ~0 for independent margin-product counts", result.statistic);
        assert!((result.dof - 1.0).abs() < 1e-12);
    }

    #[test]
    fn welch_t_test_matches_hand_computation() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [2.0, 3.0, 4.0, 5.0, 6.0];
        let result = t_test_two_sample(&a, &b, false).unwrap();
        assert!((result.statistic - (-1.0)).abs() < 1e-9);
        assert!((result.dof - 8.0).abs() < 1e-9);
    }
    // #endregion 🔖HypothesisTests

    // #region 🔖InformationTests
    #[test]
    fn entropy_of_uniform_four_levels_is_ln_four() {
        let codes: Vec<u32> = [0u32, 1, 2, 3].repeat(100);
        let h = entropy(&codes, 4).unwrap();
        assert!((h - 4.0_f64.ln()).abs() < 1e-9);
    }

    #[test]
    fn mutual_information_of_variable_with_itself_is_its_entropy() {
        let codes: Vec<u32> = [0u32, 0, 1, 1, 2, 2].repeat(50);
        let mi = mutual_information(&codes, &codes, 3, 3).unwrap();
        let h = entropy(&codes, 3).unwrap();
        assert!((mi - h).abs() < 1e-9);
    }

    #[test]
    fn conditional_mutual_information_is_zero_on_markov_chain() {
        // X -> Z -> Y: within each Z stratum, X and Y are independently uniform over {0,1}.
        let z: Vec<u32> = [0u32, 0, 0, 0, 1, 1, 1, 1].repeat(20);
        let x: Vec<u32> = [0u32, 0, 1, 1, 0, 0, 1, 1].repeat(20);
        let y: Vec<u32> = [0u32, 1, 0, 1, 0, 1, 0, 1].repeat(20);
        let z_refs: Vec<&[u32]> = vec![&z];
        let cmi = conditional_mutual_information(&x, &y, &z_refs, (2, 2, &[2])).unwrap();
        assert!(cmi.abs() < 1e-9, "CMI {cmi} should be ~0 on a Markov chain");
    }
    // #endregion 🔖InformationTests
}
// #endregion 🔖UnitTests
