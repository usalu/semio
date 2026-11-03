use super::{Dense, Skyline};
use crate::{Control, MechanicsError, MechanicsResult};

pub const DENSE_LIMIT: usize = 600;

/// 🌌️ Eigen-decomposition `A = Z diag(d) Zᵀ` of a symmetric row-major matrix (Householder tridiagonalisation, implicit QL); columns of `Z` are eigenvectors, values ascending.
pub fn symmetric_eigen(matrix: &Dense, control: &Control<'_>) -> MechanicsResult<(Vec<f64>, Dense)> {
    let n = matrix.n;
    let mut a = matrix.a.clone();
    let mut q = Dense::zeros(n);
    for i in 0..n {
        q.a[i * n + i] = 1.0;
    }
    for k in 0..n.saturating_sub(2) {
        control.check()?;
        let m = n - k - 1;
        let mut v: Vec<f64> = (0..m).map(|i| a[(k + 1 + i) * n + k]).collect();
        let tail: f64 = v[1..].iter().map(|x| x * x).sum();
        if tail == 0.0 {
            continue;
        }
        let norm = (v[0] * v[0] + tail).sqrt();
        let alpha = if v[0] >= 0.0 { -norm } else { norm };
        v[0] -= alpha;
        let vn = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.iter_mut().for_each(|x| *x /= vn);
        let p: Vec<f64> = (0..m).map(|i| 2.0 * (0..m).map(|j| a[(k + 1 + i) * n + k + 1 + j] * v[j]).sum::<f64>()).collect();
        let vp: f64 = v.iter().zip(&p).map(|(x, y)| x * y).sum();
        let w: Vec<f64> = p.iter().zip(&v).map(|(pi, vi)| pi - vp * vi).collect();
        for i in 0..m {
            for j in 0..m {
                a[(k + 1 + i) * n + k + 1 + j] -= v[i] * w[j] + w[i] * v[j];
            }
        }
        for i in 0..m {
            let val = if i == 0 { alpha } else { 0.0 };
            a[(k + 1 + i) * n + k] = val;
            a[k * n + k + 1 + i] = val;
        }
        for r in 0..n {
            let s: f64 = (0..m).map(|j| q.a[r * n + k + 1 + j] * v[j]).sum();
            for j in 0..m {
                q.a[r * n + k + 1 + j] -= 2.0 * s * v[j];
            }
        }
    }
    let mut d: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    let mut e: Vec<f64> = (0..n).map(|i| if i + 1 < n { a[(i + 1) * n + i] } else { 0.0 }).collect();
    for l in 0..n {
        if l % 16 == 0 {
            control.check()?;
        }
        let mut iter = 0;
        loop {
            let mut m = l;
            while m + 1 < n {
                let dd = d[m].abs() + d[m + 1].abs();
                if e[m].abs() <= f64::EPSILON * dd {
                    break;
                }
                m += 1;
            }
            if m == l {
                break;
            }
            iter += 1;
            if iter > 80 {
                return Err(MechanicsError::NoConvergence);
            }
            let mut g = (d[l + 1] - d[l]) / (2.0 * e[l]);
            let mut r = g.hypot(1.0);
            g = d[m] - d[l] + e[l] / (g + r.copysign(g));
            let (mut s, mut c, mut p) = (1.0_f64, 1.0_f64, 0.0_f64);
            let mut early = false;
            let mut i = m;
            while i > l {
                i -= 1;
                let f = s * e[i];
                let b = c * e[i];
                r = f.hypot(g);
                e[i + 1] = r;
                if r == 0.0 {
                    d[i + 1] -= p;
                    e[m] = 0.0;
                    early = true;
                    break;
                }
                s = f / r;
                c = g / r;
                g = d[i + 1] - p;
                r = (d[i] - g) * s + 2.0 * c * b;
                p = s * r;
                d[i + 1] = g + p;
                g = c * r - b;
                for k in 0..n {
                    let f = q.a[k * n + i + 1];
                    q.a[k * n + i + 1] = s * q.a[k * n + i] + c * f;
                    q.a[k * n + i] = c * q.a[k * n + i] - s * f;
                }
            }
            if early {
                continue;
            }
            d[l] -= p;
            e[l] = g;
            e[m] = 0.0;
        }
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&x, &y| d[x].total_cmp(&d[y]));
    let mut z = Dense::zeros(n);
    for (new, &old) in order.iter().enumerate() {
        for r in 0..n {
            z.a[r * n + new] = q.a[r * n + old];
        }
    }
    Ok((order.iter().map(|&i| d[i]).collect(), z))
}

/// 🎚️ Largest positive eigenpairs `μ` of `K x = λ B x` with `K` symmetric positive definite and `B` symmetric (possibly indefinite or semi-definite); returns `(λ = 1/μ, x)` ascending in `λ`, `x` normalised to `xᵀ B x = ±1`.
pub fn generalized_dense(k: &Dense, b: &Dense, count: usize, control: &Control<'_>) -> MechanicsResult<Vec<(f64, Vec<f64>)>> {
    let n = k.n;
    let l = k.cholesky(control)?;
    let mut y = b.clone();
    for c in 0..n {
        control.check()?;
        let mut col: Vec<f64> = (0..n).map(|r| y.at(r, c)).collect();
        l.forward(&mut col);
        for r in 0..n {
            y.a[r * n + c] = col[r];
        }
    }
    let mut z = Dense::zeros(n);
    for r in 0..n {
        for c in 0..n {
            z.a[c * n + r] = y.at(r, c);
        }
    }
    for c in 0..n {
        control.check()?;
        let mut col: Vec<f64> = (0..n).map(|r| z.at(r, c)).collect();
        l.forward(&mut col);
        for r in 0..n {
            z.a[r * n + c] = col[r];
        }
    }
    for i in 0..n {
        for j in 0..i {
            let s = 0.5 * (z.at(i, j) + z.at(j, i));
            z.a[i * n + j] = s;
            z.a[j * n + i] = s;
        }
    }
    let (mu, vec) = symmetric_eigen(&z, control)?;
    let max = mu.iter().cloned().fold(0.0, f64::max);
    let mut out = Vec::new();
    for idx in (0..n).rev() {
        if out.len() >= count || mu[idx] <= 1e-12 * max || mu[idx] <= 0.0 {
            break;
        }
        let mut x: Vec<f64> = (0..n).map(|r| vec.at(r, idx)).collect();
        l.backward_transposed(&mut x);
        let bx = b.mul_vec(&x);
        let norm: f64 = x.iter().zip(&bx).map(|(p, q)| p * q).sum::<f64>().abs().sqrt();
        if norm > 0.0 {
            x.iter_mut().for_each(|v| *v /= norm);
        }
        out.push((1.0 / mu[idx], x));
    }
    Ok(out)
}

/// 🔁️ Subspace iteration on `K⁻¹ B` with Rayleigh-Ritz projection for models above [`DENSE_LIMIT`]; `k` must be the unfactored matrix and `factor` its factorisation.
pub fn generalized_subspace(k: &Skyline, factor: &Skyline, b: &Skyline, count: usize, control: &Control<'_>) -> MechanicsResult<Vec<(f64, Vec<f64>)>> {
    let n = k.n;
    let p = (2 * count).max(count + 8).min(n);
    let mut seed = 0x9E37_79B9_7F4A_7C15_u64;
    let mut x: Vec<Vec<f64>> = (0..p)
        .map(|_| {
            (0..n)
                .map(|_| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    (seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5
                })
                .collect()
        })
        .collect();
    let mut previous: Vec<f64> = Vec::new();
    for iteration in 0..100 {
        control.check()?;
        control.report("subspace", iteration as f64 / 100.0);
        let y: Vec<Vec<f64>> = x
            .iter()
            .map(|v| {
                let mut w = b.mul_vec(v);
                factor.solve(&mut w);
                w
            })
            .collect();
        let mut kr = Dense::zeros(p);
        let mut br = Dense::zeros(p);
        let ky: Vec<Vec<f64>> = y.iter().map(|v| k.mul_vec(v)).collect();
        let by: Vec<Vec<f64>> = y.iter().map(|v| b.mul_vec(v)).collect();
        for i in 0..p {
            for j in 0..p {
                kr.a[i * p + j] = y[i].iter().zip(&ky[j]).map(|(a, c)| a * c).sum();
                br.a[i * p + j] = y[i].iter().zip(&by[j]).map(|(a, c)| a * c).sum();
            }
        }
        for i in 0..p {
            for j in 0..i {
                let s = 0.5 * (kr.at(i, j) + kr.at(j, i));
                kr.a[i * p + j] = s;
                kr.a[j * p + i] = s;
                let s = 0.5 * (br.at(i, j) + br.at(j, i));
                br.a[i * p + j] = s;
                br.a[j * p + i] = s;
            }
        }
        let pairs = generalized_dense(&kr, &br, p, control)?;
        x = pairs
            .iter()
            .map(|(_, phi)| (0..n).map(|r| (0..p).map(|c| y[c][r] * phi[c]).sum()).collect())
            .collect();
        let lambdas: Vec<f64> = pairs.iter().map(|(l, _)| *l).collect();
        let m = count.min(lambdas.len());
        let converged = previous.len() >= m && m > 0 && (0..m).all(|i| (lambdas[i] - previous[i]).abs() <= 1e-10 * lambdas[i].abs());
        previous = lambdas;
        if converged {
            break;
        }
        while x.len() < p {
            let k = x.len();
            x.push((0..n).map(|i| ((i * 31 + k * 17) % 97) as f64 / 97.0 - 0.5).collect());
        }
    }
    Ok(previous.iter().cloned().zip(x).take(count).collect())
}
