//! 🔥️ EN 1999-1-2 fire: strength and stiffness reduction, gas and member temperature, critical temperature. See [`super`].

use crate::*;
use semio_s_artifact_norm_en1999_model::AluminiumSection;

/// 🌡️ Temperatures [°C] of the retention table of EN 1999-1-2 Table 3.1/3.2; the factors themselves are the unverified registry rows `ndp.en1999.fire.k-0.2proof.<θ>` and `ndp.en1999.fire.k-e.<θ>`.
pub const TABLE_THETAS: [u32; 10] = [20, 50, 100, 150, 200, 250, 300, 350, 400, 550];

/// 🌡️ `(θ, k_0.2proof,θ, k_E,θ)` rows read once from the registry.
fn table() -> &'static [(f64, f64, f64); 10] {
    static ROWS: std::sync::OnceLock<[(f64, f64, f64); 10]> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| TABLE_THETAS.map(|t| (t as f64, semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1999.fire.k-0.2proof.{t}")), semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1999.fire.k-e.{t}")))))
}

fn interpolate(theta: f64, column: usize) -> f64 {
    let table = table();
    let pick = |r: &(f64, f64, f64)| if column == 1 { r.1 } else { r.2 };
    if theta <= table[0].0 { return pick(&table[0]); }
    for w in table.windows(2) {
        if theta <= w[1].0 {
            let t = (theta - w[0].0) / (w[1].0 - w[0].0);
            return pick(&w[0]) + t * (pick(&w[1]) - pick(&w[0]));
        }
    }
    pick(&table[table.len() - 1])
}

/// 🔥️ k_0.2proof,θ: strength retained at the member temperature θ_a.
pub fn k_theta(theta_a: f64) -> f64 { interpolate(theta_a, 1) }

/// 🔥️ k_E,θ: stiffness retained at the member temperature θ_a.
pub fn k_e_theta(theta_a: f64) -> f64 { interpolate(theta_a, 2) }

/// 🌡️ Critical temperature θ_cr at which k_0.2proof,θ falls to the utilization μ0 (bisection on the monotone table).
pub fn critical_temperature_c(mu0: f64) -> f64 {
    if mu0 >= 1.0 { return 20.0; }
    if mu0 <= 0.0 { return 550.0; }
    let (mut lo, mut hi) = (20.0, 550.0);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if k_theta(mid) > mu0 { lo = mid; } else { hi = mid; }
    }
    0.5 * (lo + hi)
}

/// 🔥️ ISO 834 standard fire gas temperature θ_g(t) = 20 + 345·log10(8·t_min + 1).
pub fn gas_temperature_c(t_s: f64) -> f64 { 20.0 + 345.0 * (8.0 * t_s / 60.0 + 1.0).log10() }

/// 🌡️ Specific heat of aluminium c_a = 0.41·θ + 903 [J/(kg·K)].
pub fn specific_heat(theta_a: f64) -> f64 { 0.41 * theta_a + 903.0 }

pub const DENSITY: f64 = 2700.0;

/// 🔥️ Convective heat transfer coefficient α_c [W/(m²·K)] of the registry (`ndp.en1999.fire-alpha-c`, the same for every annex).
pub fn alpha_c() -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.fire-alpha-c") }

/// 🔥️ Surface emissivity of aluminium of the registry (`ndp.en1999.fire-emissivity`, the same for every annex).
pub fn emissivity() -> f64 { semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.fire-emissivity") }

/// 🌡️ Unprotected member temperature after `duration_s` from the start temperature `theta_0` by explicit stepping of Δθ = k_sh·(A_m/V)/(c_a·ρ)·h_net·Δt (k_sh = 1, Δt ≤ 5 s).
pub fn unprotected_temperature_c(theta_0: f64, am_over_v: f64, duration_s: f64) -> f64 {
    let dt = 5.0;
    let mut theta = theta_0.max(20.0);
    let (alpha_c, emissivity) = (alpha_c(), emissivity());
    let mut t = 0.0;
    while t < duration_s - 1e-9 {
        let step = dt.min(duration_s - t);
        let t_next = t + step;
        let g = gas_temperature_c(t_next);
        let h_c = alpha_c * (g - theta);
        let h_r = emissivity * na_de::STEFAN_BOLTZMANN * ((g + 273.0).powi(4) - (theta + 273.0).powi(4));
        theta += am_over_v / (specific_heat(theta) * DENSITY) * (h_c + h_r) * step;
        t = t_next;
    }
    theta
}

/// 🧱 Insulation of a protected member: thickness d_p [m], conductivity λ_p [W/(m·K)], specific heat c_p [J/(kg·K)] and density ρ_p [kg/m³].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Insulation { pub thickness: f64, pub conductivity: f64, pub specific_heat: f64, pub density: f64 }

impl From<&semio_s_artifact_norm_en1999_model::FireInsulation> for Insulation {
    fn from(row: &semio_s_artifact_norm_en1999_model::FireInsulation) -> Self { Self { thickness: row.thickness, conductivity: row.conductivity, specific_heat: row.specific_heat, density: row.density } }
}

/// 🧱 Protected member temperature after `duration_s` by explicit stepping (Δt = 5 s) of Δθ_a = [λ_p/d_p·(A_p/V)/(c_a·ρ_a)]·(θ_g − θ_a)/(1 + φ/k_1)·Δt − (e^{φ/k_2} − 1)·Δθ_g with φ = (c_p·ρ_p/(c_a·ρ_a))·d_p·A_p/V, Δθ_a ≥ 0 while the gas heats; k_1 and k_2 are the unverified registry rows `ndp.en1999.fire-protected-denominator` and `-exponent`. A non-positive thickness is the unprotected member.
pub fn protected_temperature_c(theta_0: f64, ap_over_v: f64, duration_s: f64, insulation: &Insulation) -> f64 {
    if insulation.thickness <= 0.0 { return unprotected_temperature_c(theta_0, ap_over_v, duration_s); }
    let (k_1, k_2) = (semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.fire-protected-denominator"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1999.fire-protected-exponent"));
    let dt = 5.0;
    let mut theta = theta_0.max(20.0);
    let mut t = 0.0;
    while t < duration_s - 1e-9 {
        let step = dt.min(duration_s - t);
        let (g_now, g_next) = (gas_temperature_c(t), gas_temperature_c(t + step));
        let c_a = specific_heat(theta);
        let phi = insulation.specific_heat * insulation.density / (c_a * DENSITY) * insulation.thickness * ap_over_v;
        let rise = insulation.conductivity / insulation.thickness * ap_over_v / (c_a * DENSITY) * (g_next - theta) / (1.0 + phi / k_1) * step - ((phi / k_2).exp() - 1.0) * (g_next - g_now);
        theta += if g_next > g_now { rise.max(0.0) } else { rise };
        t += step;
    }
    theta
}

/// 🌡️ Member temperature after `duration_s`: protected by `insulation` ([`protected_temperature_c`]) or unprotected ([`unprotected_temperature_c`]) without one or with a non-positive thickness.
pub fn member_temperature_c(theta_0: f64, am_over_v: f64, duration_s: f64, insulation: Option<&Insulation>) -> f64 {
    insulation.filter(|i| i.thickness > 0.0).map_or_else(|| unprotected_temperature_c(theta_0, am_over_v, duration_s), |i| protected_temperature_c(theta_0, am_over_v, duration_s, i))
}

/// 📏 Exposed perimeter [m] of the section on all sides.
pub fn exposed_perimeter(section: &AluminiumSection) -> f64 {
    let h = section.height; let b = section.width; let tw = section.web_thickness;
    match section.kind {
        semio_s_artifact_norm_en1999_model::SectionKind::Chs => std::f64::consts::PI * section.outer_diameter.max(section.width),
        semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs => 2.0 * (h + b),
        semio_s_artifact_norm_en1999_model::SectionKind::Angle => 2.0 * (h + b),
        semio_s_artifact_norm_en1999_model::SectionKind::Channel => 2.0 * h + 4.0 * b - 2.0 * tw,
        _ => 2.0 * h + 4.0 * b - 2.0 * tw,
    }
}

/// 🌡️ Section factor A_m/V [1/m].
pub fn section_factor(section: &AluminiumSection) -> f64 {
    let (a, _, _, _, _, _, _) = part_1_1::section_geometry(section);
    if a <= 0.0 { 0.0 } else { exposed_perimeter(section) / a }
}

/// 🧊️ The temperature field over the strips of a section after `duration_s` and what it does to the resistances: `thetas[i]` is the unprotected temperature of strip `i` of [`part_1_1::strips`] (section factor 2/t of the strip, both faces exposed), the ratios compare the strength retained with that field against the uniform member temperature (N: Σ k_θ,i·A_i, M_y and M_z: the same sum weighted with the strip distance to the centroid).
#[derive(Clone, Debug, PartialEq)]
pub struct FireField { pub thetas: Vec<f64>, pub n_ratio: f64, pub m_y_ratio: f64, pub m_z_ratio: f64, pub theta_min: f64, pub theta_max: f64 }

/// ⏱️ Unprotected temperature history of every strip of the section: `history[i][k]` is the temperature of strip `i` of [`part_1_1::strips`] (section factor 2/t) at `times_s[k]`; empty for a round tube.
pub fn strip_history(section: &AluminiumSection, f_o_pa: f64, class_a: bool, theta_0: f64, times_s: &[f64]) -> Vec<Vec<f64>> {
    if part_1_1::is_tube(section) { return Vec::new(); }
    part_1_1::strips(section, &part_1_1::EffCtx::gross(f_o_pa, class_a)).iter().map(|s| times_s.iter().map(|t| unprotected_temperature_c(theta_0, 2.0 / s.dy.min(s.dz).max(1.0e-6), *t)).collect()).collect()
}

/// 🧊️ The field of a section; `None` for a round tube (one wall, uniform) or a section without strips. `theta_uniform` is the member temperature of the lumped section factor.
pub fn temperature_field(section: &AluminiumSection, f_o_pa: f64, class_a: bool, theta_0: f64, duration_s: f64, theta_uniform: f64) -> Option<FireField> {
    if part_1_1::is_tube(section) { return None; }
    let ss = part_1_1::strips(section, &part_1_1::EffCtx::gross(f_o_pa, class_a));
    if ss.is_empty() { return None; }
    let thetas: Vec<f64> = ss.iter().map(|s| unprotected_temperature_c(theta_0, 2.0 / s.dy.min(s.dz).max(1.0e-6), duration_s)).collect();
    let areas: Vec<f64> = ss.iter().map(|s| s.dy * s.dz).collect();
    let total: f64 = areas.iter().sum();
    let (yg, zg) = (ss.iter().zip(&areas).map(|(s, a)| a * s.yc).sum::<f64>() / total, ss.iter().zip(&areas).map(|(s, a)| a * s.zc).sum::<f64>() / total);
    let k_u = k_theta(theta_uniform);
    let ratio = |weight: &dyn Fn(&part_1_1::Strip) -> f64| -> f64 {
        let den: f64 = ss.iter().zip(&areas).map(|(s, a)| a * weight(s)).sum();
        let num: f64 = ss.iter().zip(&thetas).zip(&areas).map(|((s, th), a)| k_theta(*th) * a * weight(s)).sum();
        if k_u < 1.0e-6 || den <= 0.0 { 1.0 } else { num / den / k_u }
    };
    let (n_ratio, m_y_ratio, m_z_ratio) = (ratio(&|_| 1.0), ratio(&|s| (s.zc - zg).abs()), ratio(&|s| (s.yc - yg).abs()));
    let (theta_min, theta_max) = thetas.iter().fold((f64::MAX, f64::MIN), |m, v| (m.0.min(*v), m.1.max(*v)));
    Some(FireField { thetas, n_ratio, m_y_ratio, m_z_ratio, theta_min, theta_max })
}
