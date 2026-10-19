//! ↩️ Inverse for `change-annex`.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(_payload: &ChangeAnnex, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex.clone() })]

    })())
}
