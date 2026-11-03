//! 🦋️ Lateral-torsional buckling of one thin-walled member: Hermite elements with `[v, v', φ, φ']` per node (warping DOF `φ'`).

use crate::linalg::{generalized_dense, Dense};
use crate::*;

const GAUSS: [(f64, f64); 4] = [
    (-0.861_136_311_594_052_6, 0.347_854_845_137_453_8),
    (-0.339_981_043_584_856_3, 0.652_145_154_862_546_1),
    (0.339_981_043_584_856_3, 0.652_145_154_862_546_1),
    (0.861_136_311_594_052_6, 0.347_854_845_137_453_8),
];

fn interp(table: &[(f64, f64)], x: f64) -> f64 {
    match table {
        [] => 0.0,
        [(_, v)] => *v,
        _ => {
            if x <= table[0].0 {
                return table[0].1;
            }
            for w in table.windows(2) {
                if x <= w[1].0 {
                    let t = (x - w[0].0) / (w[1].0 - w[0].0);
                    return w[0].1 + t * (w[1].1 - w[0].1);
                }
            }
            table[table.len() - 1].1
        }
    }
}

fn hermite(h: f64, xi: f64) -> ([f64; 4], [f64; 4], [f64; 4]) {
    (
        [1.0 - 3.0 * xi * xi + 2.0 * xi.powi(3), h * (xi - 2.0 * xi * xi + xi.powi(3)), 3.0 * xi * xi - 2.0 * xi.powi(3), h * (-xi * xi + xi.powi(3))],
        [(-6.0 * xi + 6.0 * xi * xi) / h, 1.0 - 4.0 * xi + 3.0 * xi * xi, (6.0 * xi - 6.0 * xi * xi) / h, -2.0 * xi + 3.0 * xi * xi],
        [(-6.0 + 12.0 * xi) / (h * h), (-4.0 + 6.0 * xi) / h, (6.0 - 12.0 * xi) / (h * h), (-2.0 + 6.0 * xi) / h],
    )
}

/// 📈️ Piecewise-linear moment table of constant moment `m` over `length`.
pub fn uniform_moment(m: f64, length: f64) -> Vec<(f64, f64)> {
    vec![(0.0, m), (length, m)]
}

/// 📉️ Moment table with end moments `m0` at x=0 and `m1` at x=L.
pub fn end_moments(m0: f64, m1: f64, length: f64) -> Vec<(f64, f64)> {
    vec![(0.0, m0), (length, m1)]
}

/// ☔️ Sagging moment table `q x (L - x) / 2` of a uniform load on a simply supported span, sampled at `samples + 1` points.
pub fn uniform_load_moment(q: f64, length: f64, samples: usize) -> Vec<(f64, f64)> {
    (0..=samples).map(|i| length * i as f64 / samples as f64).map(|x| (x, q * x * (length - x) / 2.0)).collect()
}

/// 📍️ Sagging moment table `P L / 4` peak of a central point load on a simply supported span.
pub fn central_point_moment(p: f64, length: f64) -> Vec<(f64, f64)> {
    vec![(0.0, 0.0), (length / 2.0, p * length / 4.0), (length, 0.0)]
}

/// 🍴️ Fork supports at both ends: lateral deflection and twist held, warping and minor-axis rotation free.
pub fn fork_end() -> LtbEnd {
    LtbEnd { lateral: true, twist: true, minor_rotation: false, warping: false }
}

/// 🧱️ Full fixity at an end.
pub fn fixed_end() -> LtbEnd {
    LtbEnd { lateral: true, twist: true, minor_rotation: true, warping: true }
}

fn hold(fixed: &mut [bool], i: usize, end: &LtbEnd) {
    fixed[4 * i] |= end.lateral;
    fixed[4 * i + 1] |= end.minor_rotation;
    fixed[4 * i + 2] |= end.twist;
    fixed[4 * i + 3] |= end.warping;
}

/// 🐛️ Smallest positive load factor of the member's moment, line and point loads; `mcr` is that factor times the peak moment.
pub fn ltb_analyze(m: &LtbMember, control: &Control<'_>) -> MechanicsResult<LtbResult> {
    if !(m.length > 0.0 && m.e > 0.0 && m.g > 0.0 && m.iz > 0.0 && m.j > 0.0 && m.iw >= 0.0) {
        return Err(MechanicsError::invalid("LTB member needs positive length, E, G, Iz, J and non-negative Iw", "Kippstab braucht positive Länge, E, G, Iz, J und nicht negatives Iw"));
    }
    let target = m.length / if m.elements == 0 { 32 } else { m.elements } as f64;
    let mut cuts: Vec<f64> = vec![0.0, m.length];
    cuts.extend(m.moment.iter().map(|p| p.0));
    cuts.extend(m.line_load.iter().map(|p| p.0));
    cuts.extend(m.point_loads.iter().map(|p| p.0));
    cuts.extend(m.restraints.iter().map(|r| r.x));
    cuts.retain(|x| (0.0..=m.length).contains(x));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9 * m.length);
    let mut xs = vec![0.0];
    for w in cuts.windows(2) {
        let parts = ((w[1] - w[0]) / target).ceil().max(1.0) as usize;
        (1..=parts).for_each(|i| xs.push(w[0] + (w[1] - w[0]) * i as f64 / parts as f64));
    }
    let nodes = xs.len();
    let n = 4 * nodes;
    let mut k = Dense::zeros(n);
    let mut g = Dense::zeros(n);
    for e in 0..nodes - 1 {
        control.check()?;
        let (x0, h) = (xs[e], xs[e + 1] - xs[e]);
        let dofs: [usize; 8] = [4 * e, 4 * e + 1, 4 * e + 4, 4 * e + 5, 4 * e + 2, 4 * e + 3, 4 * e + 6, 4 * e + 7];
        for (xi, wt) in GAUSS {
            let s = 0.5 * (xi + 1.0);
            let x = x0 + s * h;
            let w = wt * 0.5 * h;
            let (n0, n1, n2) = hermite(h, s);
            let (mx, q) = (interp(&m.moment, x), interp(&m.line_load, x));
            for a in 0..4 {
                for b in 0..4 {
                    k.add(dofs[a], dofs[b], m.e * m.iz * n2[a] * n2[b] * w);
                    k.add(dofs[4 + a], dofs[4 + b], (m.e * m.iw * n2[a] * n2[b] + m.g * m.j * n1[a] * n1[b]) * w);
                    g.add(dofs[a], dofs[b], m.axial * n1[a] * n1[b] * w);
                    g.add(dofs[4 + a], dofs[4 + b], (m.axial * m.polar_radius_sq * n1[a] * n1[b] + m.wagner * mx * n1[a] * n1[b] - q * m.load_height * n0[a] * n0[b]) * w);
                    g.add(dofs[a], dofs[4 + b], mx * n2[a] * n0[b] * w);
                    g.add(dofs[4 + b], dofs[a], mx * n2[a] * n0[b] * w);
                }
            }
        }
    }
    let node_at = |x: f64| xs.iter().position(|v| (v - x).abs() < 1e-9 * m.length);
    for &(x, p) in &m.point_loads {
        let Some(i) = node_at(x) else { continue };
        g.add(4 * i + 2, 4 * i + 2, -p * m.load_height);
    }
    let mut fixed = vec![false; n];
    hold(&mut fixed, 0, &m.end_i);
    hold(&mut fixed, nodes - 1, &m.end_j);
    for r in &m.restraints {
        if let Some(i) = node_at(r.x) {
            fixed[4 * i] |= r.lateral;
            fixed[4 * i + 2] |= r.twist;
        }
    }
    let free: Vec<usize> = (0..n).filter(|&i| !fixed[i]).collect();
    let nf = free.len();
    let mut kr = Dense::zeros(nf);
    let mut br = Dense::zeros(nf);
    for (a, &i) in free.iter().enumerate() {
        for (b, &j) in free.iter().enumerate() {
            kr.a[a * nf + b] = 0.5 * (k.at(i, j) + k.at(j, i));
            br.a[a * nf + b] = -0.5 * (g.at(i, j) + g.at(j, i));
        }
    }
    let pairs = generalized_dense(&kr, &br, 1, control)?;
    let Some((alpha_cr, x)) = pairs.into_iter().next() else {
        return Err(MechanicsError::NoPositiveEigenvalue);
    };
    let mut full = vec![0.0; n];
    free.iter().zip(&x).for_each(|(&i, &v)| full[i] = v);
    let mut shape = Vec::new();
    for e in 0..nodes - 1 {
        let h = xs[e + 1] - xs[e];
        let u = [full[4 * e], full[4 * e + 1], full[4 * e + 4], full[4 * e + 5]];
        let t = [full[4 * e + 2], full[4 * e + 3], full[4 * e + 6], full[4 * e + 7]];
        for i in 0..4 {
            let s = i as f64 / 4.0;
            let (n0, _, _) = hermite(h, s);
            shape.push(LtbShapePoint { x: xs[e] + s * h, lateral: (0..4).map(|a| n0[a] * u[a]).sum(), twist: (0..4).map(|a| n0[a] * t[a]).sum() });
        }
    }
    shape.push(LtbShapePoint { x: m.length, lateral: full[4 * (nodes - 1)], twist: full[4 * (nodes - 1) + 2] });
    let peak = shape.iter().map(|p| p.lateral.abs()).fold(0.0, f64::max);
    let peak = if peak > 0.0 { peak } else { shape.iter().map(|p| p.twist.abs()).fold(0.0, f64::max) };
    if peak > 0.0 {
        shape.iter_mut().for_each(|p| {
            p.lateral /= peak;
            p.twist /= peak;
        });
    }
    let mmax = m.moment.iter().map(|p| p.1.abs()).fold(0.0, f64::max);
    Ok(LtbResult { alpha_cr, mcr: alpha_cr * mmax, shape })
}
