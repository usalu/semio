//! 🔺️ `retire-property-definition` — sparse diff construction.

use super::mutation::RetirePropertyDefinition;
use crate::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RetirePropertyDefinition, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.catalogue.property_definitions.iter().any(|definition| definition.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Property definition \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.property_definitions.retain(|definition| definition.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
