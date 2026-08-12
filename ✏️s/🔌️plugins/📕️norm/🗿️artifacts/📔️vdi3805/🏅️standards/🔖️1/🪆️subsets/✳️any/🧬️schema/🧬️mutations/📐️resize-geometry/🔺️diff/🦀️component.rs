//! 🔺️ `resize-geometry` — sparse diff construction; missing id is a no-op clone.

use super::mutation::ResizeGeometry;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ResizeGeometry, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.bbox = payload.new_bbox;
    }
    Vdi3805Diff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
