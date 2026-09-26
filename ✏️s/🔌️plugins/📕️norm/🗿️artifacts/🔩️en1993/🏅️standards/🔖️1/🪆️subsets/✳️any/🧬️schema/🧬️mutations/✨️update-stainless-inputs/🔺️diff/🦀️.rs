//! 🔺️ `upsert-material` — sparse diff construction.

use super::UpdateStainlessInputs;
use crate::diff::En1993MaterialList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.materials.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.material.id) {
        if values[idx] == payload.material {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.material.clone();
    } else {
        values.push(payload.material.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { materials: Some(En1993MaterialList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
