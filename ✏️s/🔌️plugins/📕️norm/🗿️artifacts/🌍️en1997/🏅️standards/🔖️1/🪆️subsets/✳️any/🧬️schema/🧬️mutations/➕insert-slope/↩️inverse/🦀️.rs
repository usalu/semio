use crate::insert_slope::InsertSlope;
use crate::remove_slope::RemoveSlope;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertSlope, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_slope::RemoveSlope> {
    Ok((|| {
    let at = payload.index.min(base.slopes.len());
    vec![Op::from(RemoveSlope { index: at })]

    })())
}
