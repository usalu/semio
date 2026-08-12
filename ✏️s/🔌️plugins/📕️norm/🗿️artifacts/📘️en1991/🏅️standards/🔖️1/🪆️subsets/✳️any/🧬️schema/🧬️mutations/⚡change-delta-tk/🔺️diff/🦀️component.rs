//! 🔺️ `change-delta-tk` — sparse diff construction.

use super::mutation::ChangeDeltaTK;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaTK, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { delta_t_k: Some(payload.new_delta_t_k.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
