//! 🔺️ `upsert-tension-component` — sparse diff construction.

use super::UpdateTensionComponentInputs;
use crate::diff::En1993TensionList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.tension_components.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.tension_component.id) {
        if values[idx] == payload.tension_component {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.tension_component.clone();
    } else {
        values.push(payload.tension_component.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { tension_components: Some(En1993TensionList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
