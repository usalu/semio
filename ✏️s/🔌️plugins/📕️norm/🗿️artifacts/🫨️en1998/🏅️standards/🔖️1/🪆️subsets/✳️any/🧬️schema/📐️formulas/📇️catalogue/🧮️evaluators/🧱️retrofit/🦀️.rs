//! 🧮️ Annex A strengthening cases for the existing concrete member.

use super::{Inputs, Outcome, Row};
use crate::retrofit::{concrete_jacket, SpliceClamp, SteelJacket};
use semio_s_plugin_norm_compliance::{CheckStatus, QuantityKind};

pub fn concrete(q: &Inputs<'_>) -> Outcome {
    if q.t("retrofit_basis")? != "confirmed" {
        return Err("Zusammengesetzten Querschnitt und Materialbasis nach A.4.2.2 bestätigen. Confirm the composite section and material basis under A.4.2.2.".into());
    }
    let [shear, moment, yield_rotation, ultimate_rotation] = concrete_jacket(q.n("retrofit_shear")?, q.n("retrofit_moment")?, q.n("retrofit_yield_rotation")?, q.n("retrofit_ultimate_rotation")?)
        .ok_or("Enter positive monolithic capacities with the ultimate rotation at least the yield rotation.")?;
    let info = |key, en, de, kind, value, equation| Row::new(key, en, de, kind, value, 0.0, equation).status(CheckStatus::Info);
    Ok(vec![
        info("shear", "Adjusted shear resistance before the overall factor", "Angepasster Schubwiderstand vor dem Gesamtbeiwert", QuantityKind::Force, shear, format!("VR* = 0.90 VR = {shear:.3} N")),
        info("moment", "Adjusted yield moment", "Angepasstes Fließmoment", QuantityKind::Moment, moment, format!("My* = My = {moment:.3} N·m")),
        info("yield", "Adjusted yield chord rotation", "Angepasste Sehnenrotation beim Fließen", QuantityKind::Dimensionless, yield_rotation, format!("θy* = 1.05 θy = {yield_rotation:.6} rad")),
        info("ultimate", "Adjusted ultimate chord rotation before the overall factor", "Angepasste Grenzsehnenrotation vor dem Gesamtbeiwert", QuantityKind::Dimensionless, ultimate_rotation, format!("θu* = θu = {ultimate_rotation:.6} rad")),
    ])
}

pub fn steel(q: &Inputs<'_>) -> Outcome {
    if q.t("retrofit_basis")? != "confirmed" {
        return Err("Elastisches Verhalten des Stahlmantels bestätigen. Confirm that the steel jacket meets the elastic basis of A.4.3.2.".into());
    }
    let contribution = SteelJacket {
        depth_m: q.n("jacket_depth")?,
        thickness_m: q.n("jacket_thickness")?,
        width_spacing_ratio: q.n("jacket_width_ratio")?,
        strut_deg: q.n("jacket_strut_angle")?,
        strap_deg: q.n("jacket_strap_angle")?,
        design_yield_pa: q.n("jacket_design_strength")?,
    }.shear_contribution_n().ok_or("Enter valid dimensions, angles and design steel strength for the jacket.")?;
    Ok(vec![Row::new("shear-addition", "Additive shear resistance of the jacket", "Zusätzlicher Schubwiderstand des Stahlmantels", QuantityKind::Force,
        contribution, 0.0, format!("Vj = 0.5 h (2 tj b/s) fy,j,d (cot θ + cot β) sin β = {contribution:.3} N")).status(CheckStatus::Info)])
}

pub fn splice(q: &Inputs<'_>) -> Outcome {
    let location = q.t("jacket_splice_location")?;
    if location == "unconfirmed" {
        return Err("Stoßlage und Schraubenanordnung erfassen. Record the splice location and bolt layout.".into());
    }
    let clamp = SpliceClamp {
        splice_m: q.n("jacket_splice_length")?,
        jacket_m: q.n("jacket_length")?,
        near_rows: q.n("jacket_bolts_near")?,
        far_rows: q.n("jacket_bolts_far")?,
        at_column_base: location != "away",
        base_pattern_confirmed: location == "base-confirmed",
    };
    let length = clamp.jacket_m;
    let (minimum, passed) = clamp.evaluate().ok_or("Enter positive lengths and nonnegative whole bolt row counts for each face.")?;
    Ok(vec![Row::new("clamping", "Jacket length and clamping bolt layout", "Mantellänge und Anordnung der Klemmschrauben", QuantityKind::Length,
        minimum, length, format!("Lj,min = 1.50 lsplice = {minimum:.6} m. Lj = {length:.6} m.")).status(if passed { CheckStatus::Pass } else { CheckStatus::Fail })])
}

pub fn frp_shear(q: &Inputs<'_>) -> Outcome {
    use crate::retrofit::frp_shear::{circular_shear_n, Arrangement, FrpRectangular, Layout};
    if q.t("frp_basis")? != "confirmed" {
        return Err("FRP-Detail und anwendbare Betontragwiderstände bestätigen. Confirm the FRP detail and the applicable concrete shear resistances.".into());
    }
    let t = q.n("frp_thickness")?;
    let modulus = q.n("frp_modulus")?;
    let configuration = q.t("frp_configuration")?;
    let mut rows = Vec::new();
    let contribution = if configuration == "circular" {
        circular_shear_n(q.n("frp_diameter")?, t, modulus).ok_or("Enter positive circular diameter, thickness and modulus.")?
    } else {
        let arrangement = match configuration {
            "closed" => Arrangement::Closed { corner_radius_mm: q.n("frp_radius")?, web_width_mm: q.n("frp_web_width")?, ultimate_mpa: q.n("frp_ultimate")? },
            "u" => Arrangement::OpenU,
            "side" => Arrangement::Sides,
            _ => return Err("Select the FRP configuration.".into()),
        };
        let layout = match q.t("frp_layout")? {
            "strips" => Layout::Strips { width_mm: q.n("frp_strip_width")?, spacing_mm: q.n("frp_strip_spacing")? },
            "sheet" => Layout::Sheet { web_height_mm: q.n("frp_web_height")? },
            _ => return Err("Select strips or a continuous sheet.".into()),
        };
        let rect = FrpRectangular {
            depth_mm: q.n("frp_depth")?, thickness_mm: t, strut_deg: q.n("frp_strut_angle")?, fibre_deg: q.n("frp_fibre_angle")?,
            modulus_mpa: modulus, tensile_mpa: q.n("frp_concrete_tensile")?, debonding_factor: q.n("frp_debonding_factor")?,
            arrangement, layout,
        }.evaluate().ok_or("Enter valid FRP dimensions, bond parameters and material data.")?;
        rows.push(Row::new("bond", "Effective bond length", "Wirksame Verbundlänge", QuantityKind::Length, rect.bond_length_mm / 1000.0, 0.0,
            format!("Le = {:.4} mm", rect.bond_length_mm)).status(CheckStatus::Info));
        rows.push(Row::new("effective", "Effective design debonding strength", "Wirksame Bemessungs-Ablösefestigkeit", QuantityKind::Stress, rect.effective_mpa * 1e6, 0.0,
            format!("ffdd,e = {:.4} MPa", rect.effective_mpa)).status(CheckStatus::Info));
        rect.shear_n
    };
    let existing = q.n("frp_existing")?;
    let web = q.n("frp_web_cap")?;
    let demand = q.n("frp_demand")?;
    let member = q.t("frp_member")?;
    if configuration == "circular" && member != "column" {
        return Err("The circular-section branch applies to columns.".into());
    }
    let needs_cyclic = match member {
        "wall" => true,
        "column" => {
            let ratio = q.n("frp_shear_span")?;
            if !ratio.is_finite() || ratio <= 0.0 {
                return Err("Enter the positive shear span ratio of the column.".into());
            }
            ratio <= 2.0
        }
        "beam" => false,
        _ => return Err("Identify the concrete member type.".into()),
    };
    let cyclic = if needs_cyclic { q.n("frp_cyclic_cap")? } else { web };
    if ![existing, demand].into_iter().all(|x| x.is_finite() && x >= 0.0) || ![web, cyclic].into_iter().all(|x| x.is_finite() && x > 0.0) {
        return Err("Enter finite shear demand and resistance values and positive web crushing limits.".into());
    }
    let capacity = (existing + contribution).min(web).min(cyclic);
    rows.push(Row::new("frp", "Shear contribution of FRP", "Querkraftanteil der Faserverbundverstärkung", QuantityKind::Force, contribution, 0.0,
        format!("VRd,f = {contribution:.3} N")).status(CheckStatus::Info));
    rows.push(Row::new("total-shear", "Shear demand against the capped resistance", "Bemessungsquerkraft gegen den begrenzten Widerstand", QuantityKind::Force, demand, capacity,
        format!("min({existing:.3} + {contribution:.3}, {web:.3}, {cyclic:.3}) = {capacity:.3} N")));
    Ok(rows)
}
