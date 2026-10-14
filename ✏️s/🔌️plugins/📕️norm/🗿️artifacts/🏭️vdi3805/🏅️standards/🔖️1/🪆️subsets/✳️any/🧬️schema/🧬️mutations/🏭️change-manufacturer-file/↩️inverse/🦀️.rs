//! ↩️ `change-manufacturer-file` — undo restores BASE's whole header facet.

use crate::change_manufacturer_file::ChangeManufacturerFile;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeManufacturerFile, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_manufacturer_file::ChangeManufacturerFile> {
    Ok((|| {
    vec![Op::from(ChangeManufacturerFile { new_manufacturer_file: base.catalog.file.clone() })]

    })())
}
//#endregion 🔖️Inverse
