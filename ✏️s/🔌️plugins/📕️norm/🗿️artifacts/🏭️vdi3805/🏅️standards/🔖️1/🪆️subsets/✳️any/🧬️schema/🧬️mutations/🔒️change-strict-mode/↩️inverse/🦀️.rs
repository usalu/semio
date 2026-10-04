//! ↩️ `change-strict-mode` — undo restores BASE's flag value.

use super::ChangeStrictMode;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStrictMode, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: base.strict_mode })]

    })())
}
//#endregion 🔖️Inverse
