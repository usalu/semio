//! 🛝️ Ramps. An `IfcRamp` aggregates one `IfcRampFlight` per sloped flight of the inferred run, one `IfcSlab` of type `LANDING` per landing and, when a side railing is asked for, an `IfcRailing`; every part carries the
//! faceted brep of its share of the inferred solid. IFC 2x3 has no slot for the authored path, landings, slope limit or top constraint, so the authored ramp record travels as the `Ramp` row of the `Semio_Authoring`
//! set of the `IfcRamp` (canonical JSON) and an import restores it exactly.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcsharedbldgelements/lexical/ifcramp.htm>

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep::{brep_definition, mesh_where};
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::label;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::en;
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context::{Export, Quantity};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::{strip_of, RampRun};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Ramp;
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::element_solids::ramps::{body_mesh, stations_of};

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

pub fn ramp(x: &mut Export<'_>, id: &str, row: &Ramp) {
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
    let entity = x.product("IFCRAMP", id, &row.name, placement, None, vec![en(match (ramp_type(row, run), x.schema()) { ("USERDEFINED", semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::Schema::Ifc4) => "NOTDEFINED", (kind, _) => kind })]);
    x.contain(&row.storey, id, entity);
    let body = |part: &str| part == semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::parts::BODY;
    let mut stations = stations_of(&strip, run);
    for station in &mut stations { station.z -= storey.elevation; }
    let mut components = Vec::new();
    let part_of = |x: &mut Export<'_>, entity: &str, key: String, from: f64, to: f64, tail: Vec<semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::V>| {
        let first = stations.partition_point(|station| station.s < from - 1e-9);
        let end = stations.partition_point(|station| station.s <= to + 1e-9);
        let shape = brep_definition(&mut x.ifc, &body_mesh(&stations[first..end], row.thickness));
        let axis = x.ifc.axis3([0.0, 0.0, 0.0], None, None);
        let place = x.ifc.place(Some(storey.placement), axis);
        x.product(entity, &key, &row.name, place, shape, tail)
    };
    for (index, flight) in run.flights.iter().enumerate() {
        let flight_tail = x.by(Vec::new(), vec![en(if row.path.iter().take(row.path.len().saturating_sub(1)).any(|vertex| vertex.bulge != 0.0) { "SPIRAL" } else { "STRAIGHT" })]);
        let part = part_of(x, "IFCRAMPFLIGHT", format!("{id}:flight{index}"), flight.from, flight.to, flight_tail);
        x.links.quantities.push((part, "Qto_RampFlightBaseQuantities", vec![Quantity::Length("Length", flight.length), Quantity::Length("Width", run.width)]));
        components.push(part);
    }
    for (index, landing) in run.landings.iter().enumerate() {
        let part = part_of(x, "IFCSLAB", format!("{id}:landing{index}"), landing.from, landing.to, vec![en("LANDING")]);
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
    x.quantify(entity, "Qto_RampBaseQuantities", id, |row| vec![Quantity::Length("Length", row.length), Quantity::Length("Width", row.width), Quantity::Length("Height", row.height.abs()), Quantity::Area("GrossArea", row.gross_area), Quantity::Volume("GrossVolume", row.gross_volume)]);
    x.links.authoring.push((entity, vec![(RECORD_ROW, label(&semio_framework_pack_json::to_json_string(row)))]));
}

/// 🛝️ Writes every ramp.
pub fn emit(x: &mut Export<'_>) {
    let model = x.model;
    for (id, row) in &model.ramps {
        ramp(x, id, row);
    }
}


