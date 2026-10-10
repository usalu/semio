//! 🧰️ Shared helpers of the IFC export tests: the committed house model and readers over the generated Part-21 document.

use super::{model_to_part21, Schema};
use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

/// 🏠️ The committed export fixture: one site, one building, four storeys and every element family.
pub const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 🏠️ The decoded house model.
pub fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
}

/// 🏷️ The committed psets fixture: the house with three classification systems (tables with parents), several systems per element and per type, type-level property sets and the property set templates of the IFC4 library.
pub const PSETS: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏷️psets/📸️snapshot/🔣️.json");

/// 🏷️ The decoded psets model.
pub fn psets() -> ModelSnapshot {
    from_json_str(PSETS, JsonMemberPolicy::Reject).expect("the committed psets model decodes")
}

/// 🪧️ The committed annotated room: dimensions, tags, a note and a leader around four walls, a window, a grid line and a column.
pub const NOTATED: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json");

/// 🪧️ The decoded annotated room.
pub fn notated() -> ModelSnapshot {
    from_json_str(NOTATED, JsonMemberPolicy::Reject).expect("the committed room decodes")
}

/// 🔲️ The committed ceilings: a board ceiling with a hole, a sloped and a diagonally sloped tile ceiling and one with a half-round end, on two storeys.
pub const CEILINGS: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json");

/// 🔲️ The decoded ceilings model.
pub fn ceilings() -> ModelSnapshot {
    from_json_str(CEILINGS, JsonMemberPolicy::Reject).expect("the committed ceilings decode")
}

/// 🪑️ The committed components model: every category of family, free, hosted, rotated and mirrored components, overrides, terminals of several systems and ducts, pipes and trays on two storeys.
pub const COMPONENTS: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🪑️components/📸️snapshot/🔣️.json");

/// 🪑️ The decoded components model.
pub fn components() -> ModelSnapshot {
    from_json_str(COMPONENTS, JsonMemberPolicy::Reject).expect("the committed components model decodes")
}

/// 📄️ The IFC 2x3 Part-21 document of a model.
pub fn document(model: &ModelSnapshot) -> Part21Document {
    document_in(Schema::Ifc2x3, model)
}

/// 📄️ The Part-21 document of a model in `schema`.
pub fn document_in(schema: Schema, model: &ModelSnapshot) -> Part21Document {
    model_to_part21(schema, model).expect("the model exports").0
}

/// 🧩️ Every instance of an entity type with its argument list.
pub fn rows<'a>(document: &'a Part21Document, entity: &'a str) -> Vec<(&'a Part21Instance, &'a Vec<Part21Value>)> {
    document.by_type(entity).filter_map(|instance| instance.entity(entity).map(|args| (instance, args))).collect()
}

/// 🔢️ How many instances of an entity type the document holds.
pub fn count(document: &Part21Document, entity: &str) -> usize {
    document.by_type(entity).count()
}

/// 🏷️ The string at argument `index`.
pub fn string(args: &[Part21Value], index: usize) -> Option<String> {
    args.get(index).and_then(|value| value.as_str()).map(str::to_string)
}

/// 🔢️ The real at argument `index`.
pub fn real(args: &[Part21Value], index: usize) -> f64 {
    args.get(index).and_then(|value| value.as_real()).unwrap_or(f64::NAN)
}

/// 🔗️ The instance that argument `index` refers to.
pub fn target<'a>(document: &'a Part21Document, args: &[Part21Value], index: usize) -> Option<&'a Part21Instance> {
    args.get(index).and_then(|value| document.resolve(value))
}

/// 🧮️ The value of base quantity `name` of property set `set` of the products and spaces written for model element `id` (`Tag`, or `ObjectType` of a space; a window or door shares its id with its opening element).
pub fn quantity(document: &Part21Document, id: &str, set: &str, name: &str) -> Option<f64> {
    let elements: Vec<u64> = document.instances.iter().filter(|instance| instance.primary().is_some_and(|(entity, args)| string(args, if entity == "IFCSPACE" { 4 } else { 7 }).as_deref() == Some(id))).map(|instance| instance.id).collect();
    document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES")).filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id().is_some_and(|target| elements.contains(&target))))).find_map(|args| {
        let definition = document.resolve(&args[5])?.entity("IFCELEMENTQUANTITY").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
        definition[5].as_list()?.iter().filter_map(|item| document.resolve(item)).find_map(|row| row.entities.iter().find(|(_, args)| string(args, 0).as_deref() == Some(name)).map(|(_, args)| real(args, 3)))
    })
}

/// 🏷️ The sorted `Tag` strings (argument 7) of every product of an entity type.
pub fn tags(document: &Part21Document, entity: &str) -> Vec<String> {
    let mut tags: Vec<String> = rows(document, entity).iter().filter_map(|(_, args)| string(args, 7)).collect();
    tags.sort();
    tags
}

/// 🧗️ The committed attic: gable and hip roofs with walls under them, a wall standing on a sloped slab, baseboards, a hand rail, a door, a window with a reveal and a plain window.
pub const ATTIC: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json");

/// 🧗️ The decoded attic model.
pub fn attic() -> ModelSnapshot {
    from_json_str(ATTIC, JsonMemberPolicy::Reject).expect("the committed attic decodes")
}

/// 🧮️ The value of base quantity `name` of property set `set` of the product `element`.
pub fn quantity_of(document: &Part21Document, element: u64, set: &str, name: &str) -> Option<f64> {
    document.by_type("IFCRELDEFINESBYPROPERTIES").filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES")).filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id() == Some(element)))).find_map(|args| {
        let definition = document.resolve(&args[5])?.entity("IFCELEMENTQUANTITY").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
        definition[5].as_list()?.iter().filter_map(|item| document.resolve(item)).find_map(|row| row.entities.iter().find(|(_, args)| string(args, 0).as_deref() == Some(name)).map(|(_, args)| real(args, 3)))
    })
}

/// 🏷️ The rows of property set `set` of every product tagged `id` that carries it, by property name with the typed value as written.
pub fn property_sets(document: &Part21Document, id: &str, set: &str) -> Vec<BTreeMap<String, Part21Value>> {
    let tagged: Vec<u64> = document.instances.iter().filter(|instance| instance.primary().is_some_and(|(_, args)| string(args, 7).as_deref() == Some(id))).map(|instance| instance.id).collect();
    document
        .by_type("IFCRELDEFINESBYPROPERTIES")
        .filter_map(|relation| relation.entity("IFCRELDEFINESBYPROPERTIES"))
        .filter(|args| args[4].as_list().is_some_and(|items| items.iter().any(|item| item.as_ref_id().is_some_and(|target| tagged.contains(&target)))))
        .filter_map(|args| {
            let definition = document.resolve(&args[5])?.entity("IFCPROPERTYSET").filter(|definition| string(definition, 2).as_deref() == Some(set))?;
            Some(definition[4].as_list()?.iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCPROPERTYSINGLEVALUE")).map(|row| (string(row, 0).unwrap_or_default(), row[2].clone())).collect())
        })
        .collect()
}

/// 🔤️ The string inside a typed property value.
pub fn text_of(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<String> {
    rows.get(name)?.as_typed()?.1.first()?.as_str().map(str::to_string)
}

/// 🔢️ The number inside a typed property value.
pub fn number_of(rows: &BTreeMap<String, Part21Value>, name: &str) -> Option<f64> {
    rows.get(name)?.as_typed()?.1.first()?.as_real()
}

/// 📍️ The location of the placement of a product (argument 5) relative to its parent.
pub fn location(document: &Part21Document, args: &[Part21Value]) -> Option<Vec<f64>> {
    let placement = target(document, args, 5)?.entity("IFCLOCALPLACEMENT")?;
    let axis = document.resolve(&placement[1])?.entity("IFCAXIS2PLACEMENT3D")?;
    let point = document.resolve(&axis[0])?.entity("IFCCARTESIANPOINT")?;
    Some(point[0].as_list()?.iter().filter_map(Part21Value::as_real).collect())
}
