//! 📊️ Effects and spectra: the bridge serviceability quantities against their limits, the envelope of every action effect over the generated combinations and the stress-range spectrum of a member with its Palmgren-Miner damage. Pure projections of the document and the report, every diagram has a table twin. See [`super`].

use super::*;
use model::BridgeSls;

fn bridge_rows(document: &En1990Snapshot) -> impl Iterator<Item = (&Member, &BridgeSls)> {
    document.bridge_sls.iter().filter_map(|s| document.members.iter().find(|m| m.id == s.member_id).map(|m| (m, s)))
}

fn ratio(value: f64, limit: f64) -> Option<f64> {
    (limit > 0.0 && value.is_finite()).then(|| value.abs() / limit)
}

/// 🌉️ Deflection, deck twist and deck acceleration of a bridge row as a share of their limits; the limit u = 1 is the dashed line, the deflection marker drags `bridgeDeflection` along the y axis.
pub(super) fn bridge_sls(document: &En1990Snapshot) -> Vec<Diagram2d> {
    bridge_rows(document)
        .filter_map(|(member, sls)| {
            let rows = [
                ("deflection", "Bridge deflection", "Brückendurchbiegung", sls.bridge_deflection, sls.bridge_deflection_limit, "mm", 1000.0),
                ("twist", "Deck twist", "Fahrbahnverwindung", sls.deck_twist, sls.deck_twist_limit, "mm/3 m", 1.0),
                ("acceleration", "Deck acceleration", "Fahrbahnbeschleunigung", sls.deck_acceleration, sls.deck_acceleration_limit, "m/s²", 1.0),
            ];
            let shown: Vec<_> = rows.iter().filter_map(|r| ratio(r.3, r.4).map(|u| (r, u))).collect();
            if shown.is_empty() {
                return None;
            }
            let top = shown.iter().map(|(_, u)| *u).fold(1.0, f64::max) * 1.2;
            let mut series = Vec::new();
            let mut markers = Vec::new();
            let mut notes = Vec::new();
            for (i, (row, u)) in shown.iter().enumerate() {
                let x = i as f64 * 2.0;
                series.push(Series::Polygon { id: format!("bar-{}", row.0), label: loc(&format!("{} = {:.3} of limit {:.3} {}", row.1, row.3.abs() * row.6, row.4 * row.6, row.5), &format!("{} = {:.3} von Grenze {:.3} {}", row.2, row.3.abs() * row.6, row.4 * row.6, row.5)), ring: rect(x + 0.1, 0.0, x + 1.9, u.max(1e-3)), fill: Fill { tone: utilisation_tone(*u), opacity: 0.7 }, hatch: (*u > 1.0).then_some(Hatch::Diagonal) });
                notes.push(DiagramNote { id: format!("name-{}", row.0), at: p(x + 1.0, -0.06 * top), text: loc(row.1, row.2), tone: Tone::Neutral });
                let draggable = row.0 == "deflection";
                markers.push(Marker {
                    id: format!("tip-{}", row.0),
                    at: p(x + 1.0, *u),
                    shape: if draggable { MarkerShape::Diamond } else { MarkerShape::Circle },
                    tone: utilisation_tone(*u),
                    label: loc(&format!("{} {:.0} % of the limit", row.1, u * 100.0), &format!("{} {:.0} % der Grenze", row.2, u * 100.0)),
                    utilisation: Some(*u),
                    bound_check: Some(format!("en1990.a2.bridge.{}", member.id)),
                    binding: draggable.then(|| FieldBinding { path: format!("bridgeSls[id={}].bridgeDeflection", sls.id), map: Affine { base: 0.0, k: row.4 }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.0005 }) }),
                });
            }
            series.push(Series::Limit { id: "limit".into(), label: loc("Limit u = 1", "Grenze u = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
            Some(Diagram2d {
                id: format!("bridge-sls:{}", sls.id),
                title: loc(&format!("Bridge serviceability – {}", member.label_en), &format!("Brücken-Gebrauchstauglichkeit – {}", member.label_de)),
                description: loc("Vertical deflection, deck twist and deck acceleration of the bridge (EN 1990 A2.4) as a share of their limits; the dashed line is the limit, the deflection diamond drags the deflection.", "Vertikale Durchbiegung, Fahrbahnverwindung und Fahrbahnbeschleunigung der Brücke (EN 1990 A2.4) als Anteil ihrer Grenzen; die gestrichelte Linie ist die Grenze, die Raute verschiebt die Durchbiegung."),
                axes: vec![axis("Quantity", "Größe", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.2, shown.len() as f64 * 2.0)), axis("Share of the limit", "Anteil der Grenze", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.1 * top, top))],
                series,
                markers,
                annotations: notes,
                focus: Some(EntityRef::new(EntityKind::Plate, &sls.id, format!("bridgeSls[id={}]", sls.id))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Quantity index", "Größenindex", "1", (0..shown.len()).map(|i| i as f64).collect()), column("Value", "Wert", "1", shown.iter().map(|(r, _)| r.3.abs()).collect()), column("Limit", "Grenze", "1", shown.iter().map(|(r, _)| r.4).collect()), column("Share of the limit", "Anteil der Grenze", "1", shown.iter().map(|(_, u)| *u).collect())] },
            })
        })
        .collect()
}

/// 📉️ The effect of one action on a member, `influence · magnitude`, as the range over all generated combinations (factor γ·ψ times the characteristic effect); a bar spans minimum to maximum, the diamond is the characteristic effect.
pub(super) fn effects(document: &En1990Snapshot, columns: &[Column], focus: Option<&str>) -> Vec<Diagram2d> {
    if columns.is_empty() {
        return Vec::new();
    }
    document
        .members
        .iter()
        .filter(|m| focus.is_none_or(|f| f == m.id || f == document.project_id))
        .filter_map(|member| {
            let rows: Vec<_> = document.effects.iter().filter(|e| e.member_id == member.id).collect();
            if rows.is_empty() {
                return None;
            }
            let mut series = Vec::new();
            let mut markers = Vec::new();
            let mut notes = Vec::new();
            let mut table = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            let mut extent = 1.0_f64;
            for (i, effect) in rows.iter().enumerate() {
                let characteristic = effect.influence * handles::raw_value(document, &effect.action_id);
                let values: Vec<f64> = columns.iter().filter_map(|c| c.factor(&effect.action_id)).map(|f| f * characteristic).collect();
                let (low, high) = (values.iter().copied().fold(0.0, f64::min), values.iter().copied().fold(0.0, f64::max));
                extent = extent.max(high.abs()).max(low.abs()).max(characteristic.abs());
                let x = i as f64 * 2.0;
                series.push(Series::Polygon { id: format!("range-{i}"), label: loc(&format!("{}: {:.1} to {:.1} kN over {} combinations", effect.action_id, low / 1e3, high / 1e3, values.len()), &format!("{}: {:.1} bis {:.1} kN über {} Kombinationen", effect.action_id, low / 1e3, high / 1e3, values.len())), ring: rect(x + 0.1, low, x + 1.9, high.max(low + 1e-3)), fill: Fill { tone: Tone::Info, opacity: 0.5 }, hatch: (low < 0.0 && high > 0.0).then_some(Hatch::Diagonal) });
                markers.push(Marker { id: format!("reference:characteristic-{i}"), at: p(x + 1.0, characteristic), shape: MarkerShape::Diamond, tone: Tone::Neutral, label: loc(&format!("{} characteristic effect {:.1} kN", effect.action_id, characteristic / 1e3), &format!("{} charakteristische Beanspruchung {:.1} kN", effect.action_id, characteristic / 1e3)), utilisation: None, bound_check: None, binding: None });
                notes.push(DiagramNote { id: format!("action-{i}"), at: p(x + 1.0, 0.0), text: loc(&effect.action_id, &effect.action_id), tone: Tone::Neutral });
                for (column, value) in table.iter_mut().zip([i as f64, characteristic, low, high]) {
                    column.push(value);
                }
            }
            let [index, characteristic, low, high] = table;
            Some(Diagram2d {
                id: format!("effects:{}", member.id),
                title: loc(&format!("Effect envelope – {}", member.label_en), &format!("Beanspruchungs-Einhüllende – {}", member.label_de)),
                description: loc("Per action effect on the member (influence times magnitude) the range over all generated combinations; the diamond is the characteristic effect.", "Je Einwirkungsbeanspruchung des Bauteils (Einflussbeiwert mal Größe) der Bereich über alle erzeugten Kombinationen; die Raute ist die charakteristische Beanspruchung."),
                axes: vec![axis("Action effect", "Einwirkungsbeanspruchung", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.2, rows.len() as f64 * 2.0)), axis("Effect", "Beanspruchung", QuantityKind::Force, "N", AxisScale::Linear, (-1.2 * extent, 1.2 * extent))],
                series,
                markers,
                annotations: notes,
                focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
                legend: false,
                table_twin: DataTable { columns: vec![column("Effect index", "Beanspruchungsindex", "1", index), column("Characteristic effect", "Charakteristische Beanspruchung", "N", characteristic), column("Minimum over combinations", "Minimum über Kombinationen", "N", low), column("Maximum over combinations", "Maximum über Kombinationen", "N", high)] },
            })
        })
        .collect()
}

/// 📳️ The stress-range spectrum of a member sorted by range: the damage share n/N of every bin as a bar with its cycles and range, the cumulative Palmgren-Miner sum as a line against D = 1 (EN 1990 6.4.5, S-N line of EN 1993-1-9).
pub(super) fn spectrum(document: &En1990Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    document
        .members
        .iter()
        .filter(|m| focus.is_none_or(|f| f == m.id || f == document.project_id || document.fatigue_spectra.iter().any(|b| b.id == f && b.member_id == m.id)))
        .filter_map(|member| {
            let mut bins: Vec<_> = document.fatigue_spectra.iter().filter(|b| b.member_id == member.id && b.detail_category > 0.0 && b.stress_range >= 0.0 && b.cycles >= 0.0).collect();
            if bins.is_empty() {
                return None;
            }
            bins.sort_by(|a, b| a.stress_range.total_cmp(&b.stress_range));
            let shares: Vec<f64> = bins.iter().map(|b| en1993_endurance_cycles(b.stress_range, b.detail_category).map_or(0.0, |n| b.cycles / n)).collect();
            let total: f64 = shares.iter().sum();
            let check = report.checks.iter().find(|c| c.id == format!("en1990.fat.damage.{}", member.id));
            let mut running = 0.0;
            let mut series = Vec::new();
            let mut notes = Vec::new();
            let mut markers = Vec::new();
            let mut cumulative = Vec::new();
            for (i, (bin, share)) in bins.iter().zip(&shares).enumerate() {
                running += share;
                let x = i as f64;
                series.push(Series::Polygon { id: format!("bin-{}", bin.id), label: loc(&format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4}", bin.id, bin.stress_range * 1e-6, bin.cycles), &format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4}", bin.id, bin.stress_range * 1e-6, bin.cycles)), ring: rect(x + CELL_INSET, 0.0, x + 1.0 - CELL_INSET, share.max(1e-4)), fill: Fill { tone: utilisation_tone(*share), opacity: 0.7 }, hatch: None });
                notes.push(DiagramNote { id: format!("range-{}", bin.id), at: p(x + 0.5, -0.04), text: loc(&format!("{:.0} MPa", bin.stress_range * 1e-6), &format!("{:.0} MPa", bin.stress_range * 1e-6)), tone: Tone::Neutral });
                cumulative.push(p(x + 0.5, running));
            }
            series.push(Series::Polyline { id: "cumulative".into(), label: loc("Cumulative damage ΣD", "Kumulierte Schädigung ΣD"), points: cumulative, style: stroke(Tone::Info, 2.0, Dash::Solid) });
            series.push(Series::Limit { id: "damage-limit".into(), label: loc("D = 1", "D = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
            markers.push(Marker { id: "total".into(), at: p(bins.len() as f64 - 0.5, total), shape: MarkerShape::Diamond, tone: utilisation_tone(total), label: loc(&format!("Total damage D = {total:.4}"), &format!("Gesamtschädigung D = {total:.4}")), utilisation: Some(total), bound_check: check.map(|c| c.id.clone()), binding: None });
            let top = total.max(shares.iter().copied().fold(0.0, f64::max)).max(1.0) * 1.2;
            Some(Diagram2d {
                id: format!("fatigue-spectrum:{}", member.id),
                title: loc(&format!("Stress-range spectrum – {}", member.label_en), &format!("Spannungsschwingbreiten-Kollektiv – {}", member.label_de)),
                description: loc("The bins of the spectrum sorted by stress range: damage share n/N per bin, cumulative Palmgren-Miner sum against D = 1.", "Die Klassen des Kollektivs nach Schwingbreite sortiert: Schädigungsanteil n/N je Klasse, kumulierte Palmgren-Miner-Summe gegen D = 1."),
                axes: vec![axis("Bin (by range)", "Klasse (nach Schwingbreite)", QuantityKind::Dimensionless, "1", AxisScale::Linear, (0.0, bins.len() as f64)), axis("Damage", "Schädigung", QuantityKind::Dimensionless, "1", AxisScale::Linear, (-0.08 * top, top))],
                series,
                markers,
                annotations: notes,
                focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Stress range", "Schwingbreite", "Pa", bins.iter().map(|b| b.stress_range).collect()), column("Cycles", "Zyklen", "1", bins.iter().map(|b| b.cycles).collect()), column("Damage share n/N", "Schädigungsanteil n/N", "1", shares)] },
            })
        })
        .collect()
}
