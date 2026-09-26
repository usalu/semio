//! 🔺️ `upsert-joint` — sparse diff construction.

use super::UpdateBoltInputs;
use crate::diff::En1993JointList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.joints.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.joint.id) {
        if values[idx] == payload.joint {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.joint.clone();
    } else {
        values.push(payload.joint.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { joints: Some(En1993JointList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
