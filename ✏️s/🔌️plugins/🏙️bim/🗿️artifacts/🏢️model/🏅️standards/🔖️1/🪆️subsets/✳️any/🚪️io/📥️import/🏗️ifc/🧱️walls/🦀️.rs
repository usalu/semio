//! 🧱️ Walls and their openings. A wall is read from its `Axis` representation, its swept `Body` (height and base level), and its layer set (type and location line); an opening from its void,
//! its filling window or door, and the position of its placement projected onto the host axis.

use super::data::label;
use super::frames::Rigid;
use super::reader::{real, text, Section};
use super::spatial::authoring_of;
use super::Import;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::offsets_of;
use crate::{Axis, DoorLeaves, DoorType, Layer, LayerFunction, LocationLine, Opening, OpeningKind, Phase, Point2, Swing, TopConstraint, Wall, WallType, WindowType};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;

fn phase(name: &str) -> Phase {
    match name {
        "Existing" => Phase::Existing,
        "Demolished" => Phase::Demolished,
        "Temporary" => Phase::Temporary,
        _ => Phase::New,
    }
}

fn layers_of(i: &Import<'_>, wall_ifc: u64) -> Option<(Vec<Layer>, Option<(bool, f64)>, String)> {
    let definition = *i.doc.index.materials.get(&wall_ifc)?;
    let (set, usage) = match i.doc.args(definition, "IFCMATERIALLAYERSETUSAGE") {
        Some(usage) => (usage[0].as_ref_id()?, Some((usage[2].as_enum() == Some("NEGATIVE"), real(usage, 3).unwrap_or(0.0)))),
        None => (definition, None),
    };
    let args = i.doc.args(set, "IFCMATERIALLAYERSET")?;
    let layers = args[0]
        .as_list()?
        .iter()
        .filter_map(|layer| i.doc.follow_args(layer, "IFCMATERIALLAYER"))
        .map(|layer| Layer { material: layer[0].as_ref_id().and_then(|material| i.material_ids.get(&material)).cloned().unwrap_or_default(), thickness: real(layer, 1).unwrap_or(0.0), function: LayerFunction::Structure })
        .collect();
    Some((layers, usage, text(args, 1)))
}

fn wall_type_for(i: &mut Import<'_>, wall_ifc: u64, layers: Vec<Layer>, name: &str) -> String {
    if let Some(known) = i.doc.index.types.get(&wall_ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.wall_types.contains_key(*id)) {
        return known.clone();
    }
    if let Some((id, _)) = i.model.wall_types.iter().find(|(_, kind)| kind.layers.len() == layers.len() && kind.layers.iter().zip(&layers).all(|(a, b)| a.material == b.material && (a.thickness - b.thickness).abs() < 1e-9)) {
        return id.clone();
    }
    let thickness: f64 = layers.iter().map(|layer| layer.thickness).sum();
    let base = if name.is_empty() { format!("wt-{}", (thickness * 1000.0).round() as i64) } else { format!("wt-{}", Import::slug(name)) };
    let id = Import::unused(&base, |candidate| i.model.wall_types.contains_key(candidate));
    i.model.wall_types.insert(id.clone(), WallType { name: if name.is_empty() { format!("Wall {} mm", (thickness * 1000.0).round() as i64) } else { name.to_string() }, layers });
    id
}

fn location_for(i: &Import<'_>, wall: &Wall, left: f64) -> LocationLine {
    [LocationLine::Center, LocationLine::Interior, LocationLine::Exterior, LocationLine::CoreCenter]
        .into_iter()
        .find(|location| (offsets_of(&i.model, &Wall { location: *location, ..wall.clone() }).left - left).abs() < 1e-6)
        .unwrap_or(LocationLine::Center)
}

fn wall(i: &mut Import<'_>, instance_id: u64, args: &[semio_s_artifact_stdio_ifc::part21::Part21Value], entity: &str) {
    let label_text = text(args, 2);
    let Some(storey) = i.storey_of(instance_id) else {
        i.skip(entity, &label_text, "it is not contained in a storey");
        return;
    };
    let id = super::reader::Doc::identity(args, entity).unwrap_or_else(|| format!("w-{instance_id}"));
    let to_building = i.in_building(&args[5], &storey);
    let body = i.doc.body(args);
    let edge = i.doc.axis_edge(args).or_else(|| match body.as_ref().map(|body| &body.section) {
        Some(Section::Rectangle { width, .. }) => Some(super::reader::Edge { start: [0.0, 0.0], end: [*width, 0.0], bulge: 0.0 }),
        _ => None,
    });
    let (Some(edge), Some(body)) = (edge, body) else {
        i.skip(entity, &label_text, "it has no axis and no swept body");
        return;
    };
    let map = |point: [f64; 2]| {
        let world = to_building.point([point[0], point[1], 0.0]);
        Point2 { x: world[0], y: world[1] }
    };
    let (start, end) = (map(edge.start), map(edge.end));
    let axis = if edge.bulge.abs() < 1e-12 { Axis::Line { start, end } } else { Axis::Arc { start, end, bulge: edge.bulge } };
    let height = body.depth * body.direction[2].abs();
    let base_z = to_building.point(body.position.origin)[2];
    let found = layers_of(i, instance_id);
    let (layers, usage, set_name) = found.unwrap_or_else(|| {
        let thickness = match &body.section {
            Section::Rectangle { depth, .. } => *depth,
            _ => 0.0,
        };
        (vec![Layer { material: String::new(), thickness, function: LayerFunction::Structure }], None, String::new())
    });
    let thickness: f64 = layers.iter().map(|layer| layer.thickness).sum();
    let wall_type = wall_type_for(i, instance_id, layers, &set_name);
    let left = match (usage, &body.section) {
        (Some((negative, offset)), _) => if negative { offset } else { offset + thickness },
        (None, Section::Rectangle { centre, .. }) => centre[1] + thickness / 2.0,
        _ => thickness / 2.0,
    };
    let elevation = i.levels.get(&storey).map_or(0.0, |level| level.elevation);
    let mut row = Wall { storey, wall_type, axis, location: LocationLine::Center, base_offset: base_z - elevation, top: TopConstraint::Unconnected { height }, phase: phase(&label(&authoring_of(&i.doc, instance_id), "Phase").unwrap_or_default()), name: if label_text == id { String::new() } else { label_text } };
    row.location = location_for(i, &row, left);
    i.model.walls.insert(id.clone(), row);
    i.ids.insert(instance_id, id);
}

fn window_type(i: &mut Import<'_>, fill_ifc: u64, fill: &[semio_s_artifact_stdio_ifc::part21::Part21Value], width: f64, height: f64, sill: f64) -> String {
    if let Some(known) = i.doc.index.types.get(&fill_ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.window_types.contains_key(*id)) {
        return known.clone();
    }
    let row = WindowType { name: text(fill, 2), width, height, sill, frame_width: 0.05, frame_depth: 0.08, panes: 1, material: String::new() };
    if let Some((id, _)) = i.model.window_types.iter().find(|(_, known)| **known == row) {
        return id.clone();
    }
    let id = Import::unused("wnd-imported", |candidate| i.model.window_types.contains_key(candidate));
    i.model.window_types.insert(id.clone(), row);
    id
}

fn door_type(i: &mut Import<'_>, fill_ifc: u64, fill: &[semio_s_artifact_stdio_ifc::part21::Part21Value], width: f64, height: f64) -> String {
    if let Some(known) = i.doc.index.types.get(&fill_ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.door_types.contains_key(*id)) {
        return known.clone();
    }
    let row = DoorType { name: text(fill, 2), width, height, frame_width: 0.05, frame_depth: 0.1, leaves: DoorLeaves::Single, swing: Swing::Left, material: String::new() };
    if let Some((id, _)) = i.model.door_types.iter().find(|(_, known)| **known == row) {
        return id.clone();
    }
    let id = Import::unused("dr-imported", |candidate| i.model.door_types.contains_key(candidate));
    i.model.door_types.insert(id.clone(), row);
    id
}

fn opening(i: &mut Import<'_>, host_ifc: u64, opening_ifc: u64) {
    let (Some(host), Some(args)) = (i.ids.get(&host_ifc).cloned(), i.doc.args(opening_ifc, "IFCOPENINGELEMENT")) else { return };
    let Some(wall_row) = i.model.walls.get(&host).cloned() else { return };
    let id = super::reader::Doc::identity(args, "IFCOPENINGELEMENT").unwrap_or_else(|| format!("o-{opening_ifc}"));
    let Some(body) = i.doc.body(args) else {
        i.skip("IFCOPENINGELEMENT", &id, "it has no swept body");
        return;
    };
    let to_building = i.in_building(&args[5], &wall_row.storey);
    let origin = to_building.point(body.position.origin);
    let width = match body.section {
        Section::Rectangle { width, .. } => width,
        _ => {
            i.skip("IFCOPENINGELEMENT", &id, "its profile is not a rectangle");
            return;
        }
    };
    let height = body.depth * body.direction[2].abs();
    let segment = match &wall_row.axis {
        Axis::Line { start, end } => BulgeSeg::line(Point::new(start.x, start.y), Point::new(end.x, end.y)),
        Axis::Arc { start, end, bulge } => BulgeSeg::new(Point::new(start.x, start.y), Point::new(end.x, end.y), *bulge),
    };
    let offset = super::frames::snap(segment.closest(Point::new(origin[0], origin[1])).t * segment.length());
    let base_z = i.levels.get(&wall_row.storey).map_or(0.0, |level| level.elevation) + wall_row.base_offset;
    let sill = origin[2] - base_z;
    let filler = i.doc.index.filling.get(&opening_ifc).copied();
    let window = filler.and_then(|fill| i.doc.args(fill, "IFCWINDOW").map(|args| (fill, args)));
    let door = filler.and_then(|fill| i.doc.args(fill, "IFCDOOR").map(|args| (fill, args)));
    let (kind, kind_sill, kind_size) = if let Some((fill, fill_args)) = window {
        let window_id = window_type(i, fill, fill_args, width, height, sill);
        let row = &i.model.window_types[&window_id];
        let (type_sill, size) = (row.sill, (row.width, row.height));
        (OpeningKind::Window { window_type: window_id }, type_sill, Some(size))
    } else if let Some((fill, fill_args)) = door {
        let door_id = door_type(i, fill, fill_args, width, height);
        let row = &i.model.door_types[&door_id];
        let size = (row.width, row.height);
        (OpeningKind::Door { door_type: door_id }, 0.0, Some(size))
    } else {
        (OpeningKind::Void { width, height }, 0.0, None)
    };
    let differs = |own: f64, kind: Option<f64>| kind.filter(|kind| (own - kind).abs() > 1e-9).map(|_| own);
    let sill_override = ((sill - kind_sill).abs() > 1e-9).then_some(sill);
    let flip_facing = filler.and_then(|fill| i.doc.args(fill, if window.is_some() { "IFCWINDOW" } else { "IFCDOOR" })).and_then(|fill| i.doc.follow_args(&fill[5], "IFCLOCALPLACEMENT")).map(|placement| i.doc.axis_placement(&placement[1])).is_some_and(|placement: Rigid| placement.z[1] > 0.0);
    let flip_hand = filler.map(|fill| authoring_of(&i.doc, fill)).is_some_and(|rows| label(&rows, "FlipHand").as_deref() == Some("true"));
    let name = filler.and_then(|fill| i.doc.get(fill)).and_then(|instance| instance.primary()).map(|(_, fill_args)| text(fill_args, 2)).unwrap_or_else(|| text(args, 2));
    i.model.openings.insert(
        id.clone(),
        Opening { host, kind, offset, sill_override, width: differs(width, kind_size.map(|size| size.0)), height: differs(height, kind_size.map(|size| size.1)), flip_hand, flip_facing, name: if name == id { String::new() } else { name } },
    );
    i.ids.insert(opening_ifc, id.clone());
    if let Some(fill) = filler {
        i.ids.insert(fill, id);
    }
}

/// 🧱️ Reads every wall, then every opening of those walls.
pub fn read(i: &mut Import<'_>) {
    for entity in ["IFCWALLSTANDARDCASE", "IFCWALL"] {
        for (instance, args) in i.doc.rows(entity) {
            wall(i, instance.id, args, entity);
        }
    }
    let voids: Vec<(u64, Vec<u64>)> = i.doc.index.voids.iter().map(|(host, openings)| (*host, openings.clone())).collect();
    for (host, openings) in voids {
        for opening_ifc in openings {
            opening(i, host, opening_ifc);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
