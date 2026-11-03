use crate::remove_storey::RemoveStorey;
use crate::insert_storey;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveStorey, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_storey::InsertStorey> {
    Ok((|| {
    if payload.index >= base.storeys.len() { return Vec::new(); }
    vec![Op::from(insert_storey::InsertStorey { index: Some(payload.index), storey: base.storeys[payload.index].clone() })]

    })())
}
