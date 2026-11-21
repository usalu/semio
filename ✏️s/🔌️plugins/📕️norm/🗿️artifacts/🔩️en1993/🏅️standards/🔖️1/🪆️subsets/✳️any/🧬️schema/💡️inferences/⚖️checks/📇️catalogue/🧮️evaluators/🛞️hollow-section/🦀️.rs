//! 🛞️ Fatigue resistance of EN 1993-1-9 Table 8.6 hollow-section details.

use super::{Inputs, Outcome, QuantityKind, Row, nominal_direct_checks};

/// 🛞️ Verifies the tube longitudinal stress and the separate weld shear for a slotted tube.
pub fn hollow_section_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("hollow_execution")? != "confirmed" {
        return Err("Confirm the selected detail geometry, fabrication, inspection and stress locations".into());
    }
    let thickness = q.n("hollow_wall")?;
    if !(0.0 < thickness && thickness <= 12.5) {
        return Err("Table 8.6 is limited to positive wall thicknesses up to 12.5 mm".into());
    }
    let detail = q.t("hollow_detail")?;
    let (row, category) = match detail {
        "flattened" => {
            let diameter = q.n("hollow_diameter")?;
            if !(0.0 < diameter && diameter < 200.0) {
                return Err("Detail 1 requires a circular tube diameter below 200 mm".into());
            }
            (1, 71.0)
        }
        "slotted" => {
            let angle = q.n("hollow_slot_angle")?;
            if !(0.0..=180.0).contains(&angle) {
                return Err("Enter a valid slot detail angle".into());
            }
            (2, if angle <= 45.0 { 71.0 } else { 63.0 })
        }
        "chs-butt" | "rhs-butt" => {
            let weld = q.n("hollow_weld_dimension")?;
            let reinforcement = q.n("hollow_reinforcement")?;
            if !weld.is_finite() || weld <= 0.0 || !(0.0..=0.1 * weld).contains(&reinforcement) {
                return Err("The weld reinforcement is limited to 10% of the weld dimension for details 3 and 4".into());
            }
            match (detail, thickness > 8.0) {
                ("chs-butt", false) => (3, 71.0),
                ("chs-butt", true) => (3, 90.0),
                (_, false) => (4, 56.0),
                (_, true) => (4, 71.0),
            }
        }
        "non-load-bearing" => {
            let width = q.n("hollow_attachment_width")?;
            if !(0.0 < width && width <= 100.0) {
                return Err("Detail 5 has an attachment width limit of 100 mm along the stress. Other details require Table 8.4".into());
            }
            (5, 71.0)
        }
        "chs-butt-endplate" if thickness > 8.0 => (6, 56.0),
        "chs-butt-endplate" => (6, 50.0),
        "rhs-butt-endplate" if thickness > 8.0 => (7, 50.0),
        "rhs-butt-endplate" => (7, 45.0),
        "chs-fillet-endplate" | "rhs-fillet-endplate" if thickness > 8.0 => {
            return Err("Load-carrying fillet-welded endplates in details 8 and 9 require wall thickness at most 8 mm".into());
        }
        "chs-fillet-endplate" => (8, 40.0),
        "rhs-fillet-endplate" => (9, 36.0),
        _ => return Err("Select a construction detail listed in Table 8.6".into()),
    };
    let equivalent = q.n("hollow_range")?;
    let frequent = q.n("hollow_frequent")?;
    let gamma_ff = q.n("gamma_ff")?;
    let gamma_mf = q.n("gamma_mf")?;
    let fy = q.n("fy")?;
    if ![equivalent, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || ![gamma_ff, gamma_mf, fy].into_iter().all(|value| value.is_finite() && value > 0.0)
    {
        return Err("Enter finite nonnegative stress ranges and positive material and partial-factor values".into());
    }
    let mut rows = nominal_direct_checks(q, equivalent, frequent, category, format!("Table 8.6({row}), t = {thickness:.2} mm"))?;
    if row == 2 {
        let shear = q.n("hollow_weld_shear")?;
        let frequent_shear = q.n("hollow_weld_frequent_shear")?;
        if ![shear, frequent_shear].into_iter().all(|value| value.is_finite() && value >= 0.0) {
            return Err("Enter finite nonnegative weld throat shear stress ranges".into());
        }
        let demand = gamma_ff * shear;
        let capacity = 80.0 / gamma_mf;
        let frequent_limit = 1.5 * fy / 3.0_f64.sqrt();
        rows.extend([
            Row::new("weld-shear-fatigue", "Weld throat shear fatigue", "Schubermüdung im Nahtquerschnitt", QuantityKind::Stress, demand * 1e6, capacity * 1e6,
                format!("Table 8.5(8): Δτ_C = 80 MPa, m = 5. γ_Ff Δτ_E,2 = {demand:.2} MPa ≤ Δτ_C/γ_Mf = {capacity:.2} MPa.")),
            Row::new("weld-shear-frequent", "Frequent weld throat shear range", "Häufige Schubspannungsschwingbreite im Nahtquerschnitt", QuantityKind::Stress, frequent_shear * 1e6, frequent_limit * 1e6,
                format!("Δτ(ψ₁ Q_k) = {frequent_shear:.2} MPa ≤ 1.5 f_y / √3 = {frequent_limit:.2} MPa.")),
        ]);
    }
    Ok(rows)
}
