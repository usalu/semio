//! 🧮️ Evaluators of the EN 1998 design-case catalogue: each entry calls the existing pure kernel functions (SI: kg, m, s, N, Pa) and returns demand and capacity rows. See [`super`] and [EN 1998](https://eurocodes.jrc.ec.europa.eu/).

use semio_s_artifact_norm_en1998_schema::horizontal::diaphragm;
use semio_s_artifact_norm_en1998_schema::{part_1, part_5};
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

fn base_shear_row(key: &'static str, en: &'static str, de: &'static str, force: f64, q: &Inputs<'_>) -> Outcome {
    let v_rd = q.n("v_rd")?;
    Ok(vec![Row::new(key, en, de, QuantityKind::Force, force, v_rd, format!("Fb = {force:.0} N; VRd = {v_rd:.0} N"))])
}

/// 📉 EN 1998-1 §3.2.2.5 and §4.3.3.2.2: base shear Fb = Sd(T1) m λ against the base shear resistance.
pub fn base_shear_design(q: &Inputs<'_>) -> Outcome {
    let sd = part_1::design_spectrum_horizontal(q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("q_factor")?, q.n("t_1")?);
    base_shear_row("base-shear", "Fb = Sd(T1) m λ ≤ VRd", "Fb = Sd(T1) m λ ≤ VRd", sd * q.n("mass")? * q.n("lambda_c")?, q)
}

/// 📈 EN 1998-1 §3.2.2.2: elastic response force η Se(T1) m against the base shear resistance.
pub fn elastic_force(q: &Inputs<'_>) -> Outcome {
    let eta = part_1::damping_correction(q.n("xi_percent")?);
    let se = part_1::elastic_response_spectrum(q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("t_1")?);
    base_shear_row("elastic", "Fel = η Se(T1) m ≤ VRd", "Fel = η Se(T1) m ≤ VRd", eta * se * q.n("mass")?, q)
}

/// ↕️ EN 1998-1 §3.2.2.3: vertical seismic force Fv = Sve(Tv) m against the vertical resistance.
pub fn vertical_force(q: &Inputs<'_>) -> Outcome {
    let sve = part_1::vertical_elastic_spectrum(q.n("a_g")?, q.n("avg_ratio")?, q.n("t_v")?);
    let force = sve * q.n("mass")?;
    let f_rd = q.n("f_rd_v")?;
    Ok(vec![Row::new("vertical", "Fv = Sve(Tv) m ≤ FRd,v", "Fv = Sve(Tv) m ≤ FRd,v", QuantityKind::Force, force, f_rd, format!("Sve = {sve:.3} m/s²; Fv = {force:.0} N; FRd,v = {f_rd:.0} N"))])
}

/// 🎼 EN 1998-1 §4.3.3.1(3) and §4.3.3.3: effective modal masses of a three-storey shear building (at least 90 % of the mass) and the base shear by SRSS, or by CQC when two periods are closer than 10 %.
pub fn modal_analysis(q: &Inputs<'_>) -> Outcome {
    let masses = [q.n("m_1")?, q.n("m_2")?, q.n("m_3")?];
    let stiffness = [q.n("k_1")?, q.n("k_2")?, q.n("k_3")?];
    let (periods, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let effective = part_1::effective_modal_masses(&masses, &shapes);
    let (total, covered) = (masses.iter().sum::<f64>(), effective.iter().sum::<f64>());
    let (a_g, s_soil, t_b, t_c, t_d, q_factor) = (q.n("a_g")?, q.n("s_soil")?, q.n("t_b")?, q.n("t_c")?, q.n("t_d")?, q.n("q_factor")?);
    let modal: Vec<f64> = periods.iter().zip(&effective).map(|(t, m)| m * part_1::design_spectrum_horizontal(a_g, s_soil, t_b, t_c, t_d, q_factor, *t)).collect();
    let separated = periods.iter().enumerate().all(|(i, a)| periods.iter().skip(i + 1).all(|b| (*a).min(*b) <= 0.9 * (*a).max(*b)));
    let (method, v_b) = if separated { ("SRSS", part_1::srss_combine(&modal)) } else { ("CQC", part_1::cqc_combine(&modal, &periods, q.n("xi_percent")? / 100.0)) };
    let v_rd = q.n("v_rd")?;
    Ok(vec![
        Row::new("modal-mass", "Σ Meff,n ≥ 0.9 m", "Σ Meff,n ≥ 0,9 m", QuantityKind::Mass, 0.9 * total, covered, format!("T = {periods:.3?} s; Meff = {effective:.0?} kg; Σ Meff = {covered:.0} kg of {total:.0} kg")),
        Row::new("modal-shear", "Vb ≤ VRd (modal combination)", "Vb ≤ VRd (modale Überlagerung)", QuantityKind::Force, v_b, v_rd, format!("{method} combination of {modal:.0?} N gives Vb = {v_b:.0} N; VRd = {v_rd:.0} N")),
    ])
}

/// 🌍️ EN 1998-1 §3.1.2 Eq. 3.1: vs,30 = 30/Σ(hi/vi) of three layers against the lower bound of the assumed ground type.
pub fn ground_velocity(q: &Inputs<'_>) -> Outcome {
    let layers = [(q.n("h_1")?, q.n("vs_1")?), (q.n("h_2")?, q.n("vs_2")?), (q.n("h_3")?, q.n("vs_3")?)];
    let vs30 = part_1::vs30_from_layers(&layers);
    let class = part_1::ground_class_from_vs30(vs30).map_or("-".to_string(), |c| c.to_string());
    let v_min = q.n("vs_min")?;
    Ok(vec![Row::new("vs30", "vs,30 ≥ lower bound", "vs,30 ≥ Untergrenze", QuantityKind::Dimensionless, v_min, vs30, format!("vs,30 = {vs30:.1} m/s (ground type {class}); lower bound {v_min:.0} m/s"))])
}

/// 🌊️ EN 1998-5 §4.1.4: factor of safety against liquefaction FL = CRR7.5 MSF/CSR against the required value.
pub fn liquefaction(q: &Inputs<'_>) -> Outcome {
    let f_l = part_5::liquefaction_safety_factor(q.n("n1_60")?, q.n("fines_percent")?, q.n("a_max")?, q.n("sigma_v")?, q.n("sigma_v_eff")?, q.n("z_depth")?, q.n("magnitude")?);
    let required = q.n("f_l_min")?;
    Ok(vec![Row::new("liquefaction", "FL ≥ FL,min", "FL ≥ FL,min", QuantityKind::Dimensionless, required, f_l, format!("FL = {f_l:.3}; required {required:.2}"))])
}

/// 🏗️ EN 1998-5 §5.4.2: bending moment at depth z of a long pile under the inertial head shear against the moment resistance.
pub fn pile_inertial(q: &Inputs<'_>) -> Outcome {
    let fixed = q.t("head_fixity")? == "fixed";
    let shear = q.n("head_shear")?;
    let response = part_5::pile_inertial(q.n("e_pile")?, q.n("d_pile")?, q.n("l_pile")?, q.n("k_h")?, shear, fixed);
    let moment = part_5::pile_inertial_moment_at(&response, shear, fixed, q.n("z_check")?).abs();
    let m_rd = q.n("m_rd")?;
    Ok(vec![Row::new("pile-moment", "|M(z)| ≤ MRd", "|M(z)| ≤ MRd", QuantityKind::Moment, moment, m_rd, format!("β = {:.3} 1/m; |M(z)| = {moment:.0} N·m; maximum {:.0} N·m; MRd = {m_rd:.0} N·m", response.beta, response.max_moment_nm))])
}

/// 🔗 EN 1998-1 §4.4.2.5: chord force N(x) = Fd x (L − x)/(2 L D) of a diaphragm under the storey force against the chord resistance A fd.
pub fn diaphragm_chord(q: &Inputs<'_>) -> Outcome {
    let f_d = diaphragm::design_force_n(q.n("f_storey")?, q.n("gamma_d")?);
    let n_ed = diaphragm::chord_force_at_n(f_d, q.n("span_dia")?, q.n("depth_dia")?, q.n("x_pos")?);
    let n_rd = diaphragm::chord_resistance_n(q.n("chord_area")?, q.n("chord_fd")?);
    Ok(vec![Row::new("chord", "NEd(x) ≤ NRd", "NEd(x) ≤ NRd", QuantityKind::Force, n_ed, n_rd, format!("Fd = {f_d:.0} N; NEd(x) = {n_ed:.0} N; NRd = {n_rd:.0} N"))])
}
