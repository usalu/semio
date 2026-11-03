//! 🧱️ Euler-Bernoulli and Timoshenko 3D frame element: stiffness, geometric stiffness, mass, equivalent loads and release condensation.

use crate::{MassType, MechanicsError, MechanicsResult};

pub type M12 = [[f64; 12]; 12];

const GAUSS: [(f64, f64); 4] = [
    (-0.861_136_311_594_052_6, 0.347_854_845_137_453_8),
    (-0.339_981_043_584_856_3, 0.652_145_154_862_546_1),
    (0.339_981_043_584_856_3, 0.652_145_154_862_546_1),
    (0.861_136_311_594_052_6, 0.347_854_845_137_453_8),
];

const XY: [usize; 4] = [1, 5, 7, 11];
const XZ: [usize; 4] = [2, 4, 8, 10];

#[derive(Clone, Copy, Debug, Default)]
pub struct Props {
    pub e: f64,
    pub g: f64,
    pub rho: f64,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
    pub asy: f64,
    pub asz: f64,
    pub shear: bool,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub props: Props,
    pub l: f64,
    pub r: [[f64; 3]; 3],
    pub t: M12,
    pub k: M12,
}

pub fn zero() -> M12 {
    [[0.0; 12]; 12]
}

pub fn identity() -> M12 {
    let mut m = zero();
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    m
}

pub fn mul(a: &M12, b: &M12) -> M12 {
    let mut c = zero();
    for i in 0..12 {
        for k in 0..12 {
            let v = a[i][k];
            if v != 0.0 {
                for j in 0..12 {
                    c[i][j] += v * b[k][j];
                }
            }
        }
    }
    c
}

pub fn transpose(a: &M12) -> M12 {
    let mut c = zero();
    for i in 0..12 {
        for j in 0..12 {
            c[j][i] = a[i][j];
        }
    }
    c
}

pub fn mat_vec(a: &M12, v: &[f64; 12]) -> [f64; 12] {
    let mut o = [0.0; 12];
    for i in 0..12 {
        o[i] = (0..12).map(|j| a[i][j] * v[j]).sum();
    }
    o
}

pub fn rotate(r: &[[f64; 3]; 3], v: &[f64; 12], to_global: bool) -> [f64; 12] {
    let mut o = [0.0; 12];
    for b in 0..4 {
        for i in 0..3 {
            o[3 * b + i] = (0..3).map(|j| if to_global { r[j][i] } else { r[i][j] } * v[3 * b + j]).sum();
        }
    }
    o
}

pub fn rot12(r: &[[f64; 3]; 3]) -> M12 {
    let mut m = zero();
    for b in 0..4 {
        for i in 0..3 {
            for j in 0..3 {
                m[3 * b + i][3 * b + j] = r[i][j];
            }
        }
    }
    m
}

/// 🌐️ `Trᵀ M Tr`: a local 12x12 matrix expressed in global axes.
pub fn to_global(r: &[[f64; 3]; 3], m: &M12) -> M12 {
    let t = rot12(r);
    mul(&transpose(&t), &mul(m, &t))
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}

/// 🧭️ Member length and rotation rows `[ex, ey, ez]`; `ey = Z × ex` (vertical members use global Y), rotated by `roll` about `ex`.
pub fn axes(pi: [f64; 3], pj: [f64; 3], roll: f64) -> Option<(f64, [[f64; 3]; 3])> {
    let d = sub(pj, pi);
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    if !(l > 0.0) || !l.is_finite() {
        return None;
    }
    let ex = [d[0] / l, d[1] / l, d[2] / l];
    let ey0 = if ex[2].abs() > 0.999_999 { [0.0, 1.0, 0.0] } else { unit(cross([0.0, 0.0, 1.0], ex)) };
    let ez0 = cross(ex, ey0);
    let (s, c) = roll.sin_cos();
    let ey = [0, 1, 2].map(|i| c * ey0[i] + s * ez0[i]);
    let ez = [0, 1, 2].map(|i| -s * ey0[i] + c * ez0[i]);
    Some((l, [ex, ey, ez]))
}

fn bending(ei: f64, l: f64, phi: f64) -> [[f64; 4]; 4] {
    let c = ei / (l * l * l * (1.0 + phi));
    let (kvv, kvt, a, b) = (12.0 * c, 6.0 * c * l, (4.0 + phi) * c * l * l, (2.0 - phi) * c * l * l);
    [[kvv, kvt, -kvv, kvt], [kvt, a, -kvt, b], [-kvv, -kvt, kvv, -kvt], [kvt, b, -kvt, a]]
}

fn place(m: &mut M12, blk: &[[f64; 4]; 4], idx: [usize; 4], flip: bool) {
    for a in 0..4 {
        for b in 0..4 {
            let s = if flip && (a + b) % 2 == 1 { -1.0 } else { 1.0 };
            m[idx[a]][idx[b]] += s * blk[a][b];
        }
    }
}

fn pair(m: &mut M12, i: usize, j: usize, d: f64, o: f64) {
    m[i][i] += d;
    m[j][j] += d;
    m[i][j] += o;
    m[j][i] += o;
}

fn solve_small(mut a: Vec<Vec<f64>>, mut b: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let scale = (0..n).map(|i| a[i][i].abs()).fold(0.0, f64::max);
    for c in 0..n {
        let p = (c..n).max_by(|&x, &y| a[x][c].abs().total_cmp(&a[y][c].abs()))?;
        if a[p][c].abs() <= 1e-12 * scale {
            return None;
        }
        a.swap(c, p);
        b.swap(c, p);
        for r in 0..n {
            if r != c {
                let f = a[r][c] / a[c][c];
                if f != 0.0 {
                    for k in 0..n {
                        a[r][k] -= f * a[c][k];
                    }
                    for k in 0..b[r].len() {
                        b[r][k] -= f * b[c][k];
                    }
                }
            }
        }
    }
    for r in 0..n {
        let d = a[r][r];
        b[r].iter_mut().for_each(|v| *v /= d);
    }
    Some(b)
}

/// 🔓️ Static condensation map `T` (`u_full = T u_retained`) for the released DOFs `rel`; `Tᵀ K T` is the Schur complement.
pub fn condensation(k: &M12, rel: [bool; 12]) -> MechanicsResult<M12> {
    let c: Vec<usize> = (0..12).filter(|&i| rel[i]).collect();
    let r: Vec<usize> = (0..12).filter(|&i| !rel[i]).collect();
    let mut t = identity();
    if c.is_empty() {
        return Ok(t);
    }
    for &i in &c {
        t[i][i] = 0.0;
    }
    let kcc = c.iter().map(|&i| c.iter().map(|&j| k[i][j]).collect()).collect();
    let kcr = c.iter().map(|&i| r.iter().map(|&j| k[i][j]).collect()).collect();
    let x = solve_small(kcc, kcr).ok_or_else(|| MechanicsError::invalid("member releases form a mechanism", "Stabgelenke bilden einen Mechanismus"))?;
    for (ci, &i) in c.iter().enumerate() {
        for (rj, &j) in r.iter().enumerate() {
            t[i][j] = -x[ci][rj];
        }
    }
    Ok(t)
}

impl Element {
    /// 🏗️ Build the condensed local element between two points.
    pub fn new(props: Props, pi: [f64; 3], pj: [f64; 3], roll: f64, rel_i: [bool; 6], rel_j: [bool; 6]) -> MechanicsResult<Self> {
        let (l, r) = axes(pi, pj, roll).ok_or_else(|| MechanicsError::invalid("zero-length member", "Stab der Länge null"))?;
        let k0 = stiffness(&props, l);
        let mut rel = [false; 12];
        rel[..6].copy_from_slice(&rel_i);
        rel[6..].copy_from_slice(&rel_j);
        let t = condensation(&k0, rel)?;
        let k = mul(&transpose(&t), &mul(&k0, &t));
        Ok(Self { props, l, r, t, k })
    }

    fn condense(&self, m: &M12) -> M12 {
        mul(&transpose(&self.t), &mul(m, &self.t))
    }

    /// ⚖️ Condensed geometric stiffness for axial force `n` (tension positive).
    pub fn geometric(&self, n: f64) -> M12 {
        let (l, p) = (self.l, &self.props);
        let c = n / (30.0 * l);
        let blk = [[36.0 * c, 3.0 * l * c, -36.0 * c, 3.0 * l * c], [3.0 * l * c, 4.0 * l * l * c, -3.0 * l * c, -l * l * c], [-36.0 * c, -3.0 * l * c, 36.0 * c, -3.0 * l * c], [3.0 * l * c, -l * l * c, -3.0 * l * c, 4.0 * l * l * c]];
        let mut m = zero();
        place(&mut m, &blk, XY, false);
        place(&mut m, &blk, XZ, true);
        let tors = n * (p.iy + p.iz) / (p.a * l);
        pair(&mut m, 3, 9, tors, -tors);
        self.condense(&m)
    }

    /// 🏋️ Condensed mass matrix, consistent or lumped.
    pub fn mass(&self, kind: MassType) -> M12 {
        let (l, p) = (self.l, &self.props);
        let (ra, ip) = (p.rho * p.a, p.rho * (p.iy + p.iz));
        let mut m = zero();
        match kind {
            MassType::Consistent => {
                let c = ra * l / 420.0;
                let blk = [[156.0 * c, 22.0 * l * c, 54.0 * c, -13.0 * l * c], [22.0 * l * c, 4.0 * l * l * c, 13.0 * l * c, -3.0 * l * l * c], [54.0 * c, 13.0 * l * c, 156.0 * c, -22.0 * l * c], [-13.0 * l * c, -3.0 * l * l * c, -22.0 * l * c, 4.0 * l * l * c]];
                place(&mut m, &blk, XY, false);
                place(&mut m, &blk, XZ, true);
                pair(&mut m, 0, 6, ra * l / 3.0, ra * l / 6.0);
                pair(&mut m, 3, 9, ip * l / 3.0, ip * l / 6.0);
            }
            MassType::Lumped => {
                for i in [0, 1, 2, 6, 7, 8] {
                    m[i][i] = ra * l / 2.0;
                }
                m[3][3] = ip * l / 2.0;
                m[9][9] = ip * l / 2.0;
            }
        }
        self.condense(&m)
    }

    /// 📥️ Condensed equivalent nodal load for linearly varying local line loads and thermal actions `(alpha, uniform, gradient_y, gradient_z)`.
    pub fn load_vector(&self, lines: &[([f64; 3], [f64; 3])], thermal: &[(f64, f64, f64, f64)]) -> [f64; 12] {
        let (l, p) = (self.l, &self.props);
        let mut f = [0.0; 12];
        for (w0, w1) in lines {
            for (xi, wt) in GAUSS {
                let s = 0.5 * (xi + 1.0);
                let wt = wt * 0.5 * l;
                let w = [0, 1, 2].map(|i| w0[i] + (w1[i] - w0[i]) * s);
                let h = [1.0 - 3.0 * s * s + 2.0 * s.powi(3), l * (s - 2.0 * s * s + s.powi(3)), 3.0 * s * s - 2.0 * s.powi(3), l * (-s * s + s.powi(3))];
                f[0] += (1.0 - s) * w[0] * wt;
                f[6] += s * w[0] * wt;
                for q in 0..4 {
                    let sign = if q % 2 == 1 { -1.0 } else { 1.0 };
                    f[XY[q]] += h[q] * w[1] * wt;
                    f[XZ[q]] += sign * h[q] * w[2] * wt;
                }
            }
        }
        for &(alpha, du, gy, gz) in thermal {
            let ea = p.e * p.a * alpha * du;
            f[0] -= ea;
            f[6] += ea;
            f[5] += p.e * p.iz * alpha * gy;
            f[11] -= p.e * p.iz * alpha * gy;
            f[4] -= p.e * p.iy * alpha * gz;
            f[10] += p.e * p.iy * alpha * gz;
        }
        let tt = transpose(&self.t);
        mat_vec(&tt, &f)
    }
}

/// 🪨️ Uncondensed local stiffness (Euler-Bernoulli, or Timoshenko when `shear` and the shear areas are set).
pub fn stiffness(p: &Props, l: f64) -> M12 {
    let phi = |ei: f64, as_: f64| if p.shear && as_ > 0.0 { 12.0 * ei / (p.g * as_ * l * l) } else { 0.0 };
    let mut k = zero();
    pair(&mut k, 0, 6, p.e * p.a / l, -p.e * p.a / l);
    pair(&mut k, 3, 9, p.g * p.j / l, -p.g * p.j / l);
    place(&mut k, &bending(p.e * p.iz, l, phi(p.e * p.iz, p.asy)), XY, false);
    place(&mut k, &bending(p.e * p.iy, l, phi(p.e * p.iy, p.asz)), XZ, true);
    k
}
