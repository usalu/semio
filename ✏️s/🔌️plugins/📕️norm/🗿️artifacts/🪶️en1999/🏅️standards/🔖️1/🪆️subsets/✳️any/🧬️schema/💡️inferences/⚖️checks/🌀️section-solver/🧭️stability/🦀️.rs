//! 🧭️ Stability constants of a polygon section in its principal axes: the cubic moment behind the Wagner coefficient, the lateral-torsional buckling moment of mono-symmetric and asymmetric sections and the torsional-flexural buckling load. SI metres; y is horizontal, z vertical.
//! 🔬 Independent references: the Wagner coefficient and shear centre of `sectionproperties` and the 3x3 eigenproblem of `scipy` (`🧰️scripts/en1999-wave4/wagner_ref.py`).

use super::Pt;
use std::f64::consts::PI;

/// 🧭️ Section values in a centroidal frame rotated to the principal axes: `i_y = ∫z²dA`, `i_z = ∫y²dA`, the shear-centre offsets `(y0, z0)` from the centroid and `k = ∫z(y²+z²)dA`, the cubic moment behind the Wagner coefficient β_y = k/I_y − 2·z0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame { pub theta: f64, pub i_y: f64, pub i_z: f64, pub y0: f64, pub z0: f64, pub k: f64 }

/// 🧭️ The frame about the principal axes nearest to (y, z) and the frame whose y axis is the strong axis (identical when the section is stronger about y).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frames { pub near: Frame, pub strong: Frame }

fn cubic(points: &[Pt]) -> f64 {
    let f = |y: f64, z: f64| z * (y * y + z * z);
    let (mut sum, mut area) = (0.0, 0.0);
    for i in 0..points.len() {
        let (p, q) = (points[i], points[(i + 1) % points.len()]);
        let s = (p.0 * q.1 - q.0 * p.1) / 2.0;
        let c = ((p.0 + q.0) / 3.0, (p.1 + q.1) / 3.0);
        let o = (0.2 * (p.0 + q.0), 0.2 * (p.1 + q.1));
        let a = (0.6 * p.0 + 0.2 * q.0, 0.6 * p.1 + 0.2 * q.1);
        let b = (0.6 * q.0 + 0.2 * p.0, 0.6 * q.1 + 0.2 * p.1);
        sum += s * (-27.0 / 48.0 * f(c.0, c.1) + 25.0 / 48.0 * (f(o.0, o.1) + f(a.0, a.1) + f(b.0, b.1)));
        area += s;
    }
    if area < 0.0 { -sum } else { sum }
}

fn ring_moments(points: &[Pt]) -> [f64; 6] {
    let n = points.len();
    let mut m = [0.0; 6];
    for i in 0..n {
        let (p, q) = (points[i], points[(i + 1) % n]);
        let c = p.0 * q.1 - q.0 * p.1;
        m[0] += c / 2.0;
        m[1] += (p.0 + q.0) * c / 6.0;
        m[2] += (p.1 + q.1) * c / 6.0;
        m[3] += (p.1 * p.1 + p.1 * q.1 + q.1 * q.1) * c / 12.0;
        m[4] += (p.0 * p.0 + p.0 * q.0 + q.0 * q.0) * c / 12.0;
        m[5] += (p.0 * q.1 + 2.0 * p.0 * p.1 + 2.0 * q.0 * q.1 + q.0 * p.1) * c / 24.0;
    }
    if m[0] < 0.0 { for v in &mut m { *v = -*v; } }
    m
}

/// 🧭️ Angle θ in [−45°, 45°] that rotates (y, z) to the principal axes nearest to them: tan 2θ = −2·I_yz/(I_y − I_z).
pub fn principal_angle(iy: f64, iz: f64, iyz: f64) -> f64 {
    let theta = 0.5 * (-2.0 * iyz).atan2(iy - iz);
    if theta > PI / 4.0 { theta - PI / 2.0 } else if theta < -PI / 4.0 { theta + PI / 2.0 } else { theta }
}

/// 🧭️ The frames of the polygon `outer` minus `holes` for the shear centre `(y_s, z_s)` (offsets from the centroid): the principal axes nearest to (y, z) and the frame whose y axis is the strong axis. `None` for a polygon without area.
pub fn frames(outer: &[Pt], holes: &[&[Pt]], y_s: f64, z_s: f64) -> Option<Frames> {
    let mut m = ring_moments(outer);
    for h in holes {
        let hm = ring_moments(h);
        for i in 0..6 { m[i] -= hm[i]; }
    }
    if m[0] <= 0.0 { return None; }
    let (yg, zg) = (m[1] / m[0], m[2] / m[0]);
    let inertia = (m[3] - m[0] * zg * zg, m[4] - m[0] * yg * yg, m[5] - m[0] * yg * zg);
    let theta = principal_angle(inertia.0, inertia.1, inertia.2);
    let make = |theta: f64| {
        let (c, s) = (theta.cos(), theta.sin());
        let cubic_of = |ring: &[Pt]| cubic(&ring.iter().map(|p| { let (y, z) = (p.0 - yg, p.1 - zg); (y * c + z * s, -y * s + z * c) }).collect::<Vec<Pt>>());
        let k = cubic_of(outer) - holes.iter().map(|h| cubic_of(*h)).sum::<f64>();
        Frame { theta, i_y: inertia.0 * c * c + inertia.1 * s * s - 2.0 * inertia.2 * s * c, i_z: inertia.1 * c * c + inertia.0 * s * s + 2.0 * inertia.2 * s * c, y0: y_s * c + z_s * s, z0: -y_s * s + z_s * c, k }
    };
    let near = make(theta);
    Some(Frames { near, strong: if near.i_y >= near.i_z { near } else { make(theta + PI / 2.0) } })
}

/// 🦋️ Wagner half-coefficient z_j = z0 − k/(2·I_y) for compression on the +z side (`up`), its negative for compression on the −z side; positive when the compression flange is the larger one.
pub fn z_j(f: &Frame, up: bool) -> f64 {
    let zj = f.z0 - 0.5 * f.k / f.i_y.max(1e-300);
    if up { zj } else { -zj }
}

/// 🦋️ Elastic critical moment of lateral-torsional buckling about the strong axis for a fork-supported span `l` under a moment gradient factor `c1` with the load at the shear centre: M_cr = C1·π²·E·I_z/L²·(√(I_w/I_z + L²·G·I_t/(π²·E·I_z) + z_j²) + z_j). Exact for the uniform moment (energy method, one half wave); doubly symmetric sections give z_j = 0.
pub fn m_cr_general(c1: f64, e: f64, g: f64, f: &Frame, i_t: f64, i_w: f64, l: f64, up: bool) -> f64 {
    if l <= 0.0 || f.i_z <= 0.0 { return f64::INFINITY; }
    let zj = z_j(f, up);
    c1 * PI * PI * e * f.i_z / (l * l) * ((i_w / f.i_z + l * l * g * i_t / (PI * PI * e * f.i_z) + zj * zj).sqrt() + zj)
}

/// 🧭️ `(N_cr,T, N_cr,TF)` of a member: the smallest root of i0²(N−N_u)(N−N_v)(N−N_T) − N²·z0²(N−N_v) − N²·y0²(N−N_u) = 0 with N_u = π²E·I_z/L_z² (displacement along y), N_v = π²E·I_y/L_y² (along z), N_T = (G·I_t + π²E·I_w/L_T²)/i0², i0² = (I_y + I_z)/A + y0² + z0². A length of zero or less is an infinite buckling length.
pub fn n_cr_tf(e: f64, g: f64, a: f64, f: &Frame, i_t: f64, i_w: f64, l_z: f64, l_y: f64, l_t: f64) -> (f64, f64) {
    let cap = |v: f64| if v.is_finite() { v } else { 1.0e30 };
    let i0 = (f.i_y + f.i_z) / a.max(1e-300) + f.y0 * f.y0 + f.z0 * f.z0;
    let p_u = if l_z > 0.0 { cap(PI * PI * e * f.i_z / (l_z * l_z)) } else { 1.0e30 };
    let p_v = if l_y > 0.0 { cap(PI * PI * e * f.i_y / (l_y * l_y)) } else { 1.0e30 };
    let p_t = if l_t > 0.0 { (g * i_t + PI * PI * e * i_w / (l_t * l_t)) / i0 } else { 1.0e30 };
    let cubic_f = |n: f64| i0 * (n - p_u) * (n - p_v) * (n - p_t) - n * n * f.z0 * f.z0 * (n - p_v) - n * n * f.y0 * f.y0 * (n - p_u);
    let (mut lo, mut hi) = (0.0, p_u.min(p_v).min(p_t));
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if cubic_f(mid) < 0.0 { lo = mid; } else { hi = mid; }
    }
    (p_t, 0.5 * (lo + hi))
}

