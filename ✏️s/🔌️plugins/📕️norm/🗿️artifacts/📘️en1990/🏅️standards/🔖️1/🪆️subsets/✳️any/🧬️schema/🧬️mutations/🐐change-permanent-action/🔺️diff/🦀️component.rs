//! 🔺️ `change-permanent-action` — sparse diff construction; writes only `En1990Diff.g_k`.

use super::mutation::ChangePermanentAction;
use crate::artifacts::en1990::{En1990Diff, En1990Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangePermanentAction, _base: &En1990Snapshot) -> En1990Diff {
    En1990Diff { g_k: Some(payload.new_g_k), ..Default::default() }
}
//#endregion 🔖️Diff
