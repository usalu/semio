//! 🧬️ The non-geometric data of the IFC file: materials with their physical properties, layered and profiled types, property sets, classifications,
//! base quantities and the relationship entities (aggregation, containment, typing, material association) the families recorded.

use super::writer::{en, flag, int, opt_text, real, refs, rf, text, typed, unset, V};
use super::{Export, Quantity};
use crate::{Layer, Profile, PropertyValue};
use std::collections::BTreeMap;

//#region 🔖️Properties
/// 🏷️ `IfcPropertySingleValue` rows gathered into one `IfcPropertySet`; `key` makes the set's GlobalId stable.
pub fn property_set(x: &mut Export<'_>, key: &str, name: &str, rows: Vec<(&str, V)>) -> u64 {
    let ids: Vec<u64> = rows.into_iter().map(|(property, value)| x.ifc.add("IFCPROPERTYSINGLEVALUE", vec![text(property), unset(), value, unset()])).collect();
    x.ifc.rooted("IFCPROPERTYSET", key, name, "", vec![refs(&ids)])
}

/// 🔗️ `IfcRelDefinesByProperties` of `set` on `elements`.
pub fn define(x: &mut Export<'_>, key: &str, elements: &[u64], set: u64) {
    x.ifc.rooted("IFCRELDEFINESBYPROPERTIES", key, "", "", vec![refs(elements), rf(set)]);
}

/// 🏷️ A typed IFC value of a model property.
pub fn property_value(value: &PropertyValue) -> V {
    match value {
        PropertyValue::Text { value } => typed("IFCLABEL", text(value.clone())),
        PropertyValue::Real { value } => typed("IFCREAL", real(*value)),
        PropertyValue::Integer { value } => typed("IFCINTEGER", int(i64::from(*value))),
        PropertyValue::Boolean { value } => typed("IFCBOOLEAN", flag(*value)),
        PropertyValue::Length { value } => typed("IFCLENGTHMEASURE", real(*value)),
        PropertyValue::Area { value } => typed("IFCAREAMEASURE", real(*value)),
        PropertyValue::Volume { value } => typed("IFCVOLUMEMEASURE", real(*value)),
        PropertyValue::Angle { value } => typed("IFCPLANEANGLEMEASURE", real(*value)),
    }
}

/// 🏷️ `IFCLABEL(value)`.
pub fn label(value: &str) -> V {
    typed("IFCLABEL", text(value))
}

/// 🔢️ `IFCREAL(value)`.
pub fn number(value: f64) -> V {
    typed("IFCREAL", real(value))
}
//#endregion 🔖️Properties

//#region 🔖️Types
fn extended(x: &mut Export<'_>, material: u64, name: &str, rows: Vec<(&str, V)>) {
    let ids: Vec<u64> = rows.into_iter().map(|(property, value)| x.ifc.add("IFCPROPERTYSINGLEVALUE", vec![text(property), unset(), value, unset()])).collect();
    x.ifc.add("IFCEXTENDEDMATERIALPROPERTIES", vec![rf(material), refs(&ids), unset(), text(name)]);
}

fn layer_set(x: &mut Export<'_>, kind: &'static str, id: &str, name: &str, layers: &[Layer]) -> Option<u64> {
    if layers.is_empty() {
        return None;
    }
    let ids: Vec<u64> = layers
        .iter()
        .map(|layer| {
            let material = x.links.material_defs.get(&layer.material).map_or(unset(), |material| rf(*material));
            x.ifc.add("IFCMATERIALLAYER", vec![material, real(layer.thickness), unset()])
        })
        .collect();
    let set = x.ifc.add("IFCMATERIALLAYERSET", vec![refs(&ids), opt_text(name)]);
    x.links.layer_sets.insert((kind, id.to_string()), set);
    Some(set)
}

fn layered_type(x: &mut Export<'_>, kind: &'static str, entity: &str, id: &str, name: &str, layers: &[Layer], element_type: Option<&str>, predefined: &str) {
    let functions = layers.iter().map(|layer| format!("{:?}", layer.function)).collect::<Vec<_>>().join(",");
    let authoring = property_set(x, &format!("{kind}:{id}:authoring"), "Semio_Authoring", vec![("LayerFunctions", label(&functions))]);
    let object = x.ifc.rooted(entity, id, name, "", vec![unset(), refs(&[authoring]), unset(), opt_text(id), element_type.map_or(unset(), text), en(predefined)]);
    x.links.types.insert((kind, id.to_string()), object);
    if let Some(set) = layer_set(x, kind, id, name, layers) {
        x.links.materials.entry(set).or_default().push(object);
    }
}

fn profile_rows(profile: &Profile) -> Vec<(&'static str, V)> {
    match profile {
        Profile::Rectangle { width, depth } => vec![("ProfileKind", label("Rectangle")), ("Width", number(*width)), ("Depth", number(*depth))],
        Profile::Circle { diameter } => vec![("ProfileKind", label("Circle")), ("Diameter", number(*diameter))],
        Profile::IShape { width, depth, web, flange } => vec![("ProfileKind", label("IShape")), ("Width", number(*width)), ("Depth", number(*depth)), ("Web", number(*web)), ("Flange", number(*flange))],
        Profile::Custom { .. } => vec![("ProfileKind", label("Custom"))],
        Profile::Family { family } => vec![("ProfileKind", label("Family")), ("Family", label(family))],
    }
}

fn profiled_type(x: &mut Export<'_>, kind: &'static str, entity: &str, id: &str, name: &str, profile: &Profile, material: &str, predefined: &str) {
    let authoring = property_set(x, &format!("{kind}:{id}:authoring"), "Semio_Authoring", profile_rows(profile));
    let object = x.ifc.rooted(entity, id, name, "", vec![unset(), refs(&[authoring]), unset(), opt_text(id), unset(), en(predefined)]);
    x.links.types.insert((kind, id.to_string()), object);
    if let Some(material) = x.links.material_defs.get(material).copied() {
        x.links.materials.entry(material).or_default().push(object);
    }
}

/// 🧱️ Writes materials, layer sets and every type entity before the products that refer to them.
pub fn emit_types(x: &mut Export<'_>) {
    let model = x.model;
    for (id, material) in &model.materials {
        let definition = x.ifc.add("IFCMATERIAL", vec![text(&material.name)]);
        x.links.material_defs.insert(id.clone(), definition);
        extended(x, definition, "Pset_MaterialCommon", vec![("MassDensity", typed("IFCMASSDENSITYMEASURE", real(material.density)))]);
        extended(x, definition, "Pset_MaterialThermal", vec![("ThermalConductivity", typed("IFCTHERMALCONDUCTIVITYMEASURE", real(material.conductivity))), ("SpecificHeatCapacity", typed("IFCSPECIFICHEATCAPACITYMEASURE", real(material.specific_heat)))]);
        extended(x, definition, "Semio_Authoring", vec![("Id", label(id)), ("Category", label(&format!("{:?}", material.category))), ("Red", number(material.color.r)), ("Green", number(material.color.g)), ("Blue", number(material.color.b))]);
    }
    for (id, kind) in &model.wall_types {
        layered_type(x, "wall", "IFCWALLTYPE", id, &kind.name, &kind.layers, None, "STANDARD");
    }
    for (id, kind) in &model.slab_types {
        layered_type(x, "slab", "IFCSLABTYPE", id, &kind.name, &kind.layers, None, "FLOOR");
    }
    for (id, kind) in &model.ceiling_types {
        layered_type(x, "ceiling", "IFCCOVERINGTYPE", id, &kind.name, &kind.layers, None, "CEILING");
    }
    for (id, kind) in &model.roof_types {
        layered_type(x, "roof", "IFCBUILDINGELEMENTPROXYTYPE", id, &kind.name, &kind.layers, Some("RoofType"), "USERDEFINED");
    }
    for (id, kind) in &model.column_types {
        profiled_type(x, "column", "IFCCOLUMNTYPE", id, &kind.name, &kind.profile, &kind.material, "COLUMN");
    }
    for (id, kind) in &model.beam_types {
        profiled_type(x, "beam", "IFCBEAMTYPE", id, &kind.name, &kind.profile, &kind.material, "BEAM");
    }
    for (id, kind) in &model.window_types {
        let material = model.materials.get(&kind.material).map_or(kind.material.clone(), |row| row.name.clone());
        let rows = vec![("Width", number(kind.width)), ("Height", number(kind.height)), ("Sill", number(kind.sill)), ("FrameWidth", number(kind.frame_width)), ("FrameDepth", number(kind.frame_depth)), ("Panes", typed("IFCINTEGER", int(i64::from(kind.panes)))), ("Material", label(&kind.material)), ("MaterialName", label(&material))];
        let authoring = property_set(x, &format!("window:{id}:authoring"), "Semio_Authoring", rows);
        let object = x.ifc.rooted("IFCWINDOWSTYLE", id, &kind.name, "", vec![unset(), refs(&[authoring]), unset(), opt_text(id), en("NOTDEFINED"), en("NOTDEFINED"), flag(false), flag(true)]);
        x.links.types.insert(("window", id.clone()), object);
    }
    for (id, kind) in &model.door_types {
        let operation = match (kind.leaves, kind.swing) {
            (crate::DoorLeaves::Single, crate::Swing::Left) => "SINGLE_SWING_LEFT",
            (crate::DoorLeaves::Single, crate::Swing::Right) => "SINGLE_SWING_RIGHT",
            (crate::DoorLeaves::Double, _) => "DOUBLE_DOOR_SINGLE_SWING",
        };
        let rows = vec![
            ("Width", number(kind.width)),
            ("Height", number(kind.height)),
            ("FrameWidth", number(kind.frame_width)),
            ("FrameDepth", number(kind.frame_depth)),
            ("Leaves", label(&format!("{:?}", kind.leaves))),
            ("Swing", label(&format!("{:?}", kind.swing))),
            ("Material", label(&kind.material)),
        ];
        let authoring = property_set(x, &format!("door:{id}:authoring"), "Semio_Authoring", rows);
        let object = x.ifc.rooted("IFCDOORSTYLE", id, &kind.name, "", vec![unset(), refs(&[authoring]), unset(), opt_text(id), en(operation), en("NOTDEFINED"), flag(false), flag(true)]);
        x.links.types.insert(("door", id.clone()), object);
    }
}
//#endregion 🔖️Types

//#region 🔖️Links
fn quantity(x: &mut Export<'_>, quantity: &Quantity) -> u64 {
    match quantity {
        Quantity::Length(name, value) => x.ifc.add("IFCQUANTITYLENGTH", vec![text(name), unset(), unset(), real(*value)]),
        Quantity::Area(name, value) => x.ifc.add("IFCQUANTITYAREA", vec![text(name), unset(), unset(), real(*value)]),
        Quantity::Volume(name, value) => x.ifc.add("IFCQUANTITYVOLUME", vec![text(name), unset(), unset(), real(*value)]),
        Quantity::Count(name, value) => x.ifc.add("IFCQUANTITYCOUNT", vec![text(name), unset(), unset(), real(*value as f64)]),
    }
}

/// 🔗️ Emits every relationship and attached data set the families recorded: aggregation, containment, typing, materials, quantities, authoring data, user properties and classifications.
pub fn emit_links(x: &mut Export<'_>) {
    let model = x.model;
    let mut ids: BTreeMap<u64, String> = x.links.elements.iter().map(|(id, entity)| (*entity, id.clone())).collect();
    ids.extend(x.links.types.iter().map(|((kind, id), entity)| (*entity, format!("{kind}-type:{id}"))));
    let key = |entity: u64| ids.get(&entity).cloned().unwrap_or_else(|| format!("#{entity}"));
    for (whole, parts) in std::mem::take(&mut x.links.aggregated) {
        x.ifc.aggregate(&key(whole), whole, &parts);
    }
    for (storey, products) in std::mem::take(&mut x.links.contained) {
        if let Some(reference) = x.storeys.get(&storey).copied() {
            x.ifc.rooted("IFCRELCONTAINEDINSPATIALSTRUCTURE", &storey, "", "", vec![refs(&products), rf(reference.ifc)]);
        }
    }
    for (building, products) in std::mem::take(&mut x.links.in_building) {
        if let Some(reference) = x.buildings.get(&building).copied() {
            x.ifc.rooted("IFCRELCONTAINEDINSPATIALSTRUCTURE", &building, "", "", vec![refs(&products), rf(reference.ifc)]);
        }
    }
    for (object, occurrences) in std::mem::take(&mut x.links.typed) {
        x.ifc.rooted("IFCRELDEFINESBYTYPE", &key(object), "", "", vec![refs(&occurrences), rf(object)]);
    }
    for (definition, elements) in std::mem::take(&mut x.links.materials) {
        x.ifc.rooted("IFCRELASSOCIATESMATERIAL", &key(elements[0]), "", "", vec![refs(&elements), rf(definition)]);
    }
    for (entity, name, rows) in std::mem::take(&mut x.links.quantities) {
        let items: Vec<u64> = rows.iter().map(|row| quantity(x, row)).collect();
        let set = x.ifc.rooted("IFCELEMENTQUANTITY", &format!("{}:{name}", key(entity)), name, "", vec![unset(), refs(&items)]);
        define(x, &format!("{}:{name}", key(entity)), &[entity], set);
    }
    let mut authored: BTreeMap<u64, Vec<(&'static str, V)>> = BTreeMap::new();
    for (entity, rows) in std::mem::take(&mut x.links.authoring) {
        authored.entry(entity).or_default().extend(rows);
    }
    for (entity, rows) in authored {
        let set = property_set(x, &format!("{}:authoring", key(entity)), "Semio_Authoring", rows);
        define(x, &format!("{}:authoring", key(entity)), &[entity], set);
    }
    for (element, sets) in &model.properties {
        let Some(entity) = x.links.elements.get(element).copied() else {
            x.skip("properties", element, "the element is not part of the export");
            continue;
        };
        for (name, rows) in sets {
            let rows: Vec<(&str, V)> = rows.iter().map(|(property, value)| (property.as_str(), property_value(value))).collect();
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
    let mut systems: BTreeMap<&str, u64> = BTreeMap::new();
    let mut grouped: BTreeMap<(&str, &str, &str), Vec<u64>> = BTreeMap::new();
    for (element, class) in &model.classifications {
        match x.links.elements.get(element).copied() {
            Some(entity) => grouped.entry((class.system.as_str(), class.code.as_str(), class.title.as_str())).or_default().push(entity),
            None => x.skip("classification", element, "the element is not part of the export"),
        }
    }
    for ((system, code, title), elements) in grouped {
        let source = match systems.get(system) {
            Some(source) => *source,
            None => {
                let source = x.ifc.add("IFCCLASSIFICATION", vec![text(system), text(""), unset(), text(system)]);
                systems.insert(system, source);
                source
            }
        };
        let reference = x.ifc.add("IFCCLASSIFICATIONREFERENCE", vec![unset(), opt_text(code), opt_text(title), rf(source)]);
        x.ifc.rooted("IFCRELASSOCIATESCLASSIFICATION", &format!("{system}:{code}:{title}"), "", "", vec![refs(&elements), rf(reference)]);
    }
}
//#endregion 🔖️Links

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
