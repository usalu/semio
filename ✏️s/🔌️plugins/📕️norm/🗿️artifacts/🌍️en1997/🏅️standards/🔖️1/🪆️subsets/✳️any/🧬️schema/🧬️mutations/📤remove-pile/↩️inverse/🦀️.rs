use crate::remove_pile::RemovePile;
use crate::insert_pile::InsertPile;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemovePile, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pile::InsertPile> {
    Ok((|| {
    let pile = base.piles.get(payload.index).cloned().unwrap_or_else(|| base.piles[0].clone());
    vec![Op::from(InsertPile { index: payload.index, pile })]

    })())
}
