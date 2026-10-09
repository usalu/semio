//! 🔲️ Ceilings. A level ceiling is an `IfcCovering` (`CEILING`) whose boundary, with its holes, is extruded by the sum of its layers upward from the underside, which hangs `offset` plus the thickness below
//! the top of its storey; a sloped ceiling is the faceted brep of its inferred solid and carries its authored record as the `Ceiling` row of its `Semio_Authoring` set (IFC 2x3 has no slot for the tilt, and a brep
//! has no boundary to read back). The ceiling type is an `IfcCoveringType` (`CEILING`) with the same layer set, and the base quantities are the `Qto_CoveringBaseQuantities` of the take-off.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifccovering.htm>

use super::brep::{brep_definition, mesh_where};
use super::data::label;
use super::frames::{ccw, cw};
use super::writer::{en, real, rf};
use super::{Export, Quantity};
use crate::Ceiling;

/// 🏷️ The `Semio_Authoring` row that holds the authored record of a sloped ceiling.
pub const RECORD_ROW: &str = "Ceiling";

fn covering(x: &mut Export<'_>, id: &str, row: &Ceiling) {
    let model = x.model;
    let (Some(storey), Some(kind), Some(level)) = (x.storeys.get(&row.storey).copied(), model.ceiling_types.get(&row.ceiling_type), x.inferred.storey_levels.get(&row.storey)) else {
        x.skip("ceiling", id, "its storey or type is missing");
        return;
    };
    let thickness: f64 = kind.layers.iter().map(|layer| layer.thickness).sum();
    if thickness <= 0.0 || row.boundary.len() < 3 {
        x.skip("ceiling", id, "it has no thickness or boundary");
        return;
    }
    let top = level.top_elevation - row.offset;
    let sloped = row.slope.is_some_and(|slope| slope.angle.abs() > 1e-12);
    let origin = x.ifc.axis3([0.0, 0.0, if sloped { 0.0 } else { top - thickness - storey.elevation }], None, None);
    let placement = x.ifc.place(Some(storey.placement), origin);
    let shape = if sloped {
        let Some(solid) = x.solid(id) else {
            x.skip("ceiling", id, "its sloped solid is not inferred");
            return;
        };
        brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |_, _| true))
    } else {
        let outer = x.ifc.loop_curve(&ccw(&row.boundary));
        let holes: Vec<u64> = row.holes.iter().map(|hole| x.ifc.loop_curve(&cw(hole))).collect();
        let profile = x.ifc.curve_profile(outer, &holes);
        let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], thickness);
        let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
        Some(x.ifc.definition(&[body]))
    };
    let element = x.product("IFCCOVERING", id, &row.name, placement, shape, vec![en("CEILING")]);
    x.contain(&row.storey, id, element);
    if let Some(object) = x.links.types.get(&("ceiling", row.ceiling_type.clone())).copied() {
        x.links.typed.entry(object).or_default().push(element);
    }
    if let Some(set) = x.links.layer_sets.get(&("ceiling", row.ceiling_type.clone())).copied() {
        let usage = x.ifc.add("IFCMATERIALLAYERSETUSAGE", vec![rf(set), en("AXIS3"), en("NEGATIVE"), real(thickness)]);
        x.links.materials.entry(usage).or_default().push(element);
    }
    x.quantify(element, "Qto_CoveringBaseQuantities", id, |row| vec![Quantity::Length("Width", row.width), Quantity::Area("GrossArea", row.gross_area), Quantity::Area("NetArea", row.net_area)]);
    if sloped {
        x.links.authoring.push((element, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    }
}

/// 🔲️ Writes every ceiling.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.ceilings {
        covering(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
