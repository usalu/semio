//! 🔺️ `remove-point` — sparse diff construction.

use super::mutation::RemovePoint;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Out-of-range `index` is a no-op — the clone is returned unchanged.
pub fn diff(payload: &RemovePoint, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut geometry = base.geometry.clone();
    if payload.index < geometry.points.len() {
        geometry.points.remove(payload.index);
    }
    MathematicalDiff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
