//! 🗂️ Design-case twin: the default 2D diagram and table twin of one design case (utilisation per check against the limit 1) and the optional section of a case that declares geometry. See [`super`].

use super::diagram_parts::{axis, column, stroke};
use crate::*;
use super::handle_kit::{field_binding, round};
use norm::design_case::{self, CaseCatalogue, CaseRow};
use norm::{CheckReport, CheckResult, CheckStatus as Status, UnitId};

fn tone_of(check: &CheckResult) -> Tone {
    if check.status == Status::Fail || check.utilization > 1.0 {
        Tone::Error
    } else if check.utilization > 0.8 || check.status == Status::Warning {
        Tone::Warning
    } else {
        Tone::Info
    }
}

/// 📐️ Rectangular section of `width_m × height_m` for a case that declares geometry.
pub fn case_section(width_m: f64, height_m: f64) -> SectionShape {
    SectionShape::parametric(ProfileKind::Rect, &[("h", height_m), ("b", width_m)])
}

/// 📈️ Utilisation of every applicable check of the case `case_id` as one scatter series with a limit line at 1, one marker per check bound to its check id, the optional section `(width, height)` in metres and the mandatory table twin; `None` when the case has no applicable check.
pub fn case_diagram(case_id: &str, title: &LocalizedCopy, report: &CheckReport, section: Option<(f64, f64)>) -> Option<Diagram2d> {
    let rows: Vec<&CheckResult> = report.checks.iter().filter(|c| c.subject.entity_id == case_id && c.status != Status::NotApplicable && c.utilization.is_finite()).collect();
    if rows.is_empty() {
        return None;
    }
    let count = rows.len() as f64;
    let top = rows.iter().map(|c| c.utilization).fold(1.2_f64, f64::max) * 1.1;
    let at = |index: usize, check: &CheckResult| Vec2::new(index as f64 + 1.0, check.utilization);
    let mut series = vec![
        Series::Scatter { id: "utilisation".into(), label: LocalizedCopy::new("Utilisation per check", "Ausnutzung je Nachweis"), points: rows.iter().enumerate().map(|(i, c)| ScatterPoint { at: at(i, c), tone: tone_of(c), label: Some(LocalizedCopy::new(&c.title.en, &c.title.de)) }).collect() },
        Series::Limit { id: "limit".into(), label: LocalizedCopy::new("Limit 100 %", "Grenze 100 %"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
    ];
    if let Some((width, height)) = section.filter(|(w, h)| *w > 0.0 && *h > 0.0) {
        series.push(Series::Section { id: "section".into(), label: LocalizedCopy::new("Cross-section", "Querschnitt"), shape: case_section(width, height), strain: None, stress: None, neutral_axis: None });
    }
    let markers = rows
        .iter()
        .enumerate()
        .map(|(i, c)| Marker { id: format!("check-{}", i + 1), at: at(i, c), shape: MarkerShape::Circle, tone: tone_of(c), label: LocalizedCopy::new(&c.title.en, &c.title.de), utilisation: Some(c.utilization), bound_check: Some(c.id.clone()), binding: None })
        .collect();
    Some(Diagram2d {
        id: format!("case-{case_id}"),
        title: title.clone(),
        description: LocalizedCopy::new("Utilisation of every check of the design case against the limit of 100 %.", "Ausnutzung jedes Nachweises des Bemessungsfalls gegen die Grenze von 100 %."),
        axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, count + 1.0))), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, top)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, case_id, format!("cases[id={case_id}]"))),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Check", "Nachweis", "1", (1..=rows.len()).map(|i| i as f64).collect()),
                column("Utilisation", "Ausnutzung", "1", rows.iter().map(|c| c.utilization).collect()),
                column("Computed (SI)", "Berechnet (SI)", "1", rows.iter().map(|c| c.computed.value).collect()),
                column("Limit (SI)", "Grenzwert (SI)", "1", rows.iter().map(|c| c.limit.value).collect()),
            ],
        },
    })
}

/// 📏️ The number input `key` of `row` in metres, converted from the stored unit of its vocabulary row; `None` when unset, not a length or not positive.
pub fn input_metres<C: CaseCatalogue>(row: &CaseRow, key: &str) -> Option<f64> {
    let value = row.numbers.iter().find(|n| n.id == key)?.value;
    let unit = design_case::spec::<C>(&row.entry_id, key)?.unit;
    unit.convert(value, UnitId::Metre).ok().filter(|m| m.is_finite() && *m > 0.0)
}

/// 🔁️ How many stored units of the input `key` of `row` make one metre, with the symbol of that unit.
pub fn input_per_metre<C: CaseCatalogue>(row: &CaseRow, key: &str) -> Option<(f64, &'static str)> {
    let unit = design_case::spec::<C>(&row.entry_id, key)?.unit;
    UnitId::Metre.convert(1.0, unit).ok().map(|per_metre| (per_metre, unit.symbol()))
}

/// 🕹️ The write of a geometric case input for a handle: `stored = value + factor · per_metre · s` for a drag of `s` metres, quantised to `grid_m` metres in the stored unit; `None` when the input is unset (the leaf must exist for the drag to commit) or not a length.
pub fn size_binding<C: CaseCatalogue>(row: &CaseRow, key: &str, factor: f64, grid_m: f64) -> Option<FieldBinding> {
    let value = row.numbers.iter().find(|n| n.id == key)?.value;
    let (per_metre, unit) = input_per_metre::<C>(row, key)?;
    Some(field_binding(design_case::input_path(&row.id, key, true), Affine { base: value, k: factor * per_metre }, unit, round(grid_m * per_metre)))
}

#[path = "🧊️forms/🦀️.rs"]
pub mod forms;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
