//! 🏗️ Columns and beams: `IfcColumn` (profile extruded vertically from the resolved base to the resolved top) and `IfcBeam` (profile extruded along the axis, its highest point on the
//! storey top plus `top_offset`), with rectangle, circle, I-shape or custom profile definitions.

use super::frames::{ccw, profile_top, vertical};
use super::writer::V;
use super::{Export, Quantity};
use crate::Profile;

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
    let (Some(storey), Some(kind), Some((base, top))) = (x.storeys.get(&row.storey).copied(), model.column_types.get(&row.column_type), vertical(&x.inferred.storey_levels, &row.storey, row.base_offset, &row.top)) else {
        x.skip("column", id, "its storey or type is missing");
        return;
    };
    let height = top - base;
    let Some(section) = profile(x, &kind.profile).filter(|_| height > 1e-9) else {
        x.skip("column", id, "its profile or height is not positive");
        return;
    };
    let direction = (row.rotation.abs() > 1e-12).then(|| [row.rotation.cos(), row.rotation.sin(), 0.0]);
    let axis = x.ifc.axis3([row.position.x, row.position.y, base - storey.elevation], None, direction);
    let placement = x.ifc.place(Some(storey.placement), axis);
    let solid = x.ifc.extrusion(section, x.ifc.origin, [0.0, 0.0, 1.0], height);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    let shape = x.ifc.definition(&[body]);
    let element = x.product("IFCCOLUMN", id, &row.name, placement, Some(shape), Vec::<V>::new());
    x.contain(&row.storey, id, element);
    if let Some(object) = x.links.types.get(&("column", row.column_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    associate(x, &kind.material, element);
    x.quantify(element, "Qto_ColumnBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Area("CrossSectionArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume)]);
}

fn beam(x: &mut Export<'_>, id: &str, row: &crate::Beam) {
    let model = x.model;
    let (Some(storey), Some(kind), Some(level)) = (x.storeys.get(&row.storey).copied(), model.beam_types.get(&row.beam_type), x.inferred.storey_levels.get(&row.storey)) else {
        x.skip("beam", id, "its storey or type is missing");
        return;
    };
    let (dx, dy) = (row.end.x - row.start.x, row.end.y - row.start.y);
    let length = dx.hypot(dy);
    let Some(section) = profile(x, &kind.profile).filter(|_| length > 1e-9) else {
        x.skip("beam", id, "its profile or length is not positive");
        return;
    };
    let top = level.top_elevation + row.top_offset;
    let z = top - profile_top(&kind.profile) - storey.elevation;
    let axis = x.ifc.axis3([row.start.x, row.start.y, z], Some([dx / length, dy / length, 0.0]), Some([-dy / length, dx / length, 0.0]));
    let placement = x.ifc.place(Some(storey.placement), axis);
    let solid = x.ifc.extrusion(section, x.ifc.origin, [0.0, 0.0, 1.0], length);
    let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
    let shape = x.ifc.definition(&[body]);
    let element = x.product("IFCBEAM", id, &row.name, placement, Some(shape), Vec::<V>::new());
    x.contain(&row.storey, id, element);
    if let Some(object) = x.links.types.get(&("beam", row.beam_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    associate(x, &kind.material, element);
    x.quantify(element, "Qto_BeamBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Area("CrossSectionArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume)]);
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
mod tests;
