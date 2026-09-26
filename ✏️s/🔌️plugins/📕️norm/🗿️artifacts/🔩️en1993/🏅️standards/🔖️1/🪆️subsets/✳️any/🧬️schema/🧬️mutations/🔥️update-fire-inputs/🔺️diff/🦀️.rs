//! 🔺️ `upsert-fire-exposure` — sparse diff construction.

use super::UpdateFireInputs;
use crate::diff::En1993FireList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateFireInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.fire_exposures.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.fire_exposure.id) {
        if values[idx] == payload.fire_exposure {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.fire_exposure.clone();
    } else {
        values.push(payload.fire_exposure.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: Some(En1993FireList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
