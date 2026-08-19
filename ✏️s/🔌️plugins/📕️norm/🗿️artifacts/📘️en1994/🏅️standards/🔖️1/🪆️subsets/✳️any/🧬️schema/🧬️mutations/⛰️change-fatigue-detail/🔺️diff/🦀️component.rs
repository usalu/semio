//! 🔺️ `change-fatigue-detail` — sparse diff construction.

use super::mutation::ChangeFatigueDetail;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.fatigue_detail == payload.new_fatigue_detail {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fatigue detail already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { fatigue_detail: Some(payload.new_fatigue_detail.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
