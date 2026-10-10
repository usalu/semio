//! 🪑️ Components of an IFC file. A furnishing element, flow terminal or building element proxy that carries the authored `Component` record in its `Semio_Authoring` set is restored exactly: the record, the family from the type
//! object that carries the authored family (`Family`, `Parameters` and `Solids` rows), the overrides from the `Semio_ComponentOverrides` set, the storey from the spatial structure. A product of a foreign file becomes a component of a
//! generic family named after its type: one cuboid solid of the bounding box of its body (swept, faceted, triangulated or mapped), placed by its inferred position, elevation and heading; its system is the group it is assigned to.
//! 📎 <https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgserviceelements/lexical/ifcflowterminal.htm>

use super::data::{label, type_authoring};
use super::frames::Rigid;
use super::reader::{opt_text, real, text, Doc, Section};
use super::spatial::{authoring_of, set_of, string_of};
use super::Import;
pub use crate::standards::v1::subsets::any::io::export::ifc::components::OVERRIDES_SET;
use crate::standards::v1::subsets::any::io::export::ifc::components::{FAMILY_ROWS, RECORD_ROW};
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{Component, ComponentOverride, ExprPoint3, Family, FamilyCategory, FamilyParameter, FamilySolid, MepSystem, ParameterKind, Point2, SolidShape};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

/// 🪑️ The occurrence classes a component can be.
pub const OCCURRENCES: [&str; 7] = ["IFCFURNISHINGELEMENT", "IFCFURNITURE", "IFCFLOWTERMINAL", "IFCSANITARYTERMINAL", "IFCLIGHTFIXTURE", "IFCAIRTERMINAL", "IFCBUILDINGELEMENTPROXY"];
/// 🏷️ The type classes that can carry an authored family.
pub const TYPES: [&str; 6] = ["IFCFURNITURETYPE", "IFCSANITARYTERMINALTYPE", "IFCLIGHTFIXTURETYPE", "IFCFLOWTERMINALTYPE", "IFCAIRTERMINALTYPE", "IFCBUILDINGELEMENTPROXYTYPE"];

//#region 🔖️Systems
/// 🌀️ The service a group is named after (`Supply`, `DomesticWater`, …) or, for an IFC4 distribution system, its predefined type.
pub fn system_named(name: &str) -> Option<MepSystem> {
    let flat: String = name.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_ascii_lowercase();
    Some(match flat.as_str() {
        "supply" | "supplyair" | "ventilation" => MepSystem::Supply,
        "return" | "returnair" => MepSystem::Return,
        "exhaust" | "exhaustair" => MepSystem::Exhaust,
        "domesticwater" | "domesticcoldwater" | "domestichotwater" | "watersupply" => MepSystem::DomesticWater,
        "waste" | "wastewater" | "sewage" | "drainage" => MepSystem::Waste,
        "gas" => MepSystem::Gas,
        "power" | "electrical" => MepSystem::Power,
        "data" | "communication" => MepSystem::Data,
        "lighting" => MepSystem::Lighting,
        _ => return None,
    })
}

/// 🌀️ The service every product is assigned to by an `IfcRelAssignsToGroup` of a system, by product instance id.
pub fn assignments(i: &Import<'_>) -> BTreeMap<u64, MepSystem> {
    let mut found = BTreeMap::new();
    for (_, relation) in i.doc.rows("IFCRELASSIGNSTOGROUP") {
        let Some(group) = relation.get(6).and_then(|value| i.doc.follow(value)) else { continue };
        let Some((class, args)) = group.primary().filter(|(class, _)| matches!(*class, "IFCSYSTEM" | "IFCDISTRIBUTIONSYSTEM")) else { continue };
        let predefined = if class == "IFCDISTRIBUTIONSYSTEM" { args.get(7).and_then(Part21Value::as_enum).unwrap_or_default().to_string() } else { String::new() };
        let Some(system) = [text(args, 2), text(args, 4), predefined].iter().find_map(|name| system_named(name)) else { continue };
        for member in super::reader::refs(relation, 4) {
            found.insert(member, system);
        }
    }
    found
}
//#endregion 🔖️Systems

//#region 🔖️Bounds
fn body_items<'a>(doc: &Doc<'a>, product: &[Part21Value]) -> Vec<&'a Part21Value> {
    let Some(shape) = product.get(6).and_then(|value| doc.follow_args(value, "IFCPRODUCTDEFINITIONSHAPE")) else { return Vec::new() };
    shape[2].as_list().unwrap_or_default().iter().filter_map(|value| doc.follow_args(value, "IFCSHAPEREPRESENTATION")).filter(|args| text(args, 1) == "Body").flat_map(|args| args.get(3).and_then(Part21Value::as_list).unwrap_or_default().iter()).collect()
}

fn lift(chain: &[Rigid], point: [f64; 3]) -> [f64; 3] {
    chain.iter().rev().fold(point, |at, frame| frame.point(at))
}

fn section_corners(section: &Section) -> Vec<[f64; 2]> {
    match section {
        Section::Rectangle { centre, width, depth } => vec![[centre[0] - width / 2.0, centre[1] - depth / 2.0], [centre[0] + width / 2.0, centre[1] + depth / 2.0]],
        Section::Circle { diameter } => vec![[-diameter / 2.0, -diameter / 2.0], [diameter / 2.0, diameter / 2.0]],
        Section::IShape { width, depth, .. } => vec![[-width / 2.0, -depth / 2.0], [width / 2.0, depth / 2.0]],
        Section::Outline { outer, .. } => outer.iter().map(|(point, _)| *point).collect(),
    }
}

fn gather(doc: &Doc<'_>, item: &Part21Value, chain: &[Rigid], points: &mut Vec<[f64; 3]>, depth: usize) {
    if depth > 3 {
        return;
    }
    if let Some(args) = doc.follow_args(item, "IFCEXTRUDEDAREASOLID") {
        let (Some(section), Some(direction), Some(length)) = (doc.section(&args[0]), doc.direction(&args[2]), real(args, 3)) else { return };
        let position = doc.axis_placement(&args[1]);
        let along = position.vector(direction);
        for corner in section_corners(&section) {
            let base = position.point([corner[0], corner[1], 0.0]);
            points.push(lift(chain, base));
            points.push(lift(chain, [base[0] + along[0] * length, base[1] + along[1] * length, base[2] + along[2] * length]));
        }
    } else if let Some(args) = doc.follow_args(item, "IFCFACETEDBREP") {
        let faces = doc.follow_args(&args[0], "IFCCLOSEDSHELL").and_then(|shell| shell[0].as_list()).unwrap_or_default();
        for face in faces.iter().filter_map(|face| doc.follow_args(face, "IFCFACE")) {
            for bound in face[0].as_list().unwrap_or_default() {
                let Some(bound) = doc.follow(bound).and_then(|instance| instance.primary()).map(|(_, args)| args) else { continue };
                if let Some(loop_args) = doc.follow_args(&bound[0], "IFCPOLYLOOP") {
                    points.extend(loop_args[0].as_list().unwrap_or_default().iter().map(|point| lift(chain, doc.point(point))));
                }
            }
        }
    } else if let Some(args) = doc.follow_args(item, "IFCTRIANGULATEDFACESET") {
        if let Some(list) = doc.follow_args(&args[0], "IFCCARTESIANPOINTLIST3D") {
            for row in list[0].as_list().unwrap_or_default() {
                let values: Vec<f64> = row.as_list().unwrap_or_default().iter().filter_map(Part21Value::as_real).collect();
                if values.len() == 3 {
                    points.push(lift(chain, [values[0], values[1], values[2]]));
                }
            }
        }
    } else if let Some(args) = doc.follow_args(item, "IFCMAPPEDITEM") {
        let Some(map) = doc.follow_args(&args[0], "IFCREPRESENTATIONMAP") else { return };
        let origin = doc.axis_placement(&map[0]);
        let target = doc.follow_args(&args[1], "IFCCARTESIANTRANSFORMATIONOPERATOR3D");
        let operator = target.map_or(Rigid::IDENTITY, |operator| Rigid::from_axes(operator.get(2).map_or([0.0; 3], |origin| doc.point(origin)), operator.get(4).and_then(|axis| doc.direction(axis)), operator.first().and_then(|axis| doc.direction(axis))));
        let mut next: Vec<Rigid> = chain.to_vec();
        next.push(operator);
        next.push(origin.inverse());
        if let Some(shape) = doc.follow_args(&map[1], "IFCSHAPEREPRESENTATION") {
            for inner in shape.get(3).and_then(Part21Value::as_list).unwrap_or_default() {
                gather(doc, inner, &next, points, depth + 1);
            }
        }
    }
}

/// 📦️ The corners of the box that bounds the body of a product in the frame of the product, `None` for a product without a readable body.
pub fn body_bounds(doc: &Doc<'_>, product: &[Part21Value]) -> Option<([f64; 3], [f64; 3])> {
    let mut points = Vec::new();
    for item in body_items(doc, product) {
        gather(doc, item, &[], &mut points, 0);
    }
    let first = *points.first()?;
    Some(points.iter().fold((first, first), |(low, high), point| ([low[0].min(point[0]), low[1].min(point[1]), low[2].min(point[2])], [high[0].max(point[0]), high[1].max(point[1]), high[2].max(point[2])])))
}
//#endregion 🔖️Bounds

//#region 🔖️Families
fn category_of(class: &str) -> FamilyCategory {
    match class {
        "IFCFURNISHINGELEMENT" | "IFCFURNITURE" => FamilyCategory::Furniture,
        "IFCSANITARYTERMINAL" => FamilyCategory::Plumbing,
        "IFCLIGHTFIXTURE" => FamilyCategory::Lighting,
        "IFCFLOWTERMINAL" | "IFCAIRTERMINAL" => FamilyCategory::Equipment,
        _ => FamilyCategory::Generic,
    }
}

fn read_types(i: &mut Import<'_>) {
    for class in TYPES {
        for (instance, args) in i.doc.rows(class) {
            let rows = type_authoring(&i.doc, args);
            let Some(record) = label(&rows, FAMILY_ROWS[0]) else { continue };
            let name = text(args, 2);
            let (Ok(family), Ok(parameters), Ok(solids)) = (
                from_json_str::<Family>(&record, JsonMemberPolicy::Reject),
                from_json_str::<BTreeMap<String, FamilyParameter>>(&label(&rows, FAMILY_ROWS[1]).unwrap_or_else(|| "{}".into()), JsonMemberPolicy::Reject),
                from_json_str::<BTreeMap<String, FamilySolid>>(&label(&rows, FAMILY_ROWS[2]).unwrap_or_else(|| "{}".into()), JsonMemberPolicy::Reject),
            ) else {
                i.skip(class, &name, "its authored family record is not valid");
                continue;
            };
            let id = opt_text(args, 7).unwrap_or_else(|| Import::unused(&format!("fam-{}", Import::slug(&name)), |candidate| i.model.families.contains_key(candidate)));
            if !i.model.families.contains_key(&id) {
                for (parameter, row) in parameters {
                    i.model.family_parameters.insert(formula::parameter_id(&id, &parameter), FamilyParameter { family: id.clone(), ..row });
                }
                for (key, row) in solids {
                    i.model.family_solids.insert(key, FamilySolid { family: id.clone(), ..row });
                }
                i.model.families.insert(id.clone(), family);
            }
            i.type_ids.insert(instance.id, id);
        }
    }
}

fn metres(value: f64) -> String {
    let rounded = (value * 1e6).round() / 1e6;
    let text = format!("{} m", if rounded == 0.0 { 0.0 } else { rounded });
    formula::canonical(&text).unwrap_or_else(|_| "0 m".to_string())
}

fn cuboid_family(i: &mut Import<'_>, known: &mut BTreeMap<String, String>, category: FamilyCategory, name: &str, low: [f64; 3], high: [f64; 3], material: &str) -> String {
    let size = [(high[0] - low[0]).max(1e-3), (high[1] - low[1]).max(1e-3), (high[2] - low[2]).max(1e-3)];
    let key = format!("{category:?}|{name}|{material}|{:?}", [low, size].map(|row| row.map(|v| (v * 1e6).round() as i64)));
    if let Some(id) = known.get(&key) {
        return id.clone();
    }
    let id = Import::unused(&format!("fam-{}", Import::slug(name)), |candidate| i.model.families.contains_key(candidate));
    i.model.families.insert(id.clone(), Family { name: name.to_string(), category });
    for (parameter, value) in [("width", size[0]), ("depth", size[1]), ("height", size[2])] {
        i.model.family_parameters.insert(formula::parameter_id(&id, parameter), FamilyParameter { family: id.clone(), name: parameter.into(), kind: ParameterKind::Length, value: metres(value) });
    }
    let solid = FamilySolid {
        family: id.clone(),
        name: "Body".into(),
        shape: SolidShape::Cuboid { x: metres(low[0]), y: metres(low[1]), z: metres(low[2]), width: "width".into(), depth: "depth".into(), height: "height".into() },
        material: format!("\"{material}\""),
        visible: "true".into(),
        offset: ExprPoint3 { x: "0 m".into(), y: "0 m".into(), z: "0 m".into() },
    };
    i.model.family_solids.insert(format!("fs-{}-body", id.trim_start_matches("fam-")), solid);
    known.insert(key, id.clone());
    id
}
//#endregion 🔖️Families

fn foreign(i: &mut Import<'_>, known: &mut BTreeMap<String, String>, systems: &BTreeMap<u64, MepSystem>, class: &str, ifc: u64, args: &[Part21Value], storey: &str, id: &str) -> Option<Component> {
    let name = text(args, 2);
    let world = i.in_building(&args[5], storey);
    let level = i.levels.get(storey).map_or(0.0, |level| level.elevation);
    let typed = i.doc.index.types.get(&ifc).copied();
    let family = match typed.and_then(|kind| i.type_ids.get(&kind)).filter(|family| i.model.families.contains_key(*family)).cloned() {
        Some(family) => family,
        None => {
            let (low, high) = body_bounds(&i.doc, args).or_else(|| {
                i.skip(class, &name, "its body is neither swept, faceted, triangulated nor mapped");
                None
            })?;
            let family_name = typed.and_then(|kind| i.doc.get(kind)).and_then(|kind| kind.primary()).map(|(_, kind)| text(kind, 2)).filter(|name| !name.is_empty()).or_else(|| Some(name.clone()).filter(|name| !name.is_empty())).unwrap_or_else(|| class.to_ascii_lowercase());
            let material = i.doc.index.materials.get(&ifc).and_then(|material| i.material_ids.get(material)).cloned().unwrap_or_default();
            cuboid_family(i, known, category_of(class), &family_name, low, high, &material)
        }
    };
    Some(Component { storey: storey.to_string(), family, position: Point2 { x: world.origin[0], y: world.origin[1] }, elevation: world.origin[2] - level, rotation: world.heading(), mirrored: false, host: None, system: systems.get(&ifc).copied(), name: if name == id { String::new() } else { name } })
}

/// 🪑️ Reads the family types and the components of the file into the model.
pub fn read(i: &mut Import<'_>) {
    read_types(i);
    let systems = assignments(i);
    let mut known: BTreeMap<String, String> = BTreeMap::new();
    for class in OCCURRENCES {
        for (instance, args) in i.doc.rows(class) {
            if i.ids.contains_key(&instance.id) {
                continue;
            }
            let name = text(args, 2);
            let Some(storey) = i.storey_of(instance.id) else {
                i.skip(class, &name, "it is not in a storey");
                continue;
            };
            let id = Doc::identity(args, class).unwrap_or_else(|| format!("cmp-{}", instance.id));
            let authored = label(&authoring_of(&i.doc, instance.id), RECORD_ROW).and_then(|record| from_json_str::<Component>(&record, JsonMemberPolicy::Reject).ok());
            let component = match authored {
                Some(mut component) => {
                    component.storey = storey;
                    component
                }
                None => match foreign(i, &mut known, &systems, class, instance.id, args, &storey, &id) {
                    Some(component) => component,
                    None => continue,
                },
            };
            for (parameter, value) in set_of(&i.doc, instance.id, OVERRIDES_SET) {
                if let Some(formula_text) = string_of(&value) {
                    i.model.component_overrides.insert(formula::parameter_id(&id, &parameter), ComponentOverride { component: id.clone(), name: parameter, value: formula_text });
                }
            }
            i.model.components.insert(id.clone(), component);
            i.ids.insert(instance.id, id);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
