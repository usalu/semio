//! 🔺️ `upsert-silo-shell` — sparse diff construction.

use super::UpdateSiloShellInputs;
use crate::diff::En1993SiloList;
use crate::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let mut values = base.silo_shells.clone();
    if let Some(idx) = values.iter().position(|x| x.id == payload.silo_shell.id) {
        if values[idx] == payload.silo_shell {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "Entity already has this value.");
        }
        values[idx] = payload.silo_shell.clone();
    } else {
        values.push(payload.silo_shell.clone());
    }
    protocol::MutationOutcome::new(En1993Diff { silo_shells: Some(En1993SiloList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
