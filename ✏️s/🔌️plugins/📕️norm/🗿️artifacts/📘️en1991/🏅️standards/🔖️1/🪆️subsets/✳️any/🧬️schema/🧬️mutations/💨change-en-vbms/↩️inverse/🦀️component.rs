//! ↩️ `change-en-vbms` — undo restores BASE's basic wind velocity.

use super::mutation::ChangeEnVBMS;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnVBMS, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeEnVBMS(ChangeEnVBMS { new_en_v_b_m_s: base.en_v_b_m_s.clone() })]
}
//#endregion 🔖️Inverse
