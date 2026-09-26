//! 🔺️ `upsert-fatigue-detail` — sparse diff construction.

use super::UpdateFatigueInputs;
use crate::diff::En1993FatigueList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.fatigue_details.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.fatigue_detail.id) {
        if values[idx] == payload.fatigue_detail {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.fatigue_detail.clone();
    } else {
        values.push(payload.fatigue_detail.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: Some(En1993FatigueList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
