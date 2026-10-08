use super::RemovePile;
use crate::mutations::{insert_pile, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemovePile, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.piles.len() { return Vec::new(); }
    vec![En1993Mutation::InsertPile(insert_pile::InsertPile { index: Some(payload.index), pile: base.piles[payload.index].clone() })]

    })())
}
