//! 🔺️ `upsert-member` — sparse diff construction.

use super::UpdateMemberProperties;
use crate::diff::En1993MemberList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateMemberProperties, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.members.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.member.id) {
        if values[idx] == payload.member {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.member.clone();
    } else {
        values.push(payload.member.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { members: Some(En1993MemberList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
