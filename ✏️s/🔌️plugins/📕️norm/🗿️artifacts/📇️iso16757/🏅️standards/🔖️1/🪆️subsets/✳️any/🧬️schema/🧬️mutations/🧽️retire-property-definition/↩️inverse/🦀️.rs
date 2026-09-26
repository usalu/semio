//! ↩️ `retire-property-definition` — undo re-`create`s the definition from BASE state, at its
//! original index; missing id ⇒ `Vec::new()`.

use crate::mutations::introduce_property_definition;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::RetirePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse(payload: &RetirePropertyDefinition, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(position) = base.catalogue.property_definitions.iter().position(|definition| definition.id == payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroducePropertyDefinition(introduce_property_definition::mutation::IntroducePropertyDefinition { property_definition: base.catalogue.property_definitions[position].clone(), index: Some(position) })]
}
//#endregion 🔖️Inverse
