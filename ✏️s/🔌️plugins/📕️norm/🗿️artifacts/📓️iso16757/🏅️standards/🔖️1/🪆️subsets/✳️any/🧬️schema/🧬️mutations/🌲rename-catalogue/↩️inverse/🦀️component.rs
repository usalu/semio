//! ↩️ `rename-catalogue` — undo restores BASE's preferred name.

use super::mutation::RenameCatalogue;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &RenameCatalogue, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: base.catalogue.metadata.names.preferred.text.clone() })]
}
//#endregion 🔖️Inverse
