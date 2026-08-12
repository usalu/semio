//! 🔺️ `create-property-definition` — sparse diff construction.

use super::mutation::CreatePropertyDefinition;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is a no-op — an id-keyed entity that already exists cannot be "created"
/// again; the catalogue clone is returned unchanged rather than pushing a second definition.
pub fn diff(payload: &CreatePropertyDefinition, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    if !catalogue.property_definitions.iter().any(|definition| definition.id == payload.property_definition.id) {
        match payload.index {
            Some(index) if index <= catalogue.property_definitions.len() => catalogue.property_definitions.insert(index, payload.property_definition.clone()),
            _ => catalogue.property_definitions.push(payload.property_definition.clone()),
        }
    }
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
