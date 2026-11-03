//! 🔩 Local ductility detailing of the critical regions of RC beams and columns (EN 1998-1 5.4.3 DCM, 5.5.3 DCH): critical region length l_cr, hoop diameter and spacing limits, the spacing of restrained bars, the confinement α·ω_wd against Eq. (5.15) and the total longitudinal ratio ρ_tot. Every constant is a registry row `ndp.en1998.en1.rc.*` (unverified). See [`super::part_1`] for the ρ limits and the ω_wd minimum.

use super::part_1;
use semio_s_artifact_norm_en1998_model::{DuctilityClass, En1998CriticalRegion, En1998System, MemberKind};
use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice};

/// 🔩 Required and provided values of one critical region; a value that cannot be computed from the given inputs is NaN.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detailing {
    pub column: bool,
    pub critical_length_m: f64,
    pub whole_height: bool,
    pub hoop_diameter_min_m: f64,
    pub hoop_spacing_max_m: f64,
    pub bar_spacing_max_m: f64,
    pub alpha_n: f64,
    pub alpha_s: f64,
    pub alpha: f64,
    pub omega_wd: f64,
    pub omega_wd_min: f64,
    pub nu_d: f64,
    pub nu_d_max: f64,
    pub eps_sy_d: f64,
    pub mu_phi: f64,
    pub confinement_required: f64,
    pub rho_tot: f64,
    pub rho_min: f64,
    pub rho_max: f64,
}

fn class_key(class: DuctilityClass) -> Option<&'static str> {
    match class {
        DuctilityClass::Dcm => Some("dcm"),
        DuctilityClass::Dch => Some("dch"),
        DuctilityClass::Dcl => None,
    }
}

/// 🔩 The system whose ductility class governs the detailing of a building: the strictest class, the last of equals.
pub fn governing_system(systems: &[En1998System]) -> Option<&En1998System> {
    let rank = |class: DuctilityClass| match class {
        DuctilityClass::Dch => 2,
        DuctilityClass::Dcm => 1,
        DuctilityClass::Dcl => 0,
    };
    systems.iter().max_by_key(|s| rank(s.ductility_class))
}

/// 🔩 Curvature ductility factor μ_φ = 2 q₀ − 1 for T₁ ≥ T_C and 1 + 2 (q₀ − 1) T_C/T₁ for T₁ < T_C (EN 1998-1 5.2.3.4(3)); NaN without a period.
pub fn curvature_ductility(q0: f64, t1: f64, tc: f64) -> f64 {
    if t1 >= tc { 2.0 * q0 - 1.0 } else if t1 > 0.0 { 1.0 + 2.0 * (q0 - 1.0) * tc / t1 } else { f64::NAN }
}

/// 🔩 Confinement effectiveness α_n = 1 − Σ b_i²/(6 b_0 h_0) with Σ b_i² = n b_i² of the restrained bars.
pub fn alpha_n(count: f64, spacing_m: f64, b0: f64, h0: f64) -> f64 {
    (1.0 - count * spacing_m * spacing_m / (6.0 * b0 * h0)).max(0.0)
}

/// 🔩 Confinement effectiveness α_s = (1 − s/2b_0)(1 − s/2h_0) of the hoop spacing s.
pub fn alpha_s(spacing_m: f64, b0: f64, h0: f64) -> f64 {
    ((1.0 - spacing_m / (2.0 * b0)) * (1.0 - spacing_m / (2.0 * h0))).max(0.0)
}

/// 🔩 Mechanical volumetric ratio of the confining hoops ω_wd = ρ_w f_ydw/f_cd with ρ_w = A_bw (n_x b_0 + n_y h_0)/(b_0 h_0 s), the hoop steel volume per hoop spacing over the volume of the confined core.
pub fn omega_wd(r: &En1998CriticalRegion) -> f64 {
    let area = std::f64::consts::PI * r.hoop_diameter_m * r.hoop_diameter_m / 4.0;
    let volume_ratio = area * (f64::from(r.hoop_legs_x) * r.core_width_m + f64::from(r.hoop_legs_y) * r.core_depth_m) / (r.core_width_m * r.core_depth_m * r.hoop_spacing_m);
    volume_ratio * r.hoop_design_yield_mpa / r.concrete_design_strength_mpa
}

/// 🔩 Normalised design axial force ν_d = N_Ed/(A_c f_cd) with N_Ed in newtons and f_cd in MPa.
pub fn nu_d(r: &En1998CriticalRegion) -> f64 {
    r.axial_force_n / (r.section_width_m * r.section_depth_m * r.concrete_design_strength_mpa * 1.0e6)
}

/// 🔩 Detailing limits and provided values of a critical region of a beam or column of the ductility class, `None` for DCL, a wall or brace, or a missing section; `q0`, `t1` and `tc` give μ_φ of Eq. (5.15).
pub fn detail(r: &En1998CriticalRegion, role: MemberKind, class: DuctilityClass, q0: f64, t1: f64, tc: f64, annex: AnnexChoice) -> Option<Detailing> {
    let key = class_key(class)?;
    let column = match role {
        MemberKind::Column => true,
        MemberKind::Beam => false,
        MemberKind::Wall | MemberKind::Brace => return None,
    };
    if r.section_width_m <= 0.0 || r.section_depth_m <= 0.0 || r.core_width_m <= 0.0 || r.core_depth_m <= 0.0 {
        return None;
    }
    let n = |name: &str| ndp_number(&format!("ndp.en1998.en1.rc.{name}"), annex);
    let (large, core) = (r.section_width_m.max(r.section_depth_m), r.core_width_m.min(r.core_depth_m));
    let whole_height = column && r.clear_length_m / large < n("column.critical-length.full-height-slenderness");
    let critical_length_m = if !column {
        n(&format!("beam.critical-length.depth-factor.{key}")) * r.section_depth_m
    } else if whole_height {
        r.clear_length_m
    } else {
        large.max(r.clear_length_m / n("column.critical-length.clear-length-divisor")).max(n("column.critical-length.min-m"))
    };
    let (hoop_spacing_max_m, hoop_diameter_min_m) = if column {
        let spacing = (core / n(&format!("column.hoop-spacing.core-divisor.{key}"))).min(n(&format!("column.hoop-spacing.max-m.{key}"))).min(n(&format!("column.hoop-spacing.bar-diameter-factor.{key}")) * r.longitudinal_bar_diameter_m);
        let factor = if class == DuctilityClass::Dch { n("hoop-diameter.dch-longitudinal-factor") * r.longitudinal_bar_max_diameter_m * (r.steel_design_yield_mpa / r.hoop_design_yield_mpa).sqrt() } else { 0.0 };
        (spacing, n(&format!("hoop-diameter.min-m.{key}")).max(if factor.is_finite() { factor } else { 0.0 }))
    } else {
        let spacing = (r.section_depth_m / n("beam.hoop-spacing.depth-divisor")).min(n("beam.hoop-spacing.hoop-diameter-factor") * r.hoop_diameter_m).min(n(&format!("beam.hoop-spacing.max-m.{key}"))).min(n(&format!("beam.hoop-spacing.bar-diameter-factor.{key}")) * r.longitudinal_bar_diameter_m);
        (spacing, n(&format!("hoop-diameter.min-m.{key}")))
    };
    let (b0, h0, s) = (r.core_width_m, r.core_depth_m, r.hoop_spacing_m);
    let (alpha_n, alpha_s) = (alpha_n(f64::from(r.restrained_bar_count), r.restrained_bar_spacing_m, b0, h0), alpha_s(s, b0, h0));
    let strengths = r.concrete_design_strength_mpa > 0.0 && r.hoop_design_yield_mpa > 0.0 && r.steel_design_yield_mpa > 0.0;
    let confined = column && strengths && s > 0.0 && r.hoop_diameter_m > 0.0;
    let nan = f64::NAN;
    let mu_phi = curvature_ductility(q0, t1, tc);
    let eps_sy_d = if strengths { r.steel_design_yield_mpa / ndp_number("ndp.en1998.en3.annex-a.steel-modulus-mpa", annex) } else { nan };
    let nu_d = if strengths { nu_d(r) } else { nan };
    let (rho_min, rho_max) = if column { part_1::rc_rho_limits("column", class) } else { (nan, nan) };
    Some(Detailing {
        column,
        critical_length_m,
        whole_height,
        hoop_diameter_min_m,
        hoop_spacing_max_m,
        bar_spacing_max_m: n(&format!("restrained-bar-spacing.max-m.{key}")),
        alpha_n,
        alpha_s,
        alpha: alpha_n * alpha_s,
        omega_wd: if confined { omega_wd(r) } else { nan },
        omega_wd_min: if column { part_1::rc_omega_wd_min(class) } else { nan },
        nu_d,
        nu_d_max: if column { n(&format!("column.nu-d-max.{key}")) } else { nan },
        eps_sy_d,
        mu_phi,
        confinement_required: if column && strengths { n("confinement.mu-phi-coefficient") * mu_phi * nu_d * eps_sy_d * r.section_width_m / b0 - n("confinement.offset") } else { nan },
        rho_tot: if column { r.longitudinal_steel_area_m2 / (r.section_width_m * r.section_depth_m) } else { nan },
        rho_min,
        rho_max,
    })
}

/// 🔩 Governing ratio of a critical region, the largest of the ratios of required to provided (limits that are maxima: provided over limit): critical length, hoop diameter, hoop spacing, bar spacing, confinement α·ω_wd and ω_wd minimum, ρ_tot limits and ν_d; non-finite parts are ignored.
pub fn governing_ratio(r: &En1998CriticalRegion, d: &Detailing) -> f64 {
    let up = |required: f64, provided: f64| if provided > 0.0 { required / provided } else if required > 0.0 { f64::INFINITY } else { 0.0 };
    let rho = if d.rho_tot.is_finite() { up(d.rho_tot, d.rho_max).max(up(d.rho_min, d.rho_tot)) } else { f64::NAN };
    let axial = if d.nu_d.is_finite() && d.nu_d_max.is_finite() { up(d.nu_d, d.nu_d_max) } else { f64::NAN };
    let confinement = if d.omega_wd.is_finite() { up(d.confinement_required, d.alpha * d.omega_wd).max(up(d.omega_wd_min, d.omega_wd)) } else { f64::NAN };
    [up(d.critical_length_m, r.hooped_length_m), up(d.hoop_diameter_min_m, r.hoop_diameter_m), up(r.hoop_spacing_m, d.hoop_spacing_max_m), up(r.restrained_bar_spacing_m, d.bar_spacing_max_m), rho, axial, confinement]
        .into_iter()
        .filter(|v| v.is_finite() || *v == f64::INFINITY)
        .fold(0.0, f64::max)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
