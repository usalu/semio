//! ↩️ `change-annex` — undo restores BASE's annex.

use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeAnnex, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
//#endregion 🔖️Inverse
