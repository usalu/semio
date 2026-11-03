//! 🌀️ Mcr of an arbitrarily loaded and supported thin-walled I-beam by a Hermite finite-element eigenproblem (mm, N). See [Mcr by FE](https://doi.org/10.1016/0045-7949(95)00034-8).

pub struct BeamProps {
    pub l: f64,
    pub e: f64,
    pub g: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Support {
    pub x: f64,
    pub lateral: bool,
    pub twist: bool,
    pub warping: bool,
    pub lateral_rotation: bool,
}

impl Support {
    pub fn fork(x: f64) -> Self {
        Self { x, lateral: true, twist: true, warping: false, lateral_rotation: false }
    }
    pub fn fixed(x: f64) -> Self {
        Self { x, lateral: true, twist: true, warping: true, lateral_rotation: true }
    }
}

pub struct Load<'a> {
    pub moment: &'a dyn Fn(f64) -> f64,
    pub line_load: &'a dyn Fn(f64) -> f64,
    pub zg: f64,
}

pub struct Restraints {
    pub supports: Vec<Support>,
    pub torsional_spring_per_length: f64,
    pub lateral_springs: Vec<(f64, f64)>,
}

pub struct McrResult {
    pub lambda: f64,
    pub mode_v: Vec<f64>,
    pub mode_phi: Vec<f64>,
    pub stations: Vec<f64>,
}

const GP: [(f64, f64); 4] = [
    (0.069431844202973714, 0.173927422568727),
    (0.330009478207571867, 0.326072577431273),
    (0.669990521792428133, 0.326072577431273),
    (0.930568155797026286, 0.173927422568727),
];

fn shape(xi: f64, h: f64) -> ([f64; 4], [f64; 4], [f64; 4]) {
    let n = [1.0 - 3.0 * xi * xi + 2.0 * xi.powi(3), h * (xi - 2.0 * xi * xi + xi.powi(3)), 3.0 * xi * xi - 2.0 * xi.powi(3), h * (-xi * xi + xi.powi(3))];
    let d1 = [(-6.0 * xi + 6.0 * xi * xi) / h, 1.0 - 4.0 * xi + 3.0 * xi * xi, (6.0 * xi - 6.0 * xi * xi) / h, -2.0 * xi + 3.0 * xi * xi];
    let d2 = [(-6.0 + 12.0 * xi) / (h * h), (-4.0 + 6.0 * xi) / h, (6.0 - 12.0 * xi) / (h * h), (-2.0 + 6.0 * xi) / h];
    (n, d1, d2)
}

fn cholesky_solve_lower(k: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = k.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s = k[i][j];
            for p in 0..j {
                s -= l[i][p] * l[j][p];
            }
            if i == j {
                if s <= 0.0 {
                    return None;
                }
                l[i][i] = s.sqrt();
            } else {
                l[i][j] = s / l[j][j];
            }
        }
    }
    Some(l)
}

fn jacobi(a: &mut Vec<Vec<f64>>) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n = a.len();
    let mut v = vec![vec![0.0; n]; n];
    for i in 0..n {
        v[i][i] = 1.0;
    }
    for _ in 0..100 {
        let mut off = 0.0;
        for i in 0..n {
            for j in 0..i {
                off += a[i][j] * a[i][j];
            }
        }
        if off < 1e-24 {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                if a[p][q].abs() < 1e-30 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for k in 0..n {
                    let (akp, akq) = (a[k][p], a[k][q]);
                    a[k][p] = c * akp - s * akq;
                    a[k][q] = s * akp + c * akq;
                }
                for k in 0..n {
                    let (apk, aqk) = (a[p][k], a[q][k]);
                    a[p][k] = c * apk - s * aqk;
                    a[q][k] = s * apk + c * aqk;
                }
                for k in 0..n {
                    let (vkp, vkq) = (v[k][p], v[k][q]);
                    v[k][p] = c * vkp - s * vkq;
                    v[k][q] = s * vkp + c * vkq;
                }
            }
        }
    }
    ((0..n).map(|i| a[i][i]).collect(), v)
}

/// 🌀️ Smallest positive load factor λ such that λ·(M(x), q(x)) buckles; `n_el` elements per beam.
pub fn mcr_fe(p: &BeamProps, load: &Load, r: &Restraints, n_el: usize) -> Option<McrResult> {
    let nn = n_el + 1;
    let nd = 4 * nn;
    let h = p.l / n_el as f64;
    let mut ke = vec![vec![0.0; nd]; nd];
    let mut kh = vec![vec![0.0; nd]; nd];
    for el in 0..n_el {
        let x0 = el as f64 * h;
        let base = 4 * el;
        for &(xi, w) in GP.iter() {
            let (n, d1, d2) = shape(xi, h);
            let x = x0 + xi * h;
            let m = (load.moment)(x);
            let q = (load.line_load)(x);
            let wgt = w * h;
            for a in 0..4 {
                for b in 0..4 {
                    let (ia, ib) = (base + 4 * (a / 2) + a % 2, base + 4 * (b / 2) + b % 2);
                    ke[ia][ib] += wgt * p.e * p.iz * d2[a] * d2[b];
                    ke[ia + 2][ib + 2] += wgt * (p.e * p.iw * d2[a] * d2[b] + p.g * p.it * d1[a] * d1[b] + r.torsional_spring_per_length * n[a] * n[b]);
                    kh[ia][ib + 2] += -wgt * m * d2[a] * n[b];
                    kh[ib + 2][ia] += -wgt * m * d2[a] * n[b];
                    kh[ia + 2][ib + 2] += -wgt * q * load.zg * n[a] * n[b];
                }
            }
        }
    }
    for &(x, k) in &r.lateral_springs {
        let j = ((x / h).round() as usize).min(n_el);
        ke[4 * j][4 * j] += k;
    }
    let mut fixed = vec![false; nd];
    for s in &r.supports {
        let j = ((s.x / h).round() as usize).min(n_el);
        fixed[4 * j] |= s.lateral;
        fixed[4 * j + 1] |= s.lateral_rotation;
        fixed[4 * j + 2] |= s.twist;
        fixed[4 * j + 3] |= s.warping;
    }
    let free: Vec<usize> = (0..nd).filter(|&i| !fixed[i]).collect();
    let n = free.len();
    let sub = |m: &Vec<Vec<f64>>| -> Vec<Vec<f64>> { (0..n).map(|i| (0..n).map(|j| m[free[i]][free[j]]).collect()).collect() };
    let mut kes = sub(&ke);
    let mut khs = sub(&kh);
    let d: Vec<f64> = (0..n).map(|i| 1.0 / kes[i][i].max(1e-300).sqrt()).collect();
    for i in 0..n {
        for j in 0..n {
            kes[i][j] *= d[i] * d[j];
            khs[i][j] *= d[i] * d[j];
        }
    }
    let l = cholesky_solve_lower(&kes)?;
    let mut y = vec![vec![0.0; n]; n];
    for c in 0..n {
        for i in 0..n {
            let mut s = khs[i][c];
            for k in 0..i {
                s -= l[i][k] * y[k][c];
            }
            y[i][c] = s / l[i][i];
        }
    }
    let mut cmat = vec![vec![0.0; n]; n];
    for r_ in 0..n {
        for i in 0..n {
            let mut s = y[r_][i];
            for k in 0..i {
                s -= l[i][k] * cmat[r_][k];
            }
            cmat[r_][i] = s / l[i][i];
        }
    }
    let sym: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| 0.5 * (cmat[i][j] + cmat[j][i])).collect()).collect();
    let scale = sym.iter().flatten().fold(0.0_f64, |a, &b| a.max(b.abs())).max(1e-300);
    let mut work: Vec<Vec<f64>> = sym.iter().map(|r| r.iter().map(|v| v / scale).collect()).collect();
    let (vals, vecs) = jacobi(&mut work);
    let (idx, mu) = vals.iter().enumerate().fold((usize::MAX, 0.0), |acc, (i, &m)| if m * scale < acc.1 { (i, m * scale) } else { acc });
    if idx == usize::MAX {
        return None;
    }
    let lambda = -1.0 / mu;
    let mut z = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = vecs[i][idx];
        for k in (i + 1)..n {
            s -= l[k][i] * z[k];
        }
        z[i] = s / l[i][i];
    }
    for i in 0..n {
        z[i] *= d[i];
    }
    let mut full = vec![0.0; nd];
    for (k, &f) in free.iter().enumerate() {
        full[f] = z[k];
    }
    Some(McrResult { lambda, mode_v: (0..nn).map(|j| full[4 * j]).collect(), mode_phi: (0..nn).map(|j| full[4 * j + 2]).collect(), stations: (0..nn).map(|j| j as f64 * h).collect() })
}

/// 🌀️ Equivalent C1 = Mcr,FE/Mcr(C1=1) for the maximum moment of the diagram.
pub fn c1_equivalent(mcr_max_moment: f64, mcr_uniform_closed_form: f64) -> f64 {
    mcr_max_moment / mcr_uniform_closed_form
}
