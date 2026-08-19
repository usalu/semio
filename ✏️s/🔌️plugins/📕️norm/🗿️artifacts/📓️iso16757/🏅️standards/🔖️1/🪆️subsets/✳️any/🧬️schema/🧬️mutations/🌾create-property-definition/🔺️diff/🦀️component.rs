//! 🔺️ `create-property-definition` — sparse diff construction.

use super::mutation::CreatePropertyDefinition;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.
pub async fn diff(payload: &CreatePropertyDefinition, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A property definition with id \"{}\" already exists.", payload.property_definition.id), [payload.property_definition.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.property_definitions.len());
    match payload.index {
        Some(index) if index <= catalogue.property_definitions.len() => catalogue.property_definitions.insert(index, payload.property_definition.clone()),
        _ => catalogue.property_definitions.push(payload.property_definition.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index was out of range; appended property definition \"{}\" at the end instead.", payload.property_definition.id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
