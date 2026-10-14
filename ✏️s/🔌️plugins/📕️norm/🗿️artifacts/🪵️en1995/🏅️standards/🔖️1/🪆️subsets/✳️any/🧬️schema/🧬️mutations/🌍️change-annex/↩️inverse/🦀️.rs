//! ↩️ `change-annex` inverse — restores the pre-change `annex` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::change_annex::ChangeAnnex;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &ChangeAnnex, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
//#endregion 🔖️Inverse
