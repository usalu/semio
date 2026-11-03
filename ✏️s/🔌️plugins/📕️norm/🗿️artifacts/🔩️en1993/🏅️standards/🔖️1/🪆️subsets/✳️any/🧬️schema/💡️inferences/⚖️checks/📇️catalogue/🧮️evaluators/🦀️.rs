//! 🧮️ Evaluators of the EN 1993 design-case catalogue: each entry calls the existing pure clause functions (mm, N, MPa) and returns demand and capacity rows. See [`super`] and [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

use crate::clauses::{fatigue_fire, misc, parts_2_6, resistance};
use semio_s_plugin_norm_compliance::design_case::DesignCase;
use semio_s_plugin_norm_compliance::QuantityKind;

/// 📏️ One verification row: what is demanded, what is available and the worked steps.
pub struct Row {
    pub key: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub kind: QuantityKind,
    pub demand: f64,
    pub capacity: f64,
    pub steps: String,
}

impl Row {
    pub fn new(key: &'static str, en: &'static str, de: &'static str, kind: QuantityKind, demand: f64, capacity: f64, steps: String) -> Self {
        Self { key, en, de, kind, demand, capacity, steps }
    }

    pub fn ratio(key: &'static str, en: &'static str, de: &'static str, utilisation: f64, steps: String) -> Self {
        Self::new(key, en, de, QuantityKind::Dimensionless, utilisation, 1.0, steps)
    }
}

/// 🧾 Rows of one evaluation, or the reason an input is missing.
pub type Outcome = Result<Vec<Row>, String>;

/// 🔢️ The typed numbers of one design case.
pub struct Inputs<'a>(pub &'a DesignCase);

impl Inputs<'_> {
    pub fn n(&self, key: &str) -> Result<f64, String> {
        self.0.values.numbers.get(key).copied().ok_or_else(|| format!("missing input {key}"))
    }
}

/// 🟦️ EN 1993-1-7: stress σ = k p b²/t² of a plate under transverse pressure against fy/γM0.
pub fn plate_lateral(q: &Inputs<'_>) -> Outcome {
    let sigma = misc::plate_lateral_pressure_stress(q.n("k_plate")?, q.n("p_plate")?, q.n("b_plate")?, q.n("t_plate")?);
    let limit = q.n("fy")? / q.n("gm0")?;
    Ok(vec![Row::new("sigma", "plate stress σ ≤ fy/γM0", "Plattenspannung σ ≤ fy/γM0", QuantityKind::Stress, sigma, limit, format!("σ = k p b²/t² = {sigma:.2} N/mm²; fy/γM0 = {limit:.2} N/mm²"))])
}

/// 🌉️ EN 1993-2 §9.5: λ = λ1 λ2 λ3 λ4 ≤ λmax, Δσ_E,2 = λ Φ2 Δσp and γFf Δσ_E,2 ≤ Δσc/γMf.
pub fn bridge_fatigue(q: &Inputs<'_>) -> Outcome {
    let lambda = parts_2_6::lambda_equivalence(q.n("lambda_1")?, q.n("lambda_2")?, q.n("lambda_3")?, q.n("lambda_4")?, q.n("lambda_max")?);
    let delta_e2 = fatigue_fire::delta_sigma_e2(lambda, q.n("phi_2")?, q.n("delta_sigma_p")?);
    let u = parts_2_6::bridge_fatigue_utilisation(delta_e2, q.n("delta_sigma_c")?, q.n("gamma_ff")?, q.n("gamma_mf")?);
    Ok(vec![Row::ratio("fatigue", "γFf Δσ_E,2 / (Δσc/γMf) ≤ 1", "γFf Δσ_E,2 / (Δσc/γMf) ≤ 1", u, format!("λ = {lambda:.3}; Δσ_E,2 = {delta_e2:.2} N/mm²; utilisation {u:.3}"))])
}

/// 🔁 EN 1993-1-9 §8: combined normal and shear stress ranges, (γFf Δσ/(Δσc/γMf))³ + (γFf Δτ/(Δτc/γMf))⁵ ≤ 1.
pub fn fatigue_combined(q: &Inputs<'_>) -> Outcome {
    let u = fatigue_fire::combined_normal_shear(q.n("delta_sigma")?, q.n("delta_sigma_c")?, q.n("delta_tau")?, q.n("delta_tau_c")?, q.n("gamma_ff")?, q.n("gamma_mf")?);
    Ok(vec![Row::ratio("interaction", "(γFf Δσ/(Δσc/γMf))³ + (γFf Δτ/(Δτc/γMf))⁵ ≤ 1", "(γFf Δσ/(Δσc/γMf))³ + (γFf Δτ/(Δτc/γMf))⁵ ≤ 1", u, format!("interaction sum {u:.3}"))])
}

/// 🪵 EN 1993-5 §5.2.2: sheet pile bending resistance βB Wel fy/γM0 against MEd.
pub fn sheet_pile_bending(q: &Inputs<'_>) -> Outcome {
    let m_rd = parts_2_6::sheet_pile_bending_rd(q.n("beta_b")?, q.n("w_el")?, q.n("fy")?, q.n("gm0")?);
    let m_ed = q.n("m_ed")?;
    Ok(vec![Row::new("bending", "MEd ≤ MRd = βB Wel fy/γM0", "MEd ≤ MRd = βB Wel fy/γM0", QuantityKind::Moment, m_ed, m_rd, format!("MRd = {m_rd:.0} N·mm; MEd = {m_ed:.0} N·mm"))])
}

/// 🪢 EN 1993-1-11 §6.2: cable resistance FRd = min(Fuk/γR, Fk) against FEd.
pub fn cable_tension(q: &Inputs<'_>) -> Outcome {
    let f_rd = parts_2_6::cable_resistance(q.n("f_uk")?, q.n("f_k")?, q.n("gamma_r")?);
    let f_ed = q.n("f_ed")?;
    Ok(vec![Row::new("tension", "FEd ≤ FRd = min(Fuk/γR, Fk)", "FEd ≤ FRd = min(Fuk/γR, Fk)", QuantityKind::Force, f_ed, f_rd, format!("FRd = {f_rd:.0} N; FEd = {f_ed:.0} N"))])
}

/// 🗼 EN 1993-3-1 §5: wind-amplified shaft compression NEd κ/(χ A fy/γM1) ≤ 1.
pub fn tower_leg(q: &Inputs<'_>) -> Outcome {
    let u = parts_2_6::tower_leg_utilisation(q.n("n_ed")?, q.n("amplification")?, q.n("chi")?, q.n("area")?, q.n("fy")?, q.n("gm1")?);
    Ok(vec![Row::ratio("compression", "NEd κ/(χ A fy/γM1) ≤ 1", "NEd κ/(χ A fy/γM1) ≤ 1", u, format!("utilisation {u:.3}"))])
}

/// 🏗️ EN 1993-1-8 §6.2.8: base plate T-stub compression resistance FC,Rd = fjd beff leff with c = t √(fy/(3 fjd γM0)).
pub fn base_plate(q: &Inputs<'_>) -> Outcome {
    let fjd = parts_2_6::base_plate_fjd(q.n("beta_j")?, q.n("k_j")?, q.n("f_cd")?);
    let c = parts_2_6::base_plate_c(q.n("t_plate")?, q.n("fy")?, fjd, q.n("gm0")?);
    let (b_eff, l_eff) = (q.n("t_flange")? + 2.0 * c, q.n("b_flange")? + 2.0 * c);
    let f_rd = parts_2_6::base_plate_fc_rd(fjd, b_eff, l_eff);
    let f_ed = q.n("f_ed")?;
    Ok(vec![Row::new("bearing", "FEd ≤ FC,Rd = fjd beff leff", "FEd ≤ FC,Rd = fjd beff leff", QuantityKind::Force, f_ed, f_rd, format!("fjd = {fjd:.2} N/mm²; c = {c:.1} mm; beff = {b_eff:.1} mm; leff = {l_eff:.1} mm; FC,Rd = {f_rd:.0} N"))])
}

/// 🚜️ EN 1993-6 §5.7.1: local vertical web stress σoz,Ed = Fz,Ed/(leff tw) with leff = 3.25 (Irf/tw)^(1/3) against fy/γM0.
pub fn crane_web(q: &Inputs<'_>) -> Outcome {
    let l_eff = parts_2_6::crane_l_eff(q.n("i_rf")?, q.n("t_web")?);
    let sigma = parts_2_6::crane_sigma_oz(q.n("fz_ed")?, l_eff, q.n("t_web")?);
    let limit = q.n("fy")? / q.n("gm0")?;
    Ok(vec![Row::new("sigma-oz", "σoz,Ed ≤ fy/γM0", "σoz,Ed ≤ fy/γM0", QuantityKind::Stress, sigma, limit, format!("leff = {l_eff:.1} mm; σoz,Ed = {sigma:.2} N/mm²; fy/γM0 = {limit:.2} N/mm²"))])
}

/// 🧪️ EN 1993-1-1 §6.2.1(5): von Mises yield criterion (σx/f)² + (σz/f)² − (σx/f)(σz/f) + 3 (τ/f)² ≤ 1.
pub fn von_mises(q: &Inputs<'_>) -> Outcome {
    let u = resistance::von_mises_utilisation(q.n("sigma_x")?, q.n("sigma_z")?, q.n("tau")?, q.n("fy")?, q.n("gm0")?);
    Ok(vec![Row::ratio("yield", "von Mises criterion ≤ 1", "Fließbedingung nach von Mises ≤ 1", u, format!("utilisation {u:.3}"))])
}

/// 📐️ EN 1993-1-1 §6.2.9.2: class 3 maximum fibre stress |NEd/A + My,Ed/Wel,y + Mz,Ed/Wel,z| ≤ fy/γM0.
pub fn class3_stress(q: &Inputs<'_>) -> Outcome {
    let u = resistance::class3_utilisation(q.n("n_ed")?, q.n("area")?, q.n("my_ed")?, q.n("w_el_y")?, q.n("mz_ed")?, q.n("w_el_z")?, q.n("fy")?, q.n("gm0")?);
    Ok(vec![Row::ratio("fibre-stress", "σx,Ed ≤ fy/γM0", "σx,Ed ≤ fy/γM0", u, format!("utilisation {u:.3}"))])
}

/// 🧱️ EN 1993-1-1 §6.2.9.1: I or H section under axial force and biaxial bending, (My,Ed/MN,y,Rd)^α + (Mz,Ed/MN,z,Rd)^β ≤ 1 (Eq. 6.41).
pub fn biaxial_bending(q: &Inputs<'_>) -> Outcome {
    let (fy, gm0, a_web) = (q.n("fy")?, q.n("gm0")?, q.n("a_web")?);
    let n = (q.n("n_ed")? / resistance::n_pl_rd(q.n("area")?, fy, gm0)).abs();
    let m_n_y = resistance::m_n_y_rd_i(q.n("w_pl_y")? * fy / gm0, n, a_web);
    let m_n_z = resistance::m_n_z_rd_i(q.n("w_pl_z")? * fy / gm0, n, a_web);
    let (alpha, beta) = resistance::biaxial_exponents(0, n);
    let u = resistance::biaxial_utilisation(q.n("my_ed")?, q.n("mz_ed")?, m_n_y, m_n_z, alpha, beta);
    Ok(vec![Row::ratio("interaction", "(My,Ed/MN,y,Rd)^α + (Mz,Ed/MN,z,Rd)^β ≤ 1", "(My,Ed/MN,y,Rd)^α + (Mz,Ed/MN,z,Rd)^β ≤ 1", u, format!("n = {n:.3}; MN,y,Rd = {m_n_y:.0} N·mm; MN,z,Rd = {m_n_z:.0} N·mm; α = {alpha:.2}; β = {beta:.2}"))])
}

/// 🛡️ EN 1993-1-5 §7.1: interaction of bending and shear, η̄1 + (1 − Mf,Rd/Mpl,Rd)(2 η̄3 − 1)² ≤ 1.
pub fn bending_shear_interaction(q: &Inputs<'_>) -> Outcome {
    let u = resistance::bending_shear_buckling_utilisation(q.n("m_ed")?, q.n("m_pl_rd")?, q.n("m_f_rd")?, q.n("v_ed")?, q.n("v_bw_rd")?);
    Ok(vec![Row::ratio("interaction", "η̄1 + (1 − Mf,Rd/Mpl,Rd)(2 η̄3 − 1)² ≤ 1", "η̄1 + (1 − Mf,Rd/Mpl,Rd)(2 η̄3 − 1)² ≤ 1", u, format!("utilisation {u:.3}"))])
}

/// 🔥️ EN 1993-1-2 §4.2.4 and §4.2.5: critical temperature θcr from μ0 against the unprotected steel temperature at the end of the ISO 834 exposure.
pub fn critical_temperature(q: &Inputs<'_>) -> Outcome {
    let mu0 = fatigue_fire::mu0(q.n("e_fi_d")?, q.n("r_fi_d_0")?);
    let theta_cr = fatigue_fire::theta_critical(mu0);
    let history = fatigue_fire::unprotected_steel_temperature(&fatigue_fire::gas_iso834, q.n("k_sh")?, q.n("am_over_v")?, q.n("t_fire")? * 60.0, 5.0, q.n("eps_m")?, q.n("eps_f")?);
    let theta = history.last().map_or(20.0, |(_, theta)| *theta);
    Ok(vec![Row::new("temperature", "θa ≤ θcr", "θa ≤ θcr", QuantityKind::Temperature, theta, theta_cr, format!("μ0 = {mu0:.3}; θcr = {theta_cr:.0} °C; θa = {theta:.0} °C"))])
}

/// 🛢️ EN 1993-4-3: required wall thickness t = p d/(2 fy/γM0) of a pipe under internal pressure against the provided thickness.
pub fn pipe_hoop(q: &Inputs<'_>) -> Outcome {
    let t_req = misc::hoop_thickness_barlow(q.n("p_int")?, q.n("d_pipe")?, q.n("fy")?, q.n("gm0")?);
    let t = q.n("t_pipe")?;
    Ok(vec![Row::new("thickness", "treq ≤ t", "treq ≤ t", QuantityKind::Length, t_req, t, format!("treq = {t_req:.2} mm; t = {t:.2} mm"))])
}

/// 🥫️ EN 1993-4-3: Spangler ovalisation Δd/d of a buried pipe against the permissible ratio.
pub fn pipe_ovalisation(q: &Inputs<'_>) -> Outcome {
    let d = q.n("d_pipe")?;
    let delta = misc::spangler_ovalisation(q.n("k_b")?, q.n("w_line")?, d, q.n("e_pipe")?, q.n("i_wall")?, q.n("e_soil")?);
    let limit = q.n("ovalisation_limit")?;
    Ok(vec![Row::new("ovalisation", "Δd/d ≤ limit", "Δd/d ≤ Grenzwert", QuantityKind::Dimensionless, delta / d, limit, format!("Δd = {delta:.2} mm; Δd/d = {:.4}", delta / d))])
}
