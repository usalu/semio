//! 📊️ Outcome diagrams: `check-outcomes:<row>` draws every check routed to a row whose family no dedicated diagram plots ([`is_drawn_family`](crate::kit::is_drawn_family)) as one bar at its utilisation against the limit line at 1, coloured by the shared utilisation ramp, with a marker bound to each check and a table twin. These are outcome bars, not physical curves: no formula is invented for them.

use super::*;
use crate::kit::{base_id, entity_kind_of, is_drawn_family, Rows};
use std::collections::BTreeMap;

/// 📊️ The outcome diagram of every row (all rows, or the one named by `focus`) that owns applicable checks with a finite utilisation outside the drawn families.
pub(super) fn outcomes(d: &En1999Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let rows = Rows::of(d);
    let mut owned: BTreeMap<String, Vec<&CheckResult>> = BTreeMap::new();
    for check in report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != crate::norm::CheckStatus::NotApplicable && !is_drawn_family(&c.id)) {
        if let Some(owner) = rows.owner(check).filter(|owner| focus.is_none_or(|f| base_id(f) == owner)) {
            owned.entry(owner).or_default().push(check);
        }
    }
    owned
        .into_iter()
        .filter_map(|(id, checks)| {
            let spec = entity_kind_of(d, &id)?;
            let worst = checks.iter().map(|c| c.utilization).fold(0.0, f64::max);
            let series: Vec<Series> = checks
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let tone = utilisation_tone(c.utilization);
                    let x = (i as f64 + 0.6, i as f64 + 1.4);
                    Series::Polygon { id: format!("outcome-{}", i + 1), label: loc(&c.title.en, &c.title.de), ring: vec![Vec2::new(x.0, 0.0), Vec2::new(x.1, 0.0), Vec2::new(x.1, c.utilization.max(0.0)), Vec2::new(x.0, c.utilization.max(0.0))], fill: Fill { tone, opacity: 0.55 }, hatch: (tone == Tone::Error).then_some(Hatch::Diagonal) }
                })
                .chain(std::iter::once(Series::Limit { id: "limit".into(), label: loc("Limit of the check (utilisation 1)", "Grenze des Nachweises (Ausnutzung 1)"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) }))
                .collect();
            let markers = checks
                .iter()
                .enumerate()
                .map(|(i, c)| Marker {
                    id: format!("check-{}", i + 1),
                    at: Vec2::new(i as f64 + 1.0, c.utilization.max(0.0)),
                    shape: MarkerShape::Diamond,
                    tone: utilisation_tone(c.utilization),
                    label: loc(&format!("{}: utilisation {:.2}", c.title.en, c.utilization), &format!("{}: Ausnutzung {:.2}", c.title.de, c.utilization)),
                    utilisation: Some(c.utilization.max(0.0)),
                    bound_check: Some(c.id.clone()),
                    binding: None,
                })
                .collect();
            Some(Diagram2d {
                id: format!("check-outcomes:{id}"),
                title: loc(&format!("Check outcomes – {id}, worst utilisation {worst:.2}"), &format!("Nachweisergebnisse – {id}, größte Ausnutzung {worst:.2}")),
                description: loc("Utilisation of every check of this row whose family has no dedicated diagram, as bars in check order against the limit at 1; the legend names each check.", "Ausnutzung jedes Nachweises dieser Zeile, dessen Familie kein eigenes Diagramm hat, als Balken in Nachweisreihenfolge gegen die Grenze 1; die Legende nennt jeden Nachweis."),
                axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, checks.len() as f64 + 1.0))), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, worst.max(1.0) * 1.1)))],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(EntityRef::new(spec.scene_kind, &id, format!("{}[id={id}]", spec.collection))),
                legend: true,
                table_twin: DataTable { columns: vec![column("Check", "Nachweis", "1", (1..=checks.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", checks.iter().map(|c| c.utilization).collect())] },
            })
        })
        .collect()
}
