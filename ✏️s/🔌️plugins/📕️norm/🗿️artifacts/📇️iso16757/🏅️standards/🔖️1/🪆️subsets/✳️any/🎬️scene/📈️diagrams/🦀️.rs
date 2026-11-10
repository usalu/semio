//! 📈️ Diagrams: per geometry object a plan (width x, depth z) and an elevation (width x, height y) of the body primitives, space envelopes and ports, plus the space table; each with a table twin whose columns list every envelope corner.

use super::kit::{base_id, object_ref};
use super::layout::{extent, leaf_extent, leaves};
use super::model::part_2::GeometryObject;
use super::model::Iso16757Snapshot;
use super::norm::CheckReport;
use super::scene::projection_kit::diagram_parts::{self, checked_marker_id, column, stroke};
use super::scene::*;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn ring(lo: [f64; 3], hi: [f64; 3], axes: (usize, usize)) -> Vec<Vec2> {
    let (a, b) = axes;
    [(lo[a], lo[b]), (hi[a], lo[b]), (hi[a], hi[b]), (lo[a], hi[b]), (lo[a], lo[b])].into_iter().map(|(x, y)| Vec2::new(x, y)).collect()
}

fn known(report: &CheckReport, id: &str) -> Option<String> {
    report.checks.iter().find(|c| c.subject.entity_id == id).map(|c| c.id.clone())
}

fn view(document: &Iso16757Snapshot, report: &CheckReport, id: &str, object: &GeometryObject, axes: (usize, usize), key: &str, names: ((&str, &str), (&str, &str), (&str, &str))) -> Diagram2d {
    let bounds = extent(document, object);
    let pad = 0.05 * (bounds.max[axes.0] - bounds.min[axes.0]).max(bounds.max[axes.1] - bounds.min[axes.1]).max(0.1);
    let mut series = Vec::new();
    let found = object.shape.as_ref().map(|shape| leaves(document, shape)).unwrap_or_default();
    for (index, leaf) in found.iter().enumerate() {
        let (lo, hi) = leaf_extent(leaf);
        series.push(Series::Polyline { id: format!("body-{index}"), label: loc(&format!("Body primitive {} ({})", index + 1, leaf.kind), &format!("Grundkörper {} ({})", index + 1, leaf.kind)), points: ring(lo, hi, axes), style: stroke(Tone::Info, 2.0, Dash::Solid) });
    }
    for space in &object.spaces {
        series.push(Series::Polyline { id: format!("space-{}", space.id), label: loc(&format!("Space {}", space.id), &format!("Raum {}", space.id)), points: ring(space.bounds.min, space.bounds.max, axes), style: stroke(Tone::Warning, 1.5, Dash::Dashed) });
    }
    let checked = known(report, id);
    let markers = object
        .ports
        .iter()
        .map(|p| Marker { id: checked_marker_id(&format!("port-{}", p.id), &checked), at: Vec2::new(p.position[axes.0], p.position[axes.1]), shape: MarkerShape::Diamond, tone: Tone::Neutral, label: loc(&format!("Port {} ({})", p.id, p.medium), &format!("Anschluss {} ({})", p.id, p.medium)), utilisation: None, bound_check: checked.clone(), binding: None })
        .collect();
    let axis = |name: (&str, &str), axis: usize| diagram_parts::axis(name.0, name.1, QuantityKind::Length, "m", AxisScale::Linear, Some((bounds.min[axis] - pad, bounds.max[axis] + pad)));
    let rows = object.spaces.iter().map(|s| s.bounds).collect::<Vec<_>>();
    let collect = |pick: &dyn Fn(&super::model::part_2::BoundingBox) -> f64| rows.iter().map(pick).collect::<Vec<f64>>();
    let table = vec![
        column("Minimum x", "Minimum x", "m", collect(&|b| b.min[0])),
        column("Maximum x", "Maximum x", "m", collect(&|b| b.max[0])),
        column("Minimum y", "Minimum y", "m", collect(&|b| b.min[1])),
        column("Maximum y", "Maximum y", "m", collect(&|b| b.max[1])),
        column("Minimum z", "Minimum z", "m", collect(&|b| b.min[2])),
        column("Maximum z", "Maximum z", "m", collect(&|b| b.max[2])),
    ];
    Diagram2d {
        id: format!("{key}:{id}"),
        title: loc(&format!("{} – {id}", names.0.0), &format!("{} – {id}", names.0.1)),
        description: loc(&format!("{} of the body primitives, space envelopes and ports of the geometry object (ISO 16757-2).", names.1.0), &format!("{} der Grundkörper, Raumhüllen und Anschlüsse des Geometrieobjekts (ISO 16757-2).", names.1.1)),
        axes: vec![axis(names.2, axes.0), axis(if axes.1 == 2 { ("Depth z", "Tiefe z") } else { ("Height y", "Höhe y") }, axes.1)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(object_ref(id)),
        legend: true,
        table_twin: DataTable { columns: table },
    }
}

/// 📈️ The diagrams of every geometry object, or of the object `focus` names.
pub fn diagrams(document: &Iso16757Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for (id, object) in document.geometry.objects.iter().filter(|(id, _)| focus.is_none_or(|f| base_id(f) == id.as_str())) {
        out.push(view(document, report, id, object, (0, 2), "plan", (("Plan view", "Grundriss"), ("Plan view", "Grundriss"), ("Width x", "Breite x"))));
        out.push(view(document, report, id, object, (0, 1), "elevation", (("Elevation", "Ansicht"), ("Elevation", "Ansicht"), ("Width x", "Breite x"))));
    }
    out
}
