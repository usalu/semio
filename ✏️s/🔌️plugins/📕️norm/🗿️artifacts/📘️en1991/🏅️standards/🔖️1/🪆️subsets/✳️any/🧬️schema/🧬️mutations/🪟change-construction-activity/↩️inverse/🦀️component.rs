//! ↩️ `change-construction-activity` — undo restores BASE's construction activity.

use super::mutation::ChangeConstructionActivity;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeConstructionActivity(ChangeConstructionActivity { new_construction_activity: base.construction_activity.clone() })]
}
//#endregion 🔖️Inverse
