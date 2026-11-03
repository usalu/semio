//! 📊️ Ranking and reliability: every combination ranked by its worst utilisation with the governing one highlighted, and the reliability index β against K_FI of the classes RC1 to RC3 with the project's design point. Both are pure functions of the document and its report, so a drag that changes a magnitude or the class redraws them in the same frame. See [`super::diagrams`].

use super::*;
use super::super::norm::{CheckResult, ReliabilityClass};

/// 🏁️ The worst check of every combination column, ranked by utilisation: the left bar is the governing combination (hatched, diamond marker bound to its check), the framed bar is the active one and the dashed line is the limit u = 1. Columns without a check are left out.
pub(super) fn ranking(report: &CheckReport, columns: &[Column], active: Option<&Column>) -> Option<Diagram2d> {
    let mut ranked: Vec<(usize, f64, &CheckResult)> = columns.iter().enumerate().filter_map(|(i, c)| worst_of_column(report, columns, c).map(|w| (i, w.utilization.max(0.0), w))).collect();
    if ranked.is_empty() {
        return None;
    }
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    let top = ranked[0].1.max(1.0) * 1.1;
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (rank, (at, u, check)) in ranked.iter().enumerate() {
        let column = &columns[*at];
        let x = rank as f64;
        let governing = rank == 0;
        let (family_en, family_de) = column.family.title();
        let label = loc(&format!("{family_en} · {} · lead {}: {:.0} %", column.combination.equation.label(), column.leading(), u * 100.0), &format!("{family_de} · {} · führend {}: {:.0} %", column.combination.equation.label(), column.leading(), u * 100.0));
        series.push(Series::Polygon { id: format!("rank-{rank}"), label: label.clone(), ring: rect(x + CELL_INSET, 0.0, x + 1.0 - CELL_INSET, u.max(top * 1e-3)), fill: Fill { tone: utilisation_tone(*u), opacity: if governing { 0.95 } else { 0.5 } }, hatch: governing.then_some(Hatch::Diagonal) });
        markers.push(Marker {
            id: format!("ranked-{}", column.id),
            at: p(x + 0.5, *u),
            shape: if governing { MarkerShape::Diamond } else { MarkerShape::Square },
            tone: if active.is_some_and(|a| a.id == column.id) { Tone::Error } else { utilisation_tone(*u) },
            label: loc(&format!("{}{}", label.en, if governing { " (governing)" } else { "" }), &format!("{}{}", label.de, if governing { " (maßgebend)" } else { "" })),
            utilisation: Some(*u),
            bound_check: Some(check.id.clone()),
            binding: None,
        });
    }
    series.push(Series::Limit { id: "limit".into(), label: loc("Limit u = 1", "Grenze u = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    if let Some(rank) = active.and_then(|a| ranked.iter().position(|(at, _, _)| columns[*at].id == a.id)) {
        let x = rank as f64;
        series.push(Series::Polyline { id: "active-rank".into(), label: loc("Active combination", "Aktive Kombination"), points: vec![p(x, 0.0), p(x + 1.0, 0.0), p(x + 1.0, top), p(x, top), p(x, 0.0)], style: stroke(Tone::Error, 3.0, Dash::Solid) });
    }
    let count = ranked.len();
    Some(Diagram2d {
        id: "combination-ranking:document".into(),
        title: loc("Combinations ranked by utilisation", "Kombinationen nach Ausnutzung geordnet"),
        description: loc(
            "The worst utilisation of every generated combination, largest first; the hatched bar with the diamond is the governing combination, the framed bar the active one. Dragging any action magnitude re-evaluates the affected checks and re-ranks the bars in the same frame. Markers select the underlying check and its combination.",
            "Die größte Ausnutzung jeder erzeugten Kombination, größte zuerst; der schraffierte Balken mit der Raute ist die maßgebende Kombination, der umrahmte Balken die aktive. Das Ziehen einer Einwirkungsgröße wertet die betroffenen Nachweise neu aus und ordnet die Balken im selben Bild neu. Marken wählen den zugehörigen Nachweis und seine Kombination.",
        ),
        axes: vec![axis("Rank", "Rang", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, count as f64)), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, top))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(selector_ref()),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Rank", "Rang", "1", (0..count).map(|i| i as f64).collect()),
                column("Combination index", "Kombinationsindex", "1", ranked.iter().map(|(at, _, _)| *at as f64).collect()),
                column("Utilisation", "Ausnutzung", "1", ranked.iter().map(|(_, u, _)| *u).collect()),
                column("Governing", "Maßgebend", "1", (0..count).map(|i| f64::from(u8::from(i == 0))).collect()),
            ],
        },
    })
}

/// 🛡️ The target reliability index β of RC1 to RC3 over K_FI (Table B3 against Table B2 at the reference period) with the declared class marked and the project's design point (K_FI of the declared class, computed β) on it; the shaded area below the line is a β below the target. The design point is bound to `betaComputed`, so a drag in β and a change of the class both move it live.
pub(super) fn beta_k_fi(document: &En1990Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let classes: Vec<(u8, f64, f64)> = (1..=3u8).filter_map(|rc| Some((rc, ReliabilityClass::from_number(rc)?.k_fi(document.annex), target_reliability_index_for(rc, document.reference_period_years)?))).collect();
    if classes.len() < 2 {
        return None;
    }
    let mut ordered = classes.clone();
    ordered.sort_by(|a, b| a.1.total_cmp(&b.1));
    let (lo, hi) = (ordered[0].1 - 0.05, ordered[ordered.len() - 1].1 + 0.05);
    let declared = document.reliability_class.number();
    let (k_declared, target) = classes.iter().find(|(rc, _, _)| *rc == declared).map(|(_, k, t)| (*k, *t))?;
    let beta = document.beta_computed.max(0.0);
    let check = report.checks.iter().find(|c| c.id == "en1990.annex-c.beta");
    let line: Vec<Vec2> = ordered.iter().map(|(_, k, t)| p(*k, t.clamp(0.0, BETA_MAX))).collect();
    let mut ring = vec![p(ordered[0].1, 0.0)];
    ring.extend(line.iter().cloned());
    ring.push(p(ordered[ordered.len() - 1].1, 0.0));
    let series = vec![
        Series::Polygon { id: "below-target".into(), label: loc("β below the target", "β unter dem Zielwert"), ring, fill: Fill { tone: Tone::Error, opacity: 0.12 }, hatch: Some(Hatch::Dots) },
        Series::Polyline { id: "target".into(), label: loc("Target β of RC1 to RC3", "Zielwert β von RC1 bis RC3"), points: line, style: stroke(Tone::Info, 2.5, Dash::Solid) },
        Series::Limit { id: "declared".into(), label: loc(&format!("Declared class RC{declared}: K_FI = {k_declared:.2}"), &format!("Gewählte Klasse RC{declared}: K_FI = {k_declared:.2}")), axis: AxisRef::X, value: k_declared, style: stroke(Tone::Warning, 1.5, Dash::Dotted) },
    ];
    let mut binding = FieldBinding::new("betaComputed", Affine::IDENTITY, "1");
    binding.quantise = Some(Quantise::Round { step: 0.01 });
    let mut markers: Vec<Marker> = classes
        .iter()
        .map(|(rc, k, t)| Marker {
            id: format!("reference:class-rc{rc}"),
            at: p(*k, t.clamp(0.0, BETA_MAX)),
            shape: MarkerShape::Square,
            tone: if *rc == declared { Tone::Warning } else { Tone::Neutral },
            label: loc(&format!("RC{rc}: K_FI = {k:.2}, target β = {t:.2}"), &format!("RC{rc}: K_FI = {k:.2}, Zielwert β = {t:.2}")),
            utilisation: None,
            bound_check: None,
            binding: None,
        })
        .collect();
    markers.push(Marker {
        id: "design-point".into(),
        at: p(k_declared, beta.min(BETA_MAX)),
        shape: MarkerShape::Diamond,
        tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
        label: loc(&format!("Design point: RC{declared}, K_FI = {k_declared:.2}, β = {beta:.2} against {target:.2}"), &format!("Bemessungspunkt: RC{declared}, K_FI = {k_declared:.2}, β = {beta:.2} gegenüber {target:.2}")),
        utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u >= 0.0),
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(binding),
    });
    let applied = Some(gamma_g(document.annex, document.structure_kind.domain(), FactorSet::StrB).0);
    let mut table = vec![column("Class", "Klasse", "1", ordered.iter().map(|(rc, _, _)| f64::from(*rc)).collect()), column("K_FI", "K_FI", "1", ordered.iter().map(|(_, k, _)| *k).collect()), column("Target β", "Zielwert β", "1", ordered.iter().map(|(_, _, t)| *t).collect())];
    if let Some(sup) = applied {
        table.push(column("K_FI·γ_G,sup", "K_FI·γ_G,sup", "1", ordered.iter().map(|(_, k, _)| k * sup).collect()));
    }
    Some(Diagram2d {
        id: "beta-k-fi:document".into(),
        title: loc("Reliability index against K_FI", "Zuverlässigkeitsindex gegenüber K_FI"),
        description: loc(
            "The target reliability index β of the classes RC1 to RC3 (EN 1990 Annex B Table B2 at the reference period) over the factor K_FI that scales the permanent and variable partial factors (Table B3); the diamond is the project: K_FI of the declared class against the computed β of Annex C. Below the line the reliability is insufficient. Changing the class moves the diamond along the line, dragging it changes β.",
            "Der Zielwert des Zuverlässigkeitsindex β der Klassen RC1 bis RC3 (EN 1990 Anhang B Tabelle B2 beim Bezugszeitraum) über dem Faktor K_FI, der die Teilsicherheitsbeiwerte ständiger und veränderlicher Einwirkungen skaliert (Tabelle B3); die Raute ist das Projekt: K_FI der gewählten Klasse gegenüber dem berechneten β nach Anhang C. Unter der Linie ist die Zuverlässigkeit unzureichend. Ein Klassenwechsel bewegt die Raute entlang der Linie, Ziehen ändert β.",
        ),
        axes: vec![axis("Factor K_FI", "Faktor K_FI", QuantityKind::Dimensionless, "1", AxisScale::Linear, (lo, hi)), axis("Reliability index β", "Zuverlässigkeitsindex β", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, BETA_MAX))],
        series,
        markers,
        annotations: vec![DiagramNote { id: "margin".into(), at: p(lo + 0.01, BETA_MAX - 0.4), text: loc(&format!("RC{declared}: β = {beta:.2}, target {target:.2}, margin {:+.2}", beta - target), &format!("RC{declared}: β = {beta:.2}, Zielwert {target:.2}, Reserve {:+.2}", beta - target)), tone: if beta >= target { Tone::Info } else { Tone::Error } }],
        focus: None,
        legend: true,
        table_twin: DataTable { columns: table },
    })
}

const SLS_EQUATIONS: [(&str, &str, &str); 3] = [("6.14b", "Characteristic (6.14b)", "Charakteristisch (6.14b)"), ("6.15b", "Frequent (6.15b)", "Häufig (6.15b)"), ("6.16b", "Quasi-permanent (6.16b)", "Quasi-ständig (6.16b)")];

/// 📏️ The three serviceability combinations of a member side by side: the utilisation E_d / C_d of the characteristic (6.14b), frequent (6.15b) and quasi-permanent (6.16b) combination against its own criterion C_d, bars left to right with the diamond bound to the check; the dashed line is u = 1 and the table twin lists E_d and C_d in N. Combinations without a criterion C_d have no bar.
pub(super) fn sls_combinations(report: &CheckReport, member: &Member) -> Option<Diagram2d> {
    let rows: Vec<(usize, &str, &str, &CheckResult)> = SLS_EQUATIONS.iter().enumerate().filter_map(|(i, (label, en, de))| report.checks.iter().find(|c| c.id == format!("en1990.sls.{}.{label}", member.id) && c.utilization.is_finite() && c.status != super::super::norm::CheckStatus::NotApplicable).map(|c| (i, *en, *de, c))).collect();
    if rows.is_empty() {
        return None;
    }
    let top = rows.iter().map(|(_, _, _, c)| c.utilization.max(0.0)).fold(1.0_f64, f64::max) * 1.1;
    let mut series: Vec<Series> = Vec::new();
    let mut markers: Vec<Marker> = Vec::new();
    for (i, en, de, check) in &rows {
        let (x, u) = (*i as f64, check.utilization.max(0.0));
        let label = loc(&format!("{en}: {:.0} %", u * 100.0), &format!("{de}: {:.0} %", u * 100.0));
        series.push(Series::Polygon { id: format!("sls-{i}"), label: label.clone(), ring: rect(x + CELL_INSET, 0.0, x + 1.0 - CELL_INSET, u.max(top * 1e-3)), fill: Fill { tone: utilisation_tone(u), opacity: 0.6 }, hatch: None });
        markers.push(Marker { id: format!("sls-marker-{i}"), at: p(x + 0.5, u), shape: MarkerShape::Diamond, tone: utilisation_tone(u), label, utilisation: Some(u), bound_check: Some(check.id.clone()), binding: None });
    }
    series.push(Series::Limit { id: "limit".into(), label: loc("Limit u = 1", "Grenze u = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    Some(Diagram2d {
        id: format!("sls-combinations:{}", member.id),
        title: loc(&format!("Serviceability combinations – {}", member.label_en), &format!("Gebrauchstauglichkeitskombinationen – {}", member.label_de)),
        description: loc(
            "The characteristic, frequent and quasi-permanent combination of the member (EN 1990 6.5.3, (6.14b) to (6.16b)) against their criteria C_d; the bars shrink from left to right because the combination factors psi_1 and psi_2 reduce the accompanying values.",
            "Die charakteristische, häufige und quasi-ständige Kombination des Bauteils (EN 1990 6.5.3, (6.14b) bis (6.16b)) gegenüber ihren Kriterien C_d; die Balken werden von links nach rechts kleiner, weil die Kombinationsbeiwerte psi_1 und psi_2 die begleitenden Werte verringern.",
        ),
        axes: vec![axis("Combination", "Kombination", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, SLS_EQUATIONS.len() as f64)), axis("Utilisation E_d / C_d", "Ausnutzung E_d / C_d", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, top))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Combination index", "Kombinationsindex", "1", rows.iter().map(|(i, _, _, _)| *i as f64).collect()),
                column("Effect E_d", "Beanspruchung E_d", "N", rows.iter().map(|(_, _, _, c)| c.computed.value).collect()),
                column("Criterion C_d", "Kriterium C_d", "N", rows.iter().map(|(_, _, _, c)| c.limit.value).collect()),
                column("Utilisation", "Ausnutzung", "1", rows.iter().map(|(_, _, _, c)| c.utilization).collect()),
            ],
        },
    })
}

/// 📊️ The sensitivity factors α of one FORM analysis as bars: the resistance upward, every action downward (g = R − ΣE_i, so Σα² = 1); each diamond is bound to the analysis check, so a drag on a mean or a standard deviation in the viewport moves the bars.
pub(super) fn form_bars(document: &En1990Snapshot, focus: Option<&str>) -> Vec<Diagram2d> {
    let selected = |a: &&ReliabilityAnalysis| focus.is_none_or(|f| f == a.id || f == document.project_id || (!a.member_id.is_empty() && f == a.member_id));
    document.reliability_analyses.iter().filter(selected).filter_map(|a| {
        let mut actions = vec![("E₁", a.action_distribution.as_str(), a.action_mean, a.action_sd)];
        if a.second_action_sd > 0.0 {
            actions.push(("E₂", a.second_action_distribution.as_str(), a.second_action_mean, a.second_action_sd));
        }
        let form = analysis_form_multi((a.resistance_distribution.as_str(), a.resistance_mean, a.resistance_sd), &actions.iter().map(|(_, d, m, s)| (*d, *m, *s)).collect::<Vec<_>>()).ok()?;
        let rows: Vec<(String, f64, f64)> = std::iter::once(("R".to_string(), form.alpha_r, form.design_r)).chain(actions.iter().zip(form.alpha_e.iter().zip(&form.design_e)).map(|((name, ..), (alpha, design))| (name.to_string(), *alpha, *design))).collect();
        let (mut series, mut markers) = (Vec::new(), Vec::new());
        for (i, (name, alpha, _)) in rows.iter().enumerate() {
            let x = i as f64;
            let tone = if *alpha >= 0.0 { Tone::Info } else { Tone::Warning };
            let label = loc(&format!("α_{name} = {alpha:.2}"), &format!("α_{name} = {alpha:.2}"));
            let (lo, hi) = if *alpha >= 0.0 { (0.0, *alpha) } else { (*alpha, 0.0) };
            series.push(Series::Polygon { id: format!("alpha-{i}"), label: label.clone(), ring: rect(x + CELL_INSET, lo, x + 1.0 - CELL_INSET, hi.max(lo + 1e-3)), fill: Fill { tone, opacity: 0.6 }, hatch: None });
            markers.push(Marker { id: format!("alpha-marker-{i}"), at: p(x + 0.5, *alpha), shape: MarkerShape::Diamond, tone, label, utilisation: None, bound_check: Some(format!("en1990.annex-c.form.{}", a.id)), binding: None });
        }
        series.push(Series::Limit { id: "zero".into(), label: loc("α = 0", "α = 0"), axis: AxisRef::Y, value: 0.0, style: stroke(Tone::Neutral, 1.0, Dash::Solid) });
        let count = rows.len() as f64;
        Some(Diagram2d {
            id: format!("form-bars:{}", a.id),
            title: loc(&format!("Sensitivity factors α – {}", a.id), &format!("Wichtungsfaktoren α – {}", a.id)),
            description: loc(
                "The FORM sensitivity factors of the resistance R (upward) and of every action E_i (downward) of g = R − ΣE_i with Σα² = 1: the longest bar is the variable that governs the reliability index, the others show how much a change of their mean or scatter matters.",
                "Die FORM-Wichtungsfaktoren des Widerstands R (nach oben) und jeder Einwirkung E_i (nach unten) von g = R − ΣE_i mit Σα² = 1: Der längste Balken ist die Variable, die den Zuverlässigkeitsindex bestimmt; die übrigen zeigen, wie viel eine Änderung ihres Mittelwerts oder ihrer Streuung ausmacht.",
            ),
            axes: vec![axis("Basic variable", "Basisvariable", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, count)), axis("Sensitivity factor α", "Wichtungsfaktor α", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-1.0, 1.0))],
            series,
            markers,
            annotations: vec![DiagramNote { id: "beta".into(), at: p(0.1, 0.9), text: loc(&format!("β = {:.2}, P_f = {:.2e}", form.beta, form.failure_probability()), &format!("β = {:.2}, P_f = {:.2e}", form.beta, form.failure_probability())), tone: Tone::Neutral }],
            focus: Some(EntityRef::new(EntityKind::Plate, &a.id, format!("reliabilityAnalyses[id={}]", a.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Variable index", "Variablenindex", "1", (0..rows.len()).map(|i| i as f64).collect()), column("Sensitivity factor α", "Wichtungsfaktor α", "1", rows.iter().map(|r| r.1).collect()), column("Design point X*", "Bemessungspunkt X*", "N", rows.iter().map(|r| r.2).collect())] },
        })
    }).collect()
}

/// 🧪️ The test results of one series as a scatter of theoretical against experimental resistance with the fitted line r_e = b r_t of Annex D.7.2 and, when g_rt(X_m) is given, the characteristic r_k and the design value η_d r_k as markers bound to the standard evaluation check.
pub(super) fn test_pairs(document: &En1990Snapshot, focus: Option<&str>) -> Vec<Diagram2d> {
    document.test_series.iter().filter(|s| focus.is_none_or(|f| f == s.id || f == document.project_id)).filter_map(|s| {
        let results: Vec<(f64, f64)> = s.pairs.split(';').filter(|row| !row.trim().is_empty()).map(|row| {
            let (e, t) = row.split_once(':')?;
            Some((e.trim().parse::<f64>().ok()?, t.trim().parse::<f64>().ok()?))
        }).collect::<Option<_>>()?;
        let denominator: f64 = results.iter().map(|(_, t)| t * t).sum();
        if results.len() < 2 || !(denominator > 0.0) {
            return None;
        }
        let b = results.iter().map(|(e, t)| e * t).sum::<f64>() / denominator;
        let fit = if s.theory_mean > 0.0 { standard_evaluation(&results, s.theory_variation, s.theory_mean).ok() } else { None };
        let design = fit.map(|f| s.conversion_factor * f.r_k);
        let xmax = results.iter().map(|(_, t)| *t).chain([s.theory_mean]).fold(0.0, f64::max) * 1.1;
        let ymax = results.iter().map(|(e, _)| *e).chain([b * xmax, s.required, fit.map_or(0.0, |f| f.b * s.theory_mean)]).fold(0.0, f64::max) * 1.1;
        let points: Vec<ScatterPoint> = results.iter().enumerate().map(|(i, (e, t))| ScatterPoint { at: p(*t, *e), tone: Tone::Info, label: Some(loc(&format!("Test {}: r_t = {t:.3}, r_e = {e:.3}", i + 1), &format!("Versuch {}: r_t = {t:.3}, r_e = {e:.3}", i + 1))) }).collect();
        let mut series = vec![
            Series::Scatter { id: "results".into(), label: loc("Test results (r_t, r_e)", "Versuchsergebnisse (r_t, r_e)"), points },
            Series::Polyline { id: "fit".into(), label: loc(&format!("Fit r_e = b·r_t, b = {b:.3}"), &format!("Ausgleichsgerade r_e = b·r_t, b = {b:.3}")), points: vec![p(0.0, 0.0), p(xmax, b * xmax)], style: stroke(Tone::Warning, 2.0, Dash::Solid) },
        ];
        if s.required > 0.0 {
            series.push(Series::Limit { id: "required".into(), label: loc(&format!("Required value {:.3}", s.required), &format!("Geforderter Wert {:.3}", s.required)), axis: AxisRef::Y, value: s.required, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
        }
        let check = format!("en1990.annex-d.{}.standard", s.id);
        let mut markers = Vec::new();
        if let (Some(f), Some(d)) = (fit, design) {
            let tone = if d >= s.required { Tone::Info } else { Tone::Error };
            markers.push(Marker { id: "r-k".into(), at: p(s.theory_mean, f.r_k), shape: MarkerShape::Diamond, tone, label: loc(&format!("r_k = {:.3} (k_n = {:.2})", f.r_k, f.k_n), &format!("r_k = {:.3} (k_n = {:.2})", f.r_k, f.k_n)), utilisation: None, bound_check: Some(check.clone()), binding: None });
            markers.push(Marker { id: "r-d".into(), at: p(s.theory_mean, d), shape: MarkerShape::Circle, tone, label: loc(&format!("r_d = η_d·r_k = {d:.3}"), &format!("r_d = η_d·r_k = {d:.3}")), utilisation: None, bound_check: Some(check), binding: None });
        }
        Some(Diagram2d {
            id: format!("test-pairs:{}", s.id),
            title: loc(&format!("Test results and fit – {}", s.id), &format!("Versuchsergebnisse und Ausgleichsgerade – {}", s.id)),
            description: loc(
                "Experimental against theoretical resistance of every specimen with the least-squares line r_e = b·r_t of EN 1990 Annex D.7.2; the diamond marks the characteristic resistance r_k at g_rt(X_m), the circle the design resistance η_d·r_k against the required value.",
                "Experimenteller gegenüber theoretischem Widerstand jeder Probe mit der Ausgleichsgeraden r_e = b·r_t nach EN 1990 Anhang D.7.2; die Raute markiert den charakteristischen Widerstand r_k bei g_rt(X_m), der Kreis den Bemessungswiderstand η_d·r_k gegenüber dem geforderten Wert.",
            ),
            axes: vec![axis("Theoretical resistance r_t", "Theoretischer Widerstand r_t", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, xmax)), axis("Experimental resistance r_e", "Experimenteller Widerstand r_e", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, ymax))],
            series,
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Plate, &s.id, format!("testSeries[id={}]", s.id))),
            legend: true,
            table_twin: DataTable { columns: vec![column("Theoretical resistance r_t", "Theoretischer Widerstand r_t", "1", results.iter().map(|(_, t)| *t).collect()), column("Experimental resistance r_e", "Experimenteller Widerstand r_e", "1", results.iter().map(|(e, _)| *e).collect()), column("Fitted b·r_t", "Angepasst b·r_t", "1", results.iter().map(|(_, t)| b * t).collect())] },
        })
    }).collect()
}
