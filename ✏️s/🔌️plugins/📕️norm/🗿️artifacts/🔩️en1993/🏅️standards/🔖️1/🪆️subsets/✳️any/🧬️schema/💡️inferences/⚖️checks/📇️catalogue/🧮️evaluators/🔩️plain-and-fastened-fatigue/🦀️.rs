//! 🔩️ Fatigue of unwelded members and mechanically fastened joints under EN 1993-1-9 Table 8.1.

use super::{Inputs, Outcome, QuantityKind, Row, nominal_direct_checks};

fn positive(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite positive value for {key}"))
    }
}

fn range(q: &Inputs<'_>, key: &str) -> Result<f64, String> {
    let value = q.n(key)?;
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(format!("Enter a finite nonnegative value for {key}"))
    }
}

fn clearance(q: &Inputs<'_>) -> Result<(), String> {
    let d = positive(q, "detail81_diameter")?;
    for key in ["detail81_end_parallel", "detail81_edge_transverse"] {
        if positive(q, key)? < 1.5 * d {
            return Err("The selected hole detail needs both edge and end distances of at least 1.5 times the bolt diameter".into());
        }
    }
    for key in ["detail81_pitch_parallel", "detail81_pitch_transverse"] {
        let pitch = range(q, key)?;
        if pitch > 0.0 && pitch < 2.5 * d {
            return Err("Where adjacent holes are present, pitching in both directions must be at least 2.5 times the bolt diameter".into());
        }
    }
    Ok(())
}

fn shear_rows(q: &Inputs<'_>, equivalent: f64, frequent: f64, reference: &str) -> Outcome {
    if ![equivalent, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0) {
        return Err("The nominal shear stress ranges must be finite and nonnegative".into());
    }
    let demand = q.n("gamma_ff")? * equivalent;
    let limit = 100.0 / q.n("gamma_mf")?;
    let frequent_limit = 1.5 * q.n("fy")? / 3.0_f64.sqrt();
    Ok(vec![
        Row::new("shear-fatigue", "Nominal shear fatigue", "Ermüdung unter Nennschubspannung", QuantityKind::Stress, demand * 1e6, limit * 1e6,
            format!("{reference}. Δτ_C = 100 MPa, m = 5. γ_Ff Δτ_E,2 = {demand:.2} MPa ≤ Δτ_C/γ_Mf = {limit:.2} MPa.")),
        Row::new("shear-frequent", "Frequent shear range", "Häufige Schubspannungsschwingbreite", QuantityKind::Stress, frequent * 1e6, frequent_limit * 1e6,
            format!("Δτ(ψ₁ Q_k) = {frequent:.2} MPa ≤ 1.5 f_y / √3 = {frequent_limit:.2} MPa.")),
    ])
}

fn member_shear(q: &Inputs<'_>, detail: &str) -> Outcome {
    let static_moment = range(q, "detail81_first_moment")?;
    let inertia = positive(q, "detail81_inertia")?;
    let thickness = positive(q, "detail81_thickness")?;
    let stress_per_force = static_moment / inertia / thickness;
    let equiv = range(q, "detail81_shear_force")? * stress_per_force;
    let frequent = range(q, "detail81_frequent_force")? * stress_per_force;
    let reference = if detail == "rolled-shear" { "Table 8.1(6), Δτ = ΔV S/(I t) in the rolled section" } else { "Table 8.1(7), Δτ = ΔV S/(I t) in the seamless section" };
    shear_rows(q, equiv, frequent, reference)
}

fn bolt_shear(q: &Inputs<'_>) -> Outcome {
    if q.t("detail81_threads")? != "outside" {
        return Err("For Table 8.1 detail 15 the threads must be outside the shear plane".into());
    }
    match q.t("detail81_bolt_type")? {
        "fitted" => {}
        "normal" if q.t("detail81_reversal")? == "none" && matches!(q.t("detail81_bolt_grade")?, "5.6" | "8.8" | "10.9") => {}
        _ => return Err("Detail 15 uses fitted bolts or normal class 5.6, 8.8 or 10.9 bolts without load reversal".into()),
    }
    shear_rows(q, range(q, "detail81_range")?, range(q, "detail81_frequent")?, "Table 8.1(15), nominal shear in the bolt shank section")
}

/// 🔩️ Selects the tabulated fatigue category for the member or joint and checks the designated stress section.
pub fn plain_and_fastened_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("detail81_basis")? != "confirmed" {
        return Err("Confirm the detail's material, execution and applicable gross, net, tensile or shank stress section".into());
    }
    for key in ["fy", "gamma_ff", "gamma_mf"] {
        positive(q, key)?;
    }
    let detail = q.t("detail81_detail")?;
    let (category, reference) = match detail {
        "rolled-plate" | "rolled-profile" | "seamless" | "cut-machined" | "cut-dressed" => {
            let weathering = match q.t("detail81_steel")? {
                "ordinary" => false,
                "weathering" => true,
                _ => return Err("Declare whether weathering steel is used for the unfastened detail".into()),
            };
            let category = match (detail, weathering) {
                ("cut-machined", false) | ("rolled-plate" | "rolled-profile" | "seamless", true) => 140.0,
                ("cut-dressed", false) | ("cut-machined", true) => 125.0,
                ("cut-dressed", true) => 112.0,
                _ => 160.0,
            };
            let reference = match detail {
                "rolled-plate" => "Table 8.1(1), as-rolled plate or flats",
                "rolled-profile" => "Table 8.1(2), rolled profile",
                "seamless" => "Table 8.1(3), seamless circular or rectangular section",
                "cut-machined" => "Table 8.1(4), machine-cut plate with dressed edge",
                _ => "Table 8.1(5), qualified cut plate",
            };
            (category, reference)
        }
        "rolled-shear" | "seamless-shear" => return member_shear(q, detail),
        "double-preloaded" | "double-fitted" | "single-preloaded" | "holes" | "single-fitted" | "clearance-no-preload" => {
            clearance(q)?;
            match detail {
                "double-preloaded" => (112.0, "Table 8.1(8), gross section of double-covered preloaded joint"),
                "double-fitted" => (90.0, "Table 8.1(9), net section of double-covered fitted or non-preloaded injection joint"),
                "single-preloaded" => (90.0, "Table 8.1(10), gross section of one-sided preloaded joint"),
                "holes" => (90.0, "Table 8.1(11), net section with holes under bending and axial force"),
                "single-fitted" => (80.0, "Table 8.1(12), net section of one-sided fitted or non-preloaded injection joint"),
                _ => {
                    if q.t("detail81_reversal")? != "none" {
                        return Err("Detail 13 requires no load reversal for non-preloaded bolts in clearance holes".into());
                    }
                    (50.0, "Table 8.1(13), net section with non-preloaded clearance-hole bolts")
                }
            }
        }
        "bolt-tension" => {
            let d = positive(q, "detail81_diameter")?;
            (50.0 * (30.0 / d.max(30.0)).powf(0.25), "Table 8.1(14), bolt tensile stress area with prying and other bending effects")
        }
        "bolt-shear" => return bolt_shear(q),
        _ => return Err("Select a detail listed in Table 8.1".into()),
    };
    nominal_direct_checks(q, range(q, "detail81_range")?, range(q, "detail81_frequent")?, category, reference.into())
}
