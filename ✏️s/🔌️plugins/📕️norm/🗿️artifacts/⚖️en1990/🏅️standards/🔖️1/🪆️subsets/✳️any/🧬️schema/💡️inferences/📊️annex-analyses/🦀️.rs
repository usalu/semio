//! 📊️ Document-driven analyses of the EN 1990 report: Palmgren-Miner damage of the fatigue spectrum (§6.4.5), the FORM reliability index against Table B2 (Annex B/C) and the design value from a test series (Annex D.7). See [`super`].

use super::*;
use semio_s_artifact_norm_en1990_model::{ReliabilityAnalysis, TestSeries};

fn q_stress(pa: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, pa)
}

//#region 🔖️FatigueSpectrum
/// 🔩️ D = Σ n_i / N_i over the member's spectrum bins; silent for a member without bins (its `rdFat` check stays in charge).
pub(super) fn push_fatigue_damage(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    let bins: Vec<_> = document.fatigue_spectra.iter().filter(|bin| bin.member_id == member.id).collect();
    let tuples: Vec<_> = bins.iter().map(|bin| (bin.stress_range, bin.cycles, bin.detail_category)).collect();
    let Some(at) = governing_spectrum_bin(&tuples) else { return };
    let governing = bins[at];
    let id = format!("en1990.fat.damage.{}", member.id);
    let clause = ClauseId::new("EN 1990", "§6.4.5", "6.4.5");
    let field = |name: &str| SubjectRef::new(&governing.id, format!("fatigueSpectra[id={}].{name}", governing.id), member_label(member));
    let subject = field("detailCategory");
    let title = copy("Fatigue damage sum D = Σ n/N", "Ermüdungsschädigungssumme D = Σ n/N");
    match spectrum_damage(&tuples) {
        Err(reason) => report.push(not_applicable(id, "EN 1990 FAT", clause, subject, title, document.annex, { let r = semio_s_plugin_norm_compliance::explain_reason(&reason); copy(format!("Fatigue spectrum indeterminate: {}.", r.en), format!("Ermüdungskollektiv unbestimmt: {}.", r.de)) })),
        Ok(damage) => {
            let mut builder = CheckResult::assess(id, "EN 1990 FAT", clause, subject.clone(), title)
                .utilization(q_dim(damage), q_dim(1.0))
                .annex(document.annex)
                .explanation(copy(format!("D = {damage:.4} over {} bins (S-N m = 3 / 5, cut-off Δσ_L) vs 1.0; governing bin {}", bins.len(), governing.id), format!("D = {damage:.4} über {} Klassen (Wöhlerlinie m = 3 / 5, Abschneidegrenze Δσ_L) gegenüber 1,0; maßgebende Klasse {}", bins.len(), governing.id)));
            if damage > 1.0 {
                let category = required_detail_category(&tuples, at);
                let cycles = allowed_spectrum_cycles(&tuples, at);
                if let Some(category) = category {
                    builder = builder.remedy(Remedy::at_least(
                        subject.clone(),
                        q_stress(governing.detail_category),
                        q_stress(category),
                        copy(format!("Raise Δσ_C of the governing bin {} to at least {:.1} MPa (D ≤ 1 with the other bins kept).", governing.id, category * 1e-6), format!("Kerbfall Δσ_C der maßgebenden Klasse {} auf mindestens {:.1} MPa erhöhen (D ≤ 1 bei unveränderten übrigen Klassen).", governing.id, category * 1e-6)),
                    ));
                }
                if let Some(cycles) = cycles {
                    builder = builder.remedy(Remedy::at_most(
                        field("cycles"),
                        q_dim(governing.cycles),
                        q_dim(cycles),
                        copy(format!("Or limit the cycles n of the governing bin {} to at most {cycles:.0} (D ≤ 1 with the other bins kept).", governing.id), format!("Oder die Lastspielzahl n der maßgebenden Klasse {} auf höchstens {cycles:.0} begrenzen (D ≤ 1 bei unveränderten übrigen Klassen).", governing.id)),
                    ));
                }
                if category.is_none() && cycles.is_none() {
                    builder = builder.remedy(Remedy::advisory(
                        subject,
                        copy("The other bins alone reach D = 1: no single value of the governing bin solves the sum; revise the spectrum or the detail categories of several bins.", "Die übrigen Klassen erreichen allein D = 1: kein einzelner Wert der maßgebenden Klasse löst die Summe; Kollektiv oder Kerbfälle mehrerer Klassen überarbeiten."),
                    ));
                }
            }
            report.push(builder.build());
        }
    }
}
//#endregion 🔖️FatigueSpectrum

//#region 🔖️ReliabilityAnalysis
/// 🎯️ β of the FORM run versus the Table B2 target of the document's RC and reference period.
pub(super) fn push_reliability_analysis(report: &mut CheckReport, document: &En1990Snapshot, analysis: &ReliabilityAnalysis) {
    let id = format!("en1990.annex-c.form.{}", analysis.id);
    let clause = ClauseId::new("EN 1990", "Annex C", "C.5 FORM");
    let subject = SubjectRef::new(&analysis.id, format!("reliabilityAnalyses[id={}].resistanceMean", analysis.id), copy(format!("Reliability analysis {}", analysis.id), format!("Zuverlässigkeitsanalyse {}", analysis.id)));
    let title = copy("FORM reliability index β of g = R − E", "FORM-Zuverlässigkeitsindex β von g = R − E");
    let resistance = (analysis.resistance_distribution.as_str(), analysis.resistance_mean, analysis.resistance_sd);
    let action = (analysis.action_distribution.as_str(), analysis.action_mean, analysis.action_sd);
    if analysis.second_action_sd > 0.0 {
        return push_multi_reliability(report, document, analysis, id, clause, subject);
    }
    let form = match analysis_form(resistance, action) {
        Ok(form) => form,
        Err(reason) => return report.push(not_applicable(id, "EN 1990 Annex C", clause, subject, title, document.annex, { let r = semio_s_plugin_norm_compliance::explain_reason(&reason); copy(format!("FORM indeterminate: {}.", r.en), format!("FORM unbestimmt: {}.", r.de)) })),
    };
    let Some(target) = target_reliability_index_for(document.reliability_class.number(), document.reference_period_years) else {
        return report.push(not_applicable(id, "EN 1990 Annex C", clause, subject, title, document.annex, copy("Reliability class or reference period outside Table B2.", "Zuverlässigkeitsklasse oder Bezugszeitraum außerhalb Tabelle B2.")));
    };
    let (pf, pf_target) = (form.failure_probability(), phi_cdf(-target));
    let mut builder = CheckResult::assess(id, "EN 1990 Annex C", clause, subject.clone(), title)
        .ndp("ndp.en1990.e90-28.1")
        .minimum(q_dim(form.beta), q_dim(target))
        .annex(document.annex)
        .explanation(copy(
            format!("β = {:.2} (P_f = {pf:.2e}), α_R = {:.2}, α_E = {:.2}, R* = {:.1} N, E* = {:.1} N vs β_target = {target:.2} (P_f = {pf_target:.2e}) for RC{}", form.beta, form.alpha_r, form.alpha_e, form.design_r, form.design_e, document.reliability_class.number()),
            format!("β = {:.2} (P_f = {pf:.2e}), α_R = {:.2}, α_E = {:.2}, R* = {:.1} N, E* = {:.1} N gegenüber β_Ziel = {target:.2} (P_f = {pf_target:.2e}) für RC{}", form.beta, form.alpha_r, form.alpha_e, form.design_r, form.design_e, document.reliability_class.number()),
        ));
    if form.beta < target {
        builder = builder.remedy(match required_resistance_mean(resistance, action, target) {
            Some(needed) => Remedy::at_least(
                subject,
                q_force(analysis.resistance_mean),
                q_force(needed),
                copy(format!("Raise the resistance mean μ_R to at least {:.1} kN.", needed / 1000.0), format!("Mittelwert des Widerstands μ_R auf mindestens {:.1} kN erhöhen.", needed / 1000.0)),
            ),
            None => Remedy::advisory(subject, copy("No resistance mean within the FORM search range reaches the target β: reduce the scatter σ_R or σ_E, or change the distributions.", "Kein Widerstandsmittelwert im FORM-Suchbereich erreicht das Ziel-β: Streuung σ_R oder σ_E verringern oder die Verteilungen ändern.")),
        });
    }
    report.push(builder.build());
    push_annex_c_derived(report, document, analysis, &form);
}

/// 🎯️ β of the FORM run of g = R − E1 − E2 versus the Table B2 target; the check carries α_R, α_E1, α_E2 and the three design points, the C.7 fixed values use α_E for the leading and 0.4 α_E for the accompanying action.
fn push_multi_reliability(report: &mut CheckReport, document: &En1990Snapshot, analysis: &ReliabilityAnalysis, id: String, clause: ClauseId, subject: SubjectRef) {
    let title = copy("FORM reliability index β of g = R − E₁ − E₂", "FORM-Zuverlässigkeitsindex β von g = R − E₁ − E₂");
    let actions = [(analysis.action_distribution.as_str(), analysis.action_mean, analysis.action_sd), (analysis.second_action_distribution.as_str(), analysis.second_action_mean, analysis.second_action_sd)];
    let form = match analysis_form_multi((analysis.resistance_distribution.as_str(), analysis.resistance_mean, analysis.resistance_sd), &actions) {
        Ok(form) => form,
        Err(reason) => return report.push(not_applicable(id, "EN 1990 Annex C", clause, subject, title, document.annex, { let r = semio_s_plugin_norm_compliance::explain_reason(&reason); copy(format!("FORM indeterminate: {}.", r.en), format!("FORM unbestimmt: {}.", r.de)) })),
    };
    let Some(target) = target_reliability_index_for(document.reliability_class.number(), document.reference_period_years) else {
        return report.push(not_applicable(id, "EN 1990 Annex C", clause, subject, title, document.annex, copy("Reliability class or reference period outside Table B2.", "Zuverlässigkeitsklasse oder Bezugszeitraum außerhalb Tabelle B2.")));
    };
    let (pf, pf_target) = (form.failure_probability(), phi_cdf(-target));
    let lead = usize::from(form.alpha_e[1].abs() > form.alpha_e[0].abs());
    let ratio = actions[lead].2 / analysis.resistance_sd;
    let (alpha_r, alpha_lead) = annex_c_alphas(document.annex, actions[lead].2, analysis.resistance_sd);
    let alpha_second = annex_c_secondary_alpha(document.annex, alpha_lead);
    let mut builder = CheckResult::assess(id, "EN 1990 Annex C", clause, subject.clone(), title)
        .ndp("ndp.en1990.e90-28.1")
        .minimum(q_dim(form.beta), q_dim(target))
        .annex(document.annex)
        .explanation(copy(
            format!("β = {:.2} (P_f = {pf:.2e}), α_R = {:.2}, α_E1 = {:.2}, α_E2 = {:.2}, R* = {:.1} N, E1* = {:.1} N, E2* = {:.1} N vs β_target = {target:.2} (P_f = {pf_target:.2e}) for RC{}; C.7 with E{} leading (σ_E/σ_R = {ratio:.3}): α_R = {alpha_r:.2}, α_E = {alpha_lead:.2}, accompanying α_E = {alpha_second:.2}", form.beta, form.alpha_r, form.alpha_e[0], form.alpha_e[1], form.design_r, form.design_e[0], form.design_e[1], document.reliability_class.number(), lead + 1),
            format!("β = {:.2} (P_f = {pf:.2e}), α_R = {:.2}, α_E1 = {:.2}, α_E2 = {:.2}, R* = {:.1} N, E1* = {:.1} N, E2* = {:.1} N gegenüber β_Ziel = {target:.2} (P_f = {pf_target:.2e}) für RC{}; C.7 mit Leiteinwirkung E{} (σ_E/σ_R = {ratio:.3}): α_R = {alpha_r:.2}, α_E = {alpha_lead:.2}, Begleiteinwirkung α_E = {alpha_second:.2}", form.beta, form.alpha_r, form.alpha_e[0], form.alpha_e[1], form.design_r, form.design_e[0], form.design_e[1], document.reliability_class.number(), lead + 1),
        ));
    if form.beta < target {
        builder = builder.remedy(match required_resistance_mean_multi((analysis.resistance_distribution.as_str(), analysis.resistance_mean, analysis.resistance_sd), &actions, target) {
            Some(needed) => Remedy::at_least(
                subject,
                q_force(analysis.resistance_mean),
                q_force(needed),
                copy(format!("Raise the resistance mean μ_R to at least {:.1} kN (solved on the multi-action FORM run).", needed / 1000.0), format!("Mittelwert des Widerstands μ_R auf mindestens {:.1} kN erhöhen (am Mehr-Einwirkungs-FORM gelöst).", needed / 1000.0)),
            ),
            None => Remedy::advisory(subject, copy("No resistance mean within the FORM search range reaches the target β: reduce the scatter σ_R, σ_E1 or σ_E2, or change the distributions.", "Kein Widerstandsmittelwert im FORM-Suchbereich erreicht das Ziel-β: Streuung σ_R, σ_E1 oder σ_E2 verringern oder die Verteilungen ändern.")),
        });
    }
    report.push(builder.build());
}

/// 🧮️ Annex C.7 from the FORM run: whether the fixed sensitivity factors apply and the partial factors γ_R = R_k / R* and γ_E = E* / E_k of the design point (R_k the 5 % fractile, E_k the 98 % fractile).
fn push_annex_c_derived(report: &mut CheckReport, document: &En1990Snapshot, analysis: &ReliabilityAnalysis, form: &FormResult) {
    let (Ok(resistance), Ok(action)) = (Dist::parse(analysis.resistance_distribution.as_str(), analysis.resistance_mean, analysis.resistance_sd), Dist::parse(analysis.action_distribution.as_str(), analysis.action_mean, analysis.action_sd)) else { return };
    let subject = SubjectRef::new(&analysis.id, format!("reliabilityAnalyses[id={}].actionSd", analysis.id), copy(format!("Reliability analysis {}", analysis.id), format!("Zuverlässigkeitsanalyse {}", analysis.id)));
    let clause = ClauseId::new("EN 1990", "Annex C", "C.7");
    let ratio = analysis.action_sd / analysis.resistance_sd;
    let (alpha_r, alpha_e) = annex_c_alphas(document.annex, analysis.action_sd, analysis.resistance_sd);
    let (lo, hi) = (ndp(document.annex, "ndp.en1990.form.ratio_lo"), ndp(document.annex, "ndp.en1990.form.ratio_hi"));
    let inside = ratio > lo && ratio < hi;
    let mut alphas = CheckResult::assess(format!("en1990.annex-c.alpha.{}", analysis.id), "EN 1990 Annex C", clause.clone(), subject.clone(), copy("Annex C.7 sensitivity factors", "Anhang C.7 Wichtungsfaktoren"))
        .informative(q_dim(ratio))
        .annex(document.annex)
        .explanation(copy(
            format!("σ_E/σ_R = {ratio:.3} ({} the range {lo} to {hi}): C.7 uses α_R = {alpha_r:.2}, α_E = {alpha_e:.2}; FORM gives α_R = {:.2}, α_E = {:.2}.", if inside { "inside" } else { "outside" }, form.alpha_r, form.alpha_e),
            format!("σ_E/σ_R = {ratio:.3} ({} des Bereichs {lo} bis {hi}): C.7 verwendet α_R = {alpha_r:.2}, α_E = {alpha_e:.2}; FORM liefert α_R = {:.2}, α_E = {:.2}.", if inside { "innerhalb" } else { "außerhalb" }, form.alpha_r, form.alpha_e),
        ));
    if !inside {
        alphas = alphas.status(CheckStatus::Warning);
    }
    report.push(alphas.ndp_reads().build());
    let (r_k, e_k) = (resistance.fractile(0.05), action.fractile(0.98));
    let id = format!("en1990.annex-c.partial-factors.{}", analysis.id);
    let title = copy("Partial factors from the design point γ_R·γ_E", "Teilsicherheitsbeiwerte aus dem Bemessungspunkt γ_R·γ_E");
    if !(r_k.is_finite() && e_k.is_finite() && r_k > 0.0 && e_k > 0.0 && form.design_r > 0.0 && form.design_e > 0.0) {
        return report.push(not_applicable(id, "EN 1990 Annex C", clause, subject, title, document.annex, copy("Characteristic or design values are not positive.", "Charakteristische oder Bemessungswerte sind nicht positiv.")));
    }
    let (gamma_r, gamma_e) = (r_k / form.design_r, form.design_e / e_k);
    report.push(
        CheckResult::assess(id, "EN 1990 Annex C", clause, subject, title)
            .informative(q_dim(gamma_r * gamma_e))
            .annex(document.annex)
            .explanation(copy(
                format!("γ_R = R_k / R* = {r_k:.1} / {:.1} = {gamma_r:.3}, γ_E = E* / E_k = {:.1} / {e_k:.1} = {gamma_e:.3}, product {:.3}.", form.design_r, form.design_e, gamma_r * gamma_e),
                format!("γ_R = R_k / R* = {r_k:.1} / {:.1} = {gamma_r:.3}, γ_E = E* / E_k = {:.1} / {e_k:.1} = {gamma_e:.3}, Produkt {:.3}.", form.design_r, form.design_e, gamma_r * gamma_e),
            ))
            .build(),
    );
}
//#endregion 🔖️ReliabilityAnalysis

//#region 🔖️TestSeries
/// 🧪️ Annex D.7.2 design value from a test series versus the value the structure relies on.
pub(super) fn push_test_series(report: &mut CheckReport, document: &En1990Snapshot, series: &TestSeries) {
    let id = format!("en1990.annex-d.{}", series.id);
    let clause = ClauseId::new("EN 1990", "Annex D", "D.7.2");
    let subject = SubjectRef::new(&series.id, format!("testSeries[id={}].required", series.id), copy(format!("Test series {} ({})", series.id, series.quantity), format!("Versuchsreihe {} ({})", series.id, series.quantity)));
    let title = copy("Design value from tests X_d = η_d·(m − k_n·s)", "Bemessungswert aus Versuchen X_d = η_d·(m − k_n·s)");
    match test_design_value(series.distribution.as_str(), series.sample_size, series.mean, series.std_dev, series.variation_known, series.conversion_factor) {
        Err(reason) => report.push(not_applicable(id, "EN 1990 Annex D", clause, subject, title, document.annex, { let r = semio_s_plugin_norm_compliance::explain_reason(&reason); copy(format!("Test evaluation indeterminate: {}.", r.en), format!("Versuchsauswertung unbestimmt: {}.", r.de)) })),
        Ok(design) => {
            let kn = k_n(f64::from(series.sample_size), series.variation_known > 0.0).unwrap_or(f64::NAN);
            let mut builder = CheckResult::assess(id, "EN 1990 Annex D", clause, subject.clone(), title)
                .minimum(q_dim(design), q_dim(series.required))
                .annex(document.annex)
                .explanation(copy(
                    format!("X_d = {design:.4} (n = {}, k_n = {kn:.2}, m = {:.4}, s = {:.4}) vs required {:.4}", series.sample_size, series.mean, series.std_dev, series.required),
                    format!("X_d = {design:.4} (n = {}, k_n = {kn:.2}, m = {:.4}, s = {:.4}) gegenüber gefordert {:.4}", series.sample_size, series.mean, series.std_dev, series.required),
                ));
            if design < series.required {
                builder = builder
                    .remedy(Remedy::at_most(
                        subject.clone(),
                        q_dim(series.required),
                        q_dim(design),
                        copy(format!("Rely on at most X_d = {design:.4} (lowers the value the structure relies on)."), format!("Höchstens X_d = {design:.4} ansetzen (senkt den Wert, auf den sich das Tragwerk stützt).")),
                    ))
                    .remedy(Remedy::advisory(subject, copy("Or test more specimens and enter the new results (n, m, s) of the series.", "Oder mehr Proben prüfen und die neuen Ergebnisse (n, m, s) der Reihe eintragen.")));
            }
            report.push(builder.build());
            if let Some(kd) = k_d_n(f64::from(series.sample_size), series.variation_known > 0.0) {
                let direct = series.conversion_factor * series.mean * (1.0 - kd * series.variation_known);
                report.push(
                    CheckResult::assess(format!("en1990.annex-d.{}.direct", series.id), "EN 1990 Annex D", ClauseId::new("EN 1990", "Annex D", "D.7.2 Table D2"), subject.clone(), copy("Design value directly with k_d,n (Table D2)", "Bemessungswert direkt mit k_d,n (Tabelle D2)"))
                        .informative(q_dim(direct))
                        .annex(document.annex)
                        .explanation(copy(
                            format!("X_d = η_d m (1 − k_d,n V_X) = {direct:.4} with k_d,n = {kd:.2} (V_X known, normal quantity)"),
                            format!("X_d = η_d m (1 − k_d,n V_X) = {direct:.4} mit k_d,n = {kd:.2} (V_X bekannt, normalverteilte Größe)"),
                        ))
                        .build(),
                );
            }
            if series.sample_size < 3 {
                report.push(
                    CheckResult::assess(format!("en1990.annex-d.{}.sample", series.id), "EN 1990 Annex D", ClauseId::new("EN 1990", "Annex D", "D.7.2 Table D1"), SubjectRef::new(&series.id, format!("testSeries[id={}].sampleSize", series.id), copy(format!("Test series {}", series.id), format!("Versuchsreihe {}", series.id))), copy("Number of tests n of Table D1", "Versuchsanzahl n nach Tabelle D1"))
                        .informative(q_dim(f64::from(series.sample_size)))
                        .status(CheckStatus::Warning)
                        .annex(document.annex)
                        .explanation(copy(
                            format!("Only n = {} tests: the design value relies on the assumed V_X = {:.3}; fewer than 3 tests are not representative.", series.sample_size, series.variation_known),
                            format!("Nur n = {} Versuche: der Bemessungswert beruht auf dem angenommenen V_X = {:.3}; weniger als 3 Versuche sind nicht repräsentativ.", series.sample_size, series.variation_known),
                        ))
                        .remedy(Remedy::advisory(
                            SubjectRef::new(&series.id, format!("testSeries[id={}].sampleSize", series.id), copy(format!("Test series {}", series.id), format!("Versuchsreihe {}", series.id))),
                            copy("Test at least 3 specimens and enter their results (n, m, s); the sample size alone must not be raised without data.", "Mindestens 3 Proben prüfen und deren Ergebnisse (n, m, s) eintragen; der Stichprobenumfang allein darf nicht ohne Daten erhöht werden."),
                        ))
                        .build(),
                );
            }
        }
    }
}

/// 🧪️ Annex D.7.2 standard evaluation of the test pairs of a series: the design resistance r_d = η_d r_k against the value the structure relies on; silent while the series carries no pairs.
pub(super) fn push_test_standard(report: &mut CheckReport, document: &En1990Snapshot, series: &TestSeries) {
    if series.pairs.trim().is_empty() {
        return;
    }
    let id = format!("en1990.annex-d.{}.standard", series.id);
    let clause = ClauseId::new("EN 1990", "Annex D", "D.7.2");
    let subject = SubjectRef::new(&series.id, format!("testSeries[id={}].required", series.id), copy(format!("Test series {} ({})", series.id, series.quantity), format!("Versuchsreihe {} ({})", series.id, series.quantity)));
    let title = copy("Design resistance from test pairs r_d = η_d·r_k", "Bemessungswiderstand aus Versuchspaaren r_d = η_d·r_k");
    let pairs: Result<Vec<(f64, f64)>, String> = series.pairs.split(';').filter(|row| !row.trim().is_empty()).map(|row| {
        let (e, t) = row.split_once(':').ok_or_else(|| format!("pair {row} needs r_e:r_t"))?;
        Ok((e.trim().parse::<f64>().map_err(|e| e.to_string())?, t.trim().parse::<f64>().map_err(|e| e.to_string())?))
    }).collect();
    let fit = pairs.and_then(|p| standard_evaluation(&p, series.theory_variation, series.theory_mean).map(|fit| (p.len(), fit)));
    match fit {
        Err(reason) => report.push(not_applicable(id, "EN 1990 Annex D", clause, subject, title, document.annex, { let r = semio_s_plugin_norm_compliance::explain_reason(&reason); copy(format!("Standard evaluation indeterminate: {}.", r.en), format!("Standardauswertung unbestimmt: {}.", r.de)) })),
        Ok((n, fit)) => {
            let design = series.conversion_factor * fit.r_k;
            let mut builder = CheckResult::assess(id, "EN 1990 Annex D", clause, subject.clone(), title)
                .minimum(q_dim(design), q_dim(series.required))
                .annex(document.annex)
                .explanation(copy(
                    format!("n = {n}, b = {:.4}, V_δ = {:.4}, V_r = {:.4}, k_n = {:.2}, r_k = {:.4}, r_d = η_d r_k = {design:.4} vs required {:.4}", fit.b, fit.v_delta, fit.v_r, fit.k_n, fit.r_k, series.required),
                    format!("n = {n}, b = {:.4}, V_δ = {:.4}, V_r = {:.4}, k_n = {:.2}, r_k = {:.4}, r_d = η_d r_k = {design:.4} gegenüber gefordert {:.4}", fit.b, fit.v_delta, fit.v_r, fit.k_n, fit.r_k, series.required),
                ));
            if design < series.required {
                builder = builder
                    .remedy(Remedy::at_most(subject.clone(), q_dim(series.required), q_dim(design), copy(format!("Rely on at most r_d = {design:.4} (lowers the value the structure relies on)."), format!("Höchstens r_d = {design:.4} ansetzen (senkt den Wert, auf den sich das Tragwerk stützt)."))))
                    .remedy(Remedy::advisory(subject, copy("Or add test pairs r_e:r_t to the series and re-evaluate.", "Oder Versuchspaare r_e:r_t zur Reihe hinzufügen und neu auswerten.")));
            }
            report.push(builder.build());
        }
    }
}
//#endregion 🔖️TestSeries
