use crate::insert_buckling_length::InsertBucklingLength;
use crate::remove_buckling_length;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertBucklingLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_buckling_length::RemoveBucklingLength> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.buckling_lengths.len());
    vec![Op::from(remove_buckling_length::RemoveBucklingLength { index: at })]

    })())
}
