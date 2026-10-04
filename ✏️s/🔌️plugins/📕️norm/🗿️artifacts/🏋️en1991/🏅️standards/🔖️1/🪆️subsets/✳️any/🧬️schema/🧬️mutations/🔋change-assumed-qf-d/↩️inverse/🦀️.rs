//! Inverse for `change-assumed-qf-d`.
use super::ChangeAssumedQfD;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedQfD, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAssumedQfD(ChangeAssumedQfD { new_assumed_qf_d: base.assumed_qf_d })]

    })())
}
