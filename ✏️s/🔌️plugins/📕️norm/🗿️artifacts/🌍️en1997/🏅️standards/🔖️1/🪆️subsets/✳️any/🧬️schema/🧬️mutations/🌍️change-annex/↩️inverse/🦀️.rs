use crate::change_annex::ChangeAnnex;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeAnnex, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
