use crate::remove_taper::RemoveTaper;
use crate::insert_taper;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveTaper, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_taper::InsertTaper> {
    Ok(match base.tapers.get(payload.index) {
        Some(item) => vec![Op::from(insert_taper::InsertTaper { index: payload.index, taper: item.clone() })],
        None => Vec::new(),
    })
}
