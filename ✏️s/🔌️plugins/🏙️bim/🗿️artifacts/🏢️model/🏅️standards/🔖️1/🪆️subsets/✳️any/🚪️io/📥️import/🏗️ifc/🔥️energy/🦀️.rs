//! 🌡️ The thermal data of an IFC file: the conditions of a space and the thermal data of window and door types. The record the export wrote (the `Conditions` row of the space, the `UValue`, `GValue` and `FrameFraction` rows of a type) is read exactly; a file from
//! elsewhere gives the heating set point from `Pset_SpaceThermalRequirements` (`SpaceTemperatureWinterMin`, else `SpaceTemperatureMin`), the cooling set point (`SpaceTemperatureSummerMax`, else `SpaceTemperatureMax`) and, for windows and doors,
//! `ThermalTransmittance`, `GlazingAreaFraction` (the frame fraction is one minus it) and `Pset_DoorWindowGlazingType.SolarHeatGainTransmittance` of the type or of the occurrence.
//! A temperature of 100 or more is read as kelvin, a smaller one as degrees Celsius (the file may declare either unit). The property rows the export derived from the model are listed in the `DerivedRows` row of their element and are not read back as authored properties.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifckernel/pset/pset_spacethermalrequirements.htm

use super::data::{label, number, type_authoring};
use super::reader::{refs, text};
use super::spatial::{authoring_of, set_of, single_values};
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::energy::{CONDITIONS_ROW, DERIVED_ROW, SPACE_SET};
use crate::SpaceConditions;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::{BTreeMap, BTreeSet};

/// 🌡️ The thermal data of a window or door type.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thermal {
    pub u_value: Option<f64>,
    pub g_value: Option<f64>,
    pub frame_fraction: Option<f64>,
}

fn celsius(value: f64) -> f64 {
    if value >= 100.0 {
        value - crate::standards::v1::subsets::any::io::export::ifc::energy::KELVIN
    } else {
        value
    }
}

/// 🏷️ The `Set.Property` rows the export derived from the model for the instance `ifc`.
pub fn derived_rows(doc: &super::reader::Doc<'_>, ifc: u64) -> BTreeSet<String> {
    label(&authoring_of(doc, ifc), DERIVED_ROW).map(|list| list.split(',').filter(|row| !row.is_empty()).map(str::to_string).collect()).unwrap_or_default()
}

fn from_standard(rows: &BTreeMap<String, Part21Value>) -> Option<SpaceConditions> {
    let temperature = |first: &str, second: &str| number(rows, first).or_else(|| number(rows, second)).map(celsius);
    let (heating, cooling) = (temperature("SpaceTemperatureWinterMin", "SpaceTemperatureMin"), temperature("SpaceTemperatureSummerMax", "SpaceTemperatureMax"));
    (heating.is_some() || cooling.is_some()).then(|| SpaceConditions { heating_setpoint: heating, cooling_setpoint: cooling, ..SpaceConditions::empty() })
}

/// 🌡️ The conditions of the space `ifc`: the exact record, else the standard set; `None` for a space that states neither.
pub fn conditions_of(doc: &super::reader::Doc<'_>, ifc: u64) -> Option<SpaceConditions> {
    label(&authoring_of(doc, ifc), CONDITIONS_ROW).and_then(|record| from_json_str::<SpaceConditions>(&record, JsonMemberPolicy::Reject).ok()).or_else(|| from_standard(&set_of(doc, ifc, SPACE_SET)))
}

/// 🌡️ Reads the conditions of every imported space into `space_conditions`.
pub fn read_conditions(i: &mut Import<'_>) {
    for (instance, _) in i.doc.rows("IFCSPACE") {
        let (Some(id), Some(conditions)) = (i.ids.get(&instance.id).cloned(), conditions_of(&i.doc, instance.id)) else { continue };
        i.model.space_conditions.insert(id, conditions);
    }
}

fn type_set(doc: &super::reader::Doc<'_>, type_args: &[Part21Value], name: &str) -> BTreeMap<String, Part21Value> {
    let mut rows = BTreeMap::new();
    for set in refs(type_args, 5) {
        if let Some(args) = doc.args(set, "IFCPROPERTYSET") {
            if text(args, 2) == name {
                rows.extend(single_values(doc, args));
            }
        }
    }
    rows
}

fn standard(common: &BTreeMap<String, Part21Value>, glazing: &BTreeMap<String, Part21Value>) -> Thermal {
    Thermal { u_value: number(common, "ThermalTransmittance"), g_value: number(glazing, "SolarHeatGainTransmittance"), frame_fraction: number(common, "GlazingAreaFraction").map(|fraction| 1.0 - fraction) }
}

/// 🪟️ The thermal data of a window type object: the exact rows of its `Semio_Authoring` set, else its standard sets.
pub fn type_thermal(doc: &super::reader::Doc<'_>, type_args: &[Part21Value], window: bool) -> Thermal {
    let rows = type_authoring(doc, type_args);
    let exact = Thermal { u_value: number(&rows, "UValue"), g_value: number(&rows, "GValue"), frame_fraction: number(&rows, "FrameFraction") };
    if exact != Thermal::default() {
        return exact;
    }
    let common = type_set(doc, type_args, if window { "Pset_WindowCommon" } else { "Pset_DoorCommon" });
    let glazing = type_set(doc, type_args, "Pset_DoorWindowGlazingType");
    let found = standard(&common, &glazing);
    if window { found } else { Thermal { g_value: None, frame_fraction: None, ..found } }
}

/// 🪟️ The thermal data of a window or door occurrence `ifc` whose type object says nothing: its own standard sets.
pub fn occurrence_thermal(doc: &super::reader::Doc<'_>, ifc: u64, window: bool) -> Thermal {
    let common = set_of(doc, ifc, if window { "Pset_WindowCommon" } else { "Pset_DoorCommon" });
    let found = standard(&common, &set_of(doc, ifc, "Pset_DoorWindowGlazingType"));
    if window { found } else { Thermal { g_value: None, frame_fraction: None, ..found } }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
