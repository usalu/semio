//! 🪧️ Dimensions, tags, text notes and leaders as `IfcAnnotation` (IFC 2x3 `IfcProduct`, contained in its storey). The representation is the plan drawing of the annotation, taken from the plan the inference derived
//! (never recomputed): the dimension, extension and leader lines and the end marks as `IfcPolyline` (a dot as `IfcCircle`), every text as `IfcTextLiteralWithExtent` placed by the middle of its baseline, all in the
//! `Annotation` representation of type `Annotation2D`. `Name` is the authored name else the id, `ObjectType` names the kind (`Dimension`, `Tag`, `TextNote`, `Leader`), `Description` the printed text, and `Semio_Authoring` carries the authored record so the
//! importer finds it again; the derived measured total of a dimension is the element quantity `Semio_DimensionValue`.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcproductextension/lexical/ifcannotation.htm

use super::data::{label, number};
use super::writer::{en, opt_text, real, refs, rf, text, unset, V};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::annotation_layout::text_width;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanLinework, PlanVertex};

fn curve(x: &mut Export<'_>, vertices: &[PlanVertex], closed: bool) -> Option<u64> {
    if vertices.len() == 2 && closed && vertices.iter().all(|vertex| vertex.bulge != 0.0) {
        let centre = [(vertices[0].x + vertices[1].x) / 2.0, (vertices[0].y + vertices[1].y) / 2.0];
        let radius = (vertices[0].x - vertices[1].x).hypot(vertices[0].y - vertices[1].y) / 2.0;
        let position = x.ifc.axis2(centre, None);
        return Some(x.ifc.add("IFCCIRCLE", vec![rf(position), real(radius)]));
    }
    if vertices.iter().any(|vertex| vertex.bulge != 0.0) {
        return None;
    }
    let mut points: Vec<u64> = vertices.iter().map(|vertex| x.ifc.point2([vertex.x, vertex.y])).collect();
    if closed {
        points.extend(points.first().copied());
    }
    Some(x.ifc.add("IFCPOLYLINE", vec![refs(&points)]))
}

fn items(x: &mut Export<'_>, plan: &PlanLinework, id: &str) -> Vec<u64> {
    let mut found: Vec<u64> = plan.polylines.iter().filter(|line| line.kind.is_notation() && line.element == id).filter_map(|line| curve(x, &line.vertices, line.closed)).collect();
    for item in plan.texts.iter().filter(|item| item.kind.is_notation() && item.element == id) {
        let placement = x.ifc.axis2([item.x, item.y], Some([item.rotation.cos(), item.rotation.sin()]));
        let extent = x.ifc.add("IFCPLANAREXTENT", vec![real(text_width(&item.label, item.height)), real(item.height)]);
        found.push(x.ifc.add("IFCTEXTLITERALWITHEXTENT", vec![text(&item.label), rf(placement), en("LEFT"), rf(extent), text("bottom-middle")]));
    }
    found
}

struct Row<'a> {
    id: &'a str,
    storey: &'a str,
    kind: &'static str,
    name: &'a str,
    printed: String,
    authoring: Vec<(&'static str, V)>,
    total: Option<f64>,
}

fn write(x: &mut Export<'_>, row: Row<'_>) {
    let Some(storey) = x.storeys.get(row.storey).copied() else {
        x.skip(row.kind, row.id, "its storey is not written");
        return;
    };
    let plan = x.inferred.plan_linework.get(row.storey);
    let drawn = plan.map(|plan| items(x, plan, row.id)).unwrap_or_default();
    if drawn.is_empty() {
        x.skip(row.kind, row.id, "it draws nothing (an anchor has no geometry or the element is missing)");
    }
    let shape = (!drawn.is_empty()).then(|| {
        let annotation = x.ifc.shape(x.ifc.footprint, "Annotation", "Annotation2D", &drawn);
        x.ifc.definition(&[annotation])
    });
    let origin = x.ifc.origin;
    let placement = x.ifc.place(Some(storey.placement), origin);
    let entity = x.ifc.rooted("IFCANNOTATION", row.id, &Export::label(row.name, row.id), &row.printed, vec![opt_text(row.kind), rf(placement), shape.map_or_else(unset, rf)]);
    x.contain(row.storey, row.id, entity);
    let mut authoring = vec![("Id", label(row.id)), ("Kind", label(row.kind))];
    authoring.extend(row.authoring);
    x.links.authoring.push((entity, authoring));
    if let Some(total) = row.total {
        x.links.quantities.push((entity, "Semio_DimensionValue", vec![Quantity::Length("Total", total)]));
    }
}

fn json<T: semio_framework_value::ToValue>(value: &T) -> V {
    label(&semio_framework_pack_json::to_json_string(value))
}

/// 🪧️ Writes every dimension, tag, text note and leader.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let layouts = &x.inferred.annotations;
    for (id, dimension) in &model.dimensions {
        let layout = layouts.get(&dimension.storey).and_then(|set| set.dimensions.get(id));
        let printed = layout.map(|layout| layout.total_text.clone()).unwrap_or_default();
        let total = layout.filter(|layout| layout.complete).map(|layout| layout.total);
        let mut authoring = vec![("Style", label(&dimension.style)), ("Angle", number(dimension.angle)), ("Offset", number(dimension.offset)), ("Anchors", json(&dimension.anchors))];
        authoring.extend(dimension.lock.map(|lock| ("Lock", number(lock))));
        write(x, Row { id, storey: &dimension.storey, kind: "Dimension", name: &dimension.name, printed, authoring, total });
    }
    for (id, tag) in &model.tags {
        let printed = layouts.get(&tag.storey).and_then(|set| set.tags.get(id)).map(|layout| layout.text.clone()).unwrap_or_default();
        let authoring = vec![("Style", label(&tag.style)), ("Element", label(&tag.element)), ("Category", label(&format!("{:?}", tag.category))), ("Offset", json(&tag.offset))];
        write(x, Row { id, storey: &tag.storey, kind: "Tag", name: "", printed, authoring, total: None });
    }
    for (id, note) in &model.text_notes {
        let authoring = vec![("Style", label(&note.style)), ("Position", json(&note.position)), ("Rotation", number(note.rotation))];
        write(x, Row { id, storey: &note.storey, kind: "TextNote", name: "", printed: note.text.clone(), authoring, total: None });
    }
    for (id, leader) in &model.leaders {
        let authoring = vec![("Style", label(&leader.style)), ("Anchor", json(&leader.anchor)), ("Offset", json(&leader.offset))];
        write(x, Row { id, storey: &leader.storey, kind: "Leader", name: "", printed: leader.text.clone(), authoring, total: None });
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
