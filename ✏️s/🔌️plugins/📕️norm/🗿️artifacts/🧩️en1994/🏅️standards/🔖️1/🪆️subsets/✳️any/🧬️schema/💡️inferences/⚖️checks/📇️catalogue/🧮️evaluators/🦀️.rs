//! 🧮️ Evaluators of the EN 1994 design-case catalogue: each entry calls the existing pure catalogue formulas (SI: m, Pa, N, N·m) and returns demand and capacity rows. See [`super`] and [EN 1994](https://eurocodes.jrc.ec.europa.eu/).

use crate::catalogue as formulas;
use formulas::{ISection, InterfaceKind};
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

/// 🔢️ The typed numbers and texts of one design case.
pub struct Inputs<'a>(pub &'a DesignCase);

impl Inputs<'_> {
    pub fn n(&self, key: &str) -> Result<f64, String> {
        self.0.values.numbers.get(key).copied().ok_or_else(|| format!("missing input {key}"))
    }

    pub fn t(&self, key: &str) -> Result<&str, String> {
        self.0.values.texts.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
    }
}

/// 🔩 EN 1994-1-1 §6.6.4.2: stud design resistance P_Rd reduced by the sheeting factor k_t or k_l against the force per stud.
pub fn stud_sheeting(q: &Inputs<'_>) -> Outcome {
    let (d, h_sc, fck) = (q.n("d_stud")?, q.n("h_sc")?, q.n("fck")?);
    let (_, _, p_rd) = formulas::stud_resistance_n(d, h_sc, q.n("fu_stud")?, fck, formulas::ecm_pa(fck), q.n("gamma_v")?);
    let parallel = q.t("ribs")? == "parallel";
    let k = formulas::sheeting_reduction(q.n("h_p")?, q.n("b_0")?, h_sc, d, q.n("t_sheet")?, q.n("n_r")?.round() as u32, parallel).ok_or("sheeting reduction applies up to two studs per rib")?;
    let p_ed = q.n("p_ed")?;
    Ok(vec![Row::new("stud", "PEd ≤ k PRd", "PEd ≤ k PRd", QuantityKind::Force, p_ed, k * p_rd, format!("PRd = {p_rd:.0} N; k = {k:.3}; k PRd = {:.0} N", k * p_rd))])
}

/// 🧷 EN 1993-1-5 §6 and §7.2 as used by EN 1994-1-1: web patch loading resistance and the interaction 0.8 FEd/FRd + MEd/MRd ≤ 1.4.
pub fn patch_loading(q: &Inputs<'_>) -> Outcome {
    let (f_rd, chi, l_y, lambda) = formulas::patch_loading(q.n("h_w")?, q.n("t_w")?, q.n("b_f")?, q.n("t_f")?, q.n("fyw")?, q.n("fyf")?, q.n("s_s")?, q.n("a_panel")?, q.n("gamma_m1")?);
    let f_ed = q.n("f_ed")?;
    let u = formulas::patch_interaction(f_ed, f_rd, q.n("m_ed")?, q.n("m_rd")?);
    Ok(vec![
        Row::new("patch", "FEd ≤ FRd", "FEd ≤ FRd", QuantityKind::Force, f_ed, f_rd, format!("FRd = {f_rd:.0} N; χF = {chi:.3}; ℓy = {l_y:.3} m; λF = {lambda:.3}")),
        Row::ratio("interaction", "0.8 FEd/FRd + MEd/MRd ≤ 1.4", "0,8 FEd/FRd + MEd/MRd ≤ 1,4", u, format!("utilisation against 1.4: {u:.3}")),
    ])
}

fn interface_kind(value: &str) -> Result<InterfaceKind, String> {
    match value {
        "concrete_encased" => Ok(InterfaceKind::ConcreteEncased),
        "filled_circular" => Ok(InterfaceKind::FilledCircular),
        "filled_rectangular" => Ok(InterfaceKind::FilledRectangular),
        "partially_encased_flange" => Ok(InterfaceKind::PartiallyEncasedFlange),
        "partially_encased_web" => Ok(InterfaceKind::PartiallyEncasedWeb),
        other => Err(format!("unknown interface {other}")),
    }
}

/// 🔗 EN 1994-1-1 §6.7.4: interface shear τEd = Nintro/(u lv) over the introduction length lv = min(2 d, L/3) against τRd (Table 6.6).
pub fn load_introduction(q: &Inputs<'_>) -> Outcome {
    let kind = interface_kind(q.t("kind")?)?;
    let length = formulas::introduction_length_m(q.n("d_min")?, q.n("l_col")?);
    let n = q.n("n_intro")?;
    let (tau_ed, tau_rd, u) = formulas::load_introduction(kind, n, q.n("perimeter")?, length);
    let studs = formulas::introduction_studs(n, q.n("p_rd")?);
    Ok(vec![Row::ratio("interface", "τEd ≤ τRd", "τEd ≤ τRd", u, format!("lv = {length:.3} m; τEd = {tau_ed:.0} Pa; τRd = {tau_rd:.0} Pa; studs for the full force: {studs}"))])
}

fn deflection_row(q: &Inputs<'_>, delta: f64) -> Outcome {
    let limit = q.n("l_span")? / q.n("span_ratio")?;
    Ok(vec![Row::new("deflection", "δ ≤ L/limit", "δ ≤ L/Grenzwert", QuantityKind::Length, delta, limit, format!("δ = {:.2} mm; L/limit = {:.2} mm", delta * 1e3, limit * 1e3))])
}

/// 📉 EN 1994-1-1 §7.3.1: mid-span deflection of a simply supported beam with long-term and short-term stiffness against L/limit.
pub fn deflection_simple_span(q: &Inputs<'_>) -> Outcome {
    deflection_row(q, formulas::deflection_simple(q.n("l_span")?, q.n("g_line")?, q.n("q_line")?, q.n("i_long")?, q.n("i_short")?))
}

/// 📊️ EN 1994-1-1 §7.3.1: maximum deflection of a two-span continuous beam against L/limit.
pub fn deflection_two_span(q: &Inputs<'_>) -> Outcome {
    deflection_row(q, formulas::deflection_two_span(q.n("l_span")?, q.n("g_line")?, q.n("q_line")?, q.n("i_long")?, q.n("i_short")?))
}

/// 🔥 EN 1994-1-2 §3.2: steel stress σEd against the reduced yield strength ky,θ fy at the steel temperature θa.
pub fn steel_at_temperature(q: &Inputs<'_>) -> Outcome {
    let k = formulas::ky_theta(q.n("theta_a")?);
    let f = k * q.n("fy")?;
    let sigma = q.n("sigma_ed")?;
    Ok(vec![Row::new("strength", "σEd ≤ ky,θ fy", "σEd ≤ ky,θ fy", QuantityKind::Stress, sigma, f, format!("ky,θ = {k:.3}; ky,θ fy = {f:.0} Pa"))])
}

fn section(q: &Inputs<'_>) -> Result<ISection, String> {
    Ok(ISection { h: q.n("sec_h")?, b: q.n("sec_b")?, tf: q.n("sec_tf")?, tw: q.n("sec_tw")?, a: q.n("sec_a")?, i_y: q.n("sec_iy")? })
}

/// 📐 EN 1994-1-1 §6.2.1.2: sagging plastic moment resistance Mpl,Rd by the stress block of the concrete slab (0.85 fck/γC) and the steel section.
pub fn sagging_plastic(q: &Inputs<'_>) -> Outcome {
    let fcc = formulas::fcc_beam_pa(q.n("fck")?, q.n("gamma_c")?);
    let p = formulas::plastic_full(&section(q)?, q.n("fyd")?, q.n("ys")?, q.n("b_eff")?, q.n("h_c")?, fcc);
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("moment", "MEd ≤ Mpl,Rd", "MEd ≤ Mpl,Rd", QuantityKind::Moment, m_ed, p.m_nm, format!("Mpl,Rd = {:.0} N·m; Nc = {:.0} N; x = {:.4} m; plastic axis {:.4} m ({})", p.m_nm, p.nc_n, p.x_m, p.y_na_m, p.zone))])
}

/// 🏔️ EN 1994-1-1 §6.2.1.2(2): hogging plastic moment resistance with the slab reinforcement and the steel section, concrete neglected.
pub fn hogging_plastic(q: &Inputs<'_>) -> Outcome {
    let (m_rd, y) = formulas::plastic_hogging(&section(q)?, q.n("fyd")?, q.n("ys")?, q.n("as_rebar")?, q.n("fsd")?, q.n("y_r")?);
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("moment", "|MEd| ≤ Mpl,Rd", "|MEd| ≤ Mpl,Rd", QuantityKind::Moment, m_ed, m_rd.abs(), format!("Mpl,Rd = {:.0} N·m; plastic axis {y:.4} m below the slab top", m_rd.abs()))])
}

/// 🌊️ EN 1990 A1.4.4 and EN 1994-1-1 §7.3: lowest natural frequency f1 = π/(2 L²)·√(Ea I(n0)/m) of the simply supported composite beam against the required fmin.
pub fn floor_vibration(q: &Inputs<'_>) -> Outcome {
    let (span, i_short) = (q.n("l_span")?, q.n("i_short")?);
    let mass = formulas::vibration::mass_per_length_kg_m(q.n("g_line")?, q.n("q_line")?, q.n("psi_vib")?);
    if mass <= 0.0 {
        return Err("the vibrating mass is zero".into());
    }
    let f1 = formulas::vibration::natural_frequency_hz(span, i_short, mass);
    let f_min = q.n("f_min")?;
    let u = formulas::vibration::frequency_utilisation(f_min, f1);
    Ok(vec![Row::ratio("frequency", "fmin ≤ f1", "fmin ≤ f1", u, format!("m = {mass:.1} kg/m; f1 = {f1:.3} Hz; fmin = {f_min:.3} Hz"))])
}

fn welded(q: &Inputs<'_>) -> Result<bool, String> {
    match q.t("steel_kind")? {
        "rolled" => Ok(false),
        "welded" => Ok(true),
        other => Err(format!("unknown steel kind {other}")),
    }
}

/// 🦋️ EN 1994-1-1 §6.4: lateral-torsional buckling of the steel bottom flange of a continuous composite beam over the support, inverted U-frame stiffness ks (6.4.2) and χLT (EN 1993-1-1 6.3.2.2) against the hogging moment.
pub fn ltb_continuous(q: &Inputs<'_>) -> Outcome {
    let sec = section(q)?;
    let (fy, gamma) = (q.n("fy")?, q.n("gamma_m1")?);
    let length = q.n("l_ltb")?;
    let ks = formulas::uframe_ks(q.n("i2_slab")?, q.n("a_beam")?, &sec, q.n("k_alpha")?);
    let m_cr = formulas::mcr_uframe(&sec, length, ks);
    let m_pl_k = sec.w_pl() * fy;
    let (lambda, chi) = formulas::chi_lt(m_pl_k, m_cr, sec.h / sec.b, welded(q)?);
    let m_b_rd = chi * m_pl_k / gamma;
    let m_ed = q.n("m_ed")?.abs();
    Ok(vec![Row::new("ltb", "|MEd| ≤ Mb,Rd", "|MEd| ≤ Mb,Rd", QuantityKind::Moment, m_ed, m_b_rd, format!("ks = {ks:.0} N·m/m/rad; Mcr = {m_cr:.0} N·m; λLT = {lambda:.3}; χLT = {chi:.3}; Mb,Rd = {m_b_rd:.0} N·m"))])
}

/// ⏳️ EN 1994-1-1 §5.4.2.2 and §7.3.1: deflection of a simply supported composite beam with the modular ratios n0 (short term), nL (permanent loads, ψL = 1.1) and ns (shrinkage, ψL = 0.55) plus the shrinkage curvature against L/limit.
pub fn deflection_creep_shrinkage(q: &Inputs<'_>) -> Outcome {
    let sec = section(q)?;
    let (ys, beff, hc, span) = (q.n("ys")?, q.n("b_eff")?, q.n("h_c")?, q.n("l_span")?);
    let ecm = formulas::ecm_pa(q.n("fck")?);
    let phi = q.n("phi_t")?;
    let n0 = formulas::modular_ratio(ecm, 0.0, 0.0);
    let n_l = formulas::modular_ratio(ecm, q.n("psi_l")?, phi);
    let n_s = formulas::modular_ratio(ecm, formulas::constants::PSI_L_SHRINKAGE, phi);
    let (i0, i_l, el_s) = (formulas::transformed(&sec, ys, beff, hc, n0).i, formulas::transformed(&sec, ys, beff, hc, n_l).i, formulas::transformed(&sec, ys, beff, hc, n_s));
    let load = formulas::deflection_simple(span, q.n("g_line")?, q.n("q_line")?, i_l, i0);
    let shrink = formulas::deflection_shrinkage(span, q.n("eps_cs")?, beff, hc, n_s, (el_s.y_na - hc / 2.0).max(0.0), el_s.i);
    let limit = span / q.n("span_ratio")?;
    Ok(vec![Row::new("deflection", "δ(G, Q) + δ(εcs) ≤ L/limit", "δ(G, Q) + δ(εcs) ≤ L/Grenzwert", QuantityKind::Length, load + shrink, limit, format!("n0 = {n0:.2}; nL = {n_l:.2}; ns = {n_s:.2}; I(n0) = {i0:.6} m⁴; I(nL) = {i_l:.6} m⁴; δ(G,Q) = {:.2} mm; δ(εcs) = {:.2} mm; L/limit = {:.2} mm", load * 1e3, shrink * 1e3, limit * 1e3))])
}

/// 🔥 EN 1993-1-2 §4.2.5.2 (Eq. 4.27) and §4.2.4 (Eq. 4.22): temperature θa(t) of the protected steel section against the critical temperature θa,cr of the degree of utilisation μ0.
pub fn protected_steel_fire(q: &Inputs<'_>) -> Outcome {
    let t = q.n("t_fire")?;
    let theta = formulas::protected_steel_temperature_c(t, q.n("am_v")?, q.n("d_p")?, q.n("lambda_p")?, q.n("c_p")?, q.n("rho_p")?);
    let mu0 = q.n("mu0")?;
    let theta_cr = formulas::critical_temperature_c(mu0);
    Ok(vec![Row::new("temperature", "θa(t) ≤ θa,cr", "θa(t) ≤ θa,cr", QuantityKind::Temperature, theta, theta_cr, format!("θa({t:.0} s) = {theta:.1} °C; θa,cr(μ0 = {mu0:.3}) = {theta_cr:.1} °C; ISO 834 gas {:.1} °C", formulas::iso834_c(t)))])
}
