//! Inverse for `change-annex`.
use super::ChangeAnnex;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAnnex, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]

    })())
}
