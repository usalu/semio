//! 🏗️ Columns and beams: `IfcColumn` (profile extruded vertically from the resolved base to the resolved top; a tilted column extrudes the stretched horizontal section along its leaning axis) and
//! `IfcBeam` (profile extruded along the axis, its highest point on the storey top plus `top_offset`), with rectangle, circle, I-shape or custom profile definitions. A beam that is an arc, rises along
//! its axis or is cut back by the columns it joins is the faceted brep of its inferred solid with an `Axis` curve (an `IfcTrimmedCurve` for an arc); IFC has no slot for the authored lean, offsets
//! or axis, so such a column and beam carry their authored record as the `Column` and `Beam` rows of their `Semio_Authoring` set (canonical JSON) and an import restores them exactly.

use super::brep::{body_kind, brep_definition, mesh_item, mesh_where};
use super::data::label;
use super::frames::{ccw, profile_top};
use super::writer::{en, V};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::columns::footprint;
use crate::{Axis, Profile, Vertex};

/// 🏷️ The `Semio_Authoring` row that holds the authored column record of a tilted column.
pub const COLUMN_ROW: &str = "Column";
/// 🏷️ The `Semio_Authoring` row that holds the authored beam record of an arc, inclined or joined beam.
pub const BEAM_ROW: &str = "Beam";

/// ▭️ The `IfcProfileDef` of a profile; `None` when its dimensions are not positive.
pub fn profile(x: &mut Export<'_>, profile: &Profile) -> Option<u64> {
    match profile {
        Profile::Rectangle { width, depth } if *width > 0.0 && *depth > 0.0 => Some(x.ifc.rectangle([0.0, 0.0], *width, *depth)),
        Profile::Circle { diameter } if *diameter > 0.0 => Some(x.ifc.circle(*diameter)),
        Profile::IShape { width, depth, web, flange } if *width > 0.0 && *depth > 0.0 && *web > 0.0 && *flange > 0.0 && web < width && 2.0 * flange < *depth => Some(x.ifc.i_shape(*width, *depth, *web, *flange)),
        Profile::Custom { outline } if outline.len() >= 3 => {
            let curve = x.ifc.loop_curve(&ccw(outline));
            Some(x.ifc.curve_profile(curve, &[]))
        }
        _ => None,
    }
}

fn associate(x: &mut Export<'_>, material: &str, element: u64) {
    if let Some(definition) = x.links.material_defs.get(material).copied() {
        x.links.materials.entry(definition).or_default().push(element);
    }
}

fn column(x: &mut Export<'_>, id: &str, row: &crate::Column) {
    let model = x.model;
    let (Some(storey), Some(kind), Some(bounds)) = (x.storeys.get(&row.storey).copied(), model.column_types.get(&row.column_type), x.solid(id).map(|solid| solid.bounds)) else {
        x.skip("column", id, "its storey, type or inferred solid is missing");
        return;
    };
    let (base, top) = (bounds.min.z, bounds.max.z);
    let height = top - base;
    let leaning = row.tilt.filter(|tilt| tilt.angle.abs() > 1e-12 && tilt.angle.abs() < std::f64::consts::FRAC_PI_2);
    let family = matches!(kind.profile, Profile::Family { .. });
    let Some(section) = (if leaning.is_none() && !family { profile(x, &kind.profile) } else { Some(0) }).filter(|_| height > 1e-9) else {
        x.skip("column", id, "its profile or height is not positive");
        return;
    };
    let direction = (row.rotation.abs() > 1e-12 && leaning.is_none()).then(|| [row.rotation.cos(), row.rotation.sin(), 0.0]);
    let axis = if family { x.ifc.axis3([0.0, 0.0, 0.0], None, None) } else { x.ifc.axis3([row.position.x, row.position.y, base - storey.elevation], None, direction) };
    let placement = x.ifc.place(Some(storey.placement), axis);
    let shape = if family {
        let Some(shape) = x.solid(id).and_then(|solid| brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, _| true))) else {
            x.skip("column", id, "its family-profile solid is empty");
            return;
        };
        shape
    } else {
    let solid = match leaning {
        None => x.ifc.extrusion(section, x.ifc.origin, [0.0, 0.0, 1.0], height),
        Some(tilt) => {
            let ring: Vec<Vertex> = footprint(row, &kind.profile, base, base).iter().map(|p| Vertex { point: crate::Point2 { x: p.x - row.position.x, y: p.y - row.position.y }, bulge: 0.0 }).collect();
            if ring.len() < 3 {
                x.skip("column", id, "its tilted section is not a polygon");
                return;
            }
            let curve = x.ifc.loop_curve(&ccw(&ring));
            let slanted = x.ifc.curve_profile(curve, &[]);
            let slope = [tilt.angle.sin() * tilt.direction.cos(), tilt.angle.sin() * tilt.direction.sin(), tilt.angle.cos()];
            x.ifc.extrusion(slanted, x.ifc.origin, slope, height / tilt.angle.cos())
        }
    };
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    x.ifc.definition(&[body])
    };
    let tail = x.by(Vec::<V>::new(), vec![en("COLUMN")]);
    let element = x.product("IFCCOLUMN", id, &row.name, placement, Some(shape), tail);
    x.contain(&row.storey, id, element);
    if leaning.is_some() || family {
        x.links.authoring.push((element, vec![(COLUMN_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    }
    if let Some(object) = x.links.types.get(&("column", row.column_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    associate(x, &kind.material, element);
    x.quantify(element, "Qto_ColumnBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Area("CrossSectionArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume), Quantity::Volume("NetVolume", row.net_volume)]);
}

fn beam(x: &mut Export<'_>, id: &str, row: &crate::Beam) {
    let model = x.model;
    let (Some(storey), Some(kind), Some(bounds)) = (x.storeys.get(&row.storey).copied(), model.beam_types.get(&row.beam_type), x.solid(id).map(|solid| solid.bounds)) else {
        x.skip("beam", id, "its storey, type or inferred solid is missing");
        return;
    };
    let (start, end, bulge) = match &row.axis {
        Axis::Line { start, end } => (start, end, 0.0),
        Axis::Arc { start, end, bulge } => (start, end, *bulge),
    };
    let (dx, dy) = (end.x - start.x, end.y - start.y);
    let length = dx.hypot(dy);
    let level_line = bulge == 0.0 && row.end_top_offset.is_none_or(|end_offset| (end_offset - row.top_offset).abs() < 1e-9);
    let uncut = x.solid(id).zip(x.measure(id)).is_none_or(|(solid, quantity)| (solid.volume - quantity.gross_volume).abs() <= 1e-6 * quantity.gross_volume.abs().max(1.0));
    let top = bounds.max.z;
    let family = matches!(kind.profile, Profile::Family { .. });
    let (placement, shape, joined) = if level_line && uncut && !family {
        let Some(section) = profile(x, &kind.profile).filter(|_| length > 1e-9) else {
            x.skip("beam", id, "its profile or length is not positive");
            return;
        };
        let z = top - profile_top(&kind.profile) - storey.elevation;
        let axis = x.ifc.axis3([start.x, start.y, z], Some([dx / length, dy / length, 0.0]), Some([-dy / length, dx / length, 0.0]));
        let placement = x.ifc.place(Some(storey.placement), axis);
        let solid = x.ifc.extrusion(section, x.ifc.origin, [0.0, 0.0, 1.0], length);
        let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
        (placement, x.ifc.definition(&[body]), false)
    } else {
        let Some(solid) = x.solid(id) else {
            x.skip("beam", id, "its solid is not inferred");
            return;
        };
        let Some(brep) = mesh_item(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, _| true)) else {
            x.skip("beam", id, "its solid is empty");
            return;
        };
        let trace = x.ifc.edge_curve([start.x, start.y], [end.x, end.y], bulge);
        let trace = x.ifc.shape(x.ifc.axis, "Axis", "Curve2D", &[trace]);
        let body = x.ifc.shape(x.ifc.body, "Body", body_kind(&x.ifc), &[brep]);
        let origin = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let placement = x.ifc.place(Some(storey.placement), origin);
        (placement, x.ifc.definition(&[trace, body]), true)
    };
    let tail = x.by(Vec::<V>::new(), vec![en("BEAM")]);
    let element = x.product("IFCBEAM", id, &row.name, placement, Some(shape), tail);
    x.contain(&row.storey, id, element);
    if joined {
        x.links.authoring.push((element, vec![(BEAM_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    }
    if let Some(object) = x.links.types.get(&("beam", row.beam_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    associate(x, &kind.material, element);
    x.quantify(element, "Qto_BeamBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Area("CrossSectionArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume), Quantity::Volume("NetVolume", row.net_volume)]);
}

/// 🏗️ Writes every column and beam.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.columns {
        column(x, id, row);
    }
    for (id, row) in &model.beams {
        beam(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
