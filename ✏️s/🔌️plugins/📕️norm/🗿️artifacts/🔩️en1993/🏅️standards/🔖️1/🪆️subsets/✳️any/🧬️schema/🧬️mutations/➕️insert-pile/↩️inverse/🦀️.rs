use crate::insert_pile::InsertPile;
use crate::remove_pile;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertPile, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pile::RemovePile> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.piles.len());
    vec![Op::from(remove_pile::RemovePile { index: at })]

    })())
}
