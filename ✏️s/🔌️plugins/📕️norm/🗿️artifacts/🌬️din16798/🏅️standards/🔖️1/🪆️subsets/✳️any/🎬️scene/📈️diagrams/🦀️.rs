//! 📈️ Diagrams: the adaptive comfort diagram (operative against running-mean outdoor temperature with the category bands I–IV, EN 16798-1 Annex B.2), the operative temperature bands per category (Table B.2), the ventilation rate bars (persons and building components, Tables B.6/B.7), the CO₂ and relative-humidity category checks of every room and the ventilation-system capacity of the building. Every diagram has a table twin; markers bind to checks and to document leaves.

use super::checks::{self, part_1, ComfortCategory};
use super::model::{Din16798Snapshot, ZoneDocument};
use super::norm::{CheckReport, CheckStatus};
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::*;

#[path = "🌡️comfort/🦀️.rs"]
mod comfort;
#[path = "🌬️ventilation/🦀️.rs"]
mod ventilation;
#[path = "🫧️air-quality/🦀️.rs"]
mod air_quality;

pub use comfort::{adaptive_limits, operative_bands, RUNNING_MEAN_RANGE_C};
pub use ventilation::ventilation_components;

pub(super) const CATEGORIES: [ComfortCategory; 4] = [ComfortCategory::I, ComfortCategory::II, ComfortCategory::III, ComfortCategory::IV];

pub(super) fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, domain)
}

pub(super) fn category_axis() -> Axis {
    axis("Comfort category", "Komfortkategorie", QuantityKind::Dimensionless, "1", Some((0.5, 4.5)))
}

pub(super) fn p(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

pub(super) fn category_number(category: ComfortCategory) -> f64 {
    CATEGORIES.iter().position(|c| *c == category).map_or(2.0, |index| (index + 1) as f64)
}

pub(super) fn category_name(category: ComfortCategory) -> &'static str {
    ["I", "II", "III", "IV"][category_number(category) as usize - 1]
}

pub(super) fn find<'a>(report: &'a CheckReport, id: &str) -> Option<&'a super::norm::CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

pub(super) fn check_tone(report: &CheckReport, id: &str) -> Tone {
    find(report, id).map_or(Tone::Neutral, |c| match c.status {
        CheckStatus::Fail => Tone::Error,
        CheckStatus::Warning | CheckStatus::UnverifiedBasis => Tone::Warning,
        CheckStatus::Pass | CheckStatus::Info => Tone::Info,
        CheckStatus::NotApplicable => Tone::Neutral,
    })
}

pub(super) fn check_utilisation(report: &CheckReport, id: &str) -> Option<f64> {
    find(report, id).map(|c| c.utilization.max(0.0)).filter(|v| v.is_finite())
}

pub(super) fn zone_focus(zone: &ZoneDocument) -> EntityRef {
    EntityRef::new(EntityKind::Solid, zone.id.clone(), format!("zones[id={}]", zone.id))
}

pub(super) fn marker(id: &str, at: Vec2, shape: MarkerShape, report: &CheckReport, check: &str, label: LocalizedCopy, binding: Option<FieldBinding>) -> Marker {
    Marker { id: id.to_owned(), at, shape, tone: check_tone(report, check), label, utilisation: check_utilisation(report, check), bound_check: Some(check.to_owned()), binding }
}

pub(super) fn bound(path: String, unit: &str, step: f64) -> FieldBinding {
    FieldBinding { path, map: Affine::IDENTITY, unit: unit.to_owned(), quantise: Some(Quantise::Round { step }) }
}

pub(super) fn bar(x: f64, from: f64, to: f64) -> Vec<Vec2> {
    bar_of(x, 0.35, from, to)
}

pub(super) fn bar_of(x: f64, half_width: f64, from: f64, to: f64) -> Vec<Vec2> {
    vec![p(x - half_width, from), p(x + half_width, from), p(x + half_width, to), p(x - half_width, to)]
}

pub(super) fn fill(tone: Tone, opacity: f64) -> Fill {
    Fill { tone, opacity }
}

pub(super) fn solid(tone: Tone) -> Stroke {
    stroke(tone, 2.0, Dash::Solid)
}

pub(super) fn dashed(tone: Tone) -> Stroke {
    stroke(tone, 1.0, Dash::Dashed)
}

pub(super) fn table(columns: Vec<DataColumn>) -> DataTable {
    DataTable { columns }
}

pub(super) fn col(en: &str, de: &str, unit: &str, values: Vec<f64>) -> DataColumn {
    column(en, de, unit, values)
}

/// 📈️ Every diagram of the document; `focus` set to an unknown entity yields none, to a room only its own diagrams and the building capacity, to a ventilation system only the building capacity.
pub fn diagrams(document: &Din16798Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    if focus.is_some_and(|f| super::kit::entity_kind_of(document, f).is_none()) {
        return Vec::new();
    }
    let mut out: Vec<Diagram2d> = Vec::new();
    for zone in document.zones.iter().filter(|z| focus.is_none_or(|f| f == z.id)) {
        out.push(comfort::adaptive(document, report, zone));
        out.push(comfort::operative(report, zone));
        out.push(ventilation::rates(document, report, zone));
        out.push(air_quality::co2(document, report, zone));
        out.push(air_quality::humidity(report, zone));
    }
    out.extend(ventilation::capacity(document, report));
    super::scene::projection::with_basis(out, &super::scene::projection::Basis::new(super::norm::NormFamilyId::DinEn16798, document.annex))
}

/// 🌡️ The comfort category of a zone.
pub fn category_of(zone: &ZoneDocument) -> ComfortCategory {
    checks::parse_category(&zone.comfort_category)
}
