//! 🧰️ Shared helpers of the IFC export tests: the committed house model and readers over the generated Part-21 document.

use super::model_to_part21;
use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};

/// 🏠️ The committed export fixture: one site, one building, four storeys and every element family.
pub const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 🏠️ The decoded house model.
pub fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
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

/// 📄️ The Part-21 document of a model.
pub fn document(model: &ModelSnapshot) -> Part21Document {
    model_to_part21(model).expect("the model exports").0
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
