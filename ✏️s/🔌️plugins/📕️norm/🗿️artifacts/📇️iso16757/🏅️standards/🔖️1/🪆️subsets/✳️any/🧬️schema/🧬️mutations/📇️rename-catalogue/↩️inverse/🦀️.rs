//! ↩️ `rename-catalogue` — undo restores BASE's preferred name.

use crate::rename_catalogue::mutation::RenameCatalogue;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &RenameCatalogue, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_catalogue::mutation::RenameCatalogue> {
    Ok((|| {
    vec![Op::from(RenameCatalogue { new_name: base.catalogue.metadata.names.preferred.text.clone() })]

    })())
}
//#endregion 🔖️Inverse
