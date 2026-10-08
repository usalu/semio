//! 🪟️ Curtain walls as `IfcCurtainWall` aggregating one `IfcMember` (all mullions) and one `IfcPlate` (all panels), each a faceted brep of the inferred solid's matching part.

use super::brep::{brep_definition, mesh_where};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::parts;

/// 🪟️ Writes every curtain wall.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    if model.curtain_walls.is_empty() {
        return;
    }
    for (id, row) in &model.curtain_walls {
        let Some(storey) = x.storeys.get(&row.storey).copied() else {
            x.skip("curtain wall", id, "its storey is missing");
            continue;
        };
        let Some(solid) = x.solid(id) else {
            x.skip("curtain wall", id, "its solid is not inferred");
            continue;
        };
        let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
        let wall = x.product("IFCCURTAINWALL", id, &row.name, placement, None, Vec::new());
        x.contain(&row.storey, id, wall);
        x.quantify(wall, "Qto_CurtainWallQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Length("Height", row.height), Quantity::Area("GrossSideArea", row.gross_side_area)]);
        let mut members = Vec::new();
        for (entity, part, material) in [("IFCMEMBER", parts::MULLION, &row.mullion_material), ("IFCPLATE", parts::PANEL, &row.panel_material)] {
            let mesh = mesh_where(solid, storey.elevation, |group, _| group.part == part);
            let Some(shape) = brep_definition(&mut x.ifc, &mesh) else { continue };
            let part_place = x.ifc.place(Some(storey.placement), x.ifc.origin);
            let member = x.product(entity, &format!("{id}:{part}"), &row.name, part_place, Some(shape), Vec::new());
            if let Some(definition) = x.links.material_defs.get(material).copied() {
                x.links.materials.entry(definition).or_default().push(member);
            }
            members.push(member);
        }
        x.links.aggregated.insert(wall, members);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
