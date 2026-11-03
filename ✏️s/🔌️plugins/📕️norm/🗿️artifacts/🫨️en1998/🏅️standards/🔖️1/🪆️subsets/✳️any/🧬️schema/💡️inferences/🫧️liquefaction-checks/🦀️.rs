//! 🫧 Consequences of liquefaction and the degraded soil stiffness (EN 1998-5 4.1.4, 4.2.2 and 5.4.2): the excess pore pressure ratio of every liquefiable layer, the screening of lateral spreading from the slopes on the layers, the buckling and bending of a pile over the liquefied length and the shear modulus of the layers degraded by Table 4.1. Constants are unverified registry rows. See [`super`].

use super::geotechnical_details::{liquefaction_band, liquefaction_layer, soil_profile};
use super::*;
use semio_s_artifact_norm_en1998_schema::liquefaction_hazard::{self as hazard, LiquefiedLayer, Spreading};
use semio_s_artifact_norm_en1998_schema::part_5::SoilProfile;
use semio_s_artifact_norm_en1998_schema::soil_structure;
use semio_s_plugin_norm_compliance::{ndp_number, CheckStatus};

type Foundation = semio_s_artifact_norm_en1998_model::En1998Foundation;

fn q_moment(v: f64) -> Quantity { Quantity::new(QuantityKind::Moment, v) }

fn q_pressure(v: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, v) }

/// 🫧 The liquefied layers of a foundation with their total thickness: from the soil profile where its layers are complete, else the single layer of the foundation over its depth band.
fn liquefied(annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, a_g_s: f64, required: f64) -> Vec<LiquefiedLayer> {
    if let Some((profile, _)) = soil_profile(annex, document, f, a_g_s).filter(|(p, _)| f.liq_magnitude > 0.0 && p.layers.iter().all(|l| l.unit_weight_n_m3 > 0.0 && l.thickness_m > 0.0)) {
        return hazard::liquefied_layers(&profile, required);
    }
    let Some(layer) = liquefaction_layer(annex, f, a_g_s) else { return Vec::new() };
    let (top, bottom) = liquefaction_band(f);
    let factor = (0..=8).map(|i| layer.factor_annex_b(top + (bottom - top) * f64::from(i) / 8.0)).fold(f64::INFINITY, f64::min);
    if factor.is_finite() && factor < required { vec![LiquefiedLayer { index: 0, top_m: top, bottom_m: bottom, safety_factor: factor }] } else { Vec::new() }
}

/// 🫧 Excess pore pressure ratio of every liquefiable layer of the soil profile of a foundation and the lateral spreading screening of the slopes standing on its layers.
pub fn check_liquefaction_hazard(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    let a_g_s = gamma_i * a_g * s;
    let Some((profile, rows)) = soil_profile(annex, document, f, a_g_s).filter(|(p, _)| f.liq_magnitude > 0.0 && p.layers.iter().all(|l| l.unit_weight_n_m3 > 0.0 && l.thickness_m > 0.0)) else { return };
    let required = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let _ = semio_s_plugin_norm_compliance::drain_ndp_reads();
    for (index, row) in rows.iter().enumerate().filter(|(i, _)| profile.layers[*i].n1_60 > 0.0) {
        let (z, f_l) = profile.layer_governing(index);
        if !f_l.is_finite() {
            continue;
        }
        let (alpha, exponent) = (ndp_number("ndp.en1998.en5.liquefaction.ru.alpha", annex), ndp_number("ndp.en1998.en5.liquefaction.ru.cycle-exponent", annex));
        let r_u = hazard::excess_pore_pressure_ratio(f_l, alpha, exponent);
        let (_, effective) = profile.stresses(z);
        report.push(
            CheckResult::assess(format!("en1998.5.{}.layer.{}.poreRatio", f.id, row.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.4"), SubjectRef::new(&row.id, format!("soilLayers[id={}].n160", row.id), lc(&row.id, &row.id)), lc(&format!("Excess pore pressure ratio r_u of layer {}", row.id), &format!("Porenwasserüberdruckverhältnis r_u der Schicht {}", row.id)))
                .informative(q_dim(r_u))
                .annex(annex)
                .explanation(lc(
                    &format!("F_L = {f_l:.3} at {z:.1} m gives r_u = 0.5 + asin(2 F_L^(−1/(b α)) − 1)/π = {r_u:.3} (α = {alpha}, b = {exponent}): the effective stress σ'_v = {effective:.0} Pa falls to {:.0} Pa; {}", (1.0 - r_u) * effective, if r_u >= 1.0 { "the layer liquefies (r_u = 1, no shear strength)" } else if f_l < required { "the layer stays below the required factor" } else { "the layer meets the required factor" }),
                    &format!("F_L = {f_l:.3} in {z:.1} m ergibt r_u = 0,5 + asin(2 F_L^(−1/(b α)) − 1)/π = {r_u:.3} (α = {alpha}, b = {exponent}): die effektive Spannung σ'_v = {effective:.0} Pa fällt auf {:.0} Pa; {}", (1.0 - r_u) * effective, if r_u >= 1.0 { "die Schicht verflüssigt (r_u = 1, keine Scherfestigkeit)" } else if f_l < required { "die Schicht bleibt unter dem erforderlichen Faktor" } else { "die Schicht erfüllt den erforderlichen Faktor" }),
                ))
                .ndp_reads()
                .build(),
        );
    }
    let layers = hazard::liquefied_layers(&profile, required);
    if layers.is_empty() {
        return;
    }
    let (min_percent, max_percent) = (ndp_number("ndp.en1998.en5.liquefaction.spreading.min-slope-percent", annex), ndp_number("ndp.en1998.en5.liquefaction.spreading.max-slope-percent", annex));
    let liquefied_ids: Vec<&str> = layers.iter().map(|l| rows[l.index].id.as_str()).collect();
    let slopes: Vec<_> = document.slopes.iter().filter(|sl| liquefied_ids.contains(&sl.governing_layer_id.as_str())).collect();
    let thickness = hazard::liquefied_thickness_m(&layers);
    let worst = slopes.iter().map(|sl| (sl, hazard::slope_percent(sl.angle), hazard::spreading_class(hazard::slope_percent(sl.angle), min_percent, max_percent))).max_by_key(|(_, _, class)| match class { Spreading::Level => 0, Spreading::Gentle => 1, Spreading::Steep => 2 });
    let (status, text_en, text_de) = match worst {
        None => (CheckStatus::Info, "no slope or free face stands on a liquefied layer: lateral spreading is not assessed".to_owned(), "keine Böschung oder freie Fläche steht auf einer verflüssigten Schicht: seitliches Ausbreiten wird nicht beurteilt".to_owned()),
        Some((sl, percent, Spreading::Level)) => (CheckStatus::Info, format!("slope {} on layer {} has S = {percent:.2} % below {min_percent} %: level ground, the liquefied soil oscillates but does not spread", sl.id, sl.governing_layer_id), format!("Böschung {} auf Schicht {} hat S = {percent:.2} % unter {min_percent} %: ebenes Gelände, der verflüssigte Boden schwingt, breitet sich aber nicht aus", sl.id, sl.governing_layer_id)),
        Some((sl, percent, Spreading::Gentle)) => (CheckStatus::Warning, format!("slope {} on layer {} has S = {percent:.2} % between {min_percent} % and {max_percent} %: lateral spreading of the liquefied soil is possible, foundations and lifelines in the slope need a displacement analysis", sl.id, sl.governing_layer_id), format!("Böschung {} auf Schicht {} hat S = {percent:.2} % zwischen {min_percent} % und {max_percent} %: seitliches Ausbreiten des verflüssigten Bodens ist möglich, Gründungen und Leitungen im Hang brauchen eine Verschiebungsberechnung", sl.id, sl.governing_layer_id)),
        Some((sl, percent, Spreading::Steep)) => (CheckStatus::Warning, format!("slope {} on layer {} has S = {percent:.2} % above {max_percent} %: the liquefied soil fails as a flow slide, the seismic slope stability governs", sl.id, sl.governing_layer_id), format!("Böschung {} auf Schicht {} hat S = {percent:.2} % über {max_percent} %: der verflüssigte Boden versagt als Fließrutschung, die Standsicherheit der Böschung unter Erdbeben ist maßgebend", sl.id, sl.governing_layer_id)),
    };
    let mut check = CheckResult::assess(format!("en1998.5.{}.spreading", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.1.4"), SubjectRef::new(&f.id, format!("{}.liqWaterDepthM", id_sel("foundations", &f.id)), lc(&f.id, &f.id)), lc("Lateral spreading of the liquefied ground", "Seitliches Ausbreiten des verflüssigten Bodens"))
        .informative(Quantity::new(QuantityKind::Length, thickness))
        .status(status)
        .annex(annex);
    let mut advisory_en = String::new();
    let mut advisory_de = String::new();
    if status == CheckStatus::Warning {
        for layer in &layers {
            let row = rows[layer.index];
            let improved = |n: f64| {
                let mut changed = profile.layers.clone();
                changed[layer.index].n1_60 = n;
                SoilProfile { layers: changed, ..profile.clone() }.layer_governing(layer.index).1
            };
            if improved(60.0) >= required {
                let (mut low, mut high) = (row.n1_60, 60.0);
                for _ in 0..48 {
                    let mid = 0.5 * (low + high);
                    if improved(mid) >= required { high = mid } else { low = mid }
                }
                check = check.remedy(Remedy::at_least(SubjectRef::new(&row.id, format!("soilLayers[id={}].n160", row.id), lc("Blow count (N1)60 of the layer", "Schlagzahl (N1)60 der Schicht")), q_dim(row.n1_60), q_dim(high), lc(&format!("Densify layer {} (ground improvement) until it no longer liquefies; without liquefied layer the ground cannot spread.", row.id), &format!("Schicht {} verdichten (Baugrundverbesserung), bis sie nicht mehr verflüssigt; ohne verflüssigte Schicht kann sich der Boden nicht ausbreiten.", row.id))));
            } else {
                advisory_en.push_str(&format!(" Layer {} cannot be densified to F_L {required} within (N1)60 ≤ 60: replace it or lower the ground water.", row.id));
                advisory_de.push_str(&format!(" Schicht {} lässt sich innerhalb (N1)60 ≤ 60 nicht auf F_L {required} verdichten: Schicht ersetzen oder Grundwasser absenken.", row.id));
            }
        }
        if let Some((sl, _, Spreading::Gentle)) = worst {
            let level = (min_percent / 100.0).atan().to_degrees();
            check = check.remedy(Remedy::at_most(SubjectRef::new(&sl.id, format!("slopes[id={}].angle", sl.id), lc("Slope angle β", "Böschungswinkel β")), q_dim(sl.angle), q_dim(level), lc("Grade the ground to level (below the lowest spreading slope) so that the liquefied soil only oscillates.", "Gelände eben abgraben (unter die kleinste Ausbreitungsneigung), damit der verflüssigte Boden nur schwingt.")));
        }
    }
    report.push(
        check
            .explanation(lc(
                &format!("{} layer(s) with F_L below {required} are liquefied over {thickness:.1} m: {text_en}.{advisory_en}", layers.len()),
                &format!("{} Schicht(en) mit F_L unter {required} sind über {thickness:.1} m verflüssigt: {text_de}.{advisory_de}", layers.len()),
            ))
            .ndp_reads()
            .build(),
    );
}

/// 🏗️ The pile of a foundation in liquefied soil: Euler buckling over the liquefied length plus the depth to fixity, and the bending moment of the head shear over the unsupported length, both against the pile resistances.
pub fn check_pile_liquefied(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    if f.pile_diameter_m <= 0.0 || f.pile_length_m <= 0.0 || f.pile_modulus_pa <= 0.0 || f.pile_subgrade_modulus_n_m3 <= 0.0 {
        return;
    }
    let a_g_s = gamma_i * a_g * s;
    let required = ndp_number("ndp.en1998.en5.liquefaction.min-safety-factor", annex);
    let _ = semio_s_plugin_norm_compliance::drain_ndp_reads();
    let layers = liquefied(annex, document, f, a_g_s, required);
    let _ = semio_s_plugin_norm_compliance::drain_ndp_reads();
    let unsupported = hazard::liquefied_thickness_m(&layers).min(f.pile_length_m);
    if unsupported <= 0.0 {
        return;
    }
    let path = id_sel("foundations", &f.id);
    let subject = |field: &str| SubjectRef::new(&f.id, format!("{path}.{field}"), lc(&f.id, &f.id));
    let (length_factor, fixity) = (ndp_number("ndp.en1998.en5.pile.liquefied.length-factor", annex), ndp_number("ndp.en1998.en5.pile.liquefied.fixity-factor", annex));
    let effective = |diameter: f64| hazard::pile_effective_length_m(unsupported, fixity, semio_s_artifact_norm_en1998_schema::part_5::pile_beta_per_m(f.pile_modulus_pa, diameter, f.pile_subgrade_modulus_n_m3), length_factor);
    let buckling = |diameter: f64| hazard::pile_buckling_load_n(semio_s_artifact_norm_en1998_schema::part_5::pile_bending_stiffness(f.pile_modulus_pa, diameter), effective(diameter));
    let l_eff = effective(f.pile_diameter_m);
    let ndps = ["ndp.en1998.en5.pile.liquefied.length-factor", "ndp.en1998.en5.pile.liquefied.fixity-factor"];
    if f.pile_axial_load_n > 0.0 {
        let n_cr = buckling(f.pile_diameter_m);
        let mut check = CheckResult::assess(format!("en1998.5.{}.pile.liquefiedBuckling", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileAxialLoadN"), lc("Pile buckling in liquefied soil", "Knicken des Pfahls im verflüssigten Boden"))
            .utilization(q_force(f.pile_axial_load_n), floor_limit(q_force(n_cr)))
            .annex(annex)
            .ndps(&ndps)
            .explanation(lc(
                &format!("The soil support is lost over {unsupported:.1} m: L_eff = K (L_u + c/β) = {l_eff:.2} m, N_cr = π² EI/L_eff² = {n_cr:.0} N against N_Ed = {:.0} N", f.pile_axial_load_n),
                &format!("Die Bettung entfällt über {unsupported:.1} m: L_eff = K (L_u + c/β) = {l_eff:.2} m, N_cr = π² EI/L_eff² = {n_cr:.0} N gegen N_Ed = {:.0} N", f.pile_axial_load_n),
            ));
        if f.pile_axial_load_n > n_cr {
            let mut high = f.pile_diameter_m;
            while buckling(high) < f.pile_axial_load_n && high < 100.0 * f.pile_diameter_m {
                high *= 1.5;
            }
            if buckling(high) >= f.pile_axial_load_n {
                let mut low = f.pile_diameter_m;
                for _ in 0..60 {
                    let mid = 0.5 * (low + high);
                    if buckling(mid) >= f.pile_axial_load_n { high = mid } else { low = mid }
                }
                check = check.remedy(Remedy::at_least(subject("pileDiameterM"), q_len(f.pile_diameter_m), q_len(high), lc("Enlarge the pile diameter until the Euler load over the liquefied length carries the axial force.", "Pfahldurchmesser vergrößern, bis die Eulerlast über die verflüssigte Länge die Normalkraft trägt.")));
            }
        }
        report.push(check.build());
    }
    if f.pile_head_shear_n > 0.0 {
        let fixed = f.pile_head_fixed >= 0.5;
        let m_ed = hazard::pile_unsupported_moment_nm(f.pile_head_shear_n, l_eff, fixed);
        let mut check = CheckResult::assess(format!("en1998.5.{}.pile.liquefiedMoment", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "5.4.2"), subject("pileMomentRdNm"), lc("Pile bending moment over the liquefied length", "Pfahlbiegemoment über die verflüssigte Länge"))
            .utilization(q_moment(m_ed), floor_limit(q_moment(f.pile_moment_rd_nm)))
            .annex(annex)
            .ndps(&ndps)
            .explanation(lc(
                &format!("The head shear {:.0} N acts on the unsupported length L_eff = {l_eff:.2} m ({}): M_Ed = {m_ed:.0} N·m against M_Rd = {:.0} N·m", f.pile_head_shear_n, if fixed { "head fixed, V L_eff/2" } else { "head free, V L_eff" }, f.pile_moment_rd_nm),
                &format!("Die Kopfquerkraft {:.0} N wirkt auf die ungestützte Länge L_eff = {l_eff:.2} m ({}): M_Ed = {m_ed:.0} N·m gegen M_Rd = {:.0} N·m", f.pile_head_shear_n, if fixed { "Kopf eingespannt, V L_eff/2" } else { "Kopf frei, V L_eff" }, f.pile_moment_rd_nm),
            ));
        if m_ed > f.pile_moment_rd_nm {
            check = check.remedy(Remedy::at_least(subject("pileMomentRdNm"), q_moment(f.pile_moment_rd_nm), q_moment(m_ed), lc("Increase the bending resistance of the pile for the liquefied condition.", "Biegetragfähigkeit des Pfahls für den verflüssigten Zustand erhöhen.")));
        }
        report.push(check.build());
    }
}

/// 📉 The small-strain shear modulus G_max = ρ v_s² of every layer of the profile of a foundation and the modulus degraded by EN 1998-5 Table 4.1 for the design acceleration, with the soil damping of the table.
pub fn check_degraded_modulus(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, f: &Foundation, gamma_i: f64, a_g: f64, s: f64) {
    let rows: Vec<_> = document.soil_layers.iter().filter(|l| l.foundation_id == f.id && l.unit_weight_n_m3 > 0.0 && l.shear_wave_velocity_m_s > 0.0).collect();
    if rows.is_empty() {
        return;
    }
    let a_g_s_over_g = gamma_i * a_g * s / 9.81;
    let _ = semio_s_plugin_norm_compliance::drain_ndp_reads();
    let (ratio, damping) = (soil_structure::table_4_1_vs_ratio(a_g_s_over_g, annex), soil_structure::table_4_1_damping(a_g_s_over_g, annex));
    let listing = |de: bool| {
        rows.iter()
            .map(|l| {
                let g_max = soil_structure::shear_modulus_max_pa(l.unit_weight_n_m3, l.shear_wave_velocity_m_s);
                format!("{}: G_max {:.3e} Pa, {} {:.3e} Pa, v_s {:.0} → {:.0} m/s", l.id, g_max, if de { "G abgemindert" } else { "G degraded" }, soil_structure::degraded_shear_modulus_pa(g_max, ratio), l.shear_wave_velocity_m_s, l.shear_wave_velocity_m_s * ratio)
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let top = rows[0];
    let g_top = soil_structure::degraded_shear_modulus_pa(soil_structure::shear_modulus_max_pa(top.unit_weight_n_m3, top.shear_wave_velocity_m_s), ratio);
    report.push(
        CheckResult::assess(format!("en1998.5.{}.profile.shearModulus", f.id), "DIN EN 1998-5", ClauseId::new("EN 1998-5", "5", "4.2.2"), SubjectRef::new(&top.id, format!("soilLayers[id={}].shearWaveVelocityMS", top.id), lc(&top.id, &top.id)), lc("Shear modulus of the layers degraded for the design earthquake", "Schubmodul der Schichten für das Bemessungsbeben abgemindert"))
            .informative(q_pressure(g_top))
            .annex(annex)
            .explanation(lc(
                &format!("a_g S/g = {a_g_s_over_g:.3}: v_s/v_s,max = {ratio:.3} and G/G_max = {:.3} (Table 4.1), soil damping {:.1} %; {}", ratio * ratio, 100.0 * damping, listing(false)),
                &format!("a_g S/g = {a_g_s_over_g:.3}: v_s/v_s,max = {ratio:.3} und G/G_max = {:.3} (Tabelle 4.1), Bodendämpfung {:.1} %; {}", ratio * ratio, 100.0 * damping, listing(true)),
            ))
            .ndp_reads()
            .build(),
    );
}
