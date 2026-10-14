use crate::remove_pile::RemovePile;
use crate::insert_pile;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemovePile, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pile::InsertPile> {
    Ok((|| {
    if payload.index >= base.piles.len() { return Vec::new(); }
    vec![Op::from(insert_pile::InsertPile { index: Some(payload.index), pile: base.piles[payload.index].clone() })]

    })())
}
