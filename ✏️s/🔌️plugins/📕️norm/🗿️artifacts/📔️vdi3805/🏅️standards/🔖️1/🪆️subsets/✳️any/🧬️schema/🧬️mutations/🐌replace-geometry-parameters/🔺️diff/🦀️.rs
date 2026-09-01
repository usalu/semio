//! 🔺️ `replace-geometry-parameters` — sparse diff construction; missing id is
//! `mutation.target-missing`.

use super::ReplaceGeometryParameters;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceGeometryParameters, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.parameters == payload.new_parameters {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Geometry \"{}\" already has these parameters.", payload.id));
    }
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.parameters = payload.new_parameters.clone();
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(geometry), ..Default::default() })
}
//#endregion 🔖️Diff
