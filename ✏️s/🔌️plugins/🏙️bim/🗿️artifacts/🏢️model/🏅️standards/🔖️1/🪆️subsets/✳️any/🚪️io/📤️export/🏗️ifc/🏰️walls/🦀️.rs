//! 🧱️ Walls and the openings they host. A straight wall without joins is an `IfcWallStandardCase` (rectangle swept along its axis); a curved or joined wall is an `IfcWall`
//! whose arbitrary closed profile is the join-trimmed footprint of `wall-layout`, so the exported volume equals the inferred one. Every wall carries an `Axis` representation and an
//! `IfcMaterialLayerSetUsage`. Openings become `IfcOpeningElement` + `IfcRelVoidsElement`; windows and doors fill them (`IfcRelFillsElement`) with a frame-and-pane body.

use super::frames::frame_at;
use super::writer::{en, real, refs, rf};
use super::{data, Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::opening_frames::resolve_size;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::{segment_of, WallLayout};
use crate::{Axis, Opening, OpeningKind, Wall};
use semio_framework_geometry::bulge::{band_loop, BulgeSeg};
use std::collections::BTreeMap;

struct Host {
    placement: u64,
    segment: BulgeSeg,
    standard: bool,
    base_z: f64,
    centre_y: f64,
    thickness: f64,
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
    let standard = matches!(wall.axis, Axis::Line { .. }) && layout.joins.is_empty();
    let (placement, solid, curve) = match (&wall.axis, standard) {
        (Axis::Line { start, end }, true) => {
            let angle = (end.y - start.y).atan2(end.x - start.x);
            let axis = x.ifc.axis3([start.x, start.y, z], None, Some([angle.cos(), angle.sin(), 0.0]));
            let placement = x.ifc.place(Some(storey.placement), axis);
            let profile = x.ifc.rectangle([layout.length / 2.0, (left - right) / 2.0], layout.length, layout.thickness);
            let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], layout.height);
            let (from, to) = (x.ifc.point2([0.0, 0.0]), x.ifc.point2([layout.length, 0.0]));
            (placement, solid, x.ifc.add("IFCPOLYLINE", vec![refs(&[from, to])]))
        }
        (axis, _) => {
            let Some(ring) = footprint(&layout, &segment) else {
                x.skip("wall", id, "its footprint collapses");
                return;
            };
            let outline = x.ifc.loop_curve(&ring);
            let profile = x.ifc.curve_profile(outline, &[]);
            let origin = x.ifc.axis3([0.0, 0.0, z], None, None);
            let placement = x.ifc.place(Some(storey.placement), origin);
            let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], layout.height);
            let (start, end, bulge) = match axis {
                Axis::Line { start, end } => (start, end, 0.0),
                Axis::Arc { start, end, bulge } => (start, end, *bulge),
            };
            (placement, solid, x.ifc.edge_curve([start.x, start.y], [end.x, end.y], bulge))
        }
    };
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    let trace = x.ifc.shape(x.ifc.axis, "Axis", "Curve2D", &[curve]);
    let shape = x.ifc.definition(&[trace, body]);
    let element = x.product(if standard { "IFCWALLSTANDARDCASE" } else { "IFCWALL" }, id, &wall.name, placement, Some(shape), vec![]);
    x.contain(&wall.storey, id, element);
    if let Some(object) = x.links.types.get(&("wall", wall.wall_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    if let Some(set) = x.links.layer_sets.get(&("wall", wall.wall_type.clone())).copied() {
        let usage = x.ifc.add("IFCMATERIALLAYERSETUSAGE", vec![rf(set), en("AXIS2"), en("NEGATIVE"), real(left)]);
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
    hosts.insert(id.to_string(), Host { placement, segment, standard, base_z: z, centre_y: (left - right) / 2.0, thickness: layout.thickness });
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
    let location_axis = if host.standard {
        x.ifc.axis3([row.offset, host.centre_y, size.sill], None, None)
    } else {
        let (centre, tangent) = frame_at(&host.segment, row.offset);
        let normal = [-tangent[1], tangent[0]];
        x.ifc.axis3([centre[0] + normal[0] * host.centre_y, centre[1] + normal[1] * host.centre_y, host.base_z + size.sill], None, Some([tangent[0], tangent[1], 0.0]))
    };
    let placement = x.ifc.place(Some(host.placement), location_axis);
    let profile = x.ifc.rectangle([0.0, 0.0], size.width, host.thickness);
    let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], size.height);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    let shape = x.ifc.definition(&[body]);
    let void = x.product("IFCOPENINGELEMENT", id, &row.name, placement, Some(shape), vec![]);
    let wall = x.links.elements[&row.host];
    x.ifc.rooted("IFCRELVOIDSELEMENT", id, "", "", vec![rf(wall), rf(void)]);
    let (entity, type_key) = match &row.kind {
        OpeningKind::Window { window_type } => ("IFCWINDOW", ("window", window_type.clone())),
        OpeningKind::Door { door_type } => ("IFCDOOR", ("door", door_type.clone())),
        OpeningKind::Void { .. } => {
            x.links.elements.insert(id.to_string(), void);
            return;
        }
    };
    let facing = if row.flip_facing { [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]] } else { [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0]] };
    let fill_axis = x.ifc.axis3([0.0, 0.0, 0.0], Some(facing[0]), Some(facing[1]));
    let fill_place = x.ifc.place(Some(placement), fill_axis);
    let shape = filling(x, &row.kind, size.width, size.height);
    let fill = x.product(entity, id, &row.name, fill_place, shape, vec![real(size.height), real(size.width)]);
    x.ifc.rooted("IFCRELFILLSELEMENT", id, "", "", vec![rf(void), rf(fill)]);
    x.contain(&wall_row.storey, id, fill);
    if let Some(object) = x.links.types.get(&type_key).copied() {
        x.links.typed.entry(object).or_default().push(fill);
    }
    if row.flip_hand {
        x.links.authoring.push((fill, vec![("FlipHand", data::label("true"))]));
    }
    let name = if entity == "IFCWINDOW" { "Qto_WindowBaseQuantities" } else { "Qto_DoorBaseQuantities" };
    x.quantify(fill, name, id, |row| vec![Quantity::Length("Height", row.height), Quantity::Length("Width", row.width), Quantity::Area("Area", row.gross_area)]);
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
