//! 🧮️ Evaluators of the EN 1993 design-case catalogue: each entry calls the existing pure clause functions (mm, N, MPa) and returns demand and capacity rows. See [`super`] and [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

use crate::clauses::{fatigue_fire, misc, parts_2_6, resistance};
use semio_s_plugin_norm_compliance::design_case::DesignCase;
use semio_s_plugin_norm_compliance::QuantityKind;

#[path = "🧱️lattice-node/🦀️.rs"]
mod lattice_node;

pub use lattice_node::lattice_node_fatigue;

#[path = "🛞️hollow-section/🦀️.rs"]
mod hollow_section;

pub use hollow_section::hollow_section_fatigue;

#[path = "🪢️load-carrying-weld/🦀️.rs"]
mod load_carrying_weld;
pub use load_carrying_weld::load_carrying_weld_fatigue;

#[path = "🧱️attachment-fatigue/🦀️.rs"]
mod attachment_fatigue;
pub use attachment_fatigue::attachment_fatigue;

#[path = "🪡️transverse-butt-weld/🦀️.rs"]
mod transverse_butt_weld;
pub use transverse_butt_weld::transverse_butt_weld_fatigue;

#[path = "🛠️built-up-weld/🦀️.rs"]
mod built_up_weld;
pub use built_up_weld::built_up_weld_fatigue;

#[path = "🔩️plain-and-fastened-fatigue/🦀️.rs"]
mod plain_and_fastened_fatigue;
pub use plain_and_fastened_fatigue::plain_and_fastened_fatigue;

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

    pub fn t(&self, key: &str) -> Result<&str, String> {
        self.0.values.texts.get(key).map(String::as_str).ok_or_else(|| format!("missing input {key}"))
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

const HOT_SPOT_DETAILS: [(&str, f64, bool); 7] = [
    ("ground-butt", 112.0, false),
    ("as-welded-butt", 100.0, false),
    ("full-penetration-cruciform", 100.0, true),
    ("non-load-bearing-fillet", 100.0, true),
    ("bracket-stiffener-end", 100.0, true),
    ("cover-plate-end", 100.0, true),
    ("load-bearing-fillet-cruciform", 90.0, true),
];

/// 🔩 EN 1993-1-9:2005 Annex B resistance with weld-toe structural stresses.
pub fn hot_spot_fatigue(q: &Inputs<'_>) -> Outcome {
    let detail = q.t("hot_spot_detail")?;
    let Some((row, &(_, category, check_angle))) = HOT_SPOT_DETAILS.iter().enumerate().find(|(_, item)| item.0 == detail) else {
        return Err(format!("unknown model {detail}"));
    };
    if q.t("hot_spot_crack")? != "toe" {
        return Err("Table B.1 applies to cracks starting at the weld toe".into());
    }
    if q.t("hot_spot_basis")? != "confirmed" {
        return Err("Confirm the Table B.1 fabrication conditions and include misalignment in the structural stress".into());
    }
    if check_angle && !(0.0..=60.0).contains(&q.n("hot_spot_angle")?) {
        return Err("Table B.1 requires a weld toe angle of at most 60 degrees for this detail".into());
    }

    let gamma_ff = q.n("gamma_ff")?;
    let gamma_mf = q.n("gamma_mf")?;
    let range = q.n("hot_spot_range")?;
    let frequent = q.n("hot_spot_frequent")?;
    let fy = q.n("fy")?;
    if ![gamma_ff, gamma_mf, fy].into_iter().all(|value| value.is_finite() && value > 0.0)
        || ![range, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0)
    {
        return Err("Enter finite stresses and positive partial factors and yield strength".into());
    }
    let demand = range * gamma_ff;
    let capacity = category / gamma_mf;
    let frequent_limit = 1.5 * fy;
    Ok(vec![
        Row::new("fatigue", "Weld toe fatigue resistance", "Ermüdungsfestigkeit am Nahtübergang", QuantityKind::Stress, demand * 1e6, capacity * 1e6, format!("B.1({}): γ_Ff Δσ_E,2 = {demand:.2} MPa ≤ Δσ_C/γ_Mf = {capacity:.2} MPa (Δσ_C = {category:.0} MPa).", row + 1)),
        Row::new("frequent-range", "Frequent direct stress range", "Häufige Normalspannungsschwingbreite", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6, format!("Δσ(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y = {frequent_limit:.2} MPa.")),
    ])
}

const RUNWAY_DETAILS: [(&str, f64, &str); 7] = [
    ("rolled", 160.0, "web"),
    ("full-tee", 71.0, "web"),
    ("partial-tee", 36.0, "throat"),
    ("fillet", 36.0, "throat"),
    ("t-full", 71.0, "web"),
    ("t-partial", 36.0, "throat"),
    ("t-fillet", 36.0, "throat"),
];

/// 🚋 EN 1993-1-9:2005+AC:2009 Table 8.10 and §8(1)–(2), using equivalent and frequent wheel-load stress ranges.
pub fn runway_junction_fatigue(q: &Inputs<'_>) -> Outcome {
    let detail = q.t("runway_detail")?;
    let Some((row, &(_, category, location))) = RUNWAY_DETAILS.iter().enumerate().find(|(_, item)| item.0 == detail) else {
        return Err(format!("unknown model {detail}"));
    };
    if q.t("runway_basis")? != "confirmed" {
        return Err("Confirm the Table 8.10 geometry and wheel-load stress basis, including EN 1993-1-8 conformity where effective full penetration is used".into());
    }
    if q.t("runway_location")? != location {
        return Err(if location == "web" { "Use the vertical compressive stress range in the web" } else { "Use the vertical compressive stress range in the weld throat" }.into());
    }
    let range = q.n("runway_range")?;
    let frequent = q.n("runway_frequent")?;
    let gamma_ff = q.n("gamma_ff")?;
    let gamma_mf = q.n("gamma_mf")?;
    let fy = q.n("fy")?;
    if ![range, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || ![gamma_ff, gamma_mf, fy].into_iter().all(|value| value.is_finite() && value > 0.0)
    {
        return Err("Enter finite stresses and positive partial factors and yield strength".into());
    }

    let (en, de, mark) = if location == "web" {
        ("Web fatigue resistance", "Ermüdungsfestigkeit des Stegs", "")
    } else {
        ("Weld-throat fatigue resistance", "Ermüdungsfestigkeit des Nahtquerschnitts", "*")
    };
    let demand = range * gamma_ff;
    let capacity = category / gamma_mf;
    let frequent_limit = 1.5 * fy;
    Ok(vec![
        Row::new("fatigue", en, de, QuantityKind::Stress, demand * 1e6, capacity * 1e6, format!("8.10({}): Δσ_C = {category:.0}{mark} MPa. γ_Ff Δσ_E,2 = {demand:.2} MPa ≤ Δσ_C/γ_Mf = {capacity:.2} MPa.", row + 1)),
        Row::new("frequent-range", "Frequent direct stress range", "Häufige Normalspannungsschwingbreite", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6, format!("Δσ(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y = {frequent_limit:.2} MPa.")),
    ])
}

/// 🚧 EN 1993-1-9 Table 8.9 detail 1, with the category selected at the 12 mm thickness boundary.
pub fn open_stringer_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("deck_open_end_basis")? != "confirmed" {
        return Err("Confirm the open stringer geometry and the direct stress basis for Table 8.9 detail 1".into());
    }
    let thickness = q.n("deck_open_thickness")?;
    if thickness <= 0.0 {
        return Err("Enter a positive plate thickness for Table 8.9 detail 1".into());
    }
    let category = if thickness <= 12.0 { 80.0 } else { 71.0 };
    nominal_direct_checks(q, q.n("deck_open_range")?, q.n("deck_open_frequent_direct")?, category, format!("8.9(1): t = {thickness} mm"))
}

/// 🌁 EN 1993-1-9 Table 8.9 detail 2, using the combined cross-girder stress range before damage equivalence.
pub fn continuous_open_stringer_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("deck_open_cross_basis")? != "confirmed" {
        return Err("Confirm the continuous open stringer and the cross-girder web stress basis for Table 8.9 detail 2".into());
    }
    let normal = q.n("deck_cross_direct")?;
    let shear = q.n("deck_cross_shear")?;
    let equivalence = q.n("deck_cross_equivalence")?;
    if equivalence <= 0.0 {
        return Err("Enter a positive damage equivalence and impact factor".into());
    }

    let combined = 0.5 * (normal + normal.hypot(2.0 * shear));
    let mut rows = nominal_direct_checks(q, equivalence * combined, q.n("deck_open_frequent_direct")?, 56.0, format!("8.9(2): Δσ_eq = ½(Δσ + √(Δσ² + 4 Δτ²)) = {combined:.2} MPa"))?;
    let frequent_shear = q.n("deck_cross_frequent_shear")?;
    let shear_limit = 1.5 * q.n("fy")? / 3.0_f64.sqrt();
    rows.push(Row::new("frequent-shear", "Frequent shear stress range", "Häufige Schubspannungsschwingbreite", QuantityKind::Stress, frequent_shear * 1e6, shear_limit * 1e6, format!("Δτ(ψ₁ Q_k) = {frequent_shear:.2} MPa ≤ 1.5 f_y / √3 = {shear_limit:.2} MPa.")));
    let between = q.n("deck_between_utilization")?;
    rows.push(Row::ratio("between-stringers", "Separate deck fatigue check between stringers", "Separater Ermüdungsnachweis zwischen den Längsrippen", between, format!("EN 1993-2: η = {between:.3} ≤ 1.")));
    Ok(rows)
}

fn nominal_direct_checks(q: &Inputs<'_>, equivalent: f64, frequent: f64, category: f64, reference: String) -> Outcome {
    let demand = q.n("gamma_ff")? * equivalent;
    let capacity = category / q.n("gamma_mf")?;
    let frequent_limit = 1.5 * q.n("fy")?;
    Ok(vec![
        Row::new("fatigue", "Fatigue resistance", "Ermüdungsfestigkeit", QuantityKind::Stress, demand * 1e6, capacity * 1e6, format!("{reference}. γ_Ff Δσ_E,2 = {demand:.2} MPa ≤ Δσ_C,red/γ_Mf = {capacity:.2} MPa.")),
        Row::new("frequent-range", "Frequent direct stress range", "Häufige Normalspannungsschwingbreite", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6, format!("Δσ(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y = {frequent_limit:.2} MPa.")),
    ])
}

/// 🚉 EN 1993-1-9 Table 8.8 details 1 to 4 with longitudinal stress ranges in the closed stringer.
pub fn closed_stringer_fatigue(q: &Inputs<'_>) -> Outcome {
    let detail = q.t("closed_connection_detail")?;
    let required_confirmation = if detail == "backed" { "backed-confirmed" } else { "confirmed" };
    let (detail_number, category) = match detail {
        "cutout" | "continuous" => {
            let thickness = q.n("closed_web_thickness")?;
            if thickness <= 0.0 {
                return Err("Enter a positive cross-girder web thickness".into());
            }
            let number = if detail == "cutout" { 1 } else { 2 };
            (number, if thickness <= 12.0 { 80.0 } else { 71.0 })
        }
        "separate" => (3, 36.0),
        "backed" => (4, 71.0),
        _ => return Err(format!("unknown model {detail}")),
    };
    closed_stringer_direct_checks(q, required_confirmation, category, format!("8.8({detail_number})"))
}

/// 🕳️ EN 1993-1-9 Table 8.8 detail 6 using the qualified critical-section stress range.
pub fn closed_stringer_cutout_fatigue(q: &Inputs<'_>) -> Outcome {
    let method = q.t("closed_cutout_method")?;
    let category = match method {
        "vierendeel" => 71.0,
        "bridge-method" => 112.0,
        _ => return Err(format!("unknown model {method}")),
    };
    closed_stringer_direct_checks(q, "confirmed", category, "8.8(6)".into())
}

/// 🪚 EN 1993-1-9 Table 8.8 detail 5 with the size factor of the qualified Table 8.3 weld group.
pub fn closed_stringer_splice_fatigue(q: &Inputs<'_>) -> Outcome {
    let group = q.t("closed_splice_group")?;
    let category = match group {
        "ground" => 112.0,
        "flat" => 90.0,
        "as-welded" => 80.0,
        _ => return Err(format!("unknown model {group}")),
    };
    let thickness = q.n("closed_splice_thickness")?;
    if thickness <= 0.0 {
        return Err("Enter a positive rib splice thickness".into());
    }
    let size_factor = (25.0 / thickness).min(1.0).powf(0.2);
    closed_stringer_direct_checks(q, "confirmed", size_factor * category, format!("8.8(5): Δσ_C = {category:.0} MPa; k_s = {size_factor:.4}"))
}

fn closed_stringer_direct_checks(q: &Inputs<'_>, confirmation: &str, category: f64, reference: String) -> Outcome {
    if q.t("closed_geometry")? != confirmation {
        return Err("Confirm all applicable geometry, fabrication and stress requirements, including backing tack-weld placement for detail 4".into());
    }
    nominal_direct_checks(q, q.n("closed_range")?, q.n("closed_frequent")?, category, reference)
}

/// 🛣️ EN 1993-1-9 Table 8.8 details 7 and 8 with the weld-section bending stress.
pub fn closed_stringer_deck_weld_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("closed_geometry")? != "confirmed" {
        return Err("Confirm the rib and weld geometry and the bending stress basis".into());
    }
    let rib = q.n("closed_weld_rib_thickness")?;
    let throat = q.n("closed_weld_throat")?;
    let gap = q.n("closed_weld_gap")?;
    let root = q.n("closed_weld_root_face")?;
    if rib <= 0.0 || throat <= 0.0 {
        return Err("Enter positive rib wall and weld throat thicknesses".into());
    }
    let kind = q.t("closed_weld_type")?;
    let (detail, category, thickness) = match kind {
        "recessed" if throat >= rib && gap <= 2.0 && root <= 2.0 => (7, 71.0, rib),
        "recessed" | "fillet" if gap <= 0.5 => (8, 50.0, throat),
        "recessed" | "fillet" => return Err("The weld fit-up dimensions exceed the limits in Table 8.8".into()),
        _ => return Err(format!("unknown model {kind}")),
    };
    let modulus = thickness.powi(2) / 6.0;
    let equivalent = q.n("closed_weld_moment")? / modulus;
    let frequent = q.n("closed_weld_frequent_moment")? / modulus;
    nominal_direct_checks(q, equivalent, frequent, category, format!("8.8({detail}): W_w = {modulus:.3} mm²"))
}

const CHS_LATTICE_FACTORS: [[Option<f64>; 3]; 4] = [
    [Some(1.5), None, Some(1.3)],
    [Some(1.5), Some(1.8), Some(1.4)],
    [Some(1.5), None, Some(1.2)],
    [Some(1.5), Some(1.65), Some(1.25)],
];

const RHS_LATTICE_FACTORS: [[Option<f64>; 3]; 4] = [
    [Some(1.5), None, Some(1.5)],
    [Some(1.5), Some(2.2), Some(1.6)],
    [Some(1.5), None, Some(1.3)],
    [Some(1.5), Some(2.0), Some(1.4)],
];

/// 🔲 Factors from EN 1993-1-9:2005 with AC:2009, Tables 4.1 and 4.2.
fn lattice_factor(section: &str, joint: &str, member: &str, [beta, gamma, tau, angle]: [f64; 4]) -> Result<(&'static str, f64), String> {
    let (table, factors, beta_range, gamma_range) = match section {
        "chs" => ("4.1", &CHS_LATTICE_FACTORS, 0.30..=0.60, 12.0..=30.0),
        "rhs" => ("4.2", &RHS_LATTICE_FACTORS, 0.40..=0.60, 6.25..=12.5),
        _ => return Err(format!("unknown model {section}")),
    };
    let row = match joint {
        "gap-k" => 0,
        "gap-n" | "gap-n-kt" => 1,
        "overlap-k" => 2,
        "overlap-n" | "overlap-n-kt" => 3,
        _ => return Err(format!("unknown model {joint}")),
    };
    let col = match member {
        "chord" => 0,
        "vertical" => 1,
        "diagonal" => 2,
        _ => return Err(format!("unknown model {member}")),
    };
    let Some(k_1) = factors[row][col] else {
        return Err("The selected table has no factor for a K-joint vertical".into());
    };
    if !beta_range.contains(&beta) || !gamma_range.contains(&gamma) || !(0.25..=1.0).contains(&tau) || !(30.0..=60.0).contains(&angle) {
        return Err(format!("The joint geometry exceeds the bounds of Table {table}"));
    }
    Ok((table, k_1))
}

/// 🔲 EN 1993-1-9:2005 with AC:2009, Tables 4.1 and 4.2 and Eq. (6.3).
pub fn lattice_joint_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("lattice_basis")? != "confirmed" {
        return Err("Confirm the in-plane pinned model, external member loads and Table 8.7 detail".into());
    }
    let (table, k_1) = lattice_factor(q.t("lattice_section")?, q.t("lattice_joint")?, q.t("lattice_member")?, [
        q.n("lattice_beta")?, q.n("lattice_gamma")?, q.n("lattice_tau")?, q.n("lattice_angle")?,
    ])?;
    let range = q.n("lattice_range")?;
    let category = q.n("delta_sigma_c")?;
    let gamma_ff = q.n("gamma_ff")?;
    let gamma_mf = q.n("gamma_mf")?;
    if !range.is_finite() || range < 0.0 || ![category, gamma_ff, gamma_mf].into_iter().all(|value| value.is_finite() && value > 0.0) {
        return Err("Enter a finite nonnegative stress range and positive category and partial factors".into());
    }
    let demand = k_1 * gamma_ff * range;
    let capacity = category / gamma_mf;
    Ok(vec![Row::new("fatigue", "Lattice joint fatigue resistance", "Ermüdungsfestigkeit des Fachwerkknotens", QuantityKind::Stress, demand * 1e6, capacity * 1e6, format!("{table}: k₁ = {k_1:.2}; k₁ γ_Ff Δσ_E,2 = {demand:.2} MPa ≤ Δσ_C/γ_Mf = {capacity:.2} MPa."))])
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
