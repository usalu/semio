//! 🔺️ `change-en-vbms` — sparse diff construction.

use super::mutation::ChangeEnVBMS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnVBMS, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { en_v_b_m_s: Some(payload.new_en_v_b_m_s.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
