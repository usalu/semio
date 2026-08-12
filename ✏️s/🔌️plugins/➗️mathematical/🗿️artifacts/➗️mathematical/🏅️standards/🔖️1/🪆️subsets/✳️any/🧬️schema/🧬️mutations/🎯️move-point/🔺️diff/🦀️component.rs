//! 🔺️ `move-point` — sparse diff construction.

use super::mutation::MovePoint;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MovePoint, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut geometry = base.geometry.clone();
    if let Some(point) = geometry.points.get_mut(payload.index) {
        point.x = payload.x;
        point.y = payload.y;
    }
    MathematicalDiff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
