//! 🔺️ `upsert-plated-panel` — sparse diff construction.

use super::UpdatePlatedInputs;
use crate::diff::En1993PlatedList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.plated_panels.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.plated_panel.id) {
        if values[idx] == payload.plated_panel {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.plated_panel.clone();
    } else {
        values.push(payload.plated_panel.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: Some(En1993PlatedList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
