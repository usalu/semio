//! 🔺️ `delete-curve` — sparse diff construction.

use super::mutation::DeleteCurve;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteCurve, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.curves.contains_key(&payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Curve \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut curves = base.curves.clone();
    curves.remove(&payload.id);
    protocol::MutationOutcome::new(Vdi3805Diff { curves: Some(curves), ..Default::default() })
}
//#endregion 🔖️Diff
