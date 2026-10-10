//! 🪟️ Curtain walls as `IfcCurtainWall` aggregating one `IfcMember` (all mullions), one `IfcPlate` per material of the glass and opaque panels, and one `IfcDoor` or `IfcWindow` for every cell a door or
//! window panel fills, each a faceted brep of the inferred solid's matching part. IFC has no slot for the authored grid rules, the type or the panel overrides, so they travel as the `CurtainWall`,
//! `CurtainWallTypeId`, `CurtainWallType` and `CurtainPanelOverrides` rows of the `Semio_Authoring` set of the `IfcCurtainWall` (canonical JSON) and an import restores them exactly.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifccurtainwall.htm>

use super::brep::{brep_definition, mesh_where};
use super::data::label;
use super::writer::en;
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls::filler_cell;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::parts;
use crate::{CurtainPanel, OpeningKind};
use std::collections::{BTreeMap, BTreeSet};

/// 🏷️ The `Semio_Authoring` row that holds the authored curtain wall record.
pub const WALL_ROW: &str = "CurtainWall";
/// 🏷️ The `Semio_Authoring` row that holds the id of the curtain wall type.
pub const TYPE_ID_ROW: &str = "CurtainWallTypeId";
/// 🏷️ The `Semio_Authoring` row that holds the authored curtain wall type record.
pub const TYPE_ROW: &str = "CurtainWallType";
/// 🏷️ The `Semio_Authoring` row that holds the panel overrides of the wall with their ids.
pub const OVERRIDES_ROW: &str = "CurtainPanelOverrides";

/// 🪟️ Writes every curtain wall.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.curtain_walls {
        let Some(storey) = x.storeys.get(&row.storey).copied() else {
            x.skip("curtain wall", id, "its storey is missing");
            continue;
        };
        let (Some(solid), Some(kind)) = (x.solid(id), model.curtain_wall_types.get(&row.curtain_wall_type)) else {
            x.skip("curtain wall", id, "its solid is not inferred or its type is missing");
            continue;
        };
        let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
        let wall_tail = x.by(Vec::new(), vec![super::writer::unset()]);
        let wall = x.product("IFCCURTAINWALL", id, &row.name, placement, None, wall_tail);
        x.contain(&row.storey, id, wall);
        x.quantify(wall, "Qto_CurtainWallQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Length("Height", row.height), Quantity::Area("GrossSideArea", row.gross_side_area)]);
        let mut members = Vec::new();
        let piece = |x: &mut Export<'_>, entity: &str, key: String, material: &str, tail: Vec<super::writer::V>, keep: &dyn Fn(&crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidGroup) -> bool| {
            let mesh = mesh_where(solid, storey.elevation, |group, _| keep(group));
            let shape = brep_definition(&mut x.ifc, &mesh)?;
            let part_place = x.ifc.place(Some(storey.placement), x.ifc.origin);
            let member = x.product(entity, &key, &row.name, part_place, Some(shape), tail);
            if let Some(definition) = x.links.material_defs.get(material).copied() {
                x.links.materials.entry(definition).or_default().push(member);
            }
            Some(member)
        };
        let mullion_tail = x.by(Vec::new(), vec![en("MULLION")]);
        members.extend(piece(x, "IFCMEMBER", format!("{id}:{}", parts::MULLION), &kind.mullion_material, mullion_tail, &|group| group.layer == 0 && group.part == parts::MULLION));
        let plates: BTreeSet<&str> = solid.groups.iter().filter(|group| group.layer == 0 && group.part == parts::PANEL).map(|group| group.material.as_str()).collect();
        for material in plates {
            let plate_tail = x.by(Vec::new(), vec![en("CURTAIN_PANEL")]);
            members.extend(piece(x, "IFCPLATE", format!("{id}:{}:{material}", parts::PANEL), material, plate_tail, &|group| group.layer == 0 && group.part == parts::PANEL && group.material == material));
        }
        if let Some(layout) = x.inferred.curtain_layout.get(id) {
            let layers: BTreeSet<u32> = solid.groups.iter().map(|group| group.layer).filter(|layer| *layer > 0).collect();
            for layer in layers {
                let Some(panel) = filler_cell(layout, layer).and_then(|(u, v)| layout.panel_of(u, v)) else { continue };
                let (entity, material, opening) = match panel {
                    CurtainPanel::Door { door_type } => ("IFCDOOR", model.door_types.get(door_type).map(|row| row.material.as_str()), OpeningKind::Door { door_type: door_type.clone() }),
                    CurtainPanel::Window { window_type } => ("IFCWINDOW", model.window_types.get(window_type).map(|row| row.material.as_str()), OpeningKind::Window { window_type: window_type.clone() }),
                    _ => continue,
                };
                let size = filler_cell(layout, layer).and_then(|(u, v)| layout.cell(u, v)).unwrap_or((0.0, 0.0));
                let filler_tail = super::walls::filling_tail(x, &opening, size.1, size.0);
                members.extend(piece(x, entity, format!("{id}:filler{layer}"), material.unwrap_or_default(), filler_tail, &|group| group.layer == layer));
            }
        }
        x.links.aggregated.insert(wall, members);
        let overrides: BTreeMap<String, crate::CurtainPanelOverride> = model.curtain_panel_overrides.iter().filter(|(_, row)| row.curtain == *id).map(|(key, row)| (key.clone(), row.clone())).collect();
        let mut rows = vec![(WALL_ROW, label(&semio_framework_pack_json::to_json_string(row))), (TYPE_ID_ROW, label(&row.curtain_wall_type)), (TYPE_ROW, label(&semio_framework_pack_json::to_json_string(kind)))];
        if !overrides.is_empty() {
            rows.push((OVERRIDES_ROW, label(&semio_framework_pack_json::to_json_string(&overrides))));
        }
        x.links.authoring.push((wall, rows));
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
