//! 📏️ The subject's report of an SVG export, in the shape the lxml + shapely oracle measures from the committed file: per storey the regions, lines and texts, the paths per style class, the
//! arc segments, and the area of the straight cut poché and the length of the straight lines per style. Built from the plans and the sheet frames the writer snaps its coordinates with,
//! so the numbers are those of the written geometry. Arcs are counted, never measured (a third-party library can only sample them).

use super::path::{has_arc, is_arc, path_length, points, ring_area, MM_PER_METRE};
use super::sheet::{layout, Slot};
use super::style::{style_class, STYLE_CLASSES};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{compute_plan_linework, PlanLinework, PlanStyle};
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 📊️ What the oracle measures of one storey group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoreyReport {
    pub name: String,
    pub level: i32,
    pub regions: usize,
    pub lines: usize,
    pub texts: usize,
    pub styles: BTreeMap<String, usize>,
    pub arcs: usize,
    pub poche_area: f64,
    pub line_length: BTreeMap<String, f64>,
}

/// 📊️ The report of one export: one entry per storey id, and the sheet size.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub width: f64,
    pub height: f64,
    pub storeys: BTreeMap<String, StoreyReport>,
}

const SQUARE: f64 = MM_PER_METRE * MM_PER_METRE;

fn report_of(slot: &Slot, plan: &PlanLinework) -> StoreyReport {
    let mut report = StoreyReport { name: slot.name.clone(), level: slot.level, regions: plan.regions.len(), lines: plan.polylines.len(), texts: plan.texts.len(), ..StoreyReport::default() };
    STYLE_CLASSES.iter().for_each(|class| {
        report.styles.insert(class.to_string(), 0);
        report.line_length.insert(class.to_string(), 0.0);
    });
    for region in &plan.regions {
        *report.styles.entry(style_class(region.style).into()).or_default() += 1;
        let rings = std::iter::once(&region.outer).chain(region.holes.iter());
        report.arcs += rings.clone().map(|ring| ring.iter().filter(|vertex| is_arc(vertex)).count()).sum::<usize>();
        if region.style == PlanStyle::Cut && !rings.clone().any(|ring| has_arc(ring, true)) {
            let area = ring_area(&points(&region.outer, &slot.frame)) - region.holes.iter().map(|hole| ring_area(&points(hole, &slot.frame))).sum::<f64>();
            report.poche_area += area / SQUARE;
        }
    }
    for line in &plan.polylines {
        *report.styles.entry(style_class(line.style).into()).or_default() += 1;
        let drawn = if line.closed { line.vertices.len() } else { line.vertices.len().saturating_sub(1) };
        report.arcs += line.vertices[..drawn].iter().filter(|vertex| is_arc(vertex)).count();
        if !has_arc(&line.vertices, line.closed) {
            *report.line_length.entry(style_class(line.style).into()).or_default() += path_length(&points(&line.vertices, &slot.frame), line.closed) / MM_PER_METRE;
        }
    }
    report
}

/// 📊️ The report of already inferred plans.
pub fn project(model: &ModelSnapshot, plans: &BTreeMap<String, PlanLinework>) -> Projection {
    let layout = layout(model, plans);
    let storeys = layout.slots.iter().map(|slot| (slot.storey.clone(), report_of(slot, &plans[&slot.storey]))).collect();
    Projection { width: super::path::snap(layout.width), height: super::path::snap(layout.height), storeys }
}

/// 📊️ The report of `model`'s export.
pub fn projection(model: &ModelSnapshot) -> Projection {
    project(model, &compute_plan_linework(model))
}

fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

impl Projection {
    /// 🧾️ The report as compact JSON, the shape the oracle returns.
    pub fn to_json(&self) -> String {
        let map = |values: &BTreeMap<String, f64>| values.iter().map(|(class, value)| format!("{}:{value:?}", quote(class))).collect::<Vec<_>>().join(",");
        let counts = |values: &BTreeMap<String, usize>| values.iter().map(|(class, value)| format!("{}:{value}", quote(class))).collect::<Vec<_>>().join(",");
        let storeys = self
            .storeys
            .iter()
            .map(|(id, row)| {
                format!(
                    "{}:{{\"name\":{},\"level\":{},\"regions\":{},\"lines\":{},\"texts\":{},\"styles\":{{{}}},\"arcs\":{},\"pocheArea\":{:?},\"lineLength\":{{{}}}}}",
                    quote(id),
                    quote(&row.name),
                    row.level,
                    row.regions,
                    row.lines,
                    row.texts,
                    counts(&row.styles),
                    row.arcs,
                    row.poche_area,
                    map(&row.line_length)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{{\"width\":{:?},\"height\":{:?},\"storeys\":{{{storeys}}}}}", self.width, self.height)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
