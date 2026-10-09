//! 🧬️ The non-geometric data of an IFC file: materials with their physical properties, layered and profiled types, window and door styles, user property sets and classifications.
//! Parameters IFC has no slot for are read from the `Semio_Authoring` set that the export writes.

use super::reader::{opt_text, real, refs, text, Doc};
use super::spatial::{number_of, single_values, string_of};
use super::Import;
use crate::{BeamType, CeilingType, Classification, ColumnType, DoorLeaves, DoorType, Layer, LayerFunction, Material, MaterialCategory, Profile, PropertyValue, Rgb, RoofType, SlabType, Swing, WallType, WindowType};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

fn category(name: &str) -> MaterialCategory {
    match name {
        "Concrete" => MaterialCategory::Concrete,
        "Masonry" => MaterialCategory::Masonry,
        "Wood" => MaterialCategory::Wood,
        "Metal" => MaterialCategory::Metal,
        "Glass" => MaterialCategory::Glass,
        "Insulation" => MaterialCategory::Insulation,
        "Finish" => MaterialCategory::Finish,
        "Membrane" => MaterialCategory::Membrane,
        _ => MaterialCategory::Other,
    }
}

fn function(name: &str) -> LayerFunction {
    match name {
        "Substrate" => LayerFunction::Substrate,
        "Insulation" => LayerFunction::Insulation,
        "Finish" => LayerFunction::Finish,
        "Membrane" => LayerFunction::Membrane,
        "Core" => LayerFunction::Core,
        _ => LayerFunction::Structure,
    }
}

/// 🔤️ The text of an authoring row.
pub fn label(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<String> {
    rows.get(name).and_then(string_of)
}

/// 🔢️ The number of an authoring row.
pub fn number(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<f64> {
    rows.get(name).and_then(number_of)
}

/// 🏷️ The `Semio_Authoring` rows listed in a type object's `HasPropertySets`.
pub fn type_authoring(doc: &Doc<'_>, type_args: &[Part21Value]) -> BTreeMap<String, Part21Value> {
    let mut rows = BTreeMap::new();
    for set in refs(type_args, 5) {
        if let Some(args) = doc.args(set, "IFCPROPERTYSET") {
            if text(args, 2) == "Semio_Authoring" {
                rows.extend(single_values(doc, args));
            }
        }
    }
    rows
}

//#region 🔖️Materials
fn extended_rows(doc: &Doc<'_>, material: u64) -> BTreeMap<(String, String), Part21Value> {
    let mut rows = BTreeMap::new();
    for (_, args) in doc.rows("IFCEXTENDEDMATERIALPROPERTIES") {
        if args[0].as_ref_id() == Some(material) {
            let set = text(args, 3);
            for (name, value) in args[1].as_list().unwrap_or_default().iter().filter_map(|item| doc.follow_args(item, "IFCPROPERTYSINGLEVALUE")).map(|row| (text(row, 0), row[2].clone())) {
                rows.insert((set.clone(), name), value);
            }
        }
    }
    rows
}

/// 🧱️ Reads every `IfcMaterial` with its physical properties, colour and category.
pub fn read_materials(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCMATERIAL") {
        let name = text(args, 0);
        let rows = extended_rows(&i.doc, instance.id);
        let find = |set: &str, property: &str| rows.get(&(set.to_string(), property.to_string())).and_then(number_of);
        let id = rows.get(&("Semio_Authoring".to_string(), "Id".to_string())).and_then(string_of).unwrap_or_else(|| Import::unused(&format!("m-{}", Import::slug(&name)), |candidate| i.model.materials.contains_key(candidate)));
        let kind = rows.get(&("Semio_Authoring".to_string(), "Category".to_string())).and_then(string_of).unwrap_or_default();
        let colour = |channel: &str| find("Semio_Authoring", channel).unwrap_or(0.5);
        i.model.materials.insert(
            id.clone(),
            Material {
                name,
                category: category(&kind),
                color: Rgb { r: colour("Red"), g: colour("Green"), b: colour("Blue") },
                density: find("Pset_MaterialCommon", "MassDensity").unwrap_or(0.0),
                conductivity: find("Pset_MaterialThermal", "ThermalConductivity").unwrap_or(0.0),
                specific_heat: find("Pset_MaterialThermal", "SpecificHeatCapacity").unwrap_or(0.0),
            },
        );
        i.material_ids.insert(instance.id, id);
    }
}
//#endregion 🔖️Materials

//#region 🔖️Types
fn layers_of(i: &Import<'_>, type_ifc: u64, type_args: &[Part21Value]) -> Vec<Layer> {
    let Some(set) = i.doc.index.materials.get(&type_ifc).and_then(|set| i.doc.args(*set, "IFCMATERIALLAYERSET")) else { return Vec::new() };
    let functions = label(&type_authoring(&i.doc, type_args), "LayerFunctions").unwrap_or_default();
    let functions: Vec<&str> = functions.split(',').collect();
    refs(set, 0)
        .into_iter()
        .filter_map(|layer| i.doc.args(layer, "IFCMATERIALLAYER"))
        .enumerate()
        .map(|(index, layer)| Layer { material: layer[0].as_ref_id().and_then(|material| i.material_ids.get(&material)).cloned().unwrap_or_default(), thickness: real(layer, 1).unwrap_or(0.0), function: function(functions.get(index).copied().unwrap_or("Structure")) })
        .collect()
}

fn material_of(i: &Import<'_>, object: u64) -> String {
    i.doc.index.materials.get(&object).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default()
}

/// 🧱️ The profile an authoring set describes; `Custom` carries an empty outline until an occurrence supplies it.
fn profile_of(rows: &BTreeMap<String, Part21Value>) -> Profile {
    let value = |name: &str| number(rows, name).unwrap_or(0.0);
    match label(rows, "ProfileKind").as_deref() {
        Some("Circle") => Profile::Circle { diameter: value("Diameter") },
        Some("IShape") => Profile::IShape { width: value("Width"), depth: value("Depth"), web: value("Web"), flange: value("Flange") },
        Some("Custom") => Profile::Custom { outline: Vec::new() },
        _ => Profile::Rectangle { width: value("Width"), depth: value("Depth") },
    }
}

fn door_operation(operation: &str) -> (DoorLeaves, Swing) {
    match operation {
        "SINGLE_SWING_RIGHT" => (DoorLeaves::Single, Swing::Right),
        "DOUBLE_DOOR_SINGLE_SWING" | "DOUBLE_DOOR_DOUBLE_SWING" => (DoorLeaves::Double, Swing::Right),
        _ => (DoorLeaves::Single, Swing::Left),
    }
}

/// 🧱️ Reads every type object: layered wall, slab and roof types, profiled column and beam types, window and door styles.
pub fn read_types(i: &mut Import<'_>) {
    for (instance, args) in i.doc.rows("IFCWALLTYPE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("wt-{}", Import::slug(&text(args, 2))), |c| i.model.wall_types.contains_key(c)));
        let layers = layers_of(i, instance.id, args);
        i.model.wall_types.insert(id.clone(), WallType { name: text(args, 2), layers });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCSLABTYPE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("slt-{}", Import::slug(&text(args, 2))), |c| i.model.slab_types.contains_key(c)));
        let layers = layers_of(i, instance.id, args);
        i.model.slab_types.insert(id.clone(), SlabType { name: text(args, 2), layers });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCCOVERINGTYPE").into_iter().filter(|(_, args)| args.get(9).and_then(Part21Value::as_enum) == Some("CEILING")) {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("cet-{}", Import::slug(&text(args, 2))), |c| i.model.ceiling_types.contains_key(c)));
        let layers = layers_of(i, instance.id, args);
        i.model.ceiling_types.insert(id.clone(), CeilingType { name: text(args, 2), layers });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCBUILDINGELEMENTPROXYTYPE").into_iter().filter(|(_, args)| text(args, 8) == "RoofType") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("rt-{}", Import::slug(&text(args, 2))), |c| i.model.roof_types.contains_key(c)));
        let layers = layers_of(i, instance.id, args);
        i.model.roof_types.insert(id.clone(), RoofType { name: text(args, 2), layers });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCCOLUMNTYPE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("ct-{}", Import::slug(&text(args, 2))), |c| i.model.column_types.contains_key(c)));
        let (profile, material) = (profile_of(&type_authoring(&i.doc, args)), material_of(i, instance.id));
        i.model.column_types.insert(id.clone(), ColumnType { name: text(args, 2), profile, material });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCBEAMTYPE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("bt-{}", Import::slug(&text(args, 2))), |c| i.model.beam_types.contains_key(c)));
        let (profile, material) = (profile_of(&type_authoring(&i.doc, args)), material_of(i, instance.id));
        i.model.beam_types.insert(id.clone(), BeamType { name: text(args, 2), profile, material });
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCWINDOWSTYLE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("wnd-{}", Import::slug(&text(args, 2))), |c| i.model.window_types.contains_key(c)));
        let rows = type_authoring(&i.doc, args);
        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);
        i.model.window_types.insert(
            id.clone(),
            WindowType { name: text(args, 2), width: value("Width", 1.0), height: value("Height", 1.2), sill: value("Sill", 0.9), frame_width: value("FrameWidth", 0.05), frame_depth: value("FrameDepth", 0.08), panes: value("Panes", 1.0) as u32, material: label(&rows, "Material").unwrap_or_default() },
        );
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCDOORSTYLE") {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("dr-{}", Import::slug(&text(args, 2))), |c| i.model.door_types.contains_key(c)));
        let rows = type_authoring(&i.doc, args);
        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);
        let (leaves, swing) = door_operation(args.get(8).and_then(Part21Value::as_enum).unwrap_or_default());
        let leaves = match label(&rows, "Leaves").as_deref() {
            Some("Double") => DoorLeaves::Double,
            Some("Single") => DoorLeaves::Single,
            _ => leaves,
        };
        let swing = match label(&rows, "Swing").as_deref() {
            Some("Right") => Swing::Right,
            Some("Left") => Swing::Left,
            _ => swing,
        };
        i.model.door_types.insert(id.clone(), DoorType { name: text(args, 2), width: value("Width", 0.9), height: value("Height", 2.1), frame_width: value("FrameWidth", 0.05), frame_depth: value("FrameDepth", 0.1), leaves, swing, material: label(&rows, "Material").unwrap_or_default() });
        i.type_ids.insert(instance.id, id);
    }
}
//#endregion 🔖️Types

//#region 🔖️Attached
/// 🏷️ The model property of a typed IFC value; `None` for values that are neither numbers, texts nor flags.
pub fn property_value(value: &Part21Value) -> Option<PropertyValue> {
    let (name, items) = value.as_typed()?;
    let item = items.first()?;
    let number = |make: fn(f64) -> PropertyValue| item.as_real().map(make);
    let words = || item.as_str().map(|word| PropertyValue::Text { value: word.to_string() });
    match name.to_ascii_uppercase().as_str() {
        "IFCLABEL" | "IFCTEXT" | "IFCIDENTIFIER" | "IFCDESCRIPTIVEMEASURE" => words(),
        "IFCBOOLEAN" | "IFCLOGICAL" => Some(PropertyValue::Boolean { value: item.as_enum() == Some("T") }),
        "IFCINTEGER" | "IFCCOUNTMEASURE" => number(|value| PropertyValue::Integer { value: value.round() as i32 }),
        "IFCLENGTHMEASURE" | "IFCPOSITIVELENGTHMEASURE" | "IFCNONNEGATIVELENGTHMEASURE" => number(|value| PropertyValue::Length { value }),
        "IFCAREAMEASURE" => number(|value| PropertyValue::Area { value }),
        "IFCVOLUMEMEASURE" => number(|value| PropertyValue::Volume { value }),
        "IFCPLANEANGLEMEASURE" | "IFCPOSITIVEPLANEANGLEMEASURE" => number(|value| PropertyValue::Angle { value }),
        _ => number(|value| PropertyValue::Real { value }).or_else(words),
    }
}

/// 🏷️ Reads the user property sets and the classification of every imported element.
pub fn read_attached(i: &mut Import<'_>) {
    let imported: Vec<(u64, String)> = i.ids.iter().map(|(ifc, id)| (*ifc, id.clone())).collect();
    for (ifc, id) in imported {
        for definition in i.doc.index.definitions.get(&ifc).into_iter().flatten() {
            let Some(set) = i.doc.args(*definition, "IFCPROPERTYSET") else { continue };
            let name = text(set, 2);
            if name == "Semio_Authoring" || name == super::zoning::COVERING_SET {
                continue;
            }
            let rows: BTreeMap<String, PropertyValue> = single_values(&i.doc, set).into_iter().filter_map(|(property, value)| property_value(&value).map(|value| (property, value))).collect();
            if !rows.is_empty() {
                i.model.properties.entry(id.clone()).or_default().insert(name, rows);
            }
        }
        if let Some(reference) = i.doc.index.classifications.get(&ifc).and_then(|references| references.first()).and_then(|reference| i.doc.args(*reference, "IFCCLASSIFICATIONREFERENCE")) {
            let system = i.doc.follow_args(&reference[3], "IFCCLASSIFICATION").map(|source| text(source, 3)).unwrap_or_default();
            i.model.classifications.insert(id, Classification { system, code: text(reference, 1), title: text(reference, 2) });
        }
    }
}
//#endregion 🔖️Attached

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
