use crate::insert_pile::InsertPile;
use crate::remove_pile::RemovePile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertPile, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pile::RemovePile> {
    Ok((|| {
    let at = payload.index.min(base.piles.len());
    vec![Op::from(RemovePile { index: at })]

    })())
}
