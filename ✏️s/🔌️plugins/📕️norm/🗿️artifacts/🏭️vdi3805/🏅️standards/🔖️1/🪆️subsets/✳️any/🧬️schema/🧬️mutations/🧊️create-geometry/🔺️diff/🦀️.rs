//! 🔺️ `create-geometry` — sparse diff construction.

use super::CreateGeometry;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate id is `mutation.duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub fn diff(payload: &CreateGeometry, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.geometry.contains_key(&payload.geometry.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A geometry with id \"{}\" already exists.", payload.geometry.id), [payload.geometry.id.clone()]);
    }
    let mut geometry = base.geometry.clone();
    geometry.insert(payload.geometry.id.clone(), payload.geometry.clone());
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(geometry), ..Default::default() })
}
//#endregion 🔖️Diff
