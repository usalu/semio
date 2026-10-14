//! ↩️ `rename-manufacturer` — undo restores BASE's manufacturer preferred name.

use crate::rename_manufacturer::mutation::RenameManufacturer;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &RenameManufacturer, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::rename_manufacturer::mutation::RenameManufacturer> {
    Ok((|| {
    vec![Op::from(RenameManufacturer { new_name: base.catalogue.manufacturer.names.preferred.text.clone() })]

    })())
}
//#endregion 🔖️Inverse
