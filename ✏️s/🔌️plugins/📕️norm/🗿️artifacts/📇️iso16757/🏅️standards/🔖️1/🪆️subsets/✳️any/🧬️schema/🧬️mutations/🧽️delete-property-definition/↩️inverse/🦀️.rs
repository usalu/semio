//! ↩️ `delete-property-definition` — undo re-`create`s the definition from BASE state, at its
//! original index; missing id ⇒ `Vec::new()`.

use crate::artifacts::iso16757::mutations::create_property_definition;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::DeletePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse(payload: &DeletePropertyDefinition, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.catalogue.property_definitions.iter().position(|definition| definition.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::CreatePropertyDefinition(create_property_definition::mutation::CreatePropertyDefinition { property_definition: base.catalogue.property_definitions[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
