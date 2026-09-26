//! ↩️ `introduce-property-definition` — undo is `retire-property-definition`, unless `base` already
//! had this id (then `create` was a no-op and there's nothing to undo).

use crate::mutations::retire_property_definition;
use crate::{Iso16757Mutation, Iso16757Snapshot};

use super::mutation::IntroducePropertyDefinition;

//#region 🔖️Inverse
pub fn inverse(payload: &IntroducePropertyDefinition, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetirePropertyDefinition(retire_property_definition::mutation::RetirePropertyDefinition { id: payload.property_definition.id.clone() })]
}
//#endregion 🔖️Inverse
