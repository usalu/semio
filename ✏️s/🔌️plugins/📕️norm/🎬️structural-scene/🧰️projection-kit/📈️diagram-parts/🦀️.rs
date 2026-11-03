//! 📈️ Diagram parts: the axis, stroke and table-column constructors every artifact's diagrams repeat.

use crate::*;

pub fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, scale: AxisScale, domain: Option<(f64, f64)>) -> Axis {
    Axis { label: LocalizedCopy::new(en, de), quantity, unit: unit.to_string(), scale, domain: domain.map(|(lo, hi)| Interval::new(lo, hi)), ticks: TickRule::Auto }
}

pub fn stroke(tone: Tone, width_px: f64, dash: Dash) -> Stroke {
    Stroke { width_px, dash, tone }
}

/// 📋️ One column of a diagram's table twin.
pub fn column(en: &str, de: &str, unit: &str, values: Vec<f64>) -> DataColumn {
    DataColumn { header: LocalizedCopy::new(en, de), unit: unit.to_string(), values }
}

/// 🏷️ The id of a marker that a check may draw: `id` when `check` names the check, else `reference:<id>` so a point no check ties to is explicitly a reference (see [`REFERENCE_MARKER_KIND`]).
pub fn checked_marker_id(id: &str, check: &Option<String>) -> String {
    if check.is_some() { id.to_string() } else { format!("{REFERENCE_MARKER_KIND}:{id}") }
}
