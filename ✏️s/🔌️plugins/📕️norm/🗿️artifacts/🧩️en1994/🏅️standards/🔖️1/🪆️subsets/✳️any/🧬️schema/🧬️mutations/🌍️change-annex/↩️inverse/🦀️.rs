//! Inverse for `change-annex`.
use super::ChangeAnnex;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeAnnex, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1994Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]

    })())
}
