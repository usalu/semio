//! Inverse for `insert-self-weight-elements`.
use super::InsertSelfWeightElements;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &InsertSelfWeightElements, _base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::RemoveSelfWeightElements(crate::mutations::remove_self_weight_elements::RemoveSelfWeightElements { index: payload.index })]

    })())
}
