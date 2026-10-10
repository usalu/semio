//! 📏️ The subject's report of an SVG export, in the shape the lxml + shapely oracle measures from the committed file: per view the kind, scale, regions, lines and texts, the paths per style class, the
//! arc segments, and the area of the straight cut poché and the length of the straight lines per style. Built from the plans and the sheet frames the writer snaps its coordinates with,
//! so the numbers are those of the written geometry. Arcs are counted, never measured (a third-party library can only sample them).

use super::path::{has_arc, is_arc, path_length, points, ring_area, snap};
use super::sheet::{layout, Slot};
use super::drawing::tint;
use super::style::{annotated, kind_class, style_class, view_class, STYLE_CLASSES};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanKind, PlanLinework, PlanStyle};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::ModelSnapshot;
use std::collections::BTreeMap;

/// 🪧️ What the oracle measures of one kind of annotation primitive: how many paths or texts of the kind a view draws and the straight length of its paths in metres.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NotationReport {
    pub count: usize,
    pub length: f64,
}

/// 🪑️ What the oracle measures of the primitives of one component or routed element: the straight length of its centre lines and the area of its outlines and bands in metres, the paths per kind and the colours of its service.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementReport {
    pub axis: f64,
    pub area: f64,
    pub count: BTreeMap<String, usize>,
    pub stroke: Vec<String>,
}

/// 🪑️ Whether a primitive kind belongs to a component or a routed element.
pub fn is_element(kind: PlanKind) -> bool {
    matches!(kind, PlanKind::ComponentOutline | PlanKind::ComponentFront | PlanKind::ComponentConnector | PlanKind::MepAxis | PlanKind::MepBand | PlanKind::MepDrop)
}

/// 📊️ What the oracle measures of one view group.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewReport {
    pub name: String,
    pub kind: String,
    pub scale: u32,
    pub regions: usize,
    pub lines: usize,
    pub texts: usize,
    pub styles: BTreeMap<String, usize>,
    pub arcs: usize,
    pub poche_area: f64,
    pub line_length: BTreeMap<String, f64>,
    pub notation: BTreeMap<String, NotationReport>,
    pub printed: Vec<String>,
    pub elements: BTreeMap<String, ElementReport>,
}

/// 📊️ The report of one export: one entry per view id, and the sheet size.
#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    pub width: f64,
    pub height: f64,
    pub views: BTreeMap<String, ViewReport>,
}

fn note_colour(report: &mut ElementReport, model: &ModelSnapshot, element: &str, kind: PlanKind) {
    if let Some((_, colour)) = tint(model, element, kind) {
        if !report.stroke.iter().any(|known| known == colour) {
            report.stroke.push(colour.to_string());
            report.stroke.sort();
        }
    }
}

fn report_of(model: &ModelSnapshot, slot: &Slot, plan: &PlanLinework) -> ViewReport {
    let square = slot.frame.mm * slot.frame.mm;
    let mut report = ViewReport { name: slot.name.clone(), kind: view_class(slot.kind).into(), scale: slot.scale, regions: plan.regions.len(), lines: plan.polylines.len(), texts: plan.texts.len(), ..ViewReport::default() };
    STYLE_CLASSES.iter().for_each(|class| {
        report.styles.insert(class.to_string(), 0);
        report.line_length.insert(class.to_string(), 0.0);
    });
    for region in &plan.regions {
        *report.styles.entry(style_class(region.style).into()).or_default() += 1;
        let rings = std::iter::once(&region.outer).chain(region.holes.iter());
        report.arcs += rings.clone().map(|ring| ring.iter().filter(|vertex| is_arc(vertex)).count()).sum::<usize>();
        if is_element(region.kind) {
            let row = report.elements.entry(region.element.clone()).or_default();
            *row.count.entry(kind_class(region.kind).into()).or_default() += 1;
            if !rings.clone().any(|ring| has_arc(ring, true)) {
                row.area += ring_area(&points(&region.outer, &slot.frame)) / square;
            }
            note_colour(row, model, &region.element, region.kind);
        }
        if region.style == PlanStyle::Cut && !rings.clone().any(|ring| has_arc(ring, true)) {
            let area = ring_area(&points(&region.outer, &slot.frame)) - region.holes.iter().map(|hole| ring_area(&points(hole, &slot.frame))).sum::<f64>();
            report.poche_area += area / square;
        }
    }
    for line in &plan.polylines {
        *report.styles.entry(style_class(line.style).into()).or_default() += 1;
        let drawn = if line.closed { line.vertices.len() } else { line.vertices.len().saturating_sub(1) };
        report.arcs += line.vertices[..drawn].iter().filter(|vertex| is_arc(vertex)).count();
        let straight = !has_arc(&line.vertices, line.closed);
        let length = if straight { path_length(&points(&line.vertices, &slot.frame), line.closed) / slot.frame.mm } else { 0.0 };
        *report.line_length.entry(style_class(line.style).into()).or_default() += length;
        if is_element(line.kind) {
            let row = report.elements.entry(line.element.clone()).or_default();
            *row.count.entry(kind_class(line.kind).into()).or_default() += 1;
            if line.kind == PlanKind::MepAxis && straight {
                row.axis += length;
            }
            if line.kind == PlanKind::ComponentOutline && straight {
                row.area += ring_area(&points(&line.vertices, &slot.frame)) / square;
            }
            note_colour(row, model, &line.element, line.kind);
        }
        if annotated(line.kind) {
            let row = report.notation.entry(kind_class(line.kind).into()).or_default();
            row.count += 1;
            row.length += length;
        }
    }
    for text in plan.texts.iter().filter(|text| annotated(text.kind)) {
        report.notation.entry(kind_class(text.kind).into()).or_default().count += 1;
        report.printed.push(text.label.clone());
    }
    report.printed.sort();
    report
}

/// 📊️ The report of already inferred view drawings.
pub fn project(model: &ModelSnapshot, drawings: &BTreeMap<String, ViewLinework>) -> Projection {
    let layout = layout(model, drawings);
    let views = layout.slots.iter().map(|slot| (slot.view.clone(), report_of(model, slot, &drawings[&slot.view].lines))).collect();
    Projection { width: snap(layout.width), height: snap(layout.height), views }
}

/// 📊️ The report of `model`'s export.
pub fn projection(model: &ModelSnapshot) -> Projection {
    registry::with_inference(None, model, |inferred| project(model, &inferred.view_linework))
}

fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

impl Projection {
    /// 🧾️ The report as compact JSON, the shape the oracle returns.
    pub fn to_json(&self) -> String {
        let map = |values: &BTreeMap<String, f64>| values.iter().map(|(class, value)| format!("{}:{value:?}", quote(class))).collect::<Vec<_>>().join(",");
        let notation = |values: &BTreeMap<String, NotationReport>| values.iter().map(|(class, row)| format!("{}:{{\"count\":{},\"length\":{:?}}}", quote(class), row.count, row.length)).collect::<Vec<_>>().join(",");
        let counts = |values: &BTreeMap<String, usize>| values.iter().map(|(class, value)| format!("{}:{value}", quote(class))).collect::<Vec<_>>().join(",");
        let elements = |values: &BTreeMap<String, ElementReport>| {
            if values.is_empty() {
                return String::new();
            }
            let rows = values.iter().map(|(id, row)| format!("{}:{{\"axis\":{:?},\"area\":{:?},\"count\":{{{}}},\"stroke\":[{}]}}", quote(id), row.axis, row.area, counts(&row.count), row.stroke.iter().map(|colour| quote(colour)).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
            format!(",\"elements\":{{{rows}}}")
        };
        let views = self
            .views
            .iter()
            .map(|(id, row)| {
                format!(
                    "{}:{{\"name\":{},\"kind\":{},\"scale\":{},\"regions\":{},\"lines\":{},\"texts\":{},\"styles\":{{{}}},\"arcs\":{},\"pocheArea\":{:?},\"lineLength\":{{{}}},\"notation\":{{{}}},\"printed\":[{}]{}}}",
                    quote(id),
                    quote(&row.name),
                    quote(&row.kind),
                    row.scale,
                    row.regions,
                    row.lines,
                    row.texts,
                    counts(&row.styles),
                    row.arcs,
                    row.poche_area,
                    map(&row.line_length),
                    notation(&row.notation),
                    row.printed.iter().map(|text| quote(text)).collect::<Vec<_>>().join(","),
                    elements(&row.elements)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{{\"width\":{:?},\"height\":{:?},\"views\":{{{views}}}}}", self.width, self.height)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
