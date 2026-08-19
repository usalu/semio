//! ↩️ `change-hoist-class` — undo restores BASE's hoist class.

use super::mutation::ChangeHoistClass;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeHoistClass, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeHoistClass(ChangeHoistClass { new_hoist_class: base.hoist_class.clone() })]
}
//#endregion 🔖️Inverse
