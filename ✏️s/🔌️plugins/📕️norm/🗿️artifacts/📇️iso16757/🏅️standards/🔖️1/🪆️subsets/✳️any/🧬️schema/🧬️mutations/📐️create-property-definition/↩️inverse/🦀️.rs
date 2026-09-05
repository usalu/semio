//! ↩️ `create-property-definition` — undo is `delete-property-definition`, unless `base` already
//! had this id (then `create` was a no-op and there's nothing to undo).

use crate::artifacts::iso16757::mutations::delete_property_definition;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::CreatePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse(payload: &CreatePropertyDefinition, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::DeletePropertyDefinition(delete_property_definition::mutation::DeletePropertyDefinition { id: payload.property_definition.id.clone() })]
}
//#endregion 🔖️Inverse
