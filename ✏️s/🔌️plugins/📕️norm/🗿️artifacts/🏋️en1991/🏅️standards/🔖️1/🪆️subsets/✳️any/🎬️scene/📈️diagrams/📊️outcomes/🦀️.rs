//! 📊️ Outcome diagrams: the entity that owns checks of a family no dedicated diagram draws ([`OUTCOME_FAMILIES`](crate::kit::OUTCOME_FAMILIES)) gets one diagram with a utilisation bar per check against the limit line at 1, the check titles as legend and a table twin. These are outcome bars, not physical curves: no formula is drawn that the checks do not state.

use super::*;
use std::collections::BTreeMap;

fn bar(id: &str, en: &str, de: &str, x: (f64, f64), height: f64, tone: Tone) -> Series {
    Series::Polygon { id: id.into(), label: loc(en, de), ring: vec![Vec2::new(x.0, 0.0), Vec2::new(x.1, 0.0), Vec2::new(x.1, height), Vec2::new(x.0, height)], fill: Fill { tone, opacity: 0.55 }, hatch: (tone == Tone::Error).then_some(Hatch::Diagonal) }
}

fn words(ratio: f64) -> (String, String) {
    (format!("utilisation {ratio:.2}"), format!("Ausnutzung {}", dec(format!("{ratio:.2}"))))
}

fn focus_of(document: &En1991Snapshot, id: &str) -> EntityRef {
    crate::kit::entity_kind_of(document, id).map_or_else(|| crate::kit::entity_ref(EntityKind::Group, id, "@site"), |spec| crate::kit::entity_ref(spec.scene_kind, id, spec.collection))
}

/// 📊️ The outcome diagram of every entity owning checks of an undiagrammed family, narrowed to the entity `focus` names; one diagram per owner, id `check-outcomes:<owner>`.
pub(super) fn family_outcomes(document: &En1991Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let mut owned: BTreeMap<String, Vec<&crate::norm::CheckResult>> = BTreeMap::new();
    for check in report.checks.iter().filter(|c| crate::kit::is_outcome(c)) {
        if let Some(owner) = crate::kit::owner_entity(document, check).filter(|owner| focus.is_none_or(|f| crate::kit::base_id(f) == owner)) {
            owned.entry(owner).or_default().push(check);
        }
    }
    owned
        .into_iter()
        .map(|(owner, checks)| {
            let worst = checks.iter().map(|c| c.utilization).fold(0.0, f64::max);
            let mut series: Vec<Series> = checks.iter().enumerate().map(|(i, c)| bar(&format!("outcome-{}", i + 1), &c.title.en, &c.title.de, (i as f64 + 0.6, i as f64 + 1.4), c.utilization.max(0.0), utilisation_tone(c.utilization))).collect();
            series.push(Series::Limit { id: "limit".into(), label: loc("Limit of the check (utilisation 1)", "Grenze des Nachweises (Ausnutzung 1)"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 2.0, Dash::Dashed) });
            let markers = checks
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let (en, de) = words(c.utilization);
                    Marker { id: format!("{owner}:outcome-{}", i + 1), at: Vec2::new(i as f64 + 1.0, c.utilization.max(0.0)), shape: MarkerShape::Diamond, tone: utilisation_tone(c.utilization), label: loc(&format!("{}: {en}", c.title.en), &format!("{}: {de}", c.title.de)), utilisation: Some(c.utilization), bound_check: Some(c.id.clone()), binding: None }
                })
                .collect();
            Diagram2d {
                id: format!("check-outcomes:{owner}"),
                title: loc(&format!("Check outcomes of {owner}, worst utilisation {worst:.2}"), &format!("Nachweisergebnisse von {owner}, größte Ausnutzung {}", dec(format!("{worst:.2}")))),
                description: loc("Utilisation of every check of this entity whose family has no dedicated diagram, as bars in check order against the limit at 1; the legend names each check.", "Ausnutzung jedes Nachweises dieser Einheit, dessen Familie kein eigenes Diagramm hat, als Balken in Nachweisreihenfolge gegen die Grenze bei 1; die Legende benennt jeden Nachweis."),
                axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", AxisScale::Linear, None), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(focus_of(document, &owner)),
                legend: true,
                table_twin: DataTable { columns: vec![column("Check", "Nachweis", "1", (1..=checks.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", checks.iter().map(|c| c.utilization.max(0.0)).collect())] },
            }
        })
        .collect()
}
