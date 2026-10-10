//! 🧷️ Wall sweeps. A sweep is one `IfcMember` (`ObjectType` `WallSweep`) per run of its path along the face of its host wall: a straight run on a straight face with a flat base is the `IfcExtrudedAreaSolid` of its profile,
//! every other run (an arc face, a base that follows a slab) is the faceted brep of the triangles of the inferred sweep solid that lie in the run. All runs of one sweep carry the same `Tag` (the sweep id) and the
//! `Semio_WallSweep` set with the authored record (host, side, height, inset, material, profile as JSON), so an import merges them back into one sweep; the base quantities of a run are its share of the quantity row.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcmember.htm>

use super::brep::{brep_definition, mesh_where};
use super::data::{label, number};
use super::walls::describe;
use super::writer::{en, int, rf, text, typed, V};
use super::{Export, Quantity, StoreyRef};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps::{runs, section_area, section_of};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::walls::cuts_of;
use crate::standards::v1::subsets::any::schema::authored::plan::segment_of;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::{fraction, WallLayout};
use crate::{Axis, Wall, WallSide, WallSweep};
use semio_framework_geometry::Point;

/// 🧷️ The property set that records the authored wall sweep on each of its runs.
pub const SWEEP_SET: &str = "Semio_WallSweep";

/// 🏷️ The quantity set of one run, the `Qto_MemberBaseQuantities` of IFC4 under the same name.
pub const QUANTITY_SET: &str = "Qto_MemberBaseQuantities";

fn extruded(x: &mut Export<'_>, row: &WallSweep, wall: &Wall, layout: &WallLayout, storey: StoreyRef, run: (f64, f64)) -> Option<u64> {
    let Axis::Line { start, end } = &wall.axis else { return None };
    let length = (end.x - start.x).hypot(end.y - start.y);
    if length <= 0.0 {
        return None;
    }
    let tangent = [(end.x - start.x) / length, (end.y - start.y) / length];
    let normal = [-tangent[1], tangent[0]];
    let (side, offset, anchor) = match row.side {
        WallSide::Left => (1.0, layout.offset_left, run.0),
        WallSide::Right => (-1.0, layout.offset_right, run.1),
    };
    let origin = [start.x + tangent[0] * anchor + normal[0] * side * offset, start.y + tangent[1] * anchor + normal[1] * side * offset, layout.base_z + row.height - storey.elevation];
    let position = x.ifc.axis3(origin, Some([tangent[0] * side, tangent[1] * side, 0.0]), Some([normal[0] * side, normal[1] * side, 0.0]));
    let ring: Vec<([f64; 2], f64)> = section_of(row).iter().map(|corner| ([corner.x - row.inset, corner.y], 0.0)).collect();
    let outline = x.ifc.loop_curve(&ring);
    let profile = x.ifc.curve_profile(outline, &[]);
    let solid = x.ifc.extrusion(profile, position, [0.0, 0.0, 1.0], run.1 - run.0);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    Some(x.ifc.definition(&[body]))
}

fn sweep(x: &mut Export<'_>, id: &str, row: &WallSweep) {
    let (model, inferred) = (x.model, x.inferred);
    let (Some(wall), Some(layout)) = (model.walls.get(&row.host), inferred.wall_layout.get(&row.host)) else {
        x.skip("wall sweep", id, "its host wall has no layout");
        return;
    };
    let Some(storey) = x.storeys.get(&wall.storey).copied() else {
        x.skip("wall sweep", id, "its storey is not written");
        return;
    };
    let frames = model.openings.iter().filter(|(_, opening)| opening.host == row.host).filter_map(|(opening, _)| inferred.opening_frames.get(opening));
    let paths = runs(row, wall, layout, &cuts_of(frames));
    let offset = match row.side {
        WallSide::Left => layout.offset_left,
        WallSide::Right => -layout.offset_right,
    };
    let axis = segment_of(&wall.axis);
    let Some(face) = axis.offset(offset) else {
        x.skip("wall sweep", id, "its face collapses");
        return;
    };
    if paths.is_empty() || layout.length <= 0.0 {
        x.skip("wall sweep", id, "it has no run");
        return;
    }
    let scale = face.length() / layout.length;
    let section = inferred.quantities.elements.get(id).map_or_else(|| section_area(row), |quantity| quantity.gross_area);
    let straight = matches!(wall.axis, Axis::Line { .. }) && layout.base_profile.is_empty();
    for (index, run) in paths.iter().copied().enumerate() {
        let key = if index == 0 { id.to_string() } else { format!("{id}:run{index}") };
        let shape = if straight {
            extruded(x, row, wall, layout, storey, run)
        } else {
            inferred.element_solids.get(id).and_then(|solid| {
                let inside = |centroid: [f64; 3]| {
                    let at = fraction(&axis, Point::new(centroid[0], centroid[1])) * layout.length;
                    at >= run.0 - 1e-6 && at <= run.1 + 1e-6
                };
                brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, centroid| inside(centroid)))
            })
        };
        let Some(shape) = shape else {
            x.skip("wall sweep", id, "a run has no body");
            continue;
        };
        let origin = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let placement = x.ifc.place(Some(storey.placement), origin);
        let mut tail = vec![text("WallSweep"), rf(placement), rf(shape), Export::tag(id)];
        tail.extend(x.by(vec![], vec![en("USERDEFINED")]));
        let member = x.ifc.rooted("IFCMEMBER", &key, &Export::label(&row.name, id), "", tail);
        x.contain(&wall.storey, &key, member);
        if index > 0 {
            x.phase(id, member);
        }
        if let Some(definition) = x.links.material_defs.get(&row.material).copied() {
            x.links.materials.entry(definition).or_default().push(member);
        }
        let length = (run.1 - run.0) * scale;
        x.links.quantities.push((member, QUANTITY_SET, vec![Quantity::Length("Length", length), Quantity::Area("CrossSectionArea", section), Quantity::Volume("GrossVolume", section * length)]));
        let rows: Vec<(&str, V)> = vec![
            ("SweepId", label(id)),
            ("Run", typed("IFCINTEGER", int(index as i64))),
            ("Host", label(&row.host)),
            ("Side", label(&format!("{:?}", row.side))),
            ("Height", number(row.height)),
            ("Inset", number(row.inset)),
            ("Material", label(&row.material)),
            ("Profile", label(&semio_framework_pack_json::to_json_string(&row.profile))),
        ];
        describe(x, &format!("{key}:sweep"), member, SWEEP_SET, rows);
    }
}

/// 🧷️ Writes every wall sweep as the members of its runs.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.wall_sweeps {
        sweep(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
