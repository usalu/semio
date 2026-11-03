//! 🪧️ Horizontal element checks of one building: the in-plane resistance of its diaphragms (EN 1998-1 4.4.2.5), the overturning stability (4.4.2) and the accelerogram set of a time-history analysis (3.2.3.1.2). See [`super`].

use super::*;
use semio_s_artifact_norm_en1998_model::{En1998Accelerogram, En1998Building, En1998Diaphragm, En1998System};
use semio_s_artifact_norm_en1998_schema::horizontal::{diaphragm, overturning, time_history};

const FAIL: semio_s_plugin_norm_compliance::CheckStatus = semio_s_plugin_norm_compliance::CheckStatus::Fail;
const TRANSFER_FACTOR: &str = "ndp.en1998.en1.diaphragm.transfer-factor";
const STABILISING_FACTOR: &str = "ndp.en1998.en1.overturning.stabilising-factor";
const MIN_RECORDS: &str = "ndp.en1998.en1.time-history.min-records";
const MEAN_RATIO: &str = "ndp.en1998.en1.time-history.mean-spectrum-ratio";
const ZERO_RATIO: &str = "ndp.en1998.en1.time-history.zero-period-ratio";

fn q_area(m2: f64) -> Quantity { Quantity::area_m2(m2) }

/// 🧱 Checks the in-plane shear and the chord of every diaphragm of `building` against the transferred force `γ_d F_i` of its storey for one structural system.
pub fn check_diaphragms(document: &En1998Snapshot, annex: AnnexChoice, report: &mut CheckReport, building: &En1998Building, bid: &str, system: &En1998System, forces: &[f64]) {
    let along_y = system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y;
    for d in document.diaphragms.iter().filter(|d| d.building_id == building.id) {
        let Some(index) = building.storeys.iter().position(|s| s.id == d.storey_id) else { continue };
        let (plan_span, plan_depth) = if along_y { (building.plan_length_m, building.plan_width_m) } else { (building.plan_width_m, building.plan_length_m) };
        let (span, depth) = diaphragm::extent_m(d.span_m, d.depth_m, plan_span, plan_depth);
        if span <= 0.0 || depth <= 0.0 {
            continue;
        }
        let f_i = forces.get(index).copied().unwrap_or(0.0);
        let gamma_d = if d.gamma_d > 0.0 { d.gamma_d } else { semio_s_plugin_norm_compliance::ndp_number(TRANSFER_FACTOR, annex) };
        let f_d = diaphragm::design_force_n(f_i, gamma_d);
        let path = id_sel("diaphragms", &d.id);
        let label = lc(&d.id, &d.id);
        let v_ed = diaphragm::support_shear_n(f_d);
        let v_rd = diaphragm::shear_resistance_n(d.shear_strength_mpa, d.thickness_m, depth, d.opening_width_m);
        let d_eff = diaphragm::effective_depth_m(depth, d.opening_width_m);
        let mut shear = CheckResult::assess(
            format!("en1998.1.{bid}.{}.diaphragm.{}.shear", system.id, d.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.5"),
            SubjectRef::new(&d.id, format!("{path}.thicknessM"), label.clone()),
            lc("Diaphragm in-plane shear V_Ed ≤ V_Rd", "Scheibenschub in der Ebene V_Ed ≤ V_Rd"),
        )
        .utilization(q_force(v_ed), floor_limit(q_force(v_rd)))
        .annex(annex)
        .explanation(lc(
            &format!("F_i = {f_i:.0} N at storey {}, γ_d = {gamma_d:.2}, F_d = {f_d:.0} N, V_Ed = F_d/2 = {v_ed:.0} N; V_Rd = τ_Rd t D_eff = {:.2} MPa · {:.3} m · {d_eff:.2} m = {v_rd:.0} N (span L = {span:.2} m, depth D = {depth:.2} m, openings {:.2} m)", d.storey_id, d.shear_strength_mpa, d.thickness_m, d.opening_width_m),
            &format!("F_i = {f_i:.0} N im Geschoss {}, γ_d = {gamma_d:.2}, F_d = {f_d:.0} N, V_Ed = F_d/2 = {v_ed:.0} N; V_Rd = τ_Rd t D_eff = {:.2} MPa · {:.3} m · {d_eff:.2} m = {v_rd:.0} N (Spannweite L = {span:.2} m, Tiefe D = {depth:.2} m, Öffnungen {:.2} m)", d.storey_id, d.shear_strength_mpa, d.thickness_m, d.opening_width_m),
        ));
        if d.gamma_d <= 0.0 {
            shear = shear.ndp(TRANSFER_FACTOR);
        }
        if v_ed > v_rd {
            let t_req = diaphragm::required_thickness_m(v_ed, d.shear_strength_mpa, depth, d.opening_width_m);
            if t_req.is_finite() {
                shear = shear.remedy(Remedy::at_least(
                    SubjectRef::new(&d.id, format!("{path}.thicknessM"), lc("Diaphragm thickness", "Scheibendicke")),
                    q_len(d.thickness_m),
                    q_len(t_req),
                    lc(&format!("Increase the diaphragm thickness to at least {t_req:.3} m."), &format!("Scheibendicke auf mindestens {t_req:.3} m erhöhen.")),
                ));
            }
            if d.thickness_m * d_eff > 0.0 {
                let tau_req = v_ed / (d.thickness_m * d_eff) / 1.0e6;
                shear = shear.remedy(Remedy::at_least(
                    SubjectRef::new(&d.id, format!("{path}.shearStrengthMpa"), lc("Design shear strength", "Bemessungs-Schubfestigkeit")),
                    q_press(d.shear_strength_mpa * 1.0e6),
                    q_press(tau_req * 1.0e6),
                    lc(&format!("Use a diaphragm with a design shear strength of at least {tau_req:.3} MPa."), &format!("Scheibe mit einer Bemessungs-Schubfestigkeit von mindestens {tau_req:.3} MPa verwenden.")),
                ));
            }
            if d.shear_strength_mpa * d.thickness_m > 0.0 {
                let open_max = (depth - v_ed / (d.shear_strength_mpa * 1.0e6 * d.thickness_m)).max(0.0);
                shear = shear.remedy(Remedy::at_most(
                    SubjectRef::new(&d.id, format!("{path}.openingWidthM"), lc("Opening width", "Öffnungsbreite")),
                    q_len(d.opening_width_m),
                    q_len(open_max),
                    lc(&format!("Reduce the openings across the shear section to at most {open_max:.2} m."), &format!("Öffnungen im Schubschnitt auf höchstens {open_max:.2} m verringern.")),
                ));
            }
        }
        report.push(shear.ndp_reads().build());

        let n_ed = diaphragm::chord_force_n(f_d, span, depth);
        let n_rd = diaphragm::chord_resistance_n(d.chord_area_m2, d.chord_strength_mpa);
        let mut chord = CheckResult::assess(
            format!("en1998.1.{bid}.{}.diaphragm.{}.chord", system.id, d.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.5"),
            SubjectRef::new(&d.id, format!("{path}.chordAreaM2"), label),
            lc("Diaphragm chord N_Ed ≤ N_Rd", "Scheibengurt N_Ed ≤ N_Rd"),
        )
        .utilization(q_force(n_ed), floor_limit(q_force(n_rd)))
        .annex(annex)
        .explanation(lc(
            &format!("M = F_d L/8 = {:.0} N·m, N_Ed = M/D = {n_ed:.0} N; N_Rd = A f_d = {:.5} m² · {:.1} MPa = {n_rd:.0} N", f_d * span / 8.0, d.chord_area_m2, d.chord_strength_mpa),
            &format!("M = F_d L/8 = {:.0} N·m, N_Ed = M/D = {n_ed:.0} N; N_Rd = A f_d = {:.5} m² · {:.1} MPa = {n_rd:.0} N", f_d * span / 8.0, d.chord_area_m2, d.chord_strength_mpa),
        ));
        if d.gamma_d <= 0.0 {
            chord = chord.ndp(TRANSFER_FACTOR);
        }
        if n_ed > n_rd {
            let a_req = diaphragm::required_chord_area_m2(n_ed, d.chord_strength_mpa);
            if a_req.is_finite() {
                chord = chord.remedy(Remedy::at_least(
                    SubjectRef::new(&d.id, format!("{path}.chordAreaM2"), lc("Chord area", "Gurtfläche")),
                    q_area(d.chord_area_m2),
                    q_area(a_req),
                    lc(&format!("Increase the chord area to at least {a_req:.5} m²."), &format!("Gurtquerschnitt auf mindestens {a_req:.5} m² erhöhen.")),
                ));
            }
            if d.chord_area_m2 > 0.0 {
                let f_req = n_ed / d.chord_area_m2 / 1.0e6;
                chord = chord.remedy(Remedy::at_least(
                    SubjectRef::new(&d.id, format!("{path}.chordStrengthMpa"), lc("Chord strength", "Gurtfestigkeit")),
                    q_press(d.chord_strength_mpa * 1.0e6),
                    q_press(f_req * 1.0e6),
                    lc(&format!("Use a chord with a design strength of at least {f_req:.1} MPa."), &format!("Gurt mit einer Bemessungsfestigkeit von mindestens {f_req:.1} MPa verwenden.")),
                ));
            }
        }
        report.push(chord.ndp_reads().build());
    }
}

/// 🔄 Checks the overturning stability `M_ov ≤ M_stab` of one structural system: `M_ov = Σ F_i z_i`, `M_stab = γ G_k b/2 + M_tie` about the edge of the base in the direction of the action.
pub fn check_overturning(annex: AnnexChoice, report: &mut CheckReport, building: &En1998Building, bid: &str, bpath: &str, blabel: &LocalizedCopy, system: &En1998System, forces: &[f64], heights: &[f64]) {
    let mut z = 0.0;
    let elevations: Vec<f64> = heights.iter().map(|h| { z += h; z }).collect();
    let m_ov = overturning::moment_nm(forces, &elevations);
    if m_ov <= 0.0 {
        return;
    }
    let along_y = system.direction == semio_s_artifact_norm_en1998_model::AnalysisDirection::Y;
    let (entered, plan, field) = if along_y { (building.overturning_base_y_m, building.plan_length_m, "overturningBaseYM") } else { (building.overturning_base_x_m, building.plan_width_m, "overturningBaseXM") };
    let base = if entered > 0.0 { entered } else { plan };
    let weight: f64 = building.storeys.iter().map(|s| s.permanent_gk_n).sum();
    let factor = semio_s_plugin_norm_compliance::ndp_number(STABILISING_FACTOR, annex);
    let m_stab = overturning::stabilising_moment_nm(weight, base, building.tie_down_moment_nm, factor);
    let mut check = CheckResult::assess(
        format!("en1998.1.{bid}.{}.overturning", system.id),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "4.4.2"),
        SubjectRef::new(bid, format!("{bpath}.{field}"), blabel.clone()),
        lc("Overturning stability M_ov ≤ M_stab", "Kippsicherheit M_ov ≤ M_stab"),
    )
    .utilization(q_mom(m_ov), floor_limit(q_mom(m_stab)))
    .annex(annex)
    .explanation(lc(
        &format!("M_ov = Σ F_i z_i = {m_ov:.0} N·m; M_stab = {factor:.2} · G_k b/2 + M_tie = {factor:.2} · {weight:.0} N · {base:.2} m / 2 + {:.0} N·m = {m_stab:.0} N·m (direction {})", building.tie_down_moment_nm, system.direction.token()),
        &format!("M_ov = Σ F_i z_i = {m_ov:.0} N·m; M_stab = {factor:.2} · G_k b/2 + M_tie = {factor:.2} · {weight:.0} N · {base:.2} m / 2 + {:.0} N·m = {m_stab:.0} N·m (Richtung {})", building.tie_down_moment_nm, system.direction.token()),
    ))
    .ndp(STABILISING_FACTOR);
    if m_ov > m_stab {
        let b_req = overturning::required_base_width_m(m_ov, weight, building.tie_down_moment_nm, factor);
        if b_req.is_finite() {
            check = check.remedy(Remedy::at_least(
                SubjectRef::new(bid, format!("{bpath}.{field}"), lc("Base width", "Fundamentbreite")),
                q_len(base),
                q_len(b_req),
                lc(&format!("Widen the base in the direction of the action to at least {b_req:.2} m."), &format!("Fundament in Richtung der Einwirkung auf mindestens {b_req:.2} m verbreitern.")),
            ));
        }
        let tie_req = overturning::required_tie_down_moment_nm(m_ov, weight, base, factor);
        check = check.remedy(Remedy::at_least(
            SubjectRef::new(bid, format!("{bpath}.tieDownMomentNm"), lc("Tie-down moment", "Rückhaltemoment")),
            q_mom(building.tie_down_moment_nm),
            q_mom(tie_req),
            lc(&format!("Add tie-downs with a stabilising moment of at least {tie_req:.0} N·m."), &format!("Zuganker mit einem Rückhaltemoment von mindestens {tie_req:.0} N·m ergänzen.")),
        ));
    }
    report.push(check.ndp_reads().build());
}

struct Record<'a> {
    row: &'a En1998Accelerogram,
    samples: Vec<f64>,
}

fn parse_record(row: &En1998Accelerogram) -> Result<Vec<f64>, (&'static str, &'static str)> {
    let Some(samples) = time_history::parse_samples(&row.samples) else { return Err(("a sample is not a finite number", "ein Wert ist keine endliche Zahl")) };
    if samples.len() < 2 {
        return Err(("at least two samples are needed", "mindestens zwei Werte sind nötig"));
    }
    if row.dt_s <= 0.0 {
        return Err(("the sampling step must be positive", "der Abtastschritt muss positiv sein"));
    }
    if row.scale_factor <= 0.0 {
        return Err(("the scale factor must be positive", "der Skalierungsfaktor muss positiv sein"));
    }
    Ok(samples)
}

/// 📈 Checks the accelerogram set of a building against EN 1998-1 3.2.3.1.2: enough records, mean zero-period acceleration at least `a_g S`, mean 5 % damped spectrum at least 90 % of the elastic spectrum between 0.2 T1 and 2 T1; every failing set check offers the common scale factor as remedy of every record.
pub fn check_time_history(document: &En1998Snapshot, annex: AnnexChoice, report: &mut CheckReport, gamma_i: f64, a_gr: f64, s: f64, tb: f64, tc: f64, td: f64, building: &En1998Building, bid: &str, t1: f64) {
    let rows: Vec<&En1998Accelerogram> = document.accelerograms.iter().filter(|r| r.building_id == building.id).collect();
    if rows.is_empty() {
        return;
    }
    let a_g = gamma_i * a_gr;
    let mut valid: Vec<Record> = Vec::new();
    for row in rows.iter().copied() {
        let path = id_sel("accelerograms", &row.id);
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.timeHistory.record.{}", row.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "3.2.3.1.2"),
            SubjectRef::new(&row.id, format!("{path}.samples"), lc(&row.label, &row.label)),
            lc("Accelerogram record", "Akzelerogramm-Aufzeichnung"),
        )
        .annex(annex);
        match parse_record(row) {
            Ok(samples) => {
                let pga = time_history::peak_acceleration(&samples) * row.scale_factor;
                check = check.informative(q_acc(pga)).explanation(lc(
                    &format!("{} samples, dt = {:.4} s, duration {:.2} s, scale {:.3}, PGA {pga:.3} m/s²", samples.len(), row.dt_s, (samples.len() - 1) as f64 * row.dt_s, row.scale_factor),
                    &format!("{} Werte, dt = {:.4} s, Dauer {:.2} s, Skalierung {:.3}, PGA {pga:.3} m/s²", samples.len(), row.dt_s, (samples.len() - 1) as f64 * row.dt_s, row.scale_factor),
                ));
                valid.push(Record { row, samples });
            }
            Err((en, de)) => {
                check = check.status(FAIL).explanation(lc(&format!("The record is not usable: {en}."), &format!("Die Aufzeichnung ist nicht verwendbar: {de}.")));
                if row.scale_factor <= 0.0 {
                    check = check.remedy(Remedy::exactly(
                        SubjectRef::new(&row.id, format!("{path}.scaleFactor"), lc("Scale factor", "Skalierungsfaktor")),
                        q_dim(row.scale_factor),
                        q_dim(1.0),
                        lc("Use the scale factor 1 for an unscaled record.", "Skalierungsfaktor 1 für eine unskalierte Aufzeichnung verwenden."),
                    ));
                }
            }
        }
        report.push(check.build());
    }
    let ((min_records, min_ratio, zero_ratio), ndps) = semio_s_plugin_norm_compliance::record_ndp_reads(|| {
        (semio_s_plugin_norm_compliance::ndp_number(MIN_RECORDS, annex), semio_s_plugin_norm_compliance::ndp_number(MEAN_RATIO, annex), semio_s_plugin_norm_compliance::ndp_number(ZERO_RATIO, annex))
    });
    let bpath = id_sel("buildings", bid);
    let blabel = lc(&building.name, &building.name);
    let mut count = CheckResult::assess(
        format!("en1998.1.{bid}.timeHistory.records"),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "3.2.3.1.2"),
        SubjectRef::new(bid, format!("{bpath}.id"), blabel.clone()),
        lc("Number of accelerograms", "Anzahl der Akzelerogramme"),
    )
    .minimum(q_dim(valid.len() as f64), q_dim(min_records))
    .annex(annex)
    .ndps(&ndps)
    .explanation(lc(
        &format!("{} usable of {} records, at least {min_records:.0} are required", valid.len(), rows.len()),
        &format!("{} verwendbar von {} Aufzeichnungen, mindestens {min_records:.0} sind erforderlich", valid.len(), rows.len()),
    ));
    if (valid.len() as f64) < min_records {
        count = count.status(FAIL).remedy(Remedy::one_of(
            SubjectRef::new(bid, "accelerograms", lc("Accelerograms", "Akzelerogramme")),
            vec![format!("{bid}-record-{}", rows.len() + 1)],
            lc(&format!("Add records until {min_records:.0} usable accelerograms are in the set."), &format!("Aufzeichnungen ergänzen, bis {min_records:.0} verwendbare Akzelerogramme im Satz sind.")),
        ));
    }
    report.push(count.build());
    if valid.is_empty() {
        return;
    }
    let periods = time_history::periods_s(t1.max(1e-3), time_history::PERIOD_SAMPLES);
    let elastic: Vec<f64> = periods.iter().map(|t| part_1::elastic_response_spectrum(a_g, s, tb, tc, td, *t)).collect();
    let spectra: Vec<(f64, Vec<f64>)> = valid.iter().map(|r| (time_history::peak_acceleration(&r.samples) * r.row.scale_factor, time_history::record_spectrum(&r.samples, r.row.scale_factor, r.row.dt_s, &periods))).collect();
    let judged = time_history::judge_set(&spectra, &periods, &elastic, a_g * s, zero_ratio, min_ratio);
    let k = judged.scale_to_accept;
    let scale_remedies: Vec<Remedy> = valid
        .iter()
        .map(|r| {
            let required = r.row.scale_factor * k;
            Remedy::at_least(
                SubjectRef::new(&r.row.id, format!("{}.scaleFactor", id_sel("accelerograms", &r.row.id)), lc("Scale factor", "Skalierungsfaktor")),
                q_dim(r.row.scale_factor),
                q_dim(required),
                lc(&format!("Scale the records by {k:.3} (to {required:.3} for {}) so that the set is accepted.", r.row.id), &format!("Aufzeichnungen mit {k:.3} skalieren ({required:.3} für {}), damit der Satz angenommen wird.", r.row.id)),
            )
        })
        .collect();
    let mut zero = CheckResult::assess(
        format!("en1998.1.{bid}.timeHistory.zeroPeriod"),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "3.2.3.1.2"),
        SubjectRef::new(bid, format!("{bpath}.id"), blabel.clone()),
        lc("Mean zero-period acceleration ≥ a_g S", "Mittlere Nullperiodenbeschleunigung ≥ a_g S"),
    )
    .minimum(q_acc(judged.mean_zero_period), q_acc(judged.zero_period_required))
    .annex(annex)
    .ndps(&ndps)
    .explanation(lc(
        &format!("mean PGA {:.3} m/s² of {} records against {zero_ratio:.2} · a_g S = {:.3} m/s² (a_g = γ_I a_gR = {a_g:.3} m/s², S = {s:.2})", judged.mean_zero_period, judged.records, judged.zero_period_required),
        &format!("mittlere PGA {:.3} m/s² von {} Aufzeichnungen gegen {zero_ratio:.2} · a_g S = {:.3} m/s² (a_g = γ_I a_gR = {a_g:.3} m/s², S = {s:.2})", judged.mean_zero_period, judged.records, judged.zero_period_required),
    ));
    if judged.mean_zero_period < judged.zero_period_required {
        zero = scale_remedies.iter().cloned().fold(zero.status(FAIL), |c, r| c.remedy(r));
    }
    report.push(zero.build());
    let mut spectrum = CheckResult::assess(
        format!("en1998.1.{bid}.timeHistory.spectrum"),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "3.2.3.1.2"),
        SubjectRef::new(bid, format!("{bpath}.id"), blabel),
        lc("Mean 5 % spectrum ≥ 90 % of the elastic spectrum in 0.2 T1 … 2 T1", "Mittleres 5-%-Spektrum ≥ 90 % des elastischen Spektrums in 0,2 T1 … 2 T1"),
    )
    .minimum(q_dim(judged.lowest_mean_ratio), q_dim(min_ratio))
    .annex(annex)
    .ndps(&ndps)
    .explanation(lc(
        &format!("lowest ratio mean S_a / S_e = {:.3} at T = {:.3} s over {} periods from {:.3} s to {:.3} s (T1 = {t1:.3} s), required {min_ratio:.2}", judged.lowest_mean_ratio, judged.lowest_period_s, periods.len(), periods[0], periods[periods.len() - 1]),
        &format!("kleinstes Verhältnis mittleres S_a / S_e = {:.3} bei T = {:.3} s über {} Perioden von {:.3} s bis {:.3} s (T1 = {t1:.3} s), erforderlich {min_ratio:.2}", judged.lowest_mean_ratio, judged.lowest_period_s, periods.len(), periods[0], periods[periods.len() - 1]),
    ));
    if judged.lowest_mean_ratio < min_ratio {
        spectrum = scale_remedies.iter().cloned().fold(spectrum.status(FAIL), |c, r| c.remedy(r));
    }
    report.push(spectrum.build());
}
