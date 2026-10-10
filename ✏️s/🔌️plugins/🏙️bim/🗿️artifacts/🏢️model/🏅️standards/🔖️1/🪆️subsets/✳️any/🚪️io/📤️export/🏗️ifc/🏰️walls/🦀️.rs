//! 🧱️ Walls and the openings they host. A straight wall without joins is an `IfcWallStandardCase` (rectangle swept along its axis); a curved or joined wall is an `IfcWall`
//! whose arbitrary closed profile is the join-trimmed footprint of `wall-layout`, so the exported volume equals the inferred one. Every wall carries an `Axis` representation and an
//! `IfcMaterialLayerSetUsage`. Openings become `IfcOpeningElement` + `IfcRelVoidsElement`; windows and doors fill them (`IfcRelFillsElement`) with a frame-and-pane body.
//! A wall whose top or base follows a roof, slab or ceiling is the faceted brep of its inferred envelope (the openings stay voids of it), carries the `Semio_WallAttach` set with the authored top, base slab and heights, and is
//! connected to its targets by `IfcRelConnectsElements` (`TopAttach`, `BaseAttach`). An opening with an authored reveal carries its depth and material in `Semio_OpeningReveal`, and its filling sits back from the front face.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcrelconnectselements.htm>

use super::brep::{body_kind, mesh_item, mesh_where};
use super::data::{define, label, number, property_set};
use super::frames::frame_at;
use super::writer::{en, real, refs, rf, unset, Schema, V};
use super::{data, Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::walls::wall_solid;
use crate::standards::v1::subsets::any::schema::authored::sizes::resolve_size;
use crate::standards::v1::subsets::any::schema::authored::plan::segment_of;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
use crate::{Axis, Layer, LayerFunction, ModelSnapshot, Opening, OpeningKind, TopConstraint, Wall, WallType};
use semio_framework_geometry::bulge::{band_loop, BulgeSeg};
use semio_framework_geometry::mesh::TriMesh;
use std::collections::BTreeMap;

/// 🧗️ The property set that records how a wall attaches: the authored top (`TopKind`, `TopTarget`, `TopOffset`, `TopHeight`), the base slab and offset, and the largest height and lowest base level of the inferred layout.
pub const ATTACH_SET: &str = "Semio_WallAttach";

/// 🪟️ The property set that records the authored reveal of an opening.
pub const REVEAL_SET: &str = "Semio_OpeningReveal";

struct Host {
    placement: u64,
    segment: BulgeSeg,
    standard: bool,
    base_z: f64,
    centre_y: f64,
    thickness: f64,
    elevation: f64,
    lifted: bool,
}

/// 🏷️ Writes `rows` as the property set `name` of `entity`; `key` makes the ids stable.
pub fn describe(x: &mut Export<'_>, key: &str, entity: u64, name: &str, rows: Vec<(&str, V)>) {
    let set = property_set(x, key, name, rows);
    define(x, key, &[entity], set);
}

fn envelope(wall: &Wall, layout: &WallLayout, elevation: f64) -> TriMesh {
    let mut solo = ModelSnapshot::default();
    solo.wall_types.insert(wall.wall_type.clone(), WallType { name: String::new(), layers: vec![Layer { material: String::new(), thickness: layout.thickness, function: LayerFunction::Structure }] });
    let one = WallLayout { layer_offsets: vec![layout.offset_left, -layout.offset_right], ..layout.clone() };
    mesh_where(&wall_solid(&solo, wall, &one, &[]), elevation, |_, _| true)
}

fn attach_rows(wall: &Wall, layout: &WallLayout, level: f64) -> Option<Vec<(&'static str, V)>> {
    let attached = matches!(wall.top, TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. });
    if !attached && wall.base_slab.is_none() {
        return None;
    }
    let (kind, target, offset, height) = match &wall.top {
        TopConstraint::Unconnected { height } => ("Unconnected", "", 0.0, *height),
        TopConstraint::StoreyTop { offset } => ("StoreyTop", "", *offset, 0.0),
        TopConstraint::Storey { storey, offset } => ("Storey", storey.as_str(), *offset, 0.0),
        TopConstraint::Roof { roof, offset } => ("Roof", roof.as_str(), *offset, 0.0),
        TopConstraint::Slab { slab, offset } => ("Slab", slab.as_str(), *offset, 0.0),
        TopConstraint::Ceiling { ceiling, offset } => ("Ceiling", ceiling.as_str(), *offset, 0.0),
    };
    Some(vec![
        ("TopKind", label(kind)),
        ("TopTarget", label(target)),
        ("TopOffset", number(offset)),
        ("TopHeight", number(height)),
        ("BaseSlab", label(wall.base_slab.as_deref().unwrap_or_default())),
        ("BaseOffset", number(wall.base_offset)),
        ("Height", number(layout.height)),
        ("BaseLevel", number(level)),
    ])
}

fn attach_targets(wall: &Wall) -> Vec<(String, &'static str)> {
    let top = match &wall.top {
        TopConstraint::Roof { roof: target, .. } | TopConstraint::Slab { slab: target, .. } | TopConstraint::Ceiling { ceiling: target, .. } => Some((target.clone(), "TopAttach")),
        _ => None,
    };
    top.into_iter().chain(wall.base_slab.iter().map(|slab| (slab.clone(), "BaseAttach"))).collect()
}

fn axis_curve(x: &mut Export<'_>, axis: &Axis) -> u64 {
    let (start, end, bulge) = match axis {
        Axis::Line { start, end } => (start, end, 0.0),
        Axis::Arc { start, end, bulge } => (start, end, *bulge),
    };
    x.ifc.edge_curve([start.x, start.y], [end.x, end.y], bulge)
}

fn frame_depth(model: &ModelSnapshot, kind: &OpeningKind) -> f64 {
    match kind {
        OpeningKind::Window { window_type } => model.window_types.get(window_type).map(|row| row.frame_depth),
        OpeningKind::Door { door_type } => model.door_types.get(door_type).map(|row| row.frame_depth),
        OpeningKind::Void { .. } => None,
    }
    .unwrap_or(0.0)
    .max(0.01)
}

fn footprint(layout: &WallLayout, segment: &BulgeSeg) -> Option<Vec<([f64; 2], f64)>> {
    if layout.footprint.len() >= 3 {
        return Some(layout.footprint.iter().map(|vertex| ([vertex.point.x, vertex.point.y], vertex.bulge)).collect());
    }
    band_loop(segment, layout.offset_left, layout.offset_right, None, None).map(|ring| ring.iter().map(|(corner, bulge)| ([corner.x, corner.y], *bulge)).collect())
}

fn wall(x: &mut Export<'_>, id: &str, wall: &Wall, hosts: &mut BTreeMap<String, Host>) {
    let inferred = x.inferred;
    let (Some(storey), Some(layout)) = (x.storeys.get(&wall.storey).copied(), inferred.wall_layout.get(id)) else {
        x.skip("wall", id, "its storey or layout is missing");
        return;
    };
    if layout.thickness <= 0.0 || layout.height <= 0.0 || layout.length <= 0.0 {
        x.skip("wall", id, "it has no thickness, height or length");
        return;
    }
    let (left, right) = (layout.offset_left, layout.offset_right);
    let (segment, z) = (segment_of(&wall.axis), layout.base_z - storey.elevation);
    let tessellated = !layout.top_profile.is_empty() || !layout.base_profile.is_empty();
    let standard = matches!(wall.axis, Axis::Line { .. }) && layout.joins.is_empty() && !tessellated;
    let (placement, item, kind, curve) = match (&wall.axis, standard, tessellated) {
        (Axis::Line { start, end }, true, _) => {
            let angle = (end.y - start.y).atan2(end.x - start.x);
            let axis = x.ifc.axis3([start.x, start.y, z], None, Some([angle.cos(), angle.sin(), 0.0]));
            let placement = x.ifc.place(Some(storey.placement), axis);
            let profile = x.ifc.rectangle([layout.length / 2.0, (left - right) / 2.0], layout.length, layout.thickness);
            let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], layout.height);
            let (from, to) = (x.ifc.point2([0.0, 0.0]), x.ifc.point2([layout.length, 0.0]));
            (placement, solid, "SweptSolid", x.ifc.add("IFCPOLYLINE", vec![refs(&[from, to])]))
        }
        (axis, _, true) => {
            let Some(item) = mesh_item(&mut x.ifc, &envelope(wall, layout, storey.elevation)) else {
                x.skip("wall", id, "its attached body is empty");
                return;
            };
            let origin = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
            let placement = x.ifc.place(Some(storey.placement), origin);
            (placement, item, body_kind(&x.ifc), axis_curve(x, axis))
        }
        (axis, _, false) => {
            let Some(ring) = footprint(&layout, &segment) else {
                x.skip("wall", id, "its footprint collapses");
                return;
            };
            let outline = x.ifc.loop_curve(&ring);
            let profile = x.ifc.curve_profile(outline, &[]);
            let origin = x.ifc.axis3([0.0, 0.0, z], None, None);
            let placement = x.ifc.place(Some(storey.placement), origin);
            let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], layout.height);
            (placement, solid, "SweptSolid", axis_curve(x, axis))
        }
    };
    let body = x.ifc.shape(x.ifc.body, "Body", kind, &[item]);
    let trace = x.ifc.shape(x.ifc.axis, "Axis", "Curve2D", &[curve]);
    let shape = x.ifc.definition(&[trace, body]);
    let tail = x.by(vec![], vec![en("STANDARD")]);
    let element = x.product(if standard { "IFCWALLSTANDARDCASE" } else { "IFCWALL" }, id, &wall.name, placement, Some(shape), tail);
    x.contain(&wall.storey, id, element);
    if let Some(object) = x.links.types.get(&("wall", wall.wall_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    if let Some(set) = x.links.layer_sets.get(&("wall", wall.wall_type.clone())).copied() {
        let usage = x.ifc.add("IFCMATERIALLAYERSETUSAGE", x.by(vec![rf(set), en("AXIS2"), en("NEGATIVE"), real(left)], vec![rf(set), en("AXIS2"), en("NEGATIVE"), real(left), unset()]));
        x.links.materials.entry(usage).or_default().push(element);
    }
    x.quantify(element, "Qto_WallBaseQuantities", id, |row| {
        vec![
            Quantity::Length("Length", row.length),
            Quantity::Length("Width", row.width),
            Quantity::Length("Height", row.height),
            Quantity::Area("GrossFootprintArea", row.gross_area),
            Quantity::Area("GrossSideArea", row.gross_side_area),
            Quantity::Area("NetSideArea", row.net_side_area),
            Quantity::Volume("GrossVolume", row.gross_volume),
            Quantity::Volume("NetVolume", row.net_volume),
        ]
    });
    if let Some(rows) = attach_rows(wall, layout, z) {
        describe(x, &format!("{id}:attach"), element, ATTACH_SET, rows);
    }
    x.links.connections.extend(attach_targets(wall).into_iter().map(|(target, role)| (id.to_string(), target, role)));
    hosts.insert(id.to_string(), Host { placement, segment, standard, base_z: if tessellated { 0.0 } else { z }, centre_y: (left - right) / 2.0, thickness: layout.thickness, elevation: storey.elevation, lifted: tessellated });
}

fn ring(x: &mut Export<'_>, width: f64, height: f64, frame: f64) -> u64 {
    let corners = |half: f64, from: f64, to: f64| vec![([-half, from], 0.0), ([half, from], 0.0), ([half, to], 0.0), ([-half, to], 0.0)];
    let outer = x.ifc.loop_curve(&corners(width / 2.0, 0.0, height));
    let inner = x.ifc.loop_curve(&corners((width / 2.0 - frame).max(0.0), frame, (height - frame).max(frame)));
    x.ifc.curve_profile(outer, &[inner])
}

fn filling(x: &mut Export<'_>, kind: &OpeningKind, width: f64, height: f64) -> Option<u64> {
    let model = x.model;
    let (frame, depth, leaf) = match kind {
        OpeningKind::Window { window_type } => model.window_types.get(window_type).map(|row| (row.frame_width, row.frame_depth, 0.024))?,
        OpeningKind::Door { door_type } => model.door_types.get(door_type).map(|row| (row.frame_width, row.frame_depth, 0.04))?,
        OpeningKind::Void { .. } => return None,
    };
    let (frame, depth) = (frame.clamp(0.0, width.min(height) / 2.0 - 1e-6), depth.max(0.01));
    let frame_profile = ring(x, width, height, frame);
    let frame_position = x.ifc.axis3([0.0, 0.0, -depth / 2.0], None, None);
    let frame_solid = x.ifc.extrusion(frame_profile, frame_position, [0.0, 0.0, 1.0], depth);
    let pane_profile = x.ifc.rectangle([0.0, height / 2.0], (width - 2.0 * frame).max(1e-3), (height - 2.0 * frame).max(1e-3));
    let pane_position = x.ifc.axis3([0.0, 0.0, -leaf / 2.0], None, None);
    let pane_solid = x.ifc.extrusion(pane_profile, pane_position, [0.0, 0.0, 1.0], leaf);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[frame_solid, pane_solid]);
    Some(x.ifc.definition(&[body]))
}

/// 🪟️ The attributes of an `IfcWindow` or `IfcDoor` after its tag: the overall height and width, and in IFC4 the predefined type and the partitioning (window) or operation (door) of its type.
pub fn filling_tail(x: &Export<'_>, kind: &OpeningKind, height: f64, width: f64) -> Vec<V> {
    let mut tail = vec![real(height), real(width)];
    if x.schema() == Schema::Ifc4 {
        let model = x.model;
        match kind {
            OpeningKind::Window { window_type } => tail.extend([en("WINDOW"), en(model.window_types.get(window_type).map_or("NOTDEFINED", |row| data::window_partitioning(row.panes))), unset()]),
            OpeningKind::Door { door_type } => tail.extend([en("DOOR"), en(model.door_types.get(door_type).map_or("NOTDEFINED", data::door_operation)), unset()]),
            OpeningKind::Void { .. } => {}
        }
    }
    tail
}

fn opening(x: &mut Export<'_>, id: &str, row: &Opening, hosts: &BTreeMap<String, Host>) {
    let model = x.model;
    let size = resolve_size(model, row);
    let (Some(host), Some(wall_row)) = (hosts.get(&row.host), model.walls.get(&row.host)) else {
        x.skip("opening", id, "its host is not a written wall");
        return;
    };
    if !size.type_found || size.width <= 0.0 || size.height <= 0.0 {
        x.skip("opening", id, "its type is missing or its size is not positive");
        return;
    }
    let inferred = x.inferred;
    let frame = inferred.opening_frames.get(id);
    let location_axis = if host.standard {
        x.ifc.axis3([row.offset, host.centre_y, size.sill], None, None)
    } else {
        let (centre, tangent) = frame_at(&host.segment, row.offset);
        let normal = [-tangent[1], tangent[0]];
        let lift = frame.filter(|_| host.lifted).map_or(host.base_z + size.sill, |frame| frame.local.origin.z - host.elevation);
        x.ifc.axis3([centre[0] + normal[0] * host.centre_y, centre[1] + normal[1] * host.centre_y, lift], None, Some([tangent[0], tangent[1], 0.0]))
    };
    let placement = x.ifc.place(Some(host.placement), location_axis);
    let profile = x.ifc.rectangle([0.0, 0.0], size.width, host.thickness);
    let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], size.height);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    let shape = x.ifc.definition(&[body]);
    let tail = x.by(vec![], vec![en("OPENING")]);
    let void = x.product("IFCOPENINGELEMENT", id, &row.name, placement, Some(shape), tail);
    let wall = x.links.elements[&row.host];
    x.ifc.rooted("IFCRELVOIDSELEMENT", id, "", "", vec![rf(wall), rf(void)]);
    let reveal: Vec<(&str, V)> = row.reveal_depth.map(|depth| ("RevealDepth", number(depth))).into_iter().chain(row.reveal_material.as_deref().map(|material| ("RevealMaterial", label(material)))).collect();
    if !reveal.is_empty() {
        describe(x, &format!("{id}:reveal"), void, REVEAL_SET, reveal);
    }
    let (entity, type_key) = match &row.kind {
        OpeningKind::Window { window_type } => ("IFCWINDOW", ("window", window_type.clone())),
        OpeningKind::Door { door_type } => ("IFCDOOR", ("door", door_type.clone())),
        OpeningKind::Void { .. } => {
            x.links.elements.insert(id.to_string(), void);
            if host.lifted {
                x.links.authoring.push((void, vec![("Sill", data::number(size.sill))]));
            }
            return;
        }
    };
    let facing = if row.flip_facing { [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]] } else { [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0]] };
    let toward_front = frame.and_then(|frame| frame.setback.map(|setback| frame.face_front - setback - frame_depth(model, &row.kind) / 2.0 - (frame.face_front - frame.face_back) / 2.0)).unwrap_or(0.0);
    let fill_axis = x.ifc.axis3([0.0, if row.flip_facing { -toward_front } else { toward_front }, 0.0], Some(facing[0]), Some(facing[1]));
    let fill_place = x.ifc.place(Some(placement), fill_axis);
    let shape = filling(x, &row.kind, size.width, size.height);
    let tail = filling_tail(x, &row.kind, size.height, size.width);
    let fill = x.product(entity, id, &row.name, fill_place, shape, tail);
    x.ifc.rooted("IFCRELFILLSELEMENT", id, "", "", vec![rf(void), rf(fill)]);
    x.contain(&wall_row.storey, id, fill);
    if let Some(object) = x.links.types.get(&type_key).copied() {
        x.links.typed.entry(object).or_default().push(fill);
    }
    if row.flip_hand {
        x.links.authoring.push((fill, vec![("FlipHand", data::label("true"))]));
    }
    if host.lifted {
        x.links.authoring.push((fill, vec![("Sill", data::number(size.sill))]));
    }
    let name = if entity == "IFCWINDOW" { "Qto_WindowBaseQuantities" } else { "Qto_DoorBaseQuantities" };
    x.quantify(fill, name, id, |row| vec![Quantity::Length("Height", row.height), Quantity::Length("Width", row.width), Quantity::Area("Area", row.gross_area)]);
}

/// 🔗️ Writes the `IfcRelConnectsElements` of every wall to the roof, slab or ceiling its top or base follows, once all products exist.
pub fn connect(x: &mut Export<'_>) {
    for (wall, target, role) in std::mem::take(&mut x.links.connections) {
        let (Some(from), Some(to)) = (x.links.elements.get(&wall).copied(), x.links.elements.get(&target).copied()) else {
            x.skip("attach", &wall, "its target is not part of the export");
            continue;
        };
        x.ifc.rooted("IFCRELCONNECTSELEMENTS", &format!("{wall}:{target}:{role}"), "", role, vec![unset(), rf(from), rf(to)]);
    }
}

/// 🧱️ Writes every wall, then every opening with its void and filling.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    let mut hosts = BTreeMap::new();
    for (id, row) in &model.walls {
        wall(x, id, row, &mut hosts);
    }
    for (id, row) in &model.openings {
        if model.curtain_walls.contains_key(&row.host) {
            continue;
        }
        opening(x, id, row, &hosts);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
