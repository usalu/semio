//! 🪡️ Nominal fatigue checks for transverse butt welds in Table 8.3 of EN 1993-1-9.

use super::{Inputs, Outcome, attachment_fatigue::edge_gusset_category, load_carrying_weld::toe_category, nominal_direct_checks};

fn positive(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite positive value for {key}"))
    }
}

fn nonnegative(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite nonnegative value for {key}"))
    }
}

fn thickness_factor(thickness: f64) -> f64 {
    (25.0 / thickness.max(25.0)).powf(0.2)
}

fn limit_taper(q: &Inputs<'_>, maximum: f64) -> Result<(), String> {
    if nonnegative(q, "weld83_slope")? > maximum {
        return Err(format!("The selected detail requires a taper slope no more than {maximum}"));
    }
    Ok(())
}

fn limit_reinforcement(q: &Inputs<'_>, maximum: f64) -> Result<(), String> {
    if nonnegative(q, "weld83_reinforcement")? > maximum * positive(q, "weld83_weld_width")? {
        return Err(format!("Weld convexity exceeds the {}% limit for the selected detail", maximum * 100.0));
    }
    Ok(())
}

fn offset_factor(q: &Inputs<'_>, t1: f64) -> Result<f64, String> {
    let t2 = positive(q, "weld83_thicker")?;
    if t2 < t1 {
        return Err("Detail 17 requires t₂ ≥ t₁".into());
    }
    let e = nonnegative(q, "weld83_offset")?;
    let offset = 6.0 * (e / t1) / (1.0 + (t2 / t1).powf(1.5));
    Ok(thickness_factor(t1) / (1.0 + offset))
}

fn category_crossing(q: &Inputs<'_>, t: f64, radius: f64) -> Result<f64, String> {
    let length = positive(q, "weld83_length")?;
    match q.t("weld83_cross_reference")? {
        "table84" => Ok(edge_gusset_category(radius, length)),
        "table85" => toe_category(length, t),
        _ => Err("Confirm the approved classification reference for detail 19. The English and German tables cite different detail 4 references".into()),
    }
}

fn continuous_flange(q: &Inputs<'_>) -> Outcome {
    let (category, reference) = match q.t("weld83_flange")? {
        "rounded" => (edge_gusset_category(positive(q, "weld83_radius")?, positive(q, "weld83_length")?), "Table 8.4(4), continuous flange"),
        "as-welded" => (40.0, "Table 8.4(5), continuous flange"),
        _ => return Err("Select the weld transition detail for the continuous flange".into()),
    };
    let range = nonnegative(q, "weld83_perpendicular_range")?;
    let frequent = nonnegative(q, "weld83_perpendicular_frequent")?;
    let mut rows = nominal_direct_checks(q, range, frequent, category, reference.into())?;
    rows[0].key = "flange-perpendicular-fatigue";
    rows[0].en = "Continuous flange transverse fatigue";
    rows[0].de = "Ermüdung im durchlaufenden Flansch quer zur Lastrichtung";
    rows[1].key = "flange-perpendicular-frequent";
    rows[1].en = "Frequent transverse range in continuous flange";
    rows[1].de = "Häufige Querspannungsschwingbreite im durchlaufenden Flansch";
    Ok(rows)
}

/// 🪡️ Verifies the weld and, for intersecting flanges, the continuous flange in the perpendicular direction.
pub fn transverse_butt_weld_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("weld83_basis")? != "confirmed" {
        return Err("Confirm the selected geometry, preparation, welding and inspection conditions for the tabulated detail".into());
    }
    for key in ["fy", "gamma_ff", "gamma_mf"] {
        positive(q, key)?;
    }
    let detail = q.t("weld83_detail")?;
    let t = positive(q, "weld83_thickness")?;
    let k = thickness_factor(t);
    let (number, category, factor) = match detail {
        "ground-plate" => (1, 112.0, k),
        "ground-built-up" => (2, 112.0, k),
        "ground-rolled" => (3, 112.0, k),
        "ground-tapered" => {
            limit_taper(q, 0.25)?;
            (4, 112.0, k)
        }
        "profile-plate" | "profile-rolled" | "profile-tapered" => {
            limit_reinforcement(q, 0.10)?;
            let number = match detail {
                "profile-plate" => 5,
                "profile-rolled" => 6,
                _ => {
                    limit_taper(q, 0.25)?;
                    7
                }
            };
            (number, 90.0, k)
        }
        "ground-cope" => (8, 90.0, k),
        "as-welded-built-up" | "as-welded-cope" | "as-welded-plate" => {
            let (number, maximum) = match detail {
                "as-welded-built-up" => (9, 0.20),
                "as-welded-cope" => (10, 0.10),
                _ => (11, 0.20),
            };
            limit_reinforcement(q, maximum)?;
            (number, 80.0, k)
        }
        "unbacked-rolled" => (12, 63.0, 1.0),
        "single-sided" => (13, 36.0, 1.0),
        "single-sided-inspected" => (13, 71.0, k),
        "backed" | "backed-tapered" | "backed-lower-class" => {
            let distance = nonnegative(q, "weld83_end_distance")?;
            let fit = q.t("weld83_fit")?;
            if !matches!(fit, "good" | "uncertain") {
                return Err("Declare whether the backing strip has a confirmed good fit".into());
            }
            if detail != "backed" {
                limit_taper(q, 0.25)?;
            }
            if detail == "backed-lower-class" {
                if distance > 10.0 && fit == "good" {
                    return Err("Detail 16 requires uncertain fit or backing welds ending within 10 mm of the plate edge".into());
                }
                (16, 50.0, k)
            } else {
                if distance <= 10.0 || fit != "good" {
                    return Err("With uncertain fit or a backing weld within 10 mm of the plate edge use detail 16".into());
                }
                (if detail == "backed" { 14 } else { 15 }, 71.0, k)
            }
        }
        "unequal-section" => {
            limit_taper(q, 0.5)?;
            (17, 71.0, offset_factor(q, t)?)
        }
        "intersecting" => (18, 40.0, 1.0),
        "intersecting-rounded" => {
            let radius = positive(q, "weld83_radius")?;
            (19, category_crossing(q, t, radius)?, 1.0)
        }
        _ => return Err("Select a transverse butt weld listed in Table 8.3".into()),
    };
    let range = nonnegative(q, "weld83_range")?;
    let frequent = nonnegative(q, "weld83_frequent")?;
    let mut rows = nominal_direct_checks(q, range, frequent, category * factor, format!("Table 8.3({number}), Δσ_C = {category:.2} MPa and k_s = {factor:.5}"))?;
    if number == 18 || number == 19 {
        rows.extend(continuous_flange(q)?);
    }
    Ok(rows)
}
