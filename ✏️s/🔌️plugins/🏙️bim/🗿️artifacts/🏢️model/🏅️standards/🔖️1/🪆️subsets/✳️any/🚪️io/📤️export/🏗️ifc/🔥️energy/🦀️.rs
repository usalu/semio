//! 🌡️ The thermal data of the model in the IFC file (IFC 2x3 and IFC4), for readers that know the standard property sets and for the importer that wants the model back.
//! * a space with conditions gets `Pset_SpaceThermalRequirements`: `SpaceTemperatureMin` and `SpaceTemperatureWinterMin` are the heating set point, `SpaceTemperatureMax` and `SpaceTemperatureSummerMax` the cooling set point (in kelvin: the file
//!   declares no temperature unit, so the SI base unit applies), `AirConditioning` whether a cooling set point is stated and the outdoor air flow as air changes per hour (`MechanicalVentilationRate` of a conditioned space,
//!   `NaturalVentilationRate` and `NaturalVentilation` of an unconditioned one); the record itself travels exactly as the `Conditions` row of its `Semio_Authoring` set.
//! * `ThermalTransmittance` of the standard common sets: walls (`Pset_WallCommon`), curtain walls (`Pset_CurtainWallCommon`: the authored U of their type; g-value and frame fraction travel in the `CurtainWallType` record of the element), slabs (`Pset_SlabCommon`) and roofs (`Pset_RoofCommon`, IFC4 only: the 2x3 set has no such property) take the area-weighted U-value of the
//!   envelope surfaces they hold (see [`holder_of`]); windows (`Pset_WindowCommon`, with `GlazingAreaFraction` = 1 - frame fraction, and `Pset_DoorWindowGlazingType.SolarHeatGainTransmittance`) and doors (`Pset_DoorCommon`) take the value of their
//!   type, which also keeps `UValue`, `GValue` and `FrameFraction` exactly in its `Semio_Authoring` set.
//! An authored property of that name (on the element, or on the type of a window or door) wins and is written as before; every derived row is listed in the `DerivedRows` row of the element so the importer does not read it back as an authored property.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifckernel/pset/pset_spacethermalrequirements.htm

use super::data::label;
use super::projection::{json_cell, plain, quote, schema_of, Cell};
use super::writer::{flag, real, typed, V};
use super::{Export, Schema};
use crate::standards::v1::subsets::any::io::export::holders::{holder_of, Holder};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::SurfaceKind;
use crate::OpeningKind;
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 🏷️ The `Semio_Authoring` row that holds the record of the conditions of a space as JSON.
pub const CONDITIONS_ROW: &str = "Conditions";
/// 🏷️ The `Semio_Authoring` row that lists the property rows (`Set.Property`, comma separated) derived from the model.
pub const DERIVED_ROW: &str = "DerivedRows";
/// 🌡️ The standard set of the thermal requirements of a space.
pub const SPACE_SET: &str = "Pset_SpaceThermalRequirements";
/// 🌡️ Kelvin at zero degrees Celsius.
pub const KELVIN: f64 = 273.15;

fn kelvin(celsius: f64) -> V {
    typed("IFCTHERMODYNAMICTEMPERATUREMEASURE", real(celsius + KELVIN))
}

fn transmittance(value: f64) -> V {
    typed("IFCTHERMALTRANSMITTANCEMEASURE", real(value))
}

fn authored(x: &Export<'_>, holders: &[&str], set: &str, property: &str) -> bool {
    holders.iter().any(|holder| x.model.properties.get(*holder).and_then(|sets| sets.get(set)).is_some_and(|rows| rows.contains_key(property)))
}

fn derive(x: &mut Export<'_>, element: &str, authors: &[&str], set: &'static str, rows: Vec<(&'static str, V)>) {
    let Some(entity) = x.links.elements.get(element).copied() else { return };
    let rows: Vec<(&'static str, V)> = rows.into_iter().filter(|(property, _)| !authored(x, authors, set, property)).collect();
    if rows.is_empty() {
        return;
    }
    let listed: Vec<String> = rows.iter().map(|(property, _)| format!("{set}.{property}")).collect();
    x.links.authoring.push((entity, vec![(DERIVED_ROW, label(&listed.join(",")))]));
    x.links.derived.entry(element.to_string()).or_default().push((set, rows));
}

fn spaces(x: &mut Export<'_>) {
    let model = x.model;
    for (id, conditions) in &model.space_conditions {
        let Some(entity) = x.links.elements.get(id).copied() else {
            x.skip("conditions", id, "the space is not part of the export");
            continue;
        };
        x.links.authoring.push((entity, vec![(CONDITIONS_ROW, label(&semio_framework_pack_json::to_json_string(conditions)))]));
        let mut rows: Vec<(&'static str, V)> = Vec::new();
        if let Some(heating) = conditions.heating_setpoint {
            rows.push(("SpaceTemperatureMin", kelvin(heating)));
            rows.push(("SpaceTemperatureWinterMin", kelvin(heating)));
        }
        if let Some(cooling) = conditions.cooling_setpoint {
            rows.push(("SpaceTemperatureMax", kelvin(cooling)));
            rows.push(("SpaceTemperatureSummerMax", kelvin(cooling)));
        }
        rows.push(("AirConditioning", typed("IFCBOOLEAN", flag(conditions.cooling_setpoint.is_some()))));
        let height = x.inferred.energy_envelopes.get(id).map_or(0.0, |envelope| envelope.height);
        if let (Some(rate), true) = (conditions.ventilation_rate, height > 0.0) {
            let changes = typed("IFCCOUNTMEASURE", real(rate * 3.6 / height));
            if conditions.conditioned() {
                rows.push(("MechanicalVentilationRate", changes));
            } else {
                rows.push(("NaturalVentilation", typed("IFCBOOLEAN", flag(true))));
                rows.push(("NaturalVentilationRate", changes));
            }
        }
        derive(x, id, &[id.as_str()], SPACE_SET, rows);
    }
}

fn opaque(x: &mut Export<'_>) {
    let (model, inferred) = (x.model, x.inferred);
    let mut sums: BTreeMap<(&'static str, String), (f64, f64)> = BTreeMap::new();
    for envelope in inferred.energy_envelopes.values() {
        for surface in envelope.surfaces.iter().filter(|surface| matches!(surface.kind, SurfaceKind::Wall | SurfaceKind::CurtainWall | SurfaceKind::Floor | SurfaceKind::Ceiling)) {
            let (Some(u_value), Some(holder)) = (surface.u_value, holder_of(model, surface)) else { continue };
            let set = match holder {
                Holder::Wall(_) => "Pset_WallCommon",
                Holder::CurtainWall(_) => "Pset_CurtainWallCommon",
                Holder::Slab(_) => "Pset_SlabCommon",
                Holder::Roof(_) if x.schema() == Schema::Ifc4 => "Pset_RoofCommon",
                _ => continue,
            };
            let entry = sums.entry((set, holder.id().to_string())).or_default();
            entry.0 += u_value * surface.area;
            entry.1 += surface.area;
        }
    }
    for ((set, id), (weighted, area)) in sums {
        if area > 0.0 {
            derive(x, &id, &[id.as_str()], set, vec![("ThermalTransmittance", transmittance(weighted / area))]);
        }
    }
}

fn frames(x: &mut Export<'_>) {
    let model = x.model;
    for (id, opening) in &model.openings {
        match &opening.kind {
            OpeningKind::Window { window_type } => {
                let Some(kind) = model.window_types.get(window_type) else { continue };
                let mut rows: Vec<(&'static str, V)> = Vec::new();
                rows.extend(kind.u_value.map(|value| ("ThermalTransmittance", transmittance(value))));
                rows.extend(kind.frame_fraction.map(|fraction| ("GlazingAreaFraction", typed("IFCPOSITIVERATIOMEASURE", real(1.0 - fraction)))));
                derive(x, id, &[id.as_str(), window_type.as_str()], "Pset_WindowCommon", rows);
                let glazing = kind.g_value.map(|value| ("SolarHeatGainTransmittance", typed(x.by("IFCPOSITIVERATIOMEASURE", "IFCNORMALISEDRATIOMEASURE"), real(value))));
                derive(x, id, &[id.as_str(), window_type.as_str()], "Pset_DoorWindowGlazingType", glazing.into_iter().collect());
            }
            OpeningKind::Door { door_type } => {
                let Some(kind) = model.door_types.get(door_type) else { continue };
                derive(x, id, &[id.as_str(), door_type.as_str()], "Pset_DoorCommon", kind.u_value.map(|value| ("ThermalTransmittance", transmittance(value))).into_iter().collect());
            }
            OpeningKind::Void { .. } => {}
        }
    }
}

/// 🌡️ Records the thermal rows of the spaces, walls, slabs, roofs, windows and doors; `emit_links` writes them next to the authored sets.
pub fn emit(x: &mut Export<'_>) {
    spaces(x);
    opaque(x);
    frames(x);
}

//#region 🔖️Report
/// 🌡️ The thermal sets the report reads.
pub const REPORTED: [&str; 8] = ["Pset_SpaceThermalRequirements", "Pset_WallCommon", "Pset_CurtainWallCommon", "Pset_SlabCommon", "Pset_RoofCommon", "Pset_WindowCommon", "Pset_DoorCommon", "Pset_DoorWindowGlazingType"];

const TYPES: [&str; 4] = ["IFCWINDOWSTYLE", "IFCWINDOWTYPE", "IFCDOORSTYLE", "IFCDOORTYPE"];

fn holder_id(instance: &Part21Instance) -> Option<String> {
    let (name, args) = instance.primary()?;
    args.get(if name == "IFCSPACE" { 4 } else { 7 }).and_then(Part21Value::as_str).filter(|value| !value.is_empty()).map(str::to_string)
}

fn rows_of(document: &Part21Document, set: &[Part21Value]) -> BTreeMap<String, Cell> {
    set[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (plain(row, 0), json_cell(&row[2]))).collect()
}

fn raw_text(document: &Part21Document, set: &[Part21Value], name: &str) -> Option<String> {
    set[4].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).find(|row| plain(row, 0) == name).and_then(|row| row[2].as_typed().and_then(|(_, items)| items.first()).and_then(Part21Value::as_str).map(str::to_string))
}

/// 📊️ The canonical JSON report of the thermal data of a written document, the table the IfcOpenShell oracle measures from the file: the thermal sets of every product by model id with the IFC type and value of each property, the `DerivedRows` and `Conditions`
/// bookkeeping rows, and the `UValue`, `GValue` and `FrameFraction` rows of the window and door types.
pub fn thermal_json(document: &Part21Document) -> String {
    let mut elements: BTreeMap<String, BTreeMap<String, BTreeMap<String, Cell>>> = BTreeMap::new();
    let (mut derived, mut conditions): (BTreeMap<String, String>, BTreeMap<String, String>) = (BTreeMap::new(), BTreeMap::new());
    for relation in document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|instance| instance.entity("IFCRELDEFINESBYPROPERTIES")) {
        let Some(set) = document.resolve(&relation[5]).and_then(|set| set.entity("IFCPROPERTYSET")) else { continue };
        let name = plain(set, 2);
        for id in relation[4].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(holder_id) {
            if REPORTED.contains(&name.as_str()) {
                elements.entry(id.clone()).or_default().insert(name.clone(), rows_of(document, set));
            } else if name == "Semio_Authoring" {
                if let Some(text) = raw_text(document, set, DERIVED_ROW) {
                    derived.insert(id.clone(), text);
                }
                if let Some(text) = raw_text(document, set, CONDITIONS_ROW) {
                    conditions.insert(id, text);
                }
            }
        }
    }
    let mut types: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for instance in TYPES.iter().flat_map(|name| document.by_type(name)) {
        let (Some((_, args)), Some(id)) = (instance.primary(), holder_id(instance)) else { continue };
        for set in args[5].as_list().unwrap_or_default().iter().filter_map(|item| document.resolve(item)).filter_map(|set| set.entity("IFCPROPERTYSET")).filter(|set| set[2].as_str() == Some("Semio_Authoring")) {
            let rows = rows_of(document, set);
            let found: Vec<(String, String)> = ["UValue", "GValue", "FrameFraction"].iter().filter_map(|name| rows.get(*name).map(|(_, json)| (name.to_string(), json.clone()))).collect();
            if !found.is_empty() {
                types.insert(id.clone(), found);
            }
        }
    }
    let elements = elements
        .iter()
        .map(|(id, sets)| format!("{}:{{{}}}", quote(id), sets.iter().map(|(set, rows)| format!("{}:{{{}}}", quote(set), rows.iter().map(|(name, (kind, json))| format!("{}:[{},{json}]", quote(name), quote(kind))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",")))
        .collect::<Vec<_>>()
        .join(",");
    let texts = |rows: &BTreeMap<String, String>| rows.iter().map(|(id, text)| format!("{}:{}", quote(id), quote(text))).collect::<Vec<_>>().join(",");
    let types = types.iter().map(|(id, rows)| format!("{}:{{{}}}", quote(id), rows.iter().map(|(name, json)| format!("{}:{json}", quote(name))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",");
    format!("{{\"schema\":{},\"elements\":{{{elements}}},\"derived\":{{{}}},\"conditions\":{{{}}},\"types\":{{{types}}}}}", quote(schema_of(document).id()), texts(&derived), texts(&conditions))
}

/// 📊️ The thermal report of the export of `model` in `schema`.
pub fn thermal_report(schema: Schema, model: &crate::ModelSnapshot) -> Result<String, String> {
    super::model_to_part21(schema, model).map(|(document, _)| thermal_json(&document))
}
//#endregion 🔖️Report

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
