//! 🔺️ `upsert-member-action` — sparse diff construction.

use super::UpdateWeldInputs;
use crate::diff::En1993MemberActionList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.member_actions.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.member_action.id) {
        if values[idx] == payload.member_action {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.member_action.clone();
    } else {
        values.push(payload.member_action.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { member_actions: Some(En1993MemberActionList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
