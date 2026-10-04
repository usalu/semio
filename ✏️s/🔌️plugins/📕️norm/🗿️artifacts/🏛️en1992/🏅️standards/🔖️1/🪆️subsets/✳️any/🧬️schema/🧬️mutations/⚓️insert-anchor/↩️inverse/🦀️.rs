use super::InsertAnchor;
use crate::mutations::remove_anchor::RemoveAnchor;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &InsertAnchor, _base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1992Mutation::RemoveAnchor(RemoveAnchor { anchor_id: payload.anchor.id.clone() })]

    })())
}
