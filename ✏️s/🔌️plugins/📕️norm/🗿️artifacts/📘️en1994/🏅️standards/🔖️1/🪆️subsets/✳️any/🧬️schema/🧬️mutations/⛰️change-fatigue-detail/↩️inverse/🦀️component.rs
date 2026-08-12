//! ↩️ `change-fatigue-detail` — undo restores BASE's fatigue_detail.

use super::mutation::ChangeFatigueDetail;
use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: base.fatigue_detail.clone() })]
}
//#endregion 🔖️Inverse
