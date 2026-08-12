//! 🔺️ `delete-property-definition` — sparse diff construction.

use super::mutation::DeletePropertyDefinition;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeletePropertyDefinition, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    catalogue.property_definitions.retain(|definition| definition.id != payload.id);
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
