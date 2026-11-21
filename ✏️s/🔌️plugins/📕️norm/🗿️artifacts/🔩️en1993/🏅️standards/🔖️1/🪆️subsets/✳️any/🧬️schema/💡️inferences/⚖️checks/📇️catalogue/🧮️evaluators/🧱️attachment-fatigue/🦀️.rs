//! 🧱️ Fatigue of weld attachments and stiffeners under DIN EN 1993-1-9 Table 8.4.

use super::{Inputs, Outcome, nominal_direct_checks};

fn dimension(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite positive value for {key}"))
    }
}

pub(super) fn longitudinal_category(length: f64) -> f64 {
    match length {
        l if l <= 50.0 => 80.0,
        l if l <= 80.0 => 71.0,
        l if l <= 100.0 => 63.0,
        _ => 56.0,
    }
}

pub(super) fn edge_gusset_category(radius: f64, length: f64) -> f64 {
    if radius > 150.0 || radius / length >= 1.0 / 3.0 {
        90.0
    } else if radius / length >= 1.0 / 6.0 {
        71.0
    } else {
        50.0
    }
}

/// 🧱️ Chooses the classified detail for the nominal or specified principal stress range.
pub fn attachment_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("weld84_basis")? != "confirmed" {
        return Err("Confirm the selected detail, required weld finishing and applicable stress definition".into());
    }
    let (reference, category) = match q.t("weld84_detail")? {
        "longitudinal" => {
            let length = dimension(q, "weld84_length")?;
            if dimension(q, "weld84_thickness")? >= dimension(q, "weld84_height")? {
                return Err("Detail 1 requires an attachment thinner than its height. Use Table 8.5 detail 5 or 6 where applicable".into());
            }
            ("Table 8.4(1), longitudinal attachment", longitudinal_category(length))
        }
        "sloped" => {
            let length = dimension(q, "weld84_length")?;
            let angle = q.n("weld84_angle")?;
            if length <= 100.0 || !(0.0..45.0).contains(&angle) {
                return Err("Detail 2 requires attachment length over 100 mm and angle below 45°".into());
            }
            ("Table 8.4(2), longitudinal attachment with sloped transition", 71.0)
        }
        "gusset-reinforced" => {
            let radius = dimension(q, "weld84_radius")?;
            let weld = dimension(q, "weld84_reinforced_length")?;
            if radius <= 150.0 || weld <= radius {
                return Err("Detail 3 requires a radius over 150 mm and a full-penetration reinforced weld extending beyond that radius".into());
            }
            ("Table 8.4(3), reinforced and finished gusset transition", 80.0)
        }
        "gusset-edge" => {
            let radius = dimension(q, "weld84_radius")?;
            let length = dimension(q, "weld84_length")?;
            ("Table 8.4(4), corrected radius to length ratio r/ℓ", edge_gusset_category(radius, length))
        }
        "gusset-as-welded" => ("Table 8.4(5), as-welded gusset without transition radius", 40.0),
        detail @ ("transverse" | "beam-stiffener" | "web-stiffener" | "box-diaphragm") => {
            let length = dimension(q, "weld84_length")?;
            if length > 80.0 {
                return Err("The selected transverse attachment has tabulated categories only up to ℓ = 80 mm".into());
            }
            let reference = match detail {
                "transverse" => "Table 8.4(6), transverse plate stiffener",
                "beam-stiffener" => "Table 8.4(7), vertical beam stiffener",
                "web-stiffener" => "Table 8.4(7), principal stress at a stiffener ending in the web",
                _ => "Table 8.4(8), box-girder diaphragm or qualifying ring stiffener",
            };
            (reference, if length <= 50.0 { 80.0 } else { 71.0 })
        }
        "stud-base" => ("Table 8.4(9), base material under a welded stud", 80.0),
        _ => return Err("Select an attachment or stiffener listed in Table 8.4".into()),
    };
    let equivalent = q.n("weld84_range")?;
    let frequent = q.n("weld84_frequent")?;
    if ![equivalent, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0) {
        return Err("Enter finite nonnegative stress ranges at the specified section".into());
    }
    for key in ["fy", "gamma_ff", "gamma_mf"] {
        dimension(q, key)?;
    }
    nominal_direct_checks(q, equivalent, frequent, category, reference.into())
}
