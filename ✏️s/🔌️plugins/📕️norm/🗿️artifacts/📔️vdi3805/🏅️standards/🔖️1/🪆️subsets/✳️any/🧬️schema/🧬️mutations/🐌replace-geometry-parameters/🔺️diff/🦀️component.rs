//! 🔺️ `replace-geometry-parameters` — sparse diff construction; missing id is a no-op clone.

use super::mutation::ReplaceGeometryParameters;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceGeometryParameters, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.parameters = payload.new_parameters.clone();
    }
    Vdi3805Diff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
