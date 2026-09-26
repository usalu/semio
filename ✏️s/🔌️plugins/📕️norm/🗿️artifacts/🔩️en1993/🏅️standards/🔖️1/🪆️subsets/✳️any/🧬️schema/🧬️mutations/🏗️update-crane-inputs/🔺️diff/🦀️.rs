//! 🔺️ `upsert-crane-runway` — sparse diff construction.

use super::UpdateCraneInputs;
use crate::diff::En1993CraneList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateCraneInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.crane_runways.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.crane_runway.id) {
        if values[idx] == payload.crane_runway {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.crane_runway.clone();
    } else {
        values.push(payload.crane_runway.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: Some(En1993CraneList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
