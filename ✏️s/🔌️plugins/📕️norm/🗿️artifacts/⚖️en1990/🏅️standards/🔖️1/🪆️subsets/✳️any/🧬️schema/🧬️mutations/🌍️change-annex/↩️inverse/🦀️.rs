//! ↩️ `change-annex` inverse.

use super::ChangeAnnex;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeAnnex, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]

    })())
}
