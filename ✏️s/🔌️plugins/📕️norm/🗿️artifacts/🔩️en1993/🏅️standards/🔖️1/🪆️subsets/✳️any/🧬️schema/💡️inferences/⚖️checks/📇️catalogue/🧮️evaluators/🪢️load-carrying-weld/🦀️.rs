//! 🪢️ EN 1993-1-9 Table 8.5 fatigue of load-carrying welded joints.

use super::{Inputs, Outcome, QuantityKind, Row, nominal_direct_checks};

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn ranges(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite() && *value >= 0.0)
}

pub(super) fn toe_category(length: f64, thickness: f64) -> Result<f64, String> {
    if !positive(length) || !positive(thickness) {
        return Err("Enter positive dimensions ℓ and t for the assessed plate".into());
    }
    Ok(match length {
        l if l < 50.0 => 80.0,
        l if l == 50.0 => return Err("The published Table 8.5 detail 1 covers ℓ < 50 mm and 50 mm < ℓ ≤ 80 mm. Obtain a documented category for ℓ = 50 mm".into()),
        l if l <= 80.0 => 71.0,
        l if l <= 100.0 => 63.0,
        l if l <= 120.0 || thickness <= 20.0 => 56.0,
        l if l <= 200.0 || thickness <= 30.0 => 50.0,
        l if l <= 300.0 || thickness <= 50.0 => 45.0,
        _ => 40.0,
    })
}

fn cover_category(q: &Inputs<'_>) -> Result<f64, String> {
    let length = q.n("weld85_length")?;
    let thickness = q.n("weld85_plate")?;
    let cover = q.n("weld85_cover")?;
    if ![length, thickness, cover].into_iter().all(positive) {
        return Err("Enter positive cover-plate length and plate thicknesses".into());
    }
    if length < 300.0 {
        return toe_category(length, thickness);
    }
    let thinner = cover < thickness;
    Ok(match (thickness, thinner) {
        (t, true) if t <= 20.0 => 56.0,
        (t, true) if t <= 30.0 => 50.0,
        (t, true) if t <= 50.0 => 45.0,
        (_, true) => 40.0,
        (t, false) if t <= 20.0 => 50.0,
        (t, false) if t <= 30.0 => 45.0,
        (t, false) if t <= 50.0 => 40.0,
        (_, false) => 36.0,
    })
}

fn end_distance(q: &Inputs<'_>) -> Result<(), String> {
    let distance = q.n("weld85_end_distance")?;
    if !distance.is_finite() || distance < 10.0 {
        return Err("Weld ends must be at least 10 mm from the plate end".into());
    }
    Ok(())
}

fn toe_and_root_alignment(q: &Inputs<'_>) -> Result<(), String> {
    let thickness = q.n("weld85_intermediate")?;
    let offset = q.n("weld85_misalignment")?;
    if !positive(thickness) || !(0.0..=0.15 * thickness).contains(&offset) {
        return Err("Load-carrying plate misalignment must be at most 15% of the intermediate plate thickness".into());
    }
    Ok(())
}

fn shear_rows(q: &Inputs<'_>, reference: &str) -> Outcome {
    let range = q.n("weld85_shear")?;
    let frequent = q.n("weld85_shear_frequent")?;
    if !ranges(&[range, frequent]) {
        return Err("Enter finite nonnegative ranges for longitudinal weld throat shear".into());
    }
    let demand = q.n("gamma_ff")? * range;
    let limit = 80.0 / q.n("gamma_mf")?;
    let frequent_limit = 1.5 * q.n("fy")? / 3.0_f64.sqrt();
    Ok(vec![
        Row::new("weld-shear-fatigue", "Weld throat shear fatigue", "Schubermüdung im Nahtquerschnitt", QuantityKind::Stress, demand * 1e6, limit * 1e6,
            format!("{reference}. Δτ_C = 80 MPa, m = 5. γ_Ff Δτ_E,2 = {demand:.2} MPa ≤ Δτ_C/γ_Mf = {limit:.2} MPa.")),
        Row::new("weld-shear-frequent", "Frequent weld throat shear range", "Häufige Schubspannungsschwingbreite im Nahtquerschnitt", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6,
            format!("Δτ(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y / √3 = {frequent_limit:.2} MPa.")),
    ])
}

fn root_rows(q: &Inputs<'_>) -> Outcome {
    let range = q.n("weld85_root")?;
    let frequent = q.n("weld85_root_frequent")?;
    if !ranges(&[range, frequent]) {
        return Err("Enter finite nonnegative ranges for the transverse weld throat stress".into());
    }
    let demand = q.n("gamma_ff")? * range;
    let limit = 36.0 / q.n("gamma_mf")?;
    let frequent_limit = 1.5 * q.n("fy")?;
    let mut rows = vec![
        Row::new("weld-root-fatigue", "Weld throat direct fatigue", "Normalspannungsermüdung im Nahtquerschnitt", QuantityKind::Stress, demand * 1e6, limit * 1e6,
            format!("Table 8.5(3), category 36*. γ_Ff Δσ_w,E,2 = {demand:.2} MPa ≤ Δσ_C/γ_Mf = {limit:.2} MPa.")),
        Row::new("weld-root-frequent", "Frequent weld throat direct range", "Häufige Normalspannungsschwingbreite im Nahtquerschnitt", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6,
            format!("Δσ_w(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y = {frequent_limit:.2} MPa.")),
    ];
    let shear = shear_rows(q, "Table 8.5(3)")?;
    let interaction = (demand / limit).powi(3) + (shear[0].demand / shear[0].capacity).powi(5);
    rows.extend(shear);
    rows.push(Row::ratio("weld-root-interaction", "Combined weld throat fatigue", "Kombinierte Ermüdung im Nahtquerschnitt", interaction, format!("Eq. (8.3). u_σ³ + u_τ⁵ = {interaction:.5} ≤ 1.")));
    Ok(rows)
}

/// 🪢️ Classifies Table 8.5 joints and verifies each required stress location for the selected detail.
pub fn load_carrying_weld_fatigue(q: &Inputs<'_>) -> Outcome {
    let detail = q.t("weld85_detail")?;
    if detail == "stud" {
        return Err("Table 8.5(10) uses the EN 1994-2 headed-stud fatigue procedure, with category 90 MPa and m = 8 at the nominal stud section".into());
    }
    if q.t("weld85_basis")? != "confirmed" {
        return Err("Confirm the selected geometry, fabrication and inspection requirements, and the specified stress locations for the detail".into());
    }
    if ![q.n("fy")?, q.n("gamma_ff")?, q.n("gamma_mf")?].into_iter().all(positive) {
        return Err("Enter finite positive yield strength and fatigue partial factors".into());
    }
    let (reference, category) = match detail {
        "toe" | "modified-toe" | "root" | "lap-main" => {
            if detail == "lap-main" {
                end_distance(q)?;
            } else {
                toe_and_root_alignment(q)?;
            }
            let reference = match detail {
                "toe" => "Table 8.5(1)",
                "modified-toe" => "Table 8.5(2), modified nominal stress",
                "root" => "Table 8.5(3), loaded plate toe as detail 1",
                _ => "Table 8.5(4), main plate stress at the specified effective area",
            };
            (reference, toe_category(q.n("weld85_length")?, q.n("weld85_plate")?)?)
        }
        "lap-cover" => {
            end_distance(q)?;
            ("Table 8.5(5), category 45* at the overlap plate", 45.0)
        }
        "cover-end" => ("Table 8.5(6), base classification", cover_category(q)?),
        "cover-reinforced" => {
            let cover = q.n("weld85_cover")?;
            let weld = q.n("weld85_reinforcing_length")?;
            if !positive(cover) || !positive(weld) || weld < 5.0 * cover {
                return Err("The reinforcing end weld must be at least five times the cover-plate thickness".into());
            }
            if cover > 20.0 && !(0.0..0.25).contains(&q.n("weld85_slope")?) {
                return Err("A cover plate thicker than 20 mm requires an end slope below 1:4".into());
            }
            ("Table 8.5(7), reinforced and flush-ground end weld", 56.0)
        }
        "continuous-shear" => return shear_rows(q, "Table 8.5(8)"),
        "lap-shear" => {
            end_distance(q)?;
            return shear_rows(q, "Table 8.5(9), shear based on the full weld length");
        }
        "ring-butt" => {
            if !(0.8..=1.0).contains(&q.n("weld85_penetration")?) {
                return Err("Detail 11 requires a butt weld with at least 80% penetration and a ground weld toe".into());
            }
            ("Table 8.5(11), tube section at ring flange", 71.0)
        }
        "ring-fillet" => ("Table 8.5(12), tube section at ring flange", 40.0),
        _ => return Err("Select a construction detail listed in Table 8.5".into()),
    };
    let equivalent = q.n("weld85_range")?;
    let frequent = q.n("weld85_frequent")?;
    if !ranges(&[equivalent, frequent]) {
        return Err("Enter finite nonnegative direct stress ranges for the checked section".into());
    }
    let mut rows = nominal_direct_checks(q, equivalent, frequent, category, reference.into())?;
    match detail {
        "root" => rows.extend(root_rows(q)?),
        "lap-main" | "lap-cover" => rows.extend(shear_rows(q, "Table 8.5(8), lap joint weld throat")?),
        _ => {}
    }
    Ok(rows)
}
