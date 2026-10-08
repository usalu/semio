//! 🔺️ `introduce-property-definition` — sparse diff construction.

use super::mutation::IntroducePropertyDefinition;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757PropertyDefinitionsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.

pub fn diff(payload: &IntroducePropertyDefinition, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A property definition with id \"{}\" already exists.", payload.property_definition.id), [payload.property_definition.id.clone()]);
    }
    let ids: Vec<String> = base.catalogue.property_definitions.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.property_definition.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { property_definitions: Some(Iso16757PropertyDefinitionsRows { added: vec![payload.property_definition.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended property definition \"{}\" at the end instead.", payload.property_definition.id))
    } else {
        outcome
    }
}
