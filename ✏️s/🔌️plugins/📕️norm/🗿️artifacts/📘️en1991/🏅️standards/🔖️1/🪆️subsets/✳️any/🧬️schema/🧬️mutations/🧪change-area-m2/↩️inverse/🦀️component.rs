//! ↩️ `change-area-m2` — undo restores BASE's area.

use super::mutation::ChangeAreaM2;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeAreaM2, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAreaM2(ChangeAreaM2 { new_area_m2: base.area_m2.clone() })]
}
//#endregion 🔖️Inverse
