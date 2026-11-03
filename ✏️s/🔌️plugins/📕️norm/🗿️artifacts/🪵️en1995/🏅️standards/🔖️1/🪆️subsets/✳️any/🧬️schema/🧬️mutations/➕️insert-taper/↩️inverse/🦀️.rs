use crate::insert_taper::InsertTaper;
use crate::remove_taper;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertTaper, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_taper::RemoveTaper> {
    let at = payload.index.min(base.tapers.len());
    Ok(vec![Op::from(remove_taper::RemoveTaper { index: at })])
}
