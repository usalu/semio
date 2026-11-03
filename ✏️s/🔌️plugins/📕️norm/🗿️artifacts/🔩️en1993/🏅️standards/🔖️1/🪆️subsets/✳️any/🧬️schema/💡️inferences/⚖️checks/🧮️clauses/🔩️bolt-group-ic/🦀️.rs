//! 🔩️ Bolt groups under in-plane shear and moment: the elastic vector method and the instantaneous centre of rotation with the load-deformation law R = R_ult (1 - e^(-mu delta))^lambda of one bolt (mm, N). See [EN 1993-1-8 3.12](https://eurocodes.jrc.ec.europa.eu/).

use semio_s_plugin_norm_compliance::ndp_common;

/// 🧭️ Constants of the load-deformation law of one bolt, read from the registry rows `ndp.en1993.1-8.3.12.ic.*`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Law {
    pub mu: f64,
    pub lambda: f64,
    pub delta_max: f64,
}

pub fn law() -> Law {
    Law { mu: ndp_common("ndp.en1993.1-8.3.12.ic.mu"), lambda: ndp_common("ndp.en1993.1-8.3.12.ic.lambda"), delta_max: ndp_common("ndp.en1993.1-8.3.12.ic.delta-max") }
}

/// 🔩️ Force of one bolt at the relative distance `rho = r / r_max` from the centre of rotation; the farthest bolt reaches `r_ult` once `mu delta_max` is large.
pub fn bolt_force(law: &Law, r_ult: f64, rho: f64) -> f64 {
    r_ult * (1.0 - (-law.mu * law.delta_max * rho).exp()).powf(law.lambda)
}

/// 🧮️ Elastic bolt forces `(x, y)` from the polar moment of the group about its centroid: direct share `F/n` plus the torsion `M r / I_p`.
pub fn elastic_vectors(points: &[(f64, f64)], fx: f64, fy: f64, mz: f64) -> Vec<(f64, f64)> {
    let n = points.len() as f64;
    let ip: f64 = points.iter().map(|(x, y)| x * x + y * y).sum();
    let k = if ip > 0.0 { mz / ip } else { 0.0 };
    points.iter().map(|&(x, y)| (fx / n - k * y, fy / n + k * x)).collect()
}

/// 🧮️ Largest elastic bolt force (N).
pub fn elastic_max(points: &[(f64, f64)], fx: f64, fy: f64, mz: f64) -> f64 {
    elastic_vectors(points, fx, fy, mz).iter().map(|v| v.0.hypot(v.1)).fold(0.0, f64::max)
}

/// 🎯️ Ultimate state of a bolt group about its instantaneous centre: `factor` is the multiple of the applied load `(fx, fy, mz)` the group carries, `forces` the bolt forces at that state and `centre` the instantaneous centre (`None` for a load through the centroid, where the group translates).
#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    pub factor: f64,
    pub centre: Option<(f64, f64)>,
    pub forces: Vec<(f64, f64)>,
}

fn tally(law: &Law, points: &[(f64, f64)], r_ult: f64, centre: (f64, f64), sense: f64) -> (f64, f64, f64, Vec<(f64, f64)>) {
    let rmax = points.iter().map(|p| (p.0 - centre.0).hypot(p.1 - centre.1)).fold(0.0, f64::max);
    let (mut sx, mut sy, mut m) = (0.0, 0.0, 0.0);
    let mut forces = Vec::with_capacity(points.len());
    for &(x, y) in points {
        let (rx, ry) = (x - centre.0, y - centre.1);
        let r = rx.hypot(ry);
        if r <= 1e-12 || rmax <= 0.0 {
            forces.push((0.0, 0.0));
            continue;
        }
        let force = bolt_force(law, r_ult, r / rmax);
        let (gx, gy) = (-sense * force * ry / r, sense * force * rx / r);
        sx += gx;
        sy += gy;
        m += x * gy - y * gx;
        forces.push((gx, gy));
    }
    (sx, sy, m, forces)
}

fn residual(law: &Law, points: &[(f64, f64)], r_ult: f64, unknown: [f64; 3], f: (f64, f64), mz: f64, sense: f64) -> [f64; 3] {
    let (sx, sy, m, _) = tally(law, points, r_ult, (unknown[0], unknown[1]), sense);
    [sx - unknown[2] * f.0, sy - unknown[2] * f.1, m - unknown[2] * mz]
}

fn peak(v: [f64; 3]) -> f64 {
    v.iter().fold(0.0, |a, x| a.max(x.abs()))
}

fn solve3(a: [[f64; 3]; 3], b: [f64; 3]) -> Option<[f64; 3]> {
    let mut m = [[0.0; 4]; 3];
    for i in 0..3 {
        m[i][..3].copy_from_slice(&a[i]);
        m[i][3] = b[i];
    }
    for col in 0..3 {
        let pivot = (col..3).max_by(|&i, &j| m[i][col].abs().total_cmp(&m[j][col].abs()))?;
        if m[pivot][col].abs() < 1e-300 {
            return None;
        }
        m.swap(col, pivot);
        for row in col + 1..3 {
            let factor = m[row][col] / m[col][col];
            for k in col..4 {
                m[row][k] -= factor * m[col][k];
            }
        }
    }
    let mut x = [0.0; 3];
    for i in (0..3).rev() {
        let tail: f64 = (i + 1..3).map(|j| m[i][j] * x[j]).sum();
        x[i] = (m[i][3] - tail) / m[i][i];
    }
    Some(x)
}

/// 🎯️ Solves the instantaneous centre of the group for the load `(fx, fy)` with the moment `mz` about the centroid: a bisection on the symmetric centre line perpendicular to the load, then a damped Newton polish of the three equilibrium equations (centre x, centre y, load factor). `None` when the equilibrium does not converge.
pub fn solve(law: &Law, points: &[(f64, f64)], r_ult: f64, fx: f64, fy: f64, mz: f64) -> Option<Solution> {
    let n = points.len();
    let f = fx.hypot(fy);
    let scale = points.iter().map(|p| p.0.hypot(p.1)).fold(0.0, f64::max);
    if n == 0 || r_ult <= 0.0 || (f == 0.0 && mz == 0.0) {
        return None;
    }
    if f > 0.0 && (scale <= 0.0 || mz.abs() <= 1e-9 * f * scale) {
        let forces = vec![(fx / f * r_ult, fy / f * r_ult); n];
        return Some(Solution { factor: n as f64 * r_ult / f, centre: None, forces });
    }
    if scale <= 0.0 {
        return None;
    }
    let sense = if mz > 0.0 { 1.0 } else { -1.0 };
    if f <= 1e-9 * mz.abs() / scale {
        let (_, _, _, forces) = tally(law, points, r_ult, (0.0, 0.0), sense);
        let moment: f64 = points.iter().zip(&forces).map(|(p, g)| p.0 * g.1 - p.1 * g.0).sum();
        return Some(Solution { factor: moment / mz, centre: Some((0.0, 0.0)), forces });
    }
    let (nx, ny) = (-fy / f, fx / f);
    let at = |t: f64| {
        let centre = (sense * nx * t, sense * ny * t);
        let (sx, sy, m, _) = tally(law, points, r_ult, centre, sense);
        let k = (sx * fx + sy * fy) / (f * f);
        (centre, k, m - k * mz)
    };
    let (mut lo, mut hi) = (1e-9 * scale, 1e3 * scale);
    let mut g_lo = at(lo).2;
    if g_lo * at(hi).2 > 0.0 {
        return None;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let g_mid = at(mid).2;
        if g_lo * g_mid <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            g_lo = g_mid;
        }
    }
    let (centre, k, _) = at(0.5 * (lo + hi));
    let mut unknown = [centre.0, centre.1, k];
    let tolerance = 1e-9 * n as f64 * r_ult;
    for _ in 0..40 {
        let r = residual(law, points, r_ult, unknown, (fx, fy), mz, sense);
        if peak(r) < tolerance {
            break;
        }
        let mut jac = [[0.0; 3]; 3];
        for j in 0..3 {
            let step = if j < 2 { 1e-7 * scale } else { 1e-7 * unknown[2].abs().max(1e-9) };
            let mut shifted = unknown;
            shifted[j] += step;
            let rj = residual(law, points, r_ult, shifted, (fx, fy), mz, sense);
            for i in 0..3 {
                jac[i][j] = (rj[i] - r[i]) / step;
            }
        }
        let Some(delta) = solve3(jac, [-r[0], -r[1], -r[2]]) else { break };
        let mut damp = 1.0;
        while damp > 1e-4 {
            let trial = [unknown[0] + damp * delta[0], unknown[1] + damp * delta[1], unknown[2] + damp * delta[2]];
            if peak(residual(law, points, r_ult, trial, (fx, fy), mz, sense)) < peak(r) {
                break;
            }
            damp /= 2.0;
        }
        unknown = [unknown[0] + damp * delta[0], unknown[1] + damp * delta[1], unknown[2] + damp * delta[2]];
    }
    if peak(residual(law, points, r_ult, unknown, (fx, fy), mz, sense)) > 1e-6 * n as f64 * r_ult || unknown[2] <= 0.0 {
        return None;
    }
    let (_, _, _, forces) = tally(law, points, r_ult, (unknown[0], unknown[1]), sense);
    Some(Solution { factor: unknown[2], centre: Some((unknown[0], unknown[1])), forces })
}
