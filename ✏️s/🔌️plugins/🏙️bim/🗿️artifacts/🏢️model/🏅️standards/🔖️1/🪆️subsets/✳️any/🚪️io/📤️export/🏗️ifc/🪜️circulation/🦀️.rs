//! 🪜️ Stairs and railings. An `IfcStair` carries the faceted brep of its inferred solid and aggregates one `IfcStairFlight` per flight of the inferred run (riser and tread counts, riser height, tread length);
//! an `IfcRailing` carries the faceted brep of its posts and rail. IFC 2x3 has no stringer, baluster or infill: the stringer boards of a stair are an `IfcMember` (tag `<id>:stringer`) aggregated by the `IfcStair` next to its flights, the balusters
//! of a railing an `IfcMember` (`<id>:baluster`) and its infill an `IfcPlate` (`<id>:infill`) aggregated by the `IfcRailing`; the flights and the railing keep only their own parts.

use super::brep::{brep_definition, mesh_where};
use super::data::label;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{parts, ElementSolid};
use super::writer::{en, int, real};
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::stair_runs::{StairFlightRun, StairRun};
use crate::{Stair, StairFlight};

/// 🏷️ The `Semio_Authoring` row that holds the authored stair record.
pub const STAIR_ROW: &str = "Stair";
/// 🏷️ The `Semio_Authoring` row that holds the authored railing record.
pub const RAILING_ROW: &str = "Railing";

fn stair_type(flight: &StairFlight) -> &'static str {
    match flight {
        StairFlight::Straight => "STRAIGHT_RUN_STAIR",
        StairFlight::LTurn { .. } => "QUARTER_TURN_STAIR",
        StairFlight::UTurn { .. } => "HALF_TURN_STAIR",
        StairFlight::Spiral { .. } => "SPIRAL_STAIR",
    }
}

/// 📏️ How far a plan point lies from the footprint of one flight (zero inside it).
fn gap(flight: &StairFlightRun, width: f64, centroid: [f64; 3]) -> f64 {
    let (sin, cos) = flight.direction.sin_cos();
    let (dx, dy) = (centroid[0] - flight.start.x, centroid[1] - flight.start.y);
    let (along, across) = (dx * cos + dy * sin, -dx * sin + dy * cos);
    (along - flight.length).max(-along).max(0.0).hypot((across.abs() - width / 2.0).max(0.0))
}

/// 🪜️ The flight whose footprint is nearest to a plan point; landings go to the arriving flight.
fn nearest_flight(run: &StairRun, centroid: [f64; 3]) -> usize {
    run.flights.iter().enumerate().min_by(|a, b| gap(a.1, run.width, centroid).total_cmp(&gap(b.1, run.width, centroid))).map_or(0, |best| best.0)
}

/// 🧱️ The material definition of the first group of a part of a solid.
fn material_of(x: &Export<'_>, solid: &ElementSolid, part: &str) -> Option<u64> {
    solid.groups.iter().find(|group| group.part == part).and_then(|group| x.links.material_defs.get(&group.material).copied())
}

/// 🧩️ Writes the triangles of one part of a solid as a product of `class` aggregated by its parent; nothing when the solid has none.
fn part_of(x: &mut Export<'_>, solid: &ElementSolid, storey: super::StoreyRef, id: &str, name: &str, class: &str, part: &str) -> Option<u64> {
    let shape = brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |group, _| group.part == part))?;
    let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let tail = x.by(Vec::new(), vec![en(match part { parts::STRINGER => "STRINGER", parts::BALUSTER => "POST", _ => "SHEET" })]);
    let entity = x.product(class, &format!("{id}:{part}"), name, placement, Some(shape), tail);
    if let Some(definition) = material_of(x, solid, part) {
        x.links.materials.entry(definition).or_default().push(entity);
    }
    Some(entity)
}

fn stair(x: &mut Export<'_>, id: &str, row: &Stair) {
    let Some(storey) = x.storeys.get(&row.storey).copied() else {
        x.skip("stair", id, "its storey is missing");
        return;
    };
    let (Some(solid), Some(run)) = (x.solid(id), x.inferred.stair_runs.get(id)) else {
        x.skip("stair", id, "its solid or run is not inferred");
        return;
    };
    let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let entity = x.product("IFCSTAIR", id, &row.name, placement, None, vec![en(stair_type(&row.flight))]);
    x.contain(&row.storey, id, entity);
    x.links.authoring.push((entity, vec![(STAIR_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    let mut flights = Vec::new();
    for (index, flight) in run.flights.iter().enumerate() {
        let shape = brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |group, centroid| group.part != parts::STRINGER && nearest_flight(run, centroid) == index));
        let axis = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let flight_place = x.ifc.place(Some(storey.placement), axis);
        let mut tail = vec![int(i64::from(flight.risers)), int(i64::from(flight.treads)), real(run.riser_height), real(flight.tread)];
        tail.extend(x.by(Vec::new(), vec![en(if matches!(row.flight, StairFlight::Spiral { .. }) { "SPIRAL" } else { "STRAIGHT" })]));
        let part = x.product("IFCSTAIRFLIGHT", &format!("{id}:flight{index}"), &row.name, flight_place, shape, tail);
        x.links.quantities.push((part, "Qto_StairFlightBaseQuantities", vec![Quantity::Length("Length", flight.length), Quantity::Length("Width", run.width)]));
        flights.push(part);
    }
    flights.extend(part_of(x, solid, storey, id, &row.name, "IFCMEMBER", parts::STRINGER));
    x.links.aggregated.insert(entity, flights);
}

fn railing(x: &mut Export<'_>, id: &str, row: &crate::Railing) {
    let Some(storey) = x.storeys.get(&row.storey).copied() else {
        x.skip("railing", id, "its storey is missing");
        return;
    };
    let Some(solid) = x.solid(id) else {
        x.skip("railing", id, "its solid is not inferred");
        return;
    };
    let shape = brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |group, _| group.part != parts::BALUSTER && group.part != parts::INFILL));
    let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let entity = x.product("IFCRAILING", id, &row.name, placement, shape, vec![en("GUARDRAIL")]);
    x.contain(&row.storey, id, entity);
    x.links.authoring.push((entity, vec![(RAILING_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
    let infill: Vec<u64> = [part_of(x, solid, storey, id, &row.name, "IFCMEMBER", parts::BALUSTER), part_of(x, solid, storey, id, &row.name, "IFCPLATE", parts::INFILL)].into_iter().flatten().collect();
    if !infill.is_empty() {
        x.links.aggregated.insert(entity, infill);
    }
    if let Some(definition) = x.links.material_defs.get(&row.material).copied() {
        x.links.materials.entry(definition).or_default().push(entity);
    }
    x.quantify(entity, "Qto_RailingBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Length("Height", row.height)]);
}

/// 🪜️ Writes every stair and railing.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.stairs {
        stair(x, id, row);
    }
    for (id, row) in &model.railings {
        railing(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
