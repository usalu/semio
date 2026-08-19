//! ↩️ `change-crane-class` — undo restores BASE's crane class.

use super::mutation::ChangeCraneClass;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCraneClass, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeCraneClass(ChangeCraneClass { new_crane_class: base.crane_class.clone() })]
}
//#endregion 🔖️Inverse
