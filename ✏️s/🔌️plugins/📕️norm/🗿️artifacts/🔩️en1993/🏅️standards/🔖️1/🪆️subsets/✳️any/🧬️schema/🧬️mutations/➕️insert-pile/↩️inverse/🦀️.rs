use super::InsertPile;
use crate::mutations::{remove_pile, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertPile, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.min(base.piles.len());
    vec![En1993Mutation::RemovePile(remove_pile::RemovePile { index: at })]

    })())
}
