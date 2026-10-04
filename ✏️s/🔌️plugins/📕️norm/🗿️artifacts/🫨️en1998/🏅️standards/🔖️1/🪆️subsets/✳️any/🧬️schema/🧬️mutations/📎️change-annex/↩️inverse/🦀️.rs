//! Inverse for `change-annex`.
use super::ChangeAnnex;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &ChangeAnnex, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex.clone() })]

    })())
}
