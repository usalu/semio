use crate::insert_storey::InsertStorey;
use crate::remove_storey;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertStorey, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_storey::RemoveStorey> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.storeys.len());
    vec![Op::from(remove_storey::RemoveStorey { index: at })]

    })())
}
