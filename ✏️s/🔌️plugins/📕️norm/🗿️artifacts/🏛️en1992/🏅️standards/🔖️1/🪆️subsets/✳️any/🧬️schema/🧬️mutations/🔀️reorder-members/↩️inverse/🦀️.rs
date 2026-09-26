use super::ReorderMembers;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ReorderMembers, _base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ReorderMembers(ReorderMembers { from_index: payload.to_index, to_index: payload.from_index })]
}
