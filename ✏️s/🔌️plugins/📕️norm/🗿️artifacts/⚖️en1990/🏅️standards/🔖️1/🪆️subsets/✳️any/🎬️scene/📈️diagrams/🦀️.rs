//! 📈️ Diagrams: the combination matrix (actions × combinations with γ·ψ and the leading action), the ψ chart per category, the β–P_f reliability chart with the RC targets, the FORM design point,
//! the Annex D k_n factors, the fatigue S–N line with the damage of a member and the three serviceability combinations of a member. Every diagram has a table twin; markers bind to checks and to document leaves.

extern crate semio_s_artifact_norm_en1990_checks as checks;
extern crate semio_s_artifact_norm_en1990_model as model;
extern crate semio_s_artifact_norm_en1990_scene_combinations as combinations;
extern crate semio_s_artifact_norm_en1990_scene_kit as kit;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use checks::{analysis_form_multi, standard_evaluation, annex_c_alphas, en1993_endurance_cycles, k_n, spectrum_damage, test_design_value, phi_cdf, psi_entry_of, psi_lookup, resolve_psi_category, target_reliability_index_for, gamma_g, gamma_q, k_fi_for_rc, traffic_group, ActionClass, Domain, FactorSet};
use combinations::{column_for_check, Column};
use kit::COMBINATION_ENTITY;
use model::{En1990Snapshot, Member, ReliabilityAnalysis, TestSeries};
use norm::CheckReport;
use scene::projection_kit::diagram_parts::{column, stroke};
use scene::projection_kit::diagram_parts;
use scene::*;

const BETA_MAX: f64 = 7.0;
const BETA_STEP: f64 = 0.1;
const REFERENCE_CYCLES: f64 = 2.0e6;
const CIRCLE_POINTS: usize = 64;
const CELL_INSET: f64 = 0.04;
const ROW_LABEL_WIDTH: f64 = 2.5;
const KN_NODES: [f64; 11] = [3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0, 30.0, 50.0, 100.0];

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, scale: AxisScale, domain: (f64, f64)) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, scale, Some(domain))
}

fn p(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Vec2> {
    vec![p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)]
}

fn utilisation_tone(ratio: f64) -> Tone {
    scene::utilisation_tone(ratio)
}

fn class_tone(class: ActionClass) -> Tone {
    match class {
        ActionClass::Permanent | ActionClass::Prestress => Tone::Neutral,
        ActionClass::Variable => Tone::Info,
        ActionClass::Accidental => Tone::Error,
        ActionClass::Seismic => Tone::Warning,
    }
}

fn selector_ref() -> EntityRef {
    EntityRef::new(EntityKind::Group, COMBINATION_ENTITY, format!("combinations[id={COMBINATION_ENTITY}]"))
}

fn worst_of_column<'a>(report: &'a CheckReport, columns: &[Column], target: &Column) -> Option<&'a norm::CheckResult> {
    report.checks.iter().filter(|c| c.utilization.is_finite() && column_for_check(columns, c).is_some_and(|found| found.id == target.id)).max_by(|a, b| a.utilization.total_cmp(&b.utilization))
}

fn matrix(document: &En1990Snapshot, report: &CheckReport, columns: &[Column], active: Option<&Column>) -> Option<Diagram2d> {
    let actions = combinations::structure_actions(document);
    if columns.is_empty() || actions.is_empty() {
        return None;
    }
    let (n, rows) = (columns.len(), actions.len());
    let fmax = columns.iter().flat_map(|c| c.combination.terms.iter().map(|t| t.factor.abs())).fold(1.0, f64::max);
    let mut series = Vec::new();
    let mut notes = Vec::new();
    let mut markers = Vec::new();
    let worst_per_column: Vec<_> = columns.iter().map(|col| worst_of_column(report, columns, col)).collect();
    for (r, action) in actions.iter().enumerate() {
        let y = (rows - 1 - r) as f64;
        notes.push(DiagramNote { id: format!("row-{r}"), at: p(-ROW_LABEL_WIDTH + 0.1, y + 0.5), text: loc(&action.id, &action.id), tone: class_tone(action.class) });
        for (c, col) in columns.iter().enumerate() {
            let Some(f) = col.factor(&action.id).filter(|f| f.abs() > 1e-12) else { continue };
            let x = c as f64;
            series.push(Series::Polygon {
                id: format!("cell-{c}-{r}"),
                label: loc(&format!("{} in {} lead {}: {f:.3}", action.id, col.combination.equation.label(), col.leading()), &format!("{} in {} führend {}: {f:.3}", action.id, col.combination.equation.label(), col.leading())),
                ring: rect(x + CELL_INSET, y + CELL_INSET, x + 1.0 - CELL_INSET, y + 1.0 - CELL_INSET),
                fill: Fill { tone: class_tone(action.class), opacity: 0.15 + 0.75 * (f.abs() / fmax).min(1.0) },
                hatch: col.is_leading(&action.id).then_some(Hatch::Diagonal),
            });
            notes.push(DiagramNote { id: format!("f-{c}-{r}"), at: p(x + 0.5, y + 0.5), text: loc(&format!("{f:.2}"), &format!("{f:.2}")), tone: Tone::Neutral });
            if let Some(worst) = worst_per_column[c] {
                markers.push(Marker {
                    id: format!("pick-{c}-{r}"),
                    at: p(x + 0.5, y + 0.2),
                    shape: MarkerShape::Circle,
                    tone: Tone::Neutral,
                    label: loc(&format!("Select {} · {} · lead {}", action.id, col.combination.equation.label(), col.leading()), &format!("{} · {} · führend {} wählen", action.id, col.combination.equation.label(), col.leading())),
                    utilisation: Some(worst.utilization.max(0.0)),
                    bound_check: Some(worst.id.clone()),
                    binding: None,
                });
            }
        }
    }
    for (c, col) in columns.iter().enumerate() {
        let worst = worst_of_column(report, columns, col);
        let is_active = active.is_some_and(|a| a.id == col.id);
        markers.push(Marker {
            id: format!("combination-{}", col.id),
            at: p(c as f64 + 0.5, rows as f64 + 0.5),
            shape: MarkerShape::Square,
            tone: if is_active { Tone::Error } else { worst.map_or(Tone::Neutral, |w| utilisation_tone(w.utilization)) },
            label: loc(&format!("{} · {} · lead {}", col.family.title().0, col.combination.equation.label(), col.leading()), &format!("{} · {} · führend {}", col.family.title().1, col.combination.equation.label(), col.leading())),
            utilisation: worst.map(|w| w.utilization.max(0.0)),
            bound_check: worst.map(|w| w.id.clone()),
            binding: None,
        });
    }
    if let Some(at) = active.and_then(|a| columns.iter().position(|c| c.id == a.id)) {
        let x = at as f64;
        series.push(Series::Polyline { id: "active-column".into(), label: loc("Active combination", "Aktive Kombination"), points: vec![p(x, 0.0), p(x + 1.0, 0.0), p(x + 1.0, rows as f64), p(x, rows as f64), p(x, 0.0)], style: stroke(Tone::Error, 3.0, Dash::Solid) });
    }
    let mut table = vec![column("Combination index", "Kombinationsindex", "1", (0..n).map(|c| c as f64).collect())];
    table.extend(actions.iter().map(|a| column(&format!("γ·ψ of {}", a.id), &format!("γ·ψ von {}", a.id), "1", columns.iter().map(|c| c.factor(&a.id).unwrap_or(0.0)).collect())));
    Some(Diagram2d {
        id: "matrix:document".into(),
        title: loc("Combination matrix", "Kombinationsmatrix"),
        description: loc(
            "Actions against the generated combinations of EN 1990 6.10, 6.10a/b, 6.11b, 6.12b and 6.14b to 6.16b; each cell shows the total factor γ·ψ, the leading action is hatched and the active combination is framed. Square markers select a combination.",
            "Einwirkungen gegenüber den erzeugten Kombinationen nach EN 1990 6.10, 6.10a/b, 6.11b, 6.12b und 6.14b bis 6.16b; jede Zelle zeigt den Gesamtfaktor γ·ψ, die führende Einwirkung ist schraffiert und die aktive Kombination umrahmt. Quadratische Marken wählen eine Kombination.",
        ),
        axes: vec![axis("Combination", "Kombination", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-ROW_LABEL_WIDTH, n as f64)), axis("Action", "Einwirkung", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, rows as f64 + 1.0))],
        series,
        markers,
        annotations: notes,
        focus: Some(selector_ref()),
        legend: false,
        table_twin: DataTable { columns: table },
    })
}

fn psi_chart(document: &En1990Snapshot) -> Option<Diagram2d> {
    let mut categories: Vec<String> = Vec::new();
    for v in &document.variables {
        let category = resolve_psi_category(document.annex, &v.category, document.altitude_m);
        if psi_entry_of(&category).is_some() && !categories.contains(&category) {
            categories.push(category);
        }
    }
    if categories.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let mut notes = Vec::new();
    let mut table = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for (i, category) in categories.iter().enumerate() {
        let (Some(entry), Some(row)) = (psi_entry_of(category), psi_lookup(document.annex, category)) else { continue };
        let x0 = i as f64 * 4.0;
        for (k, (symbol, value, tone)) in [("ψ0", row.psi_0, Tone::Info), ("ψ1", row.psi_1, Tone::Warning), ("ψ2", row.psi_2, Tone::Neutral)].into_iter().enumerate() {
            let x = x0 + k as f64;
            series.push(Series::Polygon {
                id: format!("psi-{i}-{k}"),
                label: loc(&format!("{symbol} {} = {value:.2}", entry.label_en), &format!("{symbol} {} = {value:.2}", entry.label_de)),
                ring: rect(x + 0.05, 0.0, x + 0.95, value.max(0.005)),
                fill: Fill { tone, opacity: 0.8 },
                hatch: (!entry.verified).then_some(Hatch::Dots),
            });
        }
        notes.push(DiagramNote { id: format!("category-{i}"), at: p(x0 + 1.5, -0.08), text: loc(entry.label_en, entry.label_de), tone: if entry.verified { Tone::Neutral } else { Tone::Warning } });
        for (column, value) in table.iter_mut().zip([i as f64, row.psi_0, row.psi_1, row.psi_2]) {
            column.push(value);
        }
    }
    let [index, psi0, psi1, psi2] = table;
    Some(Diagram2d {
        id: "psi:document".into(),
        title: loc("ψ factors per category", "ψ-Beiwerte je Kategorie"),
        description: loc("The combination values ψ0, the frequent values ψ1 and the quasi-permanent values ψ2 of the variable-action categories in use (EN 1990 Tables A1.1 and A2.1 to A2.3); dotted bars are not yet double-keyed from the printed standard.", "Die Kombinationswerte ψ0, häufigen Werte ψ1 und quasi-ständigen Werte ψ2 der verwendeten Kategorien veränderlicher Einwirkungen (EN 1990 Tabellen A1.1 und A2.1 bis A2.3); gepunktete Balken sind noch nicht aus der gedruckten Norm doppelt erfasst."),
        axes: vec![axis("Category", "Kategorie", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.5, categories.len() as f64 * 4.0)), axis("ψ", "ψ", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.15, 1.15))],
        series,
        markers: Vec::new(),
        annotations: notes,
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Category index", "Kategorieindex", "1", index), column("ψ0", "ψ0", "1", psi0), column("ψ1", "ψ1", "1", psi1), column("ψ2", "ψ2", "1", psi2)] },
    })
}

/// 🧱️ The γ·ψ bar chart: per variable action the accompanying ULS factor γ_Q·ψ0 (Eq. 6.10b, set B), the frequent ψ1 and the quasi-permanent ψ2 that scale its characteristic value in the combinations.
fn gamma_psi_chart(document: &En1990Snapshot) -> Option<Diagram2d> {
    let domain = document.structure_kind.domain();
    let mut series = Vec::new();
    let mut notes = Vec::new();
    let mut table = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for (i, action) in document.variables.iter().enumerate() {
        let category = resolve_psi_category(document.annex, &action.category, document.altitude_m);
        let Some(row) = psi_lookup(document.annex, &category) else { continue };
        let gamma = gamma_q(document.annex, domain, FactorSet::StrB, &category);
        let x0 = i as f64 * 4.0;
        let values = [("γ_Q·ψ0", gamma * row.psi_0, Tone::Info), ("ψ1", row.psi_1, Tone::Warning), ("ψ2", row.psi_2, Tone::Neutral)];
        for (k, (symbol, value, tone)) in values.into_iter().enumerate() {
            let x = x0 + k as f64;
            series.push(Series::Polygon {
                id: format!("gamma-psi-{i}-{k}"),
                label: loc(&format!("{symbol} {} = {value:.2}", action.id), &format!("{symbol} {} = {value:.2}", action.id)),
                ring: rect(x + 0.05, 0.0, x + 0.95, value.max(0.005)),
                fill: Fill { tone, opacity: 0.8 },
                hatch: None,
            });
        }
        notes.push(DiagramNote { id: format!("gamma-psi-action-{i}"), at: p(x0 + 1.5, -0.1), text: loc(&action.id, &action.id), tone: Tone::Neutral });
        for (column, value) in table.iter_mut().zip([i as f64, gamma * row.psi_0, row.psi_1, row.psi_2]) {
            column.push(value);
        }
    }
    if series.is_empty() {
        return None;
    }
    let [index, accompanying, frequent, quasi] = table;
    Some(Diagram2d {
        id: "gamma-psi:document".into(),
        title: loc("γ·ψ factors per variable action", "γ·ψ-Faktoren je veränderlicher Einwirkung"),
        description: loc("The factors that scale each variable action as accompanying action: γ_Q·ψ0 in the ultimate combination (Eq. 6.10b, set B), ψ1 in the frequent and ψ2 in the quasi-permanent serviceability combination (EN 1990 Tables A1.1 to A1.4).", "Die Faktoren, die jede veränderliche Einwirkung als Begleiteinwirkung skalieren: γ_Q·ψ0 in der Grundkombination (Gl. 6.10b, Satz B), ψ1 in der häufigen und ψ2 in der quasi-ständigen Gebrauchstauglichkeitskombination (DIN EN 1990 Tabellen A1.1 bis A1.4)."),
        axes: vec![axis("Action", "Einwirkung", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.5, document.variables.len() as f64 * 4.0)), axis("γ·ψ", "γ·ψ", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.2, 1.6))],
        series,
        markers: Vec::new(),
        annotations: notes,
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Action index", "Einwirkungsindex", "1", index), column("γ_Q·ψ0", "γ_Q·ψ0", "1", accompanying), column("ψ1", "ψ1", "1", frequent), column("ψ2", "ψ2", "1", quasi)] },
    })
}

fn pf(beta: f64) -> f64 {
    phi_cdf(-beta)
}

fn reliability(document: &En1990Snapshot, report: &CheckReport) -> Diagram2d {
    let betas: Vec<f64> = (0..=(BETA_MAX / BETA_STEP).round() as usize).map(|i| i as f64 * BETA_STEP).collect();
    let pfs: Vec<f64> = betas.iter().map(|b| pf(*b)).collect();
    let mut series = vec![Series::Polyline { id: "pf".into(), label: loc("P_f = Φ(−β)", "P_f = Φ(−β)"), points: betas.iter().zip(&pfs).map(|(b, f)| p(*b, *f)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) }];
    for rc in 1..=3u8 {
        if let Some(target) = target_reliability_index_for(rc, document.reference_period_years) {
            let own = rc == document.reliability_class.number();
            series.push(Series::Limit { id: format!("target-rc{rc}"), label: loc(&format!("RC{rc} target β = {target:.2}"), &format!("RC{rc} Zielwert β = {target:.2}")), axis: AxisRef::X, value: target.clamp(0.0, BETA_MAX), style: stroke(if own { Tone::Warning } else { Tone::Neutral }, if own { 2.0 } else { 1.0 }, if own { Dash::Dashed } else { Dash::Dotted }) });
        }
    }
    let check = report.checks.iter().find(|c| c.id == "en1990.annex-c.beta");
    let beta = document.beta_computed.max(0.0);
    let mut binding = FieldBinding::new("betaComputed", Affine::IDENTITY, "1");
    binding.quantise = Some(Quantise::Round { step: 0.01 });
    let marker = Marker {
        id: "beta".into(),
        at: p(beta.min(BETA_MAX), pf(beta.min(BETA_MAX))),
        shape: MarkerShape::Circle,
        tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
        label: loc(&format!("β = {beta:.2}, P_f = {:.2e}", pf(beta)), &format!("β = {beta:.2}, P_f = {:.2e}", pf(beta))),
        utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u >= 0.0),
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(binding),
    };
    Diagram2d {
        id: "reliability:document".into(),
        title: loc("Reliability index and failure probability", "Zuverlässigkeitsindex und Versagenswahrscheinlichkeit"),
        description: loc("Failure probability P_f = Φ(−β) over the reliability index β with the recommended targets of the reliability classes RC1 to RC3 for the reference period (EN 1990 Annex B Table B2); the marker is the computed β and can be dragged.", "Versagenswahrscheinlichkeit P_f = Φ(−β) über dem Zuverlässigkeitsindex β mit den empfohlenen Zielwerten der Zuverlässigkeitsklassen RC1 bis RC3 für den Bezugszeitraum (EN 1990 Anhang B Tabelle B2); die Marke ist das berechnete β und lässt sich ziehen."),
        axes: vec![axis("Reliability index β", "Zuverlässigkeitsindex β", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, BETA_MAX)), axis("Failure probability P_f", "Versagenswahrscheinlichkeit P_f", QuantityKind::Dimensionless, "1", AxisScale::Log10, (1e-12, 1.0))],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Reliability index β", "Zuverlässigkeitsindex β", "1", betas), column("Failure probability P_f", "Versagenswahrscheinlichkeit P_f", "1", pfs)] },
    }
}

fn form(document: &En1990Snapshot) -> Diagram2d {
    let (alpha_r, alpha_e) = annex_c_alphas(document.annex, 1.0, 1.0);
    form_diagram("form:document".into(), loc("FORM design point", "FORM-Bemessungspunkt"), document.beta_computed.max(0.5), alpha_r, alpha_e, "en1990.annex-c.beta".into(), None)
}

fn analysis_forms(document: &En1990Snapshot, focus: Option<&str>) -> Vec<Diagram2d> {
    let selected = |a: &&ReliabilityAnalysis| focus.is_none_or(|f| f == a.id || f == document.project_id);
    document.reliability_analyses.iter().filter(selected).filter_map(|a| {
        let mut actions = vec![(a.action_distribution.as_str(), a.action_mean, a.action_sd)];
        if a.second_action_sd > 0.0 {
            actions.push((a.second_action_distribution.as_str(), a.second_action_mean, a.second_action_sd));
        }
        let form = analysis_form_multi((a.resistance_distribution.as_str(), a.resistance_mean, a.resistance_sd), &actions).ok()?;
        let alpha_e = -form.alpha_e.iter().map(|alpha| alpha * alpha).sum::<f64>().sqrt();
        Some(form_diagram(format!("form:{}", a.id), loc(&format!("FORM design point – {}", a.id), &format!("FORM-Bemessungspunkt – {}", a.id)), form.beta.max(0.5), form.alpha_r, alpha_e, format!("en1990.annex-c.form.{}", a.id), Some(EntityRef::new(EntityKind::Plate, &a.id, format!("reliabilityAnalyses[id={}]", a.id)))))
    }).collect()
}

fn form_diagram(id: String, title: LocalizedCopy, beta: f64, alpha_r: f64, alpha_e: f64, bound: String, focus: Option<EntityRef>) -> Diagram2d {
    let (nx, ny) = (-alpha_e, -alpha_r);
    let norm = nx.hypot(ny).max(1e-12);
    let (nx, ny) = (nx / norm, ny / norm);
    let star = p(beta * nx, beta * ny);
    let (tx, ty) = (-ny, nx);
    let reach = 1.5 * beta;
    let (l0, l1) = (p(star.x + tx * reach, star.y + ty * reach), p(star.x - tx * reach, star.y - ty * reach));
    let far = beta;
    let region = vec![l0.clone(), l1.clone(), p(l1.x + nx * far, l1.y + ny * far), p(l0.x + nx * far, l0.y + ny * far)];
    let circle: Vec<Vec2> = (0..=CIRCLE_POINTS).map(|i| i as f64 / CIRCLE_POINTS as f64 * std::f64::consts::TAU).map(|a| p(beta * a.cos(), beta * a.sin())).collect();
    let extent = 2.5 * beta;
    let series = vec![
        Series::Polygon { id: "failure".into(), label: loc("Failure domain g < 0", "Versagensbereich g < 0"), ring: region, fill: Fill { tone: Tone::Error, opacity: 0.15 }, hatch: Some(Hatch::Diagonal) },
        Series::Polyline { id: "circle".into(), label: loc(&format!("Reliability circle β = {beta:.2}"), &format!("Zuverlässigkeitskreis β = {beta:.2}")), points: circle.clone(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Polyline { id: "limit-state".into(), label: loc("Linearised limit state g = 0", "Linearisierter Grenzzustand g = 0"), points: vec![l0, l1], style: stroke(Tone::Error, 2.0, Dash::Dashed) },
        Series::Ray { id: "alpha".into(), label: loc("Sensitivity direction α", "Sensitivitätsrichtung α"), origin: p(0.0, 0.0), through: star.clone(), to_boundary: false, style: stroke(Tone::Warning, 1.5, Dash::Solid) },
    ];
    let marker = Marker {
        id: "design-point".into(),
        at: star.clone(),
        shape: MarkerShape::Diamond,
        tone: Tone::Error,
        label: loc(&format!("Design point: u_E = {:.2}, u_R = {:.2}", star.x, star.y), &format!("Bemessungspunkt: u_E = {:.2}, u_R = {:.2}", star.x, star.y)),
        utilisation: None,
        bound_check: Some(bound),
        binding: None,
    };
    let origin = Marker { id: "reference:means".into(), at: p(0.0, 0.0), shape: MarkerShape::Cross, tone: Tone::Neutral, label: loc("Mean values", "Mittelwerte"), utilisation: None, bound_check: None, binding: None };
    let note = DiagramNote { id: "alphas".into(), at: p(-extent + 0.2, extent - 0.4), text: loc(&format!("α_E = {alpha_e:.2}, α_R = {alpha_r:.2}, P_f = {:.2e}", pf(beta)), &format!("α_E = {alpha_e:.2}, α_R = {alpha_r:.2}, P_f = {:.2e}", pf(beta))), tone: Tone::Neutral };
    Diagram2d {
        id,
        title,
        description: loc(
            "Standard normal space of action effect u_E and resistance u_R: the circle of radius β touches the limit state g = 0 in the design point whose direction is given by the sensitivity factors α_E and α_R of EN 1990 Annex C.",
            "Standardnormalraum aus Einwirkung u_E und Widerstand u_R: Der Kreis mit Radius β berührt den Grenzzustand g = 0 im Bemessungspunkt, dessen Richtung die Wichtungsfaktoren α_E und α_R nach EN 1990 Anhang C vorgeben.",
        ),
        axes: vec![axis("Action effect u_E", "Einwirkung u_E", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-extent, extent)), axis("Resistance u_R", "Widerstand u_R", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-extent, extent))],
        series,
        markers: vec![marker, origin],
        annotations: vec![note],
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Action effect u_E", "Einwirkung u_E", "1", circle.iter().map(|q| q.x).collect()), column("Resistance u_R", "Widerstand u_R", "1", circle.iter().map(|q| q.y).collect())] },
    }
}

/// 🧪️ The marker of one test series on the k_n curves, tinted by its Annex D check.
fn test_marker(series: &TestSeries) -> Option<Marker> {
    let kn = k_n(f64::from(series.sample_size), series.variation_known > 0.0)?;
    let design = test_design_value(series.distribution.as_str(), series.sample_size, series.mean, series.std_dev, series.variation_known, series.conversion_factor).ok()?;
    let ratio = (design > 0.0 && series.required.is_finite()).then(|| (series.required / design).max(0.0));
    let tone = ratio.map_or(Tone::Error, utilisation_tone);
    Some(Marker {
        id: format!("test:{}", series.id),
        at: p(f64::from(series.sample_size).clamp(3.0, 100.0), kn.clamp(1.5, 3.5)),
        shape: MarkerShape::Circle,
        tone,
        label: loc(&format!("{}: n = {}, k_n = {kn:.2}, X_d = {design:.3} vs {:.3}", series.id, series.sample_size, series.required), &format!("{}: n = {}, k_n = {kn:.2}, X_d = {design:.3} gegenüber {:.3}", series.id, series.sample_size, series.required)),
        utilisation: ratio,
        bound_check: Some(format!("en1990.annex-d.{}", series.id)),
        binding: None,
    })
}

fn annex_d(document: &En1990Snapshot, focus: Option<&str>) -> Diagram2d {
    let known: Vec<f64> = KN_NODES.iter().filter_map(|n| k_n(*n, true)).collect();
    let unknown: Vec<f64> = KN_NODES.iter().filter_map(|n| k_n(*n, false)).collect();
    let line = |values: &[f64]| KN_NODES.iter().zip(values).map(|(n, k)| p(*n, *k)).collect::<Vec<_>>();
    Diagram2d {
        id: "annex-d:document".into(),
        title: loc("Characteristic-value factor k_n", "Faktor k_n für Merkmalwerte"),
        description: loc("The factor k_n of EN 1990 Annex D Table D1 over the number n of test results for known and unknown coefficient of variation; both approach 1.64 for large n.", "Der Faktor k_n nach EN 1990 Anhang D Tabelle D1 über der Anzahl n der Versuchsergebnisse bei bekanntem und unbekanntem Variationskoeffizienten; beide nähern sich für große n dem Wert 1,64."),
        axes: vec![axis("Number of results n", "Anzahl der Ergebnisse n", QuantityKind::Dimensionless, "1", AxisScale::Log10, (3.0, 100.0)), axis("Factor k_n", "Faktor k_n", QuantityKind::Dimensionless, "1", AxisScale::Linear, (1.5, 3.5))],
        series: vec![
            Series::Polyline { id: "v-known".into(), label: loc("V known", "V bekannt"), points: line(&known), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "v-unknown".into(), label: loc("V unknown", "V unbekannt"), points: line(&unknown), style: stroke(Tone::Warning, 2.5, Dash::Solid) },
            Series::Limit { id: "asymptote".into(), label: loc("n → ∞: 1.64", "n → ∞: 1,64"), axis: AxisRef::Y, value: 1.64, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
        ],
        markers: document.test_series.iter().filter(|s| focus.is_none_or(|f| f == s.id || f == document.project_id)).filter_map(|s| test_marker(s)).collect(),
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Number of results n", "Anzahl der Ergebnisse n", "1", KN_NODES.to_vec()), column("k_n, V known", "k_n, V bekannt", "1", known), column("k_n, V unknown", "k_n, V unbekannt", "1", unknown)] },
    }
}

fn fatigue(document: &En1990Snapshot, report: &CheckReport, member: &Member) -> Option<Diagram2d> {
    let bins: Vec<_> = document.fatigue_spectra.iter().filter(|b| b.member_id == member.id && b.detail_category > 0.0 && b.stress_range >= 0.0 && b.cycles >= 0.0).collect();
    let live = |id: String| report.checks.iter().find(|c| c.id == id && c.status != norm::CheckStatus::NotApplicable);
    let range_check = live(format!("en1990.fat.{}", member.id));
    let damage_check = live(format!("en1990.fat.damage.{}", member.id));
    let check = range_check.or(damage_check)?;
    let ratios: Vec<f64> = (0..40).map(|i| 0.41 * (3.0_f64 / 0.41).powf(i as f64 / 39.0)).collect();
    let line: Vec<(f64, f64)> = ratios.iter().filter_map(|r| en1993_endurance_cycles(*r, 1.0).map(|n| (n, *r))).collect();
    if line.len() < 2 {
        return None;
    }
    let rho = if bins.is_empty() { check.utilization.max(0.0) } else { 0.0 };
    let cycles = if bins.is_empty() { en1993_endurance_cycles(rho, 1.0) } else { None };
    let damage = cycles.map(|n| REFERENCE_CYCLES / n);
    let label = match (cycles, damage) {
        (Some(n), Some(d)) => loc(&format!("Δ/Δ_C = {rho:.2}, N = {n:.2e}, D = {d:.3} for 2·10⁶ cycles"), &format!("Δ/Δ_C = {rho:.2}, N = {n:.2e}, D = {d:.3} für 2·10⁶ Zyklen")),
        _ => loc(&format!("Δ/Δ_C = {rho:.2} below the cut-off limit"), &format!("Δ/Δ_C = {rho:.2} unter der Abschneidegrenze")),
    };
    let total = spectrum_damage(&bins.iter().map(|b| (b.stress_range, b.cycles, b.detail_category)).collect::<Vec<_>>()).unwrap_or(f64::NAN);
    let mut markers: Vec<Marker> = bins
        .iter()
        .map(|b| {
            let ratio = (b.stress_range / b.detail_category).clamp(0.2, 3.0);
            let endurance = en1993_endurance_cycles(b.stress_range, b.detail_category);
            let share = endurance.map_or(0.0, |n| b.cycles / n);
            Marker {
                id: format!("bin:{}", b.id),
                at: p(endurance.unwrap_or(1.0e9).clamp(1.0e4, 1.0e9), ratio),
                shape: MarkerShape::Square,
                tone: utilisation_tone(share),
                label: loc(&format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4} (D = {total:.4})", b.id, b.stress_range * 1e-6, b.cycles), &format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4} (D = {total:.4})", b.id, b.stress_range * 1e-6, b.cycles)),
                utilisation: Some(share),
                bound_check: Some(format!("en1990.fat.damage.{}", member.id)),
                binding: None,
            }
        })
        .collect();
    let marker = Marker {
        id: "range".into(),
        at: p(cycles.unwrap_or(1.0e9).clamp(1.0e4, 1.0e9), rho.clamp(0.2, 3.0)),
        shape: MarkerShape::Circle,
        tone: utilisation_tone(rho),
        label,
        utilisation: Some(rho),
        bound_check: Some(check.id.clone()),
        binding: None,
    };
    Some(Diagram2d {
        id: format!("fatigue:{}", member.id),
        title: loc(&format!("Fatigue damage – {}", member.label_en), &format!("Ermüdungsschädigung – {}", member.label_de)),
        description: loc("Normalised S–N line Δ/Δ_C over the endurance N (slopes m = 3 and m = 5, cut-off at 10⁸ cycles) with the variable action range of the frequent combination and the Palmgren–Miner damage for 2·10⁶ cycles (EN 1990 6.4.5).", "Normierte Wöhlerlinie Δ/Δ_C über der Lebensdauer N (Neigungen m = 3 und m = 5, Abschneidegrenze bei 10⁸ Zyklen) mit der Schwingbreite der veränderlichen Einwirkung der häufigen Kombination und der Palmgren-Miner-Schädigung für 2·10⁶ Zyklen (EN 1990 6.4.5)."),
        axes: vec![axis("Endurance N", "Lebensdauer N", QuantityKind::Dimensionless, "1", AxisScale::Log10, (1.0e4, 1.0e9)), axis("Range ratio Δ/Δ_C", "Schwingbreitenverhältnis Δ/Δ_C", QuantityKind::Dimensionless, "1", AxisScale::Log10, (0.2, 3.0))],
        series: vec![
            Series::Polyline { id: "sn".into(), label: loc("S–N line", "Wöhlerlinie"), points: line.iter().map(|(n, r)| p(*n, *r)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "category".into(), label: loc("Detail category Δ_C", "Kerbfall Δ_C"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
            Series::Limit { id: "reference-cycles".into(), label: loc("2·10⁶ cycles", "2·10⁶ Zyklen"), axis: AxisRef::X, value: REFERENCE_CYCLES, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
        ],
        markers: {
            if bins.is_empty() {
                markers.push(marker);
            }
            markers
        },
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Range ratio Δ/Δ_C", "Schwingbreitenverhältnis Δ/Δ_C", "1", line.iter().map(|(_, r)| *r).collect()), column("Endurance N", "Lebensdauer N", "1", line.iter().map(|(n, _)| *n).collect())] },
    })
}

const SLS_SAMPLES: usize = 20;

/// 📐️ Normalised elastic line of a simply supported span under uniform load, `3.2·(s − 2s³ + s⁴)` (EN 1990 A1.4, Fig. A1.1), equal to the line the deflection check samples its stations on.
pub fn sls_line(s: f64) -> f64 {
    3.2 * s * (1.0 - 2.0 * s * s + s * s * s)
}

fn sls_limits(report: &CheckReport, member: &Member) -> Option<Diagram2d> {
    if member.span <= 0.0 || member.deflection_limit_ratio <= 0.0 {
        return None;
    }
    let net = member.deflection_w1 + member.deflection_w2 + member.deflection_w3 - member.precamber;
    let limit_net = member.span / member.deflection_limit_ratio * 1000.0;
    let limit_variable = (member.deflection_limit_ratio_w3 > 0.0).then(|| member.span / member.deflection_limit_ratio_w3 * 1000.0);
    let positions: Vec<f64> = (0..=SLS_SAMPLES).map(|i| member.span * i as f64 / SLS_SAMPLES as f64).collect();
    let line = |w: f64| -> Vec<Vec2> { positions.iter().map(|x| p(*x, w * 1000.0 * sls_line(x / member.span))).collect() };
    let check = report.checks.iter().find(|c| c.id == format!("en1990.a14.deflection.{}", member.id) && c.utilization.is_finite());
    let mut series = vec![
        Series::Polyline { id: "w-net".into(), label: loc("Net deflection w_net,fin", "Enddurchbiegung w_net,fin"), points: line(net), style: stroke(Tone::Info, 2.5, Dash::Solid) },
        Series::Polyline { id: "w3".into(), label: loc("Variable-action deflection w3", "Durchbiegung w3 aus veränderlicher Einwirkung"), points: line(member.deflection_w3), style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
        Series::Limit { id: "limit-net".into(), label: loc(&format!("Limit L/{:.0} = {limit_net:.1} mm", member.deflection_limit_ratio), &format!("Grenze L/{:.0} = {limit_net:.1} mm", member.deflection_limit_ratio)), axis: AxisRef::Y, value: limit_net, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
    ];
    if let Some(limit) = limit_variable {
        series.push(Series::Limit { id: "limit-w3".into(), label: loc(&format!("Limit L/{:.0} = {limit:.1} mm", member.deflection_limit_ratio_w3), &format!("Grenze L/{:.0} = {limit:.1} mm", member.deflection_limit_ratio_w3)), axis: AxisRef::Y, value: limit, style: stroke(Tone::Warning, 1.0, Dash::Dotted) });
    }
    let mid = member.span / 2.0;
    let marker = Marker {
        id: "midspan".into(),
        at: p(mid, net * 1000.0 * sls_line(0.5)),
        shape: MarkerShape::Circle,
        tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
        label: loc(&format!("w = {:.1} mm at midspan", net * 1000.0 * sls_line(0.5)), &format!("w = {:.1} mm in Feldmitte", net * 1000.0 * sls_line(0.5))),
        utilisation: check.map(|c| c.utilization),
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(FieldBinding { path: format!("members[id={}].deflectionW3", member.id), map: Affine { base: -(member.deflection_w1 + member.deflection_w2 - member.precamber), k: 1.0 / (1000.0 * sls_line(0.5)) }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.0005 }) }),
    };
    Some(Diagram2d {
        id: format!("sls-limits:{}", member.id),
        title: loc(&format!("Serviceability limit lines – {}", member.label_en), &format!("Gebrauchstauglichkeits-Grenzlinien – {}", member.label_de)),
        description: loc("Elastic deflection line of the member (EN 1990 Annex A1.4, Fig. A1.1) against the limit lines L/n for the net deflection and for the variable-action deflection; the midspan marker drags the variable-action component w3.", "Biegelinie des Bauteils (EN 1990 Anhang A1.4, Bild A1.1) gegen die Grenzlinien L/n für die Enddurchbiegung und für die Durchbiegung aus veränderlicher Einwirkung; der Marker in Feldmitte verschiebt den Anteil w3."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, (0.0, member.span)), diagram_parts::axis("Deflection w", "Durchbiegung w", QuantityKind::Length, "mm", AxisScale::Linear, None)],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", positions.clone()), column("Net deflection w_net,fin", "Enddurchbiegung w_net,fin", "mm", positions.iter().map(|x| net * 1000.0 * sls_line(x / member.span)).collect()), column("Variable-action deflection w3", "Durchbiegung w3", "mm", positions.iter().map(|x| member.deflection_w3 * 1000.0 * sls_line(x / member.span)).collect())] },
    })
}

#[path = "../📉️envelope-and-targets/🦀️.rs"]
mod envelope_and_targets;
#[path = "../🚄️rail-and-overrides/🦀️.rs"]
mod rail_and_overrides;
#[path = "../📊️ranking-and-reliability/🦀️.rs"]
mod ranking_and_reliability;
#[path = "../📊️effects-and-spectra/🦀️.rs"]
mod effects_and_spectra;

fn known(document: &En1990Snapshot, id: &str) -> bool {
    document.project_id == id
        || id == COMBINATION_ENTITY
        || document.members.iter().any(|m| m.id == id)
        || document.bridge_sls.iter().any(|s| s.id == id)
        || document.permanents.iter().any(|a| a.id == id)
        || document.variables.iter().any(|a| a.id == id)
        || document.accidentals.iter().any(|a| a.id == id)
        || document.seismics.iter().any(|a| a.id == id)
        || document.cases.iter().any(|c| c.id == id)
        || document.factor_overrides.iter().any(|o| o.id == id)
        || document.traffic_groups.iter().any(|o| o.id == id)
        || document.fatigue_spectra.iter().any(|a| a.id == id)
        || document.reliability_analyses.iter().any(|a| a.id == id)
        || document.test_series.iter().any(|a| a.id == id)
}

/// 📈️ Every diagram of the document; `focus` set to an unknown entity yields none, to a member only the global diagrams and that member's fatigue diagram. `combination` is the id of the active matrix column.
pub fn diagrams(document: &En1990Snapshot, report: &CheckReport, focus: Option<&str>, combination: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus, combination), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1990, document.annex))
}

fn diagrams_unbased(document: &En1990Snapshot, report: &CheckReport, focus: Option<&str>, combination: Option<&str>) -> Vec<Diagram2d> {
    if focus.is_some_and(|f| !known(document, f)) {
        return Vec::new();
    }
    let columns = combinations::columns(document);
    let active = combinations::active(&columns, combination);
    let mut out: Vec<Diagram2d> = Vec::new();
    out.extend(matrix(document, report, &columns, active));
    out.extend(psi_chart(document));
    out.extend(gamma_psi_chart(document));
    out.extend(envelope_and_targets::envelope(report, &columns, active));
    out.extend(ranking_and_reliability::ranking(report, &columns, active));
    out.extend(envelope_and_targets::governing_per_member(document, report, &columns, active));
    out.push(reliability(document, report));
    out.push(envelope_and_targets::beta_targets(document, report));
    out.extend(ranking_and_reliability::beta_k_fi(document, report));
    out.extend(envelope_and_targets::traffic_groups(document, report));
    out.push(form(document));
    out.extend(rail_and_overrides::frequency_limits(document, report));
    out.extend(rail_and_overrides::resonance(document, report));
    out.extend(rail_and_overrides::overrides(document, report));
    out.extend(analysis_forms(document, focus));
    out.extend(ranking_and_reliability::form_bars(document, focus));
    out.extend(ranking_and_reliability::test_pairs(document, focus));
    out.extend(effects_and_spectra::bridge_sls(document));
    out.extend(effects_and_spectra::effects(document, &columns, focus));
    out.extend(effects_and_spectra::spectrum(document, report, focus));
    out.push(annex_d(document, focus));
    for member in document.members.iter().filter(|m| (m.rd_fat > 0.0 || document.fatigue_spectra.iter().any(|b| b.member_id == m.id)) && focus.is_none_or(|f| f == m.id || f == document.project_id || document.fatigue_spectra.iter().any(|b| b.id == f && b.member_id == m.id))) {
        out.extend(fatigue(document, report, member));
    }
    for member in document.members.iter().filter(|m| focus.is_none_or(|f| f == m.id || f == document.project_id)) {
        out.extend(sls_limits(report, member));
        out.extend(ranking_and_reliability::sls_combinations(report, member));
    }
    for case in document.cases.iter().filter(|c| focus.is_none_or(|f| f == c.id)) {
        out.extend(scene::projection_kit::design_case::case_diagram(&case.id, &loc(&case.label_en, &case.label_de), report, None));
    }
    out
}
