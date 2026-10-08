//! 🔺️ `introduce-property-definition` — sparse diff construction.

use super::mutation::IntroducePropertyDefinition;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757PropertyDefinitionsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an explicit index past the end is
/// `mutation.target-missing`.

pub fn diff(payload: &IntroducePropertyDefinition, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A property definition with id \"{}\" already exists.", payload.property_definition.id), [payload.property_definition.id.clone()]);
    }
    let len = base.catalogue.property_definitions.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.property_definition.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { property_definitions: Some(Iso16757PropertyDefinitionsRows::insertion(payload.index.unwrap_or(len), payload.property_definition.clone())), ..Default::default() })
}
