//! 🚄️ Rail resonance and factor overrides: the Fig. 6.10 frequency band with the deck's natural frequency, the acceleration-versus-speed sweep of the HSLM-like spectrum with the resonance speeds, and the annex-versus-override values of every factor override. Every diagram has a table twin; markers bind to checks and, for overrides, to the override leaf.

use super::*;
use super::super::checks::rail_dynamics::{governing_acceleration, DeckModes, Train};
use super::super::checks::ndp;

const SPAN_NODES: usize = 24;
const SWEEP: usize = 40;

fn rail_rows(document: &En1990Snapshot) -> Vec<(&Member, &super::super::model::BridgeSls)> {
    document.bridge_sls.iter().filter(|s| document.structure_kind == semio_s_artifact_norm_en1990_model::StructureKind::RailBridge && s.natural_frequency > 0.0).filter_map(|s| document.members.iter().find(|m| m.id == s.member_id && m.span > 0.0).map(|m| (m, s))).collect()
}

fn check_of<'a>(report: &'a CheckReport, key: &str, member: &Member, sls: &super::super::model::BridgeSls) -> Option<&'a super::super::norm::CheckResult> {
    let id = format!("en1990.a2.rail-{key}.{}.{}", member.id, sls.id);
    report.checks.iter().find(|c| c.id == id)
}

/// 📐️ Fig. 6.10 of EN 1991-2 as formulas: the lower and upper limit of the natural frequency n0 over the span L (log axes) with the deck's own point; the point is bound to the dynamic-analysis flag check. Unverified coefficients.
pub(super) fn frequency_limits(document: &En1990Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    rail_rows(document)
        .into_iter()
        .map(|(member, sls)| {
            let annex = document.annex;
            let (lo, hi) = (ndp(annex, "ndp.en1990.a2.rail.f_range_min"), ndp(annex, "ndp.en1990.a2.rail.f_range_max"));
            let spans: Vec<f64> = (0..SPAN_NODES).map(|i| lo * (hi / lo).powf(i as f64 / (SPAN_NODES - 1) as f64)).collect();
            let limits: Vec<(f64, f64)> = spans.iter().map(|l| super::super::checks::rail_dynamics::frequency_limits(annex, *l).unwrap_or((0.0, 0.0))).collect();
            let check = check_of(report, "dynamic", member, sls);
            let inside = super::super::checks::rail_dynamics::frequency_limits(annex, member.span).is_some_and(|(l, u)| sls.natural_frequency >= l && sls.natural_frequency <= u);
            Diagram2d {
                id: format!("rail-frequency-limits:{}", sls.id),
                title: loc("Natural frequency limits (Fig. 6.10)", "Grenzen der Eigenfrequenz (Bild 6.10)"),
                description: loc("The lower and upper limit of the first natural frequency n0 of a simply supported rail bridge over its span; outside the band, or above the registry line speed, a dynamic analysis is required. The coefficients are unverified registry values.", "Untere und obere Grenze der ersten Eigenfrequenz n0 einer einfach gelagerten Eisenbahnbrücke über der Stützweite; außerhalb des Bereichs oder oberhalb der Geschwindigkeitsgrenze der Registerwerte ist eine dynamische Berechnung erforderlich. Die Koeffizienten sind ungeprüfte Registerwerte."),
                axes: vec![axis("Span L", "Stützweite L", QuantityKind::Length, "m", AxisScale::Log10, (lo, hi)), axis("Natural frequency n0", "Eigenfrequenz n0", QuantityKind::Dimensionless, "Hz", AxisScale::Log10, (1.0, 40.0))],
                series: vec![
                    Series::Polyline { id: "upper".into(), label: loc("Upper limit", "Obere Grenze"), points: spans.iter().zip(&limits).map(|(l, (_, u))| p(*l, *u)).collect(), style: stroke(Tone::Warning, 2.0, Dash::Solid) },
                    Series::Polyline { id: "lower".into(), label: loc("Lower limit", "Untere Grenze"), points: spans.iter().zip(&limits).map(|(l, (lw, _))| p(*l, *lw)).collect(), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
                ],
                markers: vec![Marker {
                    id: format!("deck-{}", sls.id),
                    at: p(member.span.clamp(lo, hi), sls.natural_frequency.clamp(1.0, 40.0)),
                    shape: MarkerShape::Circle,
                    tone: if inside { Tone::Info } else { Tone::Warning },
                    label: loc(&format!("n0 = {:.2} Hz at L = {:.1} m", sls.natural_frequency, member.span), &format!("n0 = {:.2} Hz bei L = {:.1} m", sls.natural_frequency, member.span)),
                    utilisation: None,
                    bound_check: check.map(|c| c.id.clone()),
                    binding: Some(FieldBinding { path: format!("bridgeSls[id={}].naturalFrequency", sls.id), map: Affine::IDENTITY, unit: "Hz".into(), quantise: Some(Quantise::Round { step: 0.05 }) }),
                }],
                annotations: Vec::new(),
                focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Span L", "Stützweite L", "m", spans), column("Lower limit n0", "Untere Grenze n0", "Hz", limits.iter().map(|l| l.0).collect()), column("Upper limit n0", "Obere Grenze n0", "Hz", limits.iter().map(|l| l.1).collect())] },
            }
        })
        .collect()
}

/// 🌊️ The peak mid-span acceleration of the governing HSLM-like train over the speed (kernel modes, modal superposition) against the deck limit, with the first-order resonance speeds as markers; table twin with the sweep.
pub(super) fn resonance(document: &En1990Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    rail_rows(document)
        .into_iter()
        .filter(|(member, sls)| sls.line_speed > 0.0 && sls.deck_mass > 0.0 && member.span > 0.0)
        .filter_map(|(member, sls)| {
            let annex = document.annex;
            let modes = DeckModes::compute(member.span, sls.deck_mass, sls.natural_frequency)?;
            let zeta = if sls.damping_ratio > 0.0 { sls.damping_ratio } else { ndp(annex, "ndp.en1990.a2.rail.damping_default") };
            let top = sls.line_speed * ndp(annex, "ndp.en1990.a2.rail.speed_margin") / 3.6;
            let trains = Train::spectrum(annex);
            let governing = governing_acceleration(&modes, &trains, top, zeta)?;
            let train = trains.iter().find(|t| t.name == governing.train)?;
            let speeds: Vec<f64> = (0..SWEEP).map(|i| top * (0.3 + 0.7 * i as f64 / (SWEEP - 1) as f64)).collect();
            let accelerations: Vec<f64> = speeds.iter().map(|v| modes.peak_acceleration(train, *v, zeta)).collect();
            let limit = ndp(annex, if sls.ballast_depth > 0.0 { "ndp.en1990.a2.rail.acc_ballast" } else { "ndp.en1990.a2.rail.acc_slab" });
            let peak = accelerations.iter().copied().fold(limit, f64::max) * 1.1;
            let check = check_of(report, "resonance", member, sls);
            let kmh: Vec<f64> = speeds.iter().map(|v| v * 3.6).collect();
            let mut markers: Vec<Marker> = train
                .resonance_speeds(modes.frequencies[0], 3)
                .into_iter()
                .filter(|(_, v)| *v >= speeds[0] && *v <= top)
                .map(|(order, v)| Marker {
                    id: format!("resonance-{order}"),
                    at: p(v * 3.6, modes.peak_acceleration(train, v, zeta)),
                    shape: MarkerShape::Diamond,
                    tone: Tone::Warning,
                    label: loc(&format!("Resonance order {order}: {:.0} km/h", v * 3.6), &format!("Resonanz {order}. Ordnung: {:.0} km/h", v * 3.6)),
                    utilisation: None,
                    bound_check: check.map(|c| c.id.clone()),
                    binding: None,
                })
                .collect();
            markers.push(Marker {
                id: "governing".into(),
                at: p(governing.speed * 3.6, governing.acceleration),
                shape: MarkerShape::Circle,
                tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
                label: loc(&format!("Governing: {} at {:.0} km/h, {:.2} m/s²", governing.train, governing.speed * 3.6, governing.acceleration), &format!("Maßgebend: {} bei {:.0} km/h, {:.2} m/s²", governing.train, governing.speed * 3.6, governing.acceleration)),
                utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite()),
                bound_check: check.map(|c| c.id.clone()),
                binding: None,
            });
            Some(Diagram2d {
                id: format!("rail-resonance:{}", sls.id),
                title: loc("Deck resonance under the HSLM-like spectrum", "Resonanz des Decks unter dem HSLM-ähnlichen Zugspektrum"),
                description: loc("The peak mid-span acceleration of the governing universal train over the speed up to the verification speed; the diamonds are the first resonance speeds v = n0·D/i, the dashed line the deck acceleration limit. Train data and damping default are unverified registry values; the power cars are not modelled.", "Die maximale Beschleunigung in Feldmitte des maßgebenden Universalzugs über der Geschwindigkeit bis zur Nachweisgeschwindigkeit; die Rauten sind die ersten Resonanzgeschwindigkeiten v = n0·D/i, die gestrichelte Linie die Grenze der Deckbeschleunigung. Zugdaten und Dämpfungsvorgabe sind ungeprüfte Registerwerte; die Triebköpfe sind nicht abgebildet."),
                axes: vec![axis("Speed", "Geschwindigkeit", QuantityKind::Velocity, "km/h", AxisScale::Linear, (kmh[0], kmh[SWEEP - 1])), axis("Peak acceleration", "Größte Beschleunigung", QuantityKind::Acceleration, "m/s²", AxisScale::Linear, (0.0, peak))],
                series: vec![
                    Series::Polyline { id: "acceleration".into(), label: loc(&format!("{} acceleration", governing.train), &format!("{} Beschleunigung", governing.train)), points: kmh.iter().zip(&accelerations).map(|(v, a)| p(*v, *a)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
                    Series::Limit { id: "limit".into(), label: loc(&format!("Limit {limit:.1} m/s²"), &format!("Grenze {limit:.1} m/s²")), axis: AxisRef::Y, value: limit, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
                ],
                markers,
                annotations: Vec::new(),
                focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Speed", "Geschwindigkeit", "km/h", kmh), column("Peak acceleration", "Größte Beschleunigung", "m/s²", accelerations)] },
            })
        })
        .collect()
}

/// 🎚️ The annex value against the project override of every factor-override row as two bars; the override bar is drawn in the warning tone (deviates from the National Annex) or the error tone (refused), the marker is bound to the row's check and drags the override value.
pub(super) fn overrides(document: &En1990Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if document.factor_overrides.is_empty() {
        return None;
    }
    let verdicts = super::super::inferences::factor_overrides::verdicts(document);
    let annex: Vec<f64> = document.factor_overrides.iter().map(|row| super::super::inferences::factor_overrides::annex_value(document, row).unwrap_or(0.0)).collect();
    let top = document.factor_overrides.iter().map(|r| r.value).chain(annex.iter().copied()).fold(1.0, f64::max) * 1.15;
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (i, row) in document.factor_overrides.iter().enumerate() {
        let x = i as f64;
        let applied = verdicts[i] == super::super::inferences::factor_overrides::Verdict::Applied;
        series.push(Series::Polygon { id: format!("annex-{}", row.id), label: loc(&format!("{} annex value {:.3}", row.id, annex[i]), &format!("{} Anhangswert {:.3}", row.id, annex[i])), ring: rect(x + CELL_INSET, 0.0, x + 0.5, annex[i]), fill: Fill { tone: Tone::Neutral, opacity: 0.5 }, hatch: None });
        series.push(Series::Polygon { id: format!("override-{}", row.id), label: loc(&format!("{} override {:.3} ({})", row.id, row.value, if applied { "deviates from NA" } else { "not applied" }), &format!("{} Überschreibung {:.3} ({})", row.id, row.value, if applied { "weicht vom NA ab" } else { "nicht angewendet" })), ring: rect(x + 0.5, 0.0, x + 1.0 - CELL_INSET, row.value.max(0.0)), fill: Fill { tone: if applied { Tone::Warning } else { Tone::Error }, opacity: 0.9 }, hatch: (!applied).then_some(Hatch::Diagonal) });
        let check = report.checks.iter().find(|c| c.id == format!("en1990.override.{}", row.id));
        markers.push(Marker {
            id: format!("override-{}", row.id),
            at: p(x + 0.75, row.value.max(0.0)),
            shape: MarkerShape::Diamond,
            tone: if applied { Tone::Warning } else { Tone::Error },
            label: loc(&format!("{} {} on {}: {:.3}", row.id, row.factor, if row.target.is_empty() { "all" } else { &row.target }, row.value), &format!("{} {} auf {}: {:.3}", row.id, row.factor, if row.target.is_empty() { "alle" } else { &row.target }, row.value)),
            utilisation: None,
            bound_check: check.map(|c| c.id.clone()),
            binding: Some(FieldBinding { path: format!("factorOverrides[id={}].value", row.id), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Round { step: 0.01 }) }),
        });
    }
    Some(Diagram2d {
        id: "factor-overrides:document".into(),
        title: loc("Factor overrides against the National Annex", "Faktorüberschreibungen gegenüber dem Nationalen Anhang"),
        description: loc("Per override row the annex value (grey) and the project value (amber: deviates from the National Annex, red hatched: refused). Drag a diamond to change the override value.", "Je Überschreibungszeile der Anhangswert (grau) und der Projektwert (gelb: weicht vom Nationalen Anhang ab, rot schraffiert: abgelehnt). Eine Raute ziehen, um den Überschreibungswert zu ändern."),
        axes: vec![axis("Override row", "Überschreibungszeile", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, document.factor_overrides.len() as f64)), axis("Factor value", "Faktorwert", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, top))],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Row index", "Zeilenindex", "1", (0..annex.len()).map(|i| i as f64).collect()), column("Annex value", "Anhangswert", "1", annex), column("Override value", "Überschreibungswert", "1", document.factor_overrides.iter().map(|r| r.value).collect()), column("Applied", "Angewendet", "1", verdicts.iter().map(|v| f64::from(u8::from(*v == super::super::inferences::factor_overrides::Verdict::Applied))).collect())] },
    })
}
