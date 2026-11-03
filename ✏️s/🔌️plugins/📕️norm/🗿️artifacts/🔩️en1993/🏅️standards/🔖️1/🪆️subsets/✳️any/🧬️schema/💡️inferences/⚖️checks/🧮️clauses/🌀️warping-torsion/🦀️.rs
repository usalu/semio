//! 🌀 Warping torsion of a prismatic member with fork, restrained or free ends: the bimoment B and the torque split T = T_t + T_w along the member from E I_w θ'''' - G I_t θ'' = m (St. Venant plus warping, see [EN 1993-1-1 6.2.7](https://eurocodes.jrc.ec.europa.eu/)). The equation and its end conditions are mechanics, not clause text; the boundary value problem is solved exactly per segment (no series), SI units.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum End {
    Fork,
    Fixed,
    Free,
}

/// 🌀 Rotation θ, bimoment B = -E I_w θ'' and the torque shares T_t = G I_t θ', T_w = -E I_w θ''' at the stations `x`.
#[derive(Clone, Debug, Default)]
pub struct TorsionProfile {
    pub x: Vec<f64>,
    pub theta: Vec<f64>,
    pub bimoment: Vec<f64>,
    pub t_t: Vec<f64>,
    pub t_w: Vec<f64>,
}

impl TorsionProfile {
    /// 🌀 Maximum absolute `(θ, T_t, T_w, B)`.
    pub fn maxima(&self) -> (f64, f64, f64, f64) {
        let top = |v: &[f64]| v.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        (top(&self.theta), top(&self.t_t), top(&self.t_w), top(&self.bimoment))
    }
}

fn basis(k: f64, len: f64, s: f64, order: usize) -> [f64; 4] {
    let (e1, e2) = ((-k * s).exp(), (-k * (len - s)).exp());
    match order {
        0 => [1.0, s, e1, e2],
        1 => [0.0, 1.0, -k * e1, k * e2],
        2 => [0.0, 0.0, k * k * e1, k * k * e2],
        _ => [0.0, 0.0, -k.powi(3) * e1, k.powi(3) * e2],
    }
}

fn particular(m: f64, g_it: f64, x: f64, order: usize) -> f64 {
    match order {
        0 => -m * x * x / (2.0 * g_it),
        1 => -m * x / g_it,
        2 => -m / g_it,
        _ => 0.0,
    }
}

fn solve(mut a: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = a.len();
    for c in 0..n {
        let p = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if a[p][c].abs() < 1e-300 {
            return None;
        }
        a.swap(c, p);
        for r in 0..n {
            if r != c {
                let f = a[r][c] / a[c][c];
                for j in c..=n {
                    a[r][j] -= f * a[c][j];
                }
            }
        }
    }
    Some((0..n).map(|i| a[i][n] / a[i][i]).collect())
}

/// 🌀 Solves the member under a point torque `mt` at `xa` (`udl` false) or a uniform torque per length `mt` (`udl` true) and samples it at `stations + 1` points; `None` for a singular system or non-positive stiffness.
pub fn warping_torsion(l: f64, ei_w: f64, g_it: f64, mt: f64, xa: f64, udl: bool, ends: (End, End), stations: usize) -> Option<TorsionProfile> {
    if !(l > 0.0 && ei_w > 0.0 && g_it > 0.0) {
        return None;
    }
    let k = (g_it / ei_w).sqrt();
    let xa = if udl { 0.5 * l } else { xa.clamp(1e-3 * l, (1.0 - 1e-3) * l) };
    let (len, m) = ([xa, l - xa], if udl { mt } else { 0.0 });
    let x0 = [0.0, xa];
    let ev = |seg: usize, s: f64, order: usize| -> ([f64; 4], f64) { (basis(k, len[seg], s, order), particular(m, g_it, x0[seg] + s, order)) };
    let torque = |seg: usize, s: f64| -> ([f64; 4], f64) {
        let (d1, p1) = ev(seg, s, 1);
        let (d3, p3) = ev(seg, s, 3);
        (std::array::from_fn(|i| g_it * d1[i] - ei_w * d3[i]), g_it * p1 - ei_w * p3)
    };
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut put = |seg: usize, coefficients: [f64; 4], rhs: f64, other: Option<[f64; 4]>| {
        let mut row = vec![0.0; 9];
        for i in 0..4 {
            row[seg * 4 + i] += coefficients[i];
            if let Some(o) = other {
                row[(1 - seg) * 4 + i] -= o[i];
            }
        }
        row[8] = rhs;
        rows.push(row);
    };
    let mut end = |seg: usize, s: f64, kind: End| {
        let (v, pv) = ev(seg, s, 0);
        let (d1, p1) = ev(seg, s, 1);
        let (d2, p2) = ev(seg, s, 2);
        let (t, pt) = torque(seg, s);
        match kind {
            End::Fork => {
                put(seg, v, -pv, None);
                put(seg, d2, -p2, None);
            }
            End::Fixed => {
                put(seg, v, -pv, None);
                put(seg, d1, -p1, None);
            }
            End::Free => {
                put(seg, d2, -p2, None);
                put(seg, t, -pt, None);
            }
        }
    };
    end(0, 0.0, ends.0);
    end(1, len[1], ends.1);
    for order in 0..3 {
        let (a, pa) = ev(0, len[0], order);
        let (b, pb) = ev(1, 0.0, order);
        let mut row = vec![0.0; 9];
        for i in 0..4 {
            row[i] = a[i];
            row[4 + i] = -b[i];
        }
        row[8] = pb - pa;
        rows.push(row);
    }
    let (ta, pta) = torque(0, len[0]);
    let (tb, ptb) = torque(1, 0.0);
    let mut row = vec![0.0; 9];
    for i in 0..4 {
        row[i] = -ta[i];
        row[4 + i] = tb[i];
    }
    row[8] = if udl { 0.0 } else { mt } + pta - ptb;
    rows.push(row);
    let c = solve(rows)?;
    let mut out = TorsionProfile::default();
    for i in 0..=stations {
        let x = l * i as f64 / stations.max(1) as f64;
        let seg = usize::from(x > xa);
        let s = x - x0[seg];
        let value = |order: usize| -> f64 {
            let (b, p) = ev(seg, s, order);
            (0..4).map(|j| b[j] * c[seg * 4 + j]).sum::<f64>() + p
        };
        out.x.push(x);
        out.theta.push(value(0));
        out.t_t.push(g_it * value(1));
        out.bimoment.push(-ei_w * value(2));
        out.t_w.push(-ei_w * value(3));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resistance::warping_torsion_fork;

    #[test]
    fn the_exact_fork_solution_matches_the_fourier_series() {
        let (l, ei_w, g_it) = (5000.0, 210000.0 * 125900.0e6, 81000.0 * 20.12e4);
        for (mt, xa, udl) in [(1.0e6, 2500.0, false), (1.0e6, 1200.0, false), (200.0, 0.0, true)] {
            let exact = warping_torsion(l, ei_w, g_it, mt, xa, udl, (End::Fork, End::Fork), 200).expect("solution");
            let series = warping_torsion_fork(l, ei_w, g_it, mt, xa, udl);
            let (theta, _, _, b) = exact.maxima();
            assert!((b - series.b_max).abs() / series.b_max < 5.0e-3, "B {b} vs {}", series.b_max);
            assert!((theta - series.theta_max).abs() / series.theta_max < 5.0e-3, "theta {theta} vs {}", series.theta_max);
        }
    }

    #[test]
    fn the_cantilever_under_an_end_torque_has_the_closed_form_root_bimoment() {
        let (l, ei_w, g_it, mt) = (3000.0, 210000.0 * 50000.0e6, 81000.0 * 10.0e4, 1.0e6);
        let k = (g_it / ei_w).sqrt();
        let profile = warping_torsion(l, ei_w, g_it, mt, l * (1.0 - 1e-3), false, (End::Fixed, End::Free), 400).expect("solution");
        let closed = mt / k * (k * l).tanh();
        assert!((profile.bimoment[0].abs() - closed).abs() / closed < 5.0e-3, "{} vs {closed}", profile.bimoment[0]);
        assert!(profile.bimoment[400].abs() < 1e-6 * closed);
    }

    #[test]
    fn a_restrained_member_carries_less_bimoment_at_midspan_than_the_fork_member() {
        let (l, ei_w, g_it) = (5000.0, 210000.0 * 125900.0e6, 81000.0 * 20.12e4);
        let fork = warping_torsion(l, ei_w, g_it, 200.0, 0.0, true, (End::Fork, End::Fork), 100).expect("fork");
        let fixed = warping_torsion(l, ei_w, g_it, 200.0, 0.0, true, (End::Fixed, End::Fixed), 100).expect("fixed");
        assert!(fixed.theta[50].abs() < fork.theta[50].abs());
        assert!(fixed.bimoment[0].abs() > 0.0 && fork.bimoment[0].abs() < 1e-6 * fork.bimoment[50].abs());
    }
}
