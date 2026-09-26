use super::InsertOpening;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::remove_opening;
pub fn inverse(payload: &InsertOpening, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::RemoveOpening(remove_opening::RemoveOpening { wall_index: payload.wall_index, index: payload.index })]
}
