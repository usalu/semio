//! 🔺️ `create-curve` — sparse diff construction.

use super::mutation::CreateCurve;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate id is `mutation.duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub async fn diff(payload: &CreateCurve, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.curves.contains_key(&payload.curve.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A curve with id \"{}\" already exists.", payload.curve.id), [payload.curve.id.clone()]);
    }
    let mut curves = base.curves.clone();
    curves.insert(payload.curve.id.clone(), payload.curve.clone());
    protocol::MutationOutcome::new(Vdi3805Diff { curves: Some(curves), ..Default::default() })
}
//#endregion 🔖️Diff
