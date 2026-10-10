//! 🧬️ The non-geometric data of an IFC file: materials with their physical properties, layered and profiled types, window and door styles, user property sets and classifications.
//! Parameters IFC has no slot for are read from the `Semio_Authoring` set that the export writes.

use super::reader::{opt_text, real, refs, text, Doc};
use crate::standards::v1::subsets::any::io::export::ifc::data::{parent_key, PARENTS_SET};
use crate::standards::v1::subsets::any::io::export::ifc::Schema;
use super::spatial::{number_of, single_values, string_of};
use super::Import;
use crate::standards::v1::subsets::any::io::export::ifc::ifc4::{applicable_entity, measure_type};
use crate::{entries_problem, PropertyDef, PropertyKind, PropertyTemplate, TemplateTarget, BeamType, CeilingType, ClassificationItem, ClassificationSystem, ColumnType, DoorLeaves, DoorType, Layer, LayerFunction, Material, MaterialCategory, Profile, PropertyValue, Rgb, RoofType, SlabType, Swing, WallType, WindowType};
use crate::standards::v1::subsets::any::io::export::ifc::wall_sweeps::SWEEP_SET;
use crate::standards::v1::subsets::any::io::export::ifc::walls::{ATTACH_SET, REVEAL_SET};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

/// 🧗️ The property sets of the wall depth package: bookkeeping of the export, restored into fields and never user data.
const WALL_DEPTH_SETS: [&str; 3] = [ATTACH_SET, REVEAL_SET, SWEEP_SET];

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
    let found = doc.rows("IFCEXTENDEDMATERIALPROPERTIES").into_iter().map(|(_, args)| (&args[0], &args[1], text(args, 3))).chain(doc.rows("IFCMATERIALPROPERTIES").into_iter().map(|(_, args)| (&args[3], &args[2], text(args, 0))));
    for (owner, properties, set) in found {
        if owner.as_ref_id() == Some(material) {
            for (name, value) in properties.as_list().unwrap_or_default().iter().filter_map(|item| doc.follow_args(item, "IFCPROPERTYSINGLEVALUE")).map(|row| (text(row, 0), row[2].clone())) {
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
        let kind = rows.get(&("Semio_Authoring".to_string(), "Category".to_string())).and_then(string_of).or_else(|| opt_text(args, 2)).unwrap_or_default();
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
        .map(|(index, layer)| {
            let named = functions.get(index).copied().filter(|name| !name.is_empty()).map(str::to_string).or_else(|| opt_text(layer, 5)).unwrap_or_else(|| "Structure".to_string());
            Layer { material: layer[0].as_ref_id().and_then(|material| i.material_ids.get(&material)).cloned().unwrap_or_default(), thickness: real(layer, 1).unwrap_or(0.0), function: function(&named) }
        })
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
        Some("Family") => Profile::Family { family: label(rows, "Family").unwrap_or_default() },
        _ => Profile::Rectangle { width: value("Width"), depth: value("Depth") },
    }
}

/// 🪟️ The pane count of an `IfcWindowTypePartitioningEnum` literal (one for anything but a vertical split).
fn panes_of(partitioning: Option<&str>) -> f64 {
    match partitioning {
        Some("DOUBLE_PANEL_VERTICAL") => 2.0,
        Some("TRIPLE_PANEL_VERTICAL") => 3.0,
        _ => 1.0,
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
    for (instance, args) in i.doc.rows("IFCBUILDINGELEMENTPROXYTYPE").into_iter().filter(|(_, args)| text(args, 8) == "RoofType").chain(i.doc.rows("IFCROOFTYPE")) {
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
    for (instance, args) in i.doc.rows("IFCWINDOWSTYLE").into_iter().chain(i.doc.rows("IFCWINDOWTYPE")) {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("wnd-{}", Import::slug(&text(args, 2))), |c| i.model.window_types.contains_key(c)));
        let rows = type_authoring(&i.doc, args);
        let thermal = super::energy::type_thermal(&i.doc, args, true);
        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);
        i.model.window_types.insert(
            id.clone(),
            WindowType { name: text(args, 2), width: value("Width", 1.0), height: value("Height", 1.2), sill: value("Sill", 0.9), frame_width: value("FrameWidth", 0.05), frame_depth: value("FrameDepth", 0.08), panes: value("Panes", panes_of(args.get(10).and_then(Part21Value::as_enum).filter(|_| i.schema == Schema::Ifc4))) as u32, material: label(&rows, "Material").unwrap_or_default(), u_value: thermal.u_value, g_value: thermal.g_value, frame_fraction: thermal.frame_fraction },
        );
        i.type_ids.insert(instance.id, id);
    }
    for (instance, args) in i.doc.rows("IFCDOORSTYLE").into_iter().chain(i.doc.rows("IFCDOORTYPE")) {
        let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("dr-{}", Import::slug(&text(args, 2))), |c| i.model.door_types.contains_key(c)));
        let rows = type_authoring(&i.doc, args);
        let thermal = super::energy::type_thermal(&i.doc, args, false);
        let value = |name: &str, default: f64| number(&rows, name).unwrap_or(default);
        let (leaves, swing) = door_operation(args.get(if i.schema == Schema::Ifc4 { 10 } else { 8 }).and_then(Part21Value::as_enum).unwrap_or_default());
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
        i.model.door_types.insert(id.clone(), DoorType { name: text(args, 2), width: value("Width", 0.9), height: value("Height", 2.1), frame_width: value("FrameWidth", 0.05), frame_depth: value("FrameDepth", 0.1), leaves, swing, material: label(&rows, "Material").unwrap_or_default(), u_value: thermal.u_value });
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

fn user_rows(doc: &Doc<'_>, set: &[Part21Value]) -> BTreeMap<String, PropertyValue> {
    single_values(doc, set).into_iter().filter_map(|(property, value)| property_value(&value).map(|value| (property, value))).collect()
}

/// 🏷️ Reads the user property sets of every imported element and of every imported type object (the sets listed in its `HasPropertySets`); the sets the export writes for its own bookkeeping are not user data.
pub fn read_attached(i: &mut Import<'_>) {
    let imported: Vec<(u64, String)> = i.ids.iter().map(|(ifc, id)| (*ifc, id.clone())).collect();
    for (ifc, id) in imported {
        let derived = super::energy::derived_rows(&i.doc, ifc);
        for definition in i.doc.index.definitions.get(&ifc).into_iter().flatten() {
            let Some(set) = i.doc.args(*definition, "IFCPROPERTYSET") else { continue };
            let name = text(set, 2);
            if name == "Semio_Authoring" || name == super::zoning::COVERING_SET || name == PARENTS_SET || name == super::components::OVERRIDES_SET || WALL_DEPTH_SETS.contains(&name.as_str()) {
                continue;
            }
            let mut rows = user_rows(&i.doc, set);
            rows.retain(|property, _| !derived.contains(&format!("{name}.{property}")));
            if !rows.is_empty() {
                i.model.properties.entry(id.clone()).or_default().insert(name, rows);
            }
        }
    }
    let types: Vec<(u64, String)> = i.type_ids.iter().map(|(ifc, id)| (*ifc, id.clone())).collect();
    for (ifc, id) in types {
        let Some((_, args)) = i.doc.get(ifc).and_then(|instance| instance.primary()) else { continue };
        for set in refs(args, 5).into_iter().filter_map(|set| i.doc.args(set, "IFCPROPERTYSET")) {
            let name = text(set, 2);
            let rows = user_rows(&i.doc, set);
            if name != "Semio_Authoring" && !rows.is_empty() {
                i.model.properties.entry(id.clone()).or_default().insert(name, rows);
            }
        }
    }
    read_classifications(i);
}
//#endregion 🔖️Attached

//#region 🔖️Templates
fn unescape(value: &str) -> String {
    value.replace("%0A", "\n").replace("%0D", "\r").replace("%3D", "=").replace("%3B", ";").replace("%25", "%")
}

/// 🧾️ The `key=value;` list a definition travels in (see the IFC4 export): the keys with their unescaped values.
pub fn definition_rows(text: &str) -> BTreeMap<String, String> {
    text.split(';').filter_map(|part| part.split_once('=')).map(|(key, value)| (key.to_string(), unescape(value))).collect()
}

fn value_of(kind: PropertyKind, text: &str) -> Option<PropertyValue> {
    let number = || text.parse::<f64>().ok();
    Some(match kind {
        PropertyKind::Text => PropertyValue::Text { value: text.to_string() },
        PropertyKind::Real => PropertyValue::Real { value: number()? },
        PropertyKind::Integer => PropertyValue::Integer { value: text.parse().ok()? },
        PropertyKind::Boolean => PropertyValue::Boolean { value: text == "true" },
        PropertyKind::Length => PropertyValue::Length { value: number()? },
        PropertyKind::Area => PropertyValue::Area { value: number()? },
        PropertyKind::Volume => PropertyValue::Volume { value: number()? },
        PropertyKind::Angle => PropertyValue::Angle { value: number()? },
    })
}

fn definition_of(i: &Import<'_>, row: &[Part21Value]) -> PropertyDef {
    let kind = PropertyKind::ALL.into_iter().find(|kind| measure_type(*kind) == text(row, 5)).unwrap_or(PropertyKind::Text);
    let rows = definition_rows(&text(row, 3));
    let allowed = i.doc.follow_args(&row[7], "IFCPROPERTYENUMERATION").map(|set| set[1].as_list().unwrap_or_default().iter().filter_map(property_value).collect()).unwrap_or_default();
    PropertyDef {
        name: text(row, 2),
        kind,
        unit: rows.get("unit").cloned(),
        description: rows.get("description").cloned(),
        required: rows.get("required").is_some_and(|flag| flag == "true"),
        default_value: rows.get("default").and_then(|value| value_of(kind, value)),
        allowed,
        minimum: rows.get("minimum").and_then(|value| value.parse().ok()),
        maximum: rows.get("maximum").and_then(|value| value.parse().ok()),
    }
}

/// 📚️ Reads every `IfcPropertySetTemplate` of an IFC4 file as a property template: the id from its `Description` (else made from its name), the targets from its `ApplicableEntity`, one definition per `IfcSimplePropertyTemplate`
/// (measure type, enumerated values and the `key=value;` description list).
pub fn read_templates(i: &mut Import<'_>) {
    for (_, args) in i.doc.rows("IFCPROPERTYSETTEMPLATE") {
        let name = text(args, 2);
        let properties: Vec<PropertyDef> = args[6].as_list().unwrap_or_default().iter().filter_map(|item| i.doc.follow_args(item, "IFCSIMPLEPROPERTYTEMPLATE")).map(|row| definition_of(i, row)).collect();
        let applies_to: Vec<TemplateTarget> = text(args, 5).split(',').filter_map(|entity| TemplateTarget::ALL.into_iter().find(|target| applicable_entity(*target) == entity)).collect();
        let id = opt_text(args, 3).filter(|id| !i.model.property_templates.contains_key(id)).unwrap_or_else(|| Import::unused(&format!("pt-{}", Import::slug(&name)), |candidate| i.model.property_templates.contains_key(candidate)));
        i.model.property_templates.insert(id, PropertyTemplate { name, applies_to, properties });
    }
}
//#endregion 🔖️Templates

//#region 🔖️Classifications
fn parent_rows(i: &Import<'_>) -> BTreeMap<String, String> {
    let mut rows = BTreeMap::new();
    for (project, _) in i.doc.rows("IFCPROJECT") {
        for definition in i.doc.index.definitions.get(&project.id).into_iter().flatten() {
            let Some(set) = i.doc.args(*definition, "IFCPROPERTYSET").filter(|set| text(set, 2) == PARENTS_SET) else { continue };
            rows.extend(single_values(&i.doc, set).into_iter().filter_map(|(key, value)| string_of(&value).map(|parent| (key, parent))));
        }
    }
    rows
}

/// 🗂️ Reads every `IfcClassification` as a classification system (id `cs-<slug of its name>`, made unique), every `IfcClassificationReference` as a row of the table of its system (attached or not, in file order, the parent from the
/// `Semio_ClassificationParents` set of the project when the file has one) and every `IfcRelAssociatesClassification` as the code its elements and types carry in that system. A reference without a classification or an item
/// reference is skipped with a note; a holder that names two codes of one system keeps the first; a parent column that is no forest is dropped with a note.
pub fn read_classifications(i: &mut Import<'_>) {
    let parents = parent_rows(i);
    let mut systems: BTreeMap<u64, String> = BTreeMap::new();
    for (instance, args) in i.doc.rows("IFCCLASSIFICATION") {
        let name = text(args, 3);
        let id = opt_text(args, 4).filter(|id| i.schema == Schema::Ifc4 && !i.model.classification_systems.contains_key(id)).unwrap_or_else(|| Import::unused(&format!("cs-{}", Import::slug(&name)), |candidate| i.model.classification_systems.contains_key(candidate)));
        i.model.classification_systems.insert(id.clone(), ClassificationSystem { name, edition: text(args, 1), source: opt_text(args, 0), entries: Vec::new() });
        systems.insert(instance.id, id);
    }
    let mut codes: BTreeMap<u64, (String, String)> = BTreeMap::new();
    let mut references = i.doc.rows("IFCCLASSIFICATIONREFERENCE");
    if i.schema == Schema::Ifc4 {
        references.sort_by_key(|(instance, args)| (text(args, 5), instance.id));
    }
    for (instance, args) in references {
        let code = text(args, 1);
        let (mut at, mut chained_parent) = (args.get(3).and_then(Part21Value::as_ref_id), None);
        let mut system = None;
        for hop in 0..256 {
            let Some(id) = at else { break };
            if let Some(found) = systems.get(&id) {
                system = Some(found.clone());
                break;
            }
            let Some(above) = i.doc.args(id, "IFCCLASSIFICATIONREFERENCE") else { break };
            if hop == 0 {
                chained_parent = opt_text(above, 1);
            }
            at = above.get(3).and_then(Part21Value::as_ref_id);
        }
        let Some(system) = system else {
            i.skip("IFCCLASSIFICATIONREFERENCE", &code, "it names no IFCCLASSIFICATION it belongs to");
            continue;
        };
        if code.is_empty() {
            i.skip("IFCCLASSIFICATIONREFERENCE", &text(args, 2), "it has no item reference");
            continue;
        }
        codes.insert(instance.id, (system.clone(), code.clone()));
        let Some(table) = i.model.classification_systems.get_mut(&system) else { continue };
        if table.entries.iter().all(|entry| entry.code != code) {
            let parent = chained_parent.or_else(|| parents.get(&parent_key(table, &code)).cloned());
            table.entries.push(ClassificationItem { code, title: text(args, 2), parent });
        }
    }
    let mut dropped = Vec::new();
    for (id, system) in i.model.classification_systems.iter_mut() {
        if let Some(problem) = entries_problem(&system.entries) {
            system.entries.iter_mut().for_each(|entry| entry.parent = None);
            dropped.push((id.clone(), problem));
        }
    }
    for (id, problem) in dropped {
        i.skip("classification", &id, &format!("the parent column is dropped: {problem}"));
    }
    let attached: Vec<(u64, Vec<u64>)> = i.doc.index.classifications.iter().map(|(object, references)| (*object, references.clone())).collect();
    for (object, references) in attached {
        let Some(holder) = i.ids.get(&object).or_else(|| i.type_ids.get(&object)).cloned() else { continue };
        for reference in references {
            let Some((system, code)) = codes.get(&reference).cloned() else {
                if i.doc.args(reference, "IFCCLASSIFICATIONREFERENCE").is_none() {
                    i.skip("IFCRELASSOCIATESCLASSIFICATION", &holder, "it relates the holder to something that is not a classification reference");
                }
                continue;
            };
            let kept = i.model.classifications.entry(holder.clone()).or_default().entry(system.clone()).or_insert_with(|| code.clone()).clone();
            if kept != code {
                i.skip("classification", &holder, &format!("the system {system} already has the code {kept}; {code} is ignored"));
            }
        }
    }
}
//#endregion 🔖️Classifications

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
