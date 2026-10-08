use super::ReorderMembers;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ReorderMembers, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.from_index >= base.members.len() {
        return Vec::new();
    }
    let moved_to = payload.to_index.min(base.members.len() - 1);
    vec![En1992Mutation::ReorderMembers(ReorderMembers { from_index: moved_to, to_index: payload.from_index })]

    })())
}
