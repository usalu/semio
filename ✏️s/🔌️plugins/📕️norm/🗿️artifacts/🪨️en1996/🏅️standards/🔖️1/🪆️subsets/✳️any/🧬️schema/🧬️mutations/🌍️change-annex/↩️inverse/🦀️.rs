use crate::change_annex::ChangeAnnex;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(_payload: &ChangeAnnex, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
