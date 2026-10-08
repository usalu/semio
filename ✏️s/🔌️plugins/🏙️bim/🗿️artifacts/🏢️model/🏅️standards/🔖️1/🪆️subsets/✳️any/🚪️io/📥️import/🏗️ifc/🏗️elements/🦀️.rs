//! 🏗️ Slabs, columns, beams, spaces and grids of an IFC file: the swept `Body` gives the outline or profile and the extent, the placement the position, rotation and level.

use super::reader::{opt_text, text, Doc, Loop, Section};
use super::Import;
use crate::{Beam, BeamType, Column, ColumnType, GridLine, Layer, LayerFunction, Point2, Profile, Slab, SlabType, Space, SpaceBoundary, TopConstraint, Vertex};
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn vertices(ring: &Loop, to_building: &impl Fn([f64; 2]) -> Point2) -> Vec<Vertex> {
    ring.iter().map(|(point, bulge)| Vertex { point: to_building(*point), bulge: *bulge }).collect()
}

fn profile_of(section: &Section) -> Option<Profile> {
    Some(match section {
        Section::Rectangle { width, depth, .. } => Profile::Rectangle { width: *width, depth: *depth },
        Section::Circle { diameter } => Profile::Circle { diameter: *diameter },
        Section::IShape { width, depth, web, flange } => Profile::IShape { width: *width, depth: *depth, web: *web, flange: *flange },
        Section::Outline { outer, .. } => Profile::Custom { outline: outer.iter().map(|(point, bulge)| Vertex { point: Point2 { x: point[0], y: point[1] }, bulge: *bulge }).collect() },
    })
}

fn profile_top(section: &Section) -> f64 {
    match section {
        Section::Rectangle { centre, depth, .. } => centre[1] + depth / 2.0,
        Section::Circle { diameter } => diameter / 2.0,
        Section::IShape { depth, .. } => depth / 2.0,
        Section::Outline { outer, .. } => outer.iter().map(|(point, _)| point[1]).fold(f64::NEG_INFINITY, f64::max),
    }
}

fn layered_slab_type(i: &mut Import<'_>, slab_ifc: u64, thickness: f64) -> String {
    if let Some(known) = i.doc.index.types.get(&slab_ifc).and_then(|kind| i.type_ids.get(kind)).filter(|id| i.model.slab_types.contains_key(*id)) {
        return known.clone();
    }
    let layers = vec![Layer { material: i.doc.index.materials.get(&slab_ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default(), thickness, function: LayerFunction::Structure }];
    if let Some((id, _)) = i.model.slab_types.iter().find(|(_, kind)| kind.layers.len() == 1 && (kind.layers[0].thickness - thickness).abs() < 1e-9 && kind.layers[0].material == layers[0].material) {
        return id.clone();
    }
    let id = Import::unused(&format!("slt-{}", (thickness * 1000.0).round() as i64), |candidate| i.model.slab_types.contains_key(candidate));
    i.model.slab_types.insert(id.clone(), SlabType { name: format!("Slab {} mm", (thickness * 1000.0).round() as i64), layers });
    id
}

fn slab(i: &mut Import<'_>, ifc: u64, args: &[Part21Value]) {
    if args.get(8).and_then(Part21Value::as_enum) == Some("ROOF") {
        return;
    }
    let label_text = text(args, 2);
    let (Some(storey), Some(body)) = (i.storey_of(ifc), i.doc.body(args)) else {
        i.skip("IFCSLAB", &label_text, "it is not in a storey or has no swept body");
        return;
    };
    let id = Doc::identity(args, "IFCSLAB").unwrap_or_else(|| format!("sl-{ifc}"));
    let to_building = i.in_building(&args[5], &storey);
    let map = |point: [f64; 2]| {
        let world = to_building.point(body.position.point([point[0], point[1], 0.0]));
        Point2 { x: world[0], y: world[1] }
    };
    let (outer, holes) = match &body.section {
        Section::Outline { outer, holes } => (outer.clone(), holes.clone()),
        Section::Rectangle { centre, width, depth } => (vec![([centre[0] - width / 2.0, centre[1] - depth / 2.0], 0.0), ([centre[0] + width / 2.0, centre[1] - depth / 2.0], 0.0), ([centre[0] + width / 2.0, centre[1] + depth / 2.0], 0.0), ([centre[0] - width / 2.0, centre[1] + depth / 2.0], 0.0)], Vec::new()),
        _ => {
            i.skip("IFCSLAB", &label_text, "its profile is not an outline");
            return;
        }
    };
    let thickness = body.depth * body.direction[2].abs();
    let top = to_building.point(body.position.origin)[2] + thickness;
    let elevation = i.levels.get(&storey).map_or(0.0, |level| level.elevation);
    let slab_type = layered_slab_type(i, ifc, thickness);
    i.model.slabs.insert(id.clone(), Slab { storey, slab_type, boundary: vertices(&outer, &map), holes: holes.iter().map(|hole| vertices(hole, &map)).collect(), offset: top - elevation, slope: None, name: if label_text == id { String::new() } else { label_text } });
    i.ids.insert(ifc, id);
}

fn column(i: &mut Import<'_>, ifc: u64, args: &[Part21Value]) {
    let label_text = text(args, 2);
    let (Some(storey), Some(body)) = (i.storey_of(ifc), i.doc.body(args)) else {
        i.skip("IFCCOLUMN", &label_text, "it is not in a storey or has no swept body");
        return;
    };
    let Some(profile) = profile_of(&body.section) else { return };
    let id = Doc::identity(args, "IFCCOLUMN").unwrap_or_else(|| format!("c-{ifc}"));
    let to_building = i.in_building(&args[5], &storey);
    let material = i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default();
    let column_type = match i.doc.index.types.get(&ifc).and_then(|kind| i.type_ids.get(kind)).filter(|kind| i.model.column_types.contains_key(*kind)).cloned() {
        Some(known) => {
            if let Some(ColumnType { profile: target @ Profile::Custom { .. }, .. }) = i.model.column_types.get_mut(&known) {
                if matches!(target, Profile::Custom { outline } if outline.is_empty()) {
                    *target = profile.clone();
                }
            }
            known
        }
        None => match i.model.column_types.iter().find(|(_, kind)| kind.profile == profile && kind.material == material) {
            Some((known, _)) => known.clone(),
            None => {
                let id = Import::unused("ct-imported", |candidate| i.model.column_types.contains_key(candidate));
                i.model.column_types.insert(id.clone(), ColumnType { name: "Column".into(), profile: profile.clone(), material });
                id
            }
        },
    };
    let base_z = to_building.point(body.position.origin)[2];
    let elevation = i.levels.get(&storey).map_or(0.0, |level| level.elevation);
    i.model.columns.insert(id.clone(), Column { storey, column_type, position: Point2 { x: to_building.origin[0], y: to_building.origin[1] }, rotation: to_building.heading(), base_offset: base_z - elevation, top: TopConstraint::Unconnected { height: body.depth * body.direction[2].abs() }, name: if label_text == id { String::new() } else { label_text } });
    i.ids.insert(ifc, id);
}

fn beam(i: &mut Import<'_>, ifc: u64, args: &[Part21Value]) {
    let label_text = text(args, 2);
    let (Some(storey), Some(body)) = (i.storey_of(ifc), i.doc.body(args)) else {
        i.skip("IFCBEAM", &label_text, "it is not in a storey or has no swept body");
        return;
    };
    let Some(profile) = profile_of(&body.section) else { return };
    let id = Doc::identity(args, "IFCBEAM").unwrap_or_else(|| format!("b-{ifc}"));
    let to_building = i.in_building(&args[5], &storey);
    let material = i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default();
    let beam_type = match i.doc.index.types.get(&ifc).and_then(|kind| i.type_ids.get(kind)).filter(|kind| i.model.beam_types.contains_key(*kind)).cloned() {
        Some(known) => {
            if let Some(BeamType { profile: target @ Profile::Custom { .. }, .. }) = i.model.beam_types.get_mut(&known) {
                if matches!(target, Profile::Custom { outline } if outline.is_empty()) {
                    *target = profile.clone();
                }
            }
            known
        }
        None => match i.model.beam_types.iter().find(|(_, kind)| kind.profile == profile && kind.material == material) {
            Some((known, _)) => known.clone(),
            None => {
                let id = Import::unused("bt-imported", |candidate| i.model.beam_types.contains_key(candidate));
                i.model.beam_types.insert(id.clone(), BeamType { name: "Beam".into(), profile: profile.clone(), material });
                id
            }
        },
    };
    let direction = to_building.vector([body.direction[0], body.direction[1], body.direction[2]]);
    let start = Point2 { x: to_building.origin[0], y: to_building.origin[1] };
    let end = Point2 { x: start.x + direction[0] * body.depth, y: start.y + direction[1] * body.depth };
    let top = to_building.point(body.position.origin)[2] + profile_top(&body.section);
    let storey_top = i.levels.get(&storey).map_or(0.0, |level| level.top_elevation);
    i.model.beams.insert(id.clone(), Beam { storey, beam_type, start, end, top_offset: top - storey_top, name: if label_text == id { String::new() } else { label_text } });
    i.ids.insert(ifc, id);
}

fn space(i: &mut Import<'_>, ifc: u64, args: &[Part21Value]) {
    let label_text = text(args, 7);
    let Some(storey) = i.doc.index.whole.get(&ifc).and_then(|whole| i.storey_ids.get(whole)).cloned() else {
        i.skip("IFCSPACE", &label_text, "it is not aggregated under a storey");
        return;
    };
    let Some(body) = i.doc.body(args) else {
        i.skip("IFCSPACE", &label_text, "it has no swept body, so its outline is unknown");
        return;
    };
    let Section::Outline { outer, .. } = &body.section else {
        i.skip("IFCSPACE", &label_text, "its profile is not an outline");
        return;
    };
    let id = Doc::identity(args, "IFCSPACE").unwrap_or_else(|| format!("sp-{ifc}"));
    let to_building = i.in_building(&args[5], &storey);
    let map = |point: [f64; 2]| {
        let world = to_building.point(body.position.point([point[0], point[1], 0.0]));
        Point2 { x: world[0], y: world[1] }
    };
    i.model.spaces.insert(id.clone(), Space { storey, number: text(args, 2), name: label_text, boundary: SpaceBoundary::Explicit { outline: vertices(outer, &map) }, usage: text(args, 3) });
    i.ids.insert(ifc, id);
}

fn grid(i: &mut Import<'_>, ifc: u64, args: &[Part21Value]) {
    let Some(building) = i.doc.index.contained.get(&ifc).and_then(|structure| i.ids.get(structure)).cloned() else {
        i.skip("IFCGRID", &text(args, 2), "it is not contained in a building");
        return;
    };
    let world = i.doc.world(&args[5]);
    let to_building = i.building_world.get(&building).map_or(world, |building| world.relative_to(building));
    let mut axes: Vec<u64> = Vec::new();
    for at in [7, 8] {
        for axis in super::reader::refs(args, at) {
            if !axes.contains(&axis) {
                axes.push(axis);
            }
        }
    }
    let ids = opt_text(args, 4).map(|list| list.split(',').map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    for (index, axis) in axes.into_iter().enumerate() {
        let Some(row) = i.doc.args(axis, "IFCGRIDAXIS") else { continue };
        let Some(edge) = i.doc.edge(&row[1]) else { continue };
        let map = |point: [f64; 2]| {
            let world = to_building.point([point[0], point[1], 0.0]);
            Point2 { x: world[0], y: world[1] }
        };
        let id = ids.get(index).cloned().unwrap_or_else(|| format!("g-{ifc}-{}", index + 1));
        i.model.grids.insert(id, GridLine { building: building.clone(), label: text(row, 0), start: map(edge.start), end: map(edge.end) });
    }
    i.ids.insert(ifc, format!("{building}:grid"));
}

/// 🏗️ Reads slabs, columns, beams, spaces and grids.
pub fn read(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCSLAB") {
        slab(i, instance.id, args);
    }
    for (instance, args) in i.doc.rows("IFCCOLUMN") {
        column(i, instance.id, args);
    }
    for (instance, args) in i.doc.rows("IFCBEAM") {
        beam(i, instance.id, args);
    }
    for (instance, args) in i.doc.rows("IFCSPACE") {
        space(i, instance.id, args);
    }
    for (instance, args) in i.doc.rows("IFCGRID") {
        grid(i, instance.id, args);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
