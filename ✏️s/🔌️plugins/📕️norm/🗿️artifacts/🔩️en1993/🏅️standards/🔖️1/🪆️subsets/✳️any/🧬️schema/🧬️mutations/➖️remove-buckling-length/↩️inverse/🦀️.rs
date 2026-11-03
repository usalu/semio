use crate::remove_buckling_length::RemoveBucklingLength;
use crate::insert_buckling_length;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveBucklingLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_buckling_length::InsertBucklingLength> {
    Ok((|| {
    if payload.index >= base.buckling_lengths.len() { return Vec::new(); }
    vec![Op::from(insert_buckling_length::InsertBucklingLength { index: Some(payload.index), buckling_length: base.buckling_lengths[payload.index].clone() })]

    })())
}
