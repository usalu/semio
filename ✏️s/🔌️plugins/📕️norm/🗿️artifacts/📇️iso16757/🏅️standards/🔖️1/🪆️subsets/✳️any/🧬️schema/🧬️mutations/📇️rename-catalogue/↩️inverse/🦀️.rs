//! ↩️ `rename-catalogue` — undo restores BASE's preferred name.

use super::mutation::RenameCatalogue;
use crate::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &RenameCatalogue, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: base.catalogue.metadata.names.preferred.text.clone() })]

    })())
}
//#endregion 🔖️Inverse
