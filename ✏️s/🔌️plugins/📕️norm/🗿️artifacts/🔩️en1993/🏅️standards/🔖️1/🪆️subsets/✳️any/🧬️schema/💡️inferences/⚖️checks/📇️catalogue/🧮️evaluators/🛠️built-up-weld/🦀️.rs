//! 🛠️ Nominal fatigue categories for longitudinal welds under EN 1993-1-9 Table 8.2.

use super::{Inputs, Outcome, Row, attachment_fatigue::longitudinal_category, nominal_direct_checks};

fn positive(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite positive value for {key}"))
    }
}

fn stress(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite nonnegative value for {key}"))
    }
}

fn start_stop(q: &Inputs<'_>) -> Result<&'static str, String> {
    match q.t("weld82_stop")? {
        "none" => Ok("none"),
        "ordinary" => Ok("ordinary"),
        "qualified-repair" => Ok("qualified-repair"),
        _ => Err("Declare whether the weld has start or stop positions".into()),
    }
}

fn cope_category(q: &Inputs<'_>) -> Result<(f64, &'static str), String> {
    let height = positive(q, "weld82_cope_height")?;
    if height == 60.0 && q.t("weld82_cope_basis")? != "english" {
        return Err("Exactly 60 mm requires an approved category basis. The English Table 8.2 includes this boundary".into());
    }
    if height <= 60.0 {
        return Ok((71.0, "Table 8.2(9), flange longitudinal stress at the cope hole"));
    }
    let length = positive(q, "weld82_attachment_length")?;
    if positive(q, "weld82_attachment_thickness")? >= positive(q, "weld82_attachment_height")? {
        return Err("For a cope hole over 60 mm, Table 8.4(1) requires the attachment thickness to be less than its height".into());
    }
    Ok((longitudinal_category(length), "Table 8.2(9) referring to Table 8.4(1) for larger cope holes"))
}

fn repaired_category(q: &Inputs<'_>) -> Result<f64, String> {
    match q.t("weld82_repair")? {
        "repaired" => Ok(100.0),
        "restored-125" => Ok(125.0),
        "restored-112" => Ok(112.0),
        "restored-100" => Ok(100.0),
        _ => Err("Select the repaired weld category or document specialist grinding, satisfactory verification and the original category".into()),
    }
}

/// 🛠️ Verifies the longitudinal weld and records the separate cover-plate end check where required.
pub fn built_up_weld_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("weld82_basis")? != "confirmed" {
        return Err("Confirm the specified weld process, preparation, inspection and stress location".into());
    }
    for key in ["fy", "gamma_ff", "gamma_mf"] {
        positive(q, key)?;
    }
    let detail = q.t("weld82_detail")?;
    let (category, reference) = match detail {
        "automatic-butt" | "automatic-fillet" => {
            if start_stop(q)? == "ordinary" {
                return Err("Details 1 and 2 exclude ordinary start or stop positions. Check detail 3 when applicable".into());
            }
            (125.0, if detail == "automatic-butt" { "Table 8.2(1), double-sided mechanized butt weld" } else { "Table 8.2(2), mechanized fillet weld" })
        }
        "two-sided-stops" => {
            if start_stop(q)? == "none" {
                return Err("Detail 3 represents a mechanized weld containing start or stop positions".into());
            }
            (112.0, "Table 8.2(3), two-sided mechanized weld with stops")
        }
        "single-backed" => (if start_stop(q)? == "none" { 112.0 } else { 100.0 }, "Table 8.2(4), one-sided weld with continuous backing"),
        "manual" => (100.0, "Table 8.2(5), manually made longitudinal weld"),
        "single-sided" => (100.0, "Table 8.2(6), one-sided longitudinal butt weld with adequate fit"),
        "repair" => (repaired_category(q)?, "Table 8.2(7), repaired weld"),
        "intermittent" => {
            let gap = positive(q, "weld82_gap")?;
            let height = positive(q, "weld82_web_height")?;
            if gap / height > 2.5 {
                return Err("Detail 8 requires the weld gap to web height ratio g/h to be at most 2.5".into());
            }
            (80.0, "Table 8.2(8), flange longitudinal stress at an intermittent weld")
        }
        "cope-hole" => cope_category(q)?,
        "longitudinal-butt" => {
            let category = match q.t("weld82_finish")? {
                "ground-ndt" => 125.0,
                "unground" => if start_stop(q)? == "none" { 112.0 } else { 90.0 },
                _ => return Err("Declare the longitudinal butt weld finishing and inspection conditions".into()),
            };
            (category, "Table 8.2(10), longitudinal butt seam")
        }
        "hollow-seam" => {
            let thickness = positive(q, "weld82_wall")?;
            let category = if start_stop(q)? != "none" { 90.0 } else if thickness <= 12.5 { 140.0 } else { 125.0 };
            (category, "Table 8.2(11), mechanized hollow-section seam")
        }
        _ => return Err("Select a longitudinal weld detail listed in Table 8.2".into()),
    };
    let equivalent = stress(q, "weld82_range")?;
    let frequent = stress(q, "weld82_frequent")?;
    let mut rows = nominal_direct_checks(q, equivalent, frequent, category, reference.into())?;
    if detail == "automatic-fillet" {
        match q.t("weld82_ends")? {
            "none" => {}
            "assessed" => {
                let utilization = stress(q, "weld82_ends_utilization")?;
                rows.push(Row::ratio("cover-end-fatigue", "Cover-plate-end fatigue", "Ermüdung am Gurtlamellenende", utilization,
                    format!("Governing utilization of the separate Table 8.5(6) or (7) assessment = {utilization:.4} ≤ 1.")));
            }
            _ => return Err("Assess the cover-plate ends under Table 8.5(6) or (7) and enter their governing utilization".into()),
        }
    }
    Ok(rows)
}
