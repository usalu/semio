//! 🌬️ EN 1991-1-4 catalogue entries E14.01 … E14.16: basic wind, profiles, orography, turbulence, peak pressure, return period factor, wall and roof coefficients, internal pressure, free-standing signs, forces, dynamic response, bridges, vortex shedding, dynamic parameters and friction.

use super::*;
use part_1_4::TerrainCategory;

fn terrain(c: &Case) -> Result<TerrainCategory, String> { let t = c.n("terrain")?; TerrainCategory::from_u8(t as u8).ok_or_else(|| format!("unknown terrain category {t}; admissible: 0, 1, 2, 3, 4")) }
fn rho(c: &Case) -> f64 { c.o("rho", 1.25) }

fn e14_01(c: &Case) -> R {
    let (vb0, name) = if c.de() {
        let z = c.n("zone")?;
        (na_de::WindZone::from_u8(z as u8).ok_or_else(|| format!("unknown wind zone {z}; admissible: 1, 2, 3, 4"))?.v_b0_m_s(), "NA-DE wind zone")
    } else { (c.pos("v_b0")?, "EN v_b,0") };
    let vb = c.o("c_dir", 1.0) * c.o("c_season", 1.0) * vb0;
    let qb = part_1_4::basic_velocity_pressure_pa(rho(c), vb);
    Ok(vec![
        Outcome::info("v_b", "Basic wind velocity v_b", "Basisgeschwindigkeit v_b", QuantityKind::Dimensionless, vb).steps(format!("{name}: v_b = c_dir·c_season·v_b,0 = {vb:.3} m/s.")),
        Outcome::info("q_b", "Basic velocity pressure q_b", "Basisgeschwindigkeitsdruck q_b", QuantityKind::Pressure, qb).steps(format!("q_b = ½ρv_b² = {:.4} kN/m².", qb / 1000.0)),
    ])
}

fn simplified_qp_pa(zone: u8, height: f64, coast: bool) -> Option<f64> {
    let band = if height <= 10.0 { 0 } else if height <= 18.0 { 1 } else if height <= 25.0 { 2 } else { return None };
    let row = if coast { match zone { 2 => [0.85, 1.00, 1.10], 3 => [1.05, 1.20, 1.30], 4 => [1.25, 1.40, 1.55], _ => return None } } else { match zone { 1 => [0.50, 0.65, 0.75], 2 => [0.65, 0.80, 0.90], 3 => [0.80, 0.95, 1.10], 4 => [0.95, 1.15, 1.30], _ => return None } };
    Some(row[band] * 1000.0)
}

fn e14_02(c: &Case) -> R {
    let z = c.pos("z")?;
    let zone = c.n("zone")? as u8;
    let w = na_de::WindZone::from_u8(zone).ok_or_else(|| format!("unknown wind zone {zone}; admissible: 1, 2, 3, 4"))?;
    let coast = c.o("coast", 0.0) > 0.5;
    let qb0 = part_1_4::basic_velocity_pressure_pa(1.25, w.v_b0_m_s());
    let qp = part_1_4::na_profile_pa(qb0, z, coast);
    let mut out = vec![Outcome::info("q_p", "NA-DE profile q_p(z)", "NA-DE Profil q_p(z)", QuantityKind::Pressure, qp).steps(format!("{} profile at z = {z:.1} m: q_p = {:.4} kN/m².", if coast { "coast" } else { "inland" }, qp / 1000.0)).steps_de(format!("{} Profil bei z = {z:.1} m: q_p = {:.4} kN/m².", if coast { "coast" } else { "inland" }, qp / 1000.0)).basis(&["ndp.en1991.e14.na-profile"])];
    if let Some(s) = simplified_qp_pa(zone, z, coast) { out.push(Outcome::info("simplified", "Simplified q_p", "Vereinfachter q_p", QuantityKind::Pressure, s).steps(format!("Simplified table value {:.3} kN/m²; profile/simplified = {:.3}.", s / 1000.0, qp / s)).steps_de(format!("Vereinfachter Tabellenwert {:.3} kN/m²; Profil/vereinfacht = {:.3}.", s / 1000.0, qp / s)).basis(&["ndp.en1991.e14.na-profile"])); }
    Ok(out)
}

fn e14_03(c: &Case) -> R {
    let (phi, s) = (c.n("phi")?, c.n("s")?);
    let co = part_1_4::orography_factor(phi, s);
    Ok(vec![Outcome::info("c_o", "Orography factor c_o", "Orographiebeiwert c_o", QuantityKind::Dimensionless, co).steps(format!("Φ = {phi}, s = {s}: c_o = {co:.4}."))])
}

fn e14_04(c: &Case) -> R {
    let (z, t, co) = (c.pos("z")?, terrain(c)?, c.o("c_o", 1.0));
    let iv = part_1_4::turbulence_intensity(z, t, co);
    Ok(vec![Outcome::info("I_v", "Turbulence intensity I_v", "Turbulenzintensität I_v", QuantityKind::Dimensionless, iv).steps(format!("I_v(z) = k_I/(c_o ln(z/z₀)) = {iv:.5}."))])
}

fn e14_05(c: &Case) -> R {
    let (z, t, vb, co) = (c.pos("z")?, terrain(c)?, c.pos("v_b")?, c.o("c_o", 1.0));
    let (kr, cr) = (part_1_4::k_r(t), part_1_4::c_r(z, t));
    let vm = cr * co * vb;
    let iv = part_1_4::turbulence_intensity(z, t, co);
    let qp = part_1_4::en_peak_pressure_pa(z, t, vb, rho(c), co);
    Ok(vec![
        Outcome::info("k_r", "Terrain factor k_r", "Geländefaktor k_r", QuantityKind::Dimensionless, kr).steps(format!("k_r = 0.19(z₀/0.05)^0.07 = {kr:.6}.")),
        Outcome::info("c_r", "Roughness factor c_r(z)", "Rauigkeitsbeiwert c_r(z)", QuantityKind::Dimensionless, cr).steps(format!("c_r = k_r ln(z/z₀) = {cr:.6}.")),
        Outcome::info("v_m", "Mean wind velocity v_m(z)", "Mittlere Windgeschwindigkeit v_m(z)", QuantityKind::Dimensionless, vm).steps(format!("v_m = c_r c_o v_b = {vm:.4} m/s.")),
        Outcome::info("I_v", "Turbulence intensity I_v", "Turbulenzintensität I_v", QuantityKind::Dimensionless, iv).steps(format!("I_v = {iv:.5}.")),
        Outcome::info("q_p", "Peak velocity pressure q_p(z)", "Böengeschwindigkeitsdruck q_p(z)", QuantityKind::Pressure, qp).steps(format!("q_p = (1 + 7I_v)·½ρv_m² = {:.4} kN/m².", qp / 1000.0)),
    ])
}

fn e14_06(c: &Case) -> R {
    let p = match c.opt("p") { Some(p) => p, None => 1.0 / c.pos("n_years")? };
    if !(p > 0.0 && p < 1.0) { return Err("annual exceedance probability must lie within (0, 1)".into()); }
    let f = part_1_6::c_prob(p);
    Ok(vec![Outcome::info("c_prob", "Probability factor c_prob", "Wahrscheinlichkeitsbeiwert c_prob", QuantityKind::Dimensionless, f).steps(format!("p = {p}: c_prob = ((1 − K ln(−ln(1−p)))/(1 − K ln(−ln 0.98)))^n = {f:.6}.")).basis(&["ndp.en1991.e16.returns"])])
}

fn e14_07(c: &Case) -> R {
    let (b, h, d) = (c.pos("b")?, c.pos("h")?, c.pos("d")?);
    let e = part_1_4::reference_e(b, h);
    let (a, bz, cz) = part_1_4::wall_zone_widths(e, d);
    let ze = part_1_4::reference_height_ze(h, h, b);
    Ok(vec![
        Outcome::info("e", "Reference dimension e = min(b, 2h)", "Bezugsmaß e = min(b, 2h)", QuantityKind::Length, e).steps(format!("e = {e:.3} m.")),
        Outcome::info("A", "Zone A width", "Breite Zone A", QuantityKind::Length, a).steps(format!("A = {a:.3} m.")),
        Outcome::info("B", "Zone B width", "Breite Zone B", QuantityKind::Length, bz).steps(format!("B = {bz:.3} m.")),
        Outcome::info("C", "Zone C width", "Breite Zone C", QuantityKind::Length, cz).steps(format!("C = {cz:.3} m.")),
        Outcome::info("z_e", "Reference height of the top strip", "Bezugshöhe des obersten Streifens", QuantityKind::Length, ze).steps(format!("z_e = {ze:.3} m.")),
    ])
}

fn e14_08(c: &Case) -> R {
    let zone = c.s("zone")?;
    let roof_type = c.text.get("roof_type").map(String::as_str);
    let cands = part_1_4::face_cpe_candidates(zone, c.o("h_over_d", 1.0), roof_type, c.o("pitch", 0.0), c.o("hp_over_h", 0.0)).ok_or_else(|| format!("no c_pe table for zone {zone} and roof type {}", roof_type.unwrap_or("none")))?;
    let area = c.o("area", 10.0);
    let mut out = Vec::new();
    let basis: &[&'static str] = if zone.len() == 1 && "ABCDE".contains(&zone.to_ascii_uppercase()) { &[] } else if roof_type.map_or(false, |k| k.contains("flat")) { &["ndp.en1991.e14.cpe-flat"] } else { &["ndp.en1991.e14.cpe-pitched"] };
    for (i, (c10, c1)) in cands.iter().enumerate() {
        out.push(Outcome::info(&format!("c_pe10_{i}"), "c_pe,10", "c_pe,10", QuantityKind::Dimensionless, *c10).steps(format!("Zone {zone}, load case {i}: c_pe,10 = {c10:.4}.")).steps_de(format!("Zone {zone}, Lastfall {i}: c_pe,10 = {c10:.4}.")).basis(basis));
        out.push(Outcome::info(&format!("c_pe1_{i}"), "c_pe,1", "c_pe,1", QuantityKind::Dimensionless, *c1).steps(format!("Zone {zone}, load case {i}: c_pe,1 = {c1:.4}.")).steps_de(format!("Zone {zone}, Lastfall {i}: c_pe,1 = {c1:.4}.")));
        let a = part_1_4::area_cpe(*c1, *c10, area);
        out.push(Outcome::info(&format!("c_pe_area_{i}"), "c_pe(A)", "c_pe(A)", QuantityKind::Dimensionless, a).steps(format!("A = {area:.2} m²: c_pe = c_pe,1 − (c_pe,1 − c_pe,10)·log₁₀A = {a:.4}.")));
    }
    if let (Some(e), Some(b), Some(d)) = (c.opt("e"), c.opt("b"), c.opt("d")) {
        let (f, g, h, i) = part_1_4::flat_roof_zone_areas(e, b, d);
        for (k, v) in [("area_f", f), ("area_g", g), ("area_h", h), ("area_i", i)] { out.push(Outcome::info(k, "Flat roof zone area", "Flachdach Zonenfläche", QuantityKind::Area, v).steps(format!("{k} = {v:.3} m² (F per corner).")).steps_de(format!("{k} = {v:.3} m² (F je Ecke)."))); }
    }
    Ok(out)
}

fn e14_09(c: &Case) -> R {
    let (qp, cpe, cs, cd) = (c.pos("q_p")?, c.n("c_pe")?, c.o("c_s", 1.0), c.o("c_d", 1.0));
    let mut out = Vec::new();
    for (i, cpi) in part_1_4::c_pi_cases().iter().enumerate() {
        let w = part_1_4::wind_pressure_pa(qp, cpe, *cpi, cs, cd);
        out.push(Outcome::info(&format!("w_{i}"), &format!("Net pressure for c_pi = {cpi:+.1}"), &format!("Nettodruck für c_pi = {cpi:+.1}"), QuantityKind::Pressure, w).steps(format!("w = q_p (c_pe − c_pi) c_s c_d = {:.4} kN/m².", w / 1000.0)));
    }
    if let Some((ratio, cpi)) = c.opt("dominant_ratio").and_then(|ratio| part_1_4::dominant_opening_cpi(cpe, ratio).map(|cpi| (ratio, cpi))) {
        out.push(Outcome::info("c_pi_dominant", "Internal pressure of a dominant opening", "Innendruck bei dominanter Öffnung", QuantityKind::Dimensionless, cpi).steps(format!("Opening ratio {ratio}: c_pi = {cpi:.4}.")).steps_de(format!("Öffnungsverhältnis {ratio}: c_pi = {cpi:.4}.")));
    }
    out.push(Outcome::info("w_governing", "Governing net pressure magnitude", "Maßgebender Nettodruck", QuantityKind::Pressure, part_1_4::governing_wind_pressure_pa(qp, cpe, c.o("c_pi", 0.2), cs, cd)).steps("Maximum of |w| over c_pi = +0.2, −0.3 and the given c_pi.").steps_de("Maximum von |w| über c_pi = +0,2, −0,3 und das gegebene c_pi."));
    Ok(out)
}

fn e14_10(c: &Case) -> R {
    let (qp, a_ref) = (c.pos("q_p")?, c.pos("a_ref")?);
    let cf = c.o("c_f", semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-4.cf.signboard-default"));
    let f = c.o("c_s", 1.0) * c.o("c_d", 1.0) * cf * qp * a_ref;
    let ze = c.o("z_g", 0.0) + c.pos("h")? / 2.0;
    Ok(vec![
        Outcome::info("F_w", "Signboard force F_w", "Anzeigetafel Windkraft F_w", QuantityKind::Force, f).steps(format!("F_w = c_s c_d c_f q_p A_ref = {:.3} kN (c_f = {cf}).", f / 1000.0)),
        Outcome::info("z_e", "Reference height z_e = z_g + h/2", "Bezugshöhe z_e = z_g + h/2", QuantityKind::Length, ze).steps(format!("z_e = {ze:.3} m.")),
    ])
}

fn e14_11(c: &Case) -> R {
    let (qp, a_ref) = (c.pos("q_p")?, c.pos("a_ref")?);
    let cf = match c.opt("c_f") {
        Some(v) => v,
        None => {
            let (cf0, b) = (c.pos("c_f0")?, c.pos("b")?);
            let psi_r = (1.0 - 1.9 * c.o("r", 0.0) / b).max(0.5);
            cf0 * psi_r * c.o("psi_lambda", 1.0)
        }
    };
    let f = c.o("c_s", 1.0) * c.o("c_d", 1.0) * cf * qp * a_ref;
    let mut out = vec![Outcome::info("F_w", "Wind force F_w", "Windkraft F_w", QuantityKind::Force, f).steps(format!("F_w = c_s c_d c_f q_p A_ref = {:.3} kN with c_f = {cf:.4}.", f / 1000.0)).steps_de(format!("F_w = c_s c_d c_f q_p A_ref = {:.3} kN mit c_f = {cf:.4}.", f / 1000.0))];
    if let (Some(l), Some(b)) = (c.opt("length"), c.opt("b")) {
        let lambda = if l >= 50.0 { (1.4 * l / b).min(70.0) } else if l <= 15.0 { (2.0 * l / b).min(70.0) } else { numeric::table(&[(15.0, (2.0 * 15.0 / b).min(70.0)), (50.0, (1.4 * 50.0 / b).min(70.0))], l) };
        out.push(Outcome::info("lambda", "Effective slenderness λ", "Wirksame Schlankheit λ", QuantityKind::Dimensionless, lambda).steps(format!("λ = {lambda:.3} for l = {l} m, b = {b} m.")).steps_de(format!("λ = {lambda:.3} für l = {l} m, b = {b} m.")));
    }
    Ok(out)
}

fn e14_12(c: &Case) -> R {
    let (h, b, t, vb) = (c.pos("h")?, c.pos("b")?, terrain(c)?, c.pos("v_b")?);
    let delta = c.pos("delta")?;
    let n1 = c.opt("n_1").unwrap_or(46.0 / h);
    let r = part_1_4::structural_factor(h, b, t, vb, delta, n1);
    let rows = [("z_s", "Reference height z_s", "Bezugshöhe z_s", QuantityKind::Length), ("L", "Turbulent length scale L(z_s)", "Turbulenzlänge L(z_s)", QuantityKind::Length), ("B2", "Background factor B²", "Hintergrundfaktor B²", QuantityKind::Dimensionless), ("n_1", "Natural frequency n₁", "Eigenfrequenz n₁", QuantityKind::Dimensionless), ("S_L", "Spectral density S_L", "Spektraldichte S_L", QuantityKind::Dimensionless), ("R2", "Resonance factor R²", "Resonanzfaktor R²", QuantityKind::Dimensionless), ("nu", "Up-crossing frequency ν", "Nulldurchgangsfrequenz ν", QuantityKind::Dimensionless), ("k_p", "Peak factor k_p", "Spitzenfaktor k_p", QuantityKind::Dimensionless), ("cscd", "Structural factor c_s c_d", "Strukturbeiwert c_s c_d", QuantityKind::Dimensionless)];
    Ok(rows.iter().zip(r.iter()).map(|((k, en, de, kind), v)| Outcome::info(k, en, de, *kind, *v).steps(format!("{k} = {v:.6}.")).basis(&["ndp.en1991.e14.annex-b"])).collect())
}

fn e14_13(c: &Case) -> R {
    let (vb, ce, cf, a) = (c.pos("v_b")?, c.pos("c_e")?, c.pos("c_fx")?, c.pos("a_ref")?);
    let f = 0.5 * rho(c) * vb * vb * ce * cf * a;
    Ok(vec![Outcome::info("F_w", "Bridge wind force F_W", "Brücken-Windkraft F_W", QuantityKind::Force, f).steps(format!("F_W = ½ρv_b² C A_ref = {:.3} kN with C = {:.3}.", f / 1000.0, ce * cf)).steps_de(format!("F_W = ½ρv_b² C A_ref = {:.3} kN mit C = {:.3}.", f / 1000.0, ce * cf)).basis(&["ndp.en1991.e14.bridge"])])
}

fn e14_14(c: &Case) -> R {
    let (b, n) = (c.pos("b")?, c.pos("n")?);
    let st = c.o("st", 0.18);
    let vcrit = part_1_4::vortex_critical_velocity(b, n, st);
    let mut out = vec![Outcome::info("v_crit", "Critical velocity v_crit", "Kritische Geschwindigkeit v_crit", QuantityKind::Dimensionless, vcrit).steps(format!("v_crit = b n/St = {vcrit:.4} m/s.")).basis(&["ndp.en1991.e14.annex-e"])];
    if let (Some(ds), Some(me)) = (c.opt("delta_s"), c.opt("m_e")) {
        let sc = 2.0 * ds * me / (rho(c) * b * b);
        out.push(Outcome::info("Sc", "Scruton number Sc", "Scruton-Zahl Sc", QuantityKind::Dimensionless, sc).steps(format!("Sc = 2δ_s m_e/(ρ b²) = {sc:.3}.")).basis(&["ndp.en1991.e14.annex-e"]));
    }
    if let Some(vm) = c.opt("v_m") { out.push(Outcome::warn("relevance", "Vortex shedding relevant when v_crit ≤ 1.25 v_m", "Wirbelerregung relevant bei v_crit ≤ 1,25 v_m", QuantityKind::Dimensionless, 1.25 * vm / vcrit, 1.0).steps(format!("1.25·v_m = {:.3} m/s against v_crit = {vcrit:.3} m/s.", 1.25 * vm)).steps_de(format!("1,25·v_m = {:.3} m/s gegenüber v_crit = {vcrit:.3} m/s.", 1.25 * vm)).basis(&["ndp.en1991.e14.annex-e"])); }
    Ok(out)
}

fn e14_15(c: &Case) -> R {
    let h = c.pos("h")?;
    let n1 = 46.0 / h;
    let zeta = match c.s("mode")? { mode @ ("framed" | "core" | "cantilever" | "lattice") => semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1991.1-4.mode-exponent.{mode}")), other => return Err(format!("unknown mode {other}; admissible: framed, core, cantilever, lattice")) };
    let delta_s = match c.s("structure")? { structure @ ("concrete" | "steel" | "mixed") => semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1991.1-4.delta-s.{structure}")), other => return Err(format!("unknown structure {other}; admissible: concrete, steel, mixed")) };
    let phi = (c.o("z", h) / h).powf(zeta);
    Ok(vec![
        Outcome::info("n_1", "Natural frequency n₁ = 46/h", "Eigenfrequenz n₁ = 46/h", QuantityKind::Dimensionless, n1).steps(format!("n₁ = {n1:.4} Hz.")).basis(&["ndp.en1991.e14.annex-b"]),
        Outcome::info("phi", "Mode shape Φ₁(z)", "Eigenform Φ₁(z)", QuantityKind::Dimensionless, phi).steps(format!("Φ₁ = (z/h)^{zeta} = {phi:.4}.")),
        Outcome::info("delta_s", "Structural damping δ_s", "Strukturdämpfung δ_s", QuantityKind::Dimensionless, delta_s).steps(format!("δ_s = {delta_s}.")).basis(&["ndp.en1991.e14.annex-b"]),
    ])
}

fn e14_16(c: &Case) -> R {
    let qp = c.pos("q_p")?;
    let mut out = Vec::new();
    if let Some(b) = c.opt("b") {
        let v = (2.0 * qp / rho(c)).sqrt();
        let re = b * v / 15e-6;
        out.push(Outcome::info("Re", "Reynolds number Re", "Reynolds-Zahl Re", QuantityKind::Dimensionless, re).steps(format!("v = √(2q_p/ρ) = {v:.3} m/s, Re = b v/ν = {re:.4e}.")));
    }
    if let (Some(cfr), Some(a)) = (c.opt("c_fr"), c.opt("a_fr")) {
        let f = part_1_4::friction_force_n(cfr, qp, a);
        out.push(Outcome::info("F_fr", "Frictional drag F_fr", "Reibungskraft F_fr", QuantityKind::Force, f).steps(format!("F_fr = c_fr q_p A_fr = {:.3} kN.", f / 1000.0)));
    }
    if out.is_empty() { return Err("missing input b or c_fr and a_fr".into()); }
    Ok(out)
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "E14.01", standard: "EN 1991-1-4", clause: "4.2", en: "Basic wind velocity and pressure", de: "Basiswindgeschwindigkeit und -druck", inputs: &["zone", "v_b0", "c_dir", "c_season", "rho"], eval: e14_01 },
    Entry { id: "E14.02", standard: "EN 1991-1-4", clause: "4.3.2 / NA.B", en: "NA-DE wind profiles", de: "NA-DE Windprofile", inputs: &["zone", "z", "coast"], eval: e14_02 },
    Entry { id: "E14.03", standard: "EN 1991-1-4", clause: "4.3.3 / A.3", en: "Orography", de: "Orographie", inputs: &["phi", "s"], eval: e14_03 },
    Entry { id: "E14.04", standard: "EN 1991-1-4", clause: "4.4", en: "Turbulence intensity", de: "Turbulenzintensität", inputs: &["z", "terrain", "c_o"], eval: e14_04 },
    Entry { id: "E14.05", standard: "EN 1991-1-4", clause: "4.5", en: "Peak velocity pressure", de: "Böengeschwindigkeitsdruck", inputs: &["z", "terrain", "v_b", "rho", "c_o"], eval: e14_05 },
    Entry { id: "E14.06", standard: "EN 1991-1-4", clause: "4.2(2)", en: "Probability factor", de: "Wahrscheinlichkeitsbeiwert", inputs: &["p", "n_years"], eval: e14_06 },
    Entry { id: "E14.07", standard: "EN 1991-1-4", clause: "7.2.2", en: "Vertical wall zones", de: "Zonen der Außenwände", inputs: &["b", "h", "d"], eval: e14_07 },
    Entry { id: "E14.08", standard: "EN 1991-1-4", clause: "7.2", en: "External pressure coefficients", de: "Außendruckbeiwerte", inputs: &["zone", "roof_type", "pitch", "h_over_d", "hp_over_h", "area", "e", "b", "d"], eval: e14_08 },
    Entry { id: "E14.09", standard: "EN 1991-1-4", clause: "7.2.9", en: "Internal pressure and net pressure", de: "Innendruck und Nettodruck", inputs: &["q_p", "c_pe", "c_s", "c_d", "dominant_ratio", "c_pi"], eval: e14_09 },
    Entry { id: "E14.10", standard: "EN 1991-1-4", clause: "7.4", en: "Free-standing signboards", de: "Freistehende Anzeigetafeln", inputs: &["q_p", "a_ref", "c_f", "h", "z_g", "c_s", "c_d"], eval: e14_10 },
    Entry { id: "E14.11", standard: "EN 1991-1-4", clause: "7.6–7.8", en: "Force coefficients and wind force", de: "Kraftbeiwerte und Windkraft", inputs: &["q_p", "a_ref", "c_f", "c_f0", "b", "r", "psi_lambda", "length", "c_s", "c_d"], eval: e14_11 },
    Entry { id: "E14.12", standard: "EN 1991-1-4", clause: "B", en: "Structural factor c_s c_d", de: "Strukturbeiwert c_s c_d", inputs: &["h", "b", "terrain", "v_b", "delta", "n_1"], eval: e14_12 },
    Entry { id: "E14.13", standard: "EN 1991-1-4", clause: "8", en: "Wind on bridges", de: "Windwirkungen auf Brücken", inputs: &["v_b", "c_e", "c_fx", "a_ref", "rho"], eval: e14_13 },
    Entry { id: "E14.14", standard: "EN 1991-1-4", clause: "E", en: "Vortex shedding", de: "Wirbelerregung", inputs: &["b", "n", "st", "delta_s", "m_e", "v_m", "rho"], eval: e14_14 },
    Entry { id: "E14.15", standard: "EN 1991-1-4", clause: "F", en: "Dynamic characteristics", de: "Dynamische Kenngrößen", inputs: &["h", "mode", "structure", "z"], eval: e14_15 },
    Entry { id: "E14.16", standard: "EN 1991-1-4", clause: "7.5", en: "Friction and Reynolds number", de: "Reibung und Reynolds-Zahl", inputs: &["q_p", "b", "c_fr", "a_fr", "rho"], eval: e14_16 },
];
