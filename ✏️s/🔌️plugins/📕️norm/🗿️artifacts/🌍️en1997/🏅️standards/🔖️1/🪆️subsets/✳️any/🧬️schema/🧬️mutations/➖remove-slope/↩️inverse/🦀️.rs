use crate::remove_slope::RemoveSlope;
use crate::insert_slope::InsertSlope;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemoveSlope, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_slope::InsertSlope> {
    Ok((|| {
    match base.slopes.get(payload.index).cloned() {
        Some(slope) => vec![Op::from(InsertSlope { index: payload.index, slope })],
        None => Vec::new(),
    }

    })())
}
