//! ↩️ `change-annex` inverse.

use crate::change_annex::ChangeAnnex;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeAnnex, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
