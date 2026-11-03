//! 🔩️ EN 1993-1-8 fasteners, welds and component method (mm, N, MPa, Nmm). See [EN 1993-1-8](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

pub fn bolt_fyb(class: &str) -> f64 {
    match class {
        "4.6" => 240.0,
        "4.8" => 320.0,
        "5.6" => 300.0,
        "5.8" => 400.0,
        "6.8" => 480.0,
        "8.8" => 640.0,
        "10.9" => 900.0,
        _ => 640.0,
    }
}

pub fn bolt_fub(class: &str) -> f64 {
    match class {
        "4.6" | "4.8" => 400.0,
        "5.6" | "5.8" => 500.0,
        "6.8" => 600.0,
        "8.8" => 800.0,
        "10.9" => 1000.0,
        _ => 800.0,
    }
}

pub fn bolt_as(d: f64) -> f64 {
    match d.round() as i64 {
        12 => 84.3,
        14 => 115.0,
        16 => 157.0,
        18 => 192.0,
        20 => 245.0,
        22 => 303.0,
        24 => 353.0,
        27 => 459.0,
        30 => 561.0,
        33 => 694.0,
        36 => 817.0,
        _ => PI / 4.0 * (d - 0.9382 * (d * 0.15)).powi(2),
    }
}

pub fn hole_diameter(d: f64) -> f64 {
    if d <= 14.0 { d + 1.0 } else if d <= 24.0 { d + 2.0 } else { d + 3.0 }
}

pub fn alpha_v(class: &str, thread_in_shear_plane: bool) -> f64 {
    if !thread_in_shear_plane {
        return 0.6;
    }
    match class {
        "4.6" | "5.6" | "8.8" => 0.6,
        _ => 0.5,
    }
}

pub fn fv_rd(class: &str, d: f64, thread_in_plane: bool, gm2: f64) -> f64 {
    let area = if thread_in_plane { bolt_as(d) } else { PI * d * d / 4.0 };
    alpha_v(class, thread_in_plane) * bolt_fub(class) * area / gm2
}

pub fn beta_lf(lj: f64, d: f64) -> f64 {
    if lj <= 15.0 * d { 1.0 } else { (1.0 - (lj - 15.0 * d) / (200.0 * d)).clamp(0.75, 1.0) }
}

pub fn beta_packing(d: f64, tp: f64) -> f64 {
    (9.0 * d / (8.0 * d + 3.0 * tp)).min(1.0)
}

pub fn alpha_b(end_bolt: bool, e1: f64, p1: f64, d0: f64, fub: f64, fu: f64) -> f64 {
    let ad = if end_bolt { e1 / (3.0 * d0) } else { p1 / (3.0 * d0) - 0.25 };
    ad.min(fub / fu).min(1.0)
}

pub fn k1_bearing(edge_bolt: bool, e2: f64, p2: f64, d0: f64) -> f64 {
    let inner = (1.4 * p2 / d0 - 1.7).min(2.5);
    if edge_bolt { (2.8 * e2 / d0 - 1.7).min(inner) } else { inner }
}

pub fn fb_rd(k1: f64, alpha_b: f64, fu: f64, d: f64, t: f64, gm2: f64, hole_factor: f64) -> f64 {
    hole_factor * k1 * alpha_b * fu * d * t / gm2
}

pub fn hole_bearing_factor(oversize: bool, slotted: bool) -> f64 {
    if slotted { 0.6 } else if oversize { 0.8 } else { 1.0 }
}

pub fn ft_rd(class: &str, d: f64, countersunk: bool, gm2: f64) -> f64 {
    (if countersunk { 0.63 } else { 0.9 }) * bolt_fub(class) * bolt_as(d) / gm2
}

pub fn bp_rd(dm: f64, tp: f64, fu: f64, gm2: f64) -> f64 {
    0.6 * PI * dm * tp * fu / gm2
}

/// 🔩️ Table 3.4 shear/tension interaction as one utilisation, max with the tension-only ratio.
pub fn shear_tension_utilisation(fv_ed: f64, fv_rd: f64, ft_ed: f64, ft_rd: f64) -> f64 {
    (fv_ed / fv_rd + ft_ed / (1.4 * ft_rd)).max(ft_ed / ft_rd)
}

pub fn fp_c(class: &str, d: f64) -> f64 {
    0.7 * bolt_fub(class) * bolt_as(d)
}

pub fn mu_slip(class: char) -> f64 {
    match class {
        'A' => 0.5,
        'B' => 0.4,
        'C' => 0.3,
        _ => 0.2,
    }
}

/// 🔩️ F_s,Rd with tension relief (category B: factor 0.8 on F_t,Ed).
pub fn fs_rd(ks: f64, n: f64, mu: f64, fp: f64, ft_ed: f64, gm3: f64) -> f64 {
    ks * n * mu * (fp - 0.8 * ft_ed) / gm3
}

pub fn block_tearing(fu: f64, fy: f64, a_nt: f64, a_nv: f64, concentric: bool, gm0: f64, gm2: f64) -> f64 {
    (if concentric { 1.0 } else { 0.5 }) * fu * a_nt / gm2 + fy * a_nv / (3.0_f64.sqrt() * gm0)
}

pub fn angle_beta(n_bolts: usize, p1: f64, d0: f64) -> f64 {
    let (b_lo, b_hi) = match n_bolts {
        2 => (0.4, 0.7),
        _ => (0.5, 0.7),
    };
    b_lo + ((p1 - 2.5 * d0) / (2.5 * d0)).clamp(0.0, 1.0) * (b_hi - b_lo)
}

pub fn angle_net_resistance(n_bolts: usize, p1: f64, d0: f64, a_net: f64, e2: f64, t: f64, fu: f64, gm2: f64) -> f64 {
    if n_bolts == 1 { 2.0 * (e2 - 0.5 * d0) * t * fu / gm2 } else { angle_beta(n_bolts, p1, d0) * a_net * fu / gm2 }
}

pub struct SpacingCheck {
    pub name: &'static str,
    pub value: f64,
    pub limit: f64,
    pub is_minimum: bool,
    pub ok: bool,
}

/// 📏️ Table 3.3 minimum and maximum end/edge distances and spacings (steel not exposed).
pub fn spacing_checks(e1: f64, e2: f64, p1: f64, p2: f64, d0: f64, t: f64) -> Vec<SpacingCheck> {
    let mut v = Vec::new();
    let mut add = |name: &'static str, value: f64, limit: f64, is_min: bool| {
        let ok = if is_min { value >= limit } else { value <= limit };
        v.push(SpacingCheck { name, value, limit, is_minimum: is_min, ok });
    };
    add("e1,min", e1, 1.2 * d0, true);
    add("e2,min", e2, 1.2 * d0, true);
    add("p1,min", p1, 2.2 * d0, true);
    add("p2,min", p2, 2.4 * d0, true);
    add("e1,max", e1, 4.0 * t + 40.0, false);
    add("e2,max", e2, 4.0 * t + 40.0, false);
    add("p1,max", p1, (14.0 * t).min(200.0), false);
    add("p2,max", p2, (14.0 * t).min(200.0), false);
    v
}

/// 🔩️ Elastic bolt-group shear on bolt `i` from centroid-relative coordinates, force (fx, fy) and moment about the centroid.
pub fn bolt_group_force(points: &[(f64, f64)], fx: f64, fy: f64, mz: f64, i: usize) -> f64 {
    let n = points.len() as f64;
    let ip: f64 = points.iter().map(|(x, y)| x * x + y * y).sum();
    let (x, y) = points[i];
    let k = if ip > 0.0 { mz / ip } else { 0.0 };
    let rx = fx / n - k * y;
    let ry = fy / n + k * x;
    (rx * rx + ry * ry).sqrt()
}

pub fn beta_w(grade: &str) -> f64 {
    match grade {
        "S235" | "S235W" => 0.8,
        "S275" | "S275N" | "S275NL" | "S275M" | "S275ML" => 0.85,
        "S355" | "S355W" | "S355N" | "S355NL" | "S355M" | "S355ML" => 0.9,
        "S420N" | "S420NL" | "S420M" | "S420ML" | "S420" => 1.0,
        "S460N" | "S460NL" | "S460M" | "S460ML" | "S460Q" | "S460QL" | "S460QL1" | "S460" => 1.0,
        _ => 1.0,
    }
}

pub fn fvw_d(fu: f64, beta_w: f64, gm2: f64) -> f64 {
    fu / (3.0_f64.sqrt() * beta_w * gm2)
}

/// 🔧️ Both criteria of the directional method (EN 1993-1-8 4.5.3.2, Eq. 4.1 and the σ⊥ limit) as utilisations: (`[σ⊥² + 3 (τ⊥² + τ∥²)]^0.5 / (f_u / (β_w γ_M2))`, `|σ⊥| / (0.9 f_u / γ_M2)`).
pub fn weld_directional_criteria(sigma_perp: f64, tau_perp: f64, tau_par: f64, fu: f64, beta_w: f64, gm2: f64) -> (f64, f64) {
    let equiv = (sigma_perp * sigma_perp + 3.0 * (tau_perp * tau_perp + tau_par * tau_par)).sqrt();
    (equiv / (fu / (beta_w * gm2)), sigma_perp.abs() / (0.9 * fu / gm2))
}

/// 🔧️ Directional method (4.5.3.2): the governing utilisation of both criteria for the throat stresses.
pub fn weld_directional(sigma_perp: f64, tau_perp: f64, tau_par: f64, fu: f64, beta_w: f64, gm2: f64) -> f64 {
    let (c1, c2) = weld_directional_criteria(sigma_perp, tau_perp, tau_par, fu, beta_w, gm2);
    c1.max(c2)
}

/// 🔧️ Throat stresses (σ⊥, τ⊥, τ∥) of a fillet weld (EN 1993-1-8 Figure 4.5): the force `f_parallel` along the weld axis acts as τ∥ on the throat area a ℓ; the force `f_transverse` perpendicular to the axis, in the weld cross-section and at angle `psi` to the throat-plane normal, splits into σ⊥ = F cos ψ / (a ℓ) and τ⊥ = F sin ψ / (a ℓ). An equal-leg fillet has ψ = 45° for a force along either leg.
pub fn weld_throat_stresses(f_parallel: f64, f_transverse: f64, psi: f64, a: f64, l: f64) -> (f64, f64, f64) {
    let area = a * l;
    (f_transverse * psi.cos() / area, f_transverse * psi.sin() / area, f_parallel / area)
}

pub fn l_eff_circular(m: f64) -> f64 {
    2.0 * PI * m
}

pub fn l_eff_noncircular_end_plate_row(m: f64, e: f64) -> f64 {
    4.0 * m + 1.25 * e
}

pub fn k10_bolts(as_: f64, lb: f64, n_bolts: f64) -> f64 {
    1.6 * as_ * n_bolts / lb
}

pub fn k_eq(ks: &[f64]) -> f64 {
    1.0 / ks.iter().map(|k| 1.0 / k).sum::<f64>()
}

pub fn s_j_ini(e: f64, z: f64, k_eq: f64) -> f64 {
    e * z * z * k_eq
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointStiffnessClass {
    Rigid,
    SemiRigid,
    Pinned,
}

pub fn classify_stiffness(s_j_ini: f64, e: f64, ib: f64, lb: f64, kb: f64) -> JointStiffnessClass {
    if s_j_ini >= kb * e * ib / lb {
        JointStiffnessClass::Rigid
    } else if s_j_ini <= 0.5 * e * ib / lb {
        JointStiffnessClass::Pinned
    } else {
        JointStiffnessClass::SemiRigid
    }
}

pub fn classify_strength(m_j_rd: f64, m_pl_beam: f64) -> &'static str {
    if m_j_rd >= m_pl_beam { "full" } else if m_j_rd <= 0.25 * m_pl_beam { "nominally-pinned" } else { "partial" }
}

/// 🧱️ Column web panel in shear V_wp,Rd = 0.9 fy Avc/(√3 γM0).
pub fn v_wp_rd(fy: f64, avc: f64, gm0: f64) -> f64 {
    0.9 * fy * avc / (3.0_f64.sqrt() * gm0)
}

/// 🧱️ Compression flange and web of the beam F_c,fb,Rd = M_c,Rd/(h − t_fb).
pub fn fc_fb_rd(mc_rd: f64, h: f64, tfb: f64) -> f64 {
    mc_rd / (h - tfb)
}

/// 🧊 Net tension and net shear areas (A_nt, A_nv) of the block around the outer bolt lines of a group (§3.10.2): `n1` rows along the load with pitch `p1`, `n2` bolts across with spacing `p2`, end distance `e1`, edge distance `e2`, hole `d0`, plate thickness `t`.
pub fn block_tearing_areas(n1: u32, n2: u32, p1: f64, p2: f64, e1: f64, e2: f64, d0: f64, t: f64) -> (f64, f64) {
    let rows = n1.max(1) as f64;
    let l_v = (e1 + (rows - 1.0) * p1 - (rows - 0.5) * d0).max(0.0);
    if n2 <= 1 {
        (((e2 - 0.5 * d0) * t).max(0.0), l_v * t)
    } else {
        (((n2 - 1) as f64 * (p2 - d0) * t).max(0.0), 2.0 * l_v * t)
    }
}

/// 🔘 Designation `m<size>` of a bolt diameter in metres for the registry rows of the head and nut widths (`ndp.en1993.1-8.3.6.head-nut.m20.across-flats`); a diameter that is not a whole millimetre keeps its decimals and so has no row.
pub fn bolt_size_designation(d: f64) -> String {
    let mm = d * 1e3;
    if (mm - mm.round()).abs() < 1e-6 { format!("m{}", mm.round() as i64) } else { format!("m{mm}") }
}

/// 🔘 Mean diameter d_m of the bolt head or nut, whichever is smaller, for the punching resistance (Table 3.4): the mean of the across-points and across-flats dimensions.
pub fn punching_mean_diameter(across_flats: f64, across_points: f64) -> f64 {
    0.5 * (across_flats + across_points)
}

/// 🧵️ Effective width b_eff = t_w + 2 s + 7 k t_f of a plate welded to an unstiffened flange (§4.10), k = (t_f/t_p)(f_y,f/f_y,p) ≤ 1, capped by the plate width `b`.
pub fn weld_flange_beff(tw: f64, s: f64, tf: f64, tp: f64, fy_f: f64, fy_p: f64, b: f64) -> f64 {
    let k = (tf / tp * fy_f / fy_p).min(1.0);
    (tw + 2.0 * s + 7.0 * k * tf).min(b)
}

/// 🪡 Flange thickness t_f at which b_eff reaches `required` (§4.10): linear branch for k = 1, quadratic branch for k < 1.
pub fn weld_flange_thickness_for(required: f64, tw: f64, s: f64, tp: f64, fy_f: f64, fy_p: f64) -> f64 {
    let need = (required - tw - 2.0 * s).max(0.0);
    let linear = need / 7.0;
    if linear * fy_f >= tp * fy_p { linear } else { (need * tp * fy_p / (7.0 * fy_f)).sqrt() }
}

