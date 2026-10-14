//! ↩️ `change-annex` inverse.
use crate::change_annex::ChangeAnnex;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(_payload: &ChangeAnnex, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok((|| {
    vec![Op::from(ChangeAnnex { new_annex: base.annex })]

    })())
}
