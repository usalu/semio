//! 🔺️ `upsert-tower-leg` — sparse diff construction.

use super::UpdateTowerInputs;
use crate::diff::En1993TowerList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.tower_legs.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.tower_leg.id) {
        if values[idx] == payload.tower_leg {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.tower_leg.clone();
    } else {
        values.push(payload.tower_leg.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { tower_legs: Some(En1993TowerList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
