//! ↩️ `change-strict-mode` — undo restores BASE's flag value.

use super::mutation::ChangeStrictMode;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeStrictMode, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::ChangeStrictMode(ChangeStrictMode { new_strict_mode: base.strict_mode })]
}
//#endregion 🔖️Inverse
