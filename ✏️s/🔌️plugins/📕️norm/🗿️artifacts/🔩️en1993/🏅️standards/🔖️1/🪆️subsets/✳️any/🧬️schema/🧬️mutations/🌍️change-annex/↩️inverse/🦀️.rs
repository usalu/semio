//! ↩️ `change-annex` — undo restores BASE's annex.

use super::ChangeAnnex;
use crate::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnex, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1993Mutation::ChangeAnnex(ChangeAnnex { new_annex: base.annex })]

    })())
}
//#endregion 🔖️Inverse
