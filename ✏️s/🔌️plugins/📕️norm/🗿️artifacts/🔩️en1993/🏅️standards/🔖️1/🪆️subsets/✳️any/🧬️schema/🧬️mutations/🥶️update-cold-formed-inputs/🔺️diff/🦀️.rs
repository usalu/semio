//! 🔺️ `upsert-cold-formed-member` — sparse diff construction.

use super::UpdateColdFormedInputs;
use crate::diff::En1993ColdFormedList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.cold_formed_members.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.cold_formed_member.id) {
        if values[idx] == payload.cold_formed_member {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.cold_formed_member.clone();
    } else {
        values.push(payload.cold_formed_member.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: Some(En1993ColdFormedList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
