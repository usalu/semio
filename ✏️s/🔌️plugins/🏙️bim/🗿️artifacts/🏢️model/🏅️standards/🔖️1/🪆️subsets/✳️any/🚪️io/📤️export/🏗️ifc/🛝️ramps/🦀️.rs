//! 🛝️ Ramps. An `IfcRamp` aggregates one `IfcRampFlight` per sloped flight of the inferred run, one `IfcSlab` of type `LANDING` per landing and, when a side railing is asked for, an `IfcRailing`; every part carries the
//! faceted brep of its share of the inferred solid. IFC 2x3 has no slot for the authored path, landings, slope limit or top constraint, so the authored ramp record travels as the `Ramp` row of the `Semio_Authoring`
//! set of the `IfcRamp` (canonical JSON) and an import restores it exactly.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcramp.htm>

use super::brep::{brep_definition, mesh_where};
use super::data::label;
use super::writer::en;
use super::{Export, Quantity};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::parts;
use crate::standards::v1::subsets::any::schema::inferences::ramp_runs::{strip_of, RampRun, RampStrip};
use crate::Ramp;
use semio_framework_geometry::Point;

/// 🏷️ The `Semio_Authoring` row that holds the authored ramp record.
pub const RECORD_ROW: &str = "Ramp";

fn ramp_type(ramp: &Ramp, run: &RampRun) -> &'static str {
    let curved = ramp.path.iter().take(ramp.path.len().saturating_sub(1)).any(|vertex| vertex.bulge != 0.0);
    match (curved, run.landings.len()) {
        (true, _) => "SPIRAL_RAMP",
        (false, 0..=2) => "STRAIGHT_RUN_RAMP",
        (false, 3) => "TWO_STRAIGHT_RUN_RAMP",
        _ => "USERDEFINED",
    }
}

fn station(strip: &RampStrip, at: [f64; 3]) -> f64 {
    let point = Point::new(at[0], at[1]);
    strip
        .centre
        .iter()
        .enumerate()
        .map(|(index, segment)| {
            let closest = segment.closest(point);
            (closest.distance, strip.starts[index] + closest.t * segment.length())
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map_or(0.0, |best| best.1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    Flight(usize),
    Landing(usize),
}

fn piece_of(run: &RampRun, s: f64) -> Piece {
    let covers = |from: f64, to: f64| s >= from - 1e-6 && s <= to + 1e-6;
    run.flights.iter().position(|flight| covers(flight.from, flight.to)).map(Piece::Flight).or_else(|| run.landings.iter().position(|landing| covers(landing.from, landing.to)).map(Piece::Landing)).unwrap_or(Piece::Landing(0))
}

fn ramp(x: &mut Export<'_>, id: &str, row: &Ramp) {
    let Some(storey) = x.storeys.get(&row.storey).copied() else {
        x.skip("ramp", id, "its storey is missing");
        return;
    };
    let (Some(solid), Some(run)) = (x.solid(id), x.inferred.ramp_runs.get(id)) else {
        x.skip("ramp", id, "its solid or run is not inferred");
        return;
    };
    let strip = strip_of(row);
    if strip.is_empty() || run.length <= 1e-9 {
        x.skip("ramp", id, "its path has no length");
        return;
    }
    let placement = x.ifc.place(Some(storey.placement), x.ifc.origin);
    let entity = x.product("IFCRAMP", id, &row.name, placement, None, vec![en(ramp_type(row, run))]);
    x.contain(&row.storey, id, entity);
    let body = |part: &str| part == parts::BODY;
    let mut components = Vec::new();
    let part_of = |x: &mut Export<'_>, entity: &str, key: String, piece: Piece, tail: Vec<super::writer::V>| {
        let shape = brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |group, centroid| body(&group.part) && piece_of(run, station(&strip, centroid)) == piece));
        let axis = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let place = x.ifc.place(Some(storey.placement), axis);
        x.product(entity, &key, &row.name, place, shape, tail)
    };
    for (index, flight) in run.flights.iter().enumerate() {
        let part = part_of(x, "IFCRAMPFLIGHT", format!("{id}:flight{index}"), Piece::Flight(index), Vec::new());
        x.links.quantities.push((part, "Qto_RampFlightBaseQuantities", vec![Quantity::Length("Length", flight.length), Quantity::Length("Width", run.width)]));
        components.push(part);
    }
    for (index, landing) in run.landings.iter().enumerate() {
        let part = part_of(x, "IFCSLAB", format!("{id}:landing{index}"), Piece::Landing(index), vec![en("LANDING")]);
        x.links.quantities.push((part, "Qto_SlabBaseQuantities", vec![Quantity::Length("Width", run.width), Quantity::Length("Length", landing.length)]));
        components.push(part);
    }
    if row.railing_left || row.railing_right {
        let shape = brep_definition(&mut x.ifc, &mesh_where(solid, storey.elevation, |group, _| !body(&group.part)));
        let axis = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let place = x.ifc.place(Some(storey.placement), axis);
        components.push(x.product("IFCRAILING", &format!("{id}:railing"), &row.name, place, shape, vec![en("GUARDRAIL")]));
    }
    x.links.aggregated.insert(entity, components);
    if let Some(definition) = x.links.material_defs.get(&row.material).copied() {
        x.links.materials.entry(definition).or_default().push(entity);
    }
    x.quantify(entity, "Qto_RampBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Length("Width", row.width), Quantity::Length("Height", row.height), Quantity::Area("GrossArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume)]);
    x.links.authoring.push((entity, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
}

/// 🛝️ Writes every ramp.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.ramps {
        ramp(x, id, row);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
