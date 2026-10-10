//! 🧬️ The non-geometric data of the IFC file (IFC 2x3 and IFC4): materials with their physical properties, layered and profiled types (with their authored property sets in `HasPropertySets`), property sets, classification systems with their
//! tables, base quantities and the relationship entities (aggregation, containment, typing, material association, classification association) the families recorded.

use super::writer::{en, flag, int, opt_text, real, refs, rf, text, typed, unset, Schema, V};
use super::{Export, Quantity};
use crate::{ClassificationSystem, Layer, Profile, PropertyValue};
use std::collections::BTreeMap;

/// 🏷️ The property set of the project that records the parent column of the classification tables.
pub const PARENTS_SET: &str = "Semio_ClassificationParents";

//#region 🔖️Properties
/// 🏷️ `IfcPropertySingleValue` rows gathered into one `IfcPropertySet`; `key` makes the set's GlobalId stable.
pub fn property_set(x: &mut Export<'_>, key: &str, name: &str, rows: Vec<(&str, V)>) -> u64 {
    let ids: Vec<u64> = rows.into_iter().map(|(property, value)| x.ifc.add("IFCPROPERTYSINGLEVALUE", vec![text(property), unset(), value, unset()])).collect();
    let set = x.ifc.rooted("IFCPROPERTYSET", key, name, "", vec![refs(&ids)]);
    if let Some(template) = x.links.templates.get(name).copied() {
        x.links.templated.entry(template).or_default().push(set);
    }
    set
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
/// 🏷️ The authored property sets of type `id` as `IfcPropertySet`s for the `HasPropertySets` of its type object, after the `authoring` set; defaults and values inherited by instances are inferred and never written.
fn type_sets(x: &mut Export<'_>, id: &str, authoring: u64) -> V {
    let model = x.model;
    let mut sets = vec![authoring];
    for (name, rows) in model.properties.get(id).into_iter().flatten() {
        let rows: Vec<(&str, V)> = rows.iter().map(|(property, value)| (property.as_str(), property_value(value))).collect();
        sets.push(property_set(x, &format!("{id}:{name}"), name, rows));
    }
    refs(&sets)
}

fn extended(x: &mut Export<'_>, material: u64, name: &str, rows: Vec<(&str, V)>) {
    let ids: Vec<u64> = rows.into_iter().map(|(property, value)| x.ifc.add("IFCPROPERTYSINGLEVALUE", vec![text(property), unset(), value, unset()])).collect();
    match x.schema() {
        Schema::Ifc2x3 => x.ifc.add("IFCEXTENDEDMATERIALPROPERTIES", vec![rf(material), refs(&ids), unset(), text(name)]),
        Schema::Ifc4 => x.ifc.add("IFCMATERIALPROPERTIES", vec![text(name), unset(), refs(&ids), rf(material)]),
    };
}

fn layer_set(x: &mut Export<'_>, kind: &'static str, id: &str, name: &str, layers: &[Layer]) -> Option<u64> {
    if layers.is_empty() {
        return None;
    }
    let ids: Vec<u64> = layers
        .iter()
        .map(|layer| {
            let material = x.links.material_defs.get(&layer.material).map_or(unset(), |material| rf(*material));
            let tail = x.by(Vec::new(), vec![unset(), unset(), text(format!("{:?}", layer.function)), unset()]);
            let mut args = vec![material, real(layer.thickness), unset()];
            args.extend(tail);
            x.ifc.add("IFCMATERIALLAYER", args)
        })
        .collect();
    let set = x.ifc.add("IFCMATERIALLAYERSET", x.by(vec![refs(&ids), opt_text(name)], vec![refs(&ids), opt_text(name), unset()]));
    x.links.layer_sets.insert((kind, id.to_string()), set);
    Some(set)
}

fn layered_type(x: &mut Export<'_>, kind: &'static str, entity: &str, id: &str, name: &str, layers: &[Layer], element_type: Option<&str>, predefined: &str) {
    let functions = layers.iter().map(|layer| format!("{:?}", layer.function)).collect::<Vec<_>>().join(",");
    let authoring = property_set(x, &format!("{kind}:{id}:authoring"), "Semio_Authoring", vec![("LayerFunctions", label(&functions))]);
    let sets = type_sets(x, id, authoring);
    let (entity, element_type, predefined) = if kind == "roof" && x.schema() == Schema::Ifc4 { ("IFCROOFTYPE", None, "NOTDEFINED") } else { (entity, element_type, predefined) };
    let object = x.ifc.rooted(entity, id, name, "", vec![unset(), sets, unset(), opt_text(id), element_type.map_or(unset(), text), en(predefined)]);
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
    let sets = type_sets(x, id, authoring);
    let object = x.ifc.rooted(entity, id, name, "", vec![unset(), sets, unset(), opt_text(id), unset(), en(predefined)]);
    x.links.types.insert((kind, id.to_string()), object);
    if let Some(material) = x.links.material_defs.get(material).copied() {
        x.links.materials.entry(material).or_default().push(object);
    }
}

/// 🚪️ The `IfcDoorTypeOperationEnum` literal of a door type: leaves and swing.
pub fn door_operation(kind: &crate::DoorType) -> &'static str {
    match (kind.leaves, kind.swing) {
        (crate::DoorLeaves::Single, crate::Swing::Left) => "SINGLE_SWING_LEFT",
        (crate::DoorLeaves::Single, crate::Swing::Right) => "SINGLE_SWING_RIGHT",
        (crate::DoorLeaves::Double, _) => "DOUBLE_DOOR_SINGLE_SWING",
    }
}

/// 🪟️ The `IfcWindowTypePartitioningEnum` literal of a window with `panes` panes.
pub fn window_partitioning(panes: u32) -> &'static str {
    match panes {
        1 => "SINGLE_PANEL",
        2 => "DOUBLE_PANEL_VERTICAL",
        3 => "TRIPLE_PANEL_VERTICAL",
        _ => "NOTDEFINED",
    }
}

/// 🧱️ Writes materials, layer sets and every type entity before the products that refer to them.
pub fn emit_types(x: &mut Export<'_>) {
    let model = x.model;
    for (id, material) in &model.materials {
        let definition = x.ifc.add("IFCMATERIAL", x.by(vec![text(&material.name)], vec![text(&material.name), unset(), text(format!("{:?}", material.category))]));
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
        let mut rows = vec![("Width", number(kind.width)), ("Height", number(kind.height)), ("Sill", number(kind.sill)), ("FrameWidth", number(kind.frame_width)), ("FrameDepth", number(kind.frame_depth)), ("Panes", typed("IFCINTEGER", int(i64::from(kind.panes)))), ("Material", label(&kind.material)), ("MaterialName", label(&material))];
        rows.extend(kind.u_value.map(|value| ("UValue", number(value))));
        rows.extend(kind.g_value.map(|value| ("GValue", number(value))));
        rows.extend(kind.frame_fraction.map(|value| ("FrameFraction", number(value))));
        let authoring = property_set(x, &format!("window:{id}:authoring"), "Semio_Authoring", rows);
        let sets = type_sets(x, id, authoring);
        let tail = match x.schema() {
            Schema::Ifc2x3 => vec![en("NOTDEFINED"), en("NOTDEFINED"), flag(false), flag(true)],
            Schema::Ifc4 => vec![unset(), en("WINDOW"), en(window_partitioning(kind.panes)), flag(false), unset()],
        };
        let mut args = vec![unset(), sets, unset(), opt_text(id)];
        args.extend(tail);
        let object = x.ifc.rooted(x.by("IFCWINDOWSTYLE", "IFCWINDOWTYPE"), id, &kind.name, "", args);
        x.links.types.insert(("window", id.clone()), object);
    }
    for (id, kind) in &model.door_types {
        let operation = door_operation(kind);
        let mut rows = vec![
            ("Width", number(kind.width)),
            ("Height", number(kind.height)),
            ("FrameWidth", number(kind.frame_width)),
            ("FrameDepth", number(kind.frame_depth)),
            ("Leaves", label(&format!("{:?}", kind.leaves))),
            ("Swing", label(&format!("{:?}", kind.swing))),
            ("Material", label(&kind.material)),
        ];
        rows.extend(kind.u_value.map(|value| ("UValue", number(value))));
        let authoring = property_set(x, &format!("door:{id}:authoring"), "Semio_Authoring", rows);
        let sets = type_sets(x, id, authoring);
        let tail = match x.schema() {
            Schema::Ifc2x3 => vec![en(operation), en("NOTDEFINED"), flag(false), flag(true)],
            Schema::Ifc4 => vec![unset(), en("DOOR"), en(operation), flag(false), unset()],
        };
        let mut args = vec![unset(), sets, unset(), opt_text(id)];
        args.extend(tail);
        let object = x.ifc.rooted(x.by("IFCDOORSTYLE", "IFCDOORTYPE"), id, &kind.name, "", args);
        x.links.types.insert(("door", id.clone()), object);
    }
}
//#endregion 🔖️Types

//#region 🔖️Links
fn quantity(x: &mut Export<'_>, quantity: &Quantity) -> u64 {
    let (entity, name, value) = match quantity {
        Quantity::Length(name, value) => ("IFCQUANTITYLENGTH", name, *value),
        Quantity::Area(name, value) => ("IFCQUANTITYAREA", name, *value),
        Quantity::Volume(name, value) => ("IFCQUANTITYVOLUME", name, *value),
        Quantity::Count(name, value) => ("IFCQUANTITYCOUNT", name, *value as f64),
    };
    let mut args = vec![text(name), unset(), unset(), real(value)];
    args.extend(x.by(Vec::new(), vec![unset()]));
    x.ifc.add(entity, args)
}

/// 🔗️ Emits every relationship and attached data set the families recorded: aggregation, containment, typing, materials, quantities, authoring data, user properties and classifications.
pub fn emit_links(x: &mut Export<'_>) {
    let model = x.model;
    if let (Some(library), Some(project)) = (x.links.library, x.links.elements.get(":project").copied()) {
        x.ifc.rooted("IFCRELDECLARES", "project:library", "", "", vec![rf(project), refs(&[library])]);
    }
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
    let mut derived = std::mem::take(&mut x.links.derived);
    for (element, sets) in &model.properties {
        let Some(entity) = x.links.elements.get(element).copied() else {
            if !x.links.types.keys().any(|(_, id)| id == element) {
                x.skip("properties", element, "the element is not part of the export");
            }
            continue;
        };
        for (name, rows) in sets {
            let mut rows: Vec<(&str, V)> = rows.iter().map(|(property, value)| (property.as_str(), property_value(value))).collect();
            if let Some(extra) = derived.get_mut(element) {
                for (_, more) in extra.iter_mut().filter(|(set_name, _)| *set_name == name.as_str()) {
                    rows.extend(std::mem::take(more));
                }
            }
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
    for (element, sets) in derived {
        let Some(entity) = x.links.elements.get(&element).copied() else { continue };
        for (name, rows) in sets.into_iter().filter(|(_, rows)| !rows.is_empty()) {
            let set = property_set(x, &format!("{element}:{name}"), name, rows);
            define(x, &format!("{element}:{name}"), &[entity], set);
        }
    }
    for (template, sets) in std::mem::take(&mut x.links.templated) {
        x.ifc.rooted("IFCRELDEFINESBYTEMPLATE", &format!("template:{template}"), "", "", vec![refs(&sets), rf(template)]);
    }
    emit_classifications(x);
}
//#endregion 🔖️Links

//#region 🔖️Classifications
/// 🔑️ The name under which `Semio_ClassificationParents` records the parent of the entry `code` of `system`: IFC 2x3 has no slot for a parent, so the table's parent column travels in the property set of the project.
pub fn parent_key(system: &ClassificationSystem, code: &str) -> String {
    format!("{}|{}|{code}", system.name, system.edition)
}

/// 🗂️ Writes every classification system of the library as an `IfcClassification` with one `IfcClassificationReference` per table row (attached or not, in table order), then one `IfcRelAssociatesClassification` per used (system, code)
/// listing every element and type that carries it. The parent column of the tables goes to the `Semio_ClassificationParents` set of the project. A holder outside the export, a system outside the library and a code outside its table are noted.
pub fn emit_classifications(x: &mut Export<'_>) {
    let model = x.model;
    let v4 = x.schema() == Schema::Ifc4;
    let mut sources: BTreeMap<String, u64> = BTreeMap::new();
    let mut references: BTreeMap<(String, String), u64> = BTreeMap::new();
    let mut parents: Vec<(String, String)> = Vec::new();
    for (system_id, system) in model.classification_systems.iter().filter(|_| !v4) {
        let source = x.ifc.add("IFCCLASSIFICATION", vec![text(system.source.as_deref().unwrap_or("")), text(&system.edition), unset(), text(&system.name)]);
        sources.insert(system_id.clone(), source);
        for entry in &system.entries {
            if references.contains_key(&(system_id.clone(), entry.code.clone())) {
                continue;
            }
            let reference = x.ifc.add("IFCCLASSIFICATIONREFERENCE", vec![unset(), opt_text(&entry.code), opt_text(&entry.title), rf(source)]);
            references.insert((system_id.clone(), entry.code.clone()), reference);
            if let Some(parent) = &entry.parent {
                parents.push((parent_key(system, &entry.code), parent.clone()));
            }
        }
    }
    if v4 {
        references.extend(x.links.references.iter().map(|(key, reference)| (key.clone(), *reference)));
        sources.extend(x.links.sources.iter().map(|(system_id, source)| (system_id.clone(), *source)));
    }
    let mut holders: BTreeMap<String, u64> = x.links.elements.iter().map(|(id, entity)| (id.clone(), *entity)).collect();
    holders.extend(x.links.types.iter().map(|((_, id), entity)| (id.clone(), *entity)));
    let mut attached: BTreeMap<u64, (String, Vec<u64>)> = BTreeMap::new();
    for (holder, set) in &model.classifications {
        let Some(entity) = holders.get(holder).copied() else {
            x.skip("classification", holder, "the holder is not part of the export");
            continue;
        };
        for (system_id, code) in set {
            let Some(system) = model.classification_systems.get(system_id) else {
                x.skip("classification", holder, &format!("the system {system_id} is not in the library"));
                continue;
            };
            let reference = match references.get(&(system_id.clone(), code.clone())).copied() {
                Some(reference) => reference,
                None => {
                    x.skip("classification", holder, &format!("the code {code} is no entry of {}", system.name));
                    let mut args = vec![unset(), opt_text(code), unset(), rf(sources[system_id])];
                    args.extend(x.by(Vec::new(), vec![unset(), unset()]));
                    let reference = x.ifc.add("IFCCLASSIFICATIONREFERENCE", args);
                    references.insert((system_id.clone(), code.clone()), reference);
                    reference
                }
            };
            attached.entry(reference).or_insert_with(|| (format!("{system_id}:{code}"), Vec::new())).1.push(entity);
        }
    }
    for (reference, (key, entities)) in attached {
        x.ifc.rooted("IFCRELASSOCIATESCLASSIFICATION", &key, "", "", vec![refs(&entities), rf(reference)]);
    }
    if let (false, Some(project)) = (parents.is_empty(), x.links.elements.get(":project").copied()) {
        let rows: Vec<(&str, V)> = parents.iter().map(|(key, parent)| (key.as_str(), label(parent))).collect();
        let set = property_set(x, "project:classification-parents", PARENTS_SET, rows);
        define(x, "project:classification-parents", &[project], set);
    }
}
//#endregion 🔖️Classifications

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
