use super::InsertConcentrated;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
use crate::mutations::remove_concentrated;
pub fn inverse(payload: &InsertConcentrated, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::RemoveConcentrated(remove_concentrated::RemoveConcentrated { wall_index: payload.wall_index, load_case_index: payload.load_case_index, index: payload.index })]
}
