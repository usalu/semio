//! 🏠️ Spaces as `IfcSpace`: the inferred room outline (islands as voids) extruded by the clear height from the room's floor, aggregated under its storey.
//! `Name` is the space number, `LongName` its name, `Description` its usage.

use super::frames::{ccw, cw};
use super::writer::{en, opt_text, real, rf};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::finishes::FinishSurface;
use crate::standards::v1::subsets::any::schema::inferences::quantities::ElementQuantity;
use crate::standards::v1::subsets::any::schema::inferences::spaces::SpaceStatus;

/// 🏷️ The `Semio_Authoring` row that holds the authored boundary rule of a space whose outline is inferred.
pub const BOUNDARY_ROW: &str = "Boundary";

fn finish_area(row: &ElementQuantity, surface: FinishSurface) -> f64 {
    row.finishes.iter().filter(|finish| finish.surface == surface).map(|finish| finish.area).sum()
}

/// 🏠️ Writes every space.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    if model.spaces.is_empty() {
        return;
    }
    let rooms = &x.inferred.spaces;
    for (id, row) in &model.spaces {
        let (Some(storey), Some(room)) = (x.storeys.get(&row.storey).copied(), rooms.get(id)) else {
            x.skip("space", id, "its storey is missing or no room is inferred");
            continue;
        };
        let resolved = matches!(room.status, SpaceStatus::Inferred | SpaceStatus::Explicit) && room.outline.len() >= 3 && room.clear_height > 0.0;
        let origin = x.ifc.axis3([0.0, 0.0, room.floor_z - storey.elevation], None, None);
        let placement = x.ifc.place(Some(storey.placement), origin);
        let shape = resolved.then(|| {
            let outer = x.ifc.loop_curve(&ccw(&room.outline));
            let holes: Vec<u64> = room.holes.iter().map(|hole| x.ifc.loop_curve(&cw(hole))).collect();
            let profile = x.ifc.curve_profile(outer, &holes);
            let solid = x.ifc.extrusion(profile, x.ifc.origin, [0.0, 0.0, 1.0], room.clear_height);
            let body = x.ifc.shape(x.ifc.body, "Body", "SweptSolid", &[solid]);
            x.ifc.definition(&[body])
        });
        if !resolved {
            x.skip("space", id, &format!("its outline is not resolved ({:?})", room.status));
        }
        let args = vec![opt_text(id), rf(placement), shape.map_or(super::writer::unset(), rf), opt_text(&row.name), en("ELEMENT"), en("INTERNAL"), real(room.floor_z)];
        let entity = x.ifc.rooted("IFCSPACE", id, &row.number, &row.usage, args);
        x.links.elements.insert(id.clone(), entity);
        x.links.aggregated.entry(storey.ifc).or_default().push(entity);
        x.phase(id, entity);
        if !matches!(row.boundary, crate::SpaceBoundary::Explicit { .. }) {
            x.links.authoring.push((entity, vec![(BOUNDARY_ROW, super::data::label(&semio_framework_pack_json::to_json_string(&row.boundary)))]));
        }
        super::zoning::finishes(x, id, entity);
        if resolved {
            x.quantify(entity, "Qto_SpaceBaseQuantities", id, |row| {
                vec![Quantity::Length("Height", row.height), Quantity::Area("GrossFloorArea", row.gross_area), Quantity::Area("NetFloorArea", row.net_area), Quantity::Volume("GrossVolume", row.gross_volume), Quantity::Length("GrossPerimeter", row.perimeter), Quantity::Area("GrossWallArea", row.perimeter * row.height), Quantity::Area("NetWallArea", finish_area(row, FinishSurface::Wall)), Quantity::Area("NetCeilingArea", finish_area(row, FinishSurface::Ceiling))]
            });
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
