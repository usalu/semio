//! 🕳️ Welded RHS truss joints with a gap (K and N): validity range, chord face failure and brace effective width failure (mm, N, MPa; angles in degrees). See [EN 1993-1-8 §7.4](https://eurocodes.jrc.ec.europa.eu/).

pub struct GapJoint {
    pub b0: f64,
    pub t0: f64,
    pub fy0: f64,
    pub sigma0: f64,
    pub b1: f64,
    pub h1: f64,
    pub t1: f64,
    pub theta1: f64,
    pub b2: f64,
    pub h2: f64,
    pub t2: f64,
    pub theta2: f64,
    pub gap: f64,
    pub fyi: f64,
}

/// 📐️ β = (b1 + b2 + h1 + h2)/(4 b0).
pub fn beta(j: &GapJoint) -> f64 {
    (j.b1 + j.b2 + j.h1 + j.h2) / (4.0 * j.b0)
}

/// 📐️ k_n of the chord stress (1.0 for tension, 1.3 - 0.4 n/β ≤ 1.0 in compression, n = σ_0,Ed/f_y0).
pub fn k_n(j: &GapJoint) -> f64 {
    let n = j.sigma0 / j.fy0;
    if n >= 0.0 { 1.0 } else { (1.3 - 0.4 * n / beta(j)).min(1.0) }
}

/// ✅️ Validity range of the K and N gap joint formulas: β ≥ 0.35, 15 ≤ b0/t0 ≤ 35, b_i/t_i ≤ 35, 0.5(1-β) ≤ g/b0 ≤ 1.5(1-β), g ≥ t1 + t2.
pub fn validity_violations(j: &GapJoint) -> Vec<&'static str> {
    let b = beta(j);
    let mut out = Vec::new();
    if b < 0.35 {
        out.push("beta>=0.35");
    }
    if !(15.0..=35.0).contains(&(j.b0 / j.t0)) {
        out.push("15<=b0/t0<=35");
    }
    if j.b1 / j.t1 > 35.0 || j.b2 / j.t2 > 35.0 {
        out.push("bi/ti<=35");
    }
    let g = j.gap / j.b0;
    if g < 0.5 * (1.0 - b) || g > 1.5 * (1.0 - b) {
        out.push("0.5(1-beta)<=g/b0<=1.5(1-beta)");
    }
    if j.gap < j.t1 + j.t2 {
        out.push("g>=t1+t2");
    }
    out
}

/// 🧱️ Chord face failure N_1,Rd = 8.9 k_n f_y0 t0² √γ (b1+b2+h1+h2)/(4 b0 sin θ1)/γ_M5 and N_2,Rd = N_1,Rd sin θ1/sin θ2.
pub fn chord_face_rd(j: &GapJoint, gm5: f64) -> (f64, f64) {
    let gamma = j.b0 / (2.0 * j.t0);
    let n1 = 8.9 * k_n(j) * j.fy0 * j.t0 * j.t0 * gamma.sqrt() * (j.b1 + j.b2 + j.h1 + j.h2) / (4.0 * j.b0 * j.theta1.to_radians().sin()) / gm5;
    (n1, n1 * j.theta1.to_radians().sin() / j.theta2.to_radians().sin())
}

/// 🧱️ Brace failure by effective width N_i,Rd = f_yi t_i (2 h_i - 4 t_i + b_i + b_eff)/γ_M5 with b_eff = 10/(b0/t0) f_y0 t0/(f_yi t_i) b_i ≤ b_i.
pub fn brace_effective_width_rd(j: &GapJoint, h: f64, b: f64, t: f64, gm5: f64) -> f64 {
    let b_eff = (10.0 / (j.b0 / j.t0) * (j.fy0 * j.t0) / (j.fyi * t) * b).min(b);
    j.fyi * t * (2.0 * h - 4.0 * t + b + b_eff) / gm5
}
